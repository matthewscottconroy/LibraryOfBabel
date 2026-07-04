# Exercises — Chapter 17: Quantum Mechanics Foundations for Photonics

## Mathematical Exercises

**17.1** (Dirac Notation Warm-up) A photon is prepared in $|\psi\rangle = \frac{1}{\sqrt{3}}|H\rangle + \sqrt{\frac{2}{3}}\,e^{i\pi/4}|V\rangle$.

(a) Verify normalization and write $\langle\psi|$.

(b) Compute the probability of transmission through polarizers along $|H\rangle$, $|V\rangle$, $|D\rangle$, and $|L\rangle = (|H\rangle + i|V\rangle)/\sqrt{2}$.

(c) Write the density matrix $\rho = |\psi\rangle\langle\psi|$ in the H/V basis and extract the Bloch vector $\mathbf{r} = (\langle\hat{\sigma}_x\rangle, \langle\hat{\sigma}_y\rangle, \langle\hat{\sigma}_z\rangle)$.

(d) The photon passes through a channel that randomizes the phase $e^{i\pi/4} \to e^{i\phi}$ with $\phi$ uniform in $[0, 2\pi)$. Write the resulting density matrix and its Bloch vector. What happened to the coherences?

---

**17.2** (Ladder Algebra) Starting from $[\hat{x}, \hat{p}] = i\hbar$ and the definitions of $\hat{a}, \hat{a}^\dagger$ in Section 17.2.2:

(a) Derive $[\hat{a}, \hat{a}^\dagger] = 1$, showing each step.

(b) Show $[\hat{n}, \hat{a}] = -\hat{a}$ and $[\hat{n}, \hat{a}^\dagger] = \hat{a}^\dagger$.

(c) Using only the algebra, prove $\hat{a}|n\rangle = \sqrt{n}|n-1\rangle$ (fix the normalization from $\|\hat{a}|n\rangle\|^2$).

(d) Compute $\langle n|\hat{x}^4|n\rangle$ for the ground state $n = 0$ and verify it equals $3(\hbar/2m\omega)^2$ (Gaussian statistics).

---

**17.3** (Coherent States) For the coherent state $|\alpha\rangle$:

(a) Derive the Fock expansion from the eigenvalue equation $\hat{a}|\alpha\rangle = \alpha|\alpha\rangle$.

(b) Show that the photon number distribution is Poissonian and compute the Mandel $Q$ parameter, confirming $Q = 0$.

(c) Show that $\Delta X_1 = \Delta X_2 = 1/2$ — a minimum-uncertainty state.

(d) A 100 μW beam at 1550 nm is measured for 1 μs. Compute $\bar{n}$, the shot-noise fluctuation $\sqrt{\bar{n}}$, and the shot-noise-limited SNR in dB.

(e) Compute $|\langle\alpha|-\alpha\rangle|^2$ for $\bar{n} = 1$ and $\bar{n} = 10$. At what $\bar{n}$ do opposite-phase coherent states become distinguishable at the $10^{-9}$ overlap level?

---

**17.4** (Squeezed Vacuum) Using $\hat{S}^\dagger(r)\hat{a}\hat{S}(r) = \hat{a}\cosh r - \hat{a}^\dagger\sinh r$:

(a) Derive $\Delta X_1 = e^{-r}/2$ and $\Delta X_2 = e^{+r}/2$ for the squeezed vacuum $\hat{S}(r)|0\rangle$, and verify the uncertainty product is preserved.

(b) Show $\langle\hat{n}\rangle = \sinh^2 r$.

(c) Compute $r$, $\langle\hat{n}\rangle$, and the anti-squeezed variance for 3 dB, 10 dB, and 15 dB of squeezing.

(d) The 15 dB state passes through a channel with 10% loss. Compute the output squeezing in dB. What loss reduces it to 3 dB?

---

**17.5** (Wigner Negativity) Using the parity formula $W(0,0) = \frac{2}{\pi}\langle(-1)^{\hat{n}}\rangle$:

(a) Evaluate $W(0,0)$ for vacuum, $|1\rangle$, $|2\rangle$, a coherent state $|\alpha\rangle$, and a thermal state with mean $\bar{n}$.

(b) A single photon suffers loss $1 - \eta$ (state becomes $\eta|1\rangle\langle 1| + (1-\eta)|0\rangle\langle 0|$). For what $\eta$ does $W(0,0)$ reach zero? Interpret for single-photon experiments.

(c) Show that the marginal $\int W\,dX_2$ for $|1\rangle$ is the correct homodyne distribution $P(X_1) \propto X_1^2 e^{-2X_1^2}$.

---

**17.6** (Tensor Products and Partial Trace)

(a) Show that $|\Phi^+\rangle = (|00\rangle + |11\rangle)/\sqrt{2}$ cannot be written as a product state.

(b) Compute $\rho_A = \mathrm{Tr}_B(|\Phi^+\rangle\langle\Phi^+|)$ and its purity.

(c) Verify that the four Bell states form an orthonormal basis of the two-qubit space.

(d) Show that $(\mathbb{1}\otimes\hat{\sigma}_x)|\Phi^+\rangle = |\Psi^+\rangle$ and find the local Paulis generating the other Bell states from $|\Phi^+\rangle$.

---

**17.7** (CHSH) For the state $|\Phi^+\rangle$ with linear-polarization analyzers, $E(\theta_1, \theta_2) = \cos 2(\theta_1 - \theta_2)$.

(a) Verify the quantum value $S = 2\sqrt{2}$ at the optimal angles $(0°, 45°; 22.5°, 67.5°)$.

(b) Prove the classical bound $|S| \leq 2$ for deterministic local response functions.

(c) With imperfect interference visibility $V$, the correlation degrades to $E = V\cos 2(\theta_1 - \theta_2)$. What minimum $V$ still violates CHSH?

(d) Show that the Werner state $\rho_W = p|\Psi^-\rangle\langle\Psi^-| + (1-p)\mathbb{1}/4$ violates CHSH iff $p > 1/\sqrt{2}$.

---

**17.8** (Correlations Are Not Enough) Consider the classical mixture $\rho_{cl} = \frac{1}{2}(|HH\rangle\langle HH| + |VV\rangle\langle VV|)$ and the Bell state $|\Phi^+\rangle$.

(a) Show both give identical, perfectly correlated statistics in the H/V basis.

(b) Compute the joint statistics in the D/A basis for both, and show the mixture's correlation drops to zero while the Bell state's remains perfect.

(c) Compute the concurrence of both states.

---

## Conceptual Exercises

**17.9** (No-Cloning and Amplifiers) An engineer proposes extending a photonic qubit link with an erbium-doped fiber amplifier, arguing that "gain $G = 100$ will more than compensate the loss."

(a) Using the linearity argument of Section 17.1.3, explain why no device can clone the qubit stream.

(b) Reconcile: EDFAs demonstrably work for classical coherent communication. What is different about the information encoding?

(c) What does a phase-insensitive amplifier necessarily add, and how does this enforce the no-cloning theorem quantitatively?

**17.10** (Fock versus Coherent) A colleague claims: "A laser attenuated to $\bar{n} = 0.01$ photons per pulse is a single-photon source — after all, pulses almost never contain two photons."

(a) Compute $P(1)$ and $P(2|{\geq}1)$ (probability a non-empty pulse has $\geq 2$ photons) for $\bar{n} = 0.01$.

(b) Explain why $g^{(2)}(0)$ for this source is 1, not 0, and why that matters for QKD security against photon-number-splitting attacks.

(c) What physical resources produce genuine $g^{(2)}(0) \approx 0$ light?

**17.11** (Measurement Bases as a Resource) BB84 encodes bits in randomly alternating H/V and D/A bases. Using the incompatibility $[\hat{\sigma}_z, \hat{\sigma}_x] \neq 0$ and the collapse postulate, explain (i) why an intercept-resend eavesdropper necessarily introduces errors, and (ii) why the protocol would be insecure if only one basis were used.

**17.12** (Where Is the Boundary?) Section 17.3.4 stated that experiments describable by non-negative Wigner functions are efficiently classically simulable. For each system, state whether it can exhibit quantum advantage and why: (i) squeezed states + linear optics + homodyne detection; (ii) squeezed states + linear optics + photon counting; (iii) coherent states + linear optics + photon counting; (iv) single photons + linear optics + photon counting.
