import { useState } from "react";
import { useZerotier } from "./hooks/useZerotier";
import { Sidebar } from "./components/Sidebar";
import { StatusHeader } from "./components/StatusHeader";
import { NetworkList } from "./components/NetworkList";
import { PeerList } from "./components/PeerList";
import type { View } from "./types";

function App() {
  const [view, setView] = useState<View>("dashboard");
  const {
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
  } = useZerotier();

  return (
    <div className="flex h-screen bg-zinc-950 text-zinc-100">
      <Sidebar current={view} onChange={setView} />
      <main className="flex-1 overflow-y-auto p-6">
        {error && (
          <div className="mb-4 px-4 py-3 bg-red-500/10 border border-red-500/30 rounded-lg text-sm text-red-400">
            {error}
          </div>
        )}

        {view === "dashboard" && (
          <div className="space-y-5 max-w-2xl">
            <StatusHeader
              status={status}
              serviceRunning={serviceRunning}
              peerCount={peers.length}
              networkCount={networks.length}
              onStart={startService}
              onStop={stopService}
              onRefresh={refresh}
              loading={loading}
            />
            {networks.length > 0 && (
              <NetworkList
                networks={networks}
                onJoin={joinNetwork}
                onLeave={leaveNetwork}
                serviceRunning={serviceRunning}
              />
            )}
            {peers.length > 0 && <PeerList peers={peers} />}
          </div>
        )}

        {view === "networks" && (
          <div className="max-w-2xl">
            <NetworkList
              networks={networks}
              onJoin={joinNetwork}
              onLeave={leaveNetwork}
              serviceRunning={serviceRunning}
            />
          </div>
        )}

        {view === "peers" && (
          <div className="max-w-2xl">
            <PeerList peers={peers} />
          </div>
        )}

        {loading && (
          <div className="fixed bottom-4 right-4 flex items-center gap-2 px-3 py-2 bg-zinc-800 rounded-lg text-xs text-zinc-400">
            <div className="w-2 h-2 bg-blue-400 rounded-full animate-pulse" />
            Syncing...
          </div>
        )}
      </main>
    </div>
  );
}

export default App;
