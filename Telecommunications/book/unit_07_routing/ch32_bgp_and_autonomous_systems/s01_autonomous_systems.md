# 32.1 Autonomous Systems

Everything in Chapter 31 assumed one organisation with one routing policy, choosing the
shortest path because shortest is obviously best.

**Between organisations, none of that holds.** Shortest is often not what anyone wants,
"best" is a commercial judgement, and no organisation will let another compute its routes.
This section is about the boundary where interior routing stops applying.

## The definition

> **An Autonomous System is a collection of IP prefixes under a single, clearly-defined
> routing policy.**

RFC 1930's phrasing, and every word is doing work.

**Single routing policy** — not single owner, not single technology, not single location.
An AS is defined by who decides where its traffic goes.

**The consequences:**

- One organisation may operate **several** ASes — common for large providers with
  regionally distinct policies, or after acquisitions
- Several organisations may share **one** AS — a customer using its provider's AS number
- An AS may span continents, or one rack
- **The boundary is administrative, not technical**

**This is the key idea of the chapter.** Chapter 31's areas are a *technical* division of
one administration. An AS boundary is an *administrative* division, and the protocol that
crosses it must be built for parties who do not trust each other and do not share
objectives.

## AS numbers

| | |
|---|---|
| Original | **16 bits** — 65,536 |
| Current | **32 bits** (RFC 6793) — 4.3 billion |
| Private range (16-bit) | **64512 – 65534** |
| Private range (32-bit) | 4200000000 – 4294967294 |
| Reserved | 0, 23456, 65535 |
| Assigned by | IANA → RIRs → organisations |

**The 16-bit space exhausted**, exactly as IPv4 did and for the same reason — an
allocation sized for a smaller world. The 32-bit extension was deployed from 2007 and,
unlike IPv6, transitioned successfully within a few years.

Why it worked where IPv6 did not is worth noticing: AS numbers are used only by BGP
speakers — a few tens of thousands of organisations, all of them technically sophisticated
and all of them motivated — rather than by every device on Earth. **AS 23456 (`AS_TRANS`)
was reserved as a placeholder** so that old speakers could carry a 32-bit path in a
16-bit field without understanding it, which made the transition incremental in exactly
the way Chapter 23 §23.1 says every Internet change must be.

**A small, motivated, technically-capable population and a working compatibility
mechanism** — the conditions IPv6 lacked.

**Private AS numbers** work like RFC 1918 addresses: usable internally, stripped at the
boundary (`remove-private-as`), never seen on the global Internet. A customer that
multihomes to one provider's two routers uses one, and nobody outside ever knows.

## Who has one

Roughly **75,000 ASes** are visible in the global routing table. They are not all
providers:

| Kind | Examples |
|---|---|
| **Tier 1 transit** | Lumen (3356), Arelion (1299), NTT (2914), GTT, Telia |
| Regional and national ISPs | thousands |
| **Content** | Google (15169), Cloudflare (13335), Amazon (16509), Meta (32934) |
| **Enterprises that multihome** | banks, universities, large retailers |
| Internet exchanges | the fabric operators |
| Research and government networks | JANET, Internet2, national research networks |

You need an AS number when you multihome — when you connect to two or more providers
and want to control which is used. With one provider you take their addresses and their
default route, and BGP would be pointless (Chapter 30 §30.1).

The requirement is exactly:

> You need an AS and BGP when there is a decision to make that only you should make.

## IGP versus EGP

The distinction that organises Unit VII:

| | **IGP** — interior | **EGP** — exterior |
|---|---|---|
| Runs | **within** an AS | **between** ASes |
| Protocols | OSPF, IS-IS, EIGRP, RIP | **BGP, and only BGP** |
| Goal | **shortest path** | **the policy-preferred path** |
| Metric | technical — cost, hops | **commercial and political** |
| Trust | full — one administration | **none** |
| Scale | hundreds of routers | **75,000 ASes, ~950,000 prefixes** |
| Convergence | seconds | **minutes** |
| Changes | rare | **constant** — thousands per second globally |

**"There is only one EGP" is a real statement.** BGP has no competitor and has not had one
since 1994. Chapter 22's OSI produced IDRP, which went nowhere. The reason is Chapter 23's
again: **you cannot replace the protocol that every network on Earth must speak to every
other**, and BGP's deficiencies (§32.4) persist for exactly that reason.

## Why interior protocols cannot do this job

Four reasons, each fatal on its own.

### 1. Shortest path is the wrong objective

```
   Your AS ──┬── Provider A: expensive, excellent, 2 AS hops to the destination
             └── Provider B: cheap, adequate, 4 AS hops
```

**OSPF would choose A**, because 2 < 4. **You may well want B**, because A costs
\$8,000/month more and adequate is adequate.

Or the reverse for your latency-sensitive traffic, and both simultaneously depending on
the destination.

**There is no metric that expresses "cheaper", "contractually preferred", "not through a
competitor", or "not through a jurisdiction we avoid".** Those are the actual criteria,
and they are not technical.

### 2. Nobody will run your protocol

OSPF requires every participant to share a database and to trust every advertisement in
it. **A router that accepts OSPF from another organisation accepts that organisation's
view of the world** — and a mistake or a lie propagates instantly and globally.

**Between organisations, every advertisement must be filterable, and every neighbour must
be assumed careless.**

### 3. The scale is different in kind

An OSPF area holds hundreds of routes and every router computes over the full topology.
The global table holds ~950,000 prefixes and changes thousands of times per second.

Flooding a link-state database of that size, globally, is not merely expensive — it is
impossible. Nothing can hold the Internet's topology, and no protocol that requires it
can work.

### 4. Policy must be expressible and hidden

You need to say *"prefer A for this, B for that, and never accept a route to my own
prefixes from anyone"* — and you need to do it **without revealing your commercial
arrangements** to the neighbours you are choosing between.

Interior protocols advertise everything to everyone, which is exactly wrong here.

## What BGP is instead

**A path-vector protocol carrying policy attributes.**

| Property | Detail |
|---|---|
| Transport | **TCP port 179** — reliable, so no periodic refresh |
| Updates | **incremental only** — advertise a change, not the table |
| Path | the **full AS path** — provenance, solving §31.2's problem |
| Selection | a **sequence of policy comparisons**, not a metric |
| Keepalive | 60 s; hold timer 180 s |
| AD | **eBGP 20, iBGP 200** |
| Neighbours | **manually configured** — no discovery |

**Two of those deserve emphasis now.**

**Neighbours are configured by hand.** BGP has no hello, no discovery, no automatic
adjacency. You must know who you are peering with, and so must they — which is
appropriate for a relationship that is usually contractual, and it is the one place where
BGP's trust model is sound by construction.

**It runs over TCP.** Unlike OSPF (IP protocol 89, its own reliability) and RIP (UDP),
BGP delegates reliability to TCP, which means it never needs to refresh: **once a route is
sent, it is assumed held until explicitly withdrawn.** A BGP session that stays up for
years may send nothing for hours.

**And the corollary:** if the TCP session drops, **every route learned over it is withdrawn
at once.** A flapping session is therefore far more disruptive than a flapping OSPF
adjacency, and §32.2's dampening exists because of it.

## The shape of the Internet

The AS graph is not a hierarchy, though it is often drawn as one.

```
        ┌───────── Tier 1 ─────────┐   full mesh, settlement-free
        │    3356   1299   2914    │   they peer with each other
        └───┬───────┬───────┬──────┘   and buy transit from nobody
            │       │       │
        ┌───▼───┐ ┌─▼─────┐ │              Tier 2: buy transit up,
        │Tier 2 │ │Tier 2 │◀┘              peer sideways, sell down
        └───┬───┘ └───┬───┘
            │         │
        ┌───▼───┐ ┌───▼───┐
        │Stub AS│ │Stub AS│                buy transit, sell nothing
        └───────┘ └───────┘
```

A Tier 1 network is defined by what it does not do: it **buys transit from nobody**,
and reaches the entire Internet through settlement-free peering with the other Tier 1s.
There are roughly a dozen, and membership is a commercial fact rather than a technical
one.

**The picture is increasingly wrong.** Large content networks — Google, Meta, Cloudflare,
Netflix, Amazon — peer directly with thousands of access networks, bypassing the transit
hierarchy entirely. **A large fraction of Internet traffic now never touches a Tier 1
network**, and the graph has flattened into something closer to a mesh with dense
content-to-access peering.

§32.3 is about the money that produced this shape.

## What breaks here

**Running BGP with a single provider.** You already know the answer: a default route.
Unless you need provider-independent addresses or plan to multihome, BGP adds risk without
benefit.

**Treating an AS as a technical boundary.** It is administrative. The question is *who
decides*.

**Expecting BGP to choose the fastest path.** It chooses the **policy-preferred** path,
and §32.2's algorithm has latency nowhere in it.

**Assuming a shorter AS path is better.** An AS may be one router or ten thousand
kilometres of fibre. AS path length is a proxy for nothing physical.

**Leaking a private AS number.** Strip it at the boundary.

> **Network+ note.** Objective 2.2 expects BGP as the exterior gateway protocol and the
> IGP/EGP distinction. Over-learn: **BGP is a path-vector EGP running on TCP 179**; **an
> AS is a routing-policy domain**; **AS numbers are 16-bit legacy and 32-bit current**;
> **private range 64512–65534**; and **BGP is used to multihome** — which is the answer to
> "why would an organisation run BGP?"
