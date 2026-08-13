# 5.3 The Bandwidth of a Channel

Chapter 4 used *B* as a number in a formula. This section says what *B* physically
is, where it comes from, and — importantly — why its definition contains an
arbitrary convention that everyone has agreed to stop arguing about.

## The definition, and the convention inside it

Measure a channel's response by sweeping a constant-amplitude sine wave across
frequency and recording the output amplitude at each point. Plot output against
frequency and you have the channel's **frequency response**.

For a typical channel it looks like this: flat across a range, then falling away
at the edges. There is no sharp cliff. Real channels do not stop passing at a
particular frequency; they pass progressively less.

So "the bandwidth" requires a decision about where to draw the line, and the
universal convention is:

> **The bandwidth is the width of the band over which the output power stays
> within 3 dB of its maximum.**

Three decibels is a factor of 2 in power (Chapter 4 §4.3), so this is the
**half-power bandwidth**, and the edges are the **half-power points** or **3 dB
points**.

Why 3 dB? Because a factor of two is a natural place to stop, and because in
amplitude terms it corresponds to 1/√2 ≈ 0.707, which falls out of the mathematics
of simple filters neatly. It is a convention, not a law. Different fields use
different conventions — optical engineers frequently quote 20 dB bandwidth,
regulators define occupied bandwidth by the fraction of total power contained —
and it is worth checking which one a datasheet means before comparing two products.

## Where a channel's bandwidth comes from

Four distinct sources, and identifying which one binds is a diagnostic skill.

**The physics of the medium.** Copper's attenuation rises with frequency because
of the skin effect and dielectric loss (Chapter 6 §6.1), so a long copper pair
simply stops passing high frequencies. Optical fibre's usable band is set by its
loss spectrum — the low-loss windows around 1310 nm and 1550 nm — and by the gain
band of the amplifiers available (Chapter 50 §50.3).

**The equipment at the ends.** A telephone local loop's copper passes megahertz;
the *telephone exchange* deliberately band-limited it to 300–3,400 Hz, because that
is what speech needs and because the limit permitted many conversations to share a
carrier system (Chapter 9 §9.1). DSL's entire premise, developed in Chapter 49
§49.1, is that this limit was in the equipment rather than in the cable.

**Regulation.** A Wi-Fi channel is 20, 40, 80 or 160 MHz because a regulator said
so. Nothing physical stops a wider one; the law does.

**Deliberate filtering.** A receiver's channel filter narrows its bandwidth on
purpose, to reject everything but the wanted signal — accepting less noise
(*N* = *kTB*) in exchange for accepting less of everything else too.

## Bandwidth and bit rate: the connection

The relationship students most need is Chapter 4's, restated here now that we know
what *B* means physically.

Nyquist: a channel of bandwidth *B* supports at most 2*B* symbols per second. Each
symbol carries log₂ *M* bits. So:

$$\text{maximum bit rate} = 2B \log_2 M$$

And Shannon caps *M* via the SNR: *C* = *B* log₂(1 + SNR).

The practical form, which is the one to carry: **bandwidth and bit rate are
different quantities in different units, and a given bandwidth supports a range of
bit rates depending on how many bits you extract per symbol.** A 6 MHz television
channel carries 19.4 Mb/s under ATSC and about 40 Mb/s under DVB-C with a denser
constellation. Same bandwidth; different modulation; different rate.

Anyone who says "bandwidth" and means "bit rate" is using the colloquialism
(Chapter 3 §3.1), which is fine and universal, and it becomes a problem exactly
when both senses appear in one sentence — as they do on every wireless datasheet,
which quotes a channel width in MHz and a data rate in Mb/s and expects you to
know which is which.

## Worked: how much bandwidth does a bit rate need?

A rough but useful rule follows from §5.2's square wave.

A stream of alternating bits `101010…` at *R* bits per second is a square wave at
*R*/2 Hz. Its fundamental is at *R*/2; its harmonics are at 3*R*/2, 5*R*/2 and so
on.

If the channel passes only the fundamental, the receiver sees a sine wave — rounded,
but still crossing the decision threshold at the right moments. That is enough to
decide correctly. So the **absolute minimum bandwidth to carry *R* bits per second
with two levels is *R*/2 Hz**, which is Nyquist's 2*B* limit read backwards.

Real systems want margin, because real data is not a convenient alternating pattern
and because the decision instants must be recoverable. A practical rule is that
binary signalling wants roughly 0.5 to 1 Hz per bit per second, and the excess
bandwidth of the pulse-shaping filter (Chapter 4 §4.2's roll-off factor, typically
0.15–0.35) is the formal expression of that margin.

Two consequences worth stating:

- **10BASE-T at 10 Mb/s using Manchester coding needs about 20 MHz**, because
  Manchester doubles the signalling rate (Chapter 7 §7.2). Category 3 cable is
  specified to 16 MHz, which is why 10BASE-T works on Cat3 and 100BASE-TX does not.
- **1000BASE-T runs 1 Gb/s over Cat5e, specified to 100 MHz.** That is nowhere near
  1 Gb/s ÷ 2 = 500 MHz, and the discrepancy is resolved by using **all four pairs
  simultaneously, in both directions, with five voltage levels**. The arithmetic:
  1 Gb/s ÷ 4 pairs = 250 Mb/s per pair; PAM-5 carries 2 bits per symbol, giving
  125 Msymbols/s; and 125 Msymbols/s needs about 62.5 MHz, which fits inside 100 MHz
  with margin. Every term in that chain is from this unit.

## What breaks here

**Assuming a cable's category rating is a data rate.** Cat5e is rated to 100 MHz,
not to 1 Gb/s. The gigabit comes from the encoding, and 2.5GBASE-T later extracted
2.5 Gb/s from the same 100 MHz by encoding harder (Chapter 7 §7.4). A category
rating is a *bandwidth* specification, and reading it as a rate misses the point
that the rate is a design choice made at the transceiver.

**Comparing bandwidths measured by different conventions.** A "3 dB bandwidth of
100 MHz" and a "20 dB bandwidth of 100 MHz" describe very different channels.

**Forgetting that noise scales with bandwidth.** Widening a receiver's bandwidth
to accept more signal also accepts more noise, at *kTB*. Chapter 4 §4.4's
observation that capacity gains from wider channels are less than proportional
follows directly, and Chapter 43 §43.3 shows it costing real range in Wi-Fi.
