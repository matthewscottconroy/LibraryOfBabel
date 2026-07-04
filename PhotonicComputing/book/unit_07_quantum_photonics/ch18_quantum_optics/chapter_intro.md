# Chapter 18: Quantum Optics — From Photon Statistics to Squeezing

> *"Each photon then interferes only with itself. Interference between two different photons never occurs."*
>
> — P.A.M. Dirac, *The Principles of Quantum Mechanics*, 1930

---

## Why This Chapter Exists

Chapter 17 built the states. This chapter puts them on an optical table and asks what a detector actually measures. The transition is not cosmetic. A Fock state and a coherent state of the same mean energy are indistinguishable to a power meter; they are worlds apart to a pair of photon counters watching for coincidences. Quantum optics is the discipline of the *correlation function* — of asking not "how bright?" but "given a click now, how likely is another click a nanosecond later?" — and the answers partition light into classical and non-classical in a way that no measurement of intensity or spectrum ever can.

Dirac's famous dictum in the epigraph is the right place to start, because this chapter is largely the story of its fine print. First-order interference — fringes in a Michelson interferometer — is indeed each photon interfering with itself, and it is *blind* to photon statistics: thermal light and laser light of the same spectrum produce identical fringes. The revolution of Hanbury Brown and Twiss was to measure *second*-order coherence, intensity correlations, where the statistics live. And the Hong-Ou-Mandel effect (Section 18.2.2) is exactly the phenomenon that appears to violate Dirac and does not: two photons meeting at a beam splitter interfere, but what interferes are two-*photon* probability amplitudes for indistinguishable paths, not the photons as classical waves. Dirac survives, reinterpreted — and the reinterpretation is the engine of photonic quantum computing.

Three threads run through the chapter, and all three are load-bearing for the rest of the unit:

- **Photon statistics** (Section 18.1) give us $g^{(2)}(0)$, the number that certifies a single-photon source. Antibunching, $g^{(2)}(0) < 1$, is a strict non-classicality witness: no classical field, however cleverly prepared, can produce it. This is the acceptance test every emitter in Chapter 19 must pass.
- **Two-photon interference** (Section 18.2) is the only "interaction" that linear optics offers. Photons do not touch; but two indistinguishable photons at a beam splitter refuse to leave by separate ports, and that refusal — the HOM effect — is the primitive from which every linear-optical gate in Chapter 20 is built. Its quality metric, indistinguishability, must exceed 99.9% for fault-tolerant machines.
- **Squeezing** (Section 18.3) is quantum noise engineering. Parametric processes create photons in pairs, and pairs are the raw material of both entanglement (heralded single photons, EPR states) and sub-vacuum quadrature noise. Squeezed light is the one non-classical resource that has already shipped in a metrological instrument at civilization scale: LIGO runs on it.

## The Arc of This Chapter

**Section 18.1 — Photon Statistics and Non-Classical Light.** We define the second-order coherence function $g^{(2)}(\tau)$ and compute it for the canonical states — coherent ($g^{(2)}(0)=1$), thermal ($=2$), Fock $|n\rangle$ ($=1-1/n$) — alongside the Mandel $Q$ parameter (18.1.1). We then reconstruct the Hanbury Brown-Twiss experiment (18.1.2), the two-detector coincidence apparatus that measures $g^{(2)}$ and that first exposed photon bunching in thermal light. Antibunching (18.1.3) closes the section: the Kimble-Dagenais-Mandel single-atom experiment of 1977, the physical necessity that one emitter cannot emit twice at once, and the Grangier-Roger-Aspect anticorrelation that ruled out the last classical picture of light on a beam splitter.

**Section 18.2 — The Quantum Beam Splitter.** The beam splitter is a two-mode unitary; we derive its transformation, the unitarity constraints on its reflection and transmission coefficients, and confront the necessity of the vacuum port (18.2.1). Then the Hong-Ou-Mandel effect (18.2.2): the explicit algebra by which $|1,1\rangle \to \tfrac{i}{\sqrt2}(|2,0\rangle+|0,2\rangle)$ and the coincidence rate collapses to zero. Finally we argue why this single phenomenon is the primitive of linear optical quantum computing (18.2.3), and how partial distinguishability degrades gate fidelity.

**Section 18.3 — Optical Parametric Processes and Squeezing.** The $\chi^{(2)}$ parametric interaction and spontaneous parametric down-conversion (18.3.1) yield two-mode squeezing, entangled pairs, and heralded single photons. The degenerate case gives single-mode squeezing (18.3.2), the decibel scale, and the brutal loss-sensitivity that caps every experiment. We end at LIGO (18.3.3), where squeezed vacuum injected into a dark port turned a 1981 thought experiment into gravitational-wave astronomy.

## Prerequisites

This chapter assumes Chapter 17 in full: the quantized field and its ladder operators; the Fock, coherent, and squeezed states; the quadrature operators $\hat{X}_1 = (\hat{a}+\hat{a}^\dagger)/2$ and $\hat{X}_2 = (\hat{a}-\hat{a}^\dagger)/2i$ with vacuum noise $\Delta X_1 = \Delta X_2 = 1/2$; the density matrix; and the language of entanglement. The single- and two-mode squeezing operators of Section 17.3.3 reappear here attached to the crystals that realize them. No new mathematics is required — only the discipline of normal ordering and the patience to track two modes at once.
