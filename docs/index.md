---
layout: default
title: Home
nav: home
---

<section class="hero">
  <h1>AetherWeave</h1>
  <p class="tagline"><em>The Loom of Connectivity</em> — a network-realistic ISP tycoon set in the shattered world of Oertha. Rebuild the Ancient Loom with Ley-Glass, Archon routing, and ruthless business sense.</p>
</section>

<div class="hero-cards">
  <article class="card">
    <h3>Game Design</h3>
    <p>Lore, regions, mechanics, progression phases, UI vision, and dynamic hazards — the full design bible from PRD to playable fantasy.</p>
    <a class="btn" href="{{ '/game-design/' | relative_url }}">Read design</a>
  </article>
  <article class="card">
    <h3>Tutorial</h3>
    <p>How to play from your first loan to Tier-1 Arch-Loom status. Networking tips, economy advice, and event playbooks.</p>
    <a class="btn" href="{{ '/tutorial/' | relative_url }}">Start learning</a>
  </article>
  <article class="card">
    <h3>Dev</h3>
    <p>Clone to ship: Unity DOTS, Rust routing plugin, data pipeline, CI, and a phased roadmap from this repository to Steam.</p>
    <a class="btn" href="{{ '/development/' | relative_url }}">Open dev guide</a>
  </article>
</div>

## What is AetherWeave?

You are the founder of a new **Guild of Weavers** — a startup ISP in a fantasy-steampunk world where **Ley-Glass** (fiber optics) carries **Lumina** (data). The game looks like magic; the mechanics obey real networking:

- **Layer 1** — Physical cables, attenuation, repeaters, DWDM upgrades
- **Layer 2/3** — VLANs, routing, the **Archon Routing Protocol (ARP)** — our BGP
- **Business** — Transit costs, IXPs, peering, and the path to **Tier-1**

## Documentation map

| Section | Audience | Contents |
|---------|----------|----------|
| [Game Design]({{ '/game-design/' | relative_url }}) | Designers & curious players | World, mechanics, progression, UI, events |
| [Tutorial]({{ '/tutorial/' | relative_url }}) | Players | Getting started, tips, advanced play |
| [Dev]({{ '/development/' | relative_url }}) | Engineers | Setup, architecture, Rust/Unity, roadmap |

<div class="callout">
  <div class="callout-title">Repository status</div>
  The project is in <strong>Phase 1 scaffolding</strong>: Rust SPF + flow simulation, Unity project skeleton, JSON game data, and GitHub Actions for routing tests. Gameplay systems are documented here before they exist in-engine — follow the Dev tab to implement them in order.
</div>
