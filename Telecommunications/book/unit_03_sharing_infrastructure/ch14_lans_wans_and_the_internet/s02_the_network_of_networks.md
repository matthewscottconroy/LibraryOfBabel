# 14.2 The Network of Networks

The word *Internet* is a contraction of **internetwork**, and the contraction has
obscured what the original word meant. This section recovers it, because the
construction it names is the single most important architectural idea in this book.

## The construction

An **internetwork** is what you get when you connect **networks** — not hosts —
and treat each constituent network as a black box that delivers packets internally
by means you neither know nor need to know.

That is a genuinely different idea from "a big network", and the difference is
worth being precise about.

**A big network** extends one technology further: more Ethernet switches, more
cable, one addressing scheme, one administration. It scales until the technology's
limits are reached — broadcast domain size, spanning tree diameter, address table
capacity — and then it stops.

**An internetwork** connects networks that need share nothing except a willingness
to carry a common packet format. They may use different media, different framing,
different addressing internally, different administration, different countries and
different owners. The internetwork does not care.

Cerf and Kahn's 1974 contribution (Chapter 23 §23.1) was precisely this: faced with
three incompatible networks whose operators had no intention of changing, they did
not propose a standard for all three to adopt. They proposed a **gateway** between
them and a common packet format the gateways would understand, and left each network
to carry those packets however it liked.

**And the construction is recursive.** Connect internetworks and you get an
internetwork. The global Internet is this applied at every scale: a building's
subnets connected into a site network, sites connected into an enterprise network,
enterprises connected to providers, providers connected to each other. The same
operation, repeated, and no level needs to understand the internals of any other.

## The four requirements

The construction imposes four things, and each is a chapter of this book.

### 1. A universal address space

Every device must have an identifier meaningful **across every constituent
network**.

A MAC address will not do, and Chapter 15 §15.2 explains why: it is flat, so no rule
summarises a group of them, so any device that must know where an address lives must
know it individually. That is fine at local scale and arithmetically impossible
globally.

So the internetwork needs a **hierarchical** address space, in which a prefix
identifies a network and the remainder identifies a host within it — which is
Chapter 24's IP address, and which is what makes Chapter 29's longest-prefix
forwarding possible.

Note that this is a *second* address space, layered above the local ones. A packet
crossing an Ethernet segment carries both: MAC addresses that change at every hop,
and IP addresses that do not. Chapter 18's ARP exists solely to reconcile them.

### 2. A device that joins networks

Something must sit between two networks, receive a packet from one, and decide which
network to forward it into. That is the **router**, and Chapter 29 shows that its
job is a single decision — extract the destination, consult a table, choose an
interface — repeated statelessly.

The word *gateway* was the original term and survives in "default gateway", which is
the router of last resort for a host.

### 3. A lowest-common-denominator service model

This is the requirement people find least intuitive and it is the most important.

**The internetwork can promise only what its weakest constituent can deliver.**

If IP guaranteed delivery, then every network carrying IP would have to guarantee
delivery — and some cannot. A radio link with 20% frame loss cannot promise delivery
at any price. A satellite link cannot promise bounded delay. A congested Ethernet
segment cannot promise anything at all.

So IP promises **nothing**: no reliability, no ordering, no delay bound, no
notification of failure (Chapter 24 §24.1). Best-effort delivery is not a limitation
that better engineering would remove. It is the **only universally satisfiable
contract**, and choosing it is what let IP run over Ethernet in 1980, Wi-Fi in 1997,
LTE in 2009, and whatever arrives next, without amendment.

The applications that need reliability build it at the endpoints, which is
Chapter 37's TCP and Chapter 23 §23.4's end-to-end argument. The applications that do
not are not forced to pay for it.

### 4. A way for independent networks to exchange reachability

Each constituent network knows its own internal structure. For a packet to cross
several, the networks must tell each other **which destinations they can reach** —
without any central authority, because there is none, and without trusting each
other, because they are frequently competitors.

That is BGP (Chapter 32), and it is why the Internet has no owner, no off switch,
and no complete inventory. Roughly seventy-five thousand autonomous systems exchange
reachability bilaterally, under commercial arrangements individually negotiated, and
the aggregate behaves as one network.

## The hourglass, again

Chapter 23 §23.1 draws it; the reason belongs here.

```
   many applications      ← anyone may invent one
        ╲    ╱
         ╲  ╱
          IP              ← exactly one, and deliberately minimal
         ╱  ╲
        ╱    ╲
   many link technologies ← anyone may invent one
```

The four requirements above produce this shape necessarily. A **universal address
space** and a **lowest-common-denominator service** mean there can be only one
protocol at the waist, and it must promise almost nothing. Everything else is free
to multiply.

RFC 1958 states the rule as *"everything over IP, and IP over everything"*, and the
architectural consequence is that **neither end of the hourglass needs permission
from the other**. A new link technology carries IP and instantly serves every
existing application; a new application runs over IP and instantly works across
every existing network.

This is why the web could be invented by one person at CERN without asking a single
network operator, and it is the property that Chapter 13 §13.4 identified as
mattering most in the long run.

It is also why IPv6 has taken thirty years (Chapter 28 §28.4). **The waist of an
hourglass is the hardest part to change**, because everything above and below
depends on it, and there is no incremental path — an IPv6-only host cannot reach an
IPv4-only host, so adopting it alone reaches nothing new.

## What the construction costs

Two honest observations.

**The lowest-common-denominator service is genuinely weak**, and every guarantee any
application needs must be rebuilt at the edges. TCP is eighty-five pages of
specification (Chapter 37) reimplementing reliability that a circuit provided for
free, and it is reimplemented in every operating system.

**There is no central point of control, which is a strength and a liability.**
Nobody can turn the Internet off; nobody can fix it either. A BGP misconfiguration
in one autonomous system can and repeatedly has removed large parts of the Internet
from the Internet (Chapter 32 §32.4), and the recovery depends on many independent
parties acting.

## What breaks here

**Assuming the internetwork provides a guarantee the constituent networks do not.**
It cannot, by construction.

**Designing an application that requires low latency across an internetwork** whose
constituents include a satellite link. The weakest link sets the bound.

**Expecting a middlebox to be transparent.** NAT (Chapter 33) and deep packet
inspection break the model deliberately, and applications that assume end-to-end
transparency fail in ways that are hard to diagnose.

**Treating "the Internet" as a single entity with an operator.** There is nobody to
telephone. Chapter 48 §48.2's peering and transit structure is what determines who
is actually responsible for any given path, and identifying that party is the first
useful step in a wide-area incident.

> **Network+ note.** Objective 1.6 expects the LAN/WAN/Internet distinction and
> objective 1.2 expects the router's role. The four requirements are the useful
> framing: **universal addressing, a joining device, a minimal service model, and
> inter-domain reachability exchange** — which maps exactly onto Chapters 24, 29, 24
> and 32 and makes the rest of the book's structure predictable.
