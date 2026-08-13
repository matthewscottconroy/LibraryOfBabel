# 32.2 Path Vector and Policy

BGP's two structural ideas: **carry the whole path**, which solves §31.2's loop problem
outright, and **select by a sequence of policy comparisons** rather than by a metric,
which lets commercial preference override topology.

## Path vector

**Every advertisement carries the list of ASes it has traversed.**

```
   AS 100 originates 203.0.113.0/24 with AS_PATH = ( 100 )

   → AS 200 receives it, prepends itself:        AS_PATH = ( 200 100 )
   → AS 300 receives it from 200:                AS_PATH = ( 300 200 100 )
   → AS 400 receives it from 300:                AS_PATH = ( 400 300 200 100 )
```

**Read right to left: the origin is last, the most recent AS is first.**

**Loop detection is one rule:**

> **If a router sees its own AS number in the AS_PATH, it discards the route.**

That is the whole mechanism. No split horizon, no poison reverse, no holddown, no counting
to infinity (§31.2). **The provenance that distance vector lacked is carried explicitly**,
so a router can simply look and see whether accepting the route would mean routing through
itself.

**And the same field is a metric**, of sorts — a shorter AS path is preferred, other things
being equal. But other things are usually not equal, and §32.2's selection algorithm
places AS path fourth.

**AS path length is a proxy for nothing physical.** One AS may be a single router in one
building; another may be a transcontinental network with forty internal hops. A three-AS
path may be faster than a two-AS path, routinely.

**AS path prepending** exploits the tie-break deliberately:

```
   Normal:              AS_PATH = ( 100 )
   Prepended ×3:        AS_PATH = ( 100 100 100 )
```

Advertising your own prefix with your AS repeated makes the path look longer, so other
networks prefer a different entry point. **It is the standard tool for influencing inbound
traffic** — and it is blunt, because it only works against networks for which AS path is
actually the deciding step, and many networks decide earlier on local preference.

## The path attributes

An advertisement carries a prefix and a set of attributes. The ones that matter:

| Attribute | Type | Purpose |
|---|---|---|
| **AS_PATH** | well-known mandatory | the path, and loop detection |
| **NEXT_HOP** | well-known mandatory | where to send traffic |
| **ORIGIN** | well-known mandatory | IGP / EGP / incomplete |
| **LOCAL_PREF** | well-known discretionary | **preference, within your AS** |
| **MED** | optional non-transitive | a hint to a neighbour AS |
| **COMMUNITY** | optional transitive | **a tag, for policy signalling** |
| ATOMIC_AGGREGATE, AGGREGATOR | | summarisation bookkeeping |

**Well-known** attributes must be understood by every implementation. **Optional
transitive** ones are passed on even by routers that do not understand them — which is how
COMMUNITY works across networks that have never heard of your particular tag.

### LOCAL_PREF — the important one

**Higher is better.** Default 100.

**It is the primary tool for controlling outbound traffic**, and it is **not sent to other
ASes** — it is carried only within yours, over iBGP, so every router in your AS agrees on
which exit to prefer without telling anyone outside.

```
   route-map PREFER-CHEAP permit 10
    set local-preference 200            ! this provider wins
```

**Because LOCAL_PREF is compared first** (§32.2's algorithm), it overrides AS path
entirely. A four-AS-hop path with LOCAL_PREF 200 beats a one-hop path at 100 — which is
exactly the point: **your commercial preference beats the topology.**

### MED — the weak one

**Lower is better.** A hint to a neighbouring AS about which of *your* entry points it
should use.

**And it is only a hint.** Your neighbour may ignore it, and usually does — because MED is
compared *late* in the selection process, after LOCAL_PREF, which your neighbour sets for
its own reasons.

**MED is compared only between routes from the same neighbouring AS** by default, which
further limits it. It is the weakest of the influence tools and it is the one people reach
for first.

### COMMUNITY — the useful one

**A 32-bit tag**, conventionally written `ASN:value`, attached to a route and carried
along with it.

**It carries no meaning of its own.** Its meaning is whatever the receiving network
publishes:

```
   3356:70     Lumen: set LOCAL_PREF 70   (de-prefer this route)
   65535:666   RFC 7999: BLACKHOLE — discard traffic to this prefix
   65535:65281 NO_EXPORT — do not advertise beyond this AS
```

**Communities are how customers signal policy to providers without a phone call.** A
customer under attack tags a /32 with its provider's blackhole community and the provider
drops the traffic at its edge — Chapter 30 §30.1's RTBH, triggered by the customer, in
seconds, with no human involved.

**RFC 7999 standardised `65535:666` as BLACKHOLE** precisely so that this works across
providers without each publishing its own value.

**Large communities** (RFC 8092) exist because the original 32-bit form cannot hold a
32-bit AS number plus a value.

## The selection algorithm

**When several paths to one prefix are known, BGP compares them in this order and stops at
the first difference.**

```
    1.  Highest WEIGHT                    (Cisco-only, local to one router)
    2.  Highest LOCAL_PREF                ← your policy, AS-wide
    3.  Locally originated                (you injected it yourself)
    4.  Shortest AS_PATH                  ← the only "topology" step
    5.  Lowest ORIGIN                     (IGP < EGP < incomplete)
    6.  Lowest MED                        (from the same neighbour AS)
    7.  eBGP over iBGP                    ← prefer an external path
    8.  Lowest IGP metric to the NEXT_HOP ← "hot potato"
    9.  Oldest route                      (stability over optimality)
   10.  Lowest router ID
   11.  Lowest neighbour address
```

**Read what this list is not.** There is no bandwidth, no latency, no packet loss, no
congestion, no cost, no reliability. **BGP has no idea whether a path is fast.**

**Steps 1–3 are entirely local policy.** Step 4 is the first that considers the network at
all, and it is a hop count between administrative domains.

**Step 8 is "hot potato" routing** and deserves its name. Among equally-preferred exits,
a router chooses the one **closest to itself**, which means **traffic leaves your network
as soon as possible** — you hand it to the neighbour at the nearest exit and let them carry
it the rest of the way.

**This is rational and it produces asymmetry.** Traffic from A to B leaves A's network at
A's nearest exit; traffic from B to A leaves B's network at B's nearest exit. **The two
directions take entirely different paths**, routinely, which is why Chapter 29 §29.1's
warning about asymmetric routing matters most here.

**Steps 9–11 exist to make the outcome deterministic.** "Oldest route" is a stability
preference — it avoids churning when two paths are otherwise identical.

## iBGP and the full mesh

**BGP inside your own AS**, and it has one rule that surprises everyone:

> **A route learned from an iBGP peer is never advertised to another iBGP peer.**

**This is loop prevention.** The AS_PATH does not change within an AS — every router has
the same AS number — so §32.2's loop check cannot work internally. The rule substitutes
for it.

**The consequence is severe:** every iBGP router must peer with **every other**.

$$\text{sessions} = \frac{n(n-1)}{2}$$

| Routers | Sessions |
|---|---|
| 5 | 10 |
| 10 | 45 |
| 20 | **190** |
| 50 | **1,225** |

**The same arithmetic as OSPF's broadcast segment** (Chapter 31 §31.3), and the same two
answers:

**Route reflectors.** One router reflects routes between its clients, so they need not peer
with each other. Sessions drop from *n(n−1)/2* to roughly *n*. **The standard solution**,
and it is exactly OSPF's designated router applied to iBGP.

**Confederations.** Split the AS into sub-ASes that use eBGP between them, restoring AS
path loop detection internally. More complex, less common, and useful for very large
networks or post-merger integration.

**Peer using loopback addresses** (Chapter 27 §27.4), so an iBGP session survives the
failure of any single link — the session follows whatever path the IGP finds. This is the
strongest practical argument for the loopback convention.

## Configuration

```
router bgp 65001
 bgp router-id 10.255.0.1
 bgp log-neighbor-changes

 ! external peer
 neighbor 203.0.113.1 remote-as 64500
 neighbor 203.0.113.1 description Provider-A
 neighbor 203.0.113.1 password <secret>
 neighbor 203.0.113.1 prefix-list ANNOUNCE-OURS out      ! MANDATORY
 neighbor 203.0.113.1 prefix-list SANITY in              ! MANDATORY
 neighbor 203.0.113.1 maximum-prefix 1000000 90 restart 15
 neighbor 203.0.113.1 ttl-security hops 1

 ! internal peer, on loopbacks
 neighbor 10.255.0.2 remote-as 65001
 neighbor 10.255.0.2 update-source Loopback0
 neighbor 10.255.0.2 next-hop-self

 network 203.0.113.0 mask 255.255.255.0
```

**Four lines are load-bearing:**

**`prefix-list ... out`** — **the single most important line in BGP.** Without an outbound
filter, a misconfiguration or a route leak advertises everything you have learned to
everyone you peer with, and §32.4's incidents are what follows. **Never configure a BGP
neighbour without an outbound filter.**

**`prefix-list ... in`** — reject anything implausible: your own prefixes, RFC 1918
space, default routes you did not ask for, prefixes longer than /24.

**`maximum-prefix`** — tear the session down if the neighbour sends more than expected.
**A blunt instrument that has saved many networks**, because the alternative to a dropped
session is a router that runs out of memory and stops forwarding entirely.

**`next-hop-self`** — an iBGP router receiving an external route sees the *external*
neighbour's address as NEXT_HOP, which its IGP may not know how to reach. Rewriting it to
your own address fixes it, and forgetting it produces routes that are present and
unusable.

**`ttl-security hops 1`** is the GTSM trick of Chapter 18 §18.4: send at TTL 255, require
receipt at 255, proving the peer is directly connected. Free, and it eliminates every
off-link attacker.

## What breaks here

**Routes in the BGP table but not the routing table.** The NEXT_HOP is unreachable.
`next-hop-self`, or an IGP route to it.

**iBGP routes not propagating.** The no-readvertisement rule. Add a route reflector or
complete the mesh.

**Prepending having no effect.** The other network decides on LOCAL_PREF, which you cannot
influence with AS path.

**MED being ignored.** It usually is.

**Asymmetric paths.** Hot potato routing on both sides. Expected.

**A session that flaps and takes everything with it.** BGP over TCP: session down means
every route withdrawn.

**Advertising the whole table to a peer.** No outbound filter. §32.4.

> **Network+ note.** Objective 2.2 expects BGP's path-vector nature. Over-learn: **the
> AS_PATH is used for loop detection — a router seeing its own AS discards the route**;
> **BGP selects by policy, not by shortest path**; **LOCAL_PREF is highest-wins and
> controls outbound**; **AS path prepending influences inbound**; and **iBGP peers do not
> readvertise to each other, hence route reflectors.**
