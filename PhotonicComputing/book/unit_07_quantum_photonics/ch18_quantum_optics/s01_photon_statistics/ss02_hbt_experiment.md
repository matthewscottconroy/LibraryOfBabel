# 18.1.2 The Hanbury Brown-Twiss Experiment

## Intensity Interferometry, from Radio to Sirius

In the early 1950s Robert Hanbury Brown, a radio astronomer, wanted to measure the angular diameters of stars. The classical Michelson stellar interferometer, which correlates *amplitudes* from two apertures, demands optical-path stability to a fraction of a wavelength and is wrecked by atmospheric turbulence. Hanbury Brown's radical proposal, worked out mathematically with Richard Twiss, was to correlate *intensities* instead: feed the light from two separated detectors into an electronic multiplier and measure $\langle I_1 I_2\rangle$. Intensity correlations survive atmospheric phase scrambling, and their falloff with detector separation encodes the source's angular size. In 1956 the pair pointed their intensity interferometer at Sirius and measured its angular diameter — about $6.9$ milliarcseconds — the first result of what became the Narrabri stellar intensity interferometer (Hanbury Brown & Twiss, 1956).

To validate the technique in the laboratory, they performed the experiment that gave the field its name: split a beam of filtered thermal light (an arc lamp) on a half-silvered mirror, place a photomultiplier at each output, and correlate the two photocurrents. They found a **positive correlation at zero delay** — the two detectors tended to click together. Photons from a chaotic source arrive in *bunches*.

## The Apparatus and What It Measures

The modern HBT setup is disarmingly simple and is the standard $g^{(2)}$ measurement to this day. The light under test enters one port of a 50/50 beam splitter; the two outputs, modes $\hat{c}$ and $\hat{d}$, feed two single-photon detectors whose clicks are time-tagged and cross-correlated. One builds a histogram of the delay $\tau$ between a click in $\hat{c}$ and a click in $\hat{d}$; normalized by the accidental (uncorrelated) rate, that histogram *is* $g^{(2)}(\tau)$.

Why split the beam at all, rather than time-tag one detector against itself? Because real detectors are blind for a **dead time** of tens of nanoseconds after each click (and suffer afterpulsing), so a single detector physically cannot register the two near-simultaneous photons that the $\tau\to 0$ correlation is all about. Two detectors after a beam splitter sidestep the dead time entirely: the pair that would have arrived at one detector is instead shared between two. This is the whole trick of the "HBT geometry," and it is why the antibunching dip at $\tau=0$ (Section 18.1.3) is measurable at all.

The beam splitter does not distort the measured correlation. With the signal in port $\hat{a}$ and vacuum $\hat{v}$ in the unused port, the outputs are $\hat{c}=(\hat{a}+i\hat{v})/\sqrt2$ and $\hat{d}=(i\hat{a}+\hat{v})/\sqrt2$, and the cross-correlation between the two outputs reproduces the *auto*-correlation of the input, as the worked example now shows.

**Worked example.** *Relate the measured coincidence-to-singles ratio to $g^{(2)}(0)$.*

The coincidence rate is proportional to $\langle\hat{c}^\dagger\hat{d}^\dagger\hat{d}\hat{c}\rangle$ and each singles rate to $\langle\hat{c}^\dagger\hat{c}\rangle$, $\langle\hat{d}^\dagger\hat{d}\rangle$. Acting on the input state $|\psi\rangle_a|0\rangle_v$, every term containing the annihilation operator $\hat{v}$ vanishes (it meets vacuum), so

$$\hat{d}\,\hat{c} = \tfrac{1}{2}(i\hat{a}+\hat{v})(\hat{a}+i\hat{v}) \longrightarrow \tfrac{i}{2}\,\hat{a}^2, \qquad
\hat{c}^\dagger\hat{d}^\dagger\hat{d}\hat{c} \longrightarrow \tfrac{1}{4}\,\hat{a}^{\dagger2}\hat{a}^2.$$

The singles are $\langle\hat{c}^\dagger\hat{c}\rangle = \langle\hat{d}^\dagger\hat{d}\rangle = \tfrac{1}{2}\langle\hat{a}^\dagger\hat{a}\rangle$. The normalized cross-correlation between the two outputs is therefore

$$g^{(2)}_{cd}(0) = \frac{\langle\hat{c}^\dagger\hat{d}^\dagger\hat{d}\hat{c}\rangle}{\langle\hat{c}^\dagger\hat{c}\rangle\langle\hat{d}^\dagger\hat{d}\rangle} = \frac{\tfrac14\langle\hat{a}^{\dagger2}\hat{a}^2\rangle}{\tfrac14\langle\hat{a}^\dagger\hat{a}\rangle^2} = g^{(2)}(0).$$

The splitter is transparent to the statistics: the two-output coincidence rate divided by the product of singles rates returns the input's second-order coherence exactly. Concretely, in a **pulsed** experiment one histograms coincidences versus which pair of pulses fired: the side peaks (clicks from different pulses, hence uncorrelated) have area proportional to $\langle\hat{n}\rangle^2$, while the central peak (both clicks from the *same* pulse) has area proportional to $\langle\hat{n}(\hat{n}-1)\rangle$. Their ratio is $g^{(2)}(0)$ directly:

$$g^{(2)}(0) = \frac{\text{area of central } (\tau=0)\text{ peak}}{\text{average area of side peaks}}.$$

A source showing, say, $20$ central-peak coincidences against a side-peak average of $1000$ reports $g^{(2)}(0)=0.02$ — the sort of number that certifies a good single-photon source, read straight off the coincidence histogram.

## The Controversy and Its Resolution

The bunching result provoked genuine alarm. Several physicists — notably Brannen and Ferguson (1956) — argued that a positive photon-coincidence correlation would violate quantum mechanics, or at least the uncertainty principle, and initially failed to reproduce it. The objection rested on picturing photons as classical particles that ought to arrive independently. The resolution, articulated by Purcell and by Glauber's coherence theory (Section 18.1.1), was that bunching is not a violation of quantum mechanics but a *prediction* of it: for chaotic light the Bose-Einstein number statistics give $g^{(2)}(0)=2$, an intensity-fluctuation effect equally derivable from classical wave interference of many random phasors. HBT bunching, in short, is a classical phenomenon (it obeys $g^{(2)}(0)\ge 1$). Its lasting importance was methodological — it proved that intensity correlations are real, measurable, and informative — and conceptual: it forced Glauber to build the quantum theory of optical coherence that defines $g^{(n)}$, founding modern quantum optics and earning the 2005 Nobel Prize.

## What the Bunching Width Encodes

The *width* of the HBT correlation is as informative as its height. By the Siegert relation (Section 18.1.1), the bunching peak of chaotic light decays over the coherence time $\tau_c$, so a measurement of $g^{(2)}(\tau)$ reads off $\tau_c$ directly. This is precisely how the original stellar interferometer worked, transposed from time to space: correlating intensities at two detectors *separated in position* rather than in time, Hanbury Brown and Twiss measured the spatial coherence of starlight, whose falloff with baseline encodes the star's angular size through the van Cittert-Zernike theorem. The same instrument, run in the time domain on a laboratory source, measures temporal coherence; run in the space domain on a star, it measures angular diameter. Modern implementations replace the analog multiplier with picosecond time-to-digital converters that time-tag every click, building the full $g^{(2)}(\tau)$ histogram in software — but the topology, a beam splitter feeding two detectors into a coincidence counter, is unchanged since 1956.

## Why It Matters

The HBT interferometer is the workhorse instrument of this entire unit. Every $g^{(2)}(0)$ quoted for a single-photon source in Chapter 19, every antibunching dip in Section 18.1.3, and every heralded-source characterization in Section 18.3.1 is an HBT measurement. The apparatus that was invented to size a star now certifies the quantum purity of the photons that photonic quantum computers run on — the same two detectors behind the same beam splitter, asked whether the light bunches, arrives at random, or, most tellingly, refuses to arrive in pairs at all.
