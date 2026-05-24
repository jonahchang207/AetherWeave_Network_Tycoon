---
layout: doc
title: Tech Stack
nav: dev
section: dev
---

# Technology Stack

## Engine & architecture

| Component | Choice | Notes |
|-----------|--------|--------|
| Game engine | **Unity 3D** (2022.3 LTS) | Tycoon/sim precedent; cross-platform |
| Simulation layout | **DOTS / ECS** | Contiguous data, Burst, multi-threaded macro flows |
| Rendering | **URP** + **HLSL / Shader Graph** | Ley-Glass glow, Aether-Sea stylization |

## Languages

| Layer | Language | Use |
|-------|----------|-----|
| Gameplay & UI | **C#** | ECS systems, UI Toolkit, save orchestration |
| Hot paths | **Rust** (`aetherweave-routing`) | Dijkstra SPF, flow allocation; `cdylib` for Unity |
| Shaders | **HLSL** | VFX tied to utilization and hazards |

C++ remains optional for additional native plugins; this repo standardizes on Rust for safety and unit tests.

## UI/UX

- **UI Toolkit** — dashboards, router forms, scaling layouts
- **GraphView** — logical topology editor (BGP weights, VLANs later)

## Simulation

- **Flow-based model** — Gbps aggregates over edges
- **Dijkstra** — IGP SPF; shared core for cable pathfinding with different costs
- **ARP (BGP-like)** — policy in C#; SPF in Rust

## Data & persistence

| Concern | Format |
|---------|--------|
| Design data | **JSON** (+ **Lua** for mods) in `data/` |
| Traffic / finance history | **SQLite** |
| Save games | Versioned **binary** snapshots + deterministic sim seed |

## Audio

- **FMOD Studio** — parameter-driven server hum, regional storm beds

## DevOps

- **Git** + **Git LFS** (meshes, textures, audio)
- **GitHub Actions** — `cargo test` on routing; Unity build stub in CI
- **Steamworks SDK** — deploy pipeline (post-MVP)

## Repo mapping

```
native/aetherweave-routing/  → Rust simulation + FFI
unity/AetherWeave/           → Unity project (ECS, UI, world)
data/                         → Moddable tables
docs/                         → GitHub Pages (this site)
```
