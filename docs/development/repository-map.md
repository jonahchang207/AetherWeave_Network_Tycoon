---
layout: doc
title: Repository Map
nav: dev
section: dev
---

# Repository Map

## Top level

| Path | Purpose |
|------|---------|
| `data/` | Moddable JSON — hardware, regions, cable types |
| `docs/` | Jekyll site for GitHub Pages (design + tutorial + dev) |
| `native/aetherweave-routing/` | Rust `cdylib` — graph, Dijkstra, flow engine |
| `unity/AetherWeave/` | Unity project root |
| `.github/workflows/ci.yml` | Automated `cargo test` |

## Rust crate modules

| File | Responsibility |
|------|----------------|
| `src/graph.rs` | Nodes, directed edges, weights, up/down |
| `src/routing.rs` | Dijkstra SPF + tests (fiber-cut reroute) |
| `src/flow.rs` | Gbps allocation, congestion drop ratio |
| `src/lib.rs` | Public exports |
| `src/ffi.rs` | *(planned)* C ABI for Unity |

## Unity assets (current)

| Path | Responsibility |
|------|----------------|
| `Assets/Scripts/Core/GameBootstrap.cs` | Sim tick loop placeholder |
| `Assets/Scripts/Simulation/NativeRouting.cs` | P/Invoke surface placeholder |
| `Assets/Scripts/AetherWeave.asmdef` | References Entities/Burst |
| `Assets/Plugins/` | Native `.dll` / `.so` / `.dylib` |
| `Packages/manifest.json` | DOTS + URP dependencies |

## Data files (samples)

| File | Contents |
|------|----------|
| `data/regions/verdant_shards.json` | Phase 1 region modifiers, loan, transit cost |
| `data/hardware/nexus_nodes.json` | Router stats and costs |
| `data/ley_glass/cable_types.json` | Fiber types, reach, pricing |

## Planned additions

```
unity/AetherWeave/Assets/
  UI/              # UI Toolkit UXML/USS
  Simulation/ECS/  # Components & systems
  StreamingAssets/ # Copied or symlinked data/*.json
```
