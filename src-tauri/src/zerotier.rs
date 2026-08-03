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

fn try_zerotier_command(elevated: bool, args: &[&str]) -> Result<String, String> {
    let (program, extra_args) = if elevated {
        ("pkexec", vec!["zerotier-cli"])
    } else {
        ("zerotier-cli", vec![])
    };

    let mut full_args: Vec<&str> = extra_args;
    full_args.extend_from_slice(args);

    let output = Command::new(program)
        .args(&full_args)
        .output()
        .map_err(|e| format!("Failed to execute {}: {}", program, e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(stderr)
    }
}

fn run_zerotier_command(args: &[&str]) -> Result<String, String> {
    match try_zerotier_command(false, args) {
        Ok(out) => Ok(out),
        Err(e) if e.contains("authtoken.secret") || e.contains("Permission denied") => {
            try_zerotier_command(true, args).map_err(|e2| format!("Command failed: {}", e2))
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
    let output = run_zerotier_command(&["-j", "info"])?;
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
    let output = run_zerotier_command(&["-j", "listnetworks"])?;
    parse_networks_json(&output)
}

#[tauri::command]
pub fn join_network(network_id: &str) -> Result<String, String> {
    run_zerotier_command(&["join", network_id])
}

#[tauri::command]
pub fn leave_network(network_id: &str) -> Result<String, String> {
    run_zerotier_command(&["leave", network_id])
}

#[tauri::command]
pub fn list_peers() -> Result<Vec<PeerInfo>, String> {
    let output = run_zerotier_command(&["-j", "listpeers"])?;
    parse_peers_json(&output)
}

#[tauri::command]
pub fn get_network_info(network_id: &str) -> Result<NetworkInfo, String> {
    let output = run_zerotier_command(&["-j", "listnetworks"])?;
    let networks = parse_networks_json(&output)?;

    networks
        .into_iter()
        .find(|n| n.id == network_id)
        .ok_or_else(|| "Network not found".to_string())
}

#[tauri::command]
pub fn is_service_running() -> Result<bool, String> {
    let output = Command::new("systemctl")
        .args(["is-active", "zerotier-one"])
        .output()
        .map_err(|e| format!("Failed to check service: {}", e))?;

    Ok(String::from_utf8_lossy(&output.stdout).trim() == "active")
}

fn run_systemctl(args: &[&str]) -> Result<String, String> {
    let output = Command::new("pkexec")
        .args(["systemctl"])
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
    run_zerotier_command(&[action, network_id])
}
