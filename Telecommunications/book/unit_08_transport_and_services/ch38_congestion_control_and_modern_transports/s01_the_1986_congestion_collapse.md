# 38.1 The 1986 Congestion Collapse

In October 1986 the link between Lawrence Berkeley Laboratory and the University of
California, Berkeley — 400 metres apart — dropped from 32 kb/s to 40 bits per second.

**A factor of a thousand.** Nothing was broken. No cable failed, no router crashed, no
configuration changed.

The network had discovered a failure mode nobody had designed for, and understanding it
is the reason everything else in this chapter exists.

## What happened

**The mechanism, step by step:**

1. **Traffic increases.** More users, more transfers.
2. A router's queue fills and it begins dropping packets (Chapter 24 §24.1).
3. Senders time out and retransmit — which is correct behaviour (Chapter 37 §37.3).
4. The retransmissions add to the load.
5. More packets are dropped.
6. More timeouts, more retransmissions.
7. The link is now carrying almost entirely retransmissions of packets that will be
   dropped.

```
   Offered load  ────────────────▶
   Useful throughput
        │
        │      ╱▔▔▔▔╲
        │    ╱        ╲
        │  ╱            ╲___________________  ← collapse
        │╱
        └──────────────────────────────────▶  offered load
              capacity
```

**The curve is the important part.** Throughput rises with load, peaks, **and then falls** —
and it does not recover when load is removed slightly, because the retransmissions are
themselves load.

> Congestion collapse is not degradation. It is a stable state in which the network
> carries almost nothing useful and is completely busy doing so.

## Why the design permitted it

TCP as specified in 1981 had no congestion control.

It had flow control (Chapter 37 §37.4) — the receiver's advertised window, protecting
the receiver from being overwhelmed.

**It had nothing protecting the network.** A sender that could push a full window would
push a full window, regardless of what the path could carry.

The assumption was that the receiver was the bottleneck, which was true when hosts were
slow and links were dedicated. By 1986 links were shared and hosts were fast, and the
assumption had quietly inverted.

**And the retransmission timers made it worse.** RFC 793's estimator (Chapter 37 §37.3)
tracked the mean and ignored the variance, so under congestion — where delay becomes highly
variable — it fired early, adding retransmissions of packets that were merely slow.

## Jacobson's response

Van Jacobson and Michael Karels, at Berkeley, diagnosed it and fixed it in a few
months. The fixes went into 4.3BSD and were described in the 1988 SIGCOMM paper
*Congestion Avoidance and Control*.

**Seven mechanisms**, and they are the foundation of everything since:

| Mechanism | Purpose |
|---|---|
| **Slow start** | do not begin at full speed; probe upward |
| **Congestion avoidance** | grow cautiously once near capacity |
| **The congestion window (`cwnd`)** | a sender-side limit, separate from the receiver's |
| **Better RTT estimation** | track the variance (Chapter 37 §37.3) |
| **Exponential backoff** | on repeated loss |
| **Fast retransmit** | recover without a timeout (Chapter 37 §37.2) |
| **Karn's algorithm** | do not measure ambiguous samples |

The paper's central insight is the "conservation of packets" principle:

> A connection in equilibrium should put a new packet into the network only when an old
> one leaves it.

The acknowledgement is the signal that a packet left. So a sender that transmits only
on receiving an ACK is *self-clocking* — it automatically sends at exactly the rate the
path can sustain, without knowing what that rate is.

This is the elegant heart of TCP congestion control, and everything else is machinery
for getting into equilibrium and recovering when it breaks.

## The two windows

**The addition that fixed it:**

$$\text{bytes in flight} \le \min(\underbrace{\text{rwnd}}_{\text{receiver}},\ \underbrace{\text{cwnd}}_{\text{network}})$$

| | Set by | Protects | Signalled by |
|---|---|---|---|
| **`rwnd`** | **the receiver** | the receiver | the window field |
| **`cwnd`** | **the sender itself** | **the network** | **inferred from loss** |

**`cwnd` is not in any packet.** No receiver advertises it; no router requests it. The
sender maintains it by inference, and the only information it has is which segments were
acknowledged and when.

This is a remarkable thing to have made work. The network provides no explicit
congestion signal (ECN came later, §38.3), so every TCP sender is inferring the state of a
shared resource from indirect evidence, independently, with no coordination — and the
result is a stable, roughly fair allocation.

## Loss as the congestion signal

The inference that makes it work, and the assumption it rests on:

> A dropped packet means a queue overflowed, which means the path is congested.

On a wired network this is almost always true. Bit-error rates on fibre are around
10⁻¹² — loss from corruption is negligible, so loss means queueing.

And it is the assumption that fails on wireless (Chapter 21 §21.3), where loss is
frequently interference. TCP slows down in response to a problem that slowing down does
not fix, and the link layer's retransmission (Chapter 43) exists partly to hide this from
TCP.

It also fails on very high-speed paths, where the loss rate needed to keep a window
full becomes absurdly small (§38.2's Mathis equation).

Every congestion-control algorithm since is, in one way or another, an attempt to get a
better signal than loss.

## Fairness

**A question the design had to answer:** several connections share a link. What
allocation should they converge to, and does the algorithm reach it?

The answer is AIMD, and §38.2 shows why it converges. Note here only that the
allocation TCP produces is "fair" in a specific and limited sense:

- Equal `cwnd` among flows with equal RTT — which is fair
- But throughput is `cwnd`/RTT, so a flow with half the RTT gets **twice the
  throughput**

TCP is biased toward short paths, structurally. A local connection competing with an
intercontinental one on the same bottleneck will take a much larger share, and neither is
misbehaving.

And an application that opens many connections gets many shares. Which is why browsers
opened six connections per host under HTTP/1.1 — not to be greedy, but because one
connection's share was inadequate — and why HTTP/2's single multiplexed connection was
initially *slower* on lossy paths until the transport caught up.

## Why this chapter matters beyond history

Congestion collapse is not a solved problem; it is a contained one.

The containment depends on nearly all traffic being congestion-responsive. Chapter 36
§36.4's argument follows directly: a UDP application without congestion control is
relying on everyone else's restraint, and a network in which enough traffic behaves that
way can collapse again.

It has happened since, on smaller scales — in data centres with incast (Chapter 17
§17.4), on links carrying large volumes of unresponsive UDP, and in the "bufferbloat"
regime where huge buffers delay the loss signal until the queue is seconds deep
(Chapter 13).

> The Internet works because most senders voluntarily slow down when they detect
> congestion. There is no mechanism compelling them.

That is the same shape as Chapter 32 §32.4's routing security and Chapter 27 §27.2's BCP 38
— a critical property maintained by convention rather than enforcement — and it is
worth noticing how often this book arrives at that observation.

## What breaks here

A link at 100% utilisation carrying little useful traffic. Possible collapse. Look for
retransmission rates.

**Throughput falling as offered load rises.** The signature of the curve above.

A UDP application starving TCP on a shared link. No congestion response.

TCP performing badly on wireless despite good signal. Loss interpreted as congestion.

High utilisation, high latency, and low throughput simultaneously. Bufferbloat — the
queue is full, the loss signal is delayed, and everything is waiting.

> **Network+ note.** Objective 2.2 touches congestion and objective 5.4 covers performance
> problems. The history is not examined; the concepts are. Over-learn: **TCP infers
> congestion from packet loss**; the congestion window is maintained by the sender and
> appears in no packet; and flow control protects the receiver while congestion control
> protects the network.
