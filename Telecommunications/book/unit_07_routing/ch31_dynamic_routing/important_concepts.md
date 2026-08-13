# Chapter 31 — Important Concepts

**Distance vector in one sentence** *(§31.1)* — **Tell your neighbours what you know;
believe what they tell you, plus the cost of reaching them.** A router holds a vector of
distances and **never sees the topology**.

**Information spreads one hop per round** *(§31.1)* — A network of diameter *d* takes *d*
rounds. With RIP's 30-second timer, a four-router chain takes up to **90 seconds**. This
is **Bellman–Ford**, running distributed with no coordination.

**RIP** *(§31.1)* — Metric **hop count**, max **15**, **16 = unreachable**; updates every
**30 s**; UDP **520**; multicast **`224.0.0.9`** (v2); **AD 120**. **Standardised in 1988,
six years after it was in universal use** — the specification documented what BSD had
already shipped.

**RIPv1 does not carry the mask** *(§31.1)* — So VLSM is impossible and discontiguous
networks break. This alone makes it obsolete.

**Hop count is a bad metric** *(§31.1)* — Three gigabit hops versus one 64 kb/s satellite
hop: **RIP chooses the satellite**. **A hop is not a unit of anything** — it counts
routers, and routers are not what makes a path good.

**The 15-hop limit is not a design choice** *(§31.1)* — It is **the point at which the
count-to-infinity algorithm gives up**. RIP's scale limit is a side effect of containing
its worst bug.

**What distance vector cannot see** *(§31.1)* — **A router receiving an update cannot tell
where the information came from, or whether the path loops back through itself.** It
learns a number, not a path. **"Routing by rumour."**

**The family did not die** *(§31.1)* — **EIGRP** (advanced distance vector with DUAL) and
**BGP** (path vector — distance vector carrying the full path) are its descendants. **BGP
is the most important**, and the Internet runs on it: adding an explicit path fixed
exactly the defect §31.1 identifies.

**`passive-interface` on host-facing links** *(§31.1)* — A routing protocol sent onto a
user VLAN is an information leak and an attack surface.

**Count to infinity** *(§31.2)* — `A`'s network fails; `B` advertises first, before
hearing from `A`; **`A` believes `B`'s stale claim and installs a route through it.** The
metric then climbs two at a time, forever, with traffic looping the whole while. **Nothing
is broken** — every router applies the algorithm correctly. With RIP's timers, **about
eight minutes.**

**The one-sentence cause** *(§31.2)* — **`A` believed a route that `A` itself had
originated, because `B` did not say where it had learned it.** The information had no
provenance.

**Split horizon** *(§31.2)* — **Never advertise a route back out the interface you learned
it on.** Cheap, universal, and it prevents the two-router loop outright.

**Poison reverse** *(§31.2)* — Advertise it back **at metric 16**. More assertive than
silence, which is ambiguous — an explicit "unreachable" cannot be confused with a lost
update.

**Route poisoning** *(§31.2)* — On failure, advertise metric 16 **immediately** rather
than waiting for a timeout. **An explicit withdrawal is worth far more than an implicit
timeout** — the same principle as BGP's WITHDRAW and STP's topology-change notification.

**Holddown** *(§31.2)* — Ignore new information about a failed route for 180 s, so stale
advertisements drain. **The crudest of the four** — a route that fails and genuinely
recovers is ignored for three minutes. Trades convergence for stability, exactly as
classic spanning tree's timers do.

**The fixes are heuristics, not solutions** *(§31.2)* — **Split horizon prevents
two-router loops and fails on a three-router ring**, because the route returns by a
different interface. **No amount of heuristic prevents a router from believing a route it
cannot examine.**

**How the successors fix it properly** *(§31.2)* — **Link state**: every router has the
whole topology and can see whether it is on a path. **BGP**: the advertisement carries the
full AS path, and a router seeing its own AS rejects it. **EIGRP's feasibility
condition**: accept a route only if the neighbour's reported distance is **strictly less
than your own** — because if the neighbour were routing through you, its distance would
include yours and be larger. **One inequality replacing three heuristics**, with no
timers.

**Link state's idea** *(§31.3)* — Send **your links**, flooded to **every** router; every
router holds the **complete topology** and **computes its own paths**. **Loops are
impossible by construction** in steady state.

**Consensus by identical computation** *(§31.3)* — Every router ends with the same
database and computes the tree rooted at itself. **They agree because they compute from
the same data**, not because they negotiate — the same property as Perlman's spanning
tree.

**Dijkstra** *(§31.3)* — Devised in about twenty minutes at a café in 1956. Grow a tree
outward, always taking the lowest-cost node not yet in it, relaxing its neighbours.
*O(E log V)* — microseconds for a real area, which is why full recomputation on every
change is affordable. **A router installs only the next hop**, discarding the rest of the
computed path.

**OSPF** *(§31.3)* — **AD 110**; metric **cost** from bandwidth; **IP protocol 89** with
no transport layer, so it implements its own reliability; **`224.0.0.5`** and
**`224.0.0.6`**; **hello 10 s, dead 40 s**.

**The neighbour state machine** *(§31.3)* — Down → Init → **Two-Way** → ExStart →
Exchange → Loading → **Full**. **Stuck states are diagnostic:** **Init = one-way
communication**; **ExStart = MTU mismatch** (the classic OSPF fault); **Two-Way between
non-DR routers on a LAN is normal**.

**The adjacency checklist** *(§31.3)* — Area ID, hello/dead timers, authentication, stub
flags, **MTU**, common subnet and mask, unique router IDs. Walk it when an adjacency will
not form.

**The DR** *(§31.3)* — Full mesh on a broadcast segment needs **n(n−1)/2** adjacencies —
45 for ten routers. Electing a DR reduces it to **n−1**. Highest priority (0 = never),
then highest router ID, and **it is not pre-emptive**. **Set it deliberately**, as with
the spanning-tree root.

**Configure the router ID explicitly** *(§31.3)* — Otherwise it is derived from an
interface, and if that address changes **every adjacency resets and the area recomputes.**

**LSA types** *(§31.3)* — **1** router links and **2** network, **within an area**; **3**
inter-area summaries, **from an ABR**; **5** externals, **domain-wide**, from an ASBR.
**Types 1 and 2 stay put; 3 crosses; 5 goes everywhere** — which is what makes areas work.

**`network` uses a wildcard mask** *(§31.3)* — `0.0.255.255`, not `255.255.0.0`. And
prefer the modern interface form `ip ospf 1 area 0`.

**Convergence is dominated by detection** *(§31.4)* — Physical-layer down is
**milliseconds**; **dead-timer expiry is 40 seconds**; tuned timers ~3 s; **BFD under
1 second**. The gap between the first two is Chapter 30 §30.3's problem restated: a far-end
failure with the local interface up costs 40 seconds.

**Tuned hellos are crude** *(§31.4)* — A busy router that misses three one-second hellos
drops an adjacency it should have kept, **and the resulting flap is worse than the slow
detection it replaced.** BFD is the right answer.

**There is a floor** *(§31.4)* — Even with instant detection, flooding, SPF and **FIB
installation** take time, and on a large table FIB installation is often the longest
step — which is why **loop-free alternates and fast reroute** precompute the backup next
hop, as RSTP's alternate port does.

**Why areas exist** *(§31.4)* — Not CPU or memory but **churn**: in a large flooding
domain something is always changing, so every router is always recomputing. **The problem
is not the size of the computation — it is that a change anywhere is a change
everywhere.**

**OSPF is link state within an area and distance vector between them** *(§31.4)* — Inside,
full topology. Across, an ABR advertises *"I can reach 10.1.0.0/16 at cost 20"* — a
distance, with no path.

**A link flapping in Area 2 causes Area 1 no SPF at all** *(§31.4)* — Because Area 1 never
knew that link existed. **This is Chapter 26 §26.3's aggregation argument applied to the
control plane.**

**The area rules** *(§31.4)* — **Every area must connect to Area 0**; **inter-area traffic
goes X → 0 → Y**; **Area 0 must be contiguous**. The rigidity exists because inter-area
routing is distance vector, so **hub-and-spoke makes loops structurally impossible**.
**Virtual links patch a violation and are a sign the design is wrong.**

**Area types** *(§31.4)* — **Stub** blocks Type 5; **totally stubby** blocks 3, 4 and 5,
leaving **only a default route** — the workhorse for branches, turning 800,000 routes into
one. **NSSA** is a stub with its own external connection.

**Summarisation must be configured** *(§31.4)* — Only at an **ABR or ASBR**, and only if
the addressing aggregates. **Its real value is not table size: a summary does not change
when a component changes**, so churn is contained. Add a **discard route** so traffic for
a down component is dropped rather than looping.

**Redistribution** *(§31.4)* — **Mutual redistribution at two points creates a loop** —
the routing equivalent of a switching loop, and the most damaging error in the chapter.
**Always filter; set the metric explicitly; prefer one-way; tag and filter on the tag when
two-way is unavoidable; never redistribute the full BGP table into an IGP.**
