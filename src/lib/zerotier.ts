import { invoke } from "@tauri-apps/api/core";
import type { NetworkInfo, PeerInfo, ZtStatus } from "../types";

export const zerotier = {
  getStatus: () => invoke<ZtStatus>("get_status"),
  listNetworks: () => invoke<NetworkInfo[]>("list_networks"),
  joinNetwork: (networkId: string) =>
    invoke<string>("join_network", { networkId }),
  leaveNetwork: (networkId: string) =>
    invoke<string>("leave_network", { networkId }),
  listPeers: () => invoke<PeerInfo[]>("list_peers"),
  getNetworkInfo: (networkId: string) =>
    invoke<NetworkInfo>("get_network_info", { networkId }),
  isServiceRunning: () => invoke<boolean>("is_service_running"),
  startService: () => invoke<string>("start_service"),
  stopService: () => invoke<string>("stop_service"),
  toggleAutoconnect: (networkId: string, enabled: boolean) =>
    invoke<string>("toggle_autoconnect", { networkId, enabled }),
};
