# 4.2 Nyquist and the Symbol Rate

Twenty-four years before Shannon, Harry Nyquist asked a narrower question at the
same institution: *given a channel that passes frequencies up to B hertz, how many
distinct signalling events per second can we push through it before they start
interfering with one another?*

His answer, published in 1924 and refined in 1928, is the first half of the
ceiling. It concerns bandwidth alone and says nothing about noise.

## Symbols, not bits

First, a distinction that trips up nearly everyone.

A **symbol** is one signalling event: one discrete state of the channel held for
one signalling interval. A **bit** is one unit of information. They are not the
same thing, and the ratio between them is a design choice.

- If the transmitter uses two voltage levels, each symbol carries 1 bit.
- With four levels, each symbol carries 2 bits (log₂ 4).
- With 256 levels or constellation points, each symbol carries 8 bits.

The **symbol rate** (or **signalling rate**) is measured in **baud**, after Émile
Baudot. The **data rate** is measured in bits per second. The relationship:

$$\text{data rate (b/s)} = \text{symbol rate (baud)} \times \log_2 M$$

where *M* is the number of distinguishable symbols.

Baud and bits per second are equal only when *M* = 2. This is why "9,600 baud" and
"9,600 bps" were interchangeable for early modems and stopped being so the moment
modems adopted multilevel signalling. A V.34 modem at 33.6 kb/s ran at about 3,429
baud with a large constellation — one of the reasons the phrase "baud rate" is now
almost always used incorrectly, including by people who should know better.

## The Nyquist limit

Nyquist's result:

$$\text{maximum symbol rate} = 2B \ \text{baud}$$

A channel of bandwidth *B* hertz supports at most 2*B* symbols per second without
intersymbol interference.

**Why 2B?** The intuition, without the mathematics: a channel that passes
frequencies only up to *B* cannot produce transitions faster than a sinusoid at
frequency *B*, and one full cycle of such a sinusoid contains two independent
excursions — one up, one down. Two independent choices per cycle, *B* cycles per
second, gives 2*B* independent choices per second. Push faster and successive
symbols overlap in time: the channel has not finished responding to symbol *n*
when symbol *n*+1 arrives, and the receiver cannot separate them. That overlap is
the **intersymbol interference** that destroyed the 1858 Atlantic cable.

Nyquist also showed how to shape pulses so that each one is exactly zero at every
*other* symbol's sampling instant — the **Nyquist criterion** for zero ISI. The
raised-cosine and root-raised-cosine filters used in every modern modem and radio
implement it, and the **excess bandwidth** or **roll-off factor** (typically
0.15–0.35) is the price paid for making the filter physically realisable. This is
why a "20 MHz" Wi-Fi channel actually occupies a little more than 20 MHz, and why
real systems achieve somewhat less than the theoretical 2*B*.

## Combining the two: Nyquist's capacity formula

$$C = 2B \log_2 M \ \text{bits per second}$$

Worked examples.

**A voice-grade telephone channel**, *B* ≈ 3,100 Hz.

- Binary signalling (*M* = 2): *C* = 2 × 3,100 × 1 = **6,200 b/s**
- 16 levels (*M* = 16): *C* = 2 × 3,100 × 4 = **24,800 b/s**
- 1,024 points (*M* = 1,024): *C* = 2 × 3,100 × 10 = **62,000 b/s**

Notice what this formula permits: with no constraint on *M*, capacity is
unbounded. Use a million levels and get 124 kb/s; use a billion and get more.
Nyquist's limit alone does not forbid infinite data rates.

That is obviously wrong, and the reason it is wrong is the missing ingredient.
Distinguishing a million voltage levels requires a receiver that can measure
voltage to one part in a million, and **noise** makes that impossible. Nyquist's
formula is correct and incomplete; §4.4 supplies what is missing.

**A 20 MHz Wi-Fi channel** with 256-QAM (*M* = 256, 8 bits per symbol):

$$C = 2 \times 20 \times 10^6 \times 8 = 320 \ \text{Mb/s}$$

Real 802.11ac on a 20 MHz channel with one spatial stream tops out at 86.7 Mb/s,
which is much lower — because OFDM does not use the full 2*B* (it divides the
channel into subcarriers with guard intervals), and because a substantial
fraction of symbols carry forward error correction rather than data. The formula
gives a ceiling, not a product specification.

## The sampling theorem: the same result, backwards

Nyquist's name attaches to a second, closely related result that runs in the
opposite direction and which you will meet constantly in Chapter 12.

> To reconstruct a signal containing frequencies up to *B* hertz, you must sample
> it at a rate of at least 2*B* samples per second.

Sample more slowly and you get **aliasing**: high frequencies masquerade as low
ones, irreversibly. This is the wagon-wheel effect in film, the moiré pattern in a
photograph of a screen, and the reason every analog-to-digital converter is
preceded by an anti-aliasing low-pass filter.

The direct application: telephone speech is band-limited to about 3.4 kHz, so
2 × 3.4 kHz = 6.8 kHz is the minimum sampling rate. The telephone network chose
**8,000 samples per second**, with 8 bits per sample, giving

$$8{,}000 \times 8 = 64{,}000 \ \text{b/s}$$

which is the **DS0**, the fundamental 64 kb/s unit from which the entire digital
telephone hierarchy — T1, E1, and everything above them — is constructed. That
number is not arbitrary. It is Nyquist plus a decision about quantisation
resolution, made at Bell Labs in the 1960s, and it still shapes the world.
Chapter 12 traces the consequences.

The extra headroom (8 kHz rather than 6.8 kHz) exists because real anti-aliasing
filters cannot cut off instantaneously, and the gap between 3.4 and 4 kHz is the
filter's transition band. Nothing in engineering is ever exactly at the limit.

> **Network+ note.** N10-009 does not ask for Nyquist's formula. It does expect
> you to know that channel width affects throughput — objective 2.3 on wireless
> covers 20/40/80/160 MHz channel widths, and the reason wider channels carry more
> data is precisely 2*B*. It also expects the distinction between "baud" and "bits
> per second" only insofar as it never uses the former; be aware that older
> material and some vendor documentation uses them interchangeably and wrongly.
