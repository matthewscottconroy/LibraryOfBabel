# 72.2 From Requirements to Topology

**The derivation, and the ordering matters** — **each stage's output is the next stage's input,
and a stage performed out of order produces a design that must be revisited.**

```
   Requirements (§72.1)
        │
   1.  Traffic matrix        who to whom, how much, how sensitive
        │
   2.  Sites and scope       what is local, what crosses a WAN
        │
   3.  Topology              the shape, from the matrix and the availability target
        │
   4.  Media and capacity    from the matrix plus headroom
        │
   5.  Devices               from capacity, port count and features required
        │
   6.  Physical              rooms, power, cooling, cabling
        │
   Addressing, services, security (§72.3) — designed with, not after
```

## 1. The traffic matrix

**§72.1's first question, tabulated.**

| From | To | Volume | Sensitivity | Notes |
|---|---|---|---|---|
| Staff (200) | **Cloud SaaS** | **1.5 Mb/s each, bursty** | latency | **the majority** |
| Design (12) | **Local file server** | **5 GB transfers, continuous** | **bandwidth** | on-site |
| Warehouse (30 scanners) | **Local WMS** | **trivial** | **availability, roaming** | Chapter 45 §45.2 |
| All | **Voice** | **0.1 Mb/s each** | **jitter, loss** | Chapter 52 §52.1 |
| Branch (4 sites) | **HQ + cloud** | 20 Mb/s each | mixed | |

**And three things fall out of it immediately:**

**Where the bandwidth is.** **The design team's 5 GB transfers are local**, so **they size the
LAN and not the WAN** — **which is the observation that prevents buying a circuit for a problem
that is not on the circuit.**

**Where the sensitivity is.** **The scanners need availability and roaming, not bandwidth**;
**voice needs jitter, not bandwidth.** **A design optimised for throughput serves neither.**

**And where the boundaries are.** **The warehouse talks only to the WMS; the design team only to
their server** — **which are segmentation boundaries the requirements produced rather than the
security team imposed** (§72.3).

## 2. Sites and scope

**What is local, what crosses a WAN, and what is in a cloud** (Chapter 14).

**The question that determines it:** **for each flow in the matrix, where are the two ends?**

> **A flow that stays within a building is a LAN problem. A flow that crosses a WAN is a cost
> problem** (Chapter 51 §51.1). **A flow to a cloud is both, plus egress** (Chapter 69 §69.1).

**And the decision that follows is placement:** **should the design team's file server be local
or in a cloud?**

| | Local | Cloud |
|---|---|---|
| 5 GB transfer at LAN speed | **~40 s at 1 Gb/s** | **~13 min at 50 Mb/s** |
| Cost | capital | **operational, plus egress** |
| Availability | **your problem** | the provider's |
| Backup | **your problem** | easier |

**Which is not a networking decision and the network engineer must be in it**, because **the
answer changes the WAN's sizing by an order of magnitude.**

## 3. Topology

**From the matrix and the availability target** (Chapter 56 §56.1), **and the choices are few.**

| Requirement | Topology |
|---|---|
| **One site, under ~200 users** | **collapsed core: two switches doing routing and distribution** (Chapter 11) |
| **One site, larger or multi-building** | **three-tier, or a small fabric** |
| **Data centre with east–west traffic** | **leaf–spine** (Chapter 67 §67.4) |
| **Branches to HQ** | **hub and spoke, or SD-WAN** (Chapter 51) |
| **Branches to cloud** | **local breakout** (Chapter 51 §51.2) |

**And the redundancy is derived from the cost per hour** (§72.1's question 2):

| Cost/hour | Design |
|---|---|
| **£50** | **one of everything; spares on a shelf** |
| **£900** | **redundant core; single links to edge; next-business-day support** |
| **£4,000** | **redundant everything on the critical path; two circuits; out-of-hours cover** |
| **£40,000** | **two sites** |

> **Which is Chapter 56 §56.1's arithmetic, applied as a design rule rather than as an
> analysis** — **and stating it this way is what makes the redundancy defensible** (§72.4).

**And Chapter 56 §56.2's shared-fate question applies at every redundant pair:** **two switches
in one rack on one UPS, two circuits in one duct, two firewalls with one configuration error.**

## 4. Media and capacity

**From the matrix plus headroom.**

**The worked example, for the 200-person office:**

| | |
|---|---|
| **Video calls**, 25% concurrent at 2.5 Mb/s | **125 Mb/s** |
| **Voice**, 10% concurrent at 0.1 | 2 Mb/s |
| **General**, 200 at 1.5 average | **300 Mb/s** |
| **Sum** | **427 Mb/s** |
| **With 40% headroom** | **~600 Mb/s** |
| **Order** | **500 Mb/s symmetric, or 1 Gb/s if the price step is small** |

**And the upstream is sized first** (Chapter 51 §51.4): **video is symmetric, and an asymmetric
service's upload is the number that fails.**

**Growth, at 15% in users and 40% in traffic:**

| Year | Users | **Traffic** |
|---|---|---|
| 0 | 200 | **427 Mb/s** |
| 2 | 264 | **837 Mb/s** |
| **3** | **304** | **1.17 Gb/s** |
| 5 | 402 | **2.30 Gb/s** |

> **The circuit ordered today is exhausted in year two and the design should say so** — **with
> the date, and with what the upgrade costs.** **A design that does not state when it expires is
> a design that will be described as having failed.**

**Media, from distance and rate** (Chapter 10):

| Run | Medium |
|---|---|
| **Desk to comms room, under 90 m** | **Cat6a** — and 100 m includes the patches (Chapter 65 §65.1) |
| **Between comms rooms** | **OM4 multimode**, or single-mode if the distance or the future rate warrants |
| **Between buildings** | **single-mode**, always — the cost difference is the optics and the fibre outlives them |
| **Above 100 m of copper** | **fibre; there is no other answer** |

> **Install single-mode between buildings even where multimode would suffice today.** **The
> fibre will be in the ground for twenty-five years and the optics will be replaced four
> times** (Chapter 50 §50.3's argument, at building scale) — **and it is the cheapest decision
> in this section.**

## 5. Devices

**From capacity, port count and features — in that order.**

**Port count, for the example:**

| | |
|---|---|
| 200 staff at 1.3 ports each (desk, phone) | 260 |
| **Printers, APs, cameras, building systems** | +40 |
| **Spare capacity, 20%** | +60 |
| **Total** | **~360 → 8 × 48-port switches** |

**And the features that must be specified rather than assumed:**

| | Why |
|---|---|
| **PoE budget** | **not port count** (Chapter 56 §56.3) — 48 APs at 25 W is 1,200 W |
| **Uplink capacity** | **oversubscription** (Chapter 67 §67.4) |
| **Layer 3, or not** | which determines where routing happens |
| **The hardening features** | **BPDU guard, DHCP snooping, DAI, 802.1X** (Chapter 62 §62.4) — and cheap switches lack them |
| **Stacking or MLAG** | Chapter 56 §56.2 |
| **Management interface** | **an API, or a CLI** (Chapter 70 §70.2) |
| **EOL date** | **Chapter 55 §55.3 — before purchase, not after** |

> **The commonest device-selection error is buying on port count and price and discovering the
> PoE budget, the uplink capacity or the absent security features afterwards.**

## 6. Physical

**The stage that is skipped and then constrains everything** (Chapter 56 §56.3).

| | |
|---|---|
| **Comms room locations** | **within 90 m of every desk** — which determines how many |
| **Rack space** | **including the growth from stage 5** |
| **Power** | **circuit capacity, and PoE is part of it** |
| **Cooling** | **and airflow direction** (Chapter 53 §53.2) |
| **Riser and duct routes** | **and whether the "diverse" ones are** (Chapter 56 §56.2) |
| **Out-of-band access** | **Chapter 60 §60.4 — designed in, or retrofitted expensively** |

**And the 90 m constraint is the one that shapes buildings:** **it determines the number and
placement of comms rooms**, and **it is discovered late by anyone who designs the logical
network first.**

## Headroom, deliberately

**§72.1 said the response to unstated requirements is designed-in headroom.** **Where to put
it:**

| | Headroom | Because |
|---|---|---|
| **Address space** | **50%** | **renumbering is a project** (Chapter 27 §27.2) |
| **Ports** | **20%** | growth, and moves |
| **Rack units** | **25%** | **and a full rack cannot accept a replacement** (Chapter 53 §53.2) |
| **Power** | **30%** | Chapter 56 §56.3 |
| **Bandwidth** | **40%** | bursts, and Chapter 54 §54.1's averaging |
| **Uplink capacity** | **a spare port on every switch** | for the link you did not plan |

> **Headroom is the cheapest thing in a design at the time of building and the most expensive to
> retrofit**, and **the argument for it is that every network in this book's experience needed
> more of something within three years.**

## What breaks here

**A circuit bought to fix a problem that was on the LAN.** **The traffic matrix would have shown
it.**

**A design optimised for throughput serving voice and scanners badly.** **The sensitivity
column.**

**A WAN sized for today and exhausted in eighteen months.** **The growth rate**, and the design
should have stated the date.

**Multimode between buildings, and a rate increase requiring new fibre.** **The cheapest
decision, made wrongly.**

**48 access points and a switch that powers 15 of them.** **PoE budget, not port count.**

**A comms room 130 m from the far desks.** **The 90 m constraint**, discovered after the building
work.

**Two "diverse" circuits in one duct.** **Chapter 56 §56.2**, and it is a question to ask during
design rather than during the outage.

**A design with no spare anything.** **The unstated requirements have nowhere to go.**

> **Network+ note.** Objective 1.6 and 3.1. Over-learn: **topologies — star, mesh, hybrid, spine
> and leaf, three-tier, collapsed core**; **the 100 m copper limit**; **single-mode for long
> distances and multimode for short**; **PoE budgets and standards**; and **capacity planning
> uses baselines and growth projections.** The topology-to-requirement mapping is examined, and
> the derivation order is what makes it defensible.
