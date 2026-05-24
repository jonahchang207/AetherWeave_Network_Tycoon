---
layout: default
title: Events & Hazards
nav: design
---

# Events & Hazards

Dynamic events test **resilience** of the same systems players build — not separate mini-games.

## Fiber cuts

- Construction golems sever lines.
- Dispatch **repair crews** (time + cost).
- Downtime → **lost MRR** and SLA penalties until restored.
- **Design intent:** Teach redundancy and maintenance budgets.

## Aetheric flares (solar flares)

- Temporarily worsen **SNR** on long-haul optical links.
- Modulation rates drop → effective capacity falls.
- **Counterplay:** Pre-provisioned headroom, temporary traffic engineering, customer comms.

## DDoS — Demonic Denial of Service

- Malicious flood toward specific nodes.
- **Blackhole routing** (sacrifice victim prefix to save core) vs. **Aegis Scrubbing** hardware (expensive).
- Teaches **anycast**, scrubbing centers, and transit filtering.

## Festivals & world events

- Planet-wide Scrying Orb broadcast → **massive traffic spike**.
- Tests **peering limits**, IX capacity, and CDN-like caching investments.
- **Counterplay:** Scale transit temporarily, QoS business tier, festival sponsorship revenue.

## Regional hazards (recap)

| Region | Hazard |
|--------|--------|
| Verdant Shards | Wildlife / golem cable damage |
| Frost-Forge | Avalanche, ice-wyrm cuts |
| Dune Seas | Aetheric storms (loss without shielding) |
| Abyssal Trench | Ship accidents, repeater faults |

## Event design rules

1. Every hazard maps to a **real ops playbook**.
2. Give **warning telemetry** (NOC alerts) before hard fail when possible.
3. Recovery should reward **preparation**, not only reflex.
