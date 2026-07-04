# 17.3.2 Coherent States

## The Quantum State of Laser Light

What state does an ideal laser emit? Chapter 4 described laser light classically: a stable amplitude and phase, $E(t) = E_0\cos(\omega t - \phi)$. Fock states cannot be the answer — they have no phase (17.2.3). The state that behaves as much like a classical stable wave as quantum mechanics permits is the **coherent state**, defined as the eigenstate of the annihilation operator:

$$\hat{a}\,|\alpha\rangle = \alpha\,|\alpha\rangle, \qquad \alpha = |\alpha|e^{i\theta} \in \mathbb{C}$$

Because $\hat{a}$ is not Hermitian, nothing forbids complex eigenvalues, and the eigenvalue $\alpha$ plays exactly the role of the classical complex amplitude: $\langle\alpha|\hat{E}|\alpha\rangle \propto |\alpha|\cos(\omega t - \theta)$ — a nonzero mean field oscillating classically, with $|\alpha|^2$ setting the intensity. Glauber established in 1963 that these states are the natural description of coherent optical fields (and of any field radiated by a classical current); the laser far above threshold approximates them well.

## Fock-Basis Expansion and Photon Statistics

Expand $|\alpha\rangle = \sum_n c_n|n\rangle$ and apply the eigenvalue equation: using $\hat{a}|n\rangle = \sqrt{n}|n-1\rangle$, the recursion $c_n\sqrt{n} = \alpha\, c_{n-1}$ gives $c_n = \alpha^n c_0/\sqrt{n!}$, and normalization fixes

$$|\alpha\rangle = e^{-|\alpha|^2/2}\,\sum_{n=0}^{\infty}\frac{\alpha^n}{\sqrt{n!}}\,|n\rangle$$

The photon-number distribution is **Poissonian**:

$$P(n) = |\langle n|\alpha\rangle|^2 = e^{-\bar{n}}\,\frac{\bar{n}^n}{n!}, \qquad \bar{n} = |\alpha|^2, \qquad \langle(\Delta n)^2\rangle = \bar{n}$$

Poisson statistics are exactly what independent, uncorrelated arrivals produce: coherent light delivers photons like ideal raindrops, with no memory. Mandel parameter $Q = 0$; $g^{(2)}(0) = 1$ (Section 18.1). The relative number fluctuation $\Delta n/\bar{n} = 1/\sqrt{\bar{n}}$ shrinks with brightness — the correspondence-principle route back to classical stability — but the absolute fluctuation $\sqrt{\bar{n}}$ *is* optical **shot noise**. When Unit V bounded the precision of analog photonic computing by shot noise, it was invoking this Poisson variance; the $1/\sqrt{N}$ standard quantum limit of interferometry (Section 18.3.3) is the same statistics again.

**Worked example (a milliwatt is a lot of photons).** A 1 mW laser at $\lambda = 1550$ nm emits $P/\hbar\omega = 10^{-3}/1.28\times10^{-19} \approx 7.8\times 10^{15}$ photons/s. In a detection window of 1 ns: $\bar{n} = 7.8\times 10^6$ photons, with shot-noise fluctuation $\sqrt{\bar{n}} \approx 2800$ — a relative noise of $3.6\times 10^{-4}$, i.e., a shot-noise-limited SNR of about 69 dB. The same laser attenuated to $\bar{n} = 0.1$ per pulse — the regime of decoy-state QKD (Chapter 22) — still emits *pairs* with probability $P(2) \approx \bar{n}^2 e^{-\bar{n}}/2 \approx 0.45\%$ of pulses: attenuation never turns Poisson light into single photons, which is why weak coherent pulses and true Fock states are cryptographically different resources.

## Displaced Vacuum and Minimum Uncertainty

The coherent state is the vacuum, moved. Define the unitary **displacement operator**

$$\hat{D}(\alpha) = \exp\left(\alpha\hat{a}^\dagger - \alpha^*\hat{a}\right), \qquad |\alpha\rangle = \hat{D}(\alpha)|0\rangle, \qquad \hat{D}^\dagger(\alpha)\,\hat{a}\,\hat{D}(\alpha) = \hat{a} + \alpha$$

Since displacement merely shifts the mode operators, all *fluctuations* are inherited unchanged from vacuum:

$$\langle\hat{X}_1\rangle = \mathrm{Re}\,\alpha, \quad \langle\hat{X}_2\rangle = \mathrm{Im}\,\alpha, \qquad \Delta X_1 = \Delta X_2 = \frac{1}{2}$$

— a minimum-uncertainty state ($\Delta X_1\Delta X_2 = 1/4$) with noise shared equally between quadratures. The phase-space picture: a circular fuzz-ball of diameter $\sim 1$ (the vacuum disk) centered at the classical phasor $\alpha$, rotating rigidly at $\omega$ under free evolution ($\alpha \to \alpha e^{-i\omega t}$: coherent states stay coherent, another sense in which they are the classical states). Phase uncertainty is the angle the disk subtends, $\Delta\phi \approx 1/(2|\alpha|)$, so $\Delta n\,\Delta\phi \approx 1/2$: bright coherent states have sharp phase and fuzzy number — the exact complement of Fock states.

## Overcompleteness

Coherent states are not orthogonal:

$$|\langle\beta|\alpha\rangle|^2 = e^{-|\alpha - \beta|^2}$$

Distinct coherent states overlap, negligibly when separated by many vacuum widths, substantially when close. Consequently they form not a basis but an **overcomplete** set, resolving the identity as $\frac{1}{\pi}\int |\alpha\rangle\langle\alpha|\,d^2\alpha = \mathbb{1}$. Two consequences matter later. First, the non-orthogonality of weak coherent pulses is a *feature* in QKD (states carrying key bits cannot be perfectly distinguished by an eavesdropper — 17.1.3's Helstrom bound) and a *nuisance* in coherent optical communication (it sets the minimum error rate of discriminating symbols at low photon number). Second, overcompleteness underwrites the phase-space representations of 17.3.4: the Glauber-Sudarshan $P$ function writes $\rho = \int P(\alpha)\,|\alpha\rangle\langle\alpha|\,d^2\alpha$, and "classical light" acquires its sharp definition — states whose $P(\alpha)$ is a legitimate probability density. Coherent states (a delta-function $P$) sit exactly on the classical/quantum boundary: the *most classical* pure states of light, yet still carrying irreducible vacuum noise.

## Coherent States as the Reference Frame of Quantum Photonics

Nearly every protocol in Chapters 18–22 is phrased relative to coherent states: squeezing is noise *below* the coherent-state level (17.3.3); non-classicality criteria ($g^{(2)} < 1$, $Q < 0$, Wigner negativity) are all violations of coherent-state benchmarks; homodyne detection beats a signal against a bright coherent **local oscillator**; CV quantum computing (Chapter 21) uses displacement as its cheapest gate. When a photonics engineer says "classical light," the precise translation is "a coherent state, possibly with added classical noise" — and everything genuinely quantum in this unit is defined by how it differs from that.
