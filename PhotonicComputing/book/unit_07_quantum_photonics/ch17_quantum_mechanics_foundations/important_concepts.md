# Important Concepts — Chapter 17: Quantum Mechanics Foundations

## The Postulates in One Table

| Postulate | Statement | Photonic realization |
|---|---|---|
| State | Unit vector $|\psi\rangle \in \mathcal{H}$; mixed states $\rho = \sum_i p_i|\psi_i\rangle\langle\psi_i|$ | Photon polarization/path/time-bin qubit |
| Observable | Hermitian $\hat{A} = \sum_n a_n|a_n\rangle\langle a_n|$ | Pauli operators = Stokes parameters |
| Measurement | $P(a_n) = |\langle a_n|\psi\rangle|^2$; collapse to $|a_n\rangle$ | Wave plates + PBS + single-photon detectors |
| Evolution | $|\psi(t)\rangle = e^{-i\hat{H}t/\hbar}|\psi(0)\rangle$, unitary | Lossless linear optics: phase shifters, BS, MZI meshes |
| Composition | $\mathcal{H}_{AB} = \mathcal{H}_A \otimes \mathcal{H}_B$ | Photon pairs; multimode Fock space |

## Density Matrices

$\rho$ Hermitian, positive, $\mathrm{Tr}\,\rho = 1$. Purity $\mathrm{Tr}(\rho^2) \leq 1$, equality iff pure. Qubit form: $\rho = (\mathbb{1} + \mathbf{r}\cdot\hat{\boldsymbol{\sigma}})/2$, $|\mathbf{r}| \leq 1$ (Bloch = Poincaré sphere). Coherences (off-diagonal elements) distinguish superposition from mixture; they are what decoherence destroys. Reduced state: $\rho_A = \mathrm{Tr}_B\,\rho_{AB}$.

## Uncertainty and No-Cloning

Robertson: $\sigma_A\sigma_B \geq \frac{1}{2}|\langle[\hat{A},\hat{B}]\rangle|$. Quadratures: $[\hat{X}_1, \hat{X}_2] = i/2 \Rightarrow \Delta X_1 \Delta X_2 \geq 1/4$; vacuum saturates symmetrically ($\Delta X = 1/2$).

**No-cloning**: no unitary copies unknown states; $\langle\phi|\psi\rangle = \langle\phi|\psi\rangle^2$ forces overlap 0 or 1. Consequences: no quantum amplifiers/repeater-by-amplification; QKD security; Helstrom bound $P_{\text{err}} = \frac{1}{2}(1 - \sqrt{1 - |\langle\phi|\psi\rangle|^2})$ for single-copy discrimination.

## The Harmonic Oscillator Algebra (the engine of quantum optics)

$$[\hat{a}, \hat{a}^\dagger] = 1, \qquad \hat{H} = \hbar\omega(\hat{n} + \tfrac{1}{2}), \qquad \hat{a}|n\rangle = \sqrt{n}|n{-}1\rangle, \qquad \hat{a}^\dagger|n\rangle = \sqrt{n{+}1}|n{+}1\rangle$$

$|n\rangle = (\hat{a}^\dagger)^n|0\rangle/\sqrt{n!}$; zero-point energy $\hbar\omega/2$ (uncertainty-enforced); Heisenberg evolution $\hat{a}(t) = \hat{a}e^{-i\omega t}$ = quantum phasor. Field quantization: one oscillator per mode; $\mathcal{E}_k = \sqrt{\hbar\omega_k/2\varepsilon_0 V}$ = field per photon (85 kV/m for 1 μm³ at 1550 nm). Photon = excitation of a mode; indistinguishability = mode overlap.

## The Four State Families

| State | Definition | $\langle\hat{n}\rangle$ | Number statistics | Phase | Wigner |
|---|---|---|---|---|---|
| Vacuum $|0\rangle$ | $\hat{a}|0\rangle = 0$ | 0 | — | random | Gaussian, positive |
| Fock $|n\rangle$ | $\hat{n}$ eigenstate | $n$ | $\Delta n = 0$ ($Q = -1$) | fully random | negative rings ($W(0,0) = \frac{2}{\pi}(-1)^n$) |
| Coherent $|\alpha\rangle$ | $\hat{a}$ eigenstate; $\hat{D}(\alpha)|0\rangle$ | $|\alpha|^2$ | Poisson ($Q = 0$), shot noise | $\Delta\phi \approx 1/2|\alpha|$ | displaced vacuum Gaussian |
| Squeezed $\hat{S}(\xi)|0\rangle$ | $\Delta X_1 = e^{-r}/2$ | $\sinh^2 r$ | even pairs only | — | elliptical Gaussian |

Key formulas: $|\alpha\rangle = e^{-|\alpha|^2/2}\sum_n \frac{\alpha^n}{\sqrt{n!}}|n\rangle$; $|\langle\beta|\alpha\rangle|^2 = e^{-|\alpha-\beta|^2}$ (non-orthogonal); squeezing dB $= 8.686\,r$; loss $\eta$: $\Delta X^2_{\text{out}} = \eta e^{-2r}/4 + (1-\eta)/4$ (squeezing is loss-fragile). Two-mode squeezing $\to \sum_n \tanh^n r\,|n,n\rangle/\cosh r$: photon-number-correlated, thermal marginals, EPR correlations.

## Wigner Function

Quasi-probability on $(X_1, X_2)$; correct marginals for every quadrature (homodyne tomography); can be negative, $|W| \leq 2/\pi$. Parity formula: $W(0,0) = \frac{2}{\pi}\langle(-1)^{\hat{n}}\rangle$. Hudson: only Gaussian pure states are non-negative. Mari-Eisert: positive-Wigner circuits are classically simulable $\Rightarrow$ quantum advantage requires negativity (single photons, photon counting, or non-Gaussian states). Hierarchy: $P$ (nonclassicality) $\supset$ $W$ (simulability boundary) $\supset$ $Q$ (always positive).

## Entanglement

Product vs entangled: $|\psi\rangle_{AB} \neq |\phi\rangle_A|\chi\rangle_B$. Schmidt: $|\psi\rangle = \sum_k \lambda_k|u_k\rangle|v_k\rangle$; rank 1 = separable. Bell basis:

$$|\Phi^\pm\rangle = \frac{|00\rangle \pm |11\rangle}{\sqrt{2}}, \qquad |\Psi^\pm\rangle = \frac{|01\rangle \pm |10\rangle}{\sqrt{2}}$$

Maximally entangled: $\rho_A = \mathbb{1}/2$ (locally unpolarized), 1 ebit each, interconverted by local Paulis. Correlated in *all* bases ($E = \cos 2\Delta\theta$ for $|\Phi^+\rangle$). Linear-optics Bell measurement distinguishes only $|\Psi^\pm\rangle$: max 50% success — the recurring toll of photonic architectures.

**CHSH**: LHV $\Rightarrow |S| \leq 2$; quantum max $2\sqrt{2}$ (Tsirelson) at angles $(0°, 45°; 22.5°, 67.5°)$; loophole-free violations 2015 (Delft/Vienna/NIST); Nobel 2022 (Clauser, Aspect, Zeilinger). Violation certifies: entanglement, device-independent security, genuine randomness.

**Measures**: entanglement entropy $E = S(\rho_A)$ (pure states; = Shannon entropy of $\lambda_k^2$); concurrence $C$ (two qubits, computable); negativity/PPT (Peres-Horodecki); Werner state $p|\Psi^-\rangle\langle\Psi^-| + (1-p)\mathbb{1}/4$: entangled iff $p > 1/3$, CHSH-violating iff $p > 1/\sqrt{2}$ — entanglement $\neq$ nonlocality. Monogamy: $C^2_{A|B} + C^2_{A|C} \leq C^2_{A|BC}$.

## Key Numbers

| Quantity | Value |
|----------|-------|
| Photon energy at 1550 nm | $1.28\times 10^{-19}$ J $= 0.80$ eV |
| Thermal occupation of 1550 nm mode at 300 K | $\sim 3\times 10^{-14}$ (why optics works uncooled) |
| Vacuum quadrature noise | $\Delta X_1 = \Delta X_2 = 1/2$; product $= 1/4$ |
| Field per photon, $V = 1\ \mu\text{m}^3$, 1550 nm | $\approx 8.5\times 10^4$ V/m |
| Squeezing conversion | dB $\approx 8.686\,r$; 15 dB $\Rightarrow r \approx 1.73$, $\langle\hat{n}\rangle \approx 7.4$ |
| Squeezing record (Vahlbruch et al. 2016) | 15 dB at 1064 nm |
| Photons/s in 1 mW at 1550 nm | $7.8\times 10^{15}$ |
| CHSH: classical / quantum / max | $2$ / $2\sqrt{2} \approx 2.83$ / Tsirelson bound |
| Wigner extrema | $\pm 2/\pi$; $W_{|1\rangle}(0,0) = -2/\pi$ |
| Linear-optics Bell measurement success | $\leq 50\%$ |
| CHSH detection loophole threshold | $\eta \gtrsim 83\%$ (maximally entangled states) |
