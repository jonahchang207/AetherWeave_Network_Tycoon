---
layout: default
title: Rust Plugin
nav: dev
---

# Rust Native Plugin

The `aetherweave-routing` crate is the performance core for SPF and flow allocation.

## Build

```bash
cd native/aetherweave-routing
cargo test
cargo build --release
```

## API surface (Rust)

```rust
// Graph
let mut g = Graph::new();
let a = g.add_node();
let b = g.add_node();
g.add_edge(a, b, weight, capacity_gbps);

// Routing
let path = dijkstra(&g, a, b)?;

// Flow
let mut load = vec![0.0; g.edge_count()];
FlowEngine::allocate(&g, a, b, demand_gbps, &mut load)?;
```

## Fiber-cut regression

Tests in `routing.rs` assert that when the low-metric edge fails, SPF uses the backup path. **CI must stay green** — do not merge routing changes without tests.

## Planned FFI (`src/ffi.rs`)

Export C ABI for Unity:

| Function | Purpose |
|----------|---------|
| `aw_graph_create()` | Opaque handle |
| `aw_graph_destroy(handle)` | Free memory |
| `aw_graph_add_edge(...)` | Topology mutation |
| `aw_graph_set_edge_up(edge, bool)` | Failover events |
| `aw_route(handle, src, dst, path_out, len)` | SPF path |
| `aw_flow_allocate(...)` | Push demand, update loads |

Use `#[no_mangle] pub extern "C"` and document thread-safety: **single sim thread** unless guarded.

## Unity P/Invoke

Extend `NativeRouting.cs`:

```csharp
[DllImport("aetherweave_routing")]
static extern uint aw_graph_create();
```

Match library name per platform (`__Internal` on iOS if ever needed).

## Performance notes

- Batch SPF recomputes when multiple links flap — debounce 50–100 ms sim time.
- Store adjacency in SoA buffers for Burst jobs reading utilization only.
- Incremental SPF (RFC-style) is a future optimization; full Dijkstra is fine for MVP scale.
