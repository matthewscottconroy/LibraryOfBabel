# Chapter 30 — Static Routing

The simplest way to fill a routing table is to type the entries in. This chapter is
about doing that well, and about recognising the point at which it stops being the
right answer — which arrives sooner than most people expect and is usually noticed
later than it should be.

## The case for typing

Static routing has genuine advantages, and they are worth stating properly because
the reflex to reach for a dynamic protocol is often wrong.

**No protocol overhead.** No hello packets, no updates, no adjacencies, no CPU spent
recomputing. On a low-bandwidth link, or on a small device, this matters.

**Complete predictability.** The route goes where you said. It does not change
because a distant link flapped. When you are debugging, the table is exactly what
the configuration says, which is a considerable virtue.

**No attack surface.** A dynamic routing protocol accepts information from
neighbours, and a protocol that accepts information can be lied to. Static routes
cannot be injected, poisoned or leaked.

**Appropriate for stub networks.** A branch office with one link to head office has
exactly one way out. There is nothing to compute. Running OSPF to discover a fact
you already know is ceremony.

For a genuinely large fraction of real networks — every small office, every home,
most branch sites, and the customer edge of many enterprises — static routing plus a
default route is the correct and professional answer, and treating it as a beginner's
tool is a mistake.

## The case against

Equally real, and it arrives quietly.

**It does not react.** If the configured next hop becomes unreachable, the route
stays in the table and traffic is black-holed. The router will cheerfully forward
packets toward a dead neighbour indefinitely. Nothing tells anyone. This is the
central weakness, and §30.3's floating static routes and route tracking are the
partial answers.

**It scales quadratically in administration.** *n* networks that must all reach each
other require, in the worst case, *n*(*n*−1) route statements distributed across the
devices — Chapter 11's combinatorics again, in a different costume. Ten networks is
manageable. Forty is not, and the failure mode is not that it becomes impossible but
that it becomes *inconsistent*: someone adds a network and updates seven of the nine
routers that needed it, and the resulting asymmetric reachability is very hard to
diagnose.

**Changes are manual and therefore skipped.** The route that is wrong is usually the
one added during an incident at 2 a.m. and never reviewed.

## Administrative distance: arbitrating between sources

A router may learn the same destination from several sources — a static route and
OSPF, say, or OSPF and BGP. Longest-prefix match (Chapter 29 §29.3) does not help
if the prefixes are identical. Something must break the tie, and it is
**administrative distance**: a number expressing how much the router trusts each
source, lower being more trusted.

| Source | Typical AD |
|---|---|
| Directly connected | 0 |
| Static route | 1 |
| eBGP | 20 |
| OSPF | 110 |
| RIP | 120 |
| Unknown / unusable | 255 |

The numbers are conventions rather than standards, and they vary slightly by
vendor — which is itself worth knowing, because it bites in multi-vendor networks.

Note the ordering's logic. A static route beats every dynamic protocol, because an
administrator's explicit statement is taken as more authoritative than a computed
one. eBGP beats OSPF because a route learned from an external organisation
describes a destination outside your own network, about which your interior protocol
should not be claiming knowledge.

**Metric** is different and is frequently confused with it. Administrative distance
chooses between *protocols*; metric chooses between *routes within one protocol*.
A router compares AD first; only if two routes have the same AD does it compare
metrics, and the metrics of different protocols are not comparable at all — OSPF's
cost and RIP's hop count measure different things in different units.

## Floating static routes, and the useful trick

The AD mechanism enables the chapter's most practically valuable pattern.

Configure the primary path via a dynamic protocol or a low-AD static route. Then
configure a *backup* static route to the same destination with a deliberately high
administrative distance — 200, say. Because its AD is worse, it stays out of the
routing table entirely while the primary is present. If the primary disappears, the
floating route is installed and traffic continues over the backup.

This is how a branch office with a primary MPLS circuit and a backup broadband
connection is usually configured, and it costs two lines. §30.3 works it through,
together with the important caveat: a floating static route only helps if the
primary route actually *withdraws* when the path fails. If the interface stays up
because the failure is beyond the first hop, the primary remains in the table and
the backup never activates. That is what object tracking and IP SLA probes are for,
and it is the sort of detail that separates a design that works in the lab from one
that works at 3 a.m.

## What this chapter does

§30.1 covers configuration on Linux, Cisco IOS-style and a modern network OS, with
the next-hop-address versus exit-interface distinction and why it matters on
multi-access links.

§30.2 covers administrative distance and metrics, the difference between them, and
how a router selects among competing routes.

§30.3 covers default routes, floating static routes, and the tracking mechanisms
that make failover actually work.

§30.4 covers the scaling limits with the arithmetic, and gives concrete criteria for
when to move to a dynamic protocol.

## By the end you will be able to

- Configure a static route, a default route and a floating static route on at least
  two platforms.
- Explain the difference between administrative distance and metric, and predict
  which of several competing routes a router will install.
- Design a primary/backup arrangement using AD, and identify the failure case in
  which it silently does not work.
- Compute the administrative burden of static routing for a stated topology, and
  argue from that number for or against a dynamic protocol.
- Recognise the black-hole symptom that a stale static route produces.
