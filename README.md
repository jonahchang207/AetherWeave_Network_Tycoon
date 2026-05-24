# AetherWeave: The Loom of Connectivity

A network-realistic ISP tycoon set in the fantasy-steampunk world of **Oertha**. Build Ley-Glass infrastructure, configure the Archon Routing Protocol (ARP), and grow from a local Weaver startup to a Tier-1 Arch-Loom.

## Repository layout

```
├── docs/                 # Design & architecture
├── data/                 # JSON/Lua game data (moddable)
├── native/
│   └── aetherweave-routing/   # Rust: Dijkstra, flow simulation (Unity native plugin)
└── unity/AetherWeave/    # Unity 3D project (DOTS/ECS, UI Toolkit)
```

## Tech stack

| Area | Choice |
|------|--------|
| Engine | Unity 3D (2022.3 LTS recommended) |
| Simulation architecture | DOTS / ECS |
| Hot paths | Rust native plugin (`aetherweave-routing`) |
| UI | UI Toolkit + GraphView (topology editor) |
| Data | JSON + Lua (mods) |
| Analytics / history | SQLite |
| Audio | FMOD Studio |
| VCS / CI | Git, Git LFS, GitHub Actions |

**Documentation (GitHub Pages):** enable Pages from the `/docs` folder, then open  
`https://<username>.github.io/AetherWeave_Network_Tycoon/` — full [game design](docs/game-design/), [tutorial](docs/tutorial/), and [dev guide](docs/development/). See [docs/development/github-pages.md](docs/development/github-pages.md).

## Prerequisites

- [Unity Hub](https://unity.com/download) with **2022.3 LTS** (or newer) and **Linux/IL2CPP** or **Windows/Mac** build support as needed
- [Rust toolchain](https://rustup.rs/) (`stable`) for the routing library
- Optional: FMOD Studio, Steamworks SDK (later)

## Quick start

### 1. Routing & flow simulation (Rust)

```bash
cd native/aetherweave-routing
cargo test
cargo build --release
```

CI runs these tests on every push.

### 2. Unity project

1. Open **Unity Hub** → **Add** → select `unity/AetherWeave`
2. Let Unity resolve packages from `Packages/manifest.json` (Entities, Burst, Collections, etc.)
3. Enable **DOTS** packages when prompted; import samples only if needed

Native plugin integration (Rust → `.dll` / `.so` / `.dylib`) is documented in `native/aetherweave-routing/README.md`. Wire the built library into `Assets/Plugins/` per platform.

### 3. Game data

Edit JSON under `data/` for hardware, regions, and events. Lua scripts (post-MVP) can override or extend tables for modding.

## Development phases (MVP → full game)

1. **Phase 1** — Verdant Shards: local cables, transit, simple MRR dashboard, one hazard type
2. **Phase 2** — Regional IXPs, QoS, business SLAs
3. **Phase 3** — DWDM upgrades, acquisitions, right-of-way
4. **Phase 4** — Trans-Abyssal links, Tier-1 peering, DDoS / redundancy

## License

TBD.
