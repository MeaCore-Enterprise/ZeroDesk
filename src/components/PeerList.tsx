import type { PeerInfo } from "../types";

interface Props {
  peers: PeerInfo[];
}

function getLatencyColor(latency: string): string {
  const ms = parseInt(latency, 10);
  if (isNaN(ms) || ms < 0) return "text-zinc-500";
  if (ms < 50) return "text-emerald-400";
  if (ms < 150) return "text-amber-400";
  return "text-red-400";
}

function formatLatency(latency: string): string {
  const ms = parseInt(latency, 10);
  return isNaN(ms) || ms < 0 ? "—" : `${ms}ms`;
}

function getRoleBadge(role: string): string {
  switch (role) {
    case "LEAF":
      return "bg-blue-500/20 text-blue-300";
    case "ROOT":
      return "bg-purple-500/20 text-purple-300";
    case "MOON":
      return "bg-cyan-500/20 text-cyan-300";
    default:
      return "bg-zinc-700 text-zinc-400";
  }
}

export function PeerList({ peers }: Props) {
  return (
    <div className="bg-zinc-900 border border-zinc-800 rounded-xl p-5">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-base font-semibold text-zinc-100">Peers</h3>
        <span className="text-xs text-zinc-500">{peers.length} connected</span>
      </div>

      {peers.length === 0 ? (
        <div className="text-center py-8 text-zinc-500 text-sm">
          No peers connected.
        </div>
      ) : (
        <div className="space-y-2 max-h-[300px] overflow-y-auto pr-1">
          {peers.map((peer) => (
            <div
              key={peer.id}
              className="bg-zinc-800/50 border border-zinc-700/50 rounded-lg p-3"
            >
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <span className="text-sm text-zinc-200 font-mono">{peer.address}</span>
                  <span
                    className={`px-1.5 py-0.5 text-[10px] rounded font-medium ${getRoleBadge(
                      peer.role
                    )}`}
                  >
                    {peer.role}
                  </span>
                </div>
                <span className={`text-xs font-mono ${getLatencyColor(peer.latency)}`}>
                  {formatLatency(peer.latency)}
                </span>
              </div>
              <div className="text-xs text-zinc-500 font-mono mt-1.5 truncate">
                {peer.address}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
