# 3.3 Jitter and Loss

Averages conceal. Two links with identical mean latency can behave completely
differently, and the difference is what this section measures.

## Jitter: the variation, not the value

**Jitter** is the variation in latency between successive packets. RFC 3550, which
defines RTP, specifies it precisely as the smoothed mean deviation of the
difference in packet spacing between sender and receiver — but the working
definition is simply: *how much does the delay wobble?*

Why it matters is easiest to see by considering what a real-time application must
do with it.

A voice call sends a packet every 20 ms and must play out a packet every 20 ms. If
the network delivered every packet with exactly 40 ms of delay, playback would be
trivial: play each on arrival, 40 ms behind the speaker, and nobody would notice.
The problem is that packets arrive at 38, 41, 39, 67, 40, 36 ms. The packet at
67 ms arrives *after* its playout slot has passed. It is useless. It might as well
have been lost.

The fix is a **jitter buffer**: hold arriving packets briefly, then play them out
on a strict clock. A 60 ms jitter buffer absorbs variation up to 60 ms — and adds
60 ms to the end-to-end delay, unconditionally, to every packet. This is a
straight trade: **jitter is converted into latency**. The buffer is sized to the
observed jitter, which is why adaptive jitter buffers exist and why a network with
occasional large jitter spikes forces every call on it to run with a permanently
larger delay.

The budget, from ITU-T Recommendation G.114, is the standard reference: one-way
mouth-to-ear delay up to **150 ms** is acceptable for most applications; 150–400 ms
is usable but noticeably degraded; beyond 400 ms is unacceptable for interactive
conversation. Since the codec, packetisation, jitter buffer, and propagation all
draw on the same 150 ms, jitter buffer growth is expensive. Typical enterprise
targets: jitter under 30 ms, ideally under 10 ms.

**What causes jitter.** Variable queueing is the dominant source — which means
jitter is a symptom of congestion even when average latency looks acceptable.
Others: route changes mid-flow, load balancing across paths of unequal length,
half-duplex media contention (every Wi-Fi network), and power-saving states on
endpoints and switches that add a wake-up delay to the first packet after idle.

## Loss: the quantity that averages flatter most

**Packet loss** is the fraction of transmitted packets that never arrive. It is
reported as a percentage, and the percentages that matter are much smaller than
intuition suggests.

**Where packets go.** Three main causes, and they are diagnostically distinct:

- **Buffer overflow.** A queue is full and the arriving packet is discarded. This
  is by far the most common cause, it is a *congestion* signal rather than a
  fault, and it is how the network tells TCP to slow down (Chapter 38). Correlates
  with load.
- **Corruption.** The frame arrives with a failed check sequence and is discarded.
  On copper Ethernet this is rare enough to be a fault indication; on Wi-Fi it is
  routine; on a long fibre span it is measured as a bit error rate. Correlates with
  the physical layer, not with load.
- **Policy.** A firewall or access list dropped it deliberately. Correlates with
  nothing except the rule.

**Why small numbers matter.** TCP interprets loss as congestion and reduces its
sending rate. The Mathis equation gives an approximate ceiling for classic TCP
throughput:

$$\text{throughput} \approx \frac{\text{MSS}}{\text{RTT}} \cdot \frac{C}{\sqrt{p}}$$

with *p* the loss probability and *C* = √(3/2) ≈ 1.22 for the classic AIMD
sawtooth. Take MSS = 1,460 bytes, RTT = 80 ms:

| Loss rate | Approximate max single-stream throughput |
|---|---|
| 0.001% | ~57 Mb/s |
| 0.01% | ~18 Mb/s |
| 0.1% | ~5.7 Mb/s |
| 1% | ~1.8 Mb/s |
| 3% | ~1.0 Mb/s |

**One percent loss caps a single TCP stream at under 2 Mb/s on an 80 ms path,
regardless of whether the link is 100 Mb/s or 100 Gb/s.** This is the single most
useful table in the chapter. It explains why a link that monitoring reports as
"99% healthy" can be delivering a fraction of a per cent of its capacity, and why
the answer to "we have 1% loss but the link is only 30% utilised" is not "so it's
fine."

**Read the top row again**, because it is the one people disbelieve: even at **one
loss in a hundred thousand packets**, a single classic stream on this path is held
to about 57 Mb/s. **Loss rates that sound negligible are not.**

Modern congestion control (CUBIC, and especially BBR — Chapter 38) is
substantially more loss-tolerant than the classic algorithm this equation models.
The numbers above are therefore pessimistic for current stacks. The *shape* —
throughput falling as the inverse square root of loss — still governs.

**Different applications, different tolerances:**

- **File transfer / TCP:** catastrophically sensitive, per the table. Anything
  above 0.1% is worth investigating.
- **Voice:** surprisingly tolerant, because codecs conceal loss by interpolation.
  Under 1% is generally imperceptible; 1–3% is audible; above 5% is unusable.
  Crucially, *bursty* loss is far worse than the same rate spread evenly — losing
  10 consecutive packets is 200 ms of silence, while 10 packets scattered over a
  minute is inaudible.
- **Video streaming:** tolerant, because it buffers seconds of content and can
  re-request.
- **Interactive video:** intolerant, because it cannot buffer and because loss of
  a keyframe corrupts everything until the next one.

## Measuring both

Standard `ping` gives you a start:

```
$ ping -c 100 198.51.100.10
--- 198.51.100.10 ping statistics ---
100 packets transmitted, 98 received, 2% packet loss, time 99148ms
rtt min/avg/max/mdev = 41.183/48.271/312.556/29.114 ms
```

Read all five numbers, not just the average:

- **2% loss** — significant. Above the threshold where TCP suffers badly.
- **min 41.2 ms** — the floor; approximately the propagation plus fixed processing
  component, since the fastest packet queued least.
- **avg 48.3 ms** — 7 ms above the floor, so about 7 ms of typical queueing.
- **max 312.6 ms** — something is badly wrong. A 271 ms excursion above the floor
  is a deep queue, a route flap, or a device that stalled.
- **mdev 29.1 ms** — the mean deviation, a rough jitter proxy. 29 ms of variation
  against a 41 ms floor is a congested path.

The `min` value is the most underused number in that output. It is very nearly a
pure measurement of the path's irreducible delay, because the fastest packet in a
hundred is the one that queued least. **avg − min is your queueing estimate.**
That single subtraction resolves more arguments than any tool in Chapter 64.

For serious measurement, `mtr` gives per-hop loss and jitter continuously, and
`iperf3 -u` measures jitter and loss directly on a UDP stream without TCP's
adaptation confusing the picture.

## A caution about per-hop loss in traceroute

You will regularly see a traceroute showing 40% loss at hop 6 and 0% at hops 7
through 15. Beginners report this as "hop 6 is dropping packets."

It is not. It is hop 6 *rate-limiting its own ICMP responses*, which every router
does to protect its control plane. Traffic passing *through* hop 6 is unaffected —
as proved by the fact that hops beyond it show no loss. **Loss that does not
propagate to subsequent hops is a measurement artefact, not a fault.** Loss at
hop 6 that persists through hops 7–15 is real.

This distinction appears on every certification exam and in every week of
practical work.

## What breaks here

- **Sizing a link from average utilisation** while the peaks are saturating the
  queue. Symptom: users complain, graphs look fine. Fix: look at peaks and at
  `avg − min` latency, not at five-minute averages.
- **Diagnosing loss as a bandwidth problem.** Symptom: transfers slow, link
  utilisation low. Fix: the Mathis table above.
- **Growing the jitter buffer to fix call quality**, thereby pushing one-way delay
  past G.114's 150 ms and making conversation feel like a satellite call. Symptom:
  no dropouts, but people start talking over each other.
- **Believing traceroute's intermediate loss column.** Symptom: an escalation to a
  transit provider about a hop that is working perfectly.
