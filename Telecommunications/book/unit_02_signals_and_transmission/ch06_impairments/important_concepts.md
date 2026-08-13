# Chapter 6 — Important Concepts

**The four impairments** *(chapter)* — The world can make a signal **smaller**
(attenuation), **add** to it (noise), **reshape** it (distortion and dispersion),
or let **someone else's signal** into it (crosstalk and interference). The
taxonomy is diagnostic: each varies differently with distance, frequency,
temperature and load, and those relationships are how you tell them apart.

**Attenuation** *(§6.1)* — Loss of signal power, measured in dB. Because decibels
add, a link's total loss is the sum of cable, connector and splice losses — the
**loss budget**, which is the calculation behind every media decision.

**Skin effect** *(§6.1)* — At higher frequencies, current is pushed toward a
conductor's surface, shrinking the effective cross-section and raising resistance
as √*f*. Skin depth in copper: 66 µm at 1 MHz, 2.1 µm at 1 GHz.

**Dielectric loss** *(§6.1)* — Energy absorbed by the insulation as the
alternating field flexes its molecules. Rises roughly linearly with frequency and
eventually dominates. With the skin effect, it produces copper's characteristic
rising attenuation curve — about 22 dB per 100 m for Cat5e at 100 MHz.

**Why maximum distances exist** *(§6.1)* — Every "100 m" is the output of a loss
budget, not a physical wall. The figure has been inherited unchanged from 10BASE-T
through 10GBASE-T because each new standard was designed to fit the installed
cable plant rather than to extend it.

**Fibre's loss windows** *(§6.1)* — Rayleigh scattering falls as 1/λ⁴; infrared
absorption rises beyond ~1,600 nm; the OH⁻ water peak sits at 1,383 nm. The minima
produce the O-band (1310 nm, ~0.35 dB/km) and C-band (1550 nm, **~0.17 dB/km**).
At 0.17 dB/km, 4% of the light survives 80 km — which is why amplifier huts are
spaced as they are.

**Loss budget** *(§6.1)* — Launch power minus receiver sensitivity gives the
available budget; subtract fibre, splice and connector losses; what remains is the
**margin**. Above ~7 dB is comfortable; under 3 dB is a link that works in the lab
and fails in service.

**Shot noise** *(§6.2)* — Fluctuation from the discrete arrival of charge carriers
or photons. Unusual in that its power grows with the signal; the dominant limit in
a well-designed optical front end.

**Intermodulation noise** *(§6.2)* — Sums and differences of frequencies generated
by a non-linear device — an amplifier near saturation, a corroded joint acting as a
diode. Third-order products land near the originals and cannot be filtered out. A
second reason, beyond Chapter 4's logarithmic returns, that raising power is often
wrong.

**Impulse noise** *(§6.2)* — Brief, large, irregular energy from switching
transients, motors, lightning, ESD. **The impairment that actually breaks copper
links.** Non-Gaussian and non-stationary, so Shannon's AWGN-based prediction is
not a reliable guide; and bursty, so it destroys whole frames rather than
scattering bit errors. Its diagnostic signature is an error rate that tracks a
*machine's* duty cycle rather than traffic load.

**Noise figure** *(§6.2)* — How much noise a receiver adds above the thermal
floor: SNR_in minus SNR_out, in dB. 0.5 dB for a good satellite LNA, 4–10 dB for
consumer Wi-Fi — which is why a 20 MHz Wi-Fi receiver's practical floor is about
−95 dBm rather than the −101 dBm thermal figure.

**Friis's formula** *(§6.2)* — The **first stage dominates** a chain's noise
figure, because each later stage's contribution is divided by the preceding gain.
Hence the LNA belongs at the antenna, and cable loss placed before it degrades the
whole chain irrecoverably.

**Two levers on SNR** *(§6.2)* — Raise the signal (logarithmic return, plus an
intermodulation penalty at high power) or lower the noise (usually cheaper, usually
ignored). The productive question when wireless performs badly is almost always
what raised the noise floor.

**Amplitude distortion** *(§6.3)* — A frequency response that is not flat. Rounds
a square wave's corners by removing its harmonics.

**Delay (phase) distortion** *(§6.3)* — Different frequency components travelling
at different speeds, so a pulse arrives spread out. The mechanism behind
intersymbol interference.

**Intersymbol interference** *(§6.3)* — A spread pulse extending into the following
symbol's slot, so the receiver's measurement is contaminated by neighbouring
symbols. Not noise — nothing was added — and it corrupts decisions identically.
The same constraint as Nyquist's bandwidth limit, expressed in the time domain.

**Modal dispersion** *(§6.3)* — Multimode fibre only: many propagation paths of
different lengths, so a pulse spreads proportionally to distance. Sets the OM
grades' reach (OM1 33 m to OM4 400 m at 10 Gb/s) and is eliminated entirely by
single-mode fibre's ~9 µm core.

**Chromatic dispersion** *(§6.3)* — Different wavelengths travelling at different
speeds. Near zero at 1310 nm; about 17 ps/(nm·km) at 1550 nm. Remedied historically
by dispersion-shifted fibre and compensating modules, and since ~2008 by electronic
compensation in coherent receivers.

**Polarisation mode dispersion** *(§6.3)* — Residual asymmetry making the two
polarisation states travel at slightly different speeds. Small, statistical,
temperature-dependent, and a limit only at very high rates over very long spans.

**Eye diagram** *(§6.3)* — Every symbol period overlaid on one set of axes. The
**vertical** opening is noise margin; the **horizontal** opening is timing margin;
crossing width is jitter; asymmetry indicates duty-cycle distortion. The direct
visualisation of Chapter 5's threshold behaviour: as impairments accumulate the eye
closes, and when it closes the link fails abruptly.

**Equalisation** *(§6.3)* — Applying the inverse of a channel's known distortion.
Fixed, adaptive, or decision-feedback. It is why Cat5e specified for 100 Mb/s in
1999 carries 2.5 Gb/s under 802.3bz (2016) with no change to the cable.

**Differential signalling and common-mode rejection** *(§6.4)* — The receiver reads
the *difference* between two conductors; external interference couples equally into
both and cancels in the subtraction. Twisting is what makes the coupling equal.
Good balanced pairs achieve 40–60 dB of rejection.

**Split pair** *(§6.4)* — Every pin connected correctly end to end, using one wire
from each of two different twisted pairs as a signal pair. **Passes a continuity
test**, has no common-mode rejection, works at 100 Mb/s and fails at 1 Gb/s. The
canonical demonstration that a continuity tester and a certifier measure different
things.

**NEXT / FEXT / PSNEXT / ELFEXT** *(§6.4)* — Near-end crosstalk (dominant on short
links, since the disturber is unattenuated), far-end crosstalk, the power sum from
all pairs combined (what matters when all four pairs are in use), and
attenuation-normalised FEXT.

**Alien crosstalk** *(§6.4)* — Coupling between different cables in a bundle.
Specified in Cat6a; unspecified in Cat6, which is why 10GBASE-T over Cat6 has a
bundling-dependent distance limit.

**ACR** *(§6.4)* — Attenuation-to-crosstalk ratio. Falls at both ends of the
frequency range, and the frequency at which crosstalk exceeds signal is effectively
what a cable category encodes.

**Differing twist rates** *(§6.4)* — Pairs in one jacket use different pitches so
their geometric relationship rotates along the length and pair-to-pair coupling
averages toward zero, rather than accumulating coherently.

**Shielding notation** *(§6.4)* — U/UTP, F/UTP, S/FTP, F/FTP. Shielding blocks the
field where twisting rejects by symmetry. It helps in hostile environments and can
make things **worse** if improperly earthed — a shield grounded at both ends across
a potential difference carries current and becomes an antenna.

**SINR** *(§6.4)* — Signal to interference *plus* noise. The honest measure in any
shared medium, because interference degrades identically to noise and usually
dominates it. Co-channel interference costs throughput; adjacent-channel
interference costs the link, because partially overlapping transmitters cannot
decode each other and therefore do not defer.
