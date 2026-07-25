import type { View } from "../types";

interface Props {
  current: View;
  onChange: (view: View) => void;
}

const tabs: { id: View; label: string; icon: string }[] = [
  { id: "dashboard", label: "Dashboard", icon: "⚡" },
  { id: "networks", label: "Networks", icon: "🌐" },
  { id: "peers", label: "Peers", icon: "👥" },
];

export function Sidebar({ current, onChange }: Props) {
  return (
    <aside className="w-[180px] bg-zinc-950 border-r border-zinc-800 p-3 flex flex-col gap-1 shrink-0">
      <div className="px-3 py-2 mb-2">
        <div className="text-base font-bold text-zinc-100 tracking-tight">ZeroDesk</div>
        <div className="text-[10px] text-zinc-500 uppercase tracking-widest">v0.1.0</div>
      </div>
      {tabs.map((tab) => (
        <button
          key={tab.id}
          onClick={() => onChange(tab.id)}
          className={`flex items-center gap-2.5 px-3 py-2.5 rounded-lg text-sm transition-colors ${
            current === tab.id
              ? "bg-zinc-800 text-zinc-100"
              : "text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200"
          }`}
        >
          <span className="text-base">{tab.icon}</span>
          {tab.label}
        </button>
      ))}
    </aside>
  );
}
