# 31.2 The Count-to-Infinity Problem

Distance vector's defect, worked in full. It is worth the space because it is the clearest
example in this book of a distributed system failing not through any component being
broken but through the **composition of correct local behaviour** — and because every fix
for it is a technique that reappears elsewhere.

## The setup

Three routers in a line. `A` has a network attached.

```
   [ net ] ──── A ──── B ──── C
```

**Converged:**

| Router | Distance to net | Via |
|---|---|---|
| A | **0** | connected |
| B | **1** | A |
| C | **2** | B |

Everything is correct.

## The failure

**`A`'s link to the network goes down.**

`A` knows immediately, marks the network unreachable, and prepares to tell its
neighbours.

**But `B` speaks first.**

This is the whole problem: **`B` has not yet heard from `A`, and its next scheduled update
is due now.** `B` advertises what it currently believes:

> *"I can reach the network at cost 1."*

**`A` believes it.** `A`'s own route is gone, and here is a neighbour claiming a working
path. Adding the cost of the link to `B`:

| Router | Distance | Via |
|---|---|---|
| **A** | **2** | **B** ← wrong |
| B | 1 | A |
| C | 2 | B |

**A loop has formed.** `A` sends traffic for the network to `B`; `B` sends it to `A`.
Packets bounce between them until the TTL expires (Chapter 24 §24.4).

**And nothing is broken.** `B` advertised what it believed. `A` applied the algorithm
correctly. Every router behaved exactly as specified.

## The count

Now watch the numbers climb.

**Update 1.** `A` tells `B`: *"the network is at cost 2"* (via B, though A does not say
so). `B`'s route was via `A`, so `B` accepts the update from its next hop:

| A | B | C |
|---|---|---|
| 2 | **3** | 2 |

**Update 2.** `B` tells `A` cost 3. `A`'s route is via `B`, so `A` updates:

| A | B | C |
|---|---|---|
| **4** | 3 | **4** |

**And so on:**

| Round | A | B | C |
|---|---|---|---|
| 0 | 0 | 1 | 2 |
| 1 | **2** | 1 | 2 |
| 2 | 2 | **3** | 2 |
| 3 | **4** | 3 | **4** |
| 4 | 4 | **5** | 4 |
| 5 | **6** | 5 | **6** |
| … | … | … | … |
| n | **16** | 16 | 16 |

**The metric increases forever**, two at a time, with each router believing the other. It
is called *counting to infinity* because without a cap **the process does not terminate** —
the routers would increment indefinitely, and for the entire time, **traffic loops.**

**RIP's "infinity" is 16.** That is what the 15-hop maximum is for: it is not a design
choice about network size, it is **the point at which the algorithm gives up.**

$$\text{time to converge} = 16 \times 30\text{ s} \approx \textbf{8 minutes}$$

**Eight minutes of looping traffic**, from one link failing. This is why distance vector's
defect is not academic.

## Why it happens

The single sentence:

> **`A` believed a route that `A` itself had originated, because `B` did not say where it
> had learned it.**

**The information had no provenance.** `B`'s advertisement carried a destination and a
number, and nothing about the path. `A` had no way to detect that believing `B` meant
believing its own stale claim, reflected back.

This is §31.1's limitation producing its consequence, and every fix below either **adds
provenance** or **prevents the reflection.**

## The fixes

Four, and they are partial. Their partiality is the point.

### 1. Split horizon

**Never advertise a route back out the interface you learned it on.**

`B` learned the network from `A`, so **`B` does not tell `A` about it at all.** The
reflection cannot happen, and the two-router loop is prevented outright.

**Effective, cheap, and universal** — every distance-vector implementation does it.

**And it does not fix loops involving three or more routers**, because the route can
return by a different interface than the one it left by. §31.2's ring example below shows
it.

### 2. Split horizon with poison reverse

**Do advertise it back — with metric 16 (unreachable).**

`B` tells `A`: *"the network is at cost **infinity** via me."* Which is more assertive
than silence: it does not merely fail to offer a route, it **actively denies having one.**

**Why bother, if plain split horizon already prevents it?** Because silence is ambiguous.
A router that hears nothing may be hearing nothing because the neighbour has no route, or
because an update was lost. **An explicit "unreachable" removes the ambiguity**, and it
propagates the bad news actively rather than waiting for a timeout.

**The cost is update size** — every route is advertised on every interface, some as
poison — which on a large table is significant.

### 3. Route poisoning

**When a route fails, advertise it immediately at metric 16** rather than simply removing
it and letting neighbours time it out.

**Bad news travels fast.** Without poisoning, `B` learns the network is gone only when its
180-second invalid timer expires — three minutes during which it advertises a route that
does not work.

**The general principle appears throughout networking:** *an explicit withdrawal is worth
far more than an implicit timeout.* BGP does the same with its WITHDRAW message
(Chapter 32 §32.2); spanning tree does it with topology-change notifications (Chapter 19
§19.3).

### 4. Holddown timers

**After hearing that a route is unreachable, ignore any new information about it for a
fixed period** — 180 seconds in RIP.

The reasoning: after a failure the network is full of stale advertisements in flight, and
believing one restarts the loop. So refuse to believe anything for long enough that the
stale information has drained.

**It works, and the cost is severe.** A route that fails and genuinely recovers is
ignored for three minutes. **Holddown trades convergence speed for stability**, and it is
the crudest of the four fixes — the same "wait long enough for uncertainty to clear"
reasoning as classic spanning tree's timers (Chapter 19 §19.2), with the same drawback.

### 5. Triggered updates

**Send an update immediately on a change, rather than waiting for the 30-second timer.**

Reduces the window in which stale information circulates. **Combined with poisoning,
this is what makes RIP tolerable rather than merely functional.**

## Why the fixes are not enough

**Split horizon prevents two-router loops. It does not prevent loops around a ring.**

```
        A ──── B
        │      │
        └── C ─┘
```

`A` has the network. `A` fails.

- `B` learned it from `A`, so does not tell `A` — split horizon holds.
- **`C` also learned it from `A`**, and `C` tells `B`, because `C` did not learn it *from
  B*.
- `B` believes `C`. `B` tells `A`, because `B` learned this one from `C`.
- **Loop.**

**Every fix is a heuristic against a specific reflection pattern**, and a large enough
topology has a pattern none of them cover. Holddown catches most of what escapes, at the
cost of three minutes of unnecessary outage.

> **The fixes are patches on a design that lacks the information needed to solve the
> problem properly.** No amount of heuristic prevents a router from believing a route it
> cannot examine.

**This is the argument for link state**, and it is why §31.3 is a different approach
rather than a better distance vector.

## How the successors solve it properly

| Protocol | Mechanism |
|---|---|
| **Link state** (OSPF, IS-IS) | Every router has the **whole topology** and computes paths itself. **It can see whether it is on a path**, so it never installs one through itself. Loops are impossible by construction — during steady state. |
| **BGP** (path vector) | Every advertisement carries the **full list of AS numbers**. A router that sees its own AS in the path **rejects the route**. Provenance, made explicit. |
| **EIGRP** (DUAL) | Garcia-Luna-Aceves's algorithm. A router accepts a route only if the neighbour's distance to the destination is **strictly less than its own** — the *feasibility condition* — which guarantees the neighbour is not routing through it. **Loop-free at every instant, not merely after convergence.** |

**EIGRP's feasibility condition is the elegant one**, and it is worth stating precisely
because it solves the problem with a single arithmetic test rather than a set of
heuristics:

$$\text{accept a route via } N \iff \text{RD}_N < \text{FD}_{\text{mine}}$$

where **RD** is the neighbour's reported distance to the destination and **FD** is your
own current best. If the neighbour is closer than you are, **it cannot be routing through
you**, because if it were, its distance would include yours and would therefore be larger.

**One inequality, replacing split horizon, poison reverse and holddown**, with no timers
and no waiting. DUAL is why EIGRP converges in under a second where RIP takes minutes.

## What breaks here

**A routing loop after a link failure.** Distance vector, mid-convergence. TTL contains
it; it resolves when the count reaches infinity.

**Metrics climbing steadily in `debug ip rip`.** Counting to infinity, in progress.

**A route that failed and recovered, still unreachable for three minutes.** Holddown.
Working as designed, and the reason people dislike RIP.

**A loop that split horizon should have prevented.** Three or more routers in a ring.

**High traffic on a link with nothing legitimate on it.** Looping packets, TTL-limited.

> **Network+ note.** Objective 2.2 expects loop-prevention mechanisms in distance-vector
> protocols. Over-learn: **split horizon — do not advertise back the way you learned**;
> **poison reverse — advertise it back as unreachable**; **route poisoning — advertise
> failure immediately at metric 16**; **holddown — ignore new information for a period**;
> and **counting to infinity is why RIP's maximum is 15.** All five appear as direct
> recall items.
