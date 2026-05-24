---
layout: default
title: UI & UX
nav: design
---

# UI / UX Requirements

## World map (macro view)

- Stylized **3D globe / shard map** of Oertha.
- **Nodes** as cities and infrastructure sites.
- **Glowing Ley-Glass lines** — brightness ∝ traffic utilization.
- **Warning glyphs** for link failures, congestion, and SLA breaches.

## Topology view (logical)

- **2D node-graph editor** (Visio / GNS3 feel).
- Built with Unity **GraphView**.
- Configure ARP weights, OSPF/IS-IS costs, VLANs, and port policies.
- Binds to the same entity IDs used by the simulation.

## Financial dashboard

- **MRR** (Monthly Recurring Revenue)
- **Transit costs** and peering settlement (if any)
- **Hardware depreciation**
- **OpEx vs. CapEx** breakdown
- Historical charts backed by **SQLite** time series

## Optional CLI (advanced players)

- Pseudo-Cisco/Juniper command surface.
- Correct manual config grants **efficiency bonus** (faster convergence, fewer typos penalty).
- Not required for casual play — UI wizards cover 95% of tasks.

## Audio (FMOD)

- Server hum scales with Nexus CPU load.
- Aetheric storm beds fade by camera location and zoom.
- Link failure stingers and repair-complete cues for operability feedback.

## Accessibility & readability goals

- Color-blind-safe utilization gradients (don't rely on red/green alone).
- Tooltips map fantasy terms to real networking concepts for learning.
- Pause/slow sim during topology surgery to reduce panic mis-clicks.
