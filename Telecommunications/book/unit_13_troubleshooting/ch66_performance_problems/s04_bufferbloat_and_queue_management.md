# 66.4 Bufferbloat and Queue Management

The performance problem that was created by an attempt to improve performance, and it is
diagnosed by one measurement that almost nobody makes.

## The measurement

> **Measure latency while the link is loaded.** **An idle ping proves nothing.**

```
   $ ping -c 20 8.8.8.8                       # idle
   rtt min/avg/max/mdev = 8.1/8.4/9.2/0.3 ms

   # start a large upload, then:
   $ ping -c 20 8.8.8.8                       # loaded
   rtt min/avg/max/mdev = 8.2/847.3/1912/402 ms
```

**Eight milliseconds becomes eight hundred.** The link's throughput is unchanged and every
interactive application on it has become unusable — video calls break up, SSH becomes
unbearable, web pages take seconds to start, and the user says "the internet is slow" while a
speed test reports full rate.

That gap between idle and loaded latency is bufferbloat, and it is the single most common
unrecognised performance fault on consumer and small-branch links.

## Why it happens

**Two facts that interact badly.**

**Memory became cheap, and vendors added buffers.** A deeper buffer drops fewer packets, and
dropping packets looks like a defect (Chapter 52's Jacobson material). Every individual
decision to add memory was locally reasonable.

**And loss-based congestion control needs loss.** TCP increases its rate until it loses a
packet (Chapter 38 §38.2). If the buffer is deep, the sender fills it entirely before
receiving any signal at all.

> A queue that is never full is a queue that is doing its job. A queue that is always full is
> a queue that has become pure latency. Loss-based TCP guarantees the second on a deep
> buffer.

**The arithmetic is unforgiving:**

| Buffer | At this rate | **Adds** |
|---|---|---|
| **256 KB** | **1 Mb/s** | **2,048 ms** |
| **1 MB** | **10 Mb/s** | **800 ms** |
| 128 KB | 20 Mb/s | **51 ms** |
| 64 KB | 100 Mb/s | **5 ms** |

Note that the same buffer is harmless at 100 Mb/s and catastrophic at 1 Mb/s — which is
why the problem concentrates on slow uplinks, and why a domestic connection with 40 Mb/s
down and 8 Mb/s up bloats on the upload.

## Where it lives

| Location | Why |
|---|---|
| **Home routers and CPE** | **the classic case — deep default buffers on the slowest link** |
| **The upstream direction** | **asymmetric access** (Chapter 49 §49.2) — the small pipe fills |
| **Branch WAN routers** | **especially where a shaper is not configured** (Chapter 52 §52.3) |
| **The carrier's edge device** | **where you have no visibility** — which is why shaping below the contracted rate matters |
| **Wireless drivers and firmware** | **historically severe, and much improved** |
| **Virtual switches and hypervisors** | occasionally |

> **Chapter 52 §52.3's argument arrives here as a diagnosis:** if you have not shaped below the
> carrier's rate, the queue is in the carrier's device and neither your QoS policy nor your AQM
> can touch it.

## Why it is so often missed

**Four reasons, and each is worth recognising.**

**The throughput is fine.** A speed test reports the full rate — because a speed test
measures throughput and bufferbloat does not reduce throughput.

**The idle latency is fine.** Monitoring that pings a gateway every minute sees 8 ms,
because the link is idle most of the time.

**Utilisation graphs look reasonable.** A five-minute average of 40% (Chapter 54 §54.1)
is entirely consistent with a queue that fills for seconds at a time.

**And the symptom is attributed elsewhere.** "Teams is bad", "the VPN is slow", "the Wi-Fi is
poor" — all correct observations with a cause on a different device.

## The diagnosis

Three steps, and the whole thing takes five minutes.

```
   1.  Measure idle latency to a nearby stable target.
   2.  Saturate the link — in the direction under suspicion.
   3.  Measure again, and record the maximum, not the average.
```

**Tools that do this properly:** the Waveform bufferbloat test, `flent` (specifically its
`rrul` test), and `dslreports`' bufferbloat grade — all of which load the link and measure
latency simultaneously, which is the whole test.

**And the grading is conventional:**

| Loaded latency increase | Assessment |
|---|---|
| **< 30 ms** | **good** |
| 30–100 ms | acceptable |
| **100–300 ms** | **poor — interactive traffic suffers** |
| **> 300 ms** | **severe — video calls fail** |

**Test both directions separately.** Asymmetric access means the upload usually bloats first,
and a download-only test misses it entirely.

## The fix

Active queue management, and it targets delay rather than queue length.

### CoDel

Nichols and Jacobson, 2012 (Chapter 52's reading).

> **CoDel measures the minimum queueing delay over a sliding interval.** If it stays above
> 5 ms for longer than 100 ms, CoDel begins dropping — progressively harder until the delay
> falls. It does not care how many bytes are in the queue; it cares how long they are
> staying.

And its design property is that it has no parameters to tune — which was explicit, because
RED's parameters were why RED shipped everywhere and was enabled almost nowhere (Chapter 52's
Floyd material).

### FQ-CoDel

**CoDel plus per-flow fair queueing.**

> **Each flow gets its own sub-queue**, so a bulk transfer cannot delay a voice packet
> regardless of markings — which means it works without any classification at all, and
> cannot be defeated by a host marking its own traffic (Chapter 52 §52.2's trust boundary
> problem, sidestepped).

It is the default queue discipline on modern Linux, and most people benefit from it without
knowing.

### CAKE

FQ-CoDel plus shaping, overhead accounting and DiffServ awareness — and it is the right
choice at a rate-limited edge.

```
   # On the WAN-facing interface of a Linux router or OpenWrt device:
   tc qdisc replace dev eth0 root cake bandwidth 47500kbit
```

The `bandwidth` parameter is the shaper (Chapter 52 §52.3): set it to about 95% of the
actual rate, so the queue forms in your device rather than in the carrier's.

> One line, and it is the single most effective networking configuration change available to
> most people. A domestic connection going from 800 ms of loaded latency to 15 ms is
> routine, and **almost nobody makes it.**

And CAKE handles the overhead accounting — `docsis`, `pppoe-ptm`, `ethernet` keywords —
which matters because the shaper must account for the encapsulation (Chapter 49) **to shape
accurately.**

### BBR

A different approach, at the sender rather than at the queue (Chapter 38 §38.3).

BBR estimates the path's bandwidth and its minimum RTT and paces to that, rather than
filling a buffer until loss occurs.

> A BBR sender does not bloat the buffer, so it does not suffer its own bufferbloat. It
> does not fix the buffer for anyone else — and its interaction with loss-based flows sharing
> the same queue has been genuinely contentious, with evidence that early BBR was unfair to
> Cubic. **BBRv2 and v3 address this.**

**The practical point:** BBR is a sender-side mitigation and AQM is a network-side fix, and
the network-side one helps every flow including the ones you do not control.

## What AQM does not fix

Honesty, because AQM is sometimes proposed as a general performance remedy.

| | |
|---|---|
| **It does not create bandwidth** | **a link that is genuinely too small stays too small** (Chapter 52 §52.1) |
| **It does not help where the bottleneck is elsewhere** | **the queue you manage must be the one that fills** |
| **It does not reduce propagation delay** | Chapter 3 §3.2 |
| **It does not fix loss caused by errors** | §66.2's physical faults |

**And the second is the commonest disappointment:** CAKE configured on a router whose shaper
is set too high, or not at all, does nothing — because the queue is still forming in the
carrier's device.

## What breaks here

"The internet is slow" and a speed test showing full rate. **Bufferbloat.** Measure latency
under load.

**Video calls failing while throughput is fine.** **The same.**

**Monitoring showing 8 ms and users complaining.** The monitoring pings an idle link.
Chapter 54 §54.1's argument, in its sharpest form.

**AQM configured and no improvement.** The shaper is set too high or absent, so the queue is
still in the carrier's device.

**Only the upload affected.** **Expected** — asymmetric access, and the small pipe fills first.

**Latency fine on average and terrible sometimes.** **Read the maximum.** The average is the
wrong statistic (§66.1).

A deeper buffer purchased to fix packet loss. It converts loss into delay, and for
interactive traffic delay is what you were avoiding.

BBR enabled on the servers and the branch link still bloated. BBR helps its own flows.
The queue is still there for everything else.

> **Network+ note.** Objective 5.4 touches these. Over-learn: **latency and jitter degrade
> interactive applications**; **congestion causes queuing delay**; **QoS and traffic shaping
> manage congestion**; and **buffering introduces delay.** Bufferbloat itself is not
> examinable and it is the fault you are most likely to find and fix in a real network, which
> is a reasonable summary of the gap between this book and the certification it maps to.
