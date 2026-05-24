# aetherweave-routing

Rust library for **SPF (Dijkstra)** and **flow-based** link utilization. Built as `cdylib` for Unity `DllImport` and as `rlib` for unit tests.

## Build

```bash
cargo build --release
```

Outputs (platform-dependent):

- macOS: `target/release/libaetherweave_routing.dylib`
- Windows: `target/release/aetherweave_routing.dll`
- Linux: `target/release/libaetherweave_routing.so`

Copy into `unity/AetherWeave/Assets/Plugins/<Platform>/` (create folders as needed).

## Unity FFI (planned)

C ABI entry points will live in `src/ffi.rs`. Example shape:

```csharp
[DllImport("aetherweave_routing")]
static extern uint aw_route(uint graph_handle, uint source, uint dest, uint[] out_path, ref uint path_len);
```

Graph handles and buffer ownership rules will be documented when FFI stabilizes.

## Tests

```bash
cargo test
```

Regression tests cover SPF after link failure and flow overflow drop ratios.
