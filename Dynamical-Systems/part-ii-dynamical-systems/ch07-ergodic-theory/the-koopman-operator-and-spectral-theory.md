# 7.6 The Koopman Operator and Spectral Theory

One of the most productive moves in ergodic theory is to replace the nonlinear dynamical system $(X, f)$ with the linear operator $U_f$ that it induces on functions. This turns the study of orbits into functional analysis, and functional analysis is powerful.

**Definition 7.6.1.** The *Koopman operator* of $(X, \mathcal{B}, \mu, f)$ is $U_f: L^2(\mu) \to L^2(\mu)$, $U_f \varphi = \varphi \circ f$.

For an invertible MPT, $U_f$ is unitary: $U_f^* = U_f^{-1} = U_{f^{-1}}$.

Why unitary? Because $\|U_f \varphi\|^2 = \int |\varphi \circ f|^2\,d\mu = \int |\varphi|^2\,d\mu = \|\varphi\|^2$ by measure-preservation. Measure-preservation is exactly the condition that makes $U_f$ an isometry, and invertibility makes it unitary. The whole spectral theory of unitary operators — which is rich and well-understood — becomes available.

The key insight, which Bernard Koopman recognized in 1931, is that studying the dynamical system is equivalent to studying the spectrum of the unitary operator $U_f$. Properties like ergodicity and mixing translate precisely into spectral properties.

---

## Spectral Measures

**Definition 7.6.2.** The *spectral measure* of $\varphi \in L^2$ is the Borel measure $\sigma_\varphi$ on $S^1$ defined by $\widehat{\sigma_\varphi}(n) = \langle U_f^n \varphi, \varphi \rangle = \int \varphi \circ f^n \cdot \bar{\varphi}\,d\mu$.

The spectral type of $f$ is the equivalence class of measures $\sigma_\varphi$ (under mutual absolute continuity) as $\varphi$ ranges over a cyclic vector.

The spectral measure encodes the correlation structure of the observable $\varphi$. Its Fourier coefficients are the time-correlation functions $\langle U_f^n \varphi, \varphi \rangle = \int \varphi(f^n(x))\overline{\varphi(x)}\,d\mu(x)$. These measure how correlated $\varphi$ is with its time-$n$ shifted version. For mixing systems, these correlations decay to zero — which corresponds to $\sigma_\varphi$ being absolutely continuous with respect to Lebesgue measure on $S^1$ (a spectral fact, by the Riemann-Lebesgue lemma).

**Theorem 7.6.3 (Spectral Isomorphism).** Two MPTs with the same spectral type (as abstract unitary operators) are *spectrally isomorphic* — they have the same eigenvalues, spectral measures, and mixing properties. However, spectral isomorphism is weaker than measurable isomorphism.

The second sentence is crucial. Spectral isomorphism does not imply measurable isomorphism. The spectral invariants — eigenvalues, spectral type — don't capture everything about the dynamics. Ornstein's theory (Section 7.8) identifies what's missing: entropy.

---

## Examples of Spectra

**Examples of Spectra:**
- Irrational rotation $R_\alpha$: pure point spectrum $\{e^{2\pi i n\alpha} : n \in {\mathbb Z}\}$ — all eigenvalues with eigenfunctions $e^{2\pi i n x}$.
- Bernoulli shift: Lebesgue spectrum — every cyclic component is spectrally equivalent to Lebesgue measure on $S^1$.
- Anosov diffeomorphisms: Lebesgue spectrum (with multiplicity).

The irrational rotation has the "simplest" possible spectrum: pure point, with eigenfunctions that are the Fourier modes. This is the spectral explanation for why rotations are equicontinuous and not mixing — pure point spectrum means the dynamics are determined entirely by their eigenvalues, which are roots of unity-like phases.

Bernoulli shifts have Lebesgue spectrum: the spectral measures are absolutely continuous, with no atoms. This is the spectral signature of strong mixing. The Riemann-Lebesgue lemma gives you mixing directly from the spectral type.

Anosov diffeomorphisms also have Lebesgue spectrum (with possibly higher multiplicity), which is why they are also Bernoulli — a deep result of Ornstein-Weiss that we'll touch on in the next section.

The spectral theory gives us a powerful classification tool, but it's incomplete. Two systems can be spectrally isomorphic but dynamically distinct. The missing piece is entropy, which the next two sections develop.
