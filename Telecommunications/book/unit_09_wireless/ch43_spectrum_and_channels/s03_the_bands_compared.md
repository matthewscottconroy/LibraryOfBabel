# 43.3 The Bands Compared

Three bands, and the choice between them is not a preference — **each is better at something
specific and worse at something else**, and a competent deployment uses all three for
different purposes.

## The comparison

| | **2.4 GHz** | **5 GHz** | **6 GHz** |
|---|---|---|---|
| **Spectrum** | **83.5 MHz** | ~500 MHz (US) | **1,200 MHz (US), 480 (EU)** |
| **Non-overlapping 20 MHz channels** | **3** | ~25 | **59 (US)** |
| **Range** | **best** | moderate | **shortest** |
| **Penetration** | **best** | moderate | **worst** |
| **Free-space loss vs 2.4** | — | **+6.4 dB** | **+8 dB** |
| **Interference** | **severe** | moderate | **minimal, for now** |
| **Non-Wi-Fi interferers** | **many** | few | **essentially none** |
| **Max width** | 20 MHz (practically) | 160 MHz | **320 MHz** |
| **Legacy device burden** | **heavy** | light | **none** |
| **Client support** | universal | universal | **recent devices only** |
| **DFS required** | no | **on many channels** | **no (indoor LPI)** |

## 2.4 GHz — the crowded band

**Its virtue is propagation.** Lower frequency means less free-space loss (Chapter 42 §42.3)
and better penetration through walls and floors — **so a 2.4 GHz cell is substantially larger
than a 5 GHz one from the same access point.**

**Its problem is everything else.**

**Three channels** (§43.2), which for any building with more than a handful of access points
means guaranteed co-channel contention.

**And the band is shared with a remarkable amount of non-Wi-Fi equipment:**

| Interferer | Character |
|---|---|
| **Microwave ovens** | **2.45 GHz, high power, ~50% duty cycle** — devastating while running |
| **Bluetooth** | frequency-hopping across the whole band |
| Zigbee, Thread | 2.4 GHz, and their channels sit between Wi-Fi's |
| Cordless telephones | older DECT variants and analogue units |
| Wireless cameras, baby monitors | often continuous transmission |
| Wireless microphones | continuous |
| Video senders | **continuous, wideband, extremely disruptive** |

**The microwave oven is the classic case** and is worth understanding: it emits at
2.45 GHz — **right on channel 9–11** — at high power, and typically at **50% duty cycle**
because domestic ovens pulse the magnetron rather than running it continuously.

> **A microwave oven produces a distinctive pattern: severe interference for ~10 ms,
> nothing for ~10 ms, repeating.** On a spectrum analyser it is unmistakable, and it
> explains "the wireless breaks every day at lunchtime".

**The legacy burden** is the other cost. **802.11b devices from 2000 still work**, and if
one associates, **the access point must enable protection mechanisms** (Chapter 44 §44.2)
that reduce throughput for every client on that radio.

**What 2.4 GHz is genuinely for, in a modern deployment:**

- **IoT and building systems** — sensors, controllers, and devices with 2.4-only radios
- **Coverage of difficult geometry** — where 5 GHz cannot reach
- **Legacy clients** that cannot use anything else
- **Guest and low-bandwidth access**, where capacity is not the concern

**And a common enterprise practice is worth knowing:** **disable 2.4 GHz on most access
points**, leaving it enabled on a minority for coverage and IoT. **This reduces co-channel
contention dramatically** — with fewer 2.4 GHz radios, the three channels go further — and
it is counter-intuitive enough that it needs explaining to management.

## 5 GHz — the working band

**The band most enterprise Wi-Fi actually runs on**, and for good reasons:

- **~25 non-overlapping 20 MHz channels**, so channel reuse is genuinely possible
- **Almost no non-Wi-Fi interference** — radar is the exception, and DFS handles it
- **Room for 40 and 80 MHz channels** without immediate contention
- **Universal client support** since about 2013

**Its costs:**

**Range and penetration** — Chapter 42 §42.3's 6.4 dB before absorption, and materially
worse absorption. **Cells are smaller**, so more access points are needed.

**DFS complexity** (§43.1) — a radar event moves everyone off a channel for 30 minutes, and
false detections occur.

**The UNII-1/UNII-3 crowding.** Because DFS is inconvenient, many deployments use only the
non-DFS channels — **so those channels are congested and the DFS channels are empty.** **A
deployment willing to use DFS often finds a clean channel where its neighbours have none.**

## 6 GHz — the new band

**The most significant spectrum event in Wi-Fi's history**, and its properties are worth
stating plainly.

**What it gives:**

**Enormous capacity.** 1,200 MHz in the US supports **seven 160 MHz channels** — compared
with two in 5 GHz. **Wide channels become affordable.**

**No legacy.** Only Wi-Fi 6E and 7 devices can use it, **so there are no 802.11b protection
mechanisms, no ancient clients, and no mixed-mode overhead.** The band starts clean.

**Essentially no interference.** Microwave ovens, Bluetooth and Zigbee are not there.
Incumbent fixed microwave links exist and are handled by AFC or by the indoor-only
restriction.

**No DFS for indoor use.** LPI operation requires no radar detection.

**What it costs:**

**Range.** Another ~1.6 dB of free-space loss above 5 GHz, worse penetration, **and — for
LPI — a lower power limit.** **Cells are noticeably smaller**, and a 1:1 replacement of
5 GHz access points with 6 GHz ones will leave coverage holes.

**Client support.** Wi-Fi 6E arrived in 2021 and 7 in 2024; **the installed base is
recent devices only**, so a 6 GHz-only deployment excludes most of an existing estate.

**Regulatory variation.** The US allocation is 2.5× Europe's, and some countries have
allocated none. **A global organisation cannot assume a uniform design.**

## Band steering

**The mechanism for putting dual-band clients on the better band**, and it is more delicate
than it sounds.

**The problem:** a client that can use both bands **often prefers 2.4 GHz**, because the
signal is stronger — **and stronger is not better** when the band is congested.

**The mechanism:** the access point **delays or declines probe responses on 2.4 GHz** for
clients it has seen on 5 GHz, encouraging them to associate on 5.

**Where it goes wrong:**

**A client at the edge of 5 GHz coverage** is steered onto a band it can barely use, and
performs worse than it would have on 2.4 GHz. **Aggressive steering with insufficient 5 GHz
coverage is worse than none.**

**Some clients handle it badly** — repeatedly associating and disassociating, or refusing to
connect at all.

**The guidance:** **steer only where 5 GHz coverage is genuinely good**, and use the
controller's RSSI threshold so clients below it are left alone.

## Designing across the bands

**The modern approach**, and it follows from the comparison:

| Band | Role |
|---|---|
| **2.4 GHz** | **coverage and IoT.** Enabled on a subset of access points. 20 MHz. |
| **5 GHz** | **the workhorse.** Enabled everywhere. 20 or 40 MHz in density, 80 elsewhere. |
| **6 GHz** | **capacity where clients support it.** 80 or 160 MHz. Same or denser AP spacing. |

**And size the cells by the highest band you rely on.** **If 6 GHz is carrying the load,
access-point spacing must suit 6 GHz** — which means more access points than a 5 GHz design
would need.

> **A tri-band deployment designed for 5 GHz coverage will have 6 GHz holes**, and the
> symptom is clients falling back to 5 GHz in places nobody predicted.

## The tragedy of the commons

**Worth stating explicitly, because it explains behaviour you will observe.**

**Unlicensed spectrum has no coordination mechanism** (§43.1). **Each network's rational
choice is to use wide channels and high power**, because that maximises its own performance
in isolation.

**And if everyone does it, everyone is worse off** — wide channels overlap, high power
raises everyone's noise floor, and the aggregate capacity of the building falls.

**The individually rational choice is collectively harmful, and there is no mechanism to
prevent it.**

**What partially mitigates it:**

- **Standards-based politeness** — CSMA/CA (Chapter 44 §44.2) makes Wi-Fi devices defer to
  each other **when they can hear each other**
- **Regulatory power limits** (§43.1)
- **More spectrum** — which is why 6 GHz matters, and is the only genuine fix
- **Professional deployments choosing narrow channels and low power**, which is
  cooperative behaviour with no enforcement behind it

**The parallel with Chapter 32 §32.4's routing and Chapter 27 §27.2's BCP 38 is exact:**
**a shared resource whose stability rests on voluntary restraint, with no authority and no
enforcement.** This book keeps arriving at that structure, and spectrum is the case where
the physics makes it unavoidable.

## What breaks here

**Wireless failing every day at the same time in the same place.** A microwave oven, or
another scheduled interferer.

**2.4 GHz performing badly however it is configured.** Three channels and heavy
interference. It cannot be fixed; reduce reliance on it.

**One 802.11b device degrading a whole radio.** Protection mechanisms. Disable low rates.

**Only four 5 GHz channels available.** DFS disabled.

**Clients on 2.4 GHz despite good 5 GHz coverage.** Band steering not enabled, or client
preference.

**Clients steered onto 5 GHz and performing badly.** Steering is too aggressive for the
coverage.

**6 GHz coverage holes after replacing 5 GHz access points one-for-one.** Smaller cells.

**A channel plan that worked and degraded with no change on your side.** A neighbour. You
have no recourse.

> **Network+ note.** Objective 2.4 expects band characteristics and their trade-offs, and
> **this is examined directly.** Over-learn: **2.4 GHz has better range and penetration, 3
> channels, and more interference**; **5 GHz has more channels, higher speeds and shorter
> range**; **6 GHz has the most spectrum and the shortest range**; and **microwave ovens,
> Bluetooth and cordless phones interfere at 2.4 GHz.** The range-versus-capacity trade is
> asked in several forms.
