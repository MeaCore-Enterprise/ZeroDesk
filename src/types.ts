export interface NetworkInfo {
  id: string;
  name: string;
  status: string;
  assigned_addresses: string[];
}

export interface PeerInfo {
  id: string;
  address: string;
  version: string;
  latency: string;
  paths: string;
  role: string;
}

export interface ZtStatus {
  version: string;
  address: string;
  online: boolean;
}

export type View = "dashboard" | "networks" | "peers";
