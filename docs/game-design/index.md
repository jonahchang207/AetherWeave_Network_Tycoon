---
layout: doc
title: Game Design
nav: design
section: design
---

# Game Design Bible

**Genre:** Simulation / Strategy / Tycoon  
**Title:** AetherWeave: The Loom of Connectivity  
**Audience:** Fans of deep sims (Factorio, Cities: Skylines), network engineers, and players who love logistical puzzles with narrative flavor.

## Executive summary

AetherWeave is a **highly realistic ISP and network infrastructure simulator** dressed in rich fantasy-steampunk. You begin as a scrappy local **Weaver** (startup ISP) and grow into a **Tier-1 Global Arch-Loom**, managing physical cables, routing protocols, peering agreements, and geopolitical infrastructure challenges.

## Design pillars

1. **Real networking, fantasy skin** — BGP becomes ARP; fiber is Ley-Glass; packets are Lumina — but congestion, SPF, and transit economics behave like the real world.
2. **Physical before logical** — You must lay cable, fight attenuation, and place repeaters before routing matters.
3. **Business is a weapon** — Peering and transit are as important as megabits.
4. **Scale without melting CPUs** — Flow-based simulation, not per-packet physics.
5. **Failure is content** — Fiber cuts, flares, DDoS, and festivals stress the same systems players build.

## Quick links

- [World & Lore]({{ '/game-design/world/' | relative_url }}) — Oertha, regions, the Great Silence
- [Core Mechanics]({{ '/game-design/mechanics/' | relative_url }}) — Layer 1–3, ISP business model
- [Progression]({{ '/game-design/progression/' | relative_url }}) — Four phases to Tier-1
- [UI / UX]({{ '/game-design/ui-ux/' | relative_url }}) — World map, topology view, dashboards
- [Events & Hazards]({{ '/game-design/events/' | relative_url }}) — Dynamic challenges
