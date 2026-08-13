# Chapter 29 — Forwarding and Longest-Prefix Match

A router's job description fits on one line, and the line is worth writing out
because almost every misconception about routing comes from believing the job is
larger than it is.

> **Given a destination address, choose an outgoing interface and a next hop.**

That is all. A router does not know the path to the destination. It does not know
how many hops remain, or whether the destination exists, or whether the packet will
arrive. It knows one step, it takes it, and it forgets the packet entirely.

This is **hop-by-hop destination-based forwarding**, and its consequences are worth
drawing out because they explain a great deal of observed behaviour:

- **Each router decides independently.** No router is coordinating with the others
  about this packet. They happen to agree because they have consistent tables, and
  when they do not agree you get a routing loop — which is survivable only because
  of the TTL from Chapter 24 §24.4.
- **The return path may differ entirely.** Nothing requires symmetry. Packets from
  A to B may cross five networks and packets from B to A may cross nine. This is
  routine on the Internet, it makes `traceroute` output harder to interpret than
  people assume (Chapter 34), and it is why an asymmetric path breaks stateful
  firewalls that see only one direction.
- **The router is stateless with respect to conversations.** It keeps no record of
  the packet after forwarding it. This is what allows a router to handle millions of
  simultaneous flows in hardware with no per-flow memory, and it is the direct
  operational payoff of the end-to-end argument.

## Control plane and data plane

A distinction that will recur through Chapters 30–32 and become the entire subject
of Chapter 68, so it is worth fixing here.

The **control plane** decides what the table should contain. It runs routing
protocols, processes updates, computes shortest paths, applies policy. It runs in
software, on a general-purpose CPU, on timescales of milliseconds to seconds. It is
where all the interesting complexity lives.

The **data plane** uses the table to forward packets. It runs in dedicated hardware,
does nothing but lookup and rewrite, and operates at line rate — hundreds of
millions of packets per second.

The separation is why a router can be running a computationally expensive OSPF
recalculation while forwarding traffic at full speed, and why a control-plane
failure often leaves traffic flowing (the data plane keeps using the last known
table) while the network slowly becomes wrong. It is also why pinging a router's own
interface measures its CPU rather than its forwarding performance — the point
Chapter 3 §3.2 made and which Chapter 34 will make again with feeling.

## Reading a table

§29.2 works through real output. A Linux host:

```
$ ip route
default via 192.168.10.1 dev eth0 proto dhcp metric 100
192.168.10.0/24 dev eth0 proto kernel scope link src 192.168.10.70
10.8.0.0/16 via 192.168.10.254 dev eth0 proto static
```

Three entries. The middle one is **directly connected** — installed automatically
when the interface was configured, and it means "for these addresses, ARP and send
directly, no router involved" (Chapter 18). The third is a specific route learned
from an administrator. The first, `default`, matches everything and is the route of
last resort.

The same information on a Cisco-style device is presented differently, with a
letter code per entry indicating its source (`C` connected, `S` static, `O` OSPF,
`B` BGP) and a bracketed pair giving administrative distance and metric. §29.2
reads both and, more usefully, shows how to trace what a router will do with a given
destination — which is a skill you will use in every troubleshooting scenario in
Unit XIII.

## The rule that makes hierarchy work

Here is the problem that produces the chapter's central mechanism. A table contains:

```
10.0.0.0/8       via A
10.1.0.0/16      via B
10.1.5.0/24      via C
0.0.0.0/0        via D
```

A packet arrives for `10.1.5.42`. It matches the first entry. It also matches the
second, the third, and the fourth — all four are correct statements about where
this address lives. Which does the router use?

**The most specific one: the longest prefix.** `10.1.5.0/24` has the longest mask
that matches, so the packet goes via C.

This rule is not an arbitrary tie-break. It is the mechanism that makes hierarchical
addressing usable, and it is worth seeing why. It permits **a general rule plus
exceptions**: "everything in 10/8 goes that way, except this /16 which goes another
way, except this /24 within it which goes a third way." Without longest-prefix
match, every exception would require the general rule to be broken into pieces that
exclude it, and aggregation (Chapter 26 §26.3) would be impossible.

It is exactly how the postal analogy works — a rule for France, an exception for
one Paris arrondissement — and it is why the global routing table can hold under a
million entries for twenty billion devices.

The default route is simply the degenerate case: `0.0.0.0/0` has a prefix length of
zero, so it matches everything and loses to *any* other match. It is not special-
cased anywhere. It is the shortest possible prefix and it wins only when nothing
else does.

§29.3 also covers what this costs to implement, because "find the longest matching
prefix among a million entries in 300 nanoseconds" is a genuine engineering problem
with genuine solutions — tries, TCAM, and the reason routing hardware is expensive.

## What this chapter does

§29.1 states the forwarding decision precisely and develops its consequences.

§29.2 reads real routing tables on two platforms and traces lookups.

§29.3 derives longest-prefix match from the aggregation problem, works examples, and
sketches the hardware.

§29.4 covers the default gateway from the host's perspective, connecting back to
Chapter 25 §25.3's local/remote decision and forward to the failure modes.

## By the end you will be able to

- State the forwarding decision and its three consequences.
- Distinguish control plane from data plane and predict which failures affect which.
- Read a routing table on Linux and on a router, and identify each entry's origin.
- Determine which route a router will select for any destination, correctly, by
  longest-prefix match.
- Explain why a default gateway is not a special case.
- Diagnose the two classic symptoms: no default route (local works, remote fails)
  and wrong default gateway (some remote destinations work, others do not).
