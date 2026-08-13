# 30.4 When Static Stops Scaling

Static routing is deterministic, cheap and correct. It also fails, and it fails in a way
that is worth quantifying rather than asserted, because "use a routing protocol" is
repeated far more often than it is justified.

## The arithmetic

**Every router needs a route to every network it does not directly connect to.**

For *n* routers each with one attached network, in a full mesh where every router must
reach every network:

$$\text{route statements} = n \times (n-1)$$

| Routers | Statements | Feasible? |
|---|---|---|
| 3 | 6 | trivially |
| 5 | 20 | yes |
| 10 | 90 | tedious |
| **20** | **380** | **no** |
| 50 | 2,450 | absurd |

**And that is the easy count.** Real networks have several subnets per router, so
multiply again. A twenty-router network with four subnets each needs **1,520 route
statements**, every one hand-written, every one a place to make a typing error.

**The number is not the real problem.** A script can generate 1,520 lines. What breaks is
everything downstream of them.

## What actually breaks

### Change

**Add one router with one new subnet.** Every other router needs a new route statement.

$$\text{changes for one addition} = n - 1$$

**Nineteen configuration changes to add one device.** Each on a different router, each in
its own change window, each an opportunity to make a mistake, and **the network is in an
inconsistent state until the last one is applied** — during which some routers know about
the new subnet and some do not, so connectivity to it is partial and depends on where you
test from.

**This is the failure that matters.** Not the count, the **change cost**, and it grows
linearly with the network while the network grows too.

### Failure

**A static route does not know its path is broken.**

```
   A ──── B ──── C
   │             │
   └──── D ──────┘
```

`A` has a static route to `C`'s network via `B`. **`B` fails.**

**The path via `D` exists and works.** `A` does not use it, because nothing told `A` that
the route via `B` is no longer valid. `A`'s interface toward `B` may still be up — and if
`B` failed rather than the link, it certainly is (§30.3).

**The static route stays in the table and traffic is dropped, indefinitely, until a human
notices and intervenes.**

A routing protocol would have detected the failure in seconds and switched to `D`
automatically. **This is the argument for dynamic routing, and it is about failure
detection far more than it is about path calculation.**

Floating statics and tracking (§30.3) address the simple two-path case. They do not
address a mesh, because you would need a floating static for every alternative path
through every combination of failures — which is combinatorially worse than the original
problem.

### Consistency

**With 1,520 hand-maintained statements, some are wrong.** Not "may be" — are. Typical
findings in a network that has been run this way for a few years:

- Routes to subnets that were decommissioned
- Routes with the wrong mask, working by accident because a shorter prefix covers them
- Asymmetric configurations: one direction present, the other missing
- Routes pointing at a next hop that was renumbered
- Two routes to the same place via different paths, splitting traffic unintentionally

**And nothing detects any of it**, because a static route makes no claim that can be
checked. It is simply an assertion, and an assertion nobody verifies is a comment.

### Knowledge

**The configuration is the only documentation, and it is spread across every device.**

To answer *"how does traffic get from A to Z?"* you must read *n* configurations and
simulate the forwarding by hand. Nobody does this, so nobody knows, so **changes are made
with incomplete understanding** — which is where outages come from.

A routing protocol at least gives you a single command per router that reports what it
believes and why.

## The crossover

Where does static stop being right? Honestly:

| Network | Answer |
|---|---|
| **1 router, 1 exit** | **Static.** A default route. |
| 2–3 routers, simple topology | Static, with tracked floating statics for failover |
| **4–10 routers** | **It depends** — count the changes per addition |
| Any topology with **redundant paths** | **Dynamic**, for failure detection |
| **10+ routers** | **Dynamic** |
| Any network that will grow | Dynamic, before it needs to |

**The honest test is not the router count.** It is:

> **Does the network have more than one path between any two points?**

If it does, **something must decide which to use and must notice when one fails**, and
doing that by hand does not work. If it does not — if the topology is a tree with a single
path everywhere — static routing may be correct at surprising scale, and hub-and-spoke
networks of dozens of sites have been run this way successfully.

**The second test:** *how many configuration changes does adding one subnet require?* If
the answer is more than two, the topology has outgrown static routing regardless of its
size.

## What dynamic routing costs

Because the trade is real, and Chapter 31 should not arrive as an unmixed blessing.

| Cost | Detail |
|---|---|
| **CPU and memory** | Protocols compute; large topologies compute a lot |
| **Bandwidth** | Hellos, updates, and full refreshes |
| **Complexity** | Adjacencies, timers, areas, redistribution, metrics — each a thing to understand and to get wrong |
| **New failure modes** | Flapping adjacencies, redistribution loops, suboptimal paths from bad metrics, route leaks |
| **Less predictable** | The path is computed, so it may change without anyone doing anything |
| **A security surface** | An unauthenticated protocol accepts routes from whatever speaks it |

**That last row is worth dwelling on.** A router running OSPF without authentication will
form an adjacency with anything on the segment that speaks OSPF, and will believe what it
says. **A static route cannot be attacked over the network**; a routing protocol can.
Chapter 31 §31.4 and Chapter 62 cover the mitigations, all of which are configuration that
someone must remember.

**The failure modes are genuinely different in kind.** A static network fails
predictably — a route is wrong, and it is wrong the same way every time. A dynamic network
fails *emergently*: adjacencies flap, routes oscillate, and the symptom moves. Debugging
the second is harder, and it is why "just make it static" remains a real temptation during
an incident.

## The hybrid — what most networks actually do

**Almost nobody runs one or the other exclusively.** The common arrangement:

```
   Internet ──── [ static default ] ──── Edge router
                                              │
                                          [ OSPF ]        ← the interior
                                              │
                          ┌───────────────────┼───────────────────┐
                       Core 1              Core 2              Core 3
                          │                   │                   │
                    [ OSPF, and static routes for exceptions ]
```

| Where | What | Why |
|---|---|---|
| **To the ISP** | **static default** | one exit; nothing to compute |
| **Inside the site** | **OSPF or EIGRP** | many paths; failure detection matters |
| **To a stub branch** | **static** | one way in, one way out |
| **Blackholes, exceptions** | **static** | no protocol expresses these |
| **Backup paths** | **floating static** | cheap, and adequate with tracking |
| **Between organisations** | **BGP** | policy, not shortest path (Chapter 32) |

**Each is used where its properties fit**, and a design that uses only one is usually
either very small or wrong.

## The decision, stated plainly

> **Use static routing where the answer is known and will not change. Use dynamic routing
> where the answer must be discovered, or where it must be re-discovered when something
> fails.**

Most of the difficulty in this chapter comes from networks that were built with the first
and grew into the second without anyone noticing the transition.

## What breaks here

**Adding a subnet requires changes on every router.** The topology has outgrown static
routing.

**A redundant path that never gets used.** Nothing detected the failure. This is the
argument for dynamic routing in one symptom.

**Routes to networks that no longer exist.** Nobody removed them, and nothing complained.

**Traffic working in one direction only.** Asymmetric static configuration.

**Nobody can explain how traffic reaches a given subnet.** The configuration is the
documentation, and it is on twenty devices.

**A network that was fine and became unmanageable without any single event.** The
crossover was passed gradually.

> **Network+ note.** Objective 2.2 expects the comparison between static and dynamic
> routing, and it is a standard question. Over-learn: **static is deterministic, has no
> overhead, and does not adapt to failure**; **dynamic adapts and costs CPU, bandwidth
> and complexity**; and **the deciding question is whether redundant paths exist**, not
> the number of routers.
