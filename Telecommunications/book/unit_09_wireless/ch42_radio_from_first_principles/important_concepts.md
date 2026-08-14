# Chapter 42 — Important Concepts

**What propagates** *(§42.1)* — A changing electric field produces a magnetic field and
vice versa, self-sustaining, needing no medium. The two are perpendicular to each other
and to the direction of travel — which is what polarisation means. Maxwell predicted it
in 1865; Hertz demonstrated it in 1887 and saw no use for it.

**c = fλ** *(§42.1)* — Since *c* is fixed, frequency and wavelength are inversely
related, and this one relationship determines most of what a radio system can do. λ =
300/f(MHz) metres.

The wavelengths worth carrying *(§42.1)* — 900 MHz → 33 cm; 2.4 GHz → 12.5 cm;
5 GHz → 6 cm; 6 GHz → 5 cm; 60 GHz → 5 mm.

**The fundamental trade** *(§42.1)* — Low frequency travels further and penetrates better
with little bandwidth; high frequency has enormous bandwidth and does not go far or through
anything. Every wireless technology is a position on that trade.

Absorption rises with frequency *(§42.1)* — Concrete ~12 dB at 2.4 GHz and ~20 dB at
5 GHz; the human body ~3 dB and ~6 dB, which is why a room that works empty degrades
when occupied — people are mostly water. Low-E glazing is a metallic coating that
blocks radio as well as infrared, making a modern building a Faraday cage for outdoor
signal.

Metal reflects essentially totally *(§42.1, §42.4)* — Racking, lifts, ductwork,
foil-backed insulation. Plan around them rather than through them.

HF violates the rule *(§42.1)* — 3–30 MHz reflects off the ionosphere, so a modest
transmitter reaches across an ocean. Above ~30 MHz everything is line of sight, which is
the regime all of Unit IX operates in.

Carrier and modulation are separable *(§42.1)* — The carrier determines propagation;
the modulation determines the data rate. Confusing them causes real errors in reasoning
about coverage versus throughput.

**The RSSI scale** *(§42.1)* — −50 excellent, −60 good, −67 dBm the design target for
voice, −70 fair, −80 poor, −90 unusable. Every 10 dB is a factor of ten in power; every
3 dB a factor of two.

SNR matters more than RSSI *(§42.1)* — SNR = signal − noise floor. −60 dBm with a
−95 dBm floor is excellent; the same −60 dBm with a −70 dBm floor is unusable. 20 dB is
the practical minimum for reliable data. Chapter 4's Shannon limit, applied — raising the
noise floor destroys capacity exactly as reducing the signal does, which is why "strong
signal, poor performance" is coherent.

Antenna size follows wavelength *(§42.2)* — A half-wave dipole is λ/2 — 6.25 cm at
2.4 GHz, 16.7 cm at 900 MHz. Resonance is the reason: a conductor of the wrong length
reflects energy back into the transmitter rather than radiating it.

Gain is directionality, not amplification *(§42.2)* — The antenna radiates the same total
power into a smaller solid angle. A bare bulb versus the same bulb in a spotlight.
**dBi** references an isotropic radiator; dBd references a dipole, and dBi = dBd + 2.15.

Higher-gain omnis flatten the pattern *(§42.2)* — A 9 dBi omni has a null directly
above and below it, so a ceiling-mounted one may cover the far end of the floor and not the
room beneath. The single most useful antenna fact for indoor design, and it surprises
people.

Gain and beamwidth are inversely related *(§42.2)* — A 24 dBi dish has a beam a few
degrees wide, which requires accurate aiming — and is why a long link fails after a storm
moves the dish slightly.

**Polarisation must match** *(§42.2)* — Cross-polarised loses 20–30 dB. An access point
mounted on its side is cross-polarised against every upright phone. Circular polarisation
trades a constant 3 dB for immunity to orientation.

EIRP = Ptx − losses + gain *(§42.2)* — Regulators limit EIRP, not transmitter
power, which is correct because EIRP is what interferes with others. Fitting a
higher-gain antenna can put a legal installation over the limit.

Cable loss is severe at these frequencies *(§42.2)* — 10 m of RG-58 loses 10 dB at
2.4 GHz — 90% of the power. Keep the antenna close to the radio, which is why outdoor
units put the radio in the antenna housing and run Ethernet instead.

**Reciprocity** *(§42.2)* — Gain, pattern and polarisation apply equally in both directions.
So a better antenna helps both directions and more transmit power helps only one —
turning the power up to fix coverage creates an asymmetric link where the client hears
the AP and the AP cannot hear the client. One of the commonest wireless design errors.

**Free-space path loss** *(§42.3)* — FSPL = 20log(d) + 20log(f) + 32.44. Doubling the
distance costs 6 dB. Doubling the frequency costs 6 dB. At 2.4 GHz: 80 dB at 100 m,
100 dB at 1 km, and 20 dB per decade thereafter.

5 GHz starts 6.4 dB behind 2.4 GHz *(§42.3)* — Before anything is in the way, from
frequency alone — and more once absorption is counted. Which is why a dual-band design
sizes cells by the 5 GHz coverage.

Receiver sensitivity depends on the rate *(§42.3)* — −98 dBm at 1 Mb/s, −59 dBm at MCS
9. So a weak link does not fail — it slows down, which is why "it connects but it is
slow" is the characteristic wireless complaint and why rate is a better coverage
measurement than association.

**Fade margin** *(§42.3)* — 10 dB indoors, 20 dB outdoor point-to-point, 25–30 dB
carrier-grade. A link with 3 dB of margin is not a working link; it is a link that
happens to be working — the first rainstorm or summer's foliage will take it down, and the
failure will look intermittent and unexplained.

**The Fresnel zone** *(§42.3)* — A wave occupies **an ellipsoid**, not a line, and
obstructing it attenuates even when the direct path is clear. Keep 60% of the first
zone clear — 3.4 m at 1 km, 10.6 m at 10 km. This is why a link surveyed over bare
winter trees degrades in spring, and why long links need masts far taller than visibility
requires.

**Five wave behaviours** *(§42.4)* — Reflection, refraction, **diffraction** (which is why
there is signal round a corner, and is stronger at lower frequencies), scattering,
absorption.

**Multipath** *(§42.4)* — Copies arriving by different paths. 1 metre of extra path = 3.3
ns, and indoor delay spreads of 50–300 ns are typical. Copies half a wavelength apart
cancel — 6.25 cm at 2.4 GHz — so moving a receiver a few centimetres can change the signal
by 20 dB. Which is why a stationary survey is misleading.

**Frequency-selective fading** *(§42.4)* — The path difference is a fixed distance, so it is
a different phase at different frequencies — some subcarriers are nulled and others are
not. OFDM's response is not to fight multipath but to divide the channel into pieces
small enough that each sees a simple problem, with error correction across them.

Inter-symbol interference and the guard interval *(§42.4)* — A delayed copy of one
symbol overlaps the next. The guard interval is pure overhead, so short GI gives ~11%
more throughput and fails in high-multipath environments. 802.11ax's long intervals
(1.6 µs, 3.2 µs) are for outdoor and large cells.

MIMO reversed the sign *(§42.4)* — If the paths are independent, they can carry
different data simultaneously. The receiver solves a system of equations. Multipath went
from the enemy to the enabling condition — and MIMO therefore works better indoors,
because an outdoor line-of-sight link has essentially one path and the equations are
ill-conditioned. Streams ≤ min(Ntx, Nrx), so a 4×4 AP gives a 2×2 laptop two streams.

**Diversity** *(§42.4)* — Two antennas a few centimetres apart, using whichever is better,
because a null at one is unlikely to coincide with a null at the other. Several dB of
effective gain, and the reason access points had two antennas long before MIMO.

**Five practical consequences** *(§42.4)* — **Survey while moving**; a room changes when
occupied; metal dominates any environment containing it; rate adaptation is doing
something real and forcing rates up makes things worse; and multipath is not a fault —
a deployment with no reflections would perform worse.
