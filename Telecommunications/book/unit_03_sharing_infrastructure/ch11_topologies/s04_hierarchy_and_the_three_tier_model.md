# 11.4 Hierarchy and the Three-Tier Model

Hierarchy is the general answer to scale, and it appears so many times in this book
that recognising it once saves learning it repeatedly.

## Why hierarchy scales

Three mechanisms, and they recur wherever hierarchy appears.

**Aggregation.** Traffic from many sources combines as it moves up, so upper levels
carry fewer, larger flows. This is what makes statistical multiplexing gain compound
(Chapter 9 §9.3) — a link aggregating ten thousand users needs proportionally less
headroom than one aggregating fifty.

**Summarisation.** Detail is hidden across a boundary. A router advertising
`10.4.0.0/16` instead of two hundred individual subnets has replaced two hundred
pieces of information with one. This is the same idea as postal sorting, subnetting
(Chapter 26 §26.3), and OSPF areas (Chapter 31 §31.4).

**Containment.** A failure, a broadcast storm, or a misconfiguration affects one
branch rather than everything. Blast radius is bounded by the structure.

Those three are why hierarchy is not merely tidy but necessary, and why every large
system in this book uses it — the DNS tree, IP address allocation, routing areas,
the three-tier LAN, and leaf-spine fabrics.

## The three-tier model

Cisco formalised it in the 1990s and it remains the default enterprise design.

```
                    ┌─────────┐   ┌─────────┐
      CORE          │  Core 1 │───│  Core 2 │        fast, simple, redundant
                    └────┬────┘   └────┬────┘
                    ╱    │    ╲   ╱    │    ╲
              ┌────────┐ │ ┌────────┐  │
  DISTRIBUTION│ Dist 1 │─┼─│ Dist 2 │──┘             policy, routing, aggregation
              └───┬────┘ │ └───┬────┘
                  │      │     │
            ┌─────┴──┐ ┌─┴───┐ │
   ACCESS   │ Access │ │Acc  │ │                     ports, PoE, security
            └────────┘ └─────┘ │
                 │        │
              hosts    hosts
```

### Access layer

Where devices attach. Its job:

- Provide ports, and PoE (Chapter 10 §10.1).
- Assign VLANs (Chapter 20).
- Enforce port-level security: 802.1X, port security, DHCP snooping, DAI, BPDU
  Guard — the Chapter 62 §62.4 checklist, applied here because this is the only
  layer that touches untrusted devices.
- Mark QoS at the trust boundary (Chapter 52 §52.2).

Access switches should be cheap, numerous, and dual-homed upward.

### Distribution layer

The aggregation and policy layer, and the one people most often misunderstand. Its
job:

- Aggregate access uplinks.
- **Route between VLANs** — the boundary between Layer 2 and Layer 3 in a classic
  design.
- **Summarise** addresses upward, which is where the address plan (Chapter 27 §27.4)
  pays off or fails to.
- Apply policy: access control lists, QoS, filtering.
- Provide first-hop redundancy (Chapter 56 §56.2's FHRP).

This is where the design intelligence lives. The access layer connects things and
the core moves packets fast; the distribution layer is where decisions are made.

### Core layer

Fast, simple, and redundant. Its job is to **move packets between distribution
blocks as fast as possible and do nothing else**.

The core deliberately does *not* do policy, filtering, or anything requiring
inspection, and the reason is worth stating: anything the core does, it does to all
traffic, and anything that slows or destabilises the core affects everything. Keep
it simple and let it be dull.

## Collapsed core

For smaller networks, distribution and core merge:

```
              ┌──────────────┐   ┌──────────────┐
              │  Collapsed   │───│  Collapsed   │
              │  core 1      │   │  core 2      │
              └──────┬───────┘   └──────┬───────┘
                     │                  │
              ┌──────┴───────┐   ┌──────┴───────┐
              │   Access     │   │   Access     │
              └──────────────┘   └──────────────┘
```

**Two tiers rather than three.** Fewer devices, less latency, less cost, less to
misconfigure.

**When it is correct:** a single building or campus, up to a few thousand users,
where a full three-tier design would add two devices and a failure domain for no
measurable benefit. This describes the large majority of enterprise networks, and
recommending a collapsed core is usually the right answer rather than a compromise.

**When it is not:** multiple buildings each needing their own aggregation; a
device count exceeding what the collapsed pair can support; or a requirement for
policy enforcement at a boundary the collapsed design does not create.

Chapter 72's project brief describes an organisation for which collapsed core is
correct, and a design proposing three tiers there should be asked to justify the
extra layer against a stated requirement.

## The traffic assumption, and where it broke

The three-tier model assumes **north–south** traffic: clients at the access layer
talk to servers or the Internet, so traffic flows up through distribution to core
and out. Capacity is therefore concentrated toward the core, and
**oversubscription** is deliberate at each layer — perhaps 20:1 from access to
distribution, 4:1 from distribution to core.

That assumption held while applications were monolithic.

It broke in the data centre around 2010, when applications became distributed. One
user request now triggers calls to a dozen microservices, each querying a database,
a cache and an authentication service — generating twenty or fifty internal
exchanges before a byte returns. **East–west traffic came to dwarf north–south, by
an order of magnitude.**

Under three-tier, two servers in adjacent racks may traverse access, distribution
and core — three hops up and three down — while spanning tree blocks half the
available links. The topology is optimised for a pattern that no longer exists.

**Leaf-spine** is the response (Chapter 67 §67.4): every leaf connects to every
spine, nothing connects leaf to leaf, so every server is exactly two hops from every
other. Routed rather than switched, with ECMP across all spines, so no links are
blocked.

Note what leaf-spine is, structurally: a **partial mesh** — specifically a complete
bipartite graph — rather than a tree. §11.1's cost/resilience curve, with a
different point chosen because the traffic pattern and the cost of ports both
changed.

**And note that three-tier is not obsolete.** It remains correct for campus
networks, where the traffic genuinely is north–south — users at desks reaching
servers and the Internet. Applying leaf-spine to an office building would be
expensive and pointless. The lesson is that **topology follows traffic pattern**,
and knowing which pattern you have is a requirements question.

## Hierarchy elsewhere in this book

Worth listing, because seeing the pattern once means recognising it five more times:

| Domain | The hierarchy | What it summarises |
|---|---|---|
| IP addressing | Prefix aggregation | Routes (Ch 26 §26.3) |
| OSPF | Areas connected through area 0 | Link-state detail (Ch 31 §31.4) |
| DNS | Root → TLD → domain → host | Delegated authority (Ch 39 §39.1) |
| BGP | Autonomous systems | Internal topology (Ch 32 §32.1) |
| Campus LAN | Access → distribution → core | Broadcast domains, routes |
| Data centre | Leaf → spine | Nothing — deliberately flat and routed |
| PSTN | Local → tandem → toll exchanges | Circuit routing (Ch 12 §12.1) |

The last row is worth noting: the telephone network solved the same scaling problem
with the same tool, seventy years earlier.

## What breaks here

**A three-tier design in a small building**, adding devices, latency and failure
domains for no benefit. Ask what requirement the distribution layer satisfies.

**An address plan that does not summarise** at the distribution boundary, so the
core carries every subnet individually. Chapter 31 §31.4's warning, and the reason
addressing and topology must be designed together.

**Oversubscription applied to east–west traffic.** A three-tier design in a modern
data centre saturates its uplinks with server-to-server traffic that the model never
anticipated.

**Policy in the core.** Anything the core does, it does to everything, and anything
that destabilises the core destabilises the network.

**A collapsed core with no second device.** The saving is one device and the cost is
every single point of failure in one box.

> **Network+ note.** Objective 1.6 covers the three-tier and collapsed-core models
> and expects you to identify each layer's role. Objective 3.5 covers data centre
> architecture including leaf-spine (spine-and-leaf, in CompTIA's phrasing). The
> connection worth carrying: **leaf-spine is a response to a change in traffic
> direction**, not a general improvement, and three-tier remains correct where the
> traffic is genuinely north–south.
