---
layout: doc
title: Development Roadmap
nav: dev
section: dev
---

# Development Roadmap

Phased implementation from **current scaffold** to **Phase 4** content.

## Milestone 0 — Foundation ✅ (current)

- [x] Repo layout (`data/`, `native/`, `unity/`, `docs/`)
- [x] Rust graph + Dijkstra + flow tests
- [x] Unity manifest + bootstrap stub
- [x] Sample JSON data
- [x] GitHub Actions `cargo test`
- [x] GitHub Pages documentation site

## Milestone 1 — Playable Phase 1 (4–6 weeks)

| Task | Owner layer |
|------|-------------|
| FFI: graph create, add edge, SPF, flow tick | Rust + C# |
| ECS `NetworkNode`, `NetworkLink`, `FlowDemand` | Unity ECS |
| JSON loader for region + hardware | C# |
| Place cable on 2D/3D map prototype | Unity |
| Transit $ meter + MRR tick | C# |
| Fiber-cut event + repair timer | C# |
| UI Toolkit HUD (MRR, transit, worst link) | UI Toolkit |
| Binary save v1 (graph + economy) | C# + Rust checksum |

**Exit criteria:** 30 minutes of playable Verdant Shards loop without crashes.

## Milestone 2 — Regional (Phase 2)

- Second city + trench tool
- IXP building + peering UI
- QoS queues (3 classes)
- Business SLA contracts + penalties
- Competitor AI price pressure

## Milestone 3 — Continental (Phase 3)

- DWDM upgrade action on existing edges
- ISP acquisition flow
- Right-of-way / bribery events
- ARP policy UI (local pref, prepend)

## Milestone 4 — Global (Phase 4)

- Abyssal submarine links + repeaters
- Tier-1 transit selling
- DDoS + blackhole / scrubber
- Festival traffic spike scripting
- Optional CLI terminal

## Post-launch

- *Void-Net* DLC — LEO satellite mechanics
- *Cyber-Incursion* DLC — hijacking, espionage
- Lua mod API
- Steam Workshop pipeline

## Sprint board template

Copy into GitHub Issues / Projects:

```
Backlog → Ready → In Progress → Review → Done
```

Label issues: `rust`, `unity`, `ui`, `data`, `audio`, `docs`.
