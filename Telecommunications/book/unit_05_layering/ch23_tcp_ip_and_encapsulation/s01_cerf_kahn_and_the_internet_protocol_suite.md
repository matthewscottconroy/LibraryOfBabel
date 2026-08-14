# 23.1 Cerf, Kahn and the Internet Protocol Suite

In the spring of 1973, Vint Cerf and Bob Kahn sat in the lobby of the Hyatt Cabana
hotel in Palo Alto, waiting for a conference session, and sketched on the back of an
envelope the architecture that runs the world.

The envelope story is told often enough to sound apocryphal. Cerf has confirmed it
repeatedly, with the qualification that matters: **the hard part was not the sketch. It
was the problem statement.**

## The problem

By 1973 there was not one network. There were several, and they were profoundly
different:

| Network | Character |
|---|---|
| **ARPANET** | 50 kb/s leased lines, packet switching, reliable, US-wide |
| **PRNET** (packet radio) | mobile, lossy, variable rate, San Francisco Bay |
| **SATNET** | satellite, 0.5 s latency, broadcast, transatlantic |
| Various local networks | Ethernet at Xerox PARC, Aloha in Hawaii |

Each was well-engineered for its own conditions. Each had its own packet format, its
own addressing, its own error handling, its own maximum packet size, and its own idea
of what a network guaranteed.

Kahn's problem, stated as he posed it: **how do you connect networks that were designed
without any knowledge of one another, and will not change?**

That last clause is the whole difficulty. Any solution requiring every network to adopt
a common internal design is not a solution; it is a proposal that the world be
different.

## The four principles

The answer, from Cerf and Kahn's 1974 paper *A Protocol for Packet Network
Intercommunication*, rests on four decisions. They are worth stating individually
because each is a choice that could have gone otherwise.

**1. Each network keeps its own internals.**

No network is asked to change. Ethernet stays Ethernet, packet radio stays packet
radio. The internetwork sits **on top**, using each network as an opaque delivery
mechanism.

This is §21.1's *n* side, and it is why the Internet could absorb Ethernet, then Wi-Fi,
then LTE, then 5G, then whatever comes next, without any architectural change.

**2. Gateways connect networks and stay simple.**

A **gateway** — the word "router" came later — sits between two networks, translating
packet formats and forwarding.

The critical decision: **gateways keep no per-connection state.** They forward each
packet according to its destination and forget it. Not because it is easier, but
because a stateful gateway is a gateway whose failure destroys every connection through
it. **A stateless gateway can crash and be replaced, and the connections survive**,
because the state lives at the endpoints.

This is the **fate-sharing** principle Clark later articulated: state should live with
the entity whose failure it shares. The endpoint's state dies when the endpoint dies,
which is acceptable, because there is nothing left to serve.

**3. The network is best-effort.**

Packets may be lost, duplicated, reordered or delayed. The network does not promise
otherwise.

This was the radical part. The telecoms world held that a network must be reliable —
telephone networks were — and the argument for unreliability sounds like an argument
for a worse product.

It is not. Pouzin's CYCLADES had already shown why: **a reliable network must implement
reliability in a way that suits every application, and no such way exists.** Voice wants
timeliness over accuracy; file transfer wants the opposite. A network that guarantees
delivery must retransmit, and retransmission for a voice packet delivers something worse
than silence. A network that guarantees nothing lets each application build exactly what
it needs.

Chapter 38 §38.2 gives the modern version of this argument.

**4. Reliability is the endpoints' job.**

Directly following: hosts detect loss, retransmit, reorder, and discard duplicates. The
network does not help and does not need to.

This is the **end-to-end argument** in embryo, seven years before Saltzer, Reed and
Clark formalised it (§23.4).

## The 1978 split

The 1974 protocol was **one protocol**, called TCP, doing both addressing and
reliability.

By 1978 it was clear that this was wrong, and the reason is instructive. Some
applications did not want reliability:

- **Packet voice** — a retransmitted voice sample arrives too late to play. Better to
  lose it.
- **Simple query/response** — a name lookup with one packet each way does not need
  connection setup, ordering, or congestion control.

Under a single TCP, those applications had to accept machinery they did not want or
bypass the protocol suite entirely.

So TCP was split into two:

```
   1974:  ┌──────────────────────┐
          │         TCP          │   addressing + reliability
          └──────────────────────┘

   1978:  ┌──────────┬───────────┐
          │   TCP    │    UDP    │   reliability, or not
          ├──────────┴───────────┤
          │          IP          │   addressing and forwarding only
          └──────────────────────┘
```

**IP** does addressing and forwarding. **TCP** adds reliability on top. **UDP** adds
essentially nothing — ports and a checksum — for applications that want the network's
raw service.

**This is the most important structural decision in the suite**, and it is what
makes the hourglass of §23.4 possible. If reliability were mandatory, real-time media
could not use IP, and the Internet would have been a data network with a separate voice
network beside it — which is precisely what the telecoms industry expected and built
towards for another twenty years.

The seam from that split is still visible: TCP's checksum covers a pseudo-header
containing IP addresses (§21.4), because in 1974 there was no boundary to respect.

## The flag day

**1 January 1983.** The ARPANET switched from NCP to TCP/IP in a single coordinated
transition. Roughly 400 hosts. Everything that had not converted stopped working.

It succeeded, and it is worth understanding why it could never be repeated. Four
hundred hosts, one administrative authority, a community small enough that everyone
knew everyone, and a hard deadline that could be enforced by simply switching NCP off.

**The Internet has never had another flag day and never will.** Any change since 1983
must be incrementally deployable — working when only some participants have adopted it,
and providing benefit to early adopters.

That constraint is why IPv6 has taken thirty years (Chapter 28 §28.4), why DNSSEC is
patchy (Chapter 39 §39.4), why BGP security remains unsolved (Chapter 32 §32.4), and
why every successful protocol change of the last four decades was designed to work
alongside what it replaces.

**"Can this be deployed incrementally?" is the first question about any Internet
change, and a "no" is usually fatal.**

## Why it won

Chapter 22 §22.1 covered the failure side. The success side, briefly:

**It was free.** RFCs cost nothing. ISO standards cost money. A graduate student could
read the specification, and graduate students built things.

**It was in BSD.** Berkeley's 4.2BSD (1983) shipped with a working TCP/IP stack and the
socket interface, funded by DARPA and distributed at the cost of the tape. Every Unix
vendor derived from it. **The reference implementation was free, complete, and already
running.**

**It worked.** Not "was specified to work" — was working, on the ARPANET, carrying real
traffic, with the bugs found and fixed.

**It was simple enough to implement.** RFC 791 and RFC 793 total about 130 pages. A
determined person can implement them.

**It arrived first.** By the time OSI implementations were purchasable, TCP/IP had the
installed base.

> **Rough consensus and running code.** Clark's slogan is usually quoted as a piece of
> engineering culture. It is more precisely a **statement about what constitutes
> evidence**: a working implementation is proof, and a ratified document is a claim.

## What breaks here

**Assuming the design was inevitable.** It was contested for fifteen years, by serious
people, with governments on the other side.

**Assuming the network provides guarantees.** It provides none. Everything reliable is
built at the endpoints, and knowing this changes how you debug.

**Expecting flag days.** There will not be another one. Every change must be
incremental.

**Thinking of TCP and IP as one thing.** The split is the point. "TCP/IP" as a single
word obscures the decision that made the suite general.

> **Network+ note.** Objective 1.1 expects the TCP/IP model alongside OSI. The history
> is not examined; the structure is. Over-learn: **IP provides addressing and
> forwarding only, and is best-effort**; **TCP adds reliability, UDP does not**; and
> **reliability lives at the endpoints**. These three sentences answer a surprising
> number of questions.
