# 71.1 Beyond 5G

**Chapter 46 §46.4 assessed 5G honestly: a solid generational improvement sold as a
revolution.** **This section applies the same standard to what is being proposed next**, which
requires separating the physics from the marketing before either can be evaluated.

## What 6G research is actually about

**The ITU's framework and the research programmes converge on six themes**, and their
plausibility differs enormously.

| Theme | Claim | **Assessment** |
|---|---|---|
| **Terahertz spectrum** | **100 GHz – 3 THz** | **physics is unfavourable; niche** |
| **Integrated sensing and communication** | **the network as a radar** | **plausible and genuinely novel** |
| **AI-native air interface** | **learned rather than designed** | **real research, unproven at scale** |
| **Non-terrestrial integration** | **satellites as part of the network** | **already happening** (Chapter 49 §49.4) |
| **Reconfigurable intelligent surfaces** | **programmable reflectors** | **interesting; deployment unclear** |
| **Sub-millisecond, ultra-reliable** | **URLLC, properly this time** | **5G promised it and did not deliver** |

> **The honest starting point is that 5G's URLLC and mMTC promises largely did not arrive**
> (Chapter 46 §46.4), **and 6G's framework repeats several of them.** **A claim that failed once
> deserves more scepticism the second time, not less.**

## Terahertz, and why the physics is unfavourable

**The frequency range above 100 GHz is proposed because it is empty and therefore has
bandwidth.** **The arithmetic of Chapter 42 explains the difficulty.**

**Free-space path loss rises with the square of frequency** (Chapter 42 §42.3):

| Frequency | Wavelength | **FSPL at 10 m** |
|---|---|---|
| **2.4 GHz** | 125 mm | **60 dB** |
| **28 GHz** (5G mmWave) | 10.7 mm | **81 dB** |
| **100 GHz** | **3.0 mm** | **92 dB** |
| **300 GHz** | **1.0 mm** | **102 dB** |
| **1 THz** | **0.3 mm** | **112 dB** |

**And that is only the free-space term.** **Atmospheric absorption adds more:**

| | |
|---|---|
| **Water vapour absorption peaks** | **at 183 GHz, 325 GHz and above** — **tens to hundreds of dB per kilometre** |
| **Oxygen absorption** | **60 GHz, ~15 dB/km** |
| **Rain** | **severe, and worse than at mmWave** (Chapter 49 §49.4) |
| **Foliage, glass, walls, a hand** | **effectively opaque** |

> **At 300 GHz a human hand blocks the link.** **Not attenuates — blocks.** **And the wavelength
> is one millimetre, so a raindrop is a substantial obstacle rather than a scatterer.**

**Which bounds the applications rather than eliminating them:**

| Plausible | Not plausible |
|---|---|
| **Very short range, line of sight** — a metre or two | **replacing mid-band cellular** |
| **Wireless backhaul over hundreds of metres, fixed** | **mobile coverage** |
| **Data centre wireless interconnect** | **anything through a wall** |
| **Imaging and sensing** | |

**And the same argument was made about mmWave** (Chapter 46 §46.4): **the marketing figures came
from a band that covers a few hundred metres and does not pass through a window.** **Terahertz is
that argument, an order of magnitude further along.**

> **The useful prediction: terahertz will be deployed where the geometry is controlled — fixed
> links, data centres, sensing — and mid-band spectrum will continue to carry mobile traffic.**
> **Which is exactly what happened with mmWave**, and the pattern is worth expecting.

## Integrated sensing and communication

**The genuinely novel idea, and the one least discussed.**

> **A radio that transmits and receives is a radar.** **The reflections of a communication
> signal carry information about the environment** — **objects, motion, distance, and with
> sufficient bandwidth, shape.**

**Which is real physics and is already demonstrable:** **Wi-Fi sensing detects presence, breathing
and gesture from channel state information**, and **the resolution improves with bandwidth**,
which is what terahertz has.

**The applications:**

| | |
|---|---|
| **Presence and occupancy** | without cameras |
| **Fall detection, vital signs** | **healthcare, and it works today at Wi-Fi frequencies** |
| **Gesture interfaces** | |
| **Automotive** | **the vehicle's communication radio doubling as a sensor** |
| **Industrial safety** | detecting a person entering a machine's envelope |

**And the honest concern is the same as the application:**

> **A network that can sense is a network that is sensing.** **Presence, movement and — at high
> bandwidth — activity within a building, derivable from the communication infrastructure by
> whoever operates it.** **The privacy framework for this does not exist**, and it is a
> harder problem than the engineering.

## AI-native air interface

**The claim: replace designed components — the modulation, the coding, the channel estimation,
the scheduler — with learned ones.**

**What is real:**

| | |
|---|---|
| **Channel estimation and prediction** | **learned models outperform classical estimators in some regimes** |
| **Beam management** | **predicting which beam to use** (Chapter 46 §46.4) — a good machine learning problem |
| **Scheduling** | **a resource allocation problem with a learned policy** |
| **Constellation shaping** | probabilistic shaping is already deployed and is closer to classical optimisation |

**What is not established:**

**End-to-end learned transceivers** — **an autoencoder replacing the whole physical layer** —
**are a genuine research direction and are unproven at scale**, and **the interoperability
question is fundamental: two vendors' learned transceivers must interwork, which means the
learned representation must be standardised, which removes much of the point.**

> **The deeper difficulty is Chapter 4's.** **Shannon's limit is not a design choice**, and a
> learned system cannot exceed it. **What learning can do is approach it with less computation,
> adapt faster to a changing channel, or handle a régime where the classical model is a poor
> fit** — **which is worthwhile and is not a new era.**

## Non-terrestrial networks

**The part that is already happening** (Chapter 49 §49.4).

**Direct-to-device satellite service** — **an ordinary handset connecting to a LEO satellite,
using terrestrial spectrum** — **is deployed for messaging and is being extended.**

**And its constraints are Chapter 49's:** **the link budget to a handset with a small antenna and
a watt of power is severe**, **the capacity per beam is shared across an enormous area**, and
**the applications are messaging and emergency communication rather than broadband.**

> **Which is genuinely valuable — coverage where none existed — and is not "5G from space."**

## The sub-millisecond claim

**Repeated from 5G, and the reason it did not arrive is worth restating** (Chapter 46 §46.4).

**Air interface latency is not end-to-end latency.** **A 1 ms radio and a 40 ms path to the
application is a 41 ms system**, and **the fix is edge computing** — which is an infrastructure
deployment problem rather than a radio one.

**And the reliability half is harder than the latency half.** **99.999% reliability over a radio
channel means the tail**, and **a radio link's tail is dominated by fading, interference and
mobility** (Chapter 42) — **which requires redundancy in frequency, in space, or in time, and
each costs capacity.**

> **The honest position: deterministic wireless is achievable in a controlled environment — a
> factory with known geometry and managed interference — and is not achievable in general.**
> **§71.4's TSN is the wired version of the same requirement**, and it is instructive that the
> wired version needed a decade of standards work.

## How to evaluate a claim in this area

**Five questions, and they generalise beyond 6G.**

| | |
|---|---|
| **1** | **What does the physics permit?** — Chapters 4 and 42 are the constraints |
| **2** | **What is the link budget?** — a demonstration at three metres is a demonstration at three metres |
| **3** | **What spectrum, and who has it?** — allocation is slower than technology (Chapter 43) |
| **4** | **What is the deployment cost per covered area?** — this is what killed mmWave coverage |
| **5** | **Which promise from the last generation does this repeat?** |

**The fifth is the most useful.** **URLLC, mMTC and network slicing were all promised for 5G**
(Chapter 46 §46.4), **and their absence is why 5G is described as a solid improvement rather than
a revolution.** **A 6G pitch that repeats them without explaining what changed is repeating a
failed prediction.**

## What breaks here

**A terahertz demonstration that works in a laboratory and not in a corridor.** **The physics.**
Path loss, absorption and blockage.

**A "6G" claim that is mid-band spectrum with a better air interface.** **Which is a real
improvement** and is what actually arrives, and it is worth saying so.

**A latency claim quoting the air interface.** **Not end to end** (Chapter 46 §46.4).

**Sensing deployed without a privacy assessment.** **A network that can sense is sensing**, and
the framework does not exist.

**A learned physical layer that cannot interoperate.** **The representation must be
standardised**, which removes much of the advantage.

**Direct-to-device satellite marketed as broadband.** **The link budget and the shared beam.**
Messaging and emergency use.

> **Network+ note.** 6G is beyond the syllabus. The transferable content is Chapter 42's and
> Chapter 43's: **higher frequency means more bandwidth and less range**, **spectrum is
> allocated and scarce**, and **coverage cost per area is what determines deployment.** Those
> three predict most of what will happen.
