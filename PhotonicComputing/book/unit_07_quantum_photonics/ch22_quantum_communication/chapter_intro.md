# Chapter 22: Quantum Communication and the Quantum Internet

> *"When elementary quantum systems, such as polarized photons, are used to transmit digital information, the uncertainty principle gives rise to novel cryptographic phenomena unachievable with traditional transmission media."*
>
> — Charles H. Bennett and Gilles Brassard, Bangalore, 1984

---

## The Photon's Home Turf

Every other chapter in this unit asks photons to do something they resist: interact. Quantum *communication* asks them to do exactly what they do best — travel far and fast while ignoring everything around them. A telecom photon in optical fiber propagates at two thirds the speed of light and interacts so weakly with its environment that its quantum state survives hundreds of kilometers. No other qubit can leave the laboratory at all. If quantum information is ever to move between two places, it will move on light.

That simple observation makes this chapter the most commercially mature in the book's quantum unit. Quantum key distribution (QKD) — using single photons to establish a shared secret key whose secrecy is guaranteed by physics rather than by computational assumptions — was proposed in 1984, first demonstrated over 32 cm of air in 1989, and is now sold by multiple companies, deployed in metropolitan networks on three continents, standardized by ETSI and ITU, and operated over a 2,000 km trusted-node backbone in China and via satellite across 7,600 km between continents. The engineering chain runs directly through this book's earlier chapters: attenuated lasers and modulators (Unit II and Chapter 9), low-loss fiber (Chapter 6), and superconducting single-photon detectors (Chapter 19).

The chapter's second theme is the obstacle that dominates everything: **loss, and the impossibility of amplifying around it**. Classical optical networks defeat fiber attenuation with erbium-doped amplifiers every 80 km. The no-cloning theorem — provable in four lines — forbids any quantum equivalent: an unknown quantum state cannot be copied, so it cannot be regenerated. Direct quantum transmission therefore dies exponentially with distance, a fact formalized by the PLOB bound on the secret bits extractable per channel use. At 1,000 km of fiber, the numbers become absurd: a 10 GHz single-photon source would deliver roughly one photon per three centuries.

Everything in the second half of the chapter is the world's response to that exponential. *Quantum repeaters* divide the distance into segments, generate entanglement over each, store it in quantum memories, and stitch segments together by entanglement swapping — trading exponential loss for polynomial overhead, at the price of needing a component (the quantum memory) that is still a laboratory artifact. *Satellites* change the loss law itself: free space has no absorption, only diffraction, so a photon's odds of surviving 1,200 km from orbit exceed its odds of surviving 400 km of fiber by many orders of magnitude — the insight behind the Micius satellite's string of firsts. And the *quantum internet* roadmap organizes these capabilities into stages, from today's trusted-node QKD networks to the eventual goal: entanglement, on demand, between any two points on Earth, enabling distributed quantum computing, blind cloud computing, and sensor networks correlated at the quantum limit.

## The Arc of This Chapter

**Section 22.1 — Quantum Key Distribution** develops the flagship application. *22.1.1* works through BB84 in full — sifting, error estimation, privacy amplification, and why security follows from the impossibility of distinguishing non-orthogonal states — plus the entanglement-based E91/BBM92 family. *22.1.2* confronts implementation: weak coherent pulses, the photon-number-splitting attack and its decoy-state cure, CV-QKD with homodyne detection, detector attacks and MDI-QKD, twin-field QKD's square-root escape from the PLOB bound, and the record table of key rates versus distance. *22.1.3* leaves the fiber for free space: the Micius satellite, kilohertz keys over 1,200 km, entanglement distribution between ground stations, and intercontinental quantum-secured video calls.

**Section 22.2 — Quantum Repeaters** faces the distance problem head on. *22.2.1* proves no-cloning and traces its consequences. *22.2.2* surveys quantum memories — atomic ensembles, rare-earth crystals, single defects — and their efficiency/storage-time/multimode scorecard. *22.2.3* develops the repeater toolkit: teleportation, entanglement swapping, purification, and the rate scaling that makes segmented entanglement distribution polynomial rather than exponential in distance.

**Section 22.3 — The Quantum Internet** zooms out. *22.3.1* presents the staged capability roadmap (trusted-repeater networks through full quantum computing networks) and where deployed systems sit on it today. *22.3.2* examines network architecture: the physical/link/network protocol stack, entanglement routing, platform choices for end nodes, and the demonstrator networks now running.

## Prerequisites

Chapter 17 (superposition, measurement, no-cloning's conceptual basis), Chapter 18 (coherent states, photon statistics), Chapter 19 (single-photon sources and detectors — SNSPD efficiency and dark counts set QKD's distance records), Chapter 20 (Bell states and Bell measurements), and Chapter 21 (homodyne detection, for CV-QKD). Fiber attenuation arithmetic (0.2 dB/km at 1550 nm, Chapter 6) is used constantly.

---

## References for the Chapter Introduction

[1] Bennett, C.H. & Brassard, G. (1984). Quantum cryptography: Public key distribution and coin tossing. *Proceedings of the IEEE International Conference on Computers, Systems and Signal Processing*, Bangalore, India, 175–179. [Reprinted in *Theoretical Computer Science*, 560, 7–11 (2014).]

[2] Wootters, W.K. & Zurek, W.H. (1982). A single quantum cannot be cloned. *Nature*, 299, 802–803. [DOI: 10.1038/299802a0]

[3] Kimble, H.J. (2008). The quantum internet. *Nature*, 453, 1023–1030. [DOI: 10.1038/nature07127]

[4] Wehner, S., Elkouss, D., & Hanson, R. (2018). Quantum internet: A vision for the road ahead. *Science*, 362(6412), eaam9288. [DOI: 10.1126/science.aam9288]
