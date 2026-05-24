---
layout: doc
title: Core Mechanics
nav: design
section: design
---

# Core Mechanics

Despite the fantasy presentation, systems map closely to real ISP operations.

## Layer 1 — Physical infrastructure

### Ley-Glass (fiber)

- Players **chart physical paths** for cables on the world map.
- **Distance → attenuation** (signal loss).
- **Repeater stations** (EDFA amplifiers) required every *X* km depending on cable type.
- Upgrades: multi-mode → single-mode Ley-Glass; later **Prismatic Multiplexing (DWDM)** for many wavelengths on one strand.

### Nexus nodes

Hardware in cities with stats that matter:

| Stat | Effect |
|------|--------|
| Switching capacity (Tbps) | Max throughput through device |
| Forwarding rate (Mpps) | Small-packet / control-plane headroom |
| Buffer size | Micro-burst tolerance before drops |
| Power consumption | Must connect to local **Mana-Grid** |

### Last mile

- **Copper-Weave** — DSL/cable-style (cheaper, lower satisfaction ceiling).
- **Direct Ley-Glass** — FTTH (expensive, best ARPU and SLA scores).

## Layer 2 & 3 — Protocols and traffic

### Archon Routing Protocol (ARP ≈ BGP)

- Expanding beyond one city grants an **Autonomous System Number (ASN)**.
- Configure ARP to announce **Incantation Prefix** blocks.
- Policy knobs (local preference, prepend, communities) arrive in later phases.

### Traffic simulation

- **Lumina** is simulated at **flow granularity** (Gbps aggregates), not individual packets.
- NPCs request services: Scrying Orb streams, Grimoire downloads, etc.
- **Congestion:** over-utilized links fill buffers → **packet loss** → SLA breaches → revenue hit.

### Interior gateway (OSPF / IS-IS style)

- Shortest-path inside your AS uses **Dijkstra SPF** on link metrics.
- Link failure triggers **fast reroute** — critical for submarine cable drama.

## ISP business model

| Tier | Description |
|------|-------------|
| **Tier 3 (local)** | Buy **transit** per Gbps from larger guilds |
| **Tier 2 (regional)** | **Peering** at Lumina Exchange Points — settlement-free when balanced |
| **Tier 1 (global)** | Everyone buys transit from **you**; you own trans-oceanic paths |

**QoS** (Phase 2+): prioritize enterprise SLAs over residential best-effort during congestion.

## Simulation constraints (technical design)

- Thousands of nodes, millions of logical flows — **flow-based fluid model** on edges.
- Deterministic saves: every port, router, cable, and policy state must round-trip.
- Hot pathfinding in **native code** (Rust plugin); policy orchestration in **Unity ECS**.
