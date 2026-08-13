# 24.1 Best-Effort Delivery

IP promises nothing. It will attempt to deliver your packet and it makes no
commitment about whether it succeeds, when, in what order, or how many times.

Stated that way it sounds like a failure of engineering. It is the opposite: **the
most consequential design decision in the entire suite**, argued about for a decade,
and correct.

## What "best-effort" means precisely

IP does not guarantee:

| Not guaranteed | Meaning |
|---|---|
| **Delivery** | The packet may be dropped, and nobody will tell you |
| **Ordering** | Packet 2 may arrive before packet 1 |
| **No duplication** | The same packet may arrive twice |
| **Timeliness** | It may take 5 ms or 5 seconds |
| **Integrity of the payload** | The header is checksummed; the payload is not |
| **Notification of failure** | ICMP *may* report a problem; it may not |

What IP does guarantee is narrower and worth stating:

> **If a packet is delivered, its header was not corrupted in a way the checksum
> detects, and it was delivered to the address in that header.**

That is all. Everything else — reliability, ordering, deduplication, timeliness — is the
endpoints' problem (Chapter 23 §23.4).

## Why packets are dropped

Not because something is broken. **Dropping is normal operation.**

**Congestion.** The commonest cause by far. Traffic arrives at a router faster than an
outbound interface can transmit. The router buffers what it can and **drops what it
cannot** (Chapter 17 §17.4). Nothing has failed; the demand exceeded the capacity for a
moment.

**TTL expiry.** The hop limit reached zero (§24.4). Either the packet was in a loop, or
the path is longer than the sender allowed.

**No route.** The router has nothing matching the destination, not even a default. It
drops and — usually — sends an ICMP Destination Unreachable.

**Header checksum failure.** Corruption in the header. The packet cannot be trusted to
be going where it says.

**Policy.** An access list or firewall rule says no.

**Buffer exhaustion.** Sustained overload; even the queue is full.

**The mental adjustment:** a network with zero packet loss is a network that is
underutilised. TCP *needs* loss to discover the available capacity (Chapter 38 §38.2),
and a small, steady loss rate on a busy link is the system working as designed. Chapter
54 §54.3 is about telling that apart from a fault, and the distinction is not obvious.

## Why this was the right choice

Four arguments, and they compound.

**1. Applications disagree about what they want.**

| Application | Wants | Would a reliable network help? |
|---|---|---|
| File transfer | every byte, eventually | yes |
| Live voice | timely delivery, gaps tolerable | **no — retransmission is worse than loss** |
| Live video | timely, some loss tolerable | no |
| DNS query | one packet, fast, retry if lost | mostly no |
| Bulk backup | every byte, speed secondary | yes |
| Online game | most recent state, old state worthless | **no** |

A network that guarantees delivery must retransmit. **For live voice a retransmitted
sample arrives after its playout deadline** — it is not merely useless but harmful,
because delivering it costs bandwidth and delays what follows.

**A network cannot serve both without serving one badly.** Providing nothing and
letting each application build what it needs serves both.

**2. Simple routers scale and survive.**

A router keeping no per-flow state:

- Can be **replaced mid-conversation** without breaking anything (Chapter 23 §23.1's
  fate-sharing)
- Has **memory proportional to the routing table, not to traffic**
- Can be built in hardware, because forwarding is a table lookup
- **Cannot leak state, exhaust state, or become inconsistent**

A router that guaranteed delivery would need per-packet state: what was sent, what was
acknowledged, what to retransmit. That is per-flow state at every hop, and it is what
made X.25 and ATM virtual circuits expensive and fragile.

**3. Reliability at the endpoints is the only place it can be complete.**

§23.4's argument. The endpoint must verify regardless, so network-level guarantees are
redundant for correctness.

**4. It permits partial failure.**

A best-effort network that loses 2% of packets still works. Applications degrade —
slower transfers, a click in the audio — rather than stopping.

A guaranteed network that cannot guarantee has to do something, and the options are all
bad: fail the connection, buffer indefinitely, or admit that the guarantee was
conditional.

**Graceful degradation is a property of promising little.**

## The comparison that makes it concrete

| | Circuit-switched (telephone) | Virtual circuit (X.25, ATM) | Datagram (IP) |
|---|---|---|---|
| State in the network | per call | per circuit | **none** |
| Setup before data | **required** | **required** | none |
| Guaranteed capacity | yes | yes | **no** |
| Behaviour under overload | **call blocked** | circuit refused | **degradation** |
| Node failure | call drops | circuit drops | **reroute, survive** |
| Efficiency for bursty traffic | **terrible** | moderate | **excellent** |
| Cost per node | high | high | **low** |

The last row decided it commercially; the fourth and fifth decided it technically.

**Under overload, a telephone network refuses new calls** — which is why you could not
place a call after a major incident, while the Internet merely slowed down. Chapter 3
§3.4 covered the history; the design consequence is here.

## What best-effort forced into existence

Every guarantee the network declines to make has to be built somewhere, and the list is
essentially Units VIII–XI:

| Missing guarantee | Built by | Chapter |
|---|---|---|
| Reliability | TCP's acknowledgements and retransmission | 36 |
| Ordering | TCP's sequence numbers | 36 |
| Deduplication | TCP's sequence numbers | 36 |
| Flow control | TCP's window | 36 |
| Congestion control | TCP's congestion window | 36 |
| Timeliness | QoS scheduling, and application adaptation | 55 |
| Integrity of payload | TCP/UDP checksums, and TLS | 36, 60 |
| Security | TLS, IPsec | 60 |

**This is not duplicated work.** It is work done once, at the endpoints, where it can be
done correctly and where the application can choose how much of it it wants.

## What best-effort does not excuse

An important boundary, because "best-effort" is sometimes offered as an explanation for
a network that is simply broken.

Best-effort means the *protocol* makes no guarantee. It does not mean **an operator
should tolerate loss**. A well-run network delivers essentially everything it is asked
to, and:

- Loss on an **uncongested** link is a fault — a bad cable, a failing transceiver, a
  duplex mismatch
- **Sustained** loss above a fraction of a per cent degrades TCP badly (Chapter 38
  §38.2's Mathis equation makes this quantitative)
- Loss that is **bursty rather than random** hurts far more than its average suggests

**"IP is best-effort" is an architectural statement, not an operational excuse.**
Chapter 54 §54.3 covers what loss rates are acceptable and how to tell congestion from
corruption.

## What breaks here

**Expecting the network to guarantee delivery.** It does not, and a design that assumes
otherwise fails in production.

**Treating any loss as a fault.** Loss during congestion is normal. Loss on an idle link
is not.

**Building an application with no retry logic** on the assumption that UDP will get
there. It frequently will not.

**Assuming zero loss is the goal.** Zero loss on a busy link means the buffers are too
large, which is bufferbloat (Chapter 13), and the latency cost is worse than the loss
would have been.

> **Network+ note.** Objective 1.1 expects **IP as connectionless and best-effort** and
> the contrast with TCP's connection-oriented reliability. This is examined repeatedly,
> often as "which protocol guarantees delivery?" Over-learn: **IP guarantees nothing;
> TCP adds reliability; UDP adds none.**
