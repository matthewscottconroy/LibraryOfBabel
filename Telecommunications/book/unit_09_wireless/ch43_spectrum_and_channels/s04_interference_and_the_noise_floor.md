# 43.4 Interference and the Noise Floor

Chapter 42 §42.1 established that **SNR determines capacity, and that raising the noise
floor destroys throughput exactly as attenuating the signal does.**

**This section is about what raises it**, and about the diagnostic that distinguishes the two
kinds of problem — because the remedies are entirely different.

## The noise floor

**The total received power when nothing you care about is transmitting.**

**Its irreducible component is thermal noise** (Chapter 4 §4.3):

$$N = -174 + 10\log_{10}(B) + NF \quad \text{dBm}$$

with **B in Hz** and **NF the receiver's noise figure** in dB.

**For a 20 MHz channel with a typical 5 dB noise figure:**

$$N = -174 + 73 + 5 = \mathbf{-96\ dBm}$$

**That is the floor physics imposes.** In practice a clean environment measures **−95 to
−100 dBm**, and anything above that is something in the room.

| Measured floor | Interpretation |
|---|---|
| **−95 to −100 dBm** | **clean** — thermal noise only |
| −90 dBm | slightly elevated; some ambient activity |
| **−85 dBm** | **noticeably raised** — investigate |
| **−80 dBm** | **serious** — something is transmitting nearby |
| −70 dBm or above | **severe**; the band is unusable for weak clients |

**And every 3 dB of elevation halves the effective range**, because a signal 3 dB weaker
gives the same SNR at a shorter distance.

> **A noise floor 15 dB above thermal costs you 15 dB of link budget** — which by Chapter 42
> §42.3's rule is more than half the distance, and it costs it for every client
> simultaneously.

## The two kinds of interference

**The distinction that determines the remedy:**

| | **Co-channel (CCI)** | **Adjacent-channel (ACI)** |
|---|---|---|
| Source | **another 802.11 device on the same channel** | **a device on a partly overlapping channel, or a non-Wi-Fi emitter** |
| Can it be decoded? | **yes** | **no** |
| Behaviour | **devices take turns** (CSMA/CA) | **transmissions collide** |
| Effect | **shared capacity** — slower | **corruption** — retries and errors |
| Severity | **manageable** | **much worse** |
| Fix | more channels, smaller cells | **eliminate it, or move off the channel** |

**This is §43.2's overlap argument restated as a diagnosis**, and it is the key to reading
wireless problems correctly.

**Co-channel interference is contention.** Everyone is behaving correctly; there are simply
too many of them on one channel. **The symptom is uniform slowness that scales with the
number of active devices.**

**Adjacent-channel and non-Wi-Fi interference is corruption.** Frames are damaged, retries
climb, and **the symptom is errors and retransmissions rather than fair sharing.**

**The measurement that separates them:**

| Observation | Diagnosis |
|---|---|
| **High channel utilisation, low retry rate** | **co-channel contention** — busy, working |
| **High retry rate, moderate utilisation** | **corruption** — interference, or poor SNR |
| **High utilisation *and* high retries** | both |
| **Noise floor elevated with no Wi-Fi visible** | **a non-Wi-Fi emitter** |

**The last row is the one only a spectrum analyser answers**, because a Wi-Fi adapter cannot
see what it cannot demodulate.

## What a Wi-Fi adapter cannot see

**The fundamental limitation of survey tools**, and it is worth stating clearly.

**A Wi-Fi adapter decodes 802.11 frames.** It reports the networks it can hear, their
channels, and their signal strengths.

**It cannot see anything that is not 802.11.** A video sender occupying 20 MHz continuously
appears as **a raised noise floor with no explanation** — the tool reports "no networks on
this channel" and the channel is unusable.

> **"My survey shows the channel is clear and nothing works" is the signature of a non-Wi-Fi
> interferer**, and it requires a spectrum analyser to identify.

**A spectrum analyser measures raw energy across frequency**, regardless of what modulated
it — so it sees the microwave oven, the video sender and the failing fluorescent ballast that
the Wi-Fi adapter cannot.

**The options:**

| Tool | Cost | Character |
|---|---|---|
| **Built-in AP spectrum analysis** | included | many enterprise APs have a dedicated radio; **check whether yours does** |
| **RTL-SDR dongle** | ~£25 | crude, and enough to see gross interference |
| **Wi-Spy / Chanalyzer** | mid | purpose-built for the ISM bands |
| Professional analyser | high | precise, and rarely necessary |

**Enterprise access points frequently have spectrum analysis built in and unused.** It is
worth finding out before buying anything.

## The interferers, by signature

**Each has a characteristic pattern**, and recognising them on a waterfall display is a real
skill:

| Source | Signature | Effect |
|---|---|---|
| **Microwave oven** | **~2.45 GHz, wide, ~50% duty cycle**, on for ~10 ms | **severe while running** |
| **Video sender / analogue camera** | **continuous, wideband, unmoving** | **channel unusable** |
| **Bluetooth** | **narrow, hopping across the whole band** | mild; each hop is brief |
| Cordless phone (older) | narrow, fixed or slow-hopping | moderate |
| Wireless microphone | narrow, continuous while in use | moderate |
| **Zigbee / Thread** | narrow, fixed channel | mild, and it sits between Wi-Fi channels |
| **Failing electrical equipment** | **broadband noise, no structure** | **can be severe, and is often intermittent** |
| **Radar** (5 GHz) | brief, swept, periodic | triggers DFS (§43.1) |
| Motion sensors, some lighting | narrow, periodic | mild |

**Two deserve specific note.**

**The video sender is the worst case.** A cheap analogue camera transmitter occupies a large
part of the band **continuously**, does not back off, and cannot be negotiated with. **It
will make several Wi-Fi channels unusable and nothing in the Wi-Fi domain can address it** —
it must be found and removed.

**Failing electrical equipment is the one people never suspect.** A degrading fluorescent
ballast, a failing power supply, a motor with worn brushes — **each can radiate broadband
noise across the band**, and the interference is often **intermittent in a way that
correlates with nothing in the network.** Chapter 66's methodology covers the general
problem; the specific tell is a raised noise floor with no signal structure at all.

## Reading the measurements

**The three numbers that matter**, and most survey tools report all three:

**Channel utilisation** — what fraction of time the medium is busy.

| Utilisation | Meaning |
|---|---|
| < 30% | comfortable |
| **50%** | **noticeable delay** |
| **70%** | **degraded** — this is the practical ceiling |
| > 80% | **saturated** |

**Wi-Fi degrades well before 100%**, because CSMA/CA's contention overhead rises sharply as
utilisation climbs (Chapter 16 §16.1's ALOHA analysis has the same shape). **70% is the
number to design against.**

**Retry rate** — the fraction of frames retransmitted.

| Retries | Meaning |
|---|---|
| < 5% | healthy |
| **10%** | **investigate** |
| **20%+** | **serious** — corruption or very poor SNR |

**And the crucial interpretation:** **high retries with good RSSI means interference**, not
coverage. **High retries with poor RSSI means coverage.** Two different problems that look
identical from the user's side.

**SNR** — Chapter 42 §42.1's, and **20 dB is the practical minimum.**

## What to do about it

**In order of effectiveness:**

**1. Find and remove the source.** The only complete fix for non-Wi-Fi interference. **A
spectrum analyser and a walk around** — most interferers are physically findable, and the
signal strength rises as you approach.

**2. Change channel.** Free, immediate, and it works if there is a clean channel.

**3. Narrow the channel.** An 80 MHz channel is four times more likely to overlap an
interferer than a 20 MHz one (§43.2), **and narrowing may move you off it entirely.**

**4. Reduce cell size — more access points at lower power.** This raises the *signal* rather
than lowering the noise, which improves SNR by the same arithmetic. **And it is usually the
right answer in a dense environment** for the reasons in Chapter 45 §45.3.

**5. Move band.** 5 GHz or 6 GHz has fewer non-Wi-Fi interferers by a wide margin.

**6. Shield or relocate.** Occasionally practical — moving an access point away from a
lift motor room, or the microwave oven away from the access point.

**What does not work:**

**Raising transmit power.** Chapter 42 §42.2's reciprocity — it improves the downlink and
not the uplink, and it raises the noise floor for your neighbours, who will raise theirs.
**It is the escalation that §43.3's tragedy of the commons describes.**

## A worked diagnosis

*"Wireless is unusable in the east wing every afternoon."*

| Step | Finding | Inference |
|---|---|---|
| 1. RSSI at the location | **−58 dBm** | **coverage is fine** |
| 2. SNR | **12 dB** | **poor — so the noise floor is high** |
| 3. Noise floor | **−70 dBm** | **26 dB above thermal — something is transmitting** |
| 4. Wi-Fi survey | one distant network, weak | **not Wi-Fi** |
| 5. Retry rate | **31%** | corruption, consistent with interference |
| 6. Spectrum analyser | **continuous wideband emission, 2.40–2.44 GHz** | a non-Wi-Fi transmitter |
| 7. Walk toward the strongest reading | a wireless camera in a storeroom | **found it** |

**Step 2 is the pivot.** Good RSSI with poor SNR **can only mean an elevated noise floor**,
and that immediately excludes coverage, access-point placement and power as causes.

**And step 4 is what rules out the Wi-Fi domain entirely** — after which no amount of channel
planning would have helped.

## What breaks here

**Good signal, poor performance.** Elevated noise floor. Measure SNR, not RSSI.

**A survey showing a clear channel that does not work.** A non-Wi-Fi interferer. You need a
spectrum analyser.

**High retries with strong signal.** Interference, not coverage. Do not add access points
until you know which.

**Interference that appears and disappears with no pattern.** Failing electrical equipment
is a candidate, as are anything on a duty cycle and anything a person switches on.

**Utilisation at 75% and users complaining despite "not being full".** 70% is the practical
ceiling for CSMA/CA.

**Performance degrading after raising transmit power everywhere.** Everyone's noise floor
rose, including yours, and the uplinks did not improve.

> **Network+ note.** Objective 5.4 expects interference and its diagnosis, and **this is
> examined.** Over-learn: **co-channel interference is contention and adjacent-channel or
> non-Wi-Fi interference is corruption**; **microwave ovens, Bluetooth, cordless phones and
> video senders interfere at 2.4 GHz**; **a Wi-Fi adapter cannot see non-Wi-Fi
> interference**; and **high retries with good signal means interference rather than
> coverage.**
