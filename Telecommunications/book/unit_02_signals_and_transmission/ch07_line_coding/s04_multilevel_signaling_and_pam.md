# 7.4 Multilevel Signalling and PAM

Everything so far has spent capacity to buy timing. This section runs the other
way: extracting more bits from each symbol, which buys rate at the cost of noise
margin.

## The idea, and the immediate cost

Two voltage levels carry one bit per symbol. Four levels carry two. Sixteen carry
four. In general, *M* levels carry log₂ *M* bits, which is Chapter 4 §4.2's
arithmetic applied to baseband copper.

**Pulse amplitude modulation** — PAM-*M* — is the name. PAM-2 is NRZ under another
name; PAM-4 is four levels; PAM-8 and PAM-16 exist.

The cost is immediate and computable. Fix the peak-to-peak voltage swing, because
that is what the transmitter's supply and the cable's tolerance permit. Now:

- **PAM-2** has one gap between its two levels: the full swing.
- **PAM-4** has three gaps in the same swing: each is **one third** as large.
- **PAM-8** has seven gaps: each is **one seventh**.

The receiver decides by measuring which level is nearest, and it goes wrong when
noise exceeds half the gap. So the noise tolerance falls in proportion to the gap:

$$\text{SNR penalty (dB)} = 20 \log_{10}(M - 1)$$

| Scheme | Levels | Bits/symbol | Gap (relative) | SNR penalty |
|---|---|---|---|---|
| PAM-2 (NRZ) | 2 | 1 | 1 | 0 dB |
| PAM-4 | 4 | 2 | 1/3 | **9.5 dB** |
| PAM-8 | 8 | 3 | 1/7 | 16.9 dB |
| PAM-16 | 16 | 4 | 1/15 | 23.5 dB |

**Doubling the bits per symbol from 1 to 2 costs 9.5 dB of SNR.** That is a factor
of nearly nine in required signal power for the same error rate.

## So why do it?

Because on many real channels **bandwidth is the binding constraint and SNR is
not**, and where that holds the trade is favourable.

Consider a copper channel. Chapter 6 §6.1 established that attenuation rises
steeply with frequency — Cat5e loses 22 dB per 100 m at 100 MHz and far more above
that. So doubling the symbol rate, which doubles the required bandwidth, costs
enormous additional loss. Whereas the SNR at the frequencies you are already using
may be perfectly adequate to support more levels.

The comparison, for doubling the data rate:

| Approach | What it costs |
|---|---|
| Double the symbol rate | Double the bandwidth, and copper's loss rises steeply — potentially 6–10 dB more attenuation |
| Double the bits per symbol | 9.5 dB more SNR required, at the *same* bandwidth |

These are comparable, and which wins depends on the channel's loss slope. On a
short, clean channel, more levels wins. On a long one where the high frequencies
are already dead, more levels is the only option available at all.

And crucially, **the SNR penalty can be attacked with processing** — equalisation
(Chapter 6 §6.3) and forward error correction — whereas the bandwidth is a property
of the cable that no processing recovers.

That asymmetry is why modern high-speed copper is heavily multilevel.

## Where it is used

**1000BASE-T** uses **PAM-5**: five levels (−2, −1, 0, +1, +2) on each of four
pairs, in both directions simultaneously.

The arithmetic is worth doing because it explains how 1 Gb/s fits in Cat5e's
100 MHz:

- 1 Gb/s ÷ 4 pairs = **250 Mb/s per pair**
- PAM-5 carries 2 bits per symbol usefully (the fifth level is used by the
  trellis-coded modulation for error correction, not for data)
- 250 Mb/s ÷ 2 bits = **125 Msymbols/s per pair**
- 125 Mbaud needs roughly **62.5 MHz** of fundamental
- Cat5e is specified to 100 MHz → **fits, with margin**

Every term in that chain is from this unit. And note the additional trick:
**simultaneous bidirectional transmission on every pair**, which requires each
transceiver to subtract its own transmitted signal from what it receives — echo
cancellation, borrowed from telephony.

**10GBASE-T** uses **PAM-16** on four pairs at 800 Mbaud, with a low-density
parity-check code providing substantial coding gain to offset the 23.5 dB penalty.
This is why 10GBASE-T requires Cat6a and why its transceivers consume noticeably
more power than fibre equivalents: the DSP is doing a great deal of work.

**2.5GBASE-T and 5GBASE-T** (802.3bz, 2016) are 10GBASE-T's signalling scaled down
to fit Cat5e and Cat6 respectively. This is the standard that extracted 2.5 Gb/s
from cable specified in 1999 for 100 Mb/s, and it did so entirely by encoding
harder — the cable did not change.

**400GBASE-DR4 and the modern optical standards** use PAM-4 per lane. So does
DDR5 memory. So does PCI Express 6.0, which moved to PAM-4 after five generations
of NRZ because the channel had run out of bandwidth headroom.

## Trellis coding: getting some of the penalty back

The 9.5 dB figure assumes each symbol is decided independently. It need not be.

**Trellis-coded modulation**, invented by Gottfried Ungerboeck at IBM Zurich in
1982, combines the constellation with a convolutional code so that only certain
*sequences* of symbols are legal. The receiver decodes a sequence rather than
individual symbols, using the Viterbi algorithm, and rejects sequences that the
code forbids.

The effect is to increase the effective distance between valid signal sequences
without increasing the power, recovering typically 3–6 dB of the penalty. It is
why 1000BASE-T uses five levels rather than four: the fifth is redundancy for the
trellis code.

Ungerboeck's result was startling at the time. Coding theory and modulation had
been treated as separate problems — first choose a constellation, then add error
correction on top — and he showed that designing them **together** gained several
decibels for free. It went straight into the V.32 and V.34 modem standards and is
now in essentially every multilevel system.

## Where this leads

§7.4 closes Unit II's baseband story, and the multilevel idea is about to reappear
in a different form.

Chapter 8's QAM is the same idea applied to a modulated carrier: instead of *M*
voltage levels, *M* combinations of amplitude and phase. The arithmetic is
identical — more points, more bits per symbol, less distance between them, more
SNR required — and the constellation diagram of §8.3 is the picture of exactly the
tradeoff tabulated above.

The unifying statement, which is worth carrying out of both chapters:

> **Bits per symbol trades against noise margin, at roughly 3 dB per bit at high
> SNR. Whether the trade is worth making depends on whether your channel is
> bandwidth-limited or SNR-limited, and that is a question you can answer by
> measurement.**

## What breaks here

**2.5GBASE-T failing on cable that carries 1 Gb/s fine.** The higher-order
signalling has less noise margin, so a cable with marginal crosstalk or a
termination fault fails at the higher rate first. This is the modern form of
Chapter 5's exercise E1, and it is common at equipment refresh.

**10GBASE-T over Cat6 in a tight bundle.** Alien crosstalk (Chapter 6 §6.4) eats
the margin that PAM-16 does not have to spare.

**Assuming a link that negotiates a rate will sustain it.** Multilevel schemes
negotiate optimistically and then reveal their marginality as an error rate under
load. Check the counters, not the negotiated speed.

> **Network+ note.** N10-009 expects the cable category requirements for each
> Ethernet standard — Cat5e for 1000BASE-T and 2.5GBASE-T, Cat6a for 10GBASE-T
> at 100 m. The reason those requirements differ is this section: higher-order
> PAM needs more SNR, and cable category is fundamentally a specification of how
> much SNR the cable delivers at a given frequency.
