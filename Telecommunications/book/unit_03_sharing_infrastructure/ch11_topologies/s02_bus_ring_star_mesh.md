# 11.2 Bus, Ring, Star, Mesh

The classical shapes, with their real instances and their honest advantages. Two of
them are usually dismissed too quickly.

## Bus

One shared medium; every station taps onto it; terminated at both ends.

```
  ═╤═══════╤═══════╤═══════╤═══════╤═
   │       │       │       │       │
   A       B       C       D       E
  term                            term
```

**Instances:** 10BASE5 ("thicknet", vampire taps into a rigid coaxial cable),
10BASE2 ("thinnet", BNC T-connectors), and — still very much alive — the industrial
and automotive buses: **CAN bus** in every vehicle, Modbus RTU, PROFIBUS, and the
RS-485 multidrop links throughout building automation.

**Advantages:** minimal cable — one run serves everybody, which mattered enormously
when cable was the dominant cost. Simple to extend by tapping. No active central
device.

**Disadvantages, and they are what killed it in the LAN:**

- **One shared collision domain.** Everybody hears everybody, bandwidth is divided,
  and a medium access protocol is mandatory.
- **A break anywhere splits the network** into two unterminated halves, both of
  which reflect (Chapter 10 §10.2) and neither of which works.
- **A missing or failed terminator takes down everything.**
- **Fault isolation is a walk along the cable.** With forty taps and one bad
  connector, there is no diagnostic short of bisection.
- **Changes disrupt everyone**, because adding a station means breaking the bus.

The last three are operational rather than electrical, and Chapter 10 §10.2 argued
that operational characteristics are what actually decide media and topology
questions. Bus lost the LAN for exactly those reasons.

**Where it survives, and why:** CAN bus in a car connects perhaps seventy nodes over
a few metres, at rates in the hundreds of kilobits, in an environment where
minimising wiring weight is a genuine engineering objective and where the whole
harness is installed once and never changed. Every disadvantage above is either
irrelevant or acceptable there. The topology is not obsolete; its former
application is.

## Ring

Each station connects to exactly two others, forming a closed loop. Data circulates
around it.

```
        A ─── B
        │     │
        E     C
        │     │
        └─ D ─┘
```

**Instances:** Token Ring (IEEE 802.5), FDDI, and — the important one —
**SONET/SDH**, which is the architecture of carrier metropolitan networks
worldwide.

**Advantages, which are usually undersold:**

- **Deterministic access.** In a token-passing ring, a station holding the token
  transmits without contention, so the maximum delay before any station can transmit
  is **bounded**. Ethernet cannot promise this. For industrial control and for
  voice, a bounded worst case is worth more than a better average.
- **Graceful degradation under load.** As offered load rises, a token ring's
  throughput approaches its capacity and stays there. Ethernet's CSMA/CD throughput
  *falls* under heavy load as collisions multiply, which is why the two behave so
  differently at saturation.
- **Fault tolerance with a dual ring.** Two counter-rotating rings survive any
  single failure by wrapping at the two adjacent nodes.

**Disadvantages:**

- **A single ring breaks entirely on one failure.**
- **Latency accumulates**, since data passes through every intermediate station.
- **Adding a station disrupts the ring** unless bypass relays are fitted.
- **Token management is complex** — token loss, duplicate tokens, and a monitor
  station to detect both.

**SONET's dual ring** deserves emphasis because it is the counter-example to the
usual dismissal. Two counter-rotating fibre paths; on a cut, the two adjacent nodes
**wrap** the traffic back the other way; the switchover completes in **under
50 milliseconds** — fast enough that a telephone call does not drop and a human does
not perceive it.

That number was a design requirement, met in 1988. Spanning tree (Chapter 19)
originally took 30 to 50 *seconds* to do the equivalent, and Ethernet-based networks
did not approach 50 ms until link aggregation and fast reroute matured in the 2010s.
Anyone tempted to treat ring topologies as quaint should sit with a three-orders-of-
magnitude advantage that stood for twenty years.

## Star

Every station connects to a central device.

```
          A
          │
     B ───H─── C
        ╱   ╲
       D     E
```

**Instances:** every switched Ethernet network, every Wi-Fi cell (the access point
is the centre), every PON (the splitter is the centre), and essentially every LAN
built since 1990.

**Advantages:**

- **Fault isolation.** One cable fails, one station is affected. This is the
  property that beat bus.
- **Changes are non-disruptive.** Adding a station affects nobody.
- **Central point for management, monitoring and policy.** Every frame passes
  through one place, which is where you put the port security, the mirroring, and
  the statistics.
- **Per-station bandwidth**, once the centre is a switch rather than a hub
  (Chapter 17).

**Disadvantage:** the centre is a single point of failure, and it is a total one.

Which is precisely why redundancy is concentrated there: dual power supplies, dual
supervisors, stacking, and dual uplinks from every access switch. The topology
concentrates the risk, so the engineering concentrates the mitigation.

## Tree

Stars of stars — a hierarchy. Every enterprise network is one.

```
            core
           ╱    ╲
      dist        dist
     ╱    ╲      ╱    ╲
   acc    acc  acc    acc
```

**Advantages:** scales by adding levels; aggregates traffic naturally; matches how
buildings are physically laid out; and permits summarisation at each level, which
is the argument §11.4 develops.

**Disadvantage:** **every internal node is a single point of failure for everything
beneath it.** A distribution switch's failure isolates every access switch under it.

The remedy is dual-homing every level — each access switch to two distribution
switches, each distribution pair to two core switches — which converts the tree into
a partial mesh at the cost of doubling the uplinks and introducing loops that
Chapter 19 must then manage.

## Hybrid, and what real networks are

No real network is one shape. A typical enterprise is:

- **Star** from each access switch to its attached devices
- **Tree** from access through distribution to core
- **Partial mesh** between core devices and between sites
- **Ring** for the metropolitan fibre the carrier provides
- **Bus** for whatever industrial control system is in the plant room

And the wireless portion is a star in physical topology and a shared medium in
logical topology, which is §11.3's subject.

## The comparison, assembled

| | Bus | Ring | Star | Tree | Full mesh |
|---|---|---|---|---|---|
| Links for *n* nodes | 1 | *n* | *n* | *n*−1 | *n*(*n*−1)/2 |
| Single point of failure | the medium | 1 link (single ring) | the hub | every internal node | none |
| Bandwidth | shared | shared (token) | **per-station** | per-station | dedicated |
| Deterministic access | no | **yes** | yes (switched) | yes | yes |
| Fault isolation | **poor** | poor | **good** | good | excellent |
| Add a station | disruptive | disruptive | **trivial** | trivial | *n* new links |
| Cost | **lowest** | low | low | moderate | prohibitive |
| Modern instance | CAN, industrial | SONET, metro | switched LAN | enterprise | core, DC fabric |

## What breaks here

**Bus:** a break, a bad terminator, or one faulty tap takes down everything, and
finding which requires bisection.

**Single ring:** one failure splits it. Dual ring or nothing, in any application
where availability matters.

**Star:** the hub. Concentrate the redundancy there or accept the exposure
explicitly.

**Tree:** a distribution-layer failure isolates a whole branch. Dual-home, or state
in the design that you have accepted the risk and why.

> **Network+ note.** Objective 1.6 lists these topologies and expects their
> characteristics. Two things beyond the exam: **ring topologies achieved 50 ms
> protection two decades before packet networks did**, and **the star's dominance is
> a fault-isolation argument rather than a performance one**.
