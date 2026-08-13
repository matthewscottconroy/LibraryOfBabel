# 31.1 Distance Vector and RIP

The first routing protocols were distance vector, they are the simplest thing that works,
and their failure modes are so instructive that they are worth studying carefully even
though you will rarely deploy one.

## The idea

**Tell your neighbours what you know. Believe what they tell you, plus the cost of
reaching them.**

That is the whole algorithm. Each router:

1. Knows its **directly connected** networks, at cost 0 (or 1).
2. **Periodically sends its entire table** to each neighbour.
3. On receiving a neighbour's table, **adds the cost of that link** to every entry.
4. If the result is better than what it has, **installs it, with that neighbour as the
   next hop.**

**A router never sees the topology.** It sees only its neighbours' claims and its own
distance to those neighbours. The name is exact: each router holds a **vector of
distances**, indexed by destination.

## Convergence, traced

```
   A ──── B ──── C ──── D
```

Each router starts knowing only its own network.

**Round 0 — initial state:**

| | to A | to B | to C | to D |
|---|---|---|---|---|
| **A** | 0 | — | — | — |
| **B** | — | 0 | — | — |
| **C** | — | — | 0 | — |
| **D** | — | — | — | 0 |

**Round 1** — everyone tells their neighbours:

| | to A | to B | to C | to D |
|---|---|---|---|---|
| **A** | 0 | **1 via B** | — | — |
| **B** | **1 via A** | 0 | **1 via C** | — |
| **C** | — | **1 via B** | 0 | **1 via D** |
| **D** | — | — | **1 via C** | 0 |

**Round 2:**

| | to A | to B | to C | to D |
|---|---|---|---|---|
| **A** | 0 | 1 via B | **2 via B** | — |
| **B** | 1 via A | 0 | 1 via C | **2 via C** |
| **C** | **2 via B** | 1 via B | 0 | 1 via D |
| **D** | — | **2 via C** | 1 via C | 0 |

**Round 3 — converged:**

| | to A | to B | to C | to D |
|---|---|---|---|---|
| **A** | 0 | 1 | 2 | **3 via B** |
| **B** | 1 | 0 | 1 | 2 |
| **C** | 2 | 1 | 0 | 1 |
| **D** | **3 via C** | 2 | 1 | 0 |

**Information spreads one hop per round.** A network of diameter *d* takes *d* rounds to
converge — and with RIP's 30-second update timer, **a four-router chain takes up to 90
seconds** for a new route to reach the far end.

**This is Bellman–Ford**, the same algorithm from operations research, running
distributed with no coordination. Each router performs one relaxation step per round
against its neighbours' estimates, and the estimates converge to the true shortest paths.

## RIP

**Routing Information Protocol.** The archetype, and one of the oldest protocols still
implemented.

| | |
|---|---|
| Metric | **hop count** |
| Maximum | **15**; **16 = unreachable** |
| Update interval | **30 seconds**, full table |
| Transport | UDP port **520** |
| Destination | broadcast (v1) / **multicast `224.0.0.9`** (v2) |
| Administrative distance | **120** |
| Timers | update 30 s, invalid 180 s, holddown 180 s, flush 240 s |

**Its history is unusual.** RIP was in Berkeley's `routed` in 1982, shipped with BSD, and
was therefore on every Unix machine — **it was standardised in RFC 1058 in 1988, six years
after it was in universal use.** The specification documented what was already deployed,
which is Chapter 23's rough-consensus-and-running-code applied to a protocol nobody
designed carefully.

### The three versions

| | RIPv1 | RIPv2 | RIPng |
|---|---|---|---|
| RFC | 1058 (1988) | 2453 (1998) | 2080 (1997) |
| **Carries the mask?** | **no** | **yes** | yes |
| VLSM (Chapter 26 §26.4) | **impossible** | yes | yes |
| Delivery | broadcast | multicast `224.0.0.9` | `FF02::9` |
| Authentication | none | simple / MD5 | IPsec |
| Protocol | IPv4 | IPv4 | **IPv6** |

**RIPv1's inability to carry the mask is what makes it obsolete** (Chapter 25 §25.4).
Without a mask, a receiver must assume the classful default, so VLSM is impossible and
discontiguous networks break.

## The metric problem

**Hop count is a bad metric**, and RIP is the clearest demonstration.

```
   A ────1 Gb/s──── B ────1 Gb/s──── C ────1 Gb/s──── D
   │                                                  │
   └───────────── 64 kb/s satellite ─────────────────┘
```

| Path | Hops | Actual capacity |
|---|---|---|
| A–B–C–D | **3** | 1 Gb/s |
| A–D direct | **1** | **64 kb/s** |

**RIP chooses the satellite.** One hop beats three, and hop count cannot see bandwidth,
latency, cost, reliability or congestion.

**A hop is not a unit of anything.** It counts routers, and routers are not what makes a
path good or bad.

**The 15-hop limit** is the other half of the problem. A network wider than fifteen
routers cannot use RIP at all — and 15 was not chosen for elegance. §31.2 shows that it is
the mechanism that stops the count-to-infinity problem from running forever, so **RIP's
scale limit is a side effect of its worst bug's containment.**

## What distance vector cannot see

The deep limitation, and it drives everything in §31.2 and §31.3.

> **A router receiving a distance-vector update cannot tell where the information came
> from, or whether the path loops back through itself.**

When `B` tells `A` *"I can reach D at cost 2"*, `A` learns a number. It does not learn:

- **which routers** are on that path
- whether **`A` itself** is on that path
- whether `B` learned it **from `A` in the first place**

**All three matter**, and the third is fatal. If `B` learned the route from `A`, and `A`
now believes `B`'s advertisement, the two have created a loop by believing each other's
echo.

**This is "routing by rumour"**, and the phrase is fair: each router repeats what it heard
without any means of checking provenance.

**Link-state protocols fix it by distributing the topology instead of distances** (§31.3),
so every router can compute the path itself and can see whether it is on it. **BGP fixes
it by carrying the full AS path** (Chapter 32 §32.2), so a router can check whether its
own AS appears and reject the route if so. Both are answers to the same defect.

## What distance vector gets right

Its reputation is poor and two of its properties are genuinely good.

**It is simple.** RIP is implementable in a few hundred lines. In 1982 that mattered
enormously; on a constrained device it still can.

**Each router needs no global knowledge.** Memory is proportional to the number of
destinations, not to the topology's complexity, and no router ever holds a picture of the
network.

**And the family did not die.** Its descendants:

| Protocol | Relationship |
|---|---|
| **EIGRP** | Advanced distance vector, with DUAL — loop-free by construction, converges fast |
| **BGP** | **Path vector** — distance vector carrying the full path, which fixes the loop problem |
| **Babel, AODV, Batman** | Mesh and ad-hoc protocols, where simplicity and low state genuinely matter |

**BGP is distance vector's most important descendant**, and the Internet runs on it. The
addition of an explicit path — the thing §31.1's limitation identified as missing — turned
a flawed family into the protocol that scales furthest.

## Configuring RIP

Included because you may meet it, and because it is short.

```
router rip
 version 2
 no auto-summary                    ! ALWAYS — Chapter 25 §25.4
 network 10.0.0.0
 network 192.168.1.0
 passive-interface GigabitEthernet0/3
```

**`no auto-summary` is essential** — without it, RIPv2 summarises to the classful boundary
at major network borders and breaks discontiguous networks (Chapter 25 §25.4).

**`network` takes a classful network**, not a prefix, and enables RIP on **every interface
within it**. This surprises people: `network 10.0.0.0` activates RIP on all interfaces
with a `10.x.x.x` address, whether or not you wanted it there.

**`passive-interface`** stops sending updates on an interface while still advertising its
subnet — **use it on every interface facing hosts.** A routing protocol sent onto a user
VLAN is an information leak and an attack surface (§31.4).

```
show ip route rip
show ip protocols
debug ip rip                     ! useful, and noisy; disable it afterwards
```

## Where RIP is still right

Rarely, and not never:

- **Very small networks** where anything else is overkill
- **Constrained devices** with a few hundred kilobytes
- **Teaching** — its failure modes are visible and instructive
- **Legacy equipment** that supports nothing else

**In any other case, use OSPF.** It is universally supported, converges in seconds rather
than minutes, has a metric that reflects reality, and has no diameter limit.

## What breaks here

**RIP choosing a slow path.** Hop count. The protocol is doing what it says.

**A network wider than 15 hops.** RIP cannot express it. Change protocol.

**Discontiguous networks broken.** Auto-summary. `no auto-summary`.

**VLSM not working.** RIPv1. Use v2.

**Convergence taking minutes.** 30-second updates plus 180-second timers. This is normal
RIP behaviour and it is why RIP is not used.

**RIP running on an interface you did not intend.** The `network` statement is classful
and enables it everywhere within that classful range.

> **Network+ note.** Objective 2.2 expects RIP as a distance-vector protocol. Over-learn:
> **metric is hop count, maximum 15, 16 means unreachable**; **updates every 30 seconds**;
> **AD 120**; **RIPv1 does not carry the mask and RIPv2 does**; and **distance vector
> means routers exchange distances, not topology.** The 15/16 distinction is examined
> directly.
