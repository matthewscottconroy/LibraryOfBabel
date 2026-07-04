# 17.3.4 The Wigner Function

## Seeing Quantum States in Phase Space

Classical statistical optics describes a noisy oscillator by a probability density over phase space: how likely is the amplitude to be found at $(X_1, X_2)$? Quantum mechanics forbids a joint probability for two incompatible observables — $\hat{X}_1$ and $\hat{X}_2$ cannot simultaneously have values — yet it comes remarkably close to providing one. The **Wigner function** (Wigner, 1932), written here for a single mode in the quadrature convention of 17.3.1,

$$W(X_1, X_2) = \frac{2}{\pi}\int_{-\infty}^{\infty} \langle X_1 + y|\,\rho\,|X_1 - y\rangle\; e^{-4 i X_2 y}\, dy$$

(where $|X_1 \pm y\rangle$ are eigenstates of $\hat{X}_1$), is more transparently understood through its properties than its integral:

1. **Real and normalized**: $W$ is real, and $\iint W\, dX_1\, dX_2 = 1$.
2. **Correct marginals**: integrating out either quadrature yields the true measured distribution of the other, $\int W\, dX_2 = P(X_1)$ and $\int W\, dX_1 = P(X_2)$ — and more generally, integrating along any direction gives the distribution of the rotated quadrature $\hat{X}_\phi = \hat{X}_1\cos\phi + \hat{X}_2\sin\phi$, exactly what a homodyne detector at local-oscillator phase $\phi$ records.
3. **Not necessarily positive**: $W$ can dip below zero — the one concession that lets a "probability distribution" coexist with non-commuting observables. It is therefore a **quasi-probability distribution**, bounded by $|W| \leq 2/\pi$.

Property 2 makes the Wigner function measurable: record homodyne histograms at many phases $\phi$ and reconstruct $W$ by the inverse Radon transform — the same mathematics as a CT scan. This is **optical homodyne tomography** (first performed by Smithey, Beck, Raymer & Faridani, 1993), the standard method for characterizing CV states in Chapter 21 and squeezed sources in Chapter 18.

## A Gallery of States

| State | Wigner function | Character |
|---|---|---|
| Vacuum $|0\rangle$ | $\frac{2}{\pi}e^{-2(X_1^2 + X_2^2)}$ | round Gaussian of rms width $1/2$ at origin |
| Coherent $|\alpha\rangle$ | same Gaussian centered at $(\mathrm{Re}\,\alpha, \mathrm{Im}\,\alpha)$ | displaced vacuum; rotates rigidly at $\omega$ |
| Squeezed vacuum | Gaussian with widths $e^{-r}/2,\ e^{+r}/2$ | elliptical; area preserved |
| Thermal ($\bar{n}$) | round Gaussian, width $\sqrt{2\bar{n}+1}/2$ | broader than vacuum; positive |
| Fock $|1\rangle$ | $\frac{2}{\pi}\left[4(X_1^2 + X_2^2) - 1\right]e^{-2(X_1^2+X_2^2)}$ | **negative dip** at origin: $W(0,0) = -2/\pi$ |
| Cat state $\propto|\alpha\rangle + |{-\alpha}\rangle$ | two Gaussian lobes + oscillating interference fringes between them, alternating sign | negativity encodes superposition, not mixture |

The first four are Gaussian and non-negative; the last two are the genuinely quantum members. For the single-photon state, negativity at the origin is no small effect — it is the *most negative value any state can have*. A useful compact formula: the Wigner value at the origin measures mean photon-number **parity**, $W(0,0) = \frac{2}{\pi}\langle(-1)^{\hat{n}}\rangle$, immediately giving $+2/\pi$ for vacuum, $-2/\pi$ for $|1\rangle$, and $(2/\pi)(-1)^n$ for $|n\rangle$: every odd Fock state is maximally negative at the center.

**Worked example (why the mixture has no fringes).** Compare the cat state $|\psi\rangle \propto |\alpha\rangle + |{-\alpha}\rangle$ with the classical mixture $\rho = \frac{1}{2}(|\alpha\rangle\langle\alpha| + |{-\alpha}\rangle\langle{-\alpha}|)$, for $\alpha$ real and large. Both have two Gaussian lobes at $\pm\alpha$. The Wigner function is linear in $\rho$, so the mixture's $W$ is just the two positive lobes — everywhere non-negative. The pure superposition adds the cross terms $|\alpha\rangle\langle{-\alpha}| + \text{h.c.}$, which contribute an interference ridge $\propto \cos(4\alpha X_2)\, e^{-2(X_1^2+X_2^2)}$ midway between the lobes, oscillating negative-positive with period $\pi/2\alpha$. Coherence lives in the fringes; decoherence (loss) damps exactly those fringes at a rate $\propto |\alpha|^2$, turning superposition into mixture — the phase-space picture of why macroscopic cats die fast, and a preview of the error mechanics of cat-qubit encodings.

## Negativity as a Resource Boundary

The Wigner function turns "how quantum is this state?" into geometry, and the dividing line it draws is now known to be *computational*:

- **Hudson's theorem**: the only *pure* states with everywhere-non-negative Wigner functions are Gaussian states (vacuum, coherent, squeezed). Any other pure state — any Fock state, any cat — must go negative somewhere.
- **Classical simulability**: if the initial states, all operations, and all measurements of an optical experiment have non-negative Wigner representations, the entire experiment can be efficiently simulated classically by sampling phase-space trajectories (Mari & Eisert, 2012). Gaussian-only optics, however much squeezing and entanglement it contains, is not a quantum computer.
- Therefore every photonic quantum-advantage scheme must inject negativity from somewhere: **photon counting** on Gaussian states (Gaussian boson sampling, Chapter 20 — the detector's projectors $|n\rangle\langle n|$ carry the negativity), **single-photon inputs** (KLM, Chapter 20), or **non-Gaussian resource states** such as GKP grid states (Chapter 21, where Wigner negativity is literally the manufactured commodity).

The other quasi-probability representations complete the toolbox: the **Husimi $Q$ function** $Q(\alpha) = \langle\alpha|\rho|\alpha\rangle/\pi$ is always non-negative (a smoothed Wigner — what heterodyne detection samples) but hides non-classicality; the **Glauber-Sudarshan $P$ function** ($\rho = \int P(\alpha)|\alpha\rangle\langle\alpha|d^2\alpha$) is the most singular of the three, and its failure to be a legitimate probability density is the *definition* of non-classical light — a weaker condition than Wigner negativity (squeezed states have non-classical $P$ but positive $W$). One hierarchy, three magnifications: $P$ detects any non-classicality, $W$ detects the computationally potent kind, $Q$ always smiles politely.
