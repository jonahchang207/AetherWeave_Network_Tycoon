---
layout: default
title: Getting Started
nav: tutorial
---

# Getting Started — Your First Loom

## Chapter 1: Accept the loan

You begin in the **Verdant Shards** with a starter loan (~50,000 crowns in default data). This funds:

- Colo rack space in a local **data shrine** (data center)
- Your first **Apprentice Nexus** (entry router)
- A spool of **multi-mode Ley-Glass** for short urban runs

<div class="callout warn">
  <div class="callout-title">Tip</div>
  Do not max out your credit on long-haul fiber before you have paying customers. Transit bills arrive every month whether or not Lumina flows.
</div>

## Chapter 2: Rent colo & place a Nexus

1. Open the **World Map** and select your home shard-city.
2. Rent **colo units** (power from the Mana-Grid is included in rent at Tier 1).
3. Place a **Nexus Node** — this is your default gateway for all local traffic.

Your Nexus must have enough **switching capacity** for expected neighborhood demand. When in doubt, leave 40% headroom for festival spikes.

## Chapter 3: First last-mile connections

**Overhead Ley-Glass** on light-posts is cheap and fast to deploy:

1. Enter **cable mode** and click pole-to-pole through a residential cluster.
2. Attach **Copper-Weave** drops to individual homes (quick revenue).
3. Watch **utilization glow** on the map — yellow means plan an upgrade soon; red means drops and angry customers.

## Chapter 4: Buy transit

You are **Tier 3** until you announce your own prefixes globally. Purchase **transit** (Gbps/month) from an established guild to reach off-shard content.

- Start with the **minimum** tier that covers your peak evening traffic.
- Monitor **transit utilization** in the Financial Dashboard — oversubscribing transit is cheaper until it isn't (congestion kills SLAs).

## Chapter 5: Announce your prefix (ARP)

When you connect a second city:

1. Receive your **ASN** (story milestone).
2. Open **Topology View** → ARP panel.
3. Announce your **Incantation Prefix** to your transit provider.
4. Verify **route propagation** in the route monitor (green = reachable).

## First week goals

| Goal | Why |
|------|-----|
| 100+ connected homes | Baseline MRR |
| &lt; 1% packet loss at peak | Avoid churn |
| Pay loan installment on time | Keep credit for Phase 2 trenches |
| Scout second city | Regional growth |

## When things break

1. Click the **alert** on the map (fiber cut icon).
2. Dispatch **repair crew** — ETA shown on tooltip.
3. If SLA timer expires, send **goodwill credit** to business customers to prevent contract cancel.

Next: [Networking Tips]({{ '/tutorial/networking-tips/' | relative_url }})
