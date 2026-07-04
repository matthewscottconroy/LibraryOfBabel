# Unit VII Problem Set: Quantum Photonics and Quantum Computing

*These problems span Chapters 17–22 of Unit VII, covering quantum mechanics foundations, quantum optics, single-photon devices, linear optical quantum computing, continuous-variable approaches, and quantum communication. Problems are labelled: [Easy], [Medium], or [Hard]. Hints are provided for Hard problems.*

---

## Chapter 17: Quantum Mechanics Foundations

**Problem 17.1** [Easy]
A single photon is prepared in the polarization state $|\psi\rangle = \cos\theta\,|H\rangle + e^{i\phi}\sin\theta\,|V\rangle$.

(a) Verify that $|\psi\rangle$ is normalized for all $\theta, \phi$.
(b) Compute the probability of detecting the photon behind a polarizer aligned to $|D\rangle = (|H\rangle + |V\rangle)/\sqrt{2}$.
(c) Write the density matrix $\rho = |\psi\rangle\langle\psi|$ for $\theta = \pi/4$, $\phi = \pi/2$, and compute $\mathrm{Tr}(\rho^2)$.
(d) Now consider completely unpolarized light: $\rho = \frac{1}{2}(|H\rangle\langle H| + |V\rangle\langle V|)$. Compute $\mathrm{Tr}(\rho^2)$ and explain the difference.

**Problem 17.2** [Easy]
Starting from the canonical commutator $[\hat{x}, \hat{p}] = i\hbar$ and the definitions of $\hat{a}$ and $\hat{a}^\dagger$, show that $[\hat{a}, \hat{a}^\dagger] = 1$. Then show that $[\hat{n}, \hat{a}] = -\hat{a}$ and $[\hat{n}, \hat{a}^\dagger] = +\hat{a}^\dagger$, and interpret these relations physically.

**Problem 17.3** [Medium]
A coherent state is defined by $\hat{a}|\alpha\rangle = \alpha|\alpha\rangle$.

(a) Derive the Fock-basis expansion $|\alpha\rangle = e^{-|\alpha|^2/2}\sum_n (\alpha^n/\sqrt{n!})|n\rangle$.
(b) Show that the photon number distribution is Poissonian with mean $\bar{n} = |\alpha|^2$ and variance $\langle(\Delta n)^2\rangle = |\alpha|^2$.
(c) Compute the Mandel parameter $Q = \langle(\Delta n)^2\rangle/\langle n\rangle - 1$ for a coherent state, a thermal state, and the Fock state $|n\rangle$.
(d) Compute the overlap $|\langle\alpha|\beta\rangle|^2$ and show that distinct coherent states are never exactly orthogonal.

**Problem 17.4** [Medium]
For the two-qubit state $|\psi(\theta)\rangle = \cos\theta\,|00\rangle + \sin\theta\,|11\rangle$:

(a) Compute the reduced density matrix $\rho_A = \mathrm{Tr}_B(|\psi\rangle\langle\psi|)$.
(b) Compute the entanglement entropy $E = -\mathrm{Tr}(\rho_A \log_2 \rho_A)$ as a function of $\theta$.
(c) For which $\theta$ is the state maximally entangled? Separable?
(d) Show that no local unitary $U_A \otimes U_B$ can change $E$.

**Problem 17.5** [Hard]
*Hint: For the singlet state, the correlation function is $E(\mathbf{a},\mathbf{b}) = -\mathbf{a}\cdot\mathbf{b} = -\cos(\theta_a - \theta_b)$ for measurement axes in a plane.*

Consider the CHSH quantity $S = E(a,b) - E(a,b') + E(a',b) + E(a',b')$ measured on the singlet state $|\Psi^-\rangle = (|01\rangle - |10\rangle)/\sqrt{2}$.

(a) Show that for any local hidden-variable theory, $|S| \leq 2$.
(b) Using the quantum prediction, evaluate $S$ for the angle settings $\theta_a = 0$, $\theta_{a'} = \pi/2$, $\theta_b = \pi/4$, $\theta_{b'} = 3\pi/4$.
(c) Prove the Tsirelson bound: $|S| \leq 2\sqrt{2}$ for any quantum state and any observables with eigenvalues $\pm 1$. (Consider the operator identity for $\hat{S}^2$.)
(d) Suppose each photon passes through a depolarizing channel that replaces the state with white noise with probability $p$. Find the threshold $p$ above which the CHSH inequality is no longer violated.

---

## Chapter 18: Quantum Optics

**Problem 18.1** [Easy]
Compute $g^{(2)}(0) = \langle \hat{a}^\dagger\hat{a}^\dagger\hat{a}\hat{a}\rangle / \langle \hat{a}^\dagger\hat{a}\rangle^2$ for:

(a) the coherent state $|\alpha\rangle$,
(b) the Fock states $|1\rangle$ and $|2\rangle$,
(c) a single-mode thermal state with mean photon number $\bar{n}$,
(d) the superposition $(|0\rangle + |2\rangle)/\sqrt{2}$. Classify each as bunched, Poissonian, or antibunched.

**Problem 18.2** [Medium]
A 50/50 beam splitter transforms input modes as $\hat{c} = (\hat{a} + i\hat{b})/\sqrt{2}$, $\hat{d} = (i\hat{a} + \hat{b})/\sqrt{2}$.

(a) Verify that the transformation preserves the commutation relations $[\hat{c},\hat{c}^\dagger] = [\hat{d},\hat{d}^\dagger] = 1$, $[\hat{c},\hat{d}^\dagger] = 0$.
(b) Show that the input $|1,1\rangle = \hat{a}^\dagger\hat{b}^\dagger|0,0\rangle$ produces the output $\frac{i}{\sqrt{2}}(|2,0\rangle + |0,2\rangle)$ — the Hong-Ou-Mandel effect.
(c) A single photon enters port $a$ with vacuum in port $b$. Write the output state and show that the coincidence rate between the two output detectors is zero.
(d) Two *distinguishable* photons (e.g., orthogonal polarizations) enter the two ports. Compute the coincidence probability and compare to (b).

**Problem 18.3** [Medium]
A degenerate parametric amplifier applies the squeezing operator $\hat{S}(r) = \exp[\frac{r}{2}(\hat{a}^2 - \hat{a}^{\dagger 2})]$ to vacuum.

(a) Using $\hat{S}^\dagger(r)\,\hat{a}\,\hat{S}(r) = \hat{a}\cosh r - \hat{a}^\dagger \sinh r$, compute the quadrature variances $\langle(\Delta \hat{X}_1)^2\rangle$ and $\langle(\Delta \hat{X}_2)^2\rangle$ of the squeezed vacuum, where $\hat{X}_1 = (\hat{a}+\hat{a}^\dagger)/2$ and $\hat{X}_2 = (\hat{a}-\hat{a}^\dagger)/2i$.
(b) What squeezing parameter $r$ corresponds to 15 dB of noise reduction? What is the mean photon number $\sinh^2 r$ of that state?
(c) The squeezed beam passes through a channel with transmission $\eta = 0.8$. Show that the detected variance is $\eta e^{-2r}/4 + (1-\eta)/4$ and compute the observed squeezing in dB for the 15 dB input.

**Problem 18.4** [Hard]
*Hint: Model each photon as a Gaussian wavepacket $\phi(t) \propto \exp(-t^2/4\sigma_t^2)$; the coincidence probability depends on the modulus of the temporal mode overlap.*

Two single photons with identical Gaussian spectra (coherence time $\sigma_t$) arrive at a 50/50 beam splitter with relative delay $\tau$.

(a) Show that the coincidence probability is $P_{cc}(\tau) = \frac{1}{2}\left[1 - e^{-\tau^2/2\sigma_t^2}\right]$ (up to the definition of $\sigma_t$; state your convention).
(b) Sketch the HOM dip and relate its width to the photon coherence time, not to the detector timing resolution. Why did this allow Hong, Ou, and Mandel to measure sub-picosecond intervals with nanosecond-scale detectors?
(c) Each source has a residual two-photon probability characterized by $g^{(2)}(0) = 0.04$. Using the approximation that multiphoton contamination reduces the raw dip visibility by $\approx 2g^{(2)}(0)$, estimate the maximum achievable HOM visibility.

---

## Chapter 19: Single-Photon Sources and Detectors

**Problem 19.1** [Easy]
A quantum dot in a micropillar cavity has quality factor $Q = 10{,}000$ and mode volume $V = 0.5\,(\lambda/n)^3$.

(a) Compute the Purcell factor $F_P = \frac{3}{4\pi^2}\left(\frac{\lambda}{n}\right)^3 \frac{Q}{V}$.
(b) If the bulk radiative lifetime is 1 ns, what is the cavity-enhanced lifetime?
(c) Compute the $\beta$-factor $\beta = F_P/(F_P + 1)$ and explain why a large $\beta$ improves both brightness and indistinguishability.

**Problem 19.2** [Medium]
An SNSPD has system detection efficiency $\eta = 0.93$, dark count rate $D = 10$ cps, and timing jitter 20 ps. A heralded SPDC source delivers signal photons at $10^5$ per second.

(a) Compute the detected click rate and the fraction of clicks due to dark counts within a 1 ns coincidence window.
(b) The source is pulsed at 80 MHz. What is the probability per pulse of a dark count in the window?
(c) For a two-photon coincidence experiment with two such detectors, compute the overall coincidence efficiency $\eta^2$ and the accidental-coincidence rate.

**Problem 19.3** [Hard]
*Hint: Work in the one-excitation subspace spanned by $|e, 0\rangle$ and $|g, 1\rangle$.*

For the resonant Jaynes-Cummings Hamiltonian $\hat{H} = \hbar\omega(\hat{a}^\dagger\hat{a} + \frac{1}{2}\hat{\sigma}_z) + \hbar g(\hat{a}^\dagger\hat{\sigma}_- + \hat{a}\hat{\sigma}_+)$:

(a) Diagonalize $\hat{H}$ in the one-excitation subspace and show the eigenstates are split by $2\hbar g$ (vacuum Rabi splitting).
(b) An atom starts in $|e, 0\rangle$. Compute the probability of finding it excited at time $t$.
(c) Strong coupling requires $g > \kappa/2, \gamma/2$. For a photonic crystal cavity with $Q = 30{,}000$ at $\lambda = 930$ nm and $g/2\pi = 10$ GHz, is the system strongly coupled? ($\kappa = \omega/Q$; take $\gamma$ negligible.)

---

## Chapter 20: Linear Optical Quantum Computing

**Problem 20.1** [Easy]
A dual-rail qubit encodes $|0\rangle_L = |1,0\rangle$ (photon in mode $a$) and $|1\rangle_L = |0,1\rangle$ (photon in mode $b$).

(a) Show that a phase shifter $e^{i\phi \hat{n}_b}$ on mode $b$ implements the rotation $\mathrm{diag}(1, e^{i\phi})$ — a $Z$-axis rotation.
(b) Show that a 50/50 beam splitter implements (up to phases) a Hadamard-like rotation on the logical basis.
(c) Explain why photon loss maps a dual-rail qubit *outside* the logical subspace, and why this makes loss a detectable erasure rather than an undetected bit-flip.

**Problem 20.2** [Medium]
In boson sampling, $n$ photons enter an $m$-mode interferometer described by a Haar-random unitary $U$; output probabilities are proportional to $|\mathrm{Perm}(A)|^2$ for $n \times n$ submatrices $A$ of $U$.

(a) Write out the permanent of a general $2\times 2$ and $3\times 3$ matrix, and count the terms for $n \times n$.
(b) The best classical algorithm (Ryser's) computes an $n \times n$ permanent in $O(n^2 2^n)$ operations. Estimate the operation count for $n = 30$ and $n = 76$ (Jiuzhang).
(c) Explain why photon loss and partial distinguishability both open the door to efficient classical spoofing.

**Problem 20.3** [Hard]
*Hint: The KLM nonlinear sign gate succeeds with probability 1/4; two of them make a CZ with probability 1/16, improved to 1/4 in optimized schemes. For teleported gates, consult the resource-state argument rather than computing explicit networks.*

A photonic circuit requires 10 CZ gates in sequence.

(a) If each CZ succeeds independently with probability $p = 1/16$ and failure destroys the computation, what is the end-to-end success probability?
(b) With KLM gate teleportation, each of the two teleportations succeeds with probability $n/(n+1)$ using an $n$-photon ancilla state, so the gate succeeds with probability $p = n^2/(n+1)^2$; failures are heralded and correctable rather than fatal. For $n = 10$ ancilla photons per teleportation, compute $p$ and the probability that all 10 gates succeed without any repair step.
(c) Estimate the ancilla photon consumption per logical gate and discuss why fusion-based schemes with small (4–6 photon) resource states won industrially over direct KLM.

---

## Chapter 21: Continuous-Variable Quantum Computing

**Problem 21.1** [Easy]
For quadratures $\hat{X}_1, \hat{X}_2$ with $[\hat{X}_1, \hat{X}_2] = i/2$:

(a) State the uncertainty relation and verify it is saturated by vacuum ($\Delta X_1 = \Delta X_2 = 1/2$).
(b) A two-mode squeezed vacuum with parameter $r$ has $\mathrm{Var}(\hat{X}_1^{(A)} - \hat{X}_1^{(B)}) = e^{-2r}/2$. In what limit does this reproduce the original EPR state?
(c) Explain why Gaussian states are fully described by first and second moments.

**Problem 21.2** [Medium]
A GKP qubit encodes $|0\rangle_L$ and $|1\rangle_L$ as comb-like superpositions of position eigenstates spaced by $2\sqrt{\pi}$ (in units with $\hbar = 1$ and $[\hat{x},\hat{p}] = i$).

(a) Explain how a small shift error $\epsilon < \sqrt{\pi}/2$ in $\hat{X}_1$ is detected and corrected by measuring $\hat{X}_1 \bmod \sqrt{\pi}$.
(b) Physical GKP states use finite squeezing. Qualitatively, how does finite squeezing translate into a logical error rate?
(c) Why is a supply of GKP states plus Gaussian operations and homodyne detection sufficient for fault-tolerant CV quantum computing, while Gaussian resources alone are classically simulable?

**Problem 21.3** [Medium]
Gaussian boson sampling sends $K$ single-mode squeezed vacua (squeezing $r$) into an $m$-mode interferometer.

(a) What is the mean total photon number as a function of $K$ and $r$?
(b) Output probabilities involve the *hafnian* rather than the permanent. What structural feature of squeezed (pair-correlated) input states does the hafnian encode?
(c) Borealis used $K = 216$ modes with time-domain multiplexing. Explain the engineering advantage of time-bin encoding over a spatial 216-mode interferometer.

---

## Chapter 22: Quantum Communication

**Problem 22.1** [Easy]
In BB84, Alice sends single photons in randomly chosen bases $\{H/V\}$ or $\{D/A\}$; Bob measures in a random basis.

(a) What fraction of transmitted bits survives basis sifting?
(b) An intercept-resend eavesdropper measures every photon in a random basis and resends her result. Show she induces a 25% error rate in the sifted key.
(c) If Alice and Bob observe QBER = 5%, can they distill a secure key? (The BB84 security threshold against optimal collective attacks is QBER ≈ 11%.)

**Problem 22.2** [Medium]
A fiber QKD link has attenuation 0.2 dB/km, source rate $10^9$ pulses/s with mean photon number $\mu = 0.5$ (decoy-state weak coherent pulses), and detector efficiency 20%.

(a) Compute the raw detection rate at 100 km and at 400 km.
(b) Explain why a conventional optical amplifier cannot be used to extend the link, referencing the no-cloning theorem.
(c) Estimate the improvement if SNSPDs with 90% efficiency replace the InGaAs detectors.

**Problem 22.3** [Hard]
*Hint: Entanglement swapping consumes one Bell pair per link per attempt; nested repeater protocols multiply success probabilities per nesting level.*

A quantum repeater chain divides a 1000 km link into 4 segments of 250 km.

(a) With fiber loss 0.2 dB/km, compute the photon survival probability over 1000 km directly, and over one 250 km segment.
(b) Each segment distributes entanglement heralded by a two-photon coincidence, succeeding with probability proportional to segment transmission; entanglement is stored in quantum memories until neighbors succeed. Explain why memory lifetime and Bell-state-measurement efficiency set the achievable rate.
(c) Entanglement swapping with linear optics succeeds with at most 50% probability per swap. For 3 swaps, what is the swapping overhead alone? Combine with (a) to argue why repeaters only win when memories allow *asynchronous* segment success.
