---
layout: doc
title: Networking Tips
nav: tutorial
section: tutorial
---

# Networking Tips

## Capacity planning (simple)

```
Required Gbps ≈ (subscribers × average Mbps) ÷ 1000 × oversubscription factor
```

- Residential oversubscription **3:1** to **10:1** is normal early game.
- **Business SLAs** — treat as **1:1** or better on their committed path.

If utilization stays above **70%** for three peak hours, upgrade link, add parallel path, or enable QoS (Phase 2).

## Attenuation & repeaters

- Longer Ley-Glass runs need **repeater stations** (EDFAs).
- **Frost-Forge** region bonus: longer reach per repeater — still budget maintenance for ice-wyrm events.
- Running out of optical budget shows as **rising error rate** before total link death — heed yellow telemetry.

## Redundancy without bankruptcy

| Pattern | Cost | Benefit |
|---------|------|---------|
| Single path + spare cable on roll | Low | Fast repair, not failover |
| Ring topology between two cities | Medium | Automatic reroute on one cut |
| Dual transit providers | Higher opex | Survives upstream guild issues |

For Phase 1, a **ring** between your two largest revenue cities is the sweet spot.

## ARP (BGP) hygiene

- **Don't announce more specific prefixes** than you can actually deliver — blackholing traffic hurts everyone and invites regulator quests.
- **Prepend** only when you want to deprefer a path — over-prepending makes you cold potato heaven.
- At IXPs, prefer **settlement-free peering** only when traffic ratio is roughly balanced (game shows ratio meters).

## Interior routing (OSPF / IS-IS)

- Keep metrics **consistent** (don’t randomly set cost 1 on a microwave backup and cost 10 on fiber unless intentional).
- After a fiber cut, verify **reconvergence** in Topology View — traffic should shift within seconds, not minutes.

## DWDM (Phase 3)

Upgrade existing strands instead of trenching new ones when:

- Right-of-way is expensive (Zennor underground).
- You need 10× capacity on an already good physical path.

## DDoS basics (Phase 4)

1. Detect spike (scrubbing alert or abnormal Mpps on Nexus).
2. Identify targeted prefix.
3. **Blackhole** at edge if attack is huge and you lack scrubbing — or route through **Aegis** appliance if purchased.
