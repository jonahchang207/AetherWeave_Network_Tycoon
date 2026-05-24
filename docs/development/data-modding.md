---
layout: doc
title: Data & Modding
nav: dev
section: dev
---

# Data & Modding

## JSON-first workflow

All balance lives under `data/`:

```
data/
  hardware/nexus_nodes.json
  ley_glass/cable_types.json
  regions/verdant_shards.json
```

Unity should load from `StreamingAssets` or Addressables copy at build time.

## Schema version

Each file includes `"schema_version": 1`. Bump when fields change; write migrator in C# for saves and data.

## Example: Nexus device

```json
{
  "id": "nexus_apprentice_mk1",
  "display_name": "Apprentice Nexus",
  "switching_capacity_tbps": 0.04,
  "forwarding_rate_mpps": 50,
  "buffer_mb": 64,
  "power_kw": 2.5,
  "cost_crowns": 12000
}
```

## Example: Region

```json
{
  "id": "verdant_shards",
  "starting_loan_crowns": 50000,
  "transit_cost_per_gbps_month": 2.4,
  "modifiers": {
    "attenuation_per_km_db": 0.22
  }
}
```

## Lua mods (planned)

Load order:

1. Base JSON
2. Mod Lua overlays (`mods/my_pack/override.lua`)
3. Validate merged table

Expose hooks: `on_region_load`, `on_event_fire`, `register_hardware`.

## Modding guidelines

- Use unique `id` prefixes (`modpackid_device_name`).
- Never ship paid DLC IDs in free mods.
- Document real-world Easter eggs in mod README, not in base game.

## SQLite history

Tables (planned):

- `mrr_daily(date, crowns)`
- `link_utilization(edge_id, ts, gbps)`

Prune rows older than in-game 2 years for save hygiene.
