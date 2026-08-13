# 52.3 Policing, Shaping and Buffers

**Two mechanisms enforce a rate. They sound similar, behave completely differently, and
confusing them produces real outages.**

## The token bucket

**Both are built from the same abstraction**, so it is worth having once.

```
   tokens added at rate CIR
            │
            ▼
      ┌───────────┐
      │  ▓▓▓▓▓▓   │  ← bucket, depth Bc
      │  ▓▓▓▓▓▓   │
      └─────┬─────┘
            │  a packet of N bytes needs N tokens
            ▼
      conforming ──▶ forward
      exceeding  ──▶ POLICER: drop or re-mark
                 ──▶ SHAPER:  queue until tokens exist
```

| Parameter | Meaning |
|---|---|
| **CIR** | **committed information rate** — the long-run rate |
| **Bc** | **committed burst** — the bucket's depth, in bytes |
| **Tc** | **the interval** — $B_c = \mathrm{CIR} \times T_c$ |
| Be | excess burst — a second bucket, for tolerating larger bursts |

**The only difference between a policer and a shaper is what happens to a non-conforming
packet.** **Everything else is identical.**

## Policing

**Enforce the rate by dropping or re-marking excess. Immediate, memoryless, cheap.**

```
   Offered:  ████████████████████████████
   Policed:  ████░░████░░████░░████░░████     ░ = dropped
```

**Its interaction with TCP is unkind, and the effect is larger than people expect.**

**TCP increases its rate until it loses a packet, then halves** (Chapter 38 §38.2). **A policer
drops packets from a stream precisely when it is accelerating**, and does so in bursts —
because the bucket empties and then every packet in the burst is dropped, not just the excess.

**The result:**

> **A TCP flow through a policer set to 10 Mb/s typically averages well below 10 Mb/s** — often
> 60–80% of it — **because the sawtooth's mean is below its peak and the burst drops cause
> repeated timeouts rather than fast recoveries.**

**Which produces the classic complaint: "we bought 10 Mb/s and we get 7."** **The circuit is
fine. The policer is doing exactly what it was configured to do**, and the customer is
measuring the average of a sawtooth.

**Where policing is right:**

- **On traffic you receive**, where you have no ability to shape the sender
- **On a priority queue**, bounding its damage (§52.2)
- **Where the excess genuinely should be discarded** — a rate-limited guest network
- **Where you need the enforcement to be exact and immediate**

## Shaping

**Enforce the rate by buffering excess and releasing it smoothly.**

```
   Offered:  ████████████████░░░░░░░░░░░░
   Shaped:   ████████████████████████████     smoothed, delayed
```

**Kinder to TCP**, because a delayed packet is not a lost packet: **the sender's RTT increases
slightly, its window growth slows, and it settles at the shaped rate without a sawtooth.**

**Its costs are memory and delay** — the buffer is real, and packets sit in it.

**Where shaping is right:** **on traffic you send.**

> **Shape what you send; police what you receive.**

## The single highest-value configuration in branch networking

**Shape outbound traffic to slightly below the carrier's contracted rate.** Usually **95%.**

**The reason is subtle and worth being precise about.**

```
   Without shaping:

   ┌────────┐  1 Gb/s   ┌─────────────┐  50 Mb/s   ┌─────────┐
   │ Your   │──────────▶│  Carrier's  │───────────▶│ Carrier │
   │ router │           │  edge device│            │ network │
   └────────┘           └─────────────┘            └─────────┘
                              ▲
                    THE QUEUE FORMS HERE.
                    You cannot see it.
                    You cannot control what it drops.
                    Your QoS policy is upstream of it and irrelevant.

   With shaping to 47.5 Mb/s:

   ┌────────┐ shaped   ┌─────────────┐  50 Mb/s
   │ Your   │─────────▶│  Carrier's  │──────────▶ never congested
   │ router │ 47.5     │  edge device│
   └───┬────┘          └─────────────┘
       ▲
   THE QUEUE FORMS HERE NOW.
   Your QoS policy applies to it.
```

> **Shaping below the carrier's rate moves the queue from their equipment into yours**, where
> your classification, your priority queue and your drop policy decide what suffers. **Without
> it, your QoS policy is configured on a device that never queues anything.**

**The 95% figure has a reason:** **the carrier polices at the contracted rate**, and **Layer 2
overhead means your Layer 3 shaper's idea of 50 Mb/s is not theirs.** Ethernet framing, VLAN
tags, and any carrier encapsulation add bytes your shaper is not counting. **Shaping at 95%
leaves room for that discrepancy.**

**Get the overhead accounting right if you can.** Most platforms let you tell the shaper to
account for Layer 2 overhead; **when they do, you can shape closer to 98%.** When they do not,
**95% is a defensible guess and 90% is a wasteful one.**

**And set Tc appropriately.** The default interval is often 125 ms:

| CIR | Tc = 125 ms | **Tc = 10 ms** |
|---|---|---|
| 10 Mb/s | Bc = 156 kB | **Bc = 12.5 kB** |
| 50 Mb/s | Bc = 781 kB | **Bc = 62.5 kB** |

**A 125 ms interval means the shaper releases a large burst and then goes silent for most of
the interval**, which adds up to 125 ms of jitter to anything in that queue. **For voice, that
is fatal.** **Set Tc to 10 ms on any circuit carrying real-time traffic**, and accept the small
increase in scheduling overhead.

## Buffers: bigger is not better

**The intuition is that a deeper buffer absorbs more bursts and therefore drops less. It is
correct and it is not the whole story.**

> **A large buffer converts loss into delay, and for interactive traffic delay is what you
> were trying to avoid.**

**The arithmetic is unforgiving:**

| Buffer | At this rate | **Drains in** |
|---|---|---|
| 256 KB | **1 Mb/s** | **2,048 ms** |
| 1 MB | 10 Mb/s | **800 ms** |
| 1 MB | 100 Mb/s | 80 ms |
| 64 KB | 10 Mb/s | **51 ms** |

**A 256 KB buffer on a 1 Mb/s uplink adds two full seconds of latency when it fills.** **And
TCP will fill it**, because filling the bottleneck buffer is precisely what loss-based
congestion control does.

**This is bufferbloat**, and Chapter 66 §66.4 treats it as a diagnosis. **The point here is
the design rule:**

### Sizing rules, and their limits

**The classical rule of thumb: buffer = bandwidth × delay.**

$$B = \mathrm{RTT} \times C$$

**A 10 Mb/s link with a 100 ms RTT wants 125 kB** — enough to keep the link busy while a single
TCP flow recovers from a loss.

**The refinement, from Appenzeller, Keslassy and McKeown (2004):** **with $n$ independent
flows, the rule becomes $B = \mathrm{RTT} \times C / \sqrt{n}$**, because the flows' sawtooths
are not synchronised and their sum is much smoother than any one of them.

**With 100 flows, that 125 kB becomes 12.5 kB** — **a tenfold reduction**, and it was a genuinely
surprising result that changed core router design.

> **Neither rule is what you should actually do at an edge link.** Both size the buffer to
> maximise throughput. **At a branch or home edge, what you want is to minimise delay**, and
> the modern answer is not to pick a size at all.

### The modern answer

**Active queue management targets delay directly.**

**CoDel** measures the **minimum queueing delay** experienced over a sliding interval. **If it
stays above 5 ms for longer than 100 ms, CoDel begins dropping** — progressively harder until
the delay falls. **It does not care how many bytes are in the queue; it cares how long they are
staying.**

**FQ-CoDel adds per-flow fairness**: each flow gets its own sub-queue, **so a bulk transfer
cannot delay a voice packet regardless of markings.**

**CAKE adds shaping and DiffServ awareness**, which makes it the right choice for an edge
device on a rate-limited circuit.

> **On a home or small-branch link, `cake bandwidth 47.5Mbit` on the egress interface will
> outperform most hand-built QoS policies**, requires no classification, and cannot be
> defeated by a host marking its own traffic. **It is the single most effective networking
> configuration change available to most people**, and almost nobody makes it.

## What breaks here

**"We bought 10 Mb/s and get 7."** **A policer plus TCP.** Ask the carrier to shape rather than
police, or shape on your side to just under the policed rate.

**Throughput fine, latency terrible under load.** **Bufferbloat.** Measure latency *while*
loading the link — an idle ping proves nothing. **`ping` during an upload is the whole test.**

**Voice jitter with a correct-looking QoS policy.** **Tc too large.** Check the shaper's
interval; 125 ms defaults are common and wrong for voice.

**Shaping configured and the queue still forming at the carrier.** **The shaper is set too
close to the contracted rate**, or is not accounting for Layer 2 overhead. **Drop to 92% and
see whether the symptom moves.**

**A policer dropping traffic well below its configured rate.** **Bc too small.** A burst larger
than the bucket is dropped entirely even though the average is conforming. **Increase Bc, or
shape instead.**

**Adding buffer memory and making things worse.** **Expected.** More buffer is more delay.

**FQ-CoDel or CAKE configured and doing nothing.** **The bottleneck is elsewhere** — usually the
carrier's device, because the shaper is not set or is set too high. **AQM only manages the queue
it owns.**

> **Network+ note.** Objective 3.2 and 5.x. Over-learn: **traffic shaping buffers excess
> traffic and releases it smoothly**; **policing drops or re-marks traffic above a rate**;
> **shaping introduces delay, policing introduces loss**; and **bandwidth is not the same as
> throughput.** The shaping-versus-policing distinction is examined and is genuinely useful.
