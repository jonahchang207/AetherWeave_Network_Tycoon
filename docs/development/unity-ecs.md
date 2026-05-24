---
layout: doc
title: Unity & ECS
nav: dev
section: dev
---

# Unity & ECS

## Packages (manifest)

- `com.unity.entities` — ECS core
- `com.unity.burst` — SIMD jobs
- `com.unity.collections` — native containers
- `com.unity.render-pipelines.universal` — URP rendering

## Simulation tick

`GameBootstrap` accumulates `Time.deltaTime` and fires a fixed sim step (default **0.25 s**). ECS systems should run on that cadence, not every render frame.

## Planned components

| Component | Fields (example) |
|-----------|------------------|
| `NetworkNode` | `NodeId`, region, colo flags |
| `NetworkLink` | `EdgeId`, capacity, utilization, `IsUp` |
| `FlowDemand` | src, dst, gbps, service class |
| `NexusHardware` | reference to JSON device id |

## Systems (order)

1. `LoadFlowDemandsSystem` — NPC traffic generators
2. `NativeRoutingSystem` — call Rust or burst buffer update
3. `CongestionSystem` — apply drops → SLA component
4. `EconomySystem` — MRR, transit invoice
5. `EventSystem` — fiber cuts, festivals

## UI bridge

- UI Toolkit binds to singleton `GameState` or ECS singleton `FinanceSnapshot`.
- GraphView nodes mirror `NetworkNode` entities — use stable `Entity` index in label.

## Shaders

Ley-Glass utilization: pass `float utilization` to material; emissive lerp in Shader Graph.

## Testing in Unity

- Play Mode tests with `Unity.Entities.Tests`
- Edit Mode test loads sample JSON and asserts node count

## Don'ts

- Don’t run Dijkstra in C# per demand per tick at scale.
- Don’t store per-packet entities.
- Don’t hardcode Verdant Shards constants — read `data/regions/`.
