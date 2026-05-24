---
layout: doc
title: Developer Setup
nav: dev
section: dev
---

# Developer Setup

Complete guide to clone, build, and run **AetherWeave** locally — from zero to a Play-mode scene with the simulation bootstrap wired up. Estimated time: **30–60 minutes** on a fresh machine (mostly Unity Hub + package download).

## Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [1. Clone the repository](#1-clone-the-repository)
- [2. Rust routing library](#2-rust-routing-library)
- [3. Unity project](#3-unity-project)
  - [3.1 Install Unity Hub and editor](#31-install-unity-hub-and-editor)
  - [3.2 Add the correct project folder](#32-add-the-correct-project-folder)
  - [3.3 First open and package resolution](#33-first-open-and-package-resolution)
  - [3.4 Configure the native plugin in Unity](#34-configure-the-native-plugin-in-unity)
  - [3.5 Create a bootstrap scene with GameBootstrap](#35-create-a-bootstrap-scene-with-gamebootstrap)
  - [3.6 Add the scene to Build Settings](#36-add-the-scene-to-build-settings)
  - [3.7 Verify Play mode](#37-verify-play-mode)
- [4. IDE setup](#4-ide-setup)
- [5. Run CI locally](#5-run-ci-locally)
- [6. Verification checklist](#6-verification-checklist)
- [7. Daily workflow](#7-daily-workflow)
- [Troubleshooting](#troubleshooting)
- [Next steps](#next-steps)

---

## Overview

AetherWeave is a **monorepo** with three main areas you touch during setup:

| Path | What it is |
|------|------------|
| `native/aetherweave-routing/` | Rust `cdylib` — Dijkstra SPF, flow allocation, unit tests |
| `unity/AetherWeave/` | Unity 2022.3 LTS project — DOTS/ECS, C# bootstrap, native plugins |
| `data/` | Moddable JSON — regions, hardware, cable types (no build step) |

After setup you will have:

1. Rust tests passing locally (`cargo test`)
2. A release native library copied into `Assets/Plugins/`
3. Unity open on the **correct** project root with Entities/Burst/URP resolved
4. A saved scene containing a **GameBootstrap** GameObject ready for simulation work

<div class="callout">
  <div class="callout-title">Current MVP state</div>
  <code>GameBootstrap</code> runs a fixed simulation tick loop, but <code>TickSimulation()</code> is still a placeholder. The Rust DLL is built and placed in Plugins, but FFI (<code>ffi.rs</code>) and P/Invoke in <code>NativeRouting.cs</code> are not wired yet — see <a href="{{ '/development/rust-plugin/' | relative_url }}">Rust Plugin</a>.
</div>

---

## Prerequisites

Install these before starting. All are free for individual development.

| Requirement | Version / notes | Install |
|-------------|-----------------|---------|
| **Git** | 2.x+ | [git-scm.com](https://git-scm.com/) |
| **Git LFS** | Latest | Bundled with Git for Windows; or `git lfs install` |
| **Rust** | Stable via rustup | [rustup.rs](https://rustup.rs/) |
| **Unity Hub** | Latest | [unity.com/download](https://unity.com/download) |
| **Unity Editor** | **2022.3.50f1** (or nearest 2022.3 LTS patch) | Via Unity Hub → **Installs** |
| **Disk space** | ~15 GB | Unity editor + Library cache |

**Optional but recommended**

| Tool | Use |
|------|-----|
| JetBrains Rider or Visual Studio 2022 | C# editing, Unity debugging |
| VS Code / Cursor + rust-analyzer | Rust crate, docs, JSON data |
| Ruby + Bundler | Local Jekyll preview of this docs site |

**Check versions (terminal)**

```bash
git --version
git lfs version
rustc --version
cargo --version
```

Expected Rust output looks like `rustc 1.xx.x` and `cargo 1.xx.x`. If `cargo` is not found, add `$HOME/.cargo/bin` (macOS/Linux) or `%USERPROFILE%\.cargo\bin` (Windows) to your PATH and restart the terminal.

---

## 1. Clone the repository

### HTTPS (simplest)

```bash
git clone https://github.com/jonahchang207/AetherWeave_Network_Tycoon.git
cd AetherWeave_Network_Tycoon
```

Replace `jonahchang207` with your fork or organization if you cloned a different remote.

### SSH (if you use SSH keys)

```bash
git clone git@github.com:jonahchang207/AetherWeave_Network_Tycoon.git
cd AetherWeave_Network_Tycoon
```

### Pull Git LFS assets

Large binary assets (if any) are tracked with Git LFS. Always run after clone:

```bash
git lfs install
git lfs pull
```

**Verify LFS worked:** If you see tiny pointer files (a few bytes) instead of real images/meshes under `Assets/`, run `git lfs pull` again. See [Troubleshooting](#git-lfs-pointer-files-instead-of-real-assets).

### Confirm repository layout

From the repo root you should see:

```
AetherWeave_Network_Tycoon/
├── data/
├── docs/
├── native/aetherweave-routing/
├── unity/AetherWeave/
└── .github/workflows/ci.yml
```

---

## 2. Rust routing library

The crate `native/aetherweave-routing` builds a shared library consumed by Unity via `DllImport`. The same crate also runs unit tests for routing and flow logic (CI runs these on every push).

### 2.1 Install Rust (if needed)

**Windows / macOS / Linux:** download and run [rustup.rs](https://rustup.rs/), choose **stable** default.

After install, open a **new** terminal and confirm:

```bash
rustc --version
cargo --version
```

### 2.2 Run tests

```bash
cd native/aetherweave-routing
cargo test
```

**Expected:** 4 tests pass (routing preferences, link failure reroute, no path when cut, congestion drop ratio). Example output:

```
test result: ok. 4 passed; 0 failed
```

If tests fail, do not copy the DLL into Unity until fixed — see [Rust tests fail](#rust-tests-fail).

### 2.3 Build release library

```bash
cargo build --release
```

Release builds are optimized and are what you ship to `Assets/Plugins/`. Debug builds are for development only.

### 2.4 Library output by platform

| Platform | Release output path |
|----------|---------------------|
| **Windows** | `native/aetherweave-routing/target/release/aetherweave_routing.dll` |
| **macOS (Apple Silicon)** | `native/aetherweave-routing/target/release/libaetherweave_routing.dylib` |
| **macOS (Intel)** | same as Apple Silicon |
| **Linux** | `native/aetherweave-routing/target/release/libaetherweave_routing.so` |

<div class="callout warn">
  <div class="callout-title">target/ is not committed</div>
  The <code>native/**/target/</code> folder is gitignored. Every developer must run <code>cargo build --release</code> locally (or in CI) and copy the artifact into Unity.
</div>

### 2.5 Copy into Unity Plugins

Create platform folders under `unity/AetherWeave/Assets/Plugins/` if they do not exist:

```
unity/AetherWeave/Assets/Plugins/
├── README.md
├── x86_64/          ← Windows and Linux standalone (64-bit)
│   └── aetherweave_routing.dll   (or .so on Linux)
└── arm64/           ← macOS Apple Silicon
    └── libaetherweave_routing.dylib
```

**Windows (PowerShell)** — from repo root:

```powershell
New-Item -ItemType Directory -Force -Path "unity\AetherWeave\Assets\Plugins\x86_64"
Copy-Item -Force "native\aetherweave-routing\target\release\aetherweave_routing.dll" `
  "unity\AetherWeave\Assets\Plugins\x86_64\aetherweave_routing.dll"
```

**macOS / Linux (bash)** — adjust source filename for your OS:

```bash
mkdir -p unity/AetherWeave/Assets/Plugins/x86_64
# Linux:
cp native/aetherweave-routing/target/release/libaetherweave_routing.so \
   unity/AetherWeave/Assets/Plugins/x86_64/
# macOS Apple Silicon — use arm64/ instead:
mkdir -p unity/AetherWeave/Assets/Plugins/arm64
cp native/aetherweave-routing/target/release/libaetherweave_routing.dylib \
   unity/AetherWeave/Assets/Plugins/arm64/
```

After copying, Unity will auto-generate a `.meta` file for the DLL on first import. Do not hand-edit `.meta` files unless you know Unity's GUID rules.

### 2.6 Rebuild workflow (after Rust changes)

Whenever you change Rust code:

```bash
cd native/aetherweave-routing
cargo test && cargo build --release
# copy DLL/.so/.dylib again (same paths as above)
```

Return to Unity — it will detect the file change and reimport the plugin.

---

## 3. Unity project

### 3.1 Install Unity Hub and editor

1. Install **Unity Hub** from [unity.com/download](https://unity.com/download).
2. Open Hub → **Installs** → **Install Editor**.
3. Select **Unity 2022.3 LTS** and install **2022.3.50f1** if available, or the closest **2022.3.x** patch.
4. In **Add modules** for that install, include at least:
   - **Microsoft Visual Studio Community** (or use Rider separately)
   - **Windows Build Support (IL2CPP)** if building Windows standalone
   - **Mac Build Support** / **Linux Build Support** if you target those platforms

**Verify editor version matches the project**

Open `unity/AetherWeave/ProjectSettings/ProjectVersion.txt`. It should read:

```
m_EditorVersion: 2022.3.50f1
```

If Hub only has a newer 2022.3 patch (e.g. `2022.3.55f1`), that is usually fine — stay on the **2022.3 LTS** line. Do **not** open this project with Unity 6 (`6000.x`) unless you intentionally upgrade the project.

<div class="callout danger">
  <div class="callout-title">Wrong editor version</div>
  Opening <code>unity/AetherWeave</code> in Unity 6 may upgrade project files, break Entities package resolution, or create a nested template project. Always use <strong>2022.3 LTS</strong> for this repo unless the team agrees to migrate.
</div>

### 3.2 Add the correct project folder

This is the most common setup mistake.

**Correct project root (use this):**

```
<repo>/unity/AetherWeave/
```

**Wrong (do not use):**

```
<repo>/unity/AetherWeave/AetherWeave/   ← nested Unity template; no GameBootstrap scripts
```

**Steps in Unity Hub**

1. **Projects** → **Add** → **Add project from disk**
2. Navigate to `AetherWeave_Network_Tycoon/unity/AetherWeave`
3. Select the folder that contains `Assets/`, `Packages/`, and `ProjectSettings/` **directly** (not a subfolder)
4. Confirm the project card shows editor version **2022.3.x**

**How to tell you opened the right project**

In the Unity **Project** window you should see:

```
Assets/
├── Plugins/
│   ├── README.md
│   └── x86_64/aetherweave_routing.dll
└── Scripts/
    ├── AetherWeave.asmdef
    ├── Core/GameBootstrap.cs
    └── Simulation/NativeRouting.cs
```

If you only see TutorialInfo, SampleScene, and **no** `Scripts/Core/GameBootstrap.cs`, you opened the nested template — remove that Hub entry and add the parent `unity/AetherWeave` folder.

### 3.3 First open and package resolution

1. Click the project in Hub to open it in Unity 2022.3.
2. First launch may take **5–15 minutes** while Unity creates `Library/` and downloads packages.
3. Watch the bottom-right status bar: wait until it says **Idle** (not "Compiling..." or "Importing...").
4. Open **Window → General → Console** (`Ctrl+Shift+C` on Windows, `Cmd+Shift+C` on macOS).

**Packages installed from `Packages/manifest.json`**

| Package | Purpose |
|---------|---------|
| `com.unity.entities` | DOTS ECS core |
| `com.unity.burst` | SIMD jobs |
| `com.unity.collections` | Native containers |
| `com.unity.entities.graphics` | ECS rendering bridge |
| `com.unity.render-pipelines.universal` | URP |
| `com.unity.mathematics` | Math types for ECS |
| `com.unity.test-framework` | Play Mode / Edit Mode tests |

<div class="callout warn">
  <div class="callout-title">Package resolution</div>
  If the Console shows Entities or Burst errors:
  <ol>
    <li><strong>Window → Package Manager</strong></li>
    <li>Top-left dropdown: <strong>In Project</strong></li>
    <li>Click <strong>Refresh</strong> (or resolve conflicts if shown)</li>
    <li>Confirm editor is 2022.3 LTS — check <code>ProjectSettings/ProjectVersion.txt</code></li>
    <li>Close Unity, delete <code>unity/AetherWeave/Library/</code>, reopen (last resort)</li>
  </ol>
</div>

**Regenerate C# project files (for Rider / Visual Studio)**

1. **Edit → Preferences → External Tools**
2. Set **External Script Editor** to Rider or Visual Studio
3. Enable **Generate .csproj files for:** Embedded packages, Local packages, Registry packages, Git packages, Built-in packages
4. Click **Regenerate project files**

Unity creates `.sln` / `.csproj` files (gitignored) for IDE IntelliSense.

### 3.4 Configure the native plugin in Unity

After the DLL is in `Assets/Plugins/x86_64/`:

1. In the **Project** window, click `Assets/Plugins/x86_64/aetherweave_routing.dll`
2. In the **Inspector**, review **Plugin Import Settings**

**Recommended settings for Windows development (Play Mode in Editor)**

| Setting | Value |
|---------|--------|
| **Select platforms for plugin** | Uncheck "Any Platform", then enable **Editor** and **Standalone** |
| **Editor** | CPU: **x86_64**, OS: **Windows** (or your host OS) |
| **Standalone** | CPU: **x86_64**, OS: **Windows** (adjust for Linux/macOS builds) |

Click **Apply**.

**macOS Apple Silicon:** put `libaetherweave_routing.dylib` under `Assets/Plugins/arm64/`, select it, enable **Standalone** + **macOS** + **ARM64**. Intel Mac builds may need an `x86_64` folder with a x86_64 dylib — build with `rustup target add x86_64-apple-darwin` and cross-compile if needed.

**Linux:** use `libaetherweave_routing.so` in `x86_64/`, enable **Standalone** + **Linux** + **x86_64**.

<div class="callout">
  <div class="callout-title">FFI not active yet</div>
  Even with the plugin imported correctly, <code>NativeRouting.IsPluginLoaded</code> returns <code>false</code> until <code>ffi.rs</code> exports C functions and <code>NativeRouting.cs</code> declares <code>DllImport</code>. A misconfigured plugin will matter once FFI lands.
</div>

### 3.5 Create a bootstrap scene with GameBootstrap

`AetherWeave.Core.GameBootstrap` is the scene entry point. It accumulates frame time and invokes a fixed simulation step (default **every 0.25 seconds**). ECS systems and the Rust flow engine will hook into `TickSimulation()` later.

**Before you start**

- Console has **no red compile errors**
- Status bar shows **Idle**
- You are in project root `unity/AetherWeave` (scripts visible under `Assets/Scripts/`)

#### Step A — Create or open a scene

**Option 1 — New scene (recommended)**

1. **File → New Scene**
2. If prompted, choose **Basic (URP)** or **Basic (Built-in)**
3. **File → Save As…**
4. Create folder `Assets/Scenes/` if needed
5. Save as `Assets/Scenes/Main.unity`

**Option 2 — Existing scene**

Double-click any scene under `Assets/Scenes/` in the Project window.

#### Step B — Create an empty GameObject

1. In the **Hierarchy** window, **right-click** → **Create Empty**
2. Unity creates **GameObject**
3. With it selected, rename (`F2` or Inspector title):
   - Name: **`GameBootstrap`**
4. Leave **Transform** at position `(0, 0, 0)` — no mesh or renderer needed

#### Step C — Add the GameBootstrap component

With **GameBootstrap** selected in the Hierarchy:

**Method 1 — Add Component (recommended)**

1. **Inspector** → scroll down → **Add Component**
2. Search: **`GameBootstrap`** or **`Game Bootstrap`**
3. Click **Game Bootstrap (Script)** under the **AetherWeave** assembly

**Method 2 — Drag script**

1. **Project** → `Assets/Scripts/Core/GameBootstrap.cs`
2. Drag onto the **GameBootstrap** GameObject in Hierarchy or Inspector

**Expected Inspector**

```
Game Bootstrap (Script)
  Script: GameBootstrap
  Simulation Tick Seconds: 0.25
```

| Field | Default | Meaning |
|-------|---------|---------|
| **Simulation Tick Seconds** | `0.25` | Sim step interval (4 ticks/sec). Lower = finer simulation, higher CPU cost |

#### Step D — Save the scene

**File → Save** (`Ctrl+S` / `Cmd+S`). Confirm `Main.unity` (or your scene name) appears under `Assets/Scenes/`.

#### Troubleshooting — component not found

| Symptom | Fix |
|---------|-----|
| **GameBootstrap** missing from Add Component | Wrong project folder; or Console compile errors — fix scripts first |
| **"Script class can't be found"** / Missing script | Compilation failed — open Console, fix red errors |
| Component added but no **Simulation Tick Seconds** | Reimport: right-click `GameBootstrap.cs` → **Reimport** |

See also [GameBootstrap not in Add Component](#gamebootstrap-not-in-add-component-search).

### 3.6 Add the scene to Build Settings

Required for standalone builds and good practice for a single-scene MVP:

1. Open your bootstrap scene (`Main.unity`)
2. **File → Build Settings…**
3. Click **Add Open Scenes**
4. Ensure the scene is **checked** at index **0**
5. Close the window

For Editor-only testing, Play mode works without Build Settings — but CI and builds need a scene in the list.

### 3.7 Verify Play mode

1. Click **Play** (`Ctrl+P` / `Cmd+P`)
2. Select **GameBootstrap** in Hierarchy — component stays enabled
3. **Console** — no errors from bootstrap (it does not log by default yet)

**Optional: confirm ticks fire**

Temporarily edit `Assets/Scripts/Core/GameBootstrap.cs`:

```csharp
static void TickSimulation()
{
    Debug.Log("Sim tick");
}
```

Save, return to Unity, wait for recompile, press **Play**. You should see **Sim tick** roughly **4 times per second**. Remove the log when done.

**Stop Play mode:** press **Play** again or `Ctrl+P`.

---

## 4. IDE setup

Use different tools for different parts of the monorepo, or one IDE for everything.

### 4.1 JetBrains Rider (recommended for Unity + C#)

1. Install [Rider](https://www.jetbrains.com/rider/) with **Unity Support**
2. Unity: **Edit → Preferences → External Tools → External Script Editor → Rider**
3. **Regenerate project files** (see [§3.3](#33-first-open-and-package-resolution))
4. **File → Open** in Rider → select `unity/AetherWeave/`
5. Wait for indexing; open `Assets/Scripts/Core/GameBootstrap.cs` — no red squiggles on `UnityEngine`

**Debugging**

1. Unity: enter **Play** mode
2. Rider: **Run → Attach to Unity Process** → pick the Editor
3. Breakpoint in `TickSimulation()` or `Update()` — should hit on sim ticks / frames

**Rust (optional in Rider):** open `native/aetherweave-routing/` as second project or open repo root; run `#[test]` from gutter icons.

### 4.2 Visual Studio 2022 (alternative)

1. Install VS 2022 with workload **Game development with Unity**
2. Unity: set **External Script Editor** to **Visual Studio 2022**
3. **Regenerate project files**
4. Open generated `AetherWeave.sln` under `unity/AetherWeave/`
5. **Debug → Attach Unity Debugger** while Editor is in Play mode

### 4.3 rust-analyzer (VS Code / Cursor)

1. Install extension **rust-analyzer** (rust-lang)
2. **File → Open Folder** → repo root or `native/aetherweave-routing/`
3. Open `src/routing.rs` — go-to-definition should work on `Graph`, `dijkstra`, etc.
4. Terminal: `cd native/aetherweave-routing && cargo test`

**Optional workspace setting** (repo root `.vscode/settings.json`):

```json
{
  "rust-analyzer.linkedProjects": [
    "native/aetherweave-routing/Cargo.toml"
  ]
}
```

### 4.4 VS Code / Cursor (docs + JSON data)

1. Open repo root
2. Edit moddable data: `data/regions/`, `data/hardware/`, `data/ley_glass/`
3. Preview docs locally:

```bash
cd docs
bundle install
bundle exec jekyll serve --livereload
```

Open the URL Jekyll prints (usually `http://127.0.0.1:4000` + `baseurl` from `_config.yml`).

### 4.5 IDE quick reference

| Task | Tool |
|------|------|
| C# / Unity scenes / ECS | Rider or Visual Studio |
| Rust routing tests | Terminal, rust-analyzer, or Rider |
| JSON game data | VS Code / Cursor |
| This documentation site | VS Code / Cursor + Jekyll |

---

## 5. Run CI locally

GitHub Actions (`.github/workflows/ci.yml`) runs the same Rust checks as your machine:

```bash
cd native/aetherweave-routing
cargo test --verbose
cargo build --release
```

**Expected:** all 4 tests pass; release build finishes without errors.

Unity batch builds are **commented out** in CI until a Unity license / `game-ci` image is configured — see [GitHub Pages / CI]({{ '/development/github-pages/' | relative_url }}).

---

## 6. Verification checklist

Use this before opening your first feature branch:

| Check | How | Expected |
|-------|-----|----------|
| Repo cloned | `ls data native unity` | All three exist |
| Git LFS | No tiny pointer files in binary assets | Real file sizes |
| Rust tests | `cargo test` in `native/aetherweave-routing` | 4 passed |
| Release lib | File exists in `target/release/` | `.dll` / `.so` / `.dylib` |
| Plugin in Unity | `Assets/Plugins/x86_64/` (or `arm64/`) | Library present |
| Unity version | Hub + `ProjectVersion.txt` | 2022.3.x LTS |
| Correct project | Project window shows `Scripts/Core/GameBootstrap.cs` | Yes |
| Packages | Console after first open | No Entities/Burst errors |
| Bootstrap scene | Hierarchy has **GameBootstrap** + component | Sim tick field visible |
| Play mode | Press Play | No Console errors |
| CI parity | `cargo test --verbose` | Green |

---

## 7. Daily workflow

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Pull latest                                              │
│    git pull && git lfs pull                                 │
├─────────────────────────────────────────────────────────────┤
│ 2. Rust changes (if any)                                    │
│    cd native/aetherweave-routing                            │
│    cargo test && cargo build --release                      │
│    copy artifact → Assets/Plugins/<platform>/             │
├─────────────────────────────────────────────────────────────┤
│ 3. Unity                                                    │
│    Open unity/AetherWeave in 2022.3                         │
│    Play scene with GameBootstrap                            │
├─────────────────────────────────────────────────────────────┤
│ 4. Data / docs (optional)                                   │
│    Edit data/*.json or docs/*.md                            │
└─────────────────────────────────────────────────────────────┘
```

Before pushing:

```bash
cd native/aetherweave-routing && cargo test --verbose
```

---

## Troubleshooting

### `cargo` not found

Install [rustup.rs](https://rustup.rs/), then add cargo to PATH:

- **Windows:** `%USERPROFILE%\.cargo\bin`
- **macOS / Linux:** `$HOME/.cargo/bin`

Restart terminal and IDE.

### Rust tests fail

Read the failing test name — tests cover SPF path choice, link failure reroute, and congestion drops. Fix Rust code or restore graph fixtures in `routing.rs` / `flow.rs` before copying DLL to Unity. Do not merge with red CI.

### `DllNotFoundException` for routing

| Cause | Fix |
|-------|-----|
| DLL never copied | Run release build + copy to `Assets/Plugins/x86_64/` |
| Wrong CPU architecture | Rebuild for x86_64; check Plugin Import Settings |
| Wrong OS checkbox | Enable Editor + your OS in Inspector for Play Mode |
| Library name mismatch | File must be `aetherweave_routing.dll` (matches `LibName` in `NativeRouting.cs`) |

### Entities compile errors

- Use Unity **2022.3 LTS**, not Unity 6
- **Window → Package Manager → Refresh**
- Delete `Library/` and reopen project if packages are corrupted

### Git LFS pointer files instead of real assets

```bash
git lfs install
git lfs pull
```

If still broken: `git lfs fetch --all && git lfs checkout`

### GameBootstrap not in Add Component search

1. Confirm Hub project path is `unity/AetherWeave` (not nested `AetherWeave/AetherWeave`)
2. **Console** — fix all script compile errors
3. Confirm `Assets/Scripts/Core/GameBootstrap.cs` exists in Project window
4. **Assets → Reimport All** (slow; use if scripts moved)

### Opened nested `unity/AetherWeave/AetherWeave` by mistake

1. Remove wrong project from Hub
2. **Add** `unity/AetherWeave` (parent folder with `GameBootstrap.cs`)
3. Optionally delete the nested template folder after confirming you do not need its SampleScene assets — or merge any wanted scenes into the correct project manually

### Unity Play mode works but nothing happens

Expected for current MVP — `TickSimulation()` is empty. Add temporary `Debug.Log` to confirm ticks (see [§3.7](#37-verify-play-mode)).

### IDE does not resolve Unity types

**Edit → Preferences → External Tools → Regenerate project files** in Unity, then reopen `.sln` in Rider/VS.

---

## Next steps

| Topic | Doc |
|-------|-----|
| Rust API, FFI plan, fiber-cut tests | [Rust Plugin]({{ '/development/rust-plugin/' | relative_url }}) |
| ECS components, systems, sim tick design | [Unity & ECS]({{ '/development/unity-ecs/' | relative_url }}) |
| Repo layout and module map | [Repository Map]({{ '/development/repository-map/' | relative_url }}) |
| Architecture overview | [Architecture]({{ '/development/architecture/' | relative_url }}) |
| Moddable JSON | [Data Modding]({{ '/development/data-modding/' | relative_url }}) |
| Publish this docs site | [GitHub Pages]({{ '/development/github-pages/' | relative_url }}) |

When FFI stabilizes, extend `NativeRouting.cs` and add a `NativeRoutingSystem` per [Unity & ECS]({{ '/development/unity-ecs/' | relative_url }}) — call Rust from `GameBootstrap.TickSimulation()` or an ECS system on the same 0.25 s cadence.
