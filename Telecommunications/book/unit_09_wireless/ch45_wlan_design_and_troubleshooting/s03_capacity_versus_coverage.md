# 45.3 Capacity versus Coverage

Two different design problems that require opposite answers, and confusing them is why so
many wireless deployments are adequate in empty rooms and useless in full ones.

## The two questions

| | **Coverage design** | **Capacity design** |
|---|---|---|
| **Asks** | can a client **hear** an access point? | can an access point **serve** all its clients? |
| **Driven by** | **area and geometry** | **client count and demand** |
| **Wants cells** | **large** | **small** |
| **Wants power** | high | **low** |
| **Wants channel width** | wide | **narrow** (Ch 43 §43.2) |
| **AP count from** | square metres | **users** |
| Typical of | warehouses, outdoor, corridors | offices, lecture halls, stadiums |

> **Coverage design and capacity design pull in opposite directions on every parameter.**
> A design that covers a lecture theatre with one access point is a coverage design, and it
> will fail the moment the theatre is used.

## Why more access points at lower power

**The counter-intuitive result**, and it follows from three facts already established.

1. Wi-Fi is a shared medium (Chapter 44 §44.2). An access point's capacity is divided
among its active clients, so halving the clients per access point doubles each client's
share.

2. Smaller cells mean better signal (Chapter 42 §42.3), so clients run at **higher MCS**
(Chapter 44 §44.1) — which means each transfer occupies the medium for less time, and the
cell's total capacity rises more than proportionally.

**3. Airtime fairness** (Chapter 44 §44.2). Removing distant, slow clients from a cell
removes the ones consuming disproportionate airtime.

**The three compound.** Doubling the access points does not merely halve the clients per
access point — it also raises the rate at which each is served.

And the cost of getting it wrong in the other direction:

**More power does not add capacity.** It enlarges the cell, so more clients share it,
and it increases co-channel overlap so neighbouring cells contend more (Chapter 43
§43.4). Raising power in a dense deployment makes it worse, reliably.

> In capacity design, transmit power should be reduced, not raised — often to the
> minimum the coverage target permits.

## Estimating capacity

The arithmetic, and it is the part usually skipped.

### Step 1 — how many clients

Not devices owned. Devices actively using the network simultaneously.

| Space | Typical concurrent devices |
|---|---|
| Open-plan office | **2–3 per person** (laptop, phone, sometimes a tablet) |
| Lecture theatre | **1–2 per seat** |
| Conference room | 2 per seat |
| Warehouse | scanners and vehicle-mounted terminals; **count them** |
| Hotel room | 3–5 |

**And "active" needs defining.** A phone in a pocket associated and idle costs beacons and
occasional traffic; a laptop streaming video costs airtime continuously. Most estimates
use 20–40% of associated devices actively transmitting at any moment.

### Step 2 — how much each needs

| Application | Sustained |
|---|---|
| Email, web browsing | **0.5–1 Mb/s** |
| Business applications | 1–2 Mb/s |
| **Voice** | **0.1 Mb/s** (and **latency-critical**) |
| Video conferencing | **2–4 Mb/s** |
| **HD video streaming** | **5–8 Mb/s** |
| 4K streaming | 15–25 Mb/s |
| Large file transfer | as much as available |

Voice is the interesting row: negligible bandwidth and stringent latency and jitter
requirements (Chapter 41 §41.4). Capacity planning for voice is about airtime and
contention, not about megabits.

### Step 3 — what an access point delivers

Chapter 44 §44.4's reduction, and use the realistic figure:

| Band and width | Realistic aggregate |
|---|---|
| 2.4 GHz, 20 MHz | **~40 Mb/s** |
| **5 GHz, 20 MHz** | **~100 Mb/s** |
| **5 GHz, 40 MHz** | **~200 Mb/s** |
| 5 GHz, 80 MHz | ~350 Mb/s |
| 6 GHz, 80 MHz | ~400 Mb/s |

These are aggregate across all clients on that radio, not per client, and they already
account for protocol overhead.

Use the narrower figures in dense designs — because §43.2's argument says you will be
using narrow channels.

### Worked: a lecture theatre

```
   Seats:                          200
   Concurrent devices:             200 × 1.5      = 300
   Actively transmitting (30%):    300 × 0.3      = 90
   Demand per active device:                        2 Mb/s
   ─────────────────────────────────────────────────────
   Aggregate demand:               90 × 2         = 180 Mb/s

   Per AP (5 GHz, 40 MHz):                          200 Mb/s
   ─────────────────────────────────────────────────────
   By throughput:                  180 / 200      = 1 AP
```

And one access point is obviously wrong, which is the point of the next constraint.

### Step 4 — the client-count limit

Throughput is not the binding constraint in dense environments. Client count is.

| Clients per radio | Behaviour |
|---|---|
| **< 25** | **comfortable** |
| 25–40 | acceptable |
| **50** | **contention becomes significant** |
| **> 70** | **degraded regardless of bandwidth** |

Because CSMA/CA's contention overhead rises with the number of contending stations
(Chapter 16 §16.1's ALOHA analysis, Chapter 44 §44.2's efficiency), and every additional
client adds beacon-response, probe and management traffic.

**Redoing the lecture theatre:**

```
   Concurrent devices:             300
   Target per radio:               30
   ─────────────────────────────────────
   Radios needed:                  10
   Dual-band APs (5 GHz radio):    10 APs

   Cross-check against throughput: 300 × 2 Mb/s × 0.3 = 180 Mb/s
                                   10 APs × 200 Mb/s  = 2000 Mb/s  ✓ ample
```

> **Ten access points, not one.** The throughput arithmetic said one and **the client-count
> arithmetic says ten**, and the client count wins.

This is the most important idea in capacity design, and it is why lecture theatres,
auditoria and stadiums have far more access points than their area suggests.

## Making cells small

Having decided you need small cells, the mechanisms:

**Reduce transmit power.** To the minimum that meets the coverage target — often 8–14 dBm
rather than the maximum.

Raise the minimum basic rate (Chapter 44 §44.2). **The better lever**, because it shrinks
the cell without creating uplink asymmetry, and it excludes the slow distant clients that
consume airtime. 12 or 24 Mb/s is typical.

**Use directional antennas.** In a lecture theatre, patch antennas aimed at seating
sections cover the seats and not each other — which is far better than omnis that all
overlap.

**Narrow the channels** (Chapter 43 §43.2). 20 or 40 MHz, so there are enough channels for
ten access points not to contend.

Disable 2.4 GHz on most of them (Chapter 43 §43.3). With three channels, ten 2.4 GHz
radios in one room is unworkable — enable it on two, for legacy and IoT.

And under-floor or seat-back mounting in auditoria: access points beneath the seating,
covering upward into a small section, with the seats and bodies providing isolation between
cells. Unusual, expensive, and the standard approach for large venues.

## The two designs compared

**Same building, two purposes:**

| | **Warehouse (coverage)** | **Lecture theatre (capacity)** |
|---|---|---|
| Area | 5,000 m² | 300 m² |
| Users | 30 scanners | 300 devices |
| **APs** | **8** | **10** |
| Density | 1 per 625 m² | **1 per 30 m²** |
| **Power** | **maximum** | **minimum** |
| **Channel width** | 40 MHz | **20 MHz** |
| Antennas | high-gain omni or directional | **directional, sectorised** |
| Minimum rate | low, for range | **high, to shrink cells** |
| 2.4 GHz | **enabled** — scanners need it | **mostly disabled** |

Nearly every parameter is opposite, and both designs are correct for their purpose.

## Special cases

**Voice** — the requirement is not bandwidth but consistent coverage and fast roaming
(§45.2). −67 dBm everywhere, 20% overlap, 802.11r enabled, and cells sized so a handset
never runs at a low rate.

**Location tracking** — needs at least three access points hearing every point at
usable signal, which frequently means more access points than either coverage or capacity
would require, placed for geometry rather than for either.

**High-density outdoor** — stadiums and arenas. Under-seat or overhead directional
mounting, very small cells, and often 2.4 GHz disabled entirely.

**IoT at scale** — many devices, tiny traffic. Client count is the whole constraint;
bandwidth is irrelevant. **TWT** (Chapter 44 §44.1) helps substantially.

## What breaks here

A room that works empty and fails when full. Coverage design applied to a capacity
problem.

**Adding power to fix a busy area.** It enlarges the cell and adds contention. **Do the
opposite.**

One access point per room regardless of occupancy. Area-based design.

Ten access points in a theatre, all at maximum power, all overlapping. Small cells were
not actually created — power and channel width were not reduced.

Good throughput measured by one tester and complaints from users. One client alone gets
the whole cell; sixty do not. **Test under load.**

**Everything sized correctly and voice still poor.** Roaming (§45.2), or interference
(Chapter 43 §43.4) — capacity is not the only failure mode.

> **Network+ note.** Objective 2.4 expects capacity and coverage considerations. Over-learn:
> coverage design wants large cells and capacity design wants small ones; in dense
> environments the access-point count is determined by client count, not area; **reducing
> transmit power increases capacity**; and **25–40 clients per radio is the working target.**
> The "more APs at lower power" answer is counter-intuitive and examined.
