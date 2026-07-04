# 21.2.1 — The Borealis Architecture

## From Spatial to Temporal Modes

Every photonic processor in Chapter 20 encoded its modes *spatially*: $N$ modes meant $N$ waveguides and $O(N^2)$ physical beam splitters. Jiuzhang, the 2020 Chinese boson-sampling machine, filled an optical table with 100 spatial modes and 300 beam splitters — and its interferometer was frozen in place, unprogrammable, which left room for debate about how general the demonstrated advantage was.

Borealis, the machine with which Xanadu reported quantum computational advantage in 2022 [1], inverted the architecture. Its modes are *temporal*: a single squeezed-light source fires a train of pulses, one every 167 ns, and each pulse is one mode. Entangling gates between pulses are implemented by *fiber delay loops*: a variable beam splitter (VBS) either routes a pulse into a loop or interferes it with the pulse currently emerging from the loop. A loop of delay $\tau$ (one pulse period) couples each pulse to its nearest neighbor; a loop of delay $6\tau$ couples pulses six time-bins apart; a loop of $36\tau$ couples pulses 36 bins apart. Borealis uses exactly these three loops in series, giving each of its 216 modes programmable interactions at three ranges — effectively a three-dimensional coupling topology, unrolled in time.

The economy is startling: **one squeezer, three electro-optically controlled beam splitters, and three spools of fiber replace what would spatially require 216 sources and thousands of interferometer elements.** Every VBS setting (a rotation angle and a beam-splitter angle per pulse per loop) is updated dynamically as the pulse train streams through, so the machine is *fully programmable*: the user uploads the gate list, and the same hardware executes any circuit in its family. Time multiplexing is the same trick that let the Furusawa and Andersen groups build million-mode cluster states (Section 21.1.2); Borealis weaponized it for a sampling task.

## Anatomy of the Machine

1. **Source.** A pulsed optical parametric oscillator generates single-mode squeezed vacuum states (several dB of squeezing) at the 6 MHz clock rate — deterministically, one squeezed pulse per clock tick, at room temperature.
2. **Interferometer.** The three time-multiplexed VBS + loop stages apply the programmable Gaussian circuit. Total transmission must be kept high: as with all squeezed-light machinery, loss converts quantum correlations into vacuum noise and is the dominant imperfection.
3. **Readout.** The pulse train is demultiplexed into 16 spatial channels feeding superconducting transition-edge sensors (TES, Chapter 19) — genuine photon-number-resolving detectors, since Gaussian boson sampling requires counting photons per mode, not merely detecting them. Samples contained large photon numbers — up to around 220 photons across the 216 modes in the biggest runs.
4. **Control.** Room-temperature electronics stream the phase/angle settings; the cryogenics exist only for the detectors.

The computational task is Gaussian boson sampling (GBS, Section 20.4): sampling photon-number patterns whose probabilities are proportional to *hafnians* of submatrices determined by the Gaussian state — a #P-hard quantity to compute classically. In the flagship runs, Borealis produced a sample every $36\ \mu\text{s}$, while the authors estimated that the best known exact classical algorithm running on the Fugaku supercomputer would need more than $9{,}000$ years per sample — a runtime advantage of roughly 18 orders of magnitude, achieved with full programmability and public cloud access (Xanadu Cloud, later Amazon Braket) [1].

## The Fine Print: Loss and Classical Spoofing

Quantum-advantage claims are duels with classical algorithm designers, and GBS experiments give the classical side a specific opening: *photon loss*. Real machines transmit well under half their photons end to end; loss both shrinks the effective quantum correlations and makes the output distribution progressively more approximable by classical means. Since 2022, increasingly sophisticated classical attacks — tensor-network simulations exploiting loss, and samplers that match low-order photon-number correlations — have reproduced or exceeded the statistical benchmarks used to validate Borealis-class experiments at a fraction of the originally estimated cost [2]. As of the mid-2020s the honest summary is: Borealis's *exact* sampling task remains classically intractable, but the *noisy* distribution the hardware actually samples from is under active classical siege, and the advantage margin for lossy GBS has narrowed substantially. The lesson generalizes: **loss is not just an engineering nuisance in photonic quantum computing; it is the specific quantity that classical adversaries monetize.**

A second caveat tempers the celebration: GBS is a sampling benchmark, not a computation with a useful answer. Section 21.2.3 examines the proposed applications (graph problems, molecular spectra) and their contested practicality.

## Beyond Borealis: Aurora and the Fault-Tolerant Roadmap

Borealis was a milestone, not the destination. Xanadu's architecture for a *fault-tolerant* machine, published as a blueprint in 2021 [3], combines everything this chapter has developed:

- **GKP state factories**: nanophotonic chips generate GKP qubits probabilistically by photon-number-resolved heralding on squeezed light, multiplexed until success looks deterministic;
- **CV cluster-state fabric**: GKP qubits are entangled into a 3D cluster state using static beam splitters and short delay lines (the Borealis trick again);
- **Homodyne readout**: fault-tolerant computation proceeds by measurement, with analog syndrome information feeding the decoder;
- **Modularity**: chips are networked by optical fiber, so scaling means adding modules, not growing a monolith.

In January 2025 Xanadu reported *Aurora*, the first full-stack demonstration of this modular approach: 35 photonic chips linked by 13 km of fiber, 84 squeezers, and 36 photon-number-resolving detector channels, operating as a networked machine that synthesizes cluster states and performs real-time error-corrected operations — at small scale and with component qualities (especially loss and squeezing) still far from threshold, but exercising every architectural element end to end [4]. The declared gap to fault tolerance is dominated by exactly the parameters this chapter flagged: optical loss per component and the quality of GKP state generation.

## Why This Architecture Is "Very CV"

It is worth pausing on how each CV theme from Section 21.1 reappears as an engineering choice: deterministic squeezed sources replace heralded single photons (21.1.1); all gates before detection are Gaussian, hence loss-tolerant, simple, and room-temperature (21.1.2); the mandatory non-Gaussianity is pushed entirely into photon-number-resolving detection and offline GKP state preparation (21.1.2–21.1.3); and continuous homodyne outcomes supply analog error-correction information (21.1.3). The bet is that concentrating all the difficulty into one place — non-Gaussian state factories — is easier to industrialize than distributing probabilistic two-photon gates throughout the machine, as DV architectures must.

## Summary

- Borealis encodes 216 modes in a *time* sequence of squeezed pulses; three fiber loops ($\tau$, $6\tau$, $36\tau$) with programmable beam splitters provide three-range couplings — one source, three gates, full programmability.
- Readout uses 16 TES photon-number-resolving channels; the task is Gaussian boson sampling (hafnian sampling).
- Reported advantage (2022): $36\ \mu$s per sample versus an estimated $9{,}000$ years classically; later loss-exploiting classical algorithms have substantially narrowed the noisy-case gap.
- Xanadu's fault-tolerant blueprint: multiplexed GKP factories + CV cluster fabric + homodyne decoding; the 2025 Aurora system demonstrated the modular stack (35 chips, 13 km fiber) below threshold quality.
- Loss is the strategic parameter: it caps squeezing, feeds classical spoofing, and dominates the distance to fault tolerance.

---

*References*

[1] Madsen, L.S., Laudenbach, F., Askarani, M.F., Rortais, F., Vincent, T., Bulmer, J.F.F., Miatto, F.M., Neuhaus, L., Helt, L.G., Collins, M.J., Lita, A.E., Gerrits, T., Nam, S.W., Vaidya, V.D., Menotti, M., Dhand, I., Vernon, Z., Quesada, N., & Lavoie, J. (2022). Quantum computational advantage with a programmable photonic processor. *Nature*, 606, 75–81. [DOI: 10.1038/s41586-022-04725-x]

[2] Oh, C., Liu, M., Alexeev, Y., Fefferman, B., & Jiang, L. (2024). Classical algorithm for simulating experimental Gaussian boson sampling. *Nature Physics*, 20, 1461–1468. [DOI: 10.1038/s41567-024-02535-8] [Representative of the loss-exploiting classical-spoofing literature.]

[3] Bourassa, J.E., Alexander, R.N., Vasmer, M., Patil, A., Tzitrin, I., Matsuura, T., Su, D., Baragiola, B.Q., Guha, S., Dauphinais, G., Sabapathy, K.K., Menicucci, N.C., & Dhand, I. (2021). Blueprint for a scalable photonic fault-tolerant quantum computer. *Quantum*, 5, 392. [DOI: 10.22331/q-2021-02-04-392]

[4] Aghaee Rad, H., et al. (2025). Scaling and networking a modular photonic quantum computer. *Nature*, 638, 912–919. [The Aurora demonstration.]

[5] Arrazola, J.M., et al. (2021). Quantum circuits with many photons on a programmable nanophotonic chip. *Nature*, 591, 54–60. [DOI: 10.1038/s41586-021-03202-1] [Xanadu's earlier X8 chip: 8 modes, integrated squeezers.]

[6] Zhong, H.-S., et al. (2020). Quantum computational advantage using photons. *Science*, 370(6523), 1460–1463. [DOI: 10.1126/science.abe8770] [Jiuzhang, the fixed-interferometer GBS predecessor.]
