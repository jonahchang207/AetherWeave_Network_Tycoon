---
layout: doc
title: Architecture
nav: dev
section: dev
---

# Architecture

## Goals

- **Scale**: Thousands of nodes; millions of logical flows via **flow-based** simulation (not per-packet).
- **Fidelity**: Real networking concepts (attenuation, buffers, BGP-like ARP, transit/peering).
- **Determinism**: Reproducible saves and simulation steps for debugging and tests.

## Layered design

```
┌─────────────────────────────────────────────────────────────┐
│  Presentation (Unity)                                        │
│  World map · UI Toolkit dashboards · GraphView topology      │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│  Game loop (C# / ECS)                                        │
│  Placement · economy · events · save/load orchestration      │
└──────────────────────────┬──────────────────────────────────┘
                           │ FFI
┌──────────────────────────▼──────────────────────────────────┐
│  Simulation core (Rust native plugin)                        │
│  Graph · Dijkstra / SPF · flow allocation · congestion       │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│  Data (JSON/Lua) · SQLite (history) · binary save blobs      │
└─────────────────────────────────────────────────────────────┘
```

## Unity (DOTS / ECS)

- **Entities** represent nodes, links, ports, and aggregate flow counters—not individual packets.
- **Systems** (Burst where possible) tick flow injection, apply link capacity, update satisfaction/SLA flags.
- **MonoBehaviour / UI** bridge for menus, GraphView, and camera on the world map.

Keep **routing and max-flow logic** in Rust; ECS reads/writes compact buffers (adjacency deltas, flow vectors) each tick or on topology change.

## Native simulation (`aetherweave-routing`)

| Module | Role |
|--------|------|
| `graph` | Nodes, directed edges, weights, failure flags |
| `routing` | Dijkstra SPF; incremental recompute on link down/up |
| `flow` | Allocate demand (Gbps) along paths; overflow → loss ratio |
| `ffi` | C ABI for Unity `DllImport` |

Physical cable **pathfinding** on the world map can share the same graph code with different edge costs (distance, right-of-way, shielding).

## Flow-based traffic model

For each demand matrix entry (src, dst, gbps):

1. Run SPF from `src` with edge weights (IGP metric or policy).
2. Push `gbps` along the path; increment edge `utilization`.
3. If `utilization > capacity`, compute `drop_ratio` from buffer model.
4. Aggregate drops into SLA penalties and revenue modifiers.

ARP (BGP-like) policy lives in C# initially; hot SPF stays in Rust.

## Save system

1. **Snapshot** (binary): ECS world + native graph checksum + seed.
2. **Version header** + migration table per schema bump.
3. History charts → SQLite; not inside the snapshot blob.

## MVP boundary (Phase 1)

**In:** one region, cables/nodes, transit, flow sim, fiber cut, basic HUD, Rust SPF.

**Out:** full 3D globe, CLI, IXPs, DWDM, FMOD dynamics, Steam deploy.
