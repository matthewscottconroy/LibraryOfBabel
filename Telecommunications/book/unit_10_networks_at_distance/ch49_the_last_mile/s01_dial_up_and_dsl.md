# 49.1 Dial-up and DSL

**Two ways of using the same wire**, and the difference between them is one of the clearest
illustrations in this book of what an engineering assumption costs.

## The modem, and Shannon's ceiling

**A dial-up modem places a call.** It dials a number, the telephone network sets up a
circuit exactly as it would for a conversation (Chapter 13 §13.1), **and the modem sends data
as sound through a channel designed for a human voice.**

**Which means the modem is limited to what the telephone network passes: about 3.4 kHz**
(Chapter 12 §12.2), **at a signal-to-noise ratio of roughly 30 dB** on a decent line.

**Apply Shannon** (Chapter 4 §4.2):

$$C = B \log_2(1 + \mathrm{SNR}) = 3400 \times \log_2(1 + 1000) \approx 33{,}900 \text{ b/s}$$

**And V.34, the best purely analogue modem standard, achieved 33,600 b/s.**

> **The modem industry spent fifteen years and enormous engineering effort approaching a
> number that Shannon had computed in 1948**, and having reached it, it stopped. **Not because
> the engineering ran out, but because the channel did.**

**The progression is worth seeing as a table**, because the flattening at the end is the point:

| Standard | Year | Rate | |
|---|---|---|---|
| Bell 103 | 1962 | 300 b/s | |
| V.22bis | 1984 | 2,400 b/s | |
| V.32bis | 1991 | 14,400 b/s | |
| V.34 | 1994 | **28,800 b/s** | |
| V.34+ | 1996 | **33,600 b/s** | **the analogue ceiling** |
| **V.90** | 1998 | **56,000 down / 33,600 up** | **and this required cheating** |

## How 56k evaded the limit

**V.90 is asymmetric because it exploits an asymmetry in the network**, and understanding it
is worth the five minutes.

**By the 1990s the telephone network's core was digital.** A voice call is digitised at the
local exchange into **8,000 samples per second of 8-bit companded PCM — 64 kb/s** (Chapter 12
§12.3) — and carried digitally end to end, with digital-to-analogue conversion only at the
final copper loop.

```
   Home ──analogue copper── Exchange ══digital PSTN══ ISP
        ↑                            ↑
     one D/A conversion         ISP is digitally attached —
     (the only one downstream)   no D/A at all on its side
```

**So downstream, from an ISP connected digitally, there is only one analogue conversion.**
The modem is not decoding a waveform corrupted by two conversions; **it is reading the PCM
codeword directly**, by recognising which of the 256 quantisation levels the exchange sent.

**That is not Shannon-limited by the 3.4 kHz analogue channel. It is limited by the PCM
stream: 8,000 symbols per second.** Using 7 of the 8 bits — the eighth is lost to companding
nonlinearity and robbed-bit signalling — **gives exactly 56 kb/s.**

**Upstream is unchanged at 33.6 kb/s**, because the home modem must still produce an analogue
waveform for the exchange to digitise. **The asymmetry is a direct consequence of where the
converters sit.**

> **V.90 did not beat Shannon. It changed channels** — from the 3.4 kHz analogue voiceband to
> the 64 kb/s digital stream underneath it. **And in the United States, FCC power limits
> capped it at 53.3 kb/s in practice**, which is why the advertised figure was almost never
> observed.

**The lesson generalises**, and it recurs in §49.2 and §49.3: **when a channel's limit is
reached, the remaining move is to find a different channel in the same physical medium.**
DSL is the same move again.

## DSL — the observation

**The telephone company spent a century installing twisted pair to every building. The
question is what that copper can actually carry.**

**The answer is: several megahertz, badly.** The 3.4 kHz limit was never a property of the
wire. **It was a property of the filters, loading coils and channel banks the telephone
company attached to it**, deliberately, to fit many conversations into a shared carrier
system.

```
   Voice band          ADSL upstream       ADSL downstream
   ┌────┐              ┌──────┐         ┌────────────────────┐
   │0–4 │              │25–138│         │  138 kHz – 1.1 MHz │
   │ kHz│              │  kHz │         │                    │
   └────┘              └──────┘         └────────────────────┘
     ▲                                              ▲
   the telephone service, untouched          where the data goes
```

**ADSL puts data in the frequencies above the voice band.** A **splitter** — a passive
low-pass/high-pass filter costing a few pounds — separates them at the customer's premises.
**The telephone continues to work, unmodified, and it works when the power fails**, because it
is still the same circuit.

> **The entire integration cost of ADSL was a passive filter.** That is why it deployed to
> hundreds of millions of homes in under a decade, and it is Chapter 28 §28.1's argument
> again: **the technology that demands nothing of the installed base wins.**

## Why distance dominates

**Attenuation on twisted pair rises with both frequency and distance** (Chapter 6), and the
combination is punishing.

**Roughly, attenuation grows as $\sqrt{f}$** for the skin-effect-dominated régime, so a
1 MHz signal is attenuated far more per kilometre than a 100 kHz one. **DSL's response is to
measure each of its subcarriers and load bits onto them according to the SNR each actually
achieves.**

**This is discrete multitone (DMT)**, and it is OFDM (Chapter 8 §8.4) by another name:

```
   bits per
   subcarrier
     15 ┤██
     12 ┤████
      9 ┤██████
      6 ┤█████████
      3 ┤████████████
      0 ┤███████████████▁▁▁▁▁▁▁▁   ← high frequencies carry nothing
        └──────────────────────────▶  frequency
        low                      high
```

**Each 4.3125 kHz subcarrier gets 0–15 bits depending on its measured SNR**, and the total is
the line rate. **A short line loads bits onto subcarriers up to 1.1 MHz; a long line loads
nothing above 400 kHz.**

**Which produces the table every DSL engineer knows:**

| Loop length | ADSL2+ | VDSL2 (17a) | G.fast |
|---|---|---|---|
| **100 m** | 24 Mb/s | **~150 Mb/s** | **~700 Mb/s – 1 Gb/s** |
| **300 m** | 24 Mb/s | **~100 Mb/s** | **~300 Mb/s** |
| 1 km | **~20 Mb/s** | ~50 Mb/s | unusable |
| 2 km | ~12 Mb/s | ~15 Mb/s | — |
| 3 km | ~6 Mb/s | marginal | — |
| **5 km** | **~1.5 Mb/s** | — | — |
| > 5.5 km | **marginal or nothing** | — | — |

> **Every DSL variant is the same trade: use more spectrum, get more bits, lose distance.**
> ADSL2+ uses 2.2 MHz, VDSL2 uses 17 or 30 MHz, **G.fast uses 106 or 212 MHz — and works over
> 100 metres.**

## Fibre to the cabinet

**The industry's answer to distance was not a better modulation. It was to move the equipment.**

```
   Exchange ═══════════ 4 km copper ═══════════ Home      ADSL: 6 Mb/s
   
   Exchange ═══fibre═══ Cabinet ══300 m copper══ Home      VDSL2: 100 Mb/s
```

**FTTC puts a DSLAM in a street cabinet**, fed by fibre, **so the copper run is a few hundred
metres instead of several kilometres.** The existing drop wire into the house is unchanged.

**This is a genuinely sensible intermediate step**, and it is worth defending against the
common dismissal that it is merely a delaying tactic. **It reuses the most expensive asset —
the drop into the building — while replacing the part that can be replaced**, and it delivered
100 Mb/s to tens of millions of homes for a fraction of the cost of full fibre.

**Its limits are real too.** The cabinet needs power and cooling; **crosstalk between pairs in
the same bundle becomes the binding constraint** rather than attenuation; and the copper
remains a maintenance liability that ages, corrodes and admits water.

**Vectoring** addresses the crosstalk: the DSLAM **measures the interference each line induces
in the others and pre-cancels it**, which is MIMO (Chapter 44 §44.4) applied to a copper
bundle. **It can double VDSL2 rates and requires the operator to control every pair in the
bundle**, which has regulatory consequences in unbundled markets.

## What breaks here

**A line syncing well below its expected rate for the loop length.** **Check the attenuation
and SNR margin**, not the rate — the rate is the symptom. High attenuation for the distance
means a fault or a longer route than the map shows.

**A line that resyncs repeatedly.** **Noise**, and the usual sources are a faulty filter, an
unfiltered extension socket, an intermittent joint, or a nearby electrical appliance. **The
error-second counters distinguish them from a hard fault.**

**Good sync rate, poor throughput.** Not the DSL. **Interleaving depth** adds latency for
error resilience, and the ISP's provisioning, contention or upstream capacity are all
downstream of the modem's numbers.

**Rate dropping after a rain event.** Water in a joint. **The classic copper fault**, and it
recovers partially as things dry, which is what makes it hard to catch.

**A neighbour's new VDSL service coinciding with your rate dropping.** **Crosstalk.** Vectoring
is the fix and it requires the operator to manage the whole bundle.

**Dial-up achieving 44 kb/s rather than 56.** **Normal**, and it was normal throughout the
technology's life — line quality, an analogue hop somewhere in the path, or regulatory power
limits.

> **Network+ note.** Objective 1.5 covers transmission media and 2.1 touches WAN
> technologies. Over-learn: **DSL uses existing telephone copper and its rate falls with
> distance from the exchange**; **ADSL is asymmetric with more downstream than upstream**;
> **a splitter or filter separates voice from data**; and **VDSL delivers higher rates over
> shorter loops.** The distance–rate relationship is the examinable idea.
