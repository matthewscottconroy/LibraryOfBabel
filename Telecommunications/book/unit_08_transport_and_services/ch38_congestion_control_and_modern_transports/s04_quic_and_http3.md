# 38.4 QUIC and HTTP/3

QUIC is the largest change to Internet transport since TCP, it carries a substantial
fraction of the world's web traffic, and it exists because TCP could not be changed.

That last clause is the important one, and it is the culmination of an argument this book
has been building since Chapter 21.

## Why TCP could not be changed

**Three independent obstacles, each sufficient on its own.**

### 1. Ossification

**Middleboxes inspect and modify TCP** (Chapter 21 §21.4), and they were built to
understand the TCP of the year they shipped.

Honda et al. measured this in 2011 by attempting to deploy new TCP options across the
real Internet. The findings:

- **A meaningful fraction of paths stripped unknown TCP options**
- Some **dropped packets carrying them entirely**
- Some **rewrote sequence numbers**, breaking anything that depended on them
- Some **normalised** the header, removing anything unfamiliar

> **A new TCP option cannot be deployed, because a meaningful fraction of the Internet will
> silently discard or block it.**

**And "silently" is the fatal part.** A feature that fails visibly can be detected and
disabled; one that fails silently on 8% of paths produces a service that is subtly broken
for users you cannot identify.

### 2. The kernel

**TCP lives in the operating system kernel.** Changing it means shipping a kernel update to
every client on Earth and waiting years for adoption.

**Google could deploy a new algorithm on its servers in a week and could not deploy
anything requiring a client-side kernel change at all.**

### 3. Head-of-line blocking is architectural

HTTP/2 multiplexes many streams over one TCP connection — a genuine improvement over
HTTP/1.1's six parallel connections.

**And TCP does not know the streams exist.** A single lost packet blocks **every** stream
until it is retransmitted (Chapter 36 §36.2), because TCP must deliver its byte stream in
order.

```
   HTTP/2 over TCP:
   stream A ──▶ ▓▓▓▓ ✗ ▓▓▓▓        one packet lost in stream A...
   stream B ──▶ ▓▓▓▓▓▓▓▓▓▓
   stream C ──▶ ▓▓▓▓▓▓▓▓▓▓
                     ↓
   Delivered: NOTHING from B or C either, until A's loss is repaired
```

**HTTP/2 knows the streams are independent. TCP cannot be told.** Chapter 21 §21.3's
information barrier, costing real performance at scale.

**On a lossy path, HTTP/2 over TCP was measurably *worse* than HTTP/1.1's six
connections** — because six connections meant a loss stalled one sixth of the work rather
than all of it.

## The answer

**Build a new transport, in user space, over UDP.**

```
   ┌──────────────────────────────────────┐
   │              HTTP/3                  │
   ├──────────────────────────────────────┤
   │   QUIC — streams, reliability,       │  ← user space (a library)
   │   flow control, congestion control,  │
   │   TLS 1.3, connection migration      │
   ├──────────────────────────────────────┤
   │              UDP                     │  ← kernel, minimal
   ├──────────────────────────────────────┤
   │              IP                      │
   └──────────────────────────────────────┘
```

**Each choice answers one of the three obstacles:**

| Choice | Answers |
|---|---|
| **Over UDP** | ossification — middleboxes pass UDP without understanding it |
| **In user space** | the kernel — ships with the application, updates with it |
| **Streams in the transport** | head-of-line blocking — QUIC knows the streams exist |

**And a fourth choice compounds the first:** QUIC encrypts almost all of its header, so
middleboxes **cannot** inspect it, **cannot** modify it, and **cannot** ossify it. Only the
minimum needed for routing is visible.

> **QUIC is designed to be un-ossifiable.** Having watched middleboxes freeze TCP for
> twenty years, its designers made the protocol's internals invisible to them.

## What QUIC provides

### Independent streams — the central feature

**Each stream has its own sequence space.** A loss in one blocks only that one:

```
   QUIC:
   stream A ──▶ ▓▓▓▓ ✗ ▓▓▓▓        one packet lost in stream A...
   stream B ──▶ ▓▓▓▓▓▓▓▓▓▓        → delivered
   stream C ──▶ ▓▓▓▓▓▓▓▓▓▓        → delivered
```

This is the thing TCP structurally cannot do, and it is why HTTP/3 exists.

### A faster handshake

TCP + TLS 1.3 requires two round trips before application data — one for TCP's SYN
exchange, one for TLS.

**QUIC combines them into one**, because the transport and cryptographic handshakes are the
same handshake:

| | Round trips to first byte |
|---|---|
| TCP + TLS 1.2 | **3** |
| TCP + TLS 1.3 | **2** |
| **QUIC, first connection** | **1** |
| **QUIC, resumed (0-RTT)** | **0** |

0-RTT sends application data in the very first packet, using keys cached from a previous
connection.

**With a caveat that matters:** 0-RTT data is **replayable** — an attacker who captures it
can send it again. So it is safe only for idempotent requests, and the protocol requires
applications to know the difference. **A GET is fine; a POST that charges a credit card is
not.**

### Connection migration

A QUIC connection is identified by a Connection ID, not by the five-tuple (Chapter 35
§35.2).

**So changing IP address does not break it.** A phone moving from Wi-Fi to cellular keeps
its connection, its congestion state, and its cryptographic keys — **the download does not
restart.**

**TCP cannot do this**, because the connection *is* the five-tuple. Change any field and it
is a different connection.

This is a genuine capability TCP has no path to, and on mobile networks it is
substantial.

### Encryption is not optional

**QUIC has no unencrypted mode.** TLS 1.3 is part of the protocol, not a layer above it.

**Which is a security decision and also an ossification defence:** there is no plaintext
version for a middlebox to learn to parse, so there is no version to freeze.

## HTTP/3

**HTTP semantics over QUIC.** The methods, headers and status codes are unchanged from
HTTP/2 — what changes is what carries them.

| | HTTP/1.1 | HTTP/2 | HTTP/3 |
|---|---|---|---|
| Transport | TCP | TCP | **QUIC (UDP)** |
| Multiplexing | **no** — 6 connections | yes, one connection | **yes, independent streams** |
| Head-of-line blocking | per connection | **whole connection** | **per stream only** |
| Header compression | none | HPACK | **QPACK** |
| Encryption | optional | effectively required | **mandatory** |
| Handshake RTTs | 3 (with TLS 1.2) | 2 | **1, or 0** |

**QPACK exists because HPACK could not work.** HTTP/2's header compression assumes headers
arrive in order — which QUIC's independent streams do not guarantee. **QPACK is HPACK
redesigned to tolerate out-of-order delivery**, and it is a good small example of how a
change at one layer forces changes above it.

## Deployment

**Adoption has been fast by Internet standards:**

| | |
|---|---|
| Google's experimental QUIC | 2012 |
| **IETF standardisation** | **RFC 9000, May 2021** |
| Chrome, Firefox, Safari, Edge | all support HTTP/3 |
| Cloudflare, Google, Meta, Akamai, Fastly | all serve it |
| **Share of web traffic** | **roughly 25–30% and rising** |

Why so much faster than IPv6 (Chapter 28 §28.1)? Because **the deploying party benefits
immediately**:

- A site enabling HTTP/3 gets faster page loads for its own users today
- It **needs nobody else's cooperation** — clients already support it, and the fallback to
  HTTP/2 is automatic
- No coordination, no flag day, no network effect to wait for

> **QUIC is the counter-example that proves Chapter 28's point.** IPv6's benefit is
> entirely a network effect and it took thirty years; QUIC's benefit is immediate and
> unilateral and it took five.

## The costs

**Honest, because they are real.**

**CPU.** User-space processing and per-packet encryption cost more than kernel TCP with
hardware offload. QUIC has historically used 2–3× the CPU per byte, though the gap is
closing with offload support and better implementations.

**No hardware offload, yet.** TCP benefits from decades of NIC offloads (Chapter 21 §21.3);
QUIC is only beginning to.

**Operational opacity.** You cannot inspect QUIC on the wire. Middleboxes that
performed useful functions — traffic classification, some security inspection, performance
monitoring — cannot. **This is deliberate**, and it is a genuine loss for network operators
alongside a genuine gain for users.

**Debugging changed.** No `tcpdump` analysis of the transport; you need endpoint logging
(**qlog**) or the keys. Chapter 64's toolbox needs different tools.

**UDP is sometimes blocked or throttled.** Some networks rate-limit UDP or block UDP/443
outright. The fallback to TCP is automatic and works, so the failure is invisible —
which means a network blocking UDP/443 makes its users' browsing slower and nobody
notices.

## What this means for the model

QUIC does not fit the layer model (Chapter 22 §22.3), deliberately:

- It is a transport protocol inside a transport protocol
- It merges transport, security and part of the session layer
- It lives in **user space**, so the kernel/application boundary is in a new place
- It is **application-specific enough** that HTTP/3 and QUIC were designed together

**This is Chapter 21 §21.4's conclusion in its final form:**

> **Layering is a design discipline, not a law. It is violated where the cost of honouring
> it exceeds the benefit — and QUIC is the largest deliberate violation in modern
> networking because the boundary had come to cost more than it was worth.**

## What breaks here

**HTTP/3 never being used despite browser and server support.** UDP/443 blocked or
throttled. The TCP fallback hides it.

**0-RTT causing duplicate operations.** Replayable data used for a non-idempotent request.
An application bug, and a serious one.

**Higher CPU on a server after enabling HTTP/3.** Expected. Measure before assuming it is
worth it.

**Losing traffic visibility after a QUIC migration.** Deliberate. Plan endpoint logging
before, not after.

**A firewall rule permitting TCP/443 only.** It blocks HTTP/3, and everything still works
slightly worse.

> **Network+ note.** Objective 1.4 may mention QUIC and HTTP/3. Over-learn: **QUIC runs
> over UDP port 443**; **it provides reliability, ordering and congestion control in user
> space**; **it eliminates head-of-line blocking between streams**; **encryption is
> mandatory**; and **HTTP/3 is HTTP over QUIC.** The practical point for a network
> professional is that **blocking UDP/443 blocks HTTP/3** and the failure is silent.
