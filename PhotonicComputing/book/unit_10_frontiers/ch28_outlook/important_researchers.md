# Chapter 28: Important Researchers

*The researchers profiled here are chosen for framing the limits, open problems, and honest outlook of the field — the people whose analyses define what photonic computing can and cannot do, and how to tell the difference.*

---

## David A. B. Miller

David A. B. Miller (Stanford) is the intellectual keystone of this chapter, because he articulated both of its central open problems with unusual rigor. His 2010 essay asked "are optical transistors the logical next step?" and answered, in effect, no — laying out the criteria a device must satisfy to serve as a cascadable logic element and showing why no all-optical switch meets them at low energy [Miller, *Nature Photonics*, 2010]. His attojoule-optoelectronics program then argued that photonics' decisive contribution is communication, driving device energy toward the fundamental limits set by charging and detection rather than by the transistor [Miller, *Journal of Lightwave Technology*, 2017]. The missing-transistor problem (Section 28.2.1) and the interconnect-first thesis (Section 28.2.2) are both, in large part, his framing.

---

## Rolf Landauer

Rolf Landauer (IBM) established the thermodynamic foundation against which all of Section 28.1 is measured. His 1961 analysis showed that logically irreversible operations — the erasure of a bit — carry an unavoidable heat cost of at least $k_B T \ln 2$, giving physics its most famous statement that "information is physical" [Landauer, *IBM Journal of Research and Development*, 1961]. The paradox the chapter draws from him is instructive: the Landauer floor is genuine and experimentally confirmed, yet real photonic and electronic operations sit 8–10 orders of magnitude above it, so his limit is best understood not as a target but as proof that today's constraints are entirely of engineering, not thermodynamics.

---

## Peter L. McMahon

Peter L. McMahon (Cornell) wrote what has become the field's most disciplined statement of where optics can and cannot win. His 2023 review insists on end-to-end energy accounting, matched baselines, and a clean separation between what the physics permits and what a useful computer requires [McMahon, *Nature Reviews Physics*, 2023]. It is the analytical spine of this chapter: nearly every caution here — about conversion overhead, precision, and the difference between a demonstration and an advantage — is stated carefully in his survey, which is the single best companion reading to the outlook.

---

## Ryan Hamerly

Ryan Hamerly (MIT and NTT Research) supplied the energy analysis that makes the end-to-end argument concrete. His work on optical neural networks based on photoelectric multiplication quantified how coherent detection can perform multiply-accumulates at very low optical energy, while making explicit the $1/N$ amortization of the optical operation and the shot-noise-limited regime in which it runs [Hamerly et al., *Physical Review X*, 2019]. His analysis is the technical origin of the converter-wall reasoning in Section 28.2.2 and connects directly to the standard-quantum-limit discussion of Section 28.1.3.

---

## Dirk Englund

Dirk Englund (MIT) leads one of the groups pushing photonic processors toward their quantum noise floor. His team demonstrated an optical neural network operating at less than one photon per multiplication, probing directly the shot-noise limit of optical computing and showing that accurate inference survives even in the extreme photon-starved regime [Wang et al., *Nature Communications*, 2022]. This result anchors the precision-energy discussion of Sections 28.1.2 and 28.1.3, turning the abstract SQL into a measured operating point for a working classifier.

---

## Bhavin J. Shastri

Bhavin J. Shastri (Queen's University) is a principal cartographer of the neuromorphic-photonics landscape. He led the field's most cited review, which maps the research base of photonic AI hardware and states plainly what it can and cannot yet deliver [Shastri et al., *Nature Photonics*, 2021]. His framing — that photonics is a strong candidate for specific linear and interconnect-heavy kernels rather than a wholesale replacement for digital electronics — is the sober outlook that Section 28.3 builds toward.

---

## Harish Bhaskaran, Wolfram Pernice, and C. David Wright

The Oxford–Münster–Exeter collaboration of Harish Bhaskaran, Wolfram Pernice, and C. David Wright is central to the photonic-memory problem of Section 28.2.3. They demonstrated integrated, all-photonic, non-volatile multi-level memory using phase-change materials [Ríos et al., *Nature Photonics*, 2015] and extended the approach to all-optical spiking neurosynaptic networks with on-chip learning [Feldmann et al., *Nature*, 2019]. Their work defines both the promise and the limit of the leading candidate for optical storage: phase-change memory is genuinely non-volatile and multi-level, but endurance and write energy keep it from answering the memory wall outright.

---

## Alexander N. Tait

Alexander N. Tait (Queen's University; formerly NIST) brought accounting discipline to photonic neural networks by quantifying where the power actually goes. His analysis of power use in silicon photonic neural networks tallied the contributions of lasers, modulators, detectors, and conversion, showing that the surrounding electronics — not the optical core — dominates the realistic budget [Tait, *Physical Review Applied*, 2022]. This is the empirical backbone of the converter-wall conclusion in Section 28.2.2 and a model of the end-to-end evaluation the chapter urges.

---

## Keren Bergman

Keren Bergman (Columbia University) is the principal academic architect of optically interconnected computing systems, and her energy–bandwidth analyses of photonic networks-on-chip and links for high-performance computing long anticipated the field's revealed answer. Her systems-level argument — that photonics' first decisive role inside computers is communication, because data movement is the binding energy constraint — is the interconnect-first thesis (Sections 28.2.2 and 28.3.2) stated from the architecture side, complementing Miller's device-level case and forming the through-line of the chapter's outlook.
