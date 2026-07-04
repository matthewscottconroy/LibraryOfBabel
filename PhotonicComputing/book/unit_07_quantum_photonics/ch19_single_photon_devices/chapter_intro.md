# Chapter 19: Single-Photon Sources and Detectors

> *"The single photon is the fundamental resource of quantum photonics. Everything else — gates, algorithms, networks — is bookkeeping about where the photons went."*
>
> — A sentiment shared widely in the photonic quantum computing community

---

## From Quantum Optics to Quantum Hardware

Chapters 17 and 18 established the theory: light is quantized, Fock states exist, $g^{(2)}(0) < 1$ certifies nonclassicality, and two indistinguishable photons bunch at a beam splitter. This chapter turns that theory into hardware. A photonic quantum computer is, at the component level, three things: a device that produces single photons on demand, a circuit that interferes them, and a device that detects them one at a time. The interference circuits are the silicon photonic components of Unit 3. The sources and detectors are new — and they are the components on which every photonic quantum technology currently rises or falls.

The requirements are unforgiving. A useful single-photon source must be simultaneously **bright** (a photon delivered into the collection mode on nearly every trigger), **pure** ($g^{(2)}(0) < 0.01$, meaning essentially zero multi-photon contamination), and **indistinguishable** (consecutive photons interfere with Hong-Ou-Mandel visibility approaching unity). Each property is individually achievable; achieving all three at once has taken thirty years of device physics and is still not fully solved. A useful single-photon detector must register a single quantum of energy — about $1.3 \times 10^{-19}$ J at 1550 nm — with efficiency above 90%, timing uncertainty of tens of picoseconds or less, and a false-count rate of a few per second. Superconducting nanowire single-photon detectors (SNSPDs) now achieve system detection efficiencies above 98%, timing jitter down to ~3 ps in research devices (~15 ps in commercial systems), and dark count rates below 1 count per second — arguably the most mature component in the entire quantum photonics toolbox.

Behind both sources and detectors stands a single body of physics: **cavity quantum electrodynamics**, the study of a single emitter coupled to a single optical mode. The Jaynes-Cummings model, the strong-coupling regime, and the Purcell effect explain why placing a quantum dot inside a micropillar cavity transforms it from a dim, decoherence-prone emitter into one of the best single-photon sources ever built — and they set the ultimate limits on what an engineered emitter can do.

---

## Why This Chapter Matters for Photonic Computing

The linear optical quantum computing architectures of Chapter 20 consume single photons at staggering rates — a fault-tolerant photonic machine requires on the order of $10^{10}$–$10^{12}$ high-quality photons per second — and every photon must eventually be detected. The performance numbers in this chapter (source efficiency $\eta$, detector efficiency $\eta_d$, jitter, dark counts) propagate directly into the loss budgets and error-correction thresholds of Chapter 20. An $n$-photon experiment succeeds at a rate proportional to $\eta^n$: with $\eta = 0.5$, a 20-photon coincidence is suppressed by a factor of $10^6$; with $\eta = 0.98$, by a factor of only ~1.5. This exponential sensitivity is why fractions of a percent in component efficiency are worth years of engineering effort.

---

## Chapter Structure

**Section 19.1 — Single-Photon Sources**: Figures of merit (brightness, purity, indistinguishability); semiconductor quantum dots; color centers in diamond and 2D materials; heralded SPDC sources and multiplexing.

**Section 19.2 — Single-Photon Detectors**: Semiconductor avalanche detectors (SPADs and APDs); superconducting nanowire single-photon detectors (SNSPDs); transition-edge sensors, photon-number resolution, and waveguide-integrated detection.

**Section 19.3 — Cavity Quantum Electrodynamics**: The Jaynes-Cummings model; the strong-coupling regime and photon blockade; the Purcell effect and its central role in bright single-photon sources.
