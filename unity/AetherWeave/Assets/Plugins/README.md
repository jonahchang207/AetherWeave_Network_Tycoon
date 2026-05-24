# Native plugins

Place platform builds of `aetherweave-routing` here:

- `x86_64/` — Windows / Linux standalone
- `arm64/` — macOS Apple Silicon
- `x86_64/` (macOS Intel if needed)

Build from `native/aetherweave-routing` with `cargo build --release`.

Unity imports `.dll`, `.so`, and `.dylib` automatically when CPU/OS settings match the target platform.
