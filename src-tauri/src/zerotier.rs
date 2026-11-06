use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkInfo {
    pub id: String,
    pub name: String,
    pub status: String,
    pub assigned_addresses: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PeerInfo {
    pub id: String,
    pub address: String,
    pub version: String,
    pub latency: String,
    pub paths: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZtStatus {
    pub version: String,
    pub address: String,
    pub online: bool,
}

const ZEROTIER_CLI: &str = "/usr/sbin/zerotier-cli";
const PKEXEC: &str = "/usr/bin/pkexec";
const SYSTEMCTL: &str = "/usr/bin/systemctl";

fn service_is_active() -> bool {
    Command::new(SYSTEMCTL)
        .args(["is-active", "zerotier-one"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "active")
        .unwrap_or(false)
}

fn run_zerotier_non_elevated(args: &[&str]) -> Result<String, String> {
    let output = Command::new(ZEROTIER_CLI)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute {}: {}", ZEROTIER_CLI, e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn run_zerotier_elevated(args: &[&str]) -> Result<String, String> {
    let mut full_args: Vec<&str> = vec![ZEROTIER_CLI];
    full_args.extend_from_slice(args);

    let output = Command::new(PKEXEC)
        .args(&full_args)
        .output()
        .map_err(|e| format!("Failed to execute {}: {}", PKEXEC, e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        if !stderr.trim().is_empty() {
            Err(stderr)
        } else if !stdout.trim().is_empty() {
            Err(stdout)
        } else {
            Err(format!(
                "{} exited with status {:?}",
                PKEXEC,
                output.status.code()
            ))
        }
    }
}

// Read-only commands: never start the service. Elevation is allowed as a
// transparent fallback (passwordless via polkit rules) so reads work whether
// or not the authtoken is readable by the user. Retries a few times because
// the service reports "active" before the local API port is ready.
fn read_zerotier(args: &[&str]) -> Result<String, String> {
    if !service_is_active() {
        return Err("ZeroTier service is not running".to_string());
    }
    let mut last_err = String::new();
    for _ in 0..3 {
        let res = match run_zerotier_non_elevated(args) {
            Ok(out) => return Ok(out),
            Err(e) if e.contains("authtoken.secret") || e.contains("Permission denied") => {
                run_zerotier_elevated(args)
            }
            Err(e) => Err(e),
        };
        match res {
            Ok(out) => return Ok(out),
            Err(e) => {
                last_err = e;
                if last_err.contains("connection failed") {
                    std::thread::sleep(std::time::Duration::from_millis(300));
                    continue;
                }
                return Err(last_err);
            }
        }
    }
    Err(last_err)
}

// Mutating commands (user initiated): try as the user first, elevate only on permission errors.
fn write_zerotier(args: &[&str]) -> Result<String, String> {
    match run_zerotier_non_elevated(args) {
        Ok(out) => Ok(out),
        Err(e) if e.contains("authtoken.secret") || e.contains("Permission denied") => {
            run_zerotier_elevated(args).map_err(|e2| format!("Command failed: {}", e2))
        }
        Err(e) => Err(format!("Command failed: {}", e)),
    }
}

fn parse_networks_json(output: &str) -> Result<Vec<NetworkInfo>, String> {
    let json: serde_json::Value =
        serde_json::from_str(output).map_err(|e| format!("Failed to parse networks JSON: {}", e))?;

    let networks = json
        .as_array()
        .ok_or_else(|| "Invalid networks response".to_string())?;

    Ok(networks
        .iter()
        .map(|n| NetworkInfo {
            id: n["id"].as_str().unwrap_or("").to_string(),
            name: n["name"].as_str().unwrap_or("").to_string(),
            status: n["status"].as_str().unwrap_or("").to_string(),
            assigned_addresses: n["assignedAddresses"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
        })
        .collect())
}

fn parse_peers_json(output: &str) -> Result<Vec<PeerInfo>, String> {
    let json: serde_json::Value =
        serde_json::from_str(output).map_err(|e| format!("Failed to parse peers JSON: {}", e))?;

    let peers = json
        .as_array()
        .ok_or_else(|| "Invalid peers response".to_string())?;

    Ok(peers
        .iter()
        .map(|p| {
            let paths: Vec<String> = p["paths"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|path| path["address"].as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();

            PeerInfo {
                id: p["address"].as_str().unwrap_or("").to_string(),
                address: paths.first().cloned().unwrap_or_else(|| {
                    p["address"].as_str().unwrap_or("").to_string()
                }),
                version: p["version"].as_str().unwrap_or("").to_string(),
                latency: p["latency"].as_i64().unwrap_or(-1).to_string(),
                paths: paths.join(", "),
                role: p["role"].as_str().unwrap_or("").to_string(),
            }
        })
        .collect())
}

#[tauri::command]
pub fn get_status() -> Result<ZtStatus, String> {
    let output = read_zerotier(&["-j", "info"])?;
    let json: serde_json::Value =
        serde_json::from_str(&output).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    Ok(ZtStatus {
        version: json["version"].as_str().unwrap_or("unknown").to_string(),
        address: json["address"].as_str().unwrap_or("unknown").to_string(),
        online: json["online"].as_bool().unwrap_or(false),
    })
}

#[tauri::command]
pub fn list_networks() -> Result<Vec<NetworkInfo>, String> {
    let output = read_zerotier(&["-j", "listnetworks"])?;
    parse_networks_json(&output)
}

#[tauri::command]
pub fn join_network(network_id: &str) -> Result<String, String> {
    write_zerotier(&["join", network_id])
}

#[tauri::command]
pub fn leave_network(network_id: &str) -> Result<String, String> {
    write_zerotier(&["leave", network_id])
}

#[tauri::command]
pub fn list_peers() -> Result<Vec<PeerInfo>, String> {
    let output = read_zerotier(&["-j", "listpeers"])?;
    parse_peers_json(&output)
}

#[tauri::command]
pub fn get_network_info(network_id: &str) -> Result<NetworkInfo, String> {
    let output = read_zerotier(&["-j", "listnetworks"])?;
    let networks = parse_networks_json(&output)?;

    networks
        .into_iter()
        .find(|n| n.id == network_id)
        .ok_or_else(|| "Network not found".to_string())
}

#[tauri::command]
pub fn is_service_running() -> Result<bool, String> {
    Ok(service_is_active())
}

fn run_systemctl(args: &[&str]) -> Result<String, String> {
    let output = Command::new(PKEXEC)
        .args([SYSTEMCTL])
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute systemctl: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        if stderr.contains("canceled") || stderr.is_empty() {
            Err("Authentication canceled".to_string())
        } else {
            Err(format!("Failed: {}", stderr))
        }
    }
}

#[tauri::command]
pub fn start_service() -> Result<String, String> {
    run_systemctl(&["start", "zerotier-one"])?;
    Ok("Service started".to_string())
}

#[tauri::command]
pub fn stop_service() -> Result<String, String> {
    run_systemctl(&["stop", "zerotier-one"])?;
    Ok("Service stopped".to_string())
}

#[tauri::command]
pub fn toggle_autoconnect(network_id: &str, enabled: bool) -> Result<String, String> {
    let action = if enabled { "join" } else { "leave" };
    write_zerotier(&[action, network_id])
}
