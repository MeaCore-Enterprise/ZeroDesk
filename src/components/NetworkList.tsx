import { useState } from "react";
import type { NetworkInfo } from "../types";

interface Props {
  networks: NetworkInfo[];
  onJoin: (id: string) => void;
  onLeave: (id: string) => void;
  serviceRunning: boolean;
}

export function NetworkList({ networks, onJoin, onLeave, serviceRunning }: Props) {
  const [joinId, setJoinId] = useState("");

  const handleJoin = () => {
    const trimmed = joinId.trim();
    if (trimmed.length === 16) {
      onJoin(trimmed);
      setJoinId("");
    }
  };

  return (
    <div className="bg-zinc-900 border border-zinc-800 rounded-xl p-5">
      <h3 className="text-base font-semibold text-zinc-100 mb-4">Networks</h3>

      <div className="flex gap-2 mb-4">
        <input
          type="text"
          placeholder="Network ID (16 chars)"
          value={joinId}
          onChange={(e) => setJoinId(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && handleJoin()}
          className="flex-1 px-3 py-2 bg-zinc-800 border border-zinc-700 rounded-lg text-sm text-zinc-200 placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500"
          disabled={!serviceRunning}
        />
        <button
          onClick={handleJoin}
          disabled={!serviceRunning || joinId.trim().length !== 16}
          className="px-4 py-2 bg-blue-600 hover:bg-blue-500 disabled:bg-zinc-700 disabled:text-zinc-500 text-white text-sm rounded-lg transition-colors"
        >
          Join
        </button>
      </div>

      {networks.length === 0 ? (
        <div className="text-center py-8 text-zinc-500 text-sm">
          No networks joined. Enter a Network ID above to join one.
        </div>
      ) : (
        <div className="space-y-2">
          {networks.map((network) => (
            <div
              key={network.id}
              className="bg-zinc-800/50 border border-zinc-700/50 rounded-lg p-3 flex items-center justify-between"
            >
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  <span
                    className={`w-2 h-2 rounded-full ${
                      network.status === "OK" ? "bg-emerald-400" : "bg-amber-400"
                    }`}
                  />
                  <span className="text-sm text-zinc-200 font-medium truncate">
                    {network.name || network.id}
                  </span>
                </div>
                <div className="text-xs text-zinc-500 font-mono mt-1 ml-4">
                  {network.id}
                </div>
                {network.assigned_addresses?.length > 0 && (
                  <div className="text-xs text-zinc-400 mt-1 ml-4">
                    {network.assigned_addresses.join(", ")}
                  </div>
                )}
              </div>
              <button
                onClick={() => onLeave(network.id)}
                className="ml-3 px-3 py-1 text-xs bg-zinc-700 hover:bg-red-600 text-zinc-300 hover:text-white rounded-md transition-colors shrink-0"
              >
                Leave
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
