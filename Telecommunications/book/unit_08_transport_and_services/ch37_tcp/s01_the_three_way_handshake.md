# 37.1 The Three-Way Handshake

TCP is a reliable, ordered byte stream built on a network that guarantees nothing
(Chapter 24 §24.1). Everything in this chapter is machinery for that, and the handshake is
where it starts.

**The question the handshake answers is narrower and more interesting than "let's
connect".**

## The problem

**Two hosts must agree on where the byte stream starts.**

Every byte TCP sends carries a **sequence number** (§37.2), and the two directions are
independent — each side numbers its own stream. **So before any data flows, each side must
tell the other its starting number and be sure the other received it.**

And that is harder than it sounds, because the network may duplicate, delay and
reorder. A connection request from an hour ago could arrive now.

## The exchange

```
   Client                                          Server
     │                                               │
     │  ── SYN,  seq=x ─────────────────────────────▶│   "my stream starts at x"
     │                                               │
     │◀───────────────── SYN-ACK, seq=y, ack=x+1 ────│   "mine starts at y,
     │                                               │    and I got yours"
     │  ── ACK,  seq=x+1, ack=y+1 ──────────────────▶│   "I got yours"
     │                                               │
     │═══════════ connection established ════════════│
```

**Three packets, and each is necessary:**

| Packet | Establishes |
|---|---|
| **SYN** | the client's initial sequence number, sent |
| **SYN-ACK** | the server's ISN sent, **and the client's acknowledged** |
| **ACK** | the server's ISN acknowledged |

**Why three and not two?** Because **both directions must be confirmed**, and a two-packet
exchange leaves the server's sequence number unacknowledged — the server would not know
whether the client received its SYN-ACK.

**Why not four?** Because the server's acknowledgement of the client's SYN and its own SYN
travel in the same packet. The middle packet does two jobs, which is why the count is
odd.

> **The handshake is a mutual agreement about two numbers, and three messages is the
> minimum that confirms both.**

## The `+1`

The acknowledgement is `x+1`, not `x`, and this is examined and is worth understanding
rather than memorising.

**SYN consumes one sequence number**, despite carrying no data. So does FIN (§37.5).

**Why?** Because acknowledgements must be able to distinguish *"I received your SYN"* from
*"I received nothing"*. If SYN consumed no sequence space, the ACK for it would be
indistinguishable from an ACK of nothing.

**The convention:** an acknowledgement number is **the next sequence number expected**, so
acknowledging a SYN with sequence `x` means expecting `x+1` next.

## Initial sequence numbers

**They are not zero, and the reason is security.**

### The old way — and the attack

Early implementations used a simple incrementing counter. **Which meant an attacker could
predict the server's next ISN**, and predicting it enables **blind spoofing**:

1. The attacker sends a SYN **spoofing a trusted client's address**
2. The server's SYN-ACK goes **to the trusted client**, not the attacker
3. The attacker cannot see it — but if they can predict `y`, they do not need to
4. They send the third ACK with `ack=y+1`, and **the connection is established from an
   address they do not control**

This is the attack Kevin Mitnick used against Tsutomu Shimomura in December 1994, and
it is the most famous demonstration of why a protocol's random-looking numbers must
actually be random.

Bellovin had described the vulnerability in 1989 (Chapter 24's notes) — five years
before it was used publicly.

### The fix

**RFC 6528** specifies that the ISN must be unpredictable to an off-path attacker:

$$\text{ISN} = M + F(\text{src IP}, \text{src port}, \text{dst IP}, \text{dst port}, \text{secret})$$

where *M* is a timer and *F* is a cryptographic hash. **Unpredictable without the secret,
and deterministic for a given tuple**, which preserves the old-connection protection below.

This is why the handshake provides weak authentication as a side effect (Chapter 36
§36.4): an off-path attacker cannot complete it, because they cannot guess `y`. UDP has
no equivalent, which is why UDP spoofing is trivial and TCP spoofing is not.

**It is weak** — an on-path attacker sees `y` and can do as they like — but it eliminates
the entire off-path attacker class for free.

## What the handshake actually negotiates

**More than sequence numbers.** The SYN and SYN-ACK carry **options**, and these determine
much of the connection's behaviour:

| Option | Purpose | Chapter |
|---|---|---|
| **MSS** | maximum segment size this side will accept | §37.2 |
| **Window scale** | multiply the window field by 2^n | §37.4 |
| **SACK permitted** | selective acknowledgement is supported | §37.3 |
| **Timestamps** | RTT measurement and PAWS | §37.3 |

All of these are negotiated once, in the handshake, and cannot be changed afterwards.

**Which has a consequence worth stating:** if the SYN or SYN-ACK is modified or stripped by
a middlebox, the connection runs without that feature for its entire life. A middlebox
that strips the window-scale option limits the connection to 64 KB in flight forever
(§37.4), and the symptom is a connection that works and is inexplicably slow.

This is Chapter 21 §21.4's ossification, and it is why QUIC encrypts its handshake.

### MSS, and why it is not the MTU

MSS is the largest *payload* a segment may carry — it excludes the TCP and IP headers.

$$\text{MSS} = \text{MTU} - 20\ (\text{IP}) - 20\ (\text{TCP}) = 1500 - 40 = \mathbf{1460}$$

Each side announces what it is willing to receive, and **the smaller of the two
values is used** in each direction. It is not negotiated in the sense of agreement — each
side simply states a limit and the other respects it.

**MSS clamping** (Chapter 34 §34.4) works by rewriting this option in transit, which is
how a VPN gateway prevents PMTUD black holes without touching the endpoints.

## SYN flood

The handshake's cost is asymmetric, and that asymmetry is an attack.

When a server receives a SYN, it must allocate state — a Transmission Control Block
holding the sequence numbers, the negotiated options, and timers — and hold it while
waiting for the third packet.

An attacker sends many SYNs and never completes any of them:

```
   Attacker ── SYN ──▶ Server   allocates state, sends SYN-ACK, waits
   Attacker ── SYN ──▶ Server   allocates state...
   Attacker ── SYN ──▶ Server   ...
                                (never sends the third ACK)
```

**The backlog fills, and legitimate connections are refused.**

The attacker's cost is one packet; the server's cost is state held for a timeout.
And with a spoofed source, the SYN-ACKs go to innocent third parties.

**Visible in the socket table** (Chapter 35 §35.4) as many sockets in `SYN-RECV`.

### SYN cookies — the elegant defence

**Bernstein's answer, 1996, and it is genuinely clever.**

**Do not allocate state at all.** Instead, **encode the state in the sequence number you
send back**:

$$y = \text{hash}(\text{tuple},\ \text{secret},\ t) \ \|\ \text{encoded MSS}\ \|\ t$$

**When the third ACK arrives**, it carries `ack = y+1`. The server **recomputes the hash**
from the tuple in the packet and checks it matches. If it does, the client must have
received the SYN-ACK, so the handshake is genuine — **and the state is reconstructed from
the acknowledgement number itself.**

> **The server holds no state until the connection is proven real.** A SYN flood costs the
> server one hash computation per packet and nothing else.

**The cost:** TCP options cannot be fully encoded in 32 bits, so **window scale, SACK and
timestamps may be lost** on cookie-established connections. Which is why SYN cookies are
enabled **as a fallback under attack** rather than always:

```bash
sysctl net.ipv4.tcp_syncookies=1     # 1 = only when the backlog overflows
```

**This is the right setting**, and it is the default on most systems.

## Reading a handshake

```
$ tcpdump -nn -S 'tcp[tcpflags] & (tcp-syn|tcp-ack) != 0'

10.0.0.5.51234 > 203.0.113.10.443: Flags [S],  seq 1043208215,
    win 64240, options [mss 1460,sackOK,TS val 3421 ecr 0,nop,wscale 7]

203.0.113.10.443 > 10.0.0.5.51234: Flags [S.], seq 2588791345, ack 1043208216,
    win 65160, options [mss 1460,sackOK,TS val 991 ecr 3421,nop,wscale 7]

10.0.0.5.51234 > 203.0.113.10.443: Flags [.],  ack 2588791346, win 502
```

**Read four things:**

**`[S]`, `[S.]`, `[.]`** — SYN, SYN-ACK, ACK. (`tcpdump` writes ACK as `.`)

`ack 1043208216` is `seq 1043208215 + 1` — the `+1` for SYN.

**`wscale 7`** — window scale of 2⁷ = 128, so the real window is the field × 128 (§37.4).
**Both sides offered it, so it is in effect.**

**`win 502` on the third packet** — 502 × 128 = 64,256 bytes. **The scale is now applied.**

## The failure modes

**What you see, and what it means:**

| Observed | Meaning |
|---|---|
| SYN sent, **RST received** | **Nothing is listening.** The host is up and refused. |
| SYN sent, **nothing at all** | **A firewall is dropping**, or the route is asymmetric |
| SYN, SYN-ACK, **no third ACK** | rare; a broken client or an on-path problem |
| SYN, SYN-ACK, ACK, **then RST** | the application refused after accepting |
| **Repeated SYNs, no reply** | retransmission — the standard backoff is 1, 2, 4, 8… seconds |
| SYN-ACK with **no options** | a middlebox stripped them; the connection will underperform |

The first two are Chapter 22 §22.4's distinction, and it is the highest-value
observation in this section: a RST is an answer and silence is a firewall.

## What breaks here

**Connections refused immediately.** RST — nothing listening. Check `ss -tlnp`.

**Connections timing out.** Silence — a firewall, or an asymmetric route.

**A connection that establishes and is inexplicably slow.** Options stripped in the
handshake, most often window scale. Capture the SYN and look.

**Many sockets in `SYN-RECV`.** A SYN flood, or a very slow client population. Enable SYN
cookies.

**A connection that establishes and immediately gets RST.** The application accepted and
then closed — a backlog overflow, or an application-level rejection.

> **Network+ note.** Objective 1.4 expects the three-way handshake. **This is examined
> directly and frequently.** Over-learn: **SYN, SYN-ACK, ACK**; **the acknowledgement is
> the sequence number plus one, because SYN consumes a sequence number**; **a RST means
> nothing is listening**; and **a SYN flood exhausts the server's half-open connection
> state, mitigated by SYN cookies.**
