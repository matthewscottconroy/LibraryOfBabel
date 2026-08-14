# 30.2 Metrics and Administrative Distance

A router may learn the same destination from several places. Chapter 29 §29.3 named the
tie-breakers; this section is what they mean and why there are two of them.

## Two different questions

The distinction people conflate, stated as two questions:

| | Question | Compared |
|---|---|---|
| **Administrative distance** | *How much do I trust the **source** of this route?* | **between** protocols |
| **Metric** | *How good is this **path**, according to that protocol?* | **within** one protocol |

They are answered in that order, and only after longest-prefix match has already
decided that both routes are equally specific.

```
   1. Longest prefix          ← decides first, always
   2. Administrative distance ← between protocols
   3. Metric                  ← within a protocol
   4. ECMP                    ← if still tied
```

A /24 learned from RIP beats a /16 learned from a directly-connected interface.
Specificity is not overridden by trust — which surprises people, and follows directly
from §29.3.

## Administrative distance

**A number from 0 to 255. Lower is more trusted.**

| Source | AD |
|---|---|
| **Connected interface** | **0** |
| **Static route** | **1** |
| EIGRP summary | 5 |
| External BGP (eBGP) | **20** |
| Internal EIGRP | **90** |
| IGRP | 100 |
| **OSPF** | **110** |
| IS-IS | 115 |
| **RIP** | **120** |
| External EIGRP | 170 |
| Internal BGP (iBGP) | **200** |
| **Unknown / unusable** | **255** |

These are Cisco's values and have become the de-facto standard; other vendors use the
same numbers or a similar scale, and Juniper calls the same idea **route preference**.

**The numbers are not arbitrary**, and the ordering encodes a set of judgements:

**Connected is 0** — it is not a claim, it is an observation. The router has an interface
on that subnet; there is nothing to doubt.

**Static is 1** — a person said so, deliberately. **The design assumes the person knew
what they were doing**, which is why a static route beats every protocol and why a wrong
static route is so effective at breaking things.

**EIGRP (90) beats OSPF (110)** — a Cisco judgement about its own protocol, and it is why
a network running both will use EIGRP for anything both know about, regardless of the
paths involved.

**eBGP is 20 and iBGP is 200.** The same protocol, at opposite ends of the scale, and the
reason is Chapter 32's: an eBGP route came from **another organisation** and is
authoritative about that organisation's networks; an iBGP route was **relayed within your
own AS and your IGP almost certainly knows a better path to the same place. Trust the
outsider about the outside; trust yourself about the inside.**

**255 means unusable.** Setting a route's AD to 255 is a way of configuring a route
without installing it.

### Changing it

```
ip route 10.5.0.0 255.255.0.0 192.168.1.2 200
```

The trailing `200` sets the AD. This is the mechanism behind **floating static routes**
(§30.3), and it is essentially the only reason to change an AD deliberately.

Changing a protocol's AD wholesale is possible and is almost always a mistake — it
produces a network where two engineers reason correctly from the defaults and both get
the wrong answer.

## Metric

How good the path is, in whatever units the protocol uses. The units differ so
completely that comparing across protocols is meaningless:

| Protocol | Metric | Range |
|---|---|---|
| **RIP** | **hop count** | 1–15; **16 = unreachable** |
| **OSPF** | **cost**, from bandwidth | 1 – 65,535 |
| IS-IS | cost, configured | 1 – 63 (narrow) |
| **EIGRP** | composite: bandwidth, delay, (load, reliability) | large numbers |
| **BGP** | **no single metric** — a policy sequence | Chapter 32 §32.2 |

RIP's hop count is the clearest illustration of why a metric can be wrong.

```
   A ──── 1 Gb/s ──── B ──── 1 Gb/s ──── C ──── 1 Gb/s ──── D
   │                                                        │
   └──────────────── 64 kb/s satellite ────────────────────┘
```

**RIP chooses the satellite link.** One hop beats three, and RIP cannot see that one hop
is fifteen thousand times slower. Chapter 31 §31.1 develops this; it is the strongest argument against distance-vector metrics and it is why OSPF's cost is derived
from bandwidth.

**OSPF's cost** is, by default:

$$\text{cost} = \frac{\text{reference bandwidth}}{\text{interface bandwidth}}$$

with a reference of **100 Mb/s** — chosen in 1991, when 100 Mb/s was fast.

| Link | Cost (default reference) |
|---|---|
| 10 Mb/s | 10 |
| **100 Mb/s** | **1** |
| **1 Gb/s** | **1** |
| **10 Gb/s** | **1** |
| **100 Gb/s** | **1** |

Everything at 100 Mb/s and above costs 1, because the cost is an integer and cannot
go below it. So OSPF cannot distinguish a 100 Mb/s link from a 100 Gb/s one, which is a
serious defect in any modern network.

The fix, and it must be applied identically on every router:

```
router ospf 1
 auto-cost reference-bandwidth 100000     ! in Mb/s, so 100 Gb/s
```

Now 100 Mb/s costs 1000, 1 Gb/s costs 100, 10 Gb/s costs 10 and 100 Gb/s costs 1.

A mismatched reference bandwidth is a classic and horrible fault: different routers
compute different costs for the same links, so their shortest-path calculations disagree,
and traffic takes paths nobody intended — with every router individually behaving
correctly. Chapter 31 §31.4 returns to it.

## Worked comparisons

**Same prefix, different protocols:**

```
   10.5.0.0/16  via A   OSPF   [110/20]
   10.5.0.0/16  via B   RIP    [120/2]
```

**OSPF wins.** AD 110 < 120. **The metrics are never compared** — 20 and 2 are in
different units, and RIP's 2 hops does not beat OSPF's cost of 20 because the comparison
never happens.

**Same prefix, same protocol:**

```
   10.5.0.0/16  via A   OSPF   [110/20]
   10.5.0.0/16  via B   OSPF   [110/30]
```

**A wins.** Same AD, so the metric decides, and here they are comparable.

**Static versus everything:**

```
   10.5.0.0/16  via A   static  [1/0]
   10.5.0.0/16  via B   OSPF    [110/5]
```

**The static wins**, however good OSPF's path is. **This is the mechanism by which a
forgotten static route quietly overrides a correctly-functioning routing protocol**, and
it is one of the more common causes of "the routing protocol isn't working".

**Longest prefix beats everything:**

```
   10.5.0.0/16     via A   static   [1/0]
   10.5.1.0/24     via B   RIP      [120/8]
```

Traffic to `10.5.1.50` goes **via B**, despite RIP's terrible administrative distance and
hop count. Specificity is decided first and is not overridden.

## Load balancing

When AD *and* metric tie, both routes install and traffic is shared — ECMP (Chapter 29
§29.3). Hashed per flow, so **one conversation takes one path.**

EIGRP additionally supports *unequal-cost* load balancing via its `variance` command,
which installs routes up to *n* times the best metric and shares traffic in inverse
proportion. It is unique among common protocols, it is genuinely useful where links
differ in capacity, and it is used far less than it might be because the resulting
behaviour is hard to reason about.

## Reading them

```
Router# show ip route 10.5.0.0
Routing entry for 10.5.0.0/16
  Known via "ospf 1", distance 110, metric 20, type intra area
  Routing Descriptor Blocks:
  * 192.168.1.2, from 10.255.0.5, 00:23:14 ago, via GigabitEthernet0/1
      Route metric is 20, traffic share count is 1
```

**`distance 110, metric 20`** — stated separately, which is clearer than the `[110/20]`
shorthand.

```
Router# show ip protocols
Routing Protocol is "ospf 1"
  ...
  Distance: (default is 110)
```

`show ip protocols` is the command for "what is running and what does it believe".

## What breaks here

**A routing protocol "not working" while a static route exists.** AD 1 beats everything.
Look for the static.

**RIP choosing an obviously terrible path.** Hop count. Use a better protocol.

OSPF treating a 10 Gb/s link as equal to a 100 Mb/s one. Default reference bandwidth.
Change it — **on every router**.

**Two routers disagreeing about the best path, both correct.** Mismatched OSPF reference
bandwidth, or mismatched metrics generally.

**Comparing `[90/2195456]` with `[110/20]`.** The first number decides; the second is not
comparable.

**A backup path that never activates.** Its AD is not worse than the primary's, so both
are installed and traffic splits. §30.3.

> **Network+ note.** Objective 2.2 expects metrics and administrative distance. **The AD
> table is directly examined.** Over-learn: **connected 0, static 1, eBGP 20, EIGRP 90,
> OSPF 110, RIP 120, iBGP 200**; **lower AD wins**; **AD compares sources and metric
> compares paths within one protocol**; and **RIP's metric is hop count, capped at 15
> with 16 meaning unreachable**.
