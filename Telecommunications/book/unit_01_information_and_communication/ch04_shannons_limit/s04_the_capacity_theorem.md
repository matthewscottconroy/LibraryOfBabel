# 4.4 The Capacity Theorem

We have Nyquist's bandwidth ceiling and we have a way to talk about noise. Shannon
combined them, and the result is the most important formula in telecommunications.

## The Shannon–Hartley theorem

For a channel of bandwidth *B* hertz with additive white Gaussian noise, at
signal-to-noise ratio SNR (as a linear power ratio, **not** in dB), the capacity is

$$C = B \log_2 (1 + \text{SNR}) \quad \text{bits per second}$$

Below *C*, arbitrarily reliable communication is achievable. Above *C*, it is
impossible. Not difficult, not expensive — impossible, in the way that exceeding
the speed of light is impossible.

Compare this with Nyquist's *C* = 2*B* log₂ *M*. Nyquist's *M* was a free
parameter; you could pick a million levels and claim any rate you liked. Shannon's
formula has no free parameter. The SNR determines how finely the receiver can
distinguish levels, and therefore how large *M* can usefully be. The two formulas
are consistent — setting *M* ≈ √(1+SNR) makes them agree — but Shannon's closes
the loophole.

## Reading the formula

**Capacity is linear in bandwidth.** Double the bandwidth, double the capacity —
provided SNR stays constant. It usually does not, because noise power is *kTB* and
so grows with bandwidth too. Doubling *B* while holding transmit power fixed halves
the SNR. The net effect is still a gain, but less than double: this is why Wi-Fi
going from an 80 MHz to a 160 MHz channel does not double real-world throughput,
and why a 160 MHz channel has noticeably shorter usable range.

**Capacity is logarithmic in SNR.** This is the punchline of Chapter 1's Whitehouse
story, now quantified. At high SNR, log₂(1+SNR) ≈ log₂(SNR), and each doubling of
transmit power — +3 dB — buys you exactly **one additional bit per symbol per
hertz**. That is all. Going from 30 dB to 33 dB SNR takes you from about 10 to
about 11 bits/Hz, a 10% improvement, for double the power.

Power is subject to sharply diminishing returns. Bandwidth is not. **This is why
every modern communications system is built around acquiring more spectrum rather
than more watts** — why Wi-Fi 6E fought for the 6 GHz band, why 5G uses
millimetre wave, why DWDM (Chapter 50) multiplies fibre capacity by adding
wavelengths rather than by amplifying harder.

## Worked examples

**A voice-grade telephone line.** *B* = 3,100 Hz, SNR = 30 dB (a typical good
local loop), which is a linear ratio of 1,000.

$$C = 3{,}100 \times \log_2(1 + 1{,}000) = 3{,}100 \times 9.97 = 30{,}900 \ \text{b/s}$$

About 31 kb/s. And indeed, V.34 modems in 1994 achieved 28.8 kb/s and later
33.6 kb/s on good lines — right against the bound, which is why modem speeds
stopped improving on analog lines despite enormous commercial pressure.

**So how did 56k modems work?** They cheated, ingeniously. A V.90 modem's downstream
path does not traverse an analog channel at all: the ISP connects digitally to the
telephone network, so the only analog segment is the last loop to the subscriber.
The modem does not modulate a carrier; it directly selects among the 256
quantisation levels of the network's own PCM encoding (§4.2's DS0). The theoretical
ceiling becomes 8,000 symbols/s × 8 bits = 64 kb/s, reduced to 56 kb/s in practice
by the µ-law encoding's non-uniform level spacing and by regulatory power limits.
Upstream remained analog and stayed at 33.6 kb/s, which is why V.90 was asymmetric.
It was a beautiful piece of engineering built entirely on knowing exactly where the
Shannon bound was and going around it rather than through it.

**A 20 MHz Wi-Fi channel** at 30 dB SNR (linear 1,000):

$$C = 20 \times 10^6 \times 9.97 = 199 \ \text{Mb/s}$$

802.11ac on 20 MHz with one spatial stream specifies 86.7 Mb/s — about 43% of the
bound, the gap being OFDM guard intervals, forward error correction, preambles, and
the CSMA/CA overhead of Chapter 44. Note that MIMO (multiple spatial streams)
appears to beat the bound; it does not. Each spatial stream is effectively a
separate channel, and the theorem applies per channel.

**The same channel at 10 dB SNR** (linear 10), i.e. a client at the edge of
coverage:

$$C = 20 \times 10^6 \times \log_2(11) = 20 \times 10^6 \times 3.46 = 69 \ \text{Mb/s}$$

A 20 dB drop in SNR costs about two-thirds of the capacity. This is the curve your
laptop rides as you walk down the corridor.

**A fibre span.** *B* = 4 THz (the usable C-band), SNR = 20 dB (linear 100):

$$C = 4 \times 10^{12} \times \log_2(101) = 4 \times 10^{12} \times 6.66 = 2.7 \times 10^{13} = 27 \ \text{Tb/s}$$

Twenty-seven terabits on a single fibre. Commercial DWDM systems reach 25–30 Tb/s
per fibre pair, so here too the industry is operating close to the bound —
Chapter 50 explains how, and Chapter 71 discusses what happens as the margin
disappears.

## The very low SNR regime

At SNR ≪ 1 — deep space, spread spectrum, GPS — the approximation log₂(1+x) ≈
x/ln 2 gives

$$C \approx \frac{B \cdot \text{SNR}}{\ln 2} = 1.44 \, B \cdot \text{SNR}$$

Capacity becomes *linear* in SNR rather than logarithmic. Power now buys you a
proportional return, and bandwidth is nearly free. This is why deep-space missions
use enormous bandwidth spreading and very low rates, and why GPS signals arrive
below the noise floor — around −130 dBm, some 20 dB *under* thermal noise — and are
still decodable. Spreading gain recovers them.

The limiting case gives the **Shannon limit** for energy per bit:

$$\frac{E_b}{N_0} \ge \ln 2 = 0.693 = -1.59 \ \text{dB}$$

Below −1.59 dB of energy per bit to noise spectral density, no communication is
possible at any rate with any code. It is a hard floor on the universe's
willingness to carry information, and modern LDPC and turbo codes operate within
about 0.5 dB of it.

## What the theorem does not say

Three honest caveats, because the formula is often quoted carelessly.

**It assumes additive white Gaussian noise.** Real channels have impulse noise,
fading, and interference with structure. Sometimes real capacity is lower;
occasionally, with clever exploitation of structure, higher.

**It says nothing about latency.** A code approaching capacity may require encoding
over very long blocks, which adds delay. This is a real engineering constraint:
low-latency applications use weaker codes deliberately, and the ultra-reliable
low-latency mode of 5G is a direct fight with this tradeoff.

**It says nothing about complexity.** Shannon's proof is non-constructive. Finding
codes that approach the bound took five decades of work by a large field.

## The four-question test

The payoff for the whole unit. Given any proposed link, ask:

1. **What bandwidth?** (Hertz, from the medium and the standard.)
2. **What SNR?** (From transmit power, path loss, and the noise floor of §4.3.)
3. **What does Shannon permit?** (*B* log₂(1+SNR).)
4. **What rate is required?**

If (4) exceeds (3), the proposal is impossible and no engineering will save it. If
(4) is within about 60% of (3), it is achievable with good modern coding. If (4) is
well under (3), it is comfortable.

That test — four numbers and one logarithm — lets you evaluate any claim about any
link, from a vendor's Wi-Fi datasheet to a proposal to run 10 Gb/s over old copper.
It is the most durable thing in this unit.

## What breaks here

- **Believing marketing rates.** Advertised wireless rates are PHY rates at ideal
  SNR, before protocol overhead. Real throughput is typically 40–60% of the
  advertised figure at good signal, far less at the edge.
- **Adding transmit power to fix coverage.** Logarithmic returns, and in a shared
  medium it raises everyone's interference floor — you degrade the whole cell to
  marginally help one client. Chapter 45 shows the correct answer (more APs at
  lower power).
- **Widening channels to fix throughput.** More bandwidth admits more noise and
  more interference, and in the 2.4 GHz band a 40 MHz channel overlaps nearly
  everything. Frequently a net loss.
- **Assuming a clean link is a fast link.** SNR sets the ceiling; protocol
  overhead, contention, and the bandwidth–delay product of Chapter 3 determine what
  you actually get.
