# Chapter 30 — Further Reading

## Primary sources

**RFC 1812 — Baker, F. (1995). *Requirements for IP Version 4 Routers*, §5.2.4.**
The route-matching and next-hop-resolution requirements that §30.1's platform differences
implement.

**RFC 5880 / RFC 5881 — Katz, D. & Ward, D. (2010). *Bidirectional Forwarding
Detection.***
**Read the introduction.** The argument — that failure detection should not be an accident
of whichever protocol's hello timer happens to be running — is the clearest statement of
§30.3's problem.

**RFC 3882 — Turk, D. (2004). *Configuring BGP to Block Denial-of-Service Attacks.***
Remotely-triggered blackholing, of §30.1. Short, practical, and honest about what it costs
the target address.

**RFC 5635 — Kumari, W. & McPherson, D. (2009). *Remote Triggered Black Hole Filtering
with Unicast Reverse Path Forwarding.***
The refined version, including source-based blackholing.

## Books

**Doyle, J. & Carroll, J. (2005). *Routing TCP/IP, Volume 1*, 2nd ed. Cisco Press.**
**The reference for this chapter.** Chapter 3 covers static routing and administrative
distance more carefully than any vendor documentation, including the recursive-resolution
behaviour of §30.1 and the route-selection order in full.

**Odom, W. — current CCNA/CCNP Official Cert Guides.**
The configuration, drilled, with the error cases. The floating-static and IP SLA material
is well presented.

**Zinin, A. (2002). *Cisco IP Routing.* Addison-Wesley.**
Older and unusually deep on *how the routing table is actually built* — the RIB/FIB
interaction of Chapter 29 §29.1 and the route-installation logic that decides what §30.2's
comparisons produce. Worth finding.

**Medhi, D. & Ramasamy, K. (2017). *Network Routing*, 2nd ed.**
Chapter 5 onward for the formal treatment of route selection and metrics.

## Applied

**Build a four-router lab.** **FRRouting** or **BIRD** in containers, or **Containerlab**,
or GNS3. Free, and this chapter is not really learnable without it.

**The single most valuable exercise** (F1): configure a four-router diamond with static
routes only, break the middle link, and **time how long connectivity stays broken.** The
answer is "until you fix it by hand", and experiencing that once is worth more than the
argument in §30.4.

**Then repeat with OSPF** and watch it recover in seconds. The contrast is the whole
justification for Chapter 31.

**`ip route get`** and **`show ip cef`** — Chapter 29's tools, and the way to verify that
a static route is doing what you think.

**IP SLA configuration**, on any Cisco image. Configure it, watch `show track` and
`show ip sla statistics`, then unplug the far end and time the failover. **Then unplug
something *beyond* the next hop and watch it not fail over** — which is §30.3's central
point, made empirically.

**`tc qdisc ... netem loss 100%`** on a Linux router to simulate a link that is up and
carrying nothing — the Ethernet-handoff failure of §30.3, reproducible on a laptop.

**Audit a real configuration** (F3). Take any network with more than twenty static routes
and look for: routes to networks that no longer exist, asymmetric pairs, masks that are
wrong but work by accident, and next hops that were renumbered. **You will find some.**

**Lab 17** in this book's [labs/](../../../labs/) directory builds the branch design of
§30.3 — MPLS primary, Internet backup, floating static with tracking — and then breaks it
in three different ways: interface down, next hop unreachable, and carrier network failed
with the interface up. Only the first two trigger failover without tracking.

## For the certification-minded

Objective 2.2 expects static routing, default routes, administrative distance and the
static-versus-dynamic comparison. **The AD table is directly examined.**

Seven things worth over-learning:

1. **The administrative distance table**: connected 0, static 1, eBGP 20, EIGRP 90,
   OSPF 110, IS-IS 115, RIP 120, iBGP 200, unusable 255.
2. **Lower AD wins**, and **AD is compared before metric**.
3. **Metrics are not comparable across protocols.**
4. **RIP's metric is hop count**, max 15, **16 = unreachable**.
5. **OSPF cost = reference bandwidth ÷ interface bandwidth**, reference 100 Mb/s by
   default, so **everything above 100 Mb/s costs 1**.
6. **A floating static has a higher AD** and installs only when the primary is withdrawn.
7. **A default route is `0.0.0.0/0`**, and the most common static route in the world is a
   default to an ISP.

Expect a "which route is installed" question giving several routes with bracketed pairs.
**Read the first number first**, and check prefix lengths before either.

And the two operational points worth more than the objective:

**A static route is only withdrawn when its next hop becomes unreachable** — and an
Ethernet handoff to a carrier can keep the interface up while the path is dead. If you
take one thing from this chapter into a job, take this.

**Untested failover is not failover.** Schedule the test, run it during a window, and
verify applications rather than pings.
