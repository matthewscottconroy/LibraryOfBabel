# 66.2 Duplex Mismatch and Errors

The interface counters are the most under-read source of evidence in networking, and they
name their own cause if you know what each one means.

## The counters, and what each indicates

| Counter | Means | Points at |
|---|---|---|
| **CRC / FCS errors** | **the frame's checksum failed** | **physical, or a duplex mismatch** |
| **Runts** | **frames under 64 bytes** | **collisions, or a duplex mismatch** |
| **Giants** | **frames over the MTU** | **jumbo frame mismatch** |
| **Late collisions** | **a collision after 512 bit times** | **duplex mismatch, or a cable over length** |
| **Collisions** (normal) | **half duplex, working** | **only meaningful on a half-duplex link** |
| **Input errors** | the total of several | look at the breakdown |
| **Output discards** | **the queue was full** | **congestion, or a microburst** |
| **Input discards** | **the receive ring or buffer was full** | **the device cannot keep up** |
| **Pause frames** | **flow control asked the peer to stop** | **congestion, downstream** |
| **Carrier transitions** | **the link went down and up** | **physical, or the far end** |

Three of those distinctions do the diagnostic work.

### Errors versus discards

> An error is a frame that arrived damaged. A discard is a frame that arrived intact and was
> thrown away.

**They are entirely different faults:**

| | **Errors** | **Discards** |
|---|---|---|
| Cause | **cable, connector, interference, duplex** | **congestion, buffer exhaustion, policy** |
| Fix | **Layer 1** (Chapter 65 §65.1) | **capacity, QoS, or AQM** (Chapter 52) |
| Increment when the link is | idle or busy | **busy — and specifically bursty** |

And output discards on an interface that averages 30% utilisation is the microburst signature
(Chapter 54 §54.1) — the strongest single indicator that a five-minute graph is hiding
something.

### CRC with late collisions versus CRC alone

> CRC errors with late collisions is a duplex mismatch. CRC errors without them is physical.

Because a late collision cannot occur on a correctly-operating full-duplex link at all —
there is no collision detection on full duplex — so its presence means one end believes it
is half duplex.

### Input discards versus output discards

Output discards mean the egress queue overflowed — a downstream congestion problem.

Input discards mean the device could not process what arrived — CPU, a full receive ring,
or a policer — and it is a different investigation entirely.

## Duplex mismatch, worked properly

Rare now and not extinct, and its mechanism is worth understanding because the symptom is
counter-intuitive.

### How it arises

```
   Switch port:  speed 100, duplex full        ← forced
   Host NIC:     auto-negotiate                ← default

   The forced port does not advertise anything.
   The auto NIC sees no advertisement.
   It falls back to its default for the detected speed: HALF duplex.
   The link comes up.
```

> The link is up. The configuration looks reasonable at both ends. And one side is full duplex
> and the other is half.

### What happens then

The full-duplex side transmits whenever it has data. The half-duplex side is listening,
detects transmission while it is itself transmitting, and calls that a collision — which is
correct behaviour for half duplex and wrong for what is actually happening.

| On the half-duplex side | On the full-duplex side |
|---|---|
| **Late collisions** | **CRC / FCS errors** |
| Collisions | Runts |
| **Backoff and retransmission** | |

And the throughput consequence is severe and specific:

> **Small transfers work perfectly.** A ping succeeds; a web page loads; a login works.
> **Sustained transfer collapses** — **frequently to 1–5% of the link rate** — because every
> collision triggers backoff and every lost frame triggers a TCP retransmission.

**Which is why it is diagnosed late:** everything works, and only bulk transfer is
catastrophic, and users report "the file server is slow" rather than "the network is broken."

### Finding it

```
   $ ethtool eth0
   Speed: 1000Mb/s
   Duplex: Full
   Auto-negotiation: on
   Link partner advertised link modes:  <none>       ← the giveaway
```

**Or from the switch:**

```
   Switch# show interface Gi1/0/14 | include duplex|error|collision
     Full-duplex, 100Mb/s, media type is 10/100/1000BaseTX
     18492 input errors, 18492 CRC, 0 frame, 0 overrun, 0 ignored
     0 output errors, 0 collisions, 0 interface resets
```

CRC errors on a full-duplex switch port with a host at the other end, and checking the
host's own counters for late collisions completes the diagnosis.

### The rule

> **Both sides auto, or both sides forced identically. Never one of each.**

**And the default should be auto everywhere.** Auto-negotiation works, has worked for
twenty-five years, and is required for Gigabit and above — forcing is a legacy practice
that survives in configuration templates and in the habits of engineers who last met a broken
implementation in 1998.

**The exceptions are narrow:** some carrier handoffs specify forced settings contractually,
and some legacy or industrial equipment negotiates badly — and in both cases both ends
must be forced, deliberately, and documented (Chapter 55 §55.1's comment argument).

## Flow control, and why it is usually wrong

802.3x pause frames tell the peer to stop transmitting.

> Which pauses everything on the link, including traffic that was not causing the
> congestion — **head-of-line blocking, at Layer 2.**

**And it propagates.** A congested server pauses the switch; the switch's buffers fill; it
pauses its uplink; and the congestion spreads to traffic with no relationship to the original
flow.

Which is why flow control is disabled by default on most modern equipment, and why finding
it enabled on a general-purpose network is usually a finding.

**The exceptions are real and narrow:** storage networks (FCoE, iSCSI) and RDMA fabrics use
priority flow control (802.1Qbb), which pauses per traffic class rather than per link — and
that is the correct mechanism, in a network designed for it.

## Interface resets and flaps

| Symptom | Cause |
|---|---|
| **Incrementing carrier transitions** | **the link is going up and down** — Layer 1, or the far end rebooting |
| **Interface resets** | **the driver or the device restarted the interface** — frequently a symptom of a wedge |
| **Input queue drops with low utilisation** | **the CPU is not servicing the interface** — control-plane load |
| **Errors that appear only under load** | **marginal cabling, heat, or a failing transceiver** |

**And the diagnostic that separates them:** clear the counters, then watch.

```
   Switch# clear counters GigabitEthernet1/0/14
   ! wait a known interval, then read
```

> **A counter's absolute value is nearly useless** — **it may have accumulated over three
> years.** **The rate is what matters**, and clearing then re-reading over a measured interval
> is how you get one. **Note the clearing in the incident record** (Chapter 63 §63.4), because
> you have just destroyed evidence.

## Reading counters honestly

**Four caveats that prevent misdiagnosis.**

**Vendors count differently.** "Input errors" is a sum on some platforms and a distinct
counter on others, and the documentation is the only authority (Chapter 65's reading).

**Counters accumulate from boot.** 10,000 CRC errors over four years is nothing; 10,000 in an
hour is a fault. Always establish the rate.

**Some errors are normal in small numbers.** A handful of CRC errors at link establishment, a
few discards during a burst. Zero is the ideal and a very small non-zero rate on a busy
interface is not automatically a fault — the question is whether it is rising.

And a SPAN port does not show errors (Chapter 64 §64.3) — the switch discards errored
frames before mirroring, so a capture cannot substitute for the counters here.

## What breaks here

Bulk transfer at 3% of the link rate with everything else working. **Duplex mismatch.**
Late collisions on one side, CRC on the other.

**CRC errors and no late collisions.** **Physical** (Chapter 65 §65.1) — cable, connector,
interference, transceiver.

**Output discards on a link averaging 30%.** **Microbursts.** The average is hiding it.

**Input discards with the link nearly idle.** The device's CPU, not the link.

**Pause frames on a general-purpose network.** **Flow control enabled**, and it is spreading
congestion to unrelated traffic.

Giants counted on one side of a link. Jumbo frame MTU mismatch — one end configured for
9,000 and the other not.

A counter with a large value and no known baseline. **Clear and re-read.** The rate is the
evidence.

A forced-duplex configuration in a template applied estate-wide. **A legacy practice.**
Auto everywhere, with documented exceptions.

> **Network+ note.** Objective 5.2 and 5.4 cover these. Over-learn: duplex mismatch causes
> late collisions on the half-duplex side and CRC errors on the full-duplex side, with severe
> throughput degradation; **CRC errors indicate physical problems**; **runts, giants and
> discards each indicate different causes**; and **auto-negotiation should be used on both
> ends.** The duplex mismatch symptom set is examined in almost every form and is the classic
> exam question of this chapter.
