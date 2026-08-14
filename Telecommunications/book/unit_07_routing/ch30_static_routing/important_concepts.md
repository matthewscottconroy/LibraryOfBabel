# Chapter 30 — Important Concepts

A static route is a person telling a router something it could not work out *(§30.1)*
— destination, next hop, interface. Destination and one of the other two are mandatory.

The next hop must be reachable *(§30.1)* — If it is not on a directly-connected
subnet, the router has no interface on which to ARP. **IOS resolves recursively** against
the rest of the table; **Linux refuses** with "invalid gateway"; some platforms accept and
silently blackhole. Prefer a directly-connected next hop — recursive statics usually
mean the design has drifted.

**The three forms** *(§30.1)* — **Next hop only**: the default on Ethernet.
**Interface only**: correct on point-to-point, and on Ethernet it makes the router ARP
for every address in the destination range. **Both**: no recursion, no ambiguity, and
the route is withdrawn when that interface goes down. Use both for anything that
matters.

Persistence is separate from configuration *(§30.1)* — Linux `ip route add` does not
survive a reboot; Windows needs **`-p`**. A route that works until the next maintenance
window and then vanishes is entirely avoidable.

**Blackhole routes** *(§30.1)* — `Null0` / `blackhole` (silent) versus `unreachable`
(sends ICMP). For legitimate traffic, tell them; for hostile traffic, silence — an
ICMP reply confirms the address exists. Three uses: **RTBH** under attack (completing the
denial of service against one address to save everything else); **anchoring an aggregate**
so unused parts are discarded rather than following a default outward into a loop; and
**breaking default-route loops** in a hierarchy.

Verify the table before the ping *(§30.1)* — If the route is absent, the ping tells
you nothing. And check the return path — a static configured on one router and not the
other gives traffic that arrives and never comes back, which accounts for most "the static
route doesn't work" reports.

When static is right *(§30.1)* — A stub network with one exit; a default route to an
ISP, which is the most common static route in the world; fewer than a handful of
routers; a route that must not change; blackholes and exceptions; backup paths. Its
virtues are real: no protocol, no CPU, no bandwidth, no adjacency, and completely
deterministic.

**Two different questions** *(§30.2)* — **Administrative distance** asks *how much do I
trust the **source***, and is compared **between** protocols. **Metric** asks *how good is
this **path** according to that protocol*, and is compared **only within** one protocol.
Longest prefix is decided first and is never overridden — a /24 from RIP beats a /16
from a connected interface.

**The AD table** *(§30.2)* — Connected 0, static 1, EIGRP summary 5, **eBGP 20**,
**EIGRP 90**, **OSPF 110**, IS-IS 115, **RIP 120**, external EIGRP 170, **iBGP 200**,
**unusable 255**.

The numbers encode judgements *(§30.2)* — Connected is 0 because it is an
observation, not a claim. Static is 1 because a person said so deliberately — which
assumes the person knew what they were doing, and is why a forgotten static route quietly
overrides a working protocol. eBGP 20 and iBGP 200: trust the outsider about the
outside, trust yourself about the inside.

RIP's hop count can be catastrophically wrong *(§30.2)* — Three gigabit hops versus
one 64 kb/s satellite hop: RIP chooses the satellite. The strongest argument against
distance-vector metrics.

OSPF's default reference bandwidth is 100 Mb/s *(§30.2)* — So everything at
100 Mb/s and above costs 1, and OSPF cannot distinguish 100 Mb/s from 100 Gb/s. Fix with
`auto-cost reference-bandwidth`, identically on every router — a mismatch makes routers
compute different costs for the same links, so their shortest-path calculations disagree
while each behaves correctly.

**Comparisons** *(§30.2)* — `[110/20]` beats `[120/2]` on **AD**, and the metrics are
never compared. `[1/0]` beats `[110/5]`. `[90/2195456]` beats `[110/20]`. The first
number decides; the second is not comparable across protocols.

The default route configured *(§30.3)* — Right for a **single-homed** site; wrong
anywhere with more than one exit and a reason to choose.

**`default-information originate always`** *(§30.3)* — Without `always`, the router stops
advertising a default when it loses its own — which is the behaviour you want. With
`always` it advertises unconditionally, converting a working failover into a black
hole.

**The floating static** *(§30.3)* — Two defaults at the same AD both install and split
traffic. Give the backup a worse administrative distance and it stays out of the
table until the primary is withdrawn. Leave a gap (200, 250) so another route can be
inserted later. If the primary is learned by OSPF, the floating static must be above
110.

The limitation that matters *(§30.3)* — A static route is withdrawn when its next
hop becomes unreachable, which in practice means when the local interface goes down.
A carrier's network can fail while the router's Ethernet handoff stays up, so the
route stays, traffic is black-holed, and the floating static never activates. Total
outage with every interface showing "up". Not an edge case — Ethernet handoffs are the
norm.

**The fixes** *(§30.3)* — IP SLA with object tracking makes the route conditional on
the far end answering (~15 s detection); ping something beyond the next hop, so a
failure anywhere in the carrier's network is caught. **BFD** gives sub-second detection.
Or run a routing protocol across the link purely for failure detection — one of the
strongest arguments for a protocol on a two-router topology.

**Defaults with specifics** *(§30.3)* — Three lines expressing a policy: Internet by
default, corporate over MPLS, one corporate range back over the Internet. Longest-prefix
match resolves it with no further mechanism.

Untested failover is not failover *(§30.3)* — The common discovery is that the backup
path routes correctly and fails anyway: the firewall does not permit it, NAT is configured
for one path, or the far end has no return route. All three are invisible until you
test.

The arithmetic of static routing *(§30.4)* — *n* routers need **n(n−1)** statements;
twenty routers with four subnets each need **1,520**. The count is not the real
problem — a script can generate them.

The change cost is the real problem *(§30.4)* — Adding one router requires **n−1**
changes, on different devices, and the network is inconsistent until the last is
applied.

Static routes do not know they are broken *(§30.4)* — In a diamond topology, when the
middle router fails, the alternative path exists and **is not used**, indefinitely, until
a human intervenes. This is the argument for dynamic routing, and it is about failure
detection far more than path calculation.

Assertions nobody verifies are comments *(§30.4)* — In a large static network, some
routes are wrong — decommissioned subnets, wrong masks working by accident, asymmetric
pairs, stale next hops — and nothing detects any of it.

The two honest tests *(§30.4)* — Does the network have more than one path between
any two points? If so, something must choose and must notice failures. How many
changes does adding one subnet require? More than two means the topology has outgrown
static routing, regardless of size.

What dynamic routing costs *(§30.4)* — CPU and memory; bandwidth; complexity
(adjacencies, timers, areas, redistribution); new failure modes that are emergent rather
than deterministic; less predictability; and a security surface — an unauthenticated
protocol accepts routes from whatever speaks it, whereas a static route cannot be
attacked over the network.

Almost nobody runs one exclusively *(§30.4)* — Static default to the ISP, a protocol
inside, statics to stub branches, statics for blackholes and exceptions, floating statics
for backup, BGP between organisations. A design using only one is usually either very
small or wrong.

**The decision** *(§30.4)* — Use static where the answer is known and will not change.
Use dynamic where the answer must be discovered, or re-discovered when something fails.
Most difficulty comes from networks built with the first that grew into the second without
anyone noticing.
