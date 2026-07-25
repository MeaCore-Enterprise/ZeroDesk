# ZeroDesk

A modern GUI for ZeroTier built with Tauri + React + TypeScript + TailwindCSS.

![Build](https://github.com/MeaCore-Enterprise/ZeroDesk/actions/workflows/build.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT-green)
![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Platform](https://img.shields.io/badge/platform-Linux-lightgrey)

## Features

- **Dashboard** — View ZeroTier status, connected networks and peers at a glance
- **Network Management** — Join/leave networks by ID with one click
- **Peer Monitoring** — See all connected peers with latency, role and address
- **Service Control** — Start/stop the ZeroTier service from the GUI
- **Auto-refresh** — Status updates every 10 seconds
- **Dark Theme** — Modern UI with TailwindCSS

## Requirements

- Linux (Ubuntu/Debian based)
- ZeroTier One installed and running
- `pkexec` (PolicyKit) for service management

### Install ZeroTier (if not installed)

```bash
curl -s https://install.zerotier.com | sudo bash
sudo systemctl enable zerotier-one
sudo systemctl start zerotier-one
```

### Fix permissions (recommended)

To allow ZeroDesk to read the auth token without requesting root each time:

```bash
sudo chmod 644 /var/lib/zerotier-one/authtoken.secret
```

## Installation

### Option 1: .deb package (recommended)

```bash
sudo dpkg -i ZeroDesk_0.1.0_amd64.deb
sudo apt-get install -f  # install dependencies if needed
```

### Option 2: AppImage

```bash
chmod +x ZeroDesk_0.1.0_amd64.AppImage
./ZeroDesk_0.1.0_amd64.AppImage
```

### Option 3: Build from source

#### Prerequisites

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js 22+
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
sudo apt-get install -y nodejs

# System dependencies
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev \
  libappindicator3-dev librsvg2-dev patchelf
```

#### Build

```bash
git clone https://github.com/MeaCore-Enterprise/ZeroDesk.git
cd ZeroDesk
pnpm install
pnpm tauri build
```

The binaries will be in `src-tauri/target/release/bundle/`.

## Usage

1. Launch ZeroDesk from your application menu or run `zerodesk`
2. The app auto-connects to ZeroTier and shows your status
3. **Join a network**: paste the 16-character Network ID and click Join
4. **Leave a network**: click the Leave button next to the network
5. **Start/Stop service**: use the buttons in the header

## Screenshots

> Coming soon

## Architecture

```
zerodesk/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── lib.rs       # Tauri app setup + command handlers
│   │   ├── main.rs      # Entry point
│   │   └── zerotier.rs  # ZeroTier CLI wrapper + parsers
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                 # React frontend
│   ├── components/
│   │   ├── StatusHeader.tsx   # Service status + controls
│   │   ├── NetworkList.tsx    # Network join/leave
│   │   ├── PeerList.tsx       # Connected peers
│   │   └── Sidebar.tsx        # Navigation
│   ├── hooks/
│   │   └── useZerotier.ts     # State management hook
│   ├── lib/
│   │   └── zerotier.ts        # Tauri invoke wrappers
│   ├── types.ts               # TypeScript interfaces
│   ├── App.tsx
│   └── main.tsx
├── package.json
└── vite.config.ts
```

## Tech Stack

| Layer    | Technology              |
|----------|-------------------------|
| Backend  | Rust + Tauri 2          |
| Frontend | React 19 + TypeScript   |
| Styling  | TailwindCSS 4           |
| Bundler  | Vite 8                  |
| IPC      | Tauri invoke (commands) |

## Permissions

ZeroDesk uses `pkexec` to run privileged commands:
- `zerotier-cli` — network join/leave/status queries
- `systemctl` — start/stop the ZeroTier service

A polkit dialog will appear when root permissions are needed.

## License

MIT License — see [LICENSE](LICENSE) for details.

## Author

**MeaCore Enterprise** — [GitHub](https://github.com/MeaCore-Enterprise)
