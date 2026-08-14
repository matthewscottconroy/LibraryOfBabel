# Chapter 7 — Important Concepts

**NRZ (non-return-to-zero)** *(§7.1)* — Hold a voltage high for `1` and low for
`0`, one bit period each. The obvious scheme, still used at 10 Gb/s and above, and
insufficient on its own for two reasons.

**Clock recovery** *(§7.1)* — Extracting the transmitter's timing from transitions
in the received data, since no separate clock wire exists. A phase-locked loop uses
each edge as a correction. **A long run of identical bits contains no transitions
and therefore no timing information.**

**Clock drift arithmetic** *(§7.1)* — Two crystals differing by 50 ppm accumulate
half a bit period of error after 10,000 bits — less than one maximum-size Ethernet
frame. This is why clock recovery is mandatory rather than optional.

**Baseline wander** *(§7.1)* — A sustained level decaying toward zero through an
AC-coupled path. Ethernet interfaces are transformer-coupled for electrical
isolation (a safety requirement, not a refinement), and a transformer transfers
energy only when the field changes. Hence **DC balance** is required.

**What a line code must provide** *(§7.1)* — Bounded run length, DC balance, and —
ideally — low overhead, error detection, and control symbols distinguishable from
any data.

**Guarantee by construction versus statistically** *(§7.1, §7.3)* — A block code
like 4B/5B makes long runs *impossible*; a scrambler makes them *improbable*. The
industry moved from the first to the second as speeds rose and 25% overhead became
unaffordable.

**Scrambling** *(§7.1)* — XORing data with a pseudorandom sequence both ends can
reproduce, so the transmitted stream looks random and therefore has good transition
density and DC balance on average.

**Manchester encoding** *(§7.2)* — A transition in the middle of every bit: `1` is
low-to-high, `0` is high-to-low (in the 802.3 convention). Guarantees a transition
every bit period and perfect DC balance in every individual bit. **Costs 100%
overhead — 50% efficiency** — because the signalling rate is twice the data rate.
Used by 10BASE-T, which is why it runs on Cat3.

**Differential Manchester** *(§7.2)* — Encodes in the presence or absence of a
boundary transition rather than the direction of the mid-bit one. **Polarity
independent**, so a reversed pair does not invert the data. Used by Token Ring and
by industrial protocols where field wiring is done by non-specialists.

**Bipolar AMI, B8ZS, HDB3** *(§7.2)* — Alternate mark inversion gives DC balance
by alternating the polarity of `1` pulses, and fails on long runs of `0`. B8ZS
(North America) and HDB3 (Europe) substitute a deliberate, recognisable code
violation for a zero run. Present in T1 and E1 circuits.

**MLT-3** *(§7.2)* — Three levels cycled on each `1`, held on each `0`, reducing
the fundamental frequency by a factor of four. Combined with 4B/5B in 100BASE-TX,
which is how 100 Mb/s fits within Cat5's 100 MHz.

**4B/5B** *(§7.3)* — Four data bits onto five transmitted bits, choosing the 16
code words with the best transition properties. **25% overhead, 80% efficiency.**
No more than three consecutive zeros in any concatenation. The spare patterns
become **control symbols** (idle, start, end) that cannot be confused with data,
which solves frame delimiting at the physical layer.

**8B/10B** *(§7.3)* — Widmer and Franaszek, IBM, 1983. Eight bits onto ten, same
25% overhead, better properties. In 1000BASE-X, Fibre Channel, PCIe 1.0/2.0, SATA,
DisplayPort, InfiniBand.

**Running disparity** *(§7.3)* — 8B/10B's mechanism: each input maps to two
possible outputs of opposite disparity, and the encoder picks whichever corrects
the accumulated imbalance. **DC balance guaranteed by construction**, maximum run
length 5.

**Comma character (K28.5)** *(§7.3)* — A control symbol whose bit pattern cannot
occur anywhere in any concatenation of data code words, including across
boundaries. Lets a receiver that started mid-stream establish **symbol alignment**
by waiting for one.

**64B/66B** *(§7.3)* — 64 data bits plus a 2-bit sync header (`01` or `10`,
never `00` or `11`), with the payload scrambled by *x*⁵⁸ + *x*³⁹ + 1. **3.125%
overhead, 96.9% efficiency.** The header is unscrambled and provides block
alignment; the payload's properties are statistical.

**256B/257B** *(§7.3)* — Compresses four 64B/66B blocks' sync headers. **0.4%
overhead, 99.6% efficiency.** Used at 200G and 400G, typically alongside
Reed-Solomon forward error correction, which is a separate mechanism serving a
different purpose.

**The efficiency ladder** *(§7.2, §7.3)* — 50% in 1983 to 99.6% in 2017. The
engineering did not provide the same guarantee more cheaply; it renegotiated which
guarantee it was providing, because receivers became good enough that the residual
risk was smaller than risks already accepted.

**PAM-*M*** *(§7.4)* — Pulse amplitude modulation with *M* levels, carrying
log₂ *M* bits per symbol. PAM-2 is NRZ.

**The multilevel SNR penalty** *(§7.4)* — Fixing the peak-to-peak swing, *M* levels
means *M*−1 gaps each 1/(*M*−1) as large, so the penalty is 20 log₁₀(*M*−1) dB:
**9.5 dB for PAM-4**, 16.9 dB for PAM-8, 23.5 dB for PAM-16.

**Why the trade is worth making** *(§7.4)* — On copper, bandwidth is usually the
binding constraint and SNR is not, because attenuation rises steeply with
frequency. Doubling the symbol rate costs bandwidth the cable cannot supply;
doubling the bits per symbol costs SNR that equalisation and coding can partly
recover.

**1000BASE-T's arithmetic** *(§7.4)* — 1 Gb/s ÷ 4 pairs = 250 Mb/s per pair; PAM-5
gives 2 useful bits/symbol → 125 Mbaud → about 62.5 MHz, within Cat5e's 100 MHz.
Plus simultaneous bidirectional transmission on every pair, requiring echo
cancellation.

**802.3bz (2.5GBASE-T / 5GBASE-T)** *(§7.4)* — 10GBASE-T's signalling scaled to fit
Cat5e and Cat6. Extracted 2.5 Gb/s from cable specified in 1999 to 100 MHz,
entirely by encoding harder.

**Trellis-coded modulation** *(§7.4)* — Ungerboeck, IBM Zurich, 1982. Combines the
constellation with a convolutional code so that only certain *sequences* are legal,
increasing the effective distance between valid signal sequences and recovering
3–6 dB. Designing coding and modulation **together** rather than separately, which
was the surprise.

**The unifying trade** *(§7.4)* — Bits per symbol trades against noise margin at
roughly 3 dB per bit at high SNR. Whether it is worth making depends on whether the
channel is bandwidth-limited or SNR-limited — a question answerable by measurement.
