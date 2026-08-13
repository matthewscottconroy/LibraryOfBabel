# 11.1 The Combinatorics of Connection

Start with the naive design and watch it fail, because the failure is what every
other topology is a response to.

## Full mesh

Connect every device directly to every other device. For *n* devices:

$$\text{links} = \frac{n(n-1)}{2} \qquad \text{interfaces per device} = n-1$$

| Devices | Links | Interfaces each | Total interfaces |
|---|---|---|---|
| 4 | 6 | 3 | 12 |
| 10 | 45 | 9 | 90 |
| 50 | 1,225 | 49 | 2,450 |
| 100 | 4,950 | 99 | 9,900 |
| 500 | 124,750 | 499 | 249,500 |

The link count grows **quadratically** and the per-device interface count grows
**linearly**, and both become absurd quickly. At 500 devices you need a quarter of a
million interfaces and every device needs 499 ports.

Extend it to the roughly twenty billion connected devices now in existence and the
link count is about 2 × 10²⁰ — more than the estimated number of grains of sand on
Earth, and every device would need twenty billion ports.

So full mesh is out, permanently, and the arithmetic is the reason every network
ever built shares infrastructure.

## What full mesh buys, and why that matters

It is easy to read the above as "full mesh is a bad design". It is not. It is an
**unaffordable** design, and the distinction matters because everything else is
purchasing a fraction of its properties.

Full mesh gives you:

- **Maximum resilience.** Any single link failure affects exactly one pair of
  devices, and they can still reach each other via any third device.
- **Minimum latency.** Every path is one hop.
- **No shared resource.** No contention, no queueing for a link somebody else is
  using, no medium access control problem.
- **No single point of failure.** There is nothing whose loss disconnects anything.

Those are excellent properties. Every topology in §11.2 is a way of getting some of
them at a cost you can afford, and reading topologies as **points on a
cost/resilience curve** is far more useful than memorising which shape is which.

## Counting single points of failure

The complementary measure, and the one that matters operationally.

A **single point of failure** is a component whose loss disconnects something that
was previously connected. Counting them in a design is a five-minute exercise that
finds most resilience problems.

| Topology | Links | SPOFs | Notes |
|---|---|---|---|
| Full mesh | *n*(*n*−1)/2 | 0 | Unaffordable |
| Star | *n* | **1** — the hub | Cheap; the hub is everything |
| Bus | 1 | **the medium itself** | Any break splits the network |
| Ring | *n* | 1 link break tolerable if bidirectional | Two breaks split it |
| Dual ring | 2*n* | 0 for a single failure | SONET's answer |
| Tree | *n*−1 | **every internal node** | Cheap and fragile |
| Partial mesh | varies | design-dependent | The practical answer |

The star's arithmetic is worth noting because it is the dominant topology in
practice: *n* links for *n* devices — **linear**, not quadratic — and every device
needs exactly one interface. That is an enormous saving, and it is bought entirely
by accepting a single point of failure at the centre.

Which is why the centre is where the redundancy goes. A modern access switch has
dual power supplies and dual uplinks, not because switches fail often but because
the topology has concentrated all the risk there.

## Partial mesh: the practical answer

Real networks are neither star nor full mesh. They are **partial mesh**: enough
redundancy to survive the failures you care about, and no more.

The design question is not "how much redundancy" but **"which failures must this
survive?"** — which is a requirements question (Chapter 72 §72.1) and has an answer
that depends on what the network is for.

Consider four sites. Options:

| Design | Links | Survives |
|---|---|---|
| Star from site A | 3 | Nothing — A is a SPOF |
| Ring | 4 | Any one link failure |
| Full mesh | 6 | Any two link failures; any one site |
| Ring + one diagonal | 5 | Any one link, plus some two-link cases |

The ring costs one more link than the star and removes the single point of failure
entirely. That is usually the best marginal purchase in the list, and it is why ring
topologies dominate metropolitan carrier networks (Chapter 50 §50.2).

Going from ring to full mesh costs two more links for a smaller increment of
resilience. Whether that is worth buying is the design judgement, and the answer
depends on the cost of an outage — which Chapter 56 §56.1 insists should be stated
as a number.

## Cost is not only links

A caution, because link count is the easy metric and not the whole one.

**Interfaces cost money.** A 40 Gb/s port is not free, and a design needing four
uplinks per switch costs four ports on both ends of each.

**Paths cost more than links.** Chapter 9's economics: the trench, the duct, the
wayleave. Two logical links in the same duct cost barely more than one and provide
almost no additional resilience, because they share a fate (Chapter 56 §56.2). Two
links in *different* ducts cost double and are genuinely redundant.

**Complexity costs operationally.** A partial mesh with dynamic routing has more
states than a star, more ways to be misconfigured, and more that must be understood
at 3 a.m. Chapter 72 §72.1's operability constraint applies to topology as much as
to anything else.

The design that minimises link count is frequently not the cheapest design, and the
design that maximises resilience is frequently not the most reliable one — because
it is the one nobody understands.

## What breaks here

**Redundancy that shares a fate.** Two links in one duct; two switches on one power
circuit; two paths through one building. The SPOF count says zero and the reality
says one, and Chapter 56 §56.2 develops the enumeration discipline.

**A star whose hub has no redundancy.** Every device has one link and one dependency,
and the dependency is unprotected.

**A partial mesh nobody can reason about.** Enough links to be complicated, not
enough documentation to be understood, and a failure mode that requires tracing
paths by hand during an incident.

**Counting links and forgetting interfaces.** A design that looks cheap on a diagram
and requires a port count the budget does not support.

> **Network+ note.** Objective 1.6 expects the topology types and their
> characteristics. The framing worth carrying beyond the exam is the cost/resilience
> curve: **full mesh is the unaffordable ideal, and every other topology is buying a
> fraction of its properties.** That makes the comparison table derivable rather
> than memorised.
