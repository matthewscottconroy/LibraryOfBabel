# 18.3.3 LIGO: Squeezed Light in Practice

## Measuring a Strain of $10^{-21}$

LIGO is a Michelson interferometer with $4$ km arms (folded into Fabry-Pérot cavities for an effective path of hundreds of kilometers). A passing gravitational wave stretches one arm and squeezes the other by a fractional strain $h = \Delta L/L \sim 10^{-21}$ — an arm-length change of $\Delta L \sim 10^{-18}$ m, a thousandth of a proton's radius. The interferometer holds its output on a **dark fringe**, so that a differential arm-length change converts to a phase shift and hence to light at the otherwise dark port. The precision of that phase readout is what LIGO lives or dies by.

At high power the dominant noise is **photon shot noise**: the Poissonian arrival statistics of the coherent laser field (Section 17.3.2) limit phase estimation to

$$\Delta\phi \sim \frac{1}{\sqrt{N}},$$

the **standard quantum limit** for $N$ detected photons. One can beat down $\Delta\phi$ with more laser power (larger $N$), but only up to a point: more power means more **radiation-pressure back-action**, the fluctuating force of the photons buffeting the mirrors, which grows with $N$ and dominates at low frequencies. The trade-off between shot noise and back-action defines the standard quantum limit of the instrument.

## Caves' Insight: Engineer the Vacuum in the Dark Port

In 1981 Carlton Caves asked *where the shot noise comes from* and gave a startling answer (Caves, 1981). It is not the laser's amplitude noise. It is the **vacuum fluctuations entering the interferometer's unused (dark) port** — exactly the mandatory vacuum port of Section 18.2.1. The bright laser enters one port; the empty other port cannot be left empty, and the vacuum that necessarily fills it beats against the signal to set the shot-noise floor.

The consequence is transformative: since the limiting noise is set by *what enters the dark port*, replace the vacuum there with **squeezed vacuum**, oriented so the quiet quadrature aligns with the phase quadrature that carries the gravitational-wave signal. The shot noise then drops below the standard quantum limit, with no increase in laser power. Caves turned the beam splitter's inescapable vacuum port from a nuisance into a control knob — the deepest practical application of the fact that there are no unused ports, only ports carrying vacuum you may choose to shape.

**Worked example.** *From decibels of squeezing to gravitational-wave detection rate.*

Injecting $S$ decibels of squeezing reduces the shot-noise *power* by $10^{S/10}$ and hence the noise *amplitude* — the strain sensitivity — by $10^{S/20}$. Advanced LIGO's O3 run injected up to $S = 3$ dB above $50$ Hz (Tse et al., 2019). At shot-noise-limited frequencies this improves the strain sensitivity by

$$10^{3/20} = 10^{0.15} \approx 1.41 = \sqrt{2},$$

exactly the $\sqrt{2}$ expected from halving the noise power. Because the distance to which a standard "siren" (a binary merger of fixed intrinsic loudness) is detectable scales linearly with amplitude sensitivity, the detection **range** improves by up to the same factor at those frequencies. Integrated across the full detection band — where squeezing helped most at high frequency but was capped at low frequency (see below) — Advanced LIGO realized a binary-neutron-star range increase of about **15%**. Since detectable **volume** scales as range$^3$, the expected event rate rose by

$$(1.15)^3 - 1 \approx 0.52,$$

and the measured detection-rate improvement was about $40\%$ (Hanford) and $50\%$ (Livingston). A few decibels of squeezed vacuum, injected into a dark port, bought roughly half again as many gravitational-wave events — for the cost of an OPO on the output bench.

## Deployment: From GEO600 to Advanced LIGO

The path to routine use was incremental. The German-British detector **GEO600** was the first gravitational-wave observatory to operate with squeezed light, running below the shot-noise limit from around 2010–2011. Squeezing was then demonstrated in the initial LIGO Hanford detector (Aasi et al., 2013), and finally deployed across both Advanced LIGO interferometers for the O3 observing run in 2019 (Tse et al., 2019), where it delivered the range and rate gains above. Squeezed light is now a permanent subsystem of the world's gravitational-wave detectors — the first quantum-optics technique to become standard infrastructure in an operating scientific instrument.

## Radiation Pressure and Frequency-Dependent Squeezing

There is a catch that the O3 cap on squeezing reveals. Squeezing one quadrature *anti*-squeezes the conjugate (Section 18.3.2). Quieting the phase quadrature to suppress shot noise makes the amplitude quadrature noisier, which drives the mirrors *harder* and worsens radiation-pressure back-action at low frequencies. Frequency-independent squeezing therefore helps above $\sim50$ Hz but hurts below it — the reason Advanced LIGO limited its squeezing level in O3.

The cure is **frequency-dependent squeezing**: rotate the squeezed quadrature as a function of frequency, squeezing the phase quadrature where shot noise dominates (high frequency) and the amplitude quadrature where back-action dominates (low frequency). This is accomplished by reflecting the squeezed vacuum off a detuned **filter cavity** before injection, an idea proposed in 2001 and implemented in the A+ upgrade around 2023, yielding broadband sub-standard-quantum-limit operation across the entire detection band.

## The Payoff

The first direct detection of gravitational waves, GW150914 — the merger of two black holes — was announced in 2016 (Abbott et al., 2016) and recognized with the 2017 Nobel Prize in Physics to Rainer Weiss, Barry Barish, and Kip Thorne. Squeezed light was not yet in the Advanced LIGO detectors for that first event, but it is now woven into every observing run, extending the reach of gravitational-wave astronomy by tens of percent and multiplying the catalog of detected mergers.

## Why It Matters

LIGO is the proof of principle that quantum optics is a technology, not only a curiosity. The squeezed states of Section 17.3.3, generated by the OPOs of Section 18.3.2, injected through the vacuum port of Section 18.2.1, and limited by the loss formula that governs all squeezing — the entire chain of this chapter — converge on an instrument that is rewriting observational astronomy. The same resource underwrites continuous-variable quantum computing (Chapter 21): squeezing is the non-classical ingredient that both metrology and CV information processing consume, and LIGO is the existence proof that it can be produced, delivered, and used at the frontier of measurement.
