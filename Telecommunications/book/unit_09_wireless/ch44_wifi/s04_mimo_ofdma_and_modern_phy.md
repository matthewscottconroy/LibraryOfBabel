# 44.4 MIMO, OFDMA and the Modern PHY

Four mechanisms that between them account for Wi-Fi's throughput rising three orders of
magnitude while its spectrum allocation rose by one.

They are frequently confused with each other, and the distinctions are worth being
precise about.

## The four, distinguished

| Mechanism | Divides | Serves |
|---|---|---|
| **SU-MIMO** | **space** | **one client**, several streams |
| **MU-MIMO** | **space** | **several clients simultaneously** |
| **OFDMA** | **frequency** | **several clients simultaneously** |
| **Beamforming** | — | **one client, better** |

> MIMO is spatial multiplexing. OFDMA is frequency multiplexing. Beamforming is not
> multiplexing at all — it improves one link rather than creating parallel ones.

## SU-MIMO — spatial streams

**Chapter 42 §42.4's mechanism.** Several antennas at both ends, several independent paths,
several simultaneous data streams on the same frequency.

**The notation:**

```
   4 × 4 : 3
   ↑   ↑   ↑
   Tx  Rx  spatial streams
```

A 4×4:3 radio has four transmit and four receive chains and supports three streams —
more antennas than streams is common, because the extra chains improve diversity and
beamforming.

And the streams are limited by the weaker end:

$$\text{streams} = \min(N_{\text{tx streams}},\ N_{\text{rx streams}})$$

| Client | Typical streams |
|---|---|
| **Phone** | **1–2** |
| **Laptop** | **2**, occasionally 3 |
| Tablet | 1–2 |
| **IoT sensor** | **1** |
| High-end desktop adapter | 3–4 |

> **An 8×8 access point talking to a 2-stream laptop delivers 2 streams.** The extra chains
> help through beamforming and diversity, and **do not multiply that client's throughput.**

Which is why access-point stream counts beyond about four give diminishing returns for
throughput — and why the marketing figures of §44.1 assume client hardware that does not
exist.

## MU-MIMO — several clients at once

**The step from 802.11ac.**

SU-MIMO gives one client several streams. MU-MIMO gives several clients one or more streams
each, simultaneously, on the same channel.

```
   SU-MIMO:
      AP ══════ 4 streams ══════▶ one client

   MU-MIMO:
      AP ══ 2 streams ══▶ client A  ┐
         ══ 1 stream  ══▶ client B  ├ all at the same time
         ══ 1 stream  ══▶ client C  ┘
```

**How it works:** the access point knows the channel to each client — the channel state
information — and constructs transmissions that arrive correctly at one client and cancel
at the others. Spatial separation used as a multiplexing dimension.

Its requirements are demanding, and they are why the practical benefit is smaller than
advertised:

**Clients must be spatially separated.** Two clients in the same place have nearly the same
channel, so the transmissions cannot be separated and MU-MIMO gives nothing.

**Channel state information must be current.** It is obtained by **sounding** — the access
point sends a sounding frame and clients report what they received. The information goes
stale as anything moves, so sounding must repeat, and the sounding itself costs
airtime.

**Client support is required and was slow.** 802.11ac had **downlink MU-MIMO only**; 802.11ax
added **uplink**, which is harder because the clients must be synchronised.

**The honest assessment:** MU-MIMO helps in specific conditions — several spatially
separated clients with sustained traffic — and does nothing in many real situations. It is
not the general throughput multiplier the marketing suggested, and Wi-Fi 6's OFDMA addresses
the common case better.

## OFDMA — the Wi-Fi 6 change

The most important addition in 802.11ax, and it is a change of philosophy rather than a
faster radio.

### The problem

Before OFDMA, a transmission uses the whole channel.

```
   Time →
   ┌──────────────────────────────────────┐
   │        client A: 64 bytes            │  ← the WHOLE 80 MHz channel
   └──────────────────────────────────────┘
   ┌──────────────────────────────────────┐
   │        client B: 64 bytes            │  ← for a tiny frame
   └──────────────────────────────────────┘
```

And most Wi-Fi frames are small — acknowledgements, sensor readings, keystrokes, control
traffic. Giving an 80 MHz channel to a 64-byte frame wastes almost all of it, and the
per-frame overhead of §44.2 is paid in full each time.

### The mechanism

Divide the channel into Resource Units and give several clients one each, simultaneously.

```
   Frequency ↑
             ┌────────┬────────┬────────┬────────┐
             │ client │ client │ client │ client │
             │   A    │   B    │   C    │   D    │   ← ONE transmission
             └────────┴────────┴────────┴────────┘
                          Time →
```

**Resource unit sizes** are defined in subcarriers — 26, 52, 106, 242, 484, 996 — so a
20 MHz channel can serve up to nine clients simultaneously with the smallest units.

**What it buys:**

| | Without OFDMA | With OFDMA |
|---|---|---|
| Small frames | one per transmission | **several per transmission** |
| Per-frame overhead | paid **each** time | **paid once for all of them** |
| Latency in dense cells | rises with client count | **much flatter** |
| Efficiency with mixed traffic | poor | **substantially better** |

> OFDMA does not make any single client faster. It makes many clients less expensive to
> serve, which is exactly the problem in a dense deployment.

**And it is scheduled rather than contended.** The access point **allocates** resource units
via a trigger frame, so uplink transmissions from several clients are coordinated rather
than competing — which removes contention overhead for those transmissions entirely.

**This is a genuine architectural change:** 802.11 moves, for the first time, from purely
distributed contention toward **central scheduling**, in the direction cellular has always
worked (Chapter 46 §46.3).

### OFDMA versus MU-MIMO

Both serve several clients at once, and they are complementary:

| | OFDMA | MU-MIMO |
|---|---|---|
| Divides | **frequency** | **space** |
| Best for | **many small frames** | **few large transfers** |
| Requires | scheduling | **spatial separation + CSI** |
| Client density | **helps most when high** | needs separation |
| Overhead | trigger frames | **sounding** |

They can be combined — MU-MIMO within an OFDMA resource unit — and Wi-Fi 6 does.

## Beamforming

**Not multiplexing. Improving one link.**

Several antennas transmit the same signal with controlled phase offsets, so the copies
add constructively at the intended receiver and less so elsewhere.

```
   Without:                    With beamforming:
      ↗ ↑ ↖                          ↗
     ↗  ↑  ↖                       ↗
   ← ● → energy everywhere    ● ══▶ energy toward the client
     ↘  ↓  ↙                       ↘
      ↘ ↓ ↙                          ↘
```

Typically 3–5 dB of gain toward the client — which by Chapter 42 §42.3's arithmetic is
worth a substantial fraction of the range, or a higher MCS at the same range.

**Two kinds:**

**Implicit** — the access point infers the channel from frames it receives from the client.
Requires nothing of the client and is less accurate.

**Explicit** — the access point sounds the channel and the client **reports back** what it
received. More accurate, and requires client support. This is what 802.11ac and later
standardise, and it is the same channel information MU-MIMO needs.

Beamforming is why a modern access point can have eight antennas and three streams: the
extra chains are not carrying more data, they are shaping the beam.

## The full modulation picture

Bringing together Chapter 8's modulation, Chapter 42's SNR and §44.1's MCS:

| Modulation | Bits/symbol | Constellation points | SNR needed | Realistic range |
|---|---|---|---|---|
| BPSK | 1 | 2 | ~5 dB | **far** |
| QPSK | 2 | 4 | ~11 dB | far |
| 16-QAM | 4 | 16 | ~18 dB | medium |
| **64-QAM** | **6** | 64 | **~25 dB** | **medium** |
| **256-QAM** | **8** | 256 | **~31 dB** | **near** |
| **1024-QAM** | **10** | 1024 | **~35 dB** | **very near** |
| **4096-QAM** | **12** | **4096** | **~40 dB** | **metres** |

Each step up doubles the constellation points and adds about 6 dB to the SNR
requirement, because the points are packed twice as densely and the noise margin halves.

> **4096-QAM requires 40 dB SNR.** With a −95 dBm noise floor that means a signal of
> **−55 dBm** — a client a few metres from the access point in a clean environment.

Which is the honest answer to "why don't I get Wi-Fi 7 speeds": the highest modulations
are usable in a small fraction of the coverage area, and the average client is running
somewhere in the 64-QAM to 256-QAM range.

## Putting the throughput together

**The full chain, and each factor multiplies:**

$$\text{rate} = \underbrace{\text{channel width}}_{\times 1..16} \times \underbrace{\text{modulation}}_{\times 1..12} \times \underbrace{\text{coding}}_{\times 0.5..0.83} \times \underbrace{\text{streams}}_{\times 1..8} \times \underbrace{\text{efficiency}}_{\approx 0.5}$$

**A realistic laptop connection:**

```
   80 MHz channel        980 data subcarriers  (not 160 — availability, Ch 43 §43.2)
   256-QAM               8 bits/subcarrier     (not 1024 — SNR, above)
   5/6 coding
   2 spatial streams     (laptop hardware)
   13.6 µs symbol        (12.8 µs + 0.8 µs guard interval)
   ────────────────────────────────────────────────────────
        980 × 8 × 5/6 × 2
        ─────────────────  ≈  960 Mb/s nominal
            13.6 µs
   × ~50% protocol efficiency (§44.2)
   ────────────────────────────────────────────────────────
   ≈ 480 Mb/s actual, alone on the channel
   ÷ number of active clients sharing it
```

And the last line is the one people forget. Wi-Fi is a shared medium (§44.2) — thirty
active clients on that access point get roughly **16 Mb/s each**, before contention overhead.

> A datasheet claiming 9.6 Gb/s, a real client achieving 480 Mb/s alone, and 16 Mb/s in a
> busy room, are all the same access point. Every step of that reduction is legitimate and
> predictable, and being able to walk through it is what separates a capacity estimate from a
> guess.

## What breaks here

**Wi-Fi 6 producing no improvement.** The clients are older, or the problem was interference
rather than efficiency.

**MU-MIMO not helping.** Clients are not spatially separated, or the traffic is not sustained
enough for sounding to pay for itself.

An 8-stream access point not delivering 8 streams. No client has 8. Expected.

**Never reaching the top MCS.** SNR. It requires a client very close to the access point.

Throughput halving when a second client becomes active. Shared medium. Expected, and it
is what capacity planning is for (Chapter 45 §45.3).

**Beamforming not enabled.** Some clients do not support explicit sounding; check whether
implicit is available.

> **Network+ note.** Objective 2.4 expects MIMO and MU-MIMO. Over-learn: **MIMO uses several
> antennas for several spatial streams**; **MU-MIMO serves several clients simultaneously**;
> OFDMA divides the channel in frequency so several clients share one transmission; and
> beamforming directs energy toward a client rather than creating parallel streams. The
> MIMO/MU-MIMO/OFDMA distinction is examined and commonly confused.
