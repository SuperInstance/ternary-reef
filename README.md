# ternary-reef — Coral reef ecosystem pattern for long-lived collective intelligence

Models how a fleet develops persistent structures over time. Slow-growing coral frameworks, individual polyp agents, symbiotic energy relationships, spatial zoning, and stress responses (bleaching events).

## Why This Exists

Fleet intelligence isn't just about individual agents — it's about the structures they build together over time. Coral reefs are a natural metaphor: polyps (agents) build coral (persistent structures) that outlast any individual, share energy through symbionts, partition into depth zones, and experience bleaching events (stress) that can destroy or strengthen the colony. This crate makes that metaphor concrete and testable.

## Core Concepts

- **Reef** — The top-level ecosystem. Contains all corals, polyps, symbionts, and zones. Think of it as the fleet itself.
- **Coral** — A slow-growing persistent structure. Grows through stages: Seedling → Juvenile → Adult → Ancient. Gains resilience at each stage. Built by polyps.
- **Polyp** — An individual agent. Has health (Healthy/Stressed/Bleached/Dead), energy, and can be bound to a coral structure, a zone, and a symbiont.
- **Symbiodinium** — An endosymbiont that provides energy to its host polyp. Think of it as a resource provider or service dependency.
- **ReefZone** — A spatial partition by depth and light level. Zones have coral capacity limits and provide energy bonuses based on light.
- **BleachingEvent** — A stress event that can damage polyps, expel symbionts, and damage coral structures. Severity ranges from Mild to Severe.

## Quick Start

```toml
[dependencies]
ternary-reef = "0.1"
```

```rust
use ternary_reef::*;

let mut reef = Reef::new("fleet-alpha");

// Create infrastructure
let zone = reef.create_zone("shallow", 10, 80, 20);
let coral = reef.create_coral();
let polyp = reef.create_polyp();
let symbiont = reef.create_symbiont(15);

// Wire everything together
reef.zone_mut(zone).unwrap().add_coral(coral).unwrap();
reef.coral_mut(coral).unwrap().add_polyp(polyp);
reef.polyp_mut(polyp).unwrap().bind_symbiont(symbiont);
reef.symbiont_mut(symbiont).unwrap().bind(polyp);

// Feed cycle: symbionts provide energy to hosts
reef.feed_cycle();

// Coral grows over time
reef.coral_mut(coral).unwrap().grow(50);

// Handle a stress event
let event = BleachingEvent::new(BleachSeverity::Severe);
let result = reef.trigger_bleaching(&event);
println!("Bleached: {}, Expelled: {}", result.bleached_polyps.len(), result.expelled_symbionts.len());
```

## API Overview

| Type | Description |
|------|-------------|
| `Reef` | Top-level ecosystem with factories for all entities |
| `Coral` | Persistent structure with growth stages and resilience |
| `Polyp` | Individual agent with health, energy, and bindings |
| `Symbiodinium` | Energy provider bound to a host polyp |
| `ReefZone` | Spatial partition with depth, light, and coral capacity |
| `BleachingEvent` | Stress event with severity levels |
| `GrowthStage` | Seedling, Juvenile, Adult, Ancient |
| `PolypHealth` | Healthy, Stressed, Bleached, Dead |

## How It Works

The Reef maintains HashMaps of all entities, each identified by auto-incrementing IDs. The core lifecycle:

1. **Setup** — Create zones, corals, polyps, and symbionts. Bind symbionts to polyps and polyps to corals.
2. **Growth** — `Coral::grow()` accumulates growth points. When points exceed the threshold, the coral advances a stage and resets. Each stage increases resilience and the next threshold.
3. **Feeding** — `Reef::feed_cycle()` iterates all symbionts, transferring their energy output to host polyps. This is the main energy loop.
4. **Stress** — `Reef::trigger_bleaching()` applies stress to all polyps (may bleach or kill), expels symbionts from affected hosts, and damages corals (reduced by resilience).
5. **Recovery** — `Reef::recovery_cycle()` lets polyps spend energy to recover from stressed/bleached states. Dead polyps cannot recover.

## Known Limitations

- Growth is linear — no branching, no specialization, no competitive dynamics between corals.
- Bleaching is applied uniformly across all polyps. Real reefs have spatial variation.
- No migration — polyps can't move between zones or corals after placement (they can be detached/reattached manually, but there's no automatic migration logic).
- Energy model is simplified: flat output from symbionts, flat consumption by polyps. No circadian cycles or seasonal variation.
- No reproduction or spawning of new polyps/corals during runtime.

## Use Cases

- **Fleet knowledge persistence** — Coral structures represent shared knowledge bases that grow over time, outlasting individual agent sessions.
- **Service dependency modeling** — Symbiont-polyp relationships model how agents depend on energy/resource providers. Bleaching models provider outages.
- **Spatial fleet organization** — ReefZones partition the fleet by capability or load, with different resource profiles per zone.
- **Stress testing** — Bleaching events provide a structured way to simulate and measure fleet resilience.

## Ecosystem Context

Part of the SuperInstance ternary fleet ecosystem. Complements `ternary-harbor` (agents dock at harbors within reef zones), `ternary-beacon` (agents discover each other within reef structures), and `ternary-room` (rooms may contain reef ecosystems). The reef is the long-term memory layer; harbors handle short-term docking.

## License

MIT
