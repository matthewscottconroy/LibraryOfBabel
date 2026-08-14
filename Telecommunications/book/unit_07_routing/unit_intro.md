# Unit VII — Finding the Way

Unit VI gave every host in the world an address with structure in it. This unit is
about using that structure to move a packet across networks that no single
organisation controls, operated by parties with conflicting commercial interests,
without any central authority and without any component holding a complete picture.

That the Internet does this successfully, billions of times per second, is the
most impressive fact about it, and it is worth being briefly astonished
before we take it apart.

## The decision

Strip away the acronyms and a router does one thing, repeatedly:

> A packet arrives. Extract its destination address. Consult a table. Choose an
> outgoing interface and a next-hop address. Send it. Forget everything.

That is the **forwarding** operation, it is stateless, and it happens in hardware in
a few hundred nanoseconds. It is the subject of Chapter 29, and its simplicity is a
direct consequence of Chapter 24's best-effort refusal.

The interesting question is not how a packet is forwarded. It is **how the table
gets there**, and there are exactly three answers:

- **Someone typed it in** — static routing, Chapter 30. Simple, predictable,
  invisible when wrong, and it stops scaling at a size you will reach sooner than
  you expect.
- **Routers told each other** — dynamic routing within one organisation,
  Chapter 31. Distance vector and link state, RIP and OSPF, Bellman–Ford and
  Dijkstra.
- **Organisations told each other** — Chapter 32, and this is a different problem
  entirely, because the parties are not cooperating in good faith about finding the
  best path. They are pursuing commercial interests, and BGP is less a routing
  protocol than a mechanism for expressing policy.

That last distinction — **inside one administration versus between administrations**
— is the organising principle of this unit and the reason the routing protocol world
divides into interior and exterior gateway protocols. Inside, everyone wants the
shortest path and will tell the truth to get it. Outside, the shortest path may be
one you are contractually forbidden to use, or one that costs you money, or one
through a competitor, and the protocol must accommodate all of that without a
central arbiter.

## What the unit contains

**Chapter 29 — Forwarding and Longest-Prefix Match.** The decision itself; reading a
real routing table on Linux and on a router; longest-prefix match derived from the
problem before it is named; and the default gateway as the route of last resort.

**Chapter 30 — Static Routing.** Configuration on three platforms; metrics and
administrative distance; default and floating routes; and an honest account of the
point at which static routing becomes a liability.

**Chapter 31 — Dynamic Routing.** Distance vector, its count-to-infinity problem and
the several partial fixes; link state, Dijkstra's algorithm, and OSPF's areas; and
how the choice of algorithm shapes the network designs that are sensible.

**Chapter 32 — BGP and Autonomous Systems.** Path vector; policy as the primary
input; the commercial structure of peering and transit; and the failure modes —
route leaks and hijacks — that periodically take large parts of the Internet
offline, with real incidents examined.

**Chapter 33 — NAT and PAT.** The 1994 workaround that bought thirty years; the
translation table in detail; the catalogue of things NAT breaks and the workarounds
each required; and carrier-grade NAT.

**Chapter 34 — ICMP and the Diagnostic Path.** IP's error channel; what `ping`
actually proves and what it does not; the TTL trick that makes `traceroute` work,
including why its output is routinely misread; and path MTU discovery with its
black-hole failure.

## Two warnings

**This unit is where conceptual understanding and vendor configuration diverge
most.** There is a large industry teaching people to configure OSPF on a particular
manufacturer's equipment, and that is a legitimate and valuable skill which this
book does not attempt to provide. What this book provides is the model underneath —
why link state converges faster than distance vector, why areas exist, what
administrative distance is arbitrating between — because that transfers between
vendors and survives the next product generation, and because someone who has it can
learn any vendor's syntax in an afternoon.

**Do not become a router administrator here.** An introductory course that spends
four weeks on OSPF configuration produces students who can type commands and cannot
explain why a packet went the way it did. Chapter 31 will have you compute a
shortest-path tree by hand, which is worth more than a hundred lines of
configuration you have copied.

## The recurring shape

One thing to watch for as you read, because it recurs and is the unit's real
lesson.

Every routing mechanism here is a tradeoff between **how much each node knows** and
**how fast the system reacts**. Static routing: nodes know only what you told them,
and the system does not react at all. Distance vector: nodes know only what their
neighbours claim, which is cheap and converges slowly and can converge to something
wrong. Link state: every node holds a complete map of the area, which costs memory
and computation and converges in a second or two. BGP: nodes know paths rather than
distances, which permits policy and loop detection at the cost of a global table that
is now over a million entries and growing.

Knowledge, memory, convergence time, policy expressiveness. Every protocol in this
unit picks a point in that space, and knowing which point — and why — is what lets
you choose one.
