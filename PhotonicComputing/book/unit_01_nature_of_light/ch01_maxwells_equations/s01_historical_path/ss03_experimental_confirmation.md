# 1.1.3 Experimental Confirmation: Hertz and the Detection of Radio Waves

## The Prediction Awaiting Test

By 1865, Maxwell had a complete set of equations and a remarkable prediction: there exist electromagnetic waves propagating at the speed of light. But the prediction was purely theoretical. No one had ever *generated* electromagnetic waves in the laboratory, nor observed them propagate from a source to a distant detector. Light was already known to exist, but connecting it to Maxwell's prediction required showing that artificially generated electromagnetic waves behaved as the theory predicted.

The experimental confirmation came twenty-three years later, from Heinrich Hertz.

## Hertz's Experiments (1886–1888)

Heinrich Hertz, a student of Helmholtz at the University of Berlin, set out systematically to test Maxwell's predictions [1]. He constructed an oscillating electric circuit — a spark gap between two metal spheres — that generated rapid oscillations of electric and magnetic field. According to Maxwell's theory, these oscillating fields should radiate electromagnetic waves.

To detect the waves, Hertz constructed a simple loop of wire with a small gap. When the waves passed through the detector loop, they would induce an oscillating current and produce a visible spark across the gap.

Hertz's key findings:
1. **Waves were produced and detected**: A spark in the detector occurred when the transmitter sparked, even with no direct electrical connection between them.
2. **The waves could be reflected**: Directing the transmitter at a zinc sheet and positioning the detector appropriately showed clear reflection, just as light reflects from mirrors.
3. **The waves could be refracted**: Hertz built a large prism from pitch (a material with a known refractive index for these wavelengths) and showed that the waves bent as they passed through it, exactly as predicted by Snell's law.
4. **Standing waves and wavelength measurement**: By reflecting waves from a flat metal sheet and measuring the positions of nodes and antinodes, Hertz determined the wavelength. Combined with the known frequency of his circuit, he computed the wave speed — and found $c \approx 3 \times 10^8$ m/s, precisely Maxwell's prediction [2].
5. **Polarization**: The waves were transverse (the electric field oscillated perpendicular to the direction of propagation) and could be polarized, just as Maxwell's theory predicted.

## What the Experiments Proved

Hertz's experiments established, beyond reasonable doubt:
- Electromagnetic waves exist and can be generated artificially.
- They propagate at the speed of light.
- They are transverse waves (the field oscillates perpendicular to propagation).
- They obey the same laws of reflection and refraction as light.
- They can be polarized.

All of these properties are direct consequences of Maxwell's equations. The identification of light as an electromagnetic wave was confirmed.

## Relevance to Photonic Computing

The Hertz experiments established that the same physical theory — Maxwell's equations — governs both the electromagnetic waves we call radio waves and the electromagnetic waves we call light. The difference is only wavelength: radio waves at meters to kilometers, visible light at 400–700 nanometers, the telecom wavelengths used in photonic computing at around 1550 nanometers.

This continuity matters for photonic computing engineers: the physics of microwave photonics (Section 11.2), the physics of optical fibers, and the physics of laser-driven photonic processors all derive from the same four equations. The boundary between "microwave engineering" and "optics" is an engineering distinction, not a physical one.

## Oliver Heaviside and the Modern Form of Maxwell's Equations

A significant debt is owed to Oliver Heaviside, who between 1885 and 1887 reformulated Maxwell's original 20 equations (Maxwell wrote his theory in terms of quaternions and potentials) into the four vector equations we use today [3]. Maxwell never wrote "Maxwell's equations" in the form $\nabla \times \mathbf{E} = -\partial\mathbf{B}/\partial t$; that compact notation is Heaviside's. Heaviside also introduced the concepts of impedance, the telegraph equation, and made fundamental contributions to vector calculus as a tool for physics.

The four equations that now bear Maxwell's name are, more precisely, the Heaviside restatement of Maxwell's theory — but they are equivalent, and the insight and physics are Maxwell's.

---

## References

[1] Hertz, H. (1888). "Über sehr schnelle electrische Schwingungen." *Annalen der Physik und Chemie*, 267(7), 421–448. Translated as "On very rapid electric oscillations." [The primary experimental paper.]

[2] Hertz, H. (1888). "Über die Ausbreitungsgeschwindigkeit der electrodynamischen Wirkungen." *Annalen der Physik*, 270(7), 551–569. [Hertz's measurement of the propagation speed.]

[3] Heaviside, O. (1893). *Electromagnetic Theory*, Vol. I. London: The Electrician Publishing Co. [Heaviside's reformulation of Maxwell's equations in modern vector form.]

[4] Hunt, B.J. (1991). *The Maxwellians*. Cornell University Press. [Excellent scholarly history of the development and reception of Maxwell's theory, including Heaviside's role.]
