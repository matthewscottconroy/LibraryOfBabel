# 21.1.3 — GKP Encoding: A Qubit in an Oscillator

## The Problem GKP Solves

CV systems offer deterministic entanglement but suffer *continuous* errors: loss, thermal noise, and finite squeezing all act as random displacements in phase space, and a displaced quadrature value is not obviously "wrong" — there is no discrete alphabet to snap back to. Qubits, conversely, have a crisp error-correction theory but demand non-Gaussian physics to realize in optics. In 2001, Gottesman, Kitaev, and Preskill (GKP) fused the two: encode a *qubit* in a single oscillator so that the dominant *continuous* errors become detectable and correctable *discrete* syndromes [1]. Two decades later, the GKP code is the centerpiece of every serious blueprint for fault-tolerant photonic quantum computing, including Xanadu's (Section 21.2).

## The Ideal Code: Grid States

The ideal GKP codewords are combs of position eigenstates on a lattice of spacing $2\sqrt{\pi}$:

$$|0_L\rangle \propto \sum_{s\,\in\,\mathbb{Z}} |x = 2s\sqrt{\pi}\rangle, \qquad |1_L\rangle \propto \sum_{s\,\in\,\mathbb{Z}} |x = (2s+1)\sqrt{\pi}\rangle$$

$|0_L\rangle$ lives on even multiples of $\sqrt{\pi}$, $|1_L\rangle$ on odd multiples. By the Poisson summation formula, both are *also* combs in momentum with spacing $\sqrt{\pi}$: the superpositions $|\pm_L\rangle = (|0_L\rangle \pm |1_L\rangle)/\sqrt{2}$ are combs at even/odd multiples of $\sqrt{\pi}$ in $p$. The code is defined by its *stabilizers* — the two commuting displacement operators

$$\hat{S}_X = e^{-2i\sqrt{\pi}\,\hat{p}} \;(\text{shift } x \text{ by } 2\sqrt{\pi}), \qquad \hat{S}_Z = e^{2i\sqrt{\pi}\,\hat{x}} \;(\text{shift } p \text{ by } 2\sqrt{\pi})$$

(they commute because the accumulated phase $e^{i(2\sqrt{\pi})^2}=e^{4\pi i}=1$), and codewords are the simultaneous $+1$ eigenstates. The logical Paulis are the *half-lattice* displacements:

$$\hat{X}_L = e^{-i\sqrt{\pi}\,\hat{p}}, \qquad \hat{Z}_L = e^{i\sqrt{\pi}\,\hat{x}}$$

A shift of $x$ by $\sqrt{\pi}$ maps $|0_L\rangle \leftrightarrow |1_L\rangle$; a shift of $p$ by $\sqrt{\pi}$ flips the phase. The code is thus tailored against the physically dominant error: *any* displacement error $e^{-iu\hat{p}}$ with $|u| < \sqrt{\pi}/2$ is correctable, because measuring $\hat{x} \bmod \sqrt{\pi}$ reveals $u$ without revealing (or disturbing) the logical information, which is stored in *which* lattice class the state occupies. Round the measured offset to the nearest lattice point and displace back. Photon loss, the dominant optical error, can be converted into exactly such Gaussian displacement noise (by preceded amplification, or absorbed into the noise model), so GKP protects against loss as well.

Error correction is performed Steane-style with Gaussian resources only: couple the data mode to a fresh ancilla GKP state with a SUM gate ($e^{-i\hat{x}_1\hat{p}_2}$, a Gaussian operation), homodyne the ancilla, compute the offset modulo $\sqrt{\pi}$, and apply a corrective displacement. The syndrome is a *continuous* number — the analog offset — and using its full value (rather than just the rounded bit) measurably improves decoding, a uniquely CV bonus known as *analog syndrome information* [5].

A further gift: **all logical Clifford gates on GKP qubits are Gaussian operations.** $\bar{H}$ is a $90°$ phase-space rotation, $\bar{S}$ a shear, $\overline{\text{CNOT}}$ a SUM gate, logical Paulis are displacements, and logical measurement is homodyne detection. The entire hard-to-engineer non-Gaussianity of a GKP architecture is concentrated in *state preparation*; once high-quality GKP states exist, even universality can be completed with Gaussian operations alone, since Gaussian measurements on GKP codewords distill magic states (Baragiola et al., 2019 [6]).

## Finite Energy: Squeezing Is the Currency

Ideal grid states are sums of infinitely squeezed position eigenstates — unnormalizable, infinite energy. Physical GKP states replace each delta spike by a Gaussian of width $\Delta$ and impose an overall Gaussian envelope of width $1/\Delta$ (formally $|\tilde{\psi}\rangle \propto e^{-\Delta^2\hat{n}}|\psi_{\text{ideal}}\rangle$). The state is then characterized by its *GKP squeezing*: the spike variance expressed in decibels below vacuum,

$$s_{\text{dB}} = -10\log_{10}\!\left(\frac{\sigma_{\text{spike}}^2}{\sigma_{\text{vac}}^2}\right)$$

Finite $\Delta$ has two costs: the spikes themselves carry displacement uncertainty (intrinsic noise added to every error-correction round), and the envelope's finite extent causes rare misidentification of the lattice class. Both shrink exponentially with squeezing.

**Worked example.** Suppose each error-correction round subjects the $x$ quadrature to net Gaussian displacement noise of standard deviation $\sigma$ (combining channel noise and the finite spike width). A logical $X$ error occurs when the total offset exceeds the decision boundary $\sqrt{\pi}/2$ and is rounded to the wrong lattice class:

$$p_X \approx \text{erfc}\!\left(\frac{\sqrt{\pi}}{2\sqrt{2}\,\sigma}\right)$$

For noise 10 dB below vacuum, $\sigma^2 = 0.1 \times \tfrac{1}{2} = 0.05$, giving $p_X \approx \text{erfc}(2.80) \approx 7\times 10^{-5}$ per round per quadrature. At 6 dB ($\sigma^2 = 0.125$), $p_X \approx \text{erfc}(1.77) \approx 1.2\times 10^{-2}$ — over two orders of magnitude worse. The exponential sensitivity of the error rate to squeezing (roughly $p \sim e^{-\pi/(4\sigma^2)}$) is why every extra decibel matters, and why the 15 dB squeezing record (Section 21.1.1) is strategically significant.

How much squeezing does full fault tolerance require? Menicucci's 2014 analysis of CV cluster-state computing with GKP qubits established that a threshold *exists* and placed it at 20.5 dB under conservative assumptions [4]. Subsequent decoders exploiting analog syndrome information, concatenation with topological codes, and architectural optimizations have pushed theoretical estimates into the roughly 10 dB range [5, 7] — demanding, but no longer absurd. For comparison: 15 dB has been detected in bulk optics; optical GKP states themselves are far harder, because they require non-Gaussian preparation, not just squeezing.

## Where GKP States Exist Today

- **Trapped ions (2019).** Flühmann et al. encoded GKP states in the motional oscillator of a trapped ion, with stabilizer measurements via the internal qubit [2].
- **Microwave cavities (2020–2023).** Campagne-Ibarcq et al. created and error-corrected GKP states in a superconducting cavity coupled to a transmon [3]; Sivak et al. then operated a GKP logical qubit *beyond break-even* — the corrected logical qubit outlived the best physical qubit in the system by more than a factor of two — the first error-corrected qubit on any platform to clear that bar [8].
- **Propagating light (2024).** Konno et al. (Furusawa group) reported the first GKP-like logical states of *propagating* optical light, synthesized by photon-number-resolved heralding on squeezed light [9] — the form factor a photonic quantum computer actually needs, though qualities remain below fault-tolerance thresholds.

The gap between "GKP exists in matter" and "high-squeezing optical GKP on demand" is, arguably, *the* critical path for CV photonic quantum computing. Xanadu's architecture (next section) is engineered around exactly this bottleneck: probabilistic GKP preparation, multiplexed until it looks deterministic, then stitched into cluster states with Gaussian optics.

## Summary

- GKP encodes a qubit as a phase-space grid: $|0_L\rangle$/$|1_L\rangle$ are position combs on even/odd multiples of $\sqrt{\pi}$; stabilizers are lattice displacements.
- Displacements smaller than $\sqrt{\pi}/2$ are corrected by measuring quadratures modulo $\sqrt{\pi}$ — continuous noise becomes discrete, correctable syndromes with bonus analog information.
- All logical Cliffords are Gaussian; the non-Gaussian burden is concentrated in GKP state preparation.
- Finite squeezing sets logical error rates ($\sim 7\times10^{-5}$ per round at 10 dB in the worked example); fault-tolerance estimates demand squeezing of order 10 dB (early bound: 20.5 dB).
- Demonstrated in ion motion (2019), microwave cavities (2020, beyond break-even 2023), and propagating light (2024); high-quality optical GKP generation is the field's central open engineering problem.

---

*References*

[1] Gottesman, D., Kitaev, A., & Preskill, J. (2001). Encoding a qubit in an oscillator. *Physical Review A*, 64(1), 012310. [DOI: 10.1103/PhysRevA.64.012310]

[2] Flühmann, C., Nguyen, T.L., Marinelli, M., Negnevitsky, V., Mehta, K., & Home, J.P. (2019). Encoding a qubit in a trapped-ion mechanical oscillator. *Nature*, 566, 513–517. [DOI: 10.1038/s41586-019-0960-6]

[3] Campagne-Ibarcq, P., et al. (2020). Quantum error correction of a qubit encoded in grid states of an oscillator. *Nature*, 584, 368–372. [DOI: 10.1038/s41586-020-2603-3]

[4] Menicucci, N.C. (2014). Fault-tolerant measurement-based quantum computing with continuous-variable cluster states. *Physical Review Letters*, 112(12), 120504. [DOI: 10.1103/PhysRevLett.112.120504]

[5] Fukui, K., Tomita, A., Okamoto, A., & Fujii, K. (2018). High-threshold fault-tolerant quantum computation with analog quantum error correction. *Physical Review X*, 8(2), 021054. [DOI: 10.1103/PhysRevX.8.021054]

[6] Baragiola, B.Q., Pantaleoni, G., Alexander, R.N., Karanjai, A., & Menicucci, N.C. (2019). All-Gaussian universality and fault tolerance with the Gottesman-Kitaev-Preskill code. *Physical Review Letters*, 123(20), 200502. [DOI: 10.1103/PhysRevLett.123.200502]

[7] Bourassa, J.E., et al. (2021). Blueprint for a scalable photonic fault-tolerant quantum computer. *Quantum*, 5, 392. [DOI: 10.22331/q-2021-02-04-392] [Xanadu's GKP-based architecture paper.]

[8] Sivak, V.V., et al. (2023). Real-time quantum error correction beyond break-even. *Nature*, 616, 50–55. [DOI: 10.1038/s41586-023-05782-6]

[9] Konno, S., et al. (2024). Logical states for fault-tolerant quantum computation with propagating light. *Science*, 383(6680), 289–293. [DOI: 10.1126/science.adk7560]
