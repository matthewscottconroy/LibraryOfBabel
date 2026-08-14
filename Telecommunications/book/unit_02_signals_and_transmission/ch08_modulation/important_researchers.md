# Chapter 8 — The People

**Guglielmo Marconi (1874–1937).** Italian inventor and businessman who, more than
anyone, turned Hertz's laboratory demonstration into an industry. His 1901
transatlantic claim is disputed — no recording, an erratic coherer receiver, a
frequency that should not have propagated that far in daylight — and within a few
years the technique unquestionably worked. He shared the 1909 Nobel Prize with
Karl Ferdinand Braun. His transmitters were spark-gap on-off keyed, which is to say
the crudest possible amplitude-shift keying, and they occupied enormous bandwidth
because a spark is a very short pulse (Chapter 5 §5.2's observation about short
pulses and wide spectra, with consequences: early spark transmitters interfered
with everything, and their prohibition was among the first radio regulations).

**Karl Ferdinand Braun (1850–1918).** Shared the Nobel with Marconi, and deserved
it for work Marconi's did not include: the coupled-circuit transmitter that
narrowed the spark's bandwidth dramatically, and the cathode-ray tube, which is
the instrument on which every waveform in this unit was first observed.

**Edwin Armstrong (1890–1954).** American engineer who invented the regenerative
receiver, the superheterodyne receiver — the architecture of essentially every
radio receiver built since 1920 — and wideband frequency modulation. His FM work is
directly relevant here: he demonstrated in 1933 that FM's robustness to amplitude
noise was not merely theoretical, and he did so against an industry consensus,
based on a flawed analysis, that FM offered nothing. He spent his final years in
patent litigation against RCA and died by suicide in 1954; his widow won the
suits afterwards. The superheterodyne architecture — mixing a received signal down
to a fixed intermediate frequency before filtering — is the direct ancestor of the
quadrature down-conversion in §8.2.

**Robert W. Chang.** Bell Labs researcher who in 1966 patented the
principle of transmitting simultaneously on multiple overlapping orthogonal
subcarriers — OFDM, twenty-nine years before it appeared in a consumer product. The
patent (US 3,488,445) describes the technique essentially as used today. It was
impractical because generating and separating the subcarriers required analog
hardware per subcarrier.

**Stephen Weinstein and Paul Ebert.** Their 1971 paper showed that the discrete
Fourier transform could generate and demodulate the whole set of subcarriers at
once, converting Chang's idea from a curiosity into something implementable — and
making it wait another two decades for the DSP hardware to become cheap enough.
The chain from Cooley and Tukey (1965) through Weinstein and Ebert (1971) to
802.11a (1999) is a clean example of an algorithm enabling a technology that
enabled a product, over thirty-four years.

**Gottfried Ungerboeck (b. 1940).** Trellis-coded modulation, which applies to the
QAM constellations of §8.3 exactly as it applies to the PAM of Chapter 7 §7.4 —
constraining which *sequences* of constellation points are legal recovers several
decibels without extra power. See Chapter 7's notes.

**Frank Gray (1887–1969).** Bell Labs physicist. His 1953 patent covered the
reflected binary code now universally called Gray code, invented to prevent
mechanical shaft encoders from producing spurious values as they passed between
positions. Its use in constellation mapping — ensuring a symbol error between
adjacent points produces one bit error — came later and is now universal. He also
did significant early work on television scanning.

**Nikola Tesla (1856–1943) and Alexander Popov (1859–1906).** Both demonstrated
radio transmission before or around Marconi's work, and the question of priority
has been argued for a century with a substantial nationalist component. The US
Supreme Court invalidated key Marconi patents in 1943 partly on the basis of Tesla's
and Lodge's prior work, which is sometimes reported as "Tesla invented radio" and
is more accurately a statement about patent claims than about who built a working
system. Included here because the priority dispute is a good illustration that
"who invented it" is often a less useful question than "who made it work at scale",
and this book generally prefers the second.

**Claude Shannon (1916–2001).** The capacity formula is what makes §8.3's ladder a
consequence rather than a table of empirical figures. The 3 dB per bit relationship
falls directly out of *C* = *B* log₂(1 + SNR), and every constellation in this
chapter is an attempt to sit as close under that bound as the coding of the day
permits.
