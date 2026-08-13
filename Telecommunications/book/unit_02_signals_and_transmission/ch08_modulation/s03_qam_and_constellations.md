# 8.3 QAM and Constellations

§8.2 gave us a plane and the ability to place a carrier at any point in it. This
section is about choosing which points to use, and it is where Chapter 4's
bits-per-symbol arithmetic becomes a picture you can read.

## The scheme

**Quadrature amplitude modulation** varies both *I* and *Q* — that is, both
amplitude and phase together — and places its constellation points on a grid.

| Scheme | Points | Bits/symbol | Grid |
|---|---|---|---|
| BPSK | 2 | 1 | — |
| QPSK (= 4-QAM) | 4 | 2 | 2×2 |
| 16-QAM | 16 | 4 | 4×4 |
| 64-QAM | 64 | 6 | 8×8 |
| 256-QAM | 256 | 8 | 16×16 |
| 1024-QAM | 1,024 | 10 | 32×32 |
| 4096-QAM | 4,096 | 12 | 64×64 |

A 16-QAM constellation:

```
              Q
        •  •  │  •  •
              │
        •  •  │  •  •
     ─────────┼─────────  I
        •  •  │  •  •
              │
        •  •  │  •  •
```

Sixteen points, four bits each. 4096-QAM — in Wi-Fi 7 — is a 64×64 grid carrying
twelve bits per symbol, and drawing it legibly is not possible on this page, which
is itself informative about how close together those points are.

## The cost, computed

Fix the peak power the transmitter can produce, because that is what the amplifier
and the regulations permit. Now pack more points into the same plane and the
distance between adjacent points falls.

The relationship, for a square constellation:

$$\text{SNR penalty relative to QPSK} \approx 10\log_{10}\left(\frac{M-1}{3}\right) \ \text{dB}$$

and the practical figures, expressed as the SNR required for a usable error rate
before coding:

| Scheme | Bits/symbol | Approximate SNR required |
|---|---|---|
| BPSK | 1 | 4 dB |
| QPSK | 2 | 7 dB |
| 16-QAM | 4 | 15 dB |
| 64-QAM | 6 | 21 dB |
| 256-QAM | 8 | 27 dB |
| 1024-QAM | 10 | 33 dB |
| 4096-QAM | 12 | **39 dB** |

The pattern: **roughly 6 dB per doubling of constellation size, which is 3 dB per
additional bit per symbol.** That is Chapter 4 §4.4's logarithmic relationship
seen from the practical side, and it is the same 3-dB-per-bit trade that
Chapter 7 §7.4 found for PAM on copper. It is not a coincidence; PAM and QAM are
the same arithmetic in one dimension and two.

**39 dB of SNR** for 4096-QAM is a demanding requirement. In Chapter 4 §4.3's
terms, with a −95 dBm noise floor on a 20 MHz channel, you need a received signal
of about −56 dBm — which in practice means being in the same room as the access
point, with nothing else transmitting.

## Rate adaptation: the ladder in action

This table is not academic. It is the ladder your devices climb up and down
continuously.

A Wi-Fi client measures its channel — from the preamble of every received frame,
and from the acknowledgements of its own transmissions — and selects the highest
modulation whose SNR requirement it currently meets. Walk away from the access
point and the SNR falls; the radio steps down the ladder.

| Distance from AP | Typical SNR | Modulation | Relative rate |
|---|---|---|---|
| Same room | 40 dB | 4096-QAM | 12× |
| Same room, further | 33 dB | 1024-QAM | 10× |
| Next room | 27 dB | 256-QAM | 8× |
| Two rooms | 21 dB | 64-QAM | 6× |
| Down the corridor | 15 dB | 16-QAM | 4× |
| Far end of the floor | 7 dB | QPSK | 2× |
| Edge of coverage | 4 dB | BPSK | 1× |

**Twelve times the rate at the top of the ladder versus the bottom.** That range is
why "full bars" tells you almost nothing (Chapter 45), and why the useful
measurement is SNR rather than signal strength.

The adaptation happens on a timescale of milliseconds, driven by algorithms that
must balance two errors: choosing too high a modulation produces retransmissions,
which are expensive; choosing too low wastes capacity. Every vendor's rate control
algorithm is a slightly different answer to that, and the differences are
measurable — which is part of why two access points with identical specifications
perform differently.

The same ladder appears in cellular (Chapter 46), in DOCSIS (Chapter 49 §49.2), in
DVB satellite, and — as Chapter 50 §50.3 describes — in coherent optical systems,
where the same QAM constellations are impressed on light.

## Gray coding: a small idea worth knowing

When a symbol is misread, it is almost always misread as an *adjacent*
constellation point — noise moves it a little, not across the diagram. So the
assignment of bit patterns to points matters.

**Gray coding** assigns patterns so that **adjacent points differ in exactly one
bit**:

```
       Q
   00  │  01
  ─────┼─────  I
   10  │  11
```

A symbol error between adjacent points therefore produces exactly **one** bit
error rather than two or more. Since the error correction downstream is generally
better at fixing scattered single-bit errors than clustered multi-bit ones, this
reduces the post-correction error rate substantially, for free.

Frank Gray patented the code at Bell Labs in 1953, for a different purpose
entirely — reducing errors in mechanical analog-to-digital converters, where a
shaft encoder passing between two positions could otherwise briefly output a value
from neither. Every QAM constellation in every standard uses it.

## Where the constellation is not a square grid

Two variants worth recognising.

**APSK** — amplitude and phase shift keying — arranges points on concentric rings
rather than a grid. Its advantage is a lower peak-to-average power ratio, which
matters when the transmitter's amplifier is a satellite travelling-wave tube
running near saturation. DVB-S2 uses 16-APSK and 32-APSK for exactly this reason,
and it is a good example of a constellation shaped by the *transmitter's* physics
rather than the channel's.

**Non-uniform constellations**, in DVB-T2 and ATSC 3.0, deliberately space points
unevenly to squeeze out a fraction of a decibel closer to the Shannon bound. The
optimal spacing for a Gaussian channel is not uniform, and the gain is small but
free.

## Peak-to-average power ratio

A consideration that becomes important in §8.4 and is worth introducing here.

A QPSK signal has all its constellation points at the same radius, so its envelope
is constant — the amplifier sees a steady load and can be run efficiently near
saturation.

A 256-QAM signal has points at many radii, so its envelope varies substantially.
The amplifier must be **backed off** — operated well below saturation — so that the
occasional high-amplitude symbol is not compressed. Backing off wastes power and
generates heat.

This is why battery-powered devices favour lower-order modulation beyond what the
SNR alone would dictate, and it is one reason cellular uplinks historically used
constant-envelope schemes even when the downlink did not. It becomes a serious
problem for OFDM, which §8.4 addresses.

## What breaks here

**A client reporting excellent signal and poor throughput.** It has negotiated a
low modulation because the *SNR* is poor — high noise, not weak signal. Chapter 45's
distinction between the two is the diagnosis.

**A link that sustains a high modulation on a bench test and drops to a low one in
service.** The bench had no interference. Interference raises the noise floor,
lowers SNR, and walks the radio down the ladder.

**An amplifier driven too hard**, compressing the outer constellation points. Shows
as an inward-pulled constellation and a raised error vector magnitude, and it is
fixed by reducing transmit power — which is counterintuitive and correct.

**Assuming a datasheet's top rate is achievable.** 4096-QAM needs 39 dB of SNR,
which is a specific and demanding physical condition, and the advertised rate
assumes it along with maximum channel width and maximum spatial streams.

> **Network+ note.** N10-009 does not name QAM orders. It does expect you to
> understand that wireless data rates vary with conditions and that the advertised
> maximum is achieved only under ideal ones (objectives 2.3, 5.5). This table is
> the mechanism, and it is why the rate falls as you walk away.
