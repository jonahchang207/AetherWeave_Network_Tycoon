---
layout: default
title: Developer Setup
nav: dev
---

# Developer Setup

## 1. Clone the repository

```bash
git clone https://github.com/YOUR_USER/AetherWeave_Network_Tycoon.git
cd AetherWeave_Network_Tycoon
git lfs pull
```

Replace `YOUR_USER` with your GitHub username or organization.

## 2. Rust routing library

Install Rust via [rustup](https://rustup.rs/), then:

```bash
cd native/aetherweave-routing
cargo test
cargo build --release
```

| Platform | Library output |
|----------|----------------|
| macOS (Apple Silicon) | `target/release/libaetherweave_routing.dylib` |
| macOS (Intel) | same |
| Windows | `target/release/aetherweave_routing.dll` |
| Linux | `target/release/libaetherweave_routing.so` |

Copy the built library into:

```
unity/AetherWeave/Assets/Plugins/
  x86_64/          # Windows / Linux standalone
  arm64/           # macOS Apple Silicon (folder name may vary — set in Unity Inspector)
```

In Unity, select the plugin and enable the correct **CPU/OS** checkboxes.

## 3. Unity project

1. Install **Unity Hub** and editor **2022.3.50f1** (or nearest LTS patch).
2. **Add** project from disk → `unity/AetherWeave`.
3. First open installs Entities, Burst, Collections, URP per `Packages/manifest.json`.
4. Create a scene with an empty GameObject + `AetherWeave.Core.GameBootstrap` component.

<div class="callout warn">
  <div class="callout-title">Package resolution</div>
  If Entities fails to resolve, open <strong>Window → Package Manager</strong> and refresh. Match editor version to <code>ProjectSettings/ProjectVersion.txt</code>.
</div>

## 4. IDE recommendations

| Tool | Use |
|------|-----|
| Rider / Visual Studio | C# + Unity debugging |
| rust-analyzer | Rust crate |
| VS Code | Docs + JSON data editing |

## 5. Run CI locally

```bash
cd native/aetherweave-routing && cargo test --verbose
```

Unity batch builds require a license — see [GitHub Pages / CI]({{ '/development/github-pages/' | relative_url }}).

## Troubleshooting

| Problem | Fix |
|---------|-----|
| `DllNotFoundException` for routing | Plugin not in `Assets/Plugins` or wrong architecture |
| `cargo` not found | Add `~/.cargo/bin` to PATH |
| Entities compile errors | Update editor to 2022.3 LTS; reimport packages |
| Git LFS pointers in assets | Run `git lfs pull` |
