import { useState, useEffect, useCallback } from "react";
import { zerotier } from "../lib/zerotier";
import type { ZtStatus, NetworkInfo, PeerInfo } from "../types";

export function useZerotier() {
  const [status, setStatus] = useState<ZtStatus | null>(null);
  const [networks, setNetworks] = useState<NetworkInfo[]>([]);
  const [peers, setPeers] = useState<PeerInfo[]>([]);
  const [serviceRunning, setServiceRunning] = useState(false);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    const doFetch = async (): Promise<boolean> => {
      const [statusRes, networksRes, peersRes, serviceRes] = await Promise.allSettled([
        zerotier.getStatus(),
        zerotier.listNetworks(),
        zerotier.listPeers(),
        zerotier.isServiceRunning(),
      ]);

      if (statusRes.status === "fulfilled") setStatus(statusRes.value);
      if (networksRes.status === "fulfilled") setNetworks(networksRes.value);
      if (peersRes.status === "fulfilled") setPeers(peersRes.value);
      const serviceRunningRes =
        serviceRes.status === "fulfilled" && serviceRes.value === true;
      setServiceRunning(serviceRunningRes);

      if (!serviceRunningRes) {
        setStatus(null);
        setNetworks([]);
        setPeers([]);
      }

      const errors = [statusRes, networksRes, peersRes]
        .filter(
          (r) => r.status === "rejected" && serviceRunningRes
        )
        .map((r) => (r as PromiseRejectedResult).reason?.toString() || "Unknown error");

      if (errors.length > 0) {
        setError(errors[0]);
        return false;
      }
      return true;
    };

    try {
      setLoading(true);
      setError(null);
      const ok = await doFetch();
      if (!ok) {
        await new Promise((resolve) => setTimeout(resolve, 1500));
        await doFetch();
      }
    } catch (e) {
      setError(e?.toString() || "Failed to connect to ZeroTier");
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    refresh();
    const interval = setInterval(refresh, 10000);
    return () => clearInterval(interval);
  }, [refresh]);

  const joinNetwork = async (networkId: string) => {
    await zerotier.joinNetwork(networkId);
    await refresh();
  };

  const leaveNetwork = async (networkId: string) => {
    await zerotier.leaveNetwork(networkId);
    await refresh();
  };

  const startService = async () => {
    await zerotier.startService();
    await refresh();
  };

  const stopService = async () => {
    await zerotier.stopService();
    await refresh();
  };

  return {
    status,
    networks,
    peers,
    serviceRunning,
    loading,
    error,
    refresh,
    joinNetwork,
    leaveNetwork,
    startService,
    stopService,
  };
}
