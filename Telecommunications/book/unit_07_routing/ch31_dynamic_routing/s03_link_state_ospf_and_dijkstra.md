# 31.3 Link State, OSPF and Dijkstra

Distance vector fails because a router believes numbers it cannot examine. Link state
fixes it by **giving every router the whole map** and letting each compute its own paths.

The change is not an improvement to distance vector. It is a different answer to the
question *"what should a router know?"*

## The idea

| | Distance vector | Link state |
|---|---|---|
| What is sent | **my distances** to everything | **my links** and their costs |
| Sent to | neighbours only | **flooded to every router** |
| What a router holds | a vector of distances | **the complete topology** |
| Path computation | trust neighbours' arithmetic | **compute it yourself** |
| Loops during steady state | possible | **impossible by construction** |
| Convergence | rounds × timer | **one flood, then one computation** |

**Three steps, and that is the protocol:**

1. **Discover neighbours** — say hello, hear back.
2. **Flood link-state advertisements** — tell *every* router in the area about *your*
   links, and pass on what you hear from others.
3. **Run Dijkstra** on the resulting database to compute the shortest path to everything.

**Every router ends with an identical database** — the same map — and each independently
computes the tree rooted at itself. Because every router computes from the same data, they
agree. **Consensus by identical computation**, which is exactly the property that made
Perlman's spanning tree work (Chapter 19 §19.2).

## Dijkstra's algorithm

Edsger Dijkstra, 1956, published 1959. He devised it in about twenty minutes at a café in
Amsterdam, to demonstrate the power of a new computer, and it is the algorithm your
router runs several times a second.

**The method — build a tree of shortest paths outward from yourself:**

```
   1. Put yourself in the tree at cost 0. All others: cost ∞.
   2. Of the nodes NOT yet in the tree, take the one with the lowest cost.
   3. Add it to the tree.
   4. For each of its neighbours not in the tree:
          if (cost of this node + link cost) < that neighbour's recorded cost:
              record the new, lower cost and the path through this node.
   5. Repeat from 2 until every node is in the tree.
```

**Worked:**

```
        A ──2── B
        │       │
        4       1
        │       │
        C ──3── D ──5── E
```

Computing from **A**:

| Step | In tree | Costs known | Chosen |
|---|---|---|---|
| 0 | {A} | B:2, C:4 | — |
| 1 | {A,B} | **B:2**, C:4, D:3 | **B (2)** |
| 2 | {A,B,D} | C:4, **D:3**, E:8 | **D (3)** |
| 3 | {A,B,D,C} | **C:4**, E:8 | **C (4)** |
| 4 | {A,B,D,C,E} | **E:8** | **E (8)** |

**Result — A's shortest-path tree:**

| To | Cost | Path | Next hop |
|---|---|---|---|
| B | 2 | A–B | **B** |
| D | 3 | A–B–D | **B** |
| C | 4 | A–C | **C** |
| E | 8 | A–B–D–E | **B** |

**Note C.** At step 1 the direct A–C link costs 4, and A–B–D–C would cost 2+1+3 = 6. The
direct path wins. **The algorithm considers both and keeps the better** — which is exactly
what distance vector cannot reliably do, because it never sees the alternatives.

**A router installs only the next hop**, not the whole path. It computes the path in order
to know which neighbour to use, then discards everything but the first step (Chapter 29
§29.1).

**Complexity:** *O(E log V)* with a binary heap. For a 200-router area with 600 links this
is microseconds — which is why a full recomputation on every topology change is
affordable.

## OSPF

**Open Shortest Path First** — "open" because it was deliberately not proprietary, in
contrast to Cisco's IGRP.

| | |
|---|---|
| Standard | RFC 2328 (v2, IPv4), RFC 5340 (v3, IPv6) |
| Type | link state |
| Metric | **cost**, from bandwidth |
| Administrative distance | **110** |
| Transport | **IP protocol 89** — not TCP or UDP |
| Multicast | **`224.0.0.5`** (all SPF routers), **`224.0.0.6`** (DRs) |
| Hello | 10 s on broadcast links, 30 s on non-broadcast |
| Dead | **4 × hello** — 40 s |

**It runs directly on IP**, with no transport layer, so it implements its own
acknowledgement and retransmission — which is Chapter 21's layering argument seen from the
other side: OSPF needs reliability with different properties than TCP's, so it builds its
own.

### Adjacencies

Routers do not simply flood at each other. They form **adjacencies**, and the state
machine is examinable:

```
   Down → Init → Two-Way → ExStart → Exchange → Loading → Full
```

| State | Meaning |
|---|---|
| **Init** | I heard a hello from them |
| **Two-Way** | They heard mine too — **bidirectional**. On a LAN, the DR election happens here. |
| ExStart | Negotiating who leads the database exchange |
| Exchange | Trading database descriptions |
| Loading | Requesting the LSAs I lack |
| **Full** | **Databases synchronised** |

**Stuck states are diagnostic**, and this is the practical value of knowing the machine:

| Stuck at | Almost certainly |
|---|---|
| **Init** | **One-way communication** — an ACL, or a unidirectional link |
| **Two-Way** | **Normal** on a broadcast network between two non-DR routers |
| **ExStart** | **MTU mismatch** — the classic OSPF fault |
| **Loading** | Packet loss, or an LSA the neighbour cannot process |

**ExStart/MTU is worth over-learning.** OSPF compares MTU during database exchange and
refuses to proceed if the two ends disagree. Since MTU mismatches are common (Chapter 24
§24.3), and since the symptom — an adjacency that reaches ExStart and stops — points
nowhere obvious, this is one of the most frequently-diagnosed OSPF problems.

**Adjacency requirements**, all of which must match:

- **Area ID**
- **Hello and dead intervals**
- **Authentication** (type and key)
- **Stub area flags**
- **MTU**
- **Subnet and mask** — the interfaces must be on a common subnet
- **Unique router IDs**

**A checklist worth memorising**, because "the adjacency won't come up" is answered by
walking it.

### The DR and BDR

On a broadcast segment with *n* routers, full-mesh adjacencies would be:

$$\frac{n(n-1)}{2}$$

**Ten routers on a VLAN would need 45 adjacencies**, each flooding to each other — an
enormous waste on a medium where one transmission reaches everyone.

**So OSPF elects a Designated Router.** Everyone forms a full adjacency **with the DR
only**, and the DR relays. Adjacencies drop from *n(n−1)/2* to *n−1*.

A **Backup DR** is elected too, fully adjacent and ready, so the DR's failure does not
require a new election and a full resynchronisation.

**Election:** highest **priority** (default 1; **0 means never**), then highest **router
ID**.

**And it is not pre-emptive** — a new router with a better priority does **not** take over
from a working DR. Which surprises people, and is deliberate: pre-emption would cause a
database resynchronisation for no benefit.

**Set priority 0 on routers that should never be DR**, and set the DR deliberately on the
routers with capacity. Leaving it to the router-ID lottery has the same character as
leaving the spanning-tree root to the MAC address lottery (Chapter 19 §19.3).

**Non-DR routers stay at Two-Way with each other**, which is why that state is normal
rather than a fault.

### Router ID

A 32-bit number identifying the router, chosen in this order:

1. Explicitly configured `router-id`
2. Highest **loopback** interface address
3. Highest active physical interface address

**Configure it explicitly.** If it is derived from an interface and that interface's
address changes — or the interface goes down before OSPF starts — the router ID changes,
**every adjacency resets, and the whole area recomputes.** Chapter 27 §27.4's loopback
convention exists partly for this.

### LSA types

The database's contents. The examinable ones:

| Type | Name | Carries | Scope |
|---|---|---|---|
| **1** | Router LSA | a router's own links | **within an area** |
| **2** | Network LSA | a broadcast segment's routers, from the DR | within an area |
| **3** | Summary LSA | **inter-area** routes, from an ABR | between areas |
| 4 | ASBR Summary | how to reach an ASBR | between areas |
| **5** | External LSA | **redistributed** routes, from an ASBR | the whole domain |
| 7 | NSSA External | externals in a not-so-stubby area | within an NSSA |

**Types 1 and 2 stay in their area. Type 3 crosses. Type 5 goes everywhere.** That
distinction is what makes areas work, and §31.4 develops it.

## Configuring it

```
router ospf 1
 router-id 10.255.0.1
 auto-cost reference-bandwidth 100000
 passive-interface default
 no passive-interface GigabitEthernet0/1
 network 10.1.0.0 0.0.255.255 area 0
 area 1 stub
!
interface GigabitEthernet0/1
 ip ospf 1 area 0                     ! the modern alternative to network statements
 ip ospf cost 10
 ip ospf priority 0
 ip ospf authentication message-digest
 ip ospf message-digest-key 1 md5 <key>
```

**`network` uses a wildcard mask** (Chapter 25 §25.3), which catches people constantly —
`0.0.255.255`, not `255.255.0.0`.

**`passive-interface default` then selectively enabling** is the right pattern: it is
safer to opt interfaces in than to remember to opt them out, and an OSPF hello on a user
VLAN is both a leak and an attack surface.

**The modern `ip ospf <process> area <n>` form on the interface** is clearer than
`network` statements and is what new configurations should use.

## Verifying

```
show ip ospf neighbor            ! the first command, always
show ip ospf interface brief     ! what is enabled, cost, DR/BDR, timers
show ip ospf database            ! the LSDB
show ip route ospf
show ip ospf border-routers
```

**`show ip ospf neighbor` first.** If the adjacency is not `FULL` (or `2WAY` where
expected), nothing else matters, and the state names the problem.

## What breaks here

**Adjacency stuck in ExStart.** **MTU mismatch.** Check both interfaces.

**Adjacency stuck in Init.** One-way communication — an ACL, a firewall, or a
unidirectional link.

**No adjacency at all.** Walk the requirements list: area, timers, authentication, subnet,
MTU, router ID uniqueness.

**Two-Way and never Full.** Normal between non-DR routers on a broadcast segment.

**All links appearing equal.** Default reference bandwidth (Chapter 30 §30.2).

**The whole area recomputing for no reason.** A router ID changed. Configure it
explicitly.

**OSPF adjacency forming with a device you do not control.** No authentication, and no
`passive-interface`.

> **Network+ note.** Objective 2.2 expects OSPF as a link-state protocol. Over-learn:
> **link state floods topology, not distances**; **every router runs Dijkstra on an
> identical database**; **AD 110**; **metric is cost from bandwidth**; **`224.0.0.5`**;
> **hello 10 s, dead 40 s**; **the DR reduces adjacencies on a broadcast segment**; and
> **the neighbour state machine**, especially that **ExStart means MTU mismatch**.
