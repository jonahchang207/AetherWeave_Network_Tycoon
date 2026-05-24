---
layout: default
title: Development
nav: dev
---

<div class="doc-layout">
<aside class="doc-sidebar">
  <h2>Dev guide</h2>
  <ul>
    <li><a href="{{ '/development/' | relative_url }}">Hub (tabs)</a></li>
    <li><a href="{{ '/development/setup/' | relative_url }}">Setup</a></li>
    <li><a href="{{ '/development/repository-map/' | relative_url }}">Repository map</a></li>
    <li><a href="{{ '/development/architecture/' | relative_url }}">Architecture</a></li>
    <li><a href="{{ '/development/tech-stack/' | relative_url }}">Tech stack</a></li>
    <li><a href="{{ '/development/roadmap/' | relative_url }}">Roadmap</a></li>
    <li><a href="{{ '/development/rust-plugin/' | relative_url }}">Rust plugin</a></li>
    <li><a href="{{ '/development/unity-ecs/' | relative_url }}">Unity & ECS</a></li>
    <li><a href="{{ '/development/data-modding/' | relative_url }}">Data & modding</a></li>
    <li><a href="{{ '/development/github-pages/' | relative_url }}">GitHub Pages</a></li>
  </ul>
</aside>
<div>

# Development Hub

End-to-end guide from **this repository’s current state** to a shippable Steam build. Use the tabs below for quick orientation, or the sidebar for deep dives.

<div class="tab-bar" data-tab-group="dev-hub">
  <button type="button" data-tab="overview">Overview</button>
  <button type="button" data-tab="setup">Setup</button>
  <button type="button" data-tab="workflow">Daily workflow</button>
  <button type="button" data-tab="ship">Ship checklist</button>
</div>

<div class="tab-panel" data-tab-panel="dev-hub" data-tab-id="overview">

## Where we are today

| Component | Status |
|-----------|--------|
| `native/aetherweave-routing` | ✅ Graph, Dijkstra SPF, flow allocation, unit tests |
| `unity/AetherWeave` | 🟡 Skeleton — manifest, asmdef, bootstrap stub |
| `data/*.json` | 🟡 Sample region, hardware, cables |
| FFI Unity ↔ Rust | ⬜ Planned (`ffi.rs`, `NativeRouting.cs`) |
| ECS simulation | ⬜ Planned |
| UI Toolkit / GraphView | ⬜ Planned |
| FMOD / Steam | ⬜ Post-MVP |

## Repository layout

```
AetherWeave_Network_Tycoon/
├── data/                      # JSON game tables (moddable)
├── docs/                      # This GitHub Pages site (Jekyll)
├── native/aetherweave-routing # Rust cdylib — routing & flows
├── unity/AetherWeave/         # Unity 2022.3 + DOTS packages
└── .github/workflows/ci.yml   # cargo test on push
```

## Architecture (one screen)

<div class="mermaid-block">
Unity (C# / ECS) ──FFI──▶ Rust (SPF + flows)
       │                        ▲
       ▼                        │
  UI Toolkit / GraphView    JSON / SQLite / saves
</div>

Full detail: [Architecture]({{ '/development/architecture/' | relative_url }}).

</div>

<div class="tab-panel" data-tab-panel="dev-hub" data-tab-id="setup">

## Prerequisites

1. **Git** + **Git LFS** (`git lfs install`)
2. **Unity Hub** → **2022.3 LTS**
3. **Rust stable** — [rustup.rs](https://rustup.rs/)
4. Optional: **Bundler** + Ruby for local Jekyll preview

## Clone & verify

```bash
git clone https://github.com/YOUR_USER/AetherWeave_Network_Tycoon.git
cd AetherWeave_Network_Tycoon

cd native/aetherweave-routing
cargo test
cargo build --release
```

Open `unity/AetherWeave` in Unity Hub and allow package restore.

See [Setup]({{ '/development/setup/' | relative_url }}) for platform plugin paths and troubleshooting.

</div>

<div class="tab-panel" data-tab-panel="dev-hub" data-tab-id="workflow">

## Recommended daily loop

1. **Branch** from `develop` (or `main` for solo dev).
2. Change simulation in **Rust** first → `cargo test`.
3. Mirror types / FFI in **C#** if ABI changed.
4. Wire **ECS systems** to call native or burst-friendly buffers.
5. Tune constants in **`data/`** JSON — no recompile for designers.
6. Push → GitHub Actions runs routing tests.

## What to build next (MVP order)

1. `ffi.rs` + copy `.dylib`/`.dll` to `Assets/Plugins/`
2. ECS components: `NetworkNode`, `NetworkLink`, `FlowDemand`
3. Load `data/regions/verdant_shards.json` at boot
4. UI Toolkit HUD: MRR, transit, worst link utilization
5. Fiber-cut event → `set_edge_up(false)` → SPF rerun

[Roadmap]({{ '/development/roadmap/' | relative_url }}) has sprint-sized tasks.

</div>

<div class="tab-panel" data-tab-panel="dev-hub" data-tab-id="ship">

## Ship checklist (high level)

- [ ] Phase 1 playable loop in Verdant Shards
- [ ] Deterministic save/load round-trip test
- [ ] Performance budget: 10k+ logical flows @ 60 FPS sim tick
- [ ] Git LFS for all binary assets
- [ ] Unity build in CI (game-ci + license secret)
- [ ] FMOD banks integrated
- [ ] Steam depot + achievement hooks
- [ ] Store page + this docs site linked from menu

## Quality gates

- Zero failing `cargo test` on `main`
- Routing regression test for fiber-cut reroute
- No hardcoded crowns/Mbps in C# — load from JSON

</div>

</div>
</div>
