# 14.3 KAM Theory

Kolmogorov-Arnold-Moser theory answers one of the oldest questions in mathematical physics: if you perturb a completely integrable Hamiltonian system by a small perturbation, what happens to the invariant tori? Do they survive? Do they all disappear? The answer is subtle, beautiful, and quantitative.

## 14.3.1 The Problem

A *nearly integrable* system is a Hamiltonian of the form:
$$H_\varepsilon = H_0(I) + \varepsilon H_1(I, \theta)$$
where $H_0$ is completely integrable (depending only on the actions $I$) and $\varepsilon$ is small. The unperturbed system has invariant tori $\{I = \text{const}\}$, with frequency vector $\omega(I) = \partial H_0/\partial I$.

**Question:** Do the invariant tori of $H_0$ persist under the perturbation $\varepsilon H_1$?

The naive approach — formal perturbation theory, expanding solutions in powers of $\varepsilon$ — fails spectacularly. The perturbation series has denominators of the form $\omega \cdot k = \omega_1 k_1 + \cdots + \omega_n k_n$ for $k \in \mathbb{Z}^n$. When $\omega \cdot k$ is small (near-resonance), these denominators blow up. The perturbation series diverges for essentially all frequency vectors — this is the *small divisor problem* that plagued celestial mechanics for a century.

Poincaré thought this meant the tori were destroyed. He was wrong — mostly.

**The real answer:** Tori with *Diophantine* frequency vectors — those that are badly approximable by rational vectors — survive. Resonant tori (with rational $\omega \cdot k = 0$ for some $k$) are destroyed, and near them, chaotic behavior develops. But the Diophantine tori form a set of large measure, and this measure approaches full measure as $\varepsilon \to 0$.

## 14.3.2 Diophantine Conditions

**Definition 14.3.1.** $\omega \in \mathbb{R}^n$ is *Diophantine* with constants $(\gamma, \tau)$ if:
$$|\omega \cdot k| \geq \frac{\gamma}{|k|^\tau} \quad \text{for all } k \in \mathbb{Z}^n \setminus \{0\},$$
where $|k| = |k_1| + \cdots + |k_n|$.

The set of Diophantine vectors has full Lebesgue measure when $\tau > n-1$.

**Intuition:** A Diophantine frequency vector is "badly approximable by rationals" — it cannot be too close to any rational vector. The condition $|\omega \cdot k| \geq \gamma / |k|^\tau$ says that the dot product of $\omega$ with any nonzero integer vector cannot be too small. This prevents the small divisors from blowing up: instead of $|\omega \cdot k|$ being arbitrarily small, it is bounded below polynomially in $|k|$.

Resonances occur when $\omega \cdot k = 0$ for some $k \in \mathbb{Z}^n \setminus \{0\}$ — the orbit on the torus is periodic, returning exactly to the start after time $2\pi/(\omega \cdot k)$ in some direction. Diophantine vectors avoid this: they are not exactly resonant, and they are not too close to being resonant.

## 14.3.3 The KAM Theorem

The KAM theorem was announced by Kolmogorov in 1954 (with proof ideas), proved by Arnold (1963) for Hamiltonian systems, and proved by Moser (1962) for twist maps. Together, they constitute one of the landmark results of 20th-century mathematics.

**Theorem 14.3.3 (KAM Theorem).** Let $H_0$ be a real-analytic, completely integrable Hamiltonian with *nondegenerate frequency map* ($\det [\partial^2 H_0 / \partial I_i \partial I_j] \neq 0$ — the torus frequency changes with the action). Let $H_\varepsilon = H_0 + \varepsilon H_1$ be a real-analytic perturbation. Then for sufficiently small $\varepsilon$, the Hamiltonian $H_\varepsilon$ has a positive-measure family of invariant tori carrying quasi-periodic motion. Each surviving torus corresponds to a Diophantine frequency vector $\omega(I)$ of the unperturbed system.

**Consequences:**
- The measure of surviving tori approaches $1$ as $\varepsilon \to 0$: nearly all tori survive for small perturbations.
- The complement of surviving tori (the "chaotic web") has measure approaching $0$ as $\varepsilon \to 0$.
- Tori with rational frequency ratios are destroyed; in a neighborhood of each destroyed torus, chaotic layers form (the Birkhoff instability zones).
- The surviving tori are $C^\infty$ (in fact, Whitney-smooth in the actions), even though they are not holomorphic.

**The Small Divisor Problem and Newton Iteration:** Why does KAM succeed where naive perturbation theory fails? The key is a Newton-iteration scheme. Instead of expanding in powers of $\varepsilon$ (linear convergence), KAM uses quadratic Newton steps: at each step, the error is *squared*, giving convergence that is super-exponentially fast. The Diophantine condition ensures that the small divisors $|\omega \cdot k|$ are under control at each step, and the quadratic convergence is fast enough to overcome the polynomial growth of $|k|^\tau$ in the denominators.

This Newton-iteration idea — using quadratic convergence to overcome small divisors — is one of the most powerful techniques in modern analysis and appears in many other contexts (Nash-Moser implicit function theorem, etc.).

In two degrees of freedom, KAM tori form codimension-1 barriers on the energy surface — orbits cannot cross them. This gives dynamical stability: orbits starting between two KAM tori are trapped between them forever. In three or more degrees of freedom, the situation changes dramatically, as we see in Section 14.5.
