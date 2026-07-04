# 22.2.2 — Quantum Memory

## The Job Description

A repeater node must hold its half of an entangled state while waiting — for the neighboring segment to succeed, and for classical heralding messages to travel. A **quantum memory** is a device that absorbs a photonic qubit into a matter degree of freedom, preserves its quantum state (including its entanglement with a distant partner), and re-emits it *on demand*. The specification sheet, each line traceable to the repeater protocol:

- **Efficiency** $\eta_m$ (in-and-out probability): enters the rate multiplicatively at *every* node; $0.9^{10}$ across ten operations is already a 65% haircut. Target: $>90\%$.
- **Storage time**: must exceed the classical heralding time. One 50 km segment's herald takes $\sim 250\ \mu$s (light in fiber travels $\sim 200$ km/ms); end-to-end coordination over 1,000 km, with retries, pushes requirements to *milliseconds to seconds*.
- **Bandwidth**: the memory must swallow the photon it is offered — MHz for atomic-linewidth photons, GHz for the broadband photons that give high attempt rates. Bandwidth × storage-time is the figure that separates platforms.
- **Multimode capacity**: storing $M$ temporal/spectral/spatial modes at once multiplies the entanglement attempt rate by $M$ without new hardware — the single cheapest rate lever in repeater design.
- **Wavelength**: fiber demands 1550 nm; most good memories absorb elsewhere (795 nm Rb, 606 nm Pr, 637 nm NV), requiring low-noise **quantum frequency conversion**, or intrinsically telecom species (erbium at 1536 nm).
- **Fidelity**: $>99\%$ output fidelity, since purification (22.2.3) can polish only so much.

No demonstrated memory meets all lines simultaneously; the platforms below each excel on a subset.

## Atomic Ensembles and the DLCZ Protocol

An ensemble of $N \sim 10^{6}$–$10^{11}$ alkali atoms (warm vapor cell or cold, trapped cloud) stores a photon as a *collective spin excitation* — one flipped atom, coherently delocalized over all of them ("spin wave"). Collectivity is the magic: emission back into the original optical mode is enhanced by constructive interference of all $N$ amplitudes, making retrieval efficient and directional.

The **DLCZ protocol** (Duan, Lukin, Cirac & Zoller, 2001 [1]) noticed that such an ensemble is simultaneously a *source* and a *memory*: a weak write laser occasionally scatters a Raman photon, whose detection heralds — with certainty — that one collective excitation is stored. Two ensembles at segment ends, their heralding photons interfered at a middle beam splitter, become entangled upon a single click (which cannot tell where the photon came from). Entanglement generation, heralding, and storage in one primitive: the DLCZ architecture remains the conceptual template for ensemble-based repeaters, and its descendants (with cavity enhancement and telecom conversion) power most repeater-segment demonstrations.

For *absorptive* storage of externally supplied photons, ensembles use electromagnetically induced transparency (EIT) — the control-laser-induced transparency window that slowed light to bicycle speed in 1999 and stopped it entirely in 2001 — or off-resonant Raman and gradient-echo (GEM) techniques. Records: $\sim 92\%$ storage-retrieval efficiency in cold Rb EIT [2], $\sim 87\%$ in warm-vapor GEM [3], with millisecond-class storage in optically trapped ensembles; a cold-atom DLCZ memory has reached subsecond entanglement storage. Warm vapor cells are attractive for their utter simplicity (a heated glass cell); cold atoms buy longer coherence at the cost of laser-cooling infrastructure.

## Rare-Earth Crystals and the Atomic Frequency Comb

Rare-earth ions (Pr³⁺, Eu³⁺, Er³⁺) doped into cryogenic crystals like Y₂SiO₅ hide their 4f electrons inside filled shells, giving optical and spin transitions of extraordinary coherence for a solid — the record is **six hours** of spin coherence in Eu³⁺:Y₂SiO₅ under dynamical decoupling [4]. Because the ions are frozen in place (no atomic motion, no diffusion), huge inhomogeneously broadened ensembles can be spectrally sculpted by hole burning.

The **atomic frequency comb (AFC)** memory (Afzelius et al., 2009 [5]) burns the inhomogeneous absorption line into a comb of narrow teeth spaced by $\Delta$. An absorbed photon excites a superposition across all teeth; the components dephase, then — because the detunings are integer multiples of $\Delta$ — *rephase* automatically at $t = 2\pi/\Delta$, re-emitting the photon as a collective echo. Transferring the excitation to a spin level before the echo (with control pulses) converts the fixed delay into on-demand recall. The AFC's superpower is **multimode capacity**: the comb stores long trains of temporal modes simultaneously (dozens to $>1{,}000$ modes demonstrated across temporal and spectral multiplexing), directly multiplying repeater attempt rates. Efficiencies reach $\sim 56\text{–}69\%$ (with cavity or gradient-echo enhancement [6]), and in 2021 two independent experiments heralded entanglement between AFC crystal memories in separate labs using telecom-wavelength heralding photons [7] — solid-state repeater segments in embryo. Erbium's 1536 nm transition offers the tantalizing prospect of telecom-native storage.

## Single Emitters: Ions, Atoms, and Color Centers

The ensemble's opposite pole: one atom (or ion, or defect) in a high-finesse cavity, where the Purcell effect (Chapter 19) makes a single emitter absorb and emit into one mode efficiently. Single emitters add what ensembles lack — *processing*. A trapped ion or a nitrogen-vacancy (NV) center is a full qubit: it can herald, store (NV: the nearby ¹³C nuclear spins hold quantum states for minutes), and *operate* — performing the Bell measurements and purification logic of a repeater node locally. The Delft NV-center experiments used exactly this to build multi-node network demonstrations (Section 22.3.2), and a silicon-vacancy center in a diamond nanocavity at Harvard demonstrated **memory-enhanced quantum communication** in 2020 — the first device to beat the direct-transmission bound using a memory [8]. Costs: single emitters are single-mode (no multiplexing bonus), often slow (attempt rates limited by cavity and detection), and live at visible/near-IR wavelengths needing frequency conversion; ions and NVs also require vacuum/cryogenic overhead.

## The Scorecard

| Platform | Efficiency | Storage time | Multimode | Telecom | Processing |
|---|---|---|---|---|---|
| Warm vapor (EIT/GEM) | up to ~87% | μs–ms | modest | via QFC | no |
| Cold atoms (DLCZ/EIT) | up to ~92% | ms–s | modest | via QFC | limited |
| Rare-earth AFC | ~55–69% | ms (opt.), hours (spin, non-on-demand) | ★ 10²–10³ modes | Er native / QFC | no |
| Single ion/atom in cavity | high (cavity-limited) | s–min | 1 | via QFC | ★ full qubit logic |
| NV/SiV in diamond | moderate | min (nuclear spins) | 1 | via QFC | ★ full qubit logic |

The repeater architectures of the next subsection mix these: ensembles/AFCs where rate and multimode capacity dominate (the long segments), emitters where logic is needed (purification and swapping nodes). The single number to watch across all platforms is $\eta_m \times$ (modes) $\times$ (attempts per second) against the required storage time — today's demonstrations still fall one to three orders of magnitude short of what a transcontinental repeater chain demands, which is why no full quantum repeater network yet exists.

## Summary

- A repeater memory must combine efficiency ($>90\%$), storage exceeding classical heralding times (ms–s), matched bandwidth, multimode capacity, telecom compatibility, and high fidelity — no platform yet has all six.
- DLCZ ensembles unify source, herald, and memory via collective spin excitations; EIT/GEM ensembles reach 87–92% efficiency.
- Rare-earth AFC memories offer massive temporal multimode storage and record spin coherence (6 h in Eu:YSO); telecom-heralded entanglement between crystals was demonstrated in 2021.
- Single emitters (ions, NV/SiV) trade multimode capacity for on-board qubit logic; a SiV memory beat the direct-transmission bound in 2020.
- The memory is the pacing technology of the quantum internet: every architecture in Section 22.3 is designed around its limitations.

---

*References*

[1] Duan, L.-M., Lukin, M.D., Cirac, J.I., & Zoller, P. (2001). Long-distance quantum communication with atomic ensembles and linear optics. *Nature*, 414, 413–418. [DOI: 10.1038/35106500] [DLCZ.]

[2] Hsiao, Y.-F., et al. (2018). Highly efficient coherent optical memory based on electromagnetically induced transparency. *Physical Review Letters*, 120(18), 183602. [DOI: 10.1103/PhysRevLett.120.183602]

[3] Hosseini, M., Sparkes, B.M., Campbell, G., Lam, P.K., & Buchler, B.C. (2011). High efficiency coherent optical memory with warm rubidium vapour. *Nature Communications*, 2, 174. [DOI: 10.1038/ncomms1175]

[4] Zhong, M., Hedges, M.P., Ahlefeldt, R.L., Bartholomew, J.G., Beavan, S.E., Wittig, S.M., Longdell, J.J., & Sellars, M.J. (2015). Optically addressable nuclear spins in a solid with a six-hour coherence time. *Nature*, 517, 177–180. [DOI: 10.1038/nature14025]

[5] Afzelius, M., Simon, C., de Riedmatten, H., & Gisin, N. (2009). Multimode quantum memory based on atomic frequency combs. *Physical Review A*, 79(5), 052329. [DOI: 10.1103/PhysRevA.79.052329]

[6] Hedges, M.P., Longdell, J.J., Li, Y., & Sellars, M.J. (2010). Efficient quantum memory for light. *Nature*, 465, 1052–1056. [DOI: 10.1038/nature09081]

[7] Lago-Rivera, D., Grandi, S., Rakonjac, J.V., Seri, A., & de Riedmatten, H. (2021). Telecom-heralded entanglement between multimode solid-state quantum memories. *Nature*, 594, 37–40. [DOI: 10.1038/s41586-021-03481-8] [See also Liu, X., et al., *Nature*, 594, 41–45 (2021).]

[8] Bhaskar, M.K., et al. (2020). Experimental demonstration of memory-enhanced quantum communication. *Nature*, 580, 60–64. [DOI: 10.1038/s41586-020-2103-5]

[9] Sangouard, N., Simon, C., de Riedmatten, H., & Gisin, N. (2011). Quantum repeaters based on atomic ensembles and linear optics. *Reviews of Modern Physics*, 83(1), 33–80. [DOI: 10.1103/RevModPhys.83.33] [The comprehensive repeater-architecture review.]
