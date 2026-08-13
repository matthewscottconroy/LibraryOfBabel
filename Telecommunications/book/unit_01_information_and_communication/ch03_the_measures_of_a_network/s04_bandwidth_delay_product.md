# 3.4 The Bandwidth–Delay Product

We now have the pieces to explain the mystery from Chapter 1's diagnostic exercise
and Cheshire's essay, and to derive the single most useful formula in network
performance work.

## The pipe metaphor, cashed out

Think of a link as a pipe. Its **capacity** is the cross-sectional area — how much
can flow per second. Its **latency** is the length — how long the water takes to
traverse it. The question this section answers is: **how much data is inside the
pipe at any instant?**

$$\text{BDP} = \text{bandwidth} \times \text{round-trip time}$$

Units: bits per second × seconds = bits. The bandwidth–delay product is a
*quantity of data*, and it is the amount that can be in flight — sent but not yet
acknowledged — on a fully utilised path.

A worked example. A 1 Gb/s path with a 100 ms round trip:

$$\text{BDP} = 10^9 \ \text{b/s} \times 0.1 \ \text{s} = 10^8 \ \text{bits} = 12.5 \ \text{MB}$$

Twelve and a half megabytes are in flight on that path when it is running at
capacity. If your protocol will not permit that much unacknowledged data, you
cannot fill the pipe. Full stop, regardless of what you paid.

## Why this limits TCP

TCP is a *sliding window* protocol. The sender may transmit up to one window's
worth of unacknowledged data and must then wait for an acknowledgement. We
develop the mechanism properly in Chapter 37; the consequence is available now.

$$\text{max throughput} = \frac{\text{window size}}{\text{RTT}}$$

The original TCP header allocates **16 bits** to the receive window field. Sixteen
bits maxes out at 65,535 bytes. So without extensions:

$$\text{max throughput} = \frac{65{,}535 \times 8}{\text{RTT}} = \frac{524{,}280}{\text{RTT}} \ \text{b/s}$$

Tabulated, and this is the table Cheshire was pointing at:

| RTT | Max single-stream throughput (64 KB window) |
|---|---|
| 1 ms | 524 Mb/s |
| 10 ms | 52.4 Mb/s |
| 50 ms | 10.5 Mb/s |
| 100 ms | 5.2 Mb/s |
| 200 ms | 2.6 Mb/s |
| 500 ms (GEO satellite) | 1.05 Mb/s |

**None of these numbers mention the link's capacity, because the link's capacity
is not involved.** A 10 Gb/s transatlantic circuit with a 64 KB window delivers
5.2 Mb/s to a single TCP stream. This is not a hypothetical; it is what actually
happened on research networks in the early 1990s, and it is what still happens
today on any host where someone has capped the window.

## Window scaling: the fix

RFC 1323 (1992, superseded by RFC 7323 in 2014) introduced the **window scale
option**, negotiated in the SYN packets of the three-way handshake. It specifies a
left-shift count applied to the advertised window, up to 14, extending the
maximum window from 64 KB to 2³⁰ = 1 GB.

With scaling, our 1 Gb/s / 100 ms path needs a 12.5 MB window, which is entirely
achievable — and modern operating systems auto-tune the buffer up to it. Linux's
`net.ipv4.tcp_rmem` and `tcp_wmem` set the floor, default, and ceiling; the stack
grows the window dynamically based on observed BDP.

Three things still go wrong in practice, and all three are common:

**The ceiling is too low.** Auto-tuning cannot exceed the configured maximum. A
default `tcp_rmem` maximum of 6 MB caps a 100 ms path at 480 Mb/s no matter what
the link can do. On long-fat-network paths this must be raised explicitly, and
"why does my 10 Gb/s transatlantic link only do 400 Mb/s" is almost always this.

**A middlebox stripped the option.** Window scaling is negotiated only in the SYN.
Some old firewalls and load balancers remove unknown TCP options. If the option is
stripped in one direction, the two ends disagree about the scale factor, and the
result is not a clean failure but a connection that stalls mysteriously after the
first 64 KB. Chapter 66 covers the capture signature.

**The application's own buffer is the limit.** If the receiving application does
not read fast enough, the receive window shrinks regardless of the kernel's
willingness. Symptom in a capture: TCP Zero Window messages. The network is
innocent.

## Long fat networks

A path with a large bandwidth–delay product is called a **long fat network**, and
the acronym LFN is pronounced "elephan(t)" in RFC 1323, which is the only joke in
the document.

LFNs behave differently in ways beyond window size:

- **Loss is far more expensive.** A retransmission costs a full RTT, and classic
  congestion control halves the window on loss, then rebuilds it linearly — which
  on a 100 ms path takes many seconds. This is why the Mathis equation's
  sensitivity (§3.3) bites hardest exactly where bandwidth is most abundant.
- **Slow start takes real time.** Starting from a 10-segment initial window and
  doubling each RTT, reaching a 12.5 MB window needs about 10 round trips — a full
  second on a 100 ms path, during which you are not at capacity. For short
  transfers, you never reach capacity at all, which is why HTTP connection reuse
  and QUIC's 0-RTT resumption matter so much (Chapter 38).
- **Parallelism wins.** *N* parallel streams each get their own window. This is
  why `iperf3 -P 16` reports so much more than `iperf3`, why download accelerators
  worked, and why the Tokyo transfer in Chapter 1's exercise jumped from 3 Mb/s to
  48 Mb/s on sixteen connections. Nothing about the network changed; sixteen
  windows were in flight instead of one.

## The design lesson

The bandwidth–delay product is a general principle and not a TCP quirk. Any
protocol that requires an acknowledgement before proceeding is limited by

$$\text{rate} \le \frac{\text{data permitted in flight}}{\text{round-trip time}}$$

You will meet this again in several costumes: in the 802.11 acknowledgement that
follows every Wi-Fi frame (Chapter 44), in the stop-and-wait behaviour of older
SMB versions that made file access over a WAN unbearable, in iSCSI and NFS tuning,
and in the reason a chatty application protocol that performs twelve round trips
per operation is unusable over a satellite link no matter how much bandwidth it
has.

The general form: **round trips are the expensive thing.** When you design a
protocol, or diagnose an application, count the round trips first. Bandwidth is
cheap and getting cheaper; the speed of light has been stable for some time.

## What breaks here

- **A fast link delivering slow single-stream transfers.** Check window size
  against BDP before anything else. `ss -i` on Linux shows the negotiated scale
  factor and current window.
- **Throughput that plateaus at exactly the same value regardless of the link.**
  A hard number like 5.2 Mb/s or 480 Mb/s that does not move is a window ceiling,
  not a network problem.
- **A connection that transfers 64 KB and stalls.** Window scale option stripped
  by a middlebox.
- **An application that is fine on the LAN and unusable over the WAN, with low
  utilisation throughout.** Round-trip count, not bandwidth. Count the exchanges
  in a capture and multiply by RTT.

> **Network+ note.** N10-009 does not use the term "bandwidth–delay product." It
> does present scenarios — a high-speed WAN link with poor transfer performance
> and low utilisation — whose only correct answer requires this reasoning. The
> distractors will offer more bandwidth, a faster router, and a cable
> replacement. None of them is right.
