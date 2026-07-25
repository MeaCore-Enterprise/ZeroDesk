import type { ZtStatus } from "../types";

interface Props {
  status: ZtStatus | null;
  serviceRunning: boolean;
  peerCount: number;
  networkCount: number;
  onStart: () => void;
  onStop: () => void;
  onRefresh: () => void;
  loading: boolean;
}

export function StatusHeader({
  status,
  serviceRunning,
  peerCount,
  networkCount,
  onStart,
  onStop,
  onRefresh,
  loading,
}: Props) {
  return (
    <div className="bg-zinc-900 border border-zinc-800 rounded-xl p-5">
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <div
            className={`w-3 h-3 rounded-full ${
              serviceRunning && status?.online
                ? "bg-emerald-400 shadow-[0_0_8px_rgba(52,211,153,0.6)]"
                : "bg-red-400 shadow-[0_0_8px_rgba(248,113,113,0.6)]"
            }`}
          />
          <h2 className="text-lg font-semibold text-zinc-100">
            {serviceRunning && status?.online ? "Connected" : "Disconnected"}
          </h2>
        </div>
        <div className="flex gap-2">
          <button
            onClick={onRefresh}
            disabled={loading}
            className="px-3 py-1.5 text-sm bg-zinc-800 hover:bg-zinc-700 text-zinc-300 rounded-lg transition-colors disabled:opacity-50"
          >
            Refresh
          </button>
          {serviceRunning ? (
            <button
              onClick={onStop}
              className="px-3 py-1.5 text-sm bg-red-600 hover:bg-red-500 text-white rounded-lg transition-colors"
            >
              Stop
            </button>
          ) : (
            <button
              onClick={onStart}
              className="px-3 py-1.5 text-sm bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg transition-colors"
            >
              Start
            </button>
          )}
        </div>
      </div>
      <div className="grid grid-cols-3 gap-4">
        <div className="bg-zinc-800/50 rounded-lg p-3">
          <div className="text-xs text-zinc-500 uppercase tracking-wider">Version</div>
          <div className="text-sm text-zinc-200 mt-1">{status?.version || "---"}</div>
        </div>
        <div className="bg-zinc-800/50 rounded-lg p-3">
          <div className="text-xs text-zinc-500 uppercase tracking-wider">Networks</div>
          <div className="text-sm text-zinc-200 mt-1">{networkCount}</div>
        </div>
        <div className="bg-zinc-800/50 rounded-lg p-3">
          <div className="text-xs text-zinc-500 uppercase tracking-wider">Peers</div>
          <div className="text-sm text-zinc-200 mt-1">{peerCount}</div>
        </div>
      </div>
      {status?.address && (
        <div className="mt-3 px-3 py-2 bg-zinc-800/30 rounded-lg">
          <span className="text-xs text-zinc-500">Address: </span>
          <span className="text-xs text-zinc-300 font-mono">{status.address}</span>
        </div>
      )}
    </div>
  );
}
