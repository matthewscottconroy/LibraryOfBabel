# 31.4 Convergence, Areas, and Design

Link state solves distance vector's correctness problem and creates a scale problem: every
router holds the whole topology, so every change is everyone's business. Areas are the
answer, and the design rules that follow from them are why OSPF networks look the way they
do.

## Convergence

**What must happen between a link failing and traffic flowing again:**

| Step | Time |
|---|---|
| **1. Detect** the failure | **the dominant term** |
| 2. Generate and flood the LSA | tens of ms |
| 3. Every router runs Dijkstra | ~ms |
| 4. Install the new routes in the FIB | ms to hundreds of ms |

**Detection dominates**, and it is worth being precise about why:

| Mechanism | Detection time |
|---|---|
| **Physical layer down** (fibre cut, unplugged) | **milliseconds** — the interface reports it |
| **Dead timer expiry** (default) | **40 seconds** |
| Tuned timers (hello 1 s, dead 3 s) | ~3 seconds |
| **BFD** | **under 1 second**, often ~150 ms |

**The gap between the first and second rows is the entire problem of §30.3**, restated for
dynamic routing. If the interface goes down, OSPF knows at once. If the far end dies while
the local interface stays up — which is what happens with any switch or media converter in
between — **OSPF waits 40 seconds.**

**Tuning the timers works and is crude:**

```
interface GigabitEthernet0/1
 ip ospf hello-interval 1
 ip ospf dead-interval 3
```

Detection in three seconds, at the cost of a hello every second on every link and a
network that is far more sensitive to transient CPU load — a busy router that misses three
hellos drops an adjacency it should have kept, and the resulting flap is worse than the
slow detection it was meant to fix.

**BFD is the right answer:**

```
interface GigabitEthernet0/1
 bfd interval 150 min_rx 150 multiplier 3
!
router ospf 1
 bfd all-interfaces
```

**Sub-second detection with a purpose-built mechanism**, leaving OSPF's own timers relaxed.
Chapter 30's notes give the reasoning: **failure detection should not be an accident of a
routing protocol's hello timer.**

**And there is a floor.** Even with instant detection, flooding and SPF and FIB
installation take time, and on a large network **FIB installation is often the longest
remaining step** — a router with a million routes cannot rewrite its hardware tables
instantly. This is why **loop-free alternates** and **fast reroute** exist: precompute the
backup next hop, so the switch is a table flip rather than a recomputation. The same idea
as RSTP's alternate port (Chapter 19 §19.3).

## Why areas exist

**Link state's cost is that every router in the flooding domain:**

- holds **every LSA**
- runs **Dijkstra over the whole topology**
- **recomputes on every change anywhere**

For 50 routers this is nothing. For 500 it is significant. And the binding problem is not
CPU but **churn**: in a large enough domain, **something is always changing**, so every
router is always recomputing, and the network never settles.

> **The problem is not the size of the computation. It is that a change anywhere is a
> change everywhere.**

**Areas partition the flooding domain.**

```
                        ┌─────────────────┐
                        │    Area 0       │
                        │   (backbone)    │
                        └────┬───────┬────┘
                            ABR     ABR
                    ┌────────┘       └────────┐
              ┌─────▼─────┐             ┌─────▼─────┐
              │  Area 1   │             │  Area 2   │
              └───────────┘             └───────────┘
```

**Within an area:** full link-state — every router has the complete topology and runs
Dijkstra over it.

**Between areas:** **summaries only.** An Area Border Router advertises *"I can reach
10.1.0.0/16 at cost 20"* into the backbone — a **distance-vector-style** statement.

**So OSPF is link state within an area and distance vector between them**, which is a
detail that explains several of its behaviours and is rarely stated plainly.

**What areas buy:**

| | Without areas | With areas |
|---|---|---|
| LSAs held | all of them | **your area's, plus summaries** |
| SPF runs on a remote change | **yes** | **no** — a summary changed, not a topology |
| Database size | whole network | one area |
| Failure blast radius | everywhere | **contained** |

**The second row is the important one.** A link flapping in Area 2 causes Area 1's routers
**no SPF computation at all**, because they never knew that link existed. They see only a
summary, and the summary usually does not change.

**This is exactly Chapter 26 §26.3's aggregation argument** — *isolating change matters
more than reducing table size* — applied to the control plane instead of the data plane.

## The area rules

**Rule 1 — every area must connect to Area 0.**

Area 0 is the **backbone**, and all inter-area traffic passes through it.

**Rule 2 — inter-area traffic goes Area X → Area 0 → Area Y.** Never directly, even if a
physical link exists.

**Rule 3 — Area 0 must be contiguous.** A split backbone splits the network.

**Why so strict?** To prevent loops. Because inter-area routing is distance vector
(summaries carry a cost and no path), it has distance vector's defect (§31.2). **The
hub-and-spoke constraint makes loops structurally impossible** — with all inter-area
traffic passing through a single backbone, no cycle of areas can form.

> **OSPF's rigid area topology is the price of using distance vector between areas without
> inheriting its loop problem.**

**Virtual links** patch a violation of rule 1 — tunnelling an area's connection to the
backbone through an intervening area:

```
area 1 virtual-link 10.255.0.5
```

**They work and they are a bad sign.** A virtual link means the area design does not
match the physical topology, and it is worth fixing rather than patching. They are
acceptable as a temporary measure during a migration and as a permanent fixture never.

## Area types

Reducing what an area must carry:

| Type | Blocks | Result |
|---|---|---|
| **Standard** | nothing | full LSA set |
| **Stub** | **Type 5** (externals) | ABR injects a default instead |
| **Totally stubby** | **Types 3, 4 and 5** | **only a default route** — Cisco-specific |
| **NSSA** | Type 5, but allows **Type 7** | a stub area that has its own external connection |
| Totally NSSA | 3, 4, 5, allows 7 | both |

**Totally stubby is the workhorse for branch offices.** A branch router needs to know one
thing: *how do I get out?* Giving it the full external table is pure waste.

```
   Standard area at a branch:   ~800,000 routes if BGP is redistributed
   Totally stubby:              1 route
```

**A stub area cannot contain an ASBR**, because it must not carry Type 5 LSAs — which is
what NSSA exists for.

## Route summarisation

**The other half of scaling**, and it must be configured deliberately — OSPF does not
summarise automatically.

**Only at an ABR or ASBR:**

```
router ospf 1
 area 1 range 10.1.0.0 255.255.0.0            ! inter-area, at an ABR
 summary-address 172.16.0.0 255.240.0.0       ! external, at an ASBR
```

**What it buys — beyond a smaller table:**

> **A summary does not change when a component of it changes.** A flapping /24 inside
> Area 1 is invisible outside it, because the /16 summary stays up as long as anything
> inside it is up.

**This is the strongest argument for summarisation and it is not the table size.** It
converts a network where every change propagates globally into one where changes are
contained — and Chapter 26 §26.4's insistence on planning addresses hierarchically exists
precisely so this is possible. **A network whose addressing does not aggregate cannot
summarise, and therefore cannot contain churn.**

**The trade:** summarisation hides detail, so traffic may be sent toward a summary whose
specific destination is down. The remedy is the discard route — a `Null0` for the summary
range at the ABR (Chapter 30 §30.1) — so the traffic is dropped locally instead of looping
back out.

## Design rules

The received wisdom, with the reasoning:

**50 routers per area.** Soft, widely repeated, and directionally right. Modern hardware
handles far more; the binding constraint is churn, not CPU.

**Design areas around the physical topology**, not the organisation chart. An area is a
flooding domain, and flooding follows links.

**Summarise at every area boundary.** This is what areas are *for*, and an area design
without summarisation gets the database reduction and not the churn containment.

**Make branch areas totally stubby.**

**Put the backbone where the traffic is** — the core, not an arbitrary site.

**Configure router IDs explicitly** (§31.3).

**Authenticate.** OSPF without authentication forms adjacencies with anything on the
segment that speaks it. `passive-interface default` plus MD5 or SHA authentication is the
minimum, and it is one line each.

**Set the reference bandwidth**, identically everywhere (Chapter 30 §30.2).

## Redistribution

Moving routes between protocols, and it deserves a warning.

```
router ospf 1
 redistribute eigrp 100 subnets metric 100 metric-type 1
 redistribute static subnets route-map ONLY-THESE
```

**Two mutual redistribution points between two protocols creates a loop** — routes learned
from A are redistributed into B, carried around, and redistributed back into A with a
better metric than the original. **The routing equivalent of a switching loop
(Chapter 19)**, and it is the most damaging configuration error in this chapter.

**The rules:**

- **Always filter.** A route map or prefix list on every redistribution, always.
- **Set the metric explicitly.** The defaults differ and are often wrong.
- **Prefer one-way** where possible.
- **Where two-way is unavoidable**, tag routes on the way out and filter on the tag coming
  back — so a route cannot return by the path it left.
- **Never redistribute the full BGP table into an IGP.** It will not fit and the attempt
  will bring the network down.

## What breaks here

**40-second outages on link failure.** Dead-timer detection. BFD.

**Adjacencies flapping after tuning timers down.** Too aggressive; a busy CPU misses
hellos. Use BFD rather than sub-second hellos.

**The whole network recomputing constantly.** No areas, or no summarisation, or both.

**An area not connecting to Area 0.** Rule 1. A virtual link patches it; fix the design.

**A branch router with 800,000 routes.** Not a stub area.

**Summarisation configured and nothing changed.** It only works at an ABR or ASBR, and
only if the addressing aggregates.

**A routing loop after connecting two protocols.** Mutual redistribution without filtering
or tagging.

**Traffic black-holed toward a summary.** A component is down and the summary is still
advertised. Add a discard route.

> **Network+ note.** Objective 2.2 expects OSPF areas and convergence. Over-learn:
> **Area 0 is the backbone and every area must connect to it**; **inter-area traffic
> passes through Area 0**; **an ABR joins areas and an ASBR joins OSPF to another
> protocol**; **stub areas block external routes**; and **convergence time is dominated by
> failure detection.** The area rules are examined directly.
