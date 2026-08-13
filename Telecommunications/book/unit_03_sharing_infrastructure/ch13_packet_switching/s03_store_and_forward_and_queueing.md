# 13.3 Store-and-Forward and Queueing

The mechanism by which a packet actually crosses a network, and the place where all
the delay of Chapter 3 §3.2 accumulates.

## Store-and-forward

A packet switch **receives an entire packet**, verifies it, and only then forwards
it.

```
   t=0    ┌──────────┐
          │  packet  │ arriving at node A
          └──────────┘
   t=T    packet fully received at A, checksum verified
   t=T    ┌──────────┐
          │  packet  │ begins transmission toward B
          └──────────┘
   t=2T   packet fully received at B
```

The consequence for delay is direct and worth computing. If a packet of *L* bits
crosses *n* hops of rate *R*, the transmission component of the delay is

$$t_{\text{trans}} = n \times \frac{L}{R}$$

because each hop must clock the whole packet in before clocking any of it out. A
1,500-byte packet crossing 14 hops of 10 Gb/s links accumulates 14 × 1.2 µs = 16.8 µs
of store-and-forward delay — which Chapter 3 §3.2's worked example showed to be
negligible against 34 ms of propagation, and which is *not* negligible on slow links.

On a 2 Mb/s link, the same packet costs 6 ms per hop.

**Why do it at all?**

- **Error checking.** The frame check sequence covers the whole frame, so it cannot
  be verified until the whole frame has arrived. Forwarding before checking means
  forwarding corrupted frames, wasting downstream capacity on data that will be
  discarded.
- **Rate adaptation.** A packet arriving on a 10 Gb/s link and leaving on a 1 Gb/s
  link must be buffered; there is no alternative.
- **Contention.** If the outgoing link is busy, the packet must wait somewhere.

The alternative, **cut-through switching**, begins forwarding as soon as the
destination address has been read — typically after 6 bytes for Ethernet, giving
sub-microsecond latency. It cannot verify the FCS, so it forwards corrupted frames,
and it cannot adapt rates. Chapter 17 §17.4 covers the trade; the summary is that
cut-through is used where latency matters more than error containment, which is
principally high-frequency trading and some HPC fabrics.

**Fragment-free** is the compromise: forward after 64 bytes, on the grounds that
collisions on a legacy shared segment happen within the first 64 bytes
(Chapter 16 §16.2's minimum frame derivation), so most corruption is caught.

## The queue

The moment a packet may have to wait, a **queue** exists, and with it everything
that makes network performance a statistical rather than a deterministic subject.

```
        ┌───────────────────────────┐
   →→→  │ ▓ ▓ ▓ ▓ ▓ ░ ░ ░ ░ ░ ░ ░ │ →→→  outgoing link
        └───────────────────────────┘
   arrivals      buffer                  service at rate R
```

Three things can happen to an arriving packet:

- **The link is idle** — transmit immediately. Queueing delay zero.
- **The link is busy and the buffer has room** — wait. Queueing delay is the time to
  drain what is ahead.
- **The buffer is full** — **discard**.

That third case is the important one, and it is worth being precise about what it
means: **packet loss on a healthy network is not a fault. It is the mechanism by
which the network signals congestion**, and it is what TCP's congestion control
(Chapter 38) is built to interpret. A network that never dropped a packet would have
no way to tell senders to slow down.

## Why queueing delay explodes

Chapter 3 §3.2 gave the result; here is where it comes from.

For a simple M/M/1 queue — Poisson arrivals, exponential service times, one
server — the mean number waiting is

$$L_q = \frac{\rho^2}{1-\rho}$$

and the mean waiting time is

$$W_q = \frac{\rho}{\mu(1-\rho)}$$

where ρ is utilisation and 1/µ is mean service time.

The (1−ρ) in the denominator is everything. As ρ approaches 1, the denominator
approaches zero and the delay approaches infinity:

| ρ | Relative queueing delay | Multiple of the 50% figure |
|---|---|---|
| 0.5 | 1.0 | 1× |
| 0.7 | 2.33 | 2.3× |
| 0.8 | 4.0 | 4× |
| 0.9 | **9.0** | **9×** |
| 0.95 | 19.0 | 19× |
| 0.99 | 99.0 | 99× |

**Raising utilisation from 50% to 90% multiplies queueing delay by nine.** That is
not a gradual degradation; it is a cliff whose approach is invisible on a utilisation
graph, because 90% looks like sensible use of a resource you paid for.

This single relationship is why:

- **Capacity planning targets 60–70% peak**, not 95%. The apparently wasted headroom
  is buying latency.
- **"The circuit is only 60% utilised so it isn't the problem" must be checked
  against the peak.** A link averaging 60% over five minutes may be at 100% for
  three seconds every minute, and those three seconds are where the video call
  lives (Chapter 54 §54.1's averaging warning).
- **Adding load to a nearly-full link is catastrophic** rather than incremental.

And the real Internet is worse than this model predicts. Traffic is **bursty and
self-similar** rather than Poisson — Leland, Taqqu, Willinger and Wilson
demonstrated this in 1993 from Ethernet traces, showing that traffic looks equally
bursty at every timescale, which Poisson models do not capture. Self-similar traffic
produces **longer queues at a given mean utilisation** than the M/M/1 model suggests.
The shape of the curve is right; the pessimism should be greater.

## Where the four delays live

Chapter 3 §3.2's decomposition, located mechanically:

| Component | Where it happens | Varies with |
|---|---|---|
| **Propagation** | on the wire between nodes | distance, medium |
| **Transmission** | clocking bits onto the link | packet size, link rate |
| **Processing** | the forwarding decision | device, table size, features |
| **Queueing** | waiting in the buffer | **offered load** |

Only queueing depends on other people. It is therefore the only one that varies
minute to minute, the only one that a capacity decision affects, and the one that
`avg − min` in a ping estimates.

## Buffer sizing, and why intuition fails

Given that a full buffer means loss, more buffer should mean less loss and better
performance.

It does not, and the reason is Chapter 66 §66.4's bufferbloat.

TCP uses loss as its congestion signal. A large buffer does not prevent congestion;
it **hides** it. Packets that would have been dropped are queued instead — for
hundreds of milliseconds, sometimes seconds — and TCP, receiving no loss signal,
keeps increasing its window. The queue grows until it finally overflows, by which
time **every packet crossing that link is delayed by the full depth of the queue**,
including packets from latency-sensitive flows that had nothing to do with filling
it.

The observable result is familiar: a large upload makes an unrelated video call
unusable, on a connection with plenty of capacity, and the effect vanishes the
moment the upload finishes.

**More buffer is not better buffer.** A buffer's job is to absorb bursts, not to
store a backlog. The classical rule of thumb — buffer equal to the bandwidth-delay
product — was derived for a small number of long flows and substantially
over-provisions for realistic traffic mixes.

The modern answer is **active queue management** — CoDel, FQ-CoDel, PIE, CAKE —
which drops or marks based on **how long a packet has been queued** rather than on
how full the buffer is, keeping latency bounded regardless of buffer size. FQ-CoDel
is now the default on Linux and in most consumer router firmware, and it is one of
the more satisfying instances of a research result reaching deployment.

## What breaks here

**A link at 90% utilisation with users complaining and graphs looking acceptable.**
The ρ/(1−ρ) curve. Size to 60–70% peak.

**Output drops on an interface whose average utilisation is low.** Microbursts. The
average conceals them; the drop counter does not.

**A large upload destroying interactive performance.** Bufferbloat. Measure latency
*under load*, not on an idle link, and enable AQM.

**Store-and-forward delay on slow links.** A 1,500-byte packet costs 6 ms per hop at
2 Mb/s, and a 20 ms voice packet queued behind one arrives 6 ms late. This is why
low-bandwidth links use fragmentation and interleaving, and it is the origin of the
QoS mechanisms in Chapter 52.

**Assuming packet loss indicates a fault.** On a congested link it indicates
congestion, which is the network working as designed. Corruption-induced loss
(Chapter 6) is the fault case, and the two are distinguished by whether the loss
correlates with load or with the physical layer.

> **Network+ note.** Objective 5.4 covers performance issues including latency and
> jitter; objective 3.1 covers monitoring and baselines. The ρ/(1−ρ) relationship is
> not examined by name and is the mechanism behind every capacity-planning
> recommendation the exam does expect.
