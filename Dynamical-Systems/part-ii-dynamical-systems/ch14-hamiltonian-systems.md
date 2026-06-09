# Chapter 14 — Hamiltonian Systems and KAM Theory

> *Hamiltonian systems model everything that conserves energy: celestial mechanics, molecular dynamics, plasma physics, quantum field theory. KAM theory explains why solar system orbits are (approximately) stable despite resonances — and why this is not obvious.*

**Prerequisites:** Chapters 3 (differential forms, manifolds), 4 (ODEs, flows), 8 (stability theory).

**What this chapter builds:** Symplectic geometry as the natural setting for Hamiltonian mechanics; Liouville integrability and action-angle variables; the KAM theorem on persistence of invariant tori; Aubry-Mather theory for breakdown of tori; and the connections to Arnold diffusion and modern symplectic topology.

---

## 14.1 Symplectic Geometry

**Definition 14.1.1.** A *symplectic manifold* $(M, \omega)$ is a smooth manifold $M^{2n}$ equipped with a closed ($d\omega = 0$) non-degenerate ($\omega^n \neq 0$) 2-form $\omega$ (the *symplectic form*).

**Canonical Example:** $({\mathbb R}^{2n}, \omega_0)$ with $\omega_0 = \sum_{i=1}^n dq_i \wedge dp_i$ (standard symplectic form on phase space).

**Theorem 14.1.2 (Darboux).** Every symplectic manifold is locally symplectomorphic to $({\mathbb R}^{2n}, \omega_0)$: there exist local coordinates $(q_1, \ldots, q_n, p_1, \ldots, p_n)$ (Darboux coordinates) with $\omega = \sum_i dq_i \wedge dp_i$.

*Darboux's theorem is the symplectic analogue of Riemannian geometry's lack of such a result — a Riemannian manifold has intrinsic curvature, but a symplectic manifold has no local invariants.*

### 14.1.1 Hamilton's Equations

**Definition 14.1.3.** Given $H: M \to {\mathbb R}$ (the *Hamiltonian*), the *Hamiltonian vector field* $X_H$ is defined by $\iota_{X_H}\omega = dH$ (interior product), i.e., $\omega(X_H, \cdot) = dH(\cdot)$.

In Darboux coordinates:
$$\dot{q}_i = \frac{\partial H}{\partial p_i}, \quad \dot{p}_i = -\frac{\partial H}{\partial q_i}.$$

**Theorem 14.1.4 (Conservation of Energy).** $H$ is constant along trajectories: $\frac{d}{dt} H(\gamma(t)) = dH(\dot\gamma) = dH(X_H) = \omega(X_H, X_H) = 0$.

**Theorem 14.1.5 (Liouville's Theorem).** The Hamiltonian flow preserves the symplectic volume form $\omega^n / n!$ (the Liouville measure). In Darboux coordinates: $dq_1 \cdots dq_n \, dp_1 \cdots dp_n$ is invariant.

*Proof:* $\mathcal{L}_{X_H}(\omega^n) = n\omega^{n-1} \wedge \mathcal{L}_{X_H}\omega = n\omega^{n-1} \wedge d(\iota_{X_H}\omega) = n\omega^{n-1} \wedge d(dH) = 0$.

---

## 14.2 Integrable Systems

**Definition 14.2.1.** A Hamiltonian system $(M^{2n}, \omega, H)$ is *completely integrable* (in the Liouville-Arnold sense) if there exist $n$ functions $F_1 = H, F_2, \ldots, F_n: M \to {\mathbb R}$ that are:
1. *Independent*: $dF_1, \ldots, dF_n$ linearly independent on a dense open set
2. *In involution*: $\{F_i, F_j\} = 0$ for all $i, j$ (Poisson bracket vanishes)

**Theorem 14.2.2 (Liouville-Arnold Theorem).** For a completely integrable system with compact connected level sets $M_c = \{F_1 = c_1, \ldots, F_n = c_n\}$:
1. Each $M_c$ is diffeomorphic to an $n$-torus ${\mathbb T}^n$.
2. The Hamiltonian flow is quasi-periodic on ${\mathbb T}^n$: $(\theta_1, \ldots, \theta_n) \mapsto (\theta_1 + \omega_1 t, \ldots, \theta_n + \omega_n t)$ for some frequency vector $\omega = (\omega_1, \ldots, \omega_n)$.
3. There exist *action-angle coordinates* $(I_1, \ldots, I_n, \theta_1, \ldots, \theta_n)$ in which $H = H(I_1, \ldots, I_n)$ (depends only on actions) and $\omega = \sum_i dI_i \wedge d\theta_i$.

**Examples:**
- 1D Hamiltonian $H = p^2/2 + V(q)$ is always integrable ($F = H$).
- $n$ uncoupled harmonic oscillators: $H = \sum_i (p_i^2 + q_i^2)/2$.
- The Kepler problem (gravitational 2-body problem): integrable with 3 conserved quantities.
- The geodesic flow on an ellipsoid: integrable (Jacobi).

---

## 14.3 KAM Theory

### 14.3.1 The Problem

A *nearly integrable* system is $H_\varepsilon = H_0(I) + \varepsilon H_1(I, \theta)$ where $H_0$ is completely integrable and $\varepsilon$ is small.

**Question:** Do the invariant tori of $H_0$ persist under the perturbation $\varepsilon H_1$?

*Naive answer (wrong):* By averaging over angles, one might expect tori to persist for small $\varepsilon$. But there are resonances: tori with rational frequency ratios $\omega_i/\omega_j \in {\mathbb Q}$ are destroyed (they cannot support quasi-periodic motion of the perturbed system).

**The real answer:** Most tori persist — those with Diophantine frequency vectors.

### 14.3.2 Diophantine Conditions

**Definition 14.3.1.** $\omega \in {\mathbb R}^n$ is *Diophantine* with constants $(\gamma, \tau)$ if:
$$|\omega \cdot k| \geq \frac{\gamma}{|k|^\tau} \quad \text{for all } k \in {\mathbb Z}^n \setminus \{0\},$$
where $|k| = |k_1| + \cdots + |k_n|$.

The set of Diophantine vectors has full Lebesgue measure for $\tau > n-1$.

**Intuition:** Diophantine means "badly approximable by rational vectors." Resonances occur when $\omega \cdot k = 0$ (exactly rational), which causes "small divisors" that blow up in perturbation series.

### 14.3.3 The KAM Theorem

**Theorem 14.3.3 (KAM Theorem — Kolmogorov, 1954; Arnold, 1963; Moser, 1962).** Let $H_0$ be a real-analytic, completely integrable Hamiltonian with nondegenerate frequency map ($\det [\partial^2 H_0 / \partial I_i \partial I_j] \neq 0$). Let $H_\varepsilon = H_0 + \varepsilon H_1$ be a real-analytic perturbation. Then for sufficiently small $\varepsilon$, the Hamiltonian $H_\varepsilon$ has a positive-measure family of invariant tori carrying quasi-periodic motion. Each surviving torus corresponds to a Diophantine frequency vector of the unperturbed system.

**Consequences:**
- The measure of surviving tori $\to 1$ as $\varepsilon \to 0$ (nearly all tori survive).
- The complement of surviving tori has measure $\to 0$ as $\varepsilon \to 0$.
- Tori with rational frequency ratios are destroyed; near destroyed tori, chaotic layers form.

**The Small Divisor Problem:** The formal power series solution to the invariance equation has denominators $\omega \cdot k$ (the divisors). Diophantine condition ensures these are not too small. The convergence proof uses a Newton-iteration scheme (quadratic convergence) to overcome the small divisors.

---

## 14.4 Twist Maps and Aubry-Mather Theory

When tori break down (non-Diophantine frequencies), what replaces them?

**Definition 14.4.1.** A *twist map* is an area-preserving diffeomorphism $f: {\mathbb T} \times [a, b] \to {\mathbb T} \times [a,b]$ of the annulus satisfying the *twist condition*: $\partial q'/\partial p > 0$ in coordinates $q' = q + \phi(q, p)$.

*The standard map* $f(q, p) = (q + p + K\sin(q), p + K\sin(q)) \pmod{2\pi}$ is the prototype.

**Theorem 14.4.2 (Aubry-Mather Theorem, 1982-1983).** For every *irrational* rotation number $\alpha \in {\mathbb R} \setminus {\mathbb Q}$, a twist map has an *Aubry-Mather set* $M_\alpha$ with:
1. $M_\alpha$ is compact and invariant under $f$.
2. $M_\alpha$ is contained in the graph of a Lipschitz function $p = p(q)$.
3. The dynamics of $f|_{M_\alpha}$ is semi-conjugate to the rotation $R_\alpha$ on ${\mathbb T}$.
4. If $M_\alpha$ is a continuous curve (Lipschitz graph over the whole circle), it is a KAM torus. If not, $M_\alpha$ is a Cantor set (a "cantorus").

**Interpretation:** Aubry-Mather sets are the remnants of KAM tori after they break down. The cantori are like "fractal KAM tori" — they constrain the dynamics but no longer form a barrier.

---

## 14.5 Arnold Diffusion

**Theorem 14.5.1 (Arnold, 1964 — Example).** In $n \geq 3$ degrees of freedom, KAM tori do not form codimension-1 barriers. Orbits can drift in the action space $I = (I_1, \ldots, I_n)$ from any initial value to any target value: this is *Arnold diffusion*.

**Precise Statement:** There exist nearly integrable systems in 3+ degrees of freedom where orbits slowly drift through the "web" of resonances. The drift rate is exponentially slow (of order $e^{-1/\varepsilon}$), but it occurs.

**The Mather Problem:** Is Arnold diffusion generic in $n \geq 3$ degrees of freedom? This is largely resolved (Mather's variational methods, Cheng-Yan, Bernard-Kaloshin-Zhang) but remains an active area.

---

## 14.6 Generating Functions and Variational Principles

**Definition 14.6.1.** For a symplectomorphism $f: M \to M$, a *generating function* $S(q, Q)$ satisfies:
$$p = -\frac{\partial S}{\partial q}, \quad P = \frac{\partial S}{\partial Q},$$
where $(q, p) \mapsto (Q, P) = f(q, p)$.

**Principle of Least Action:** The action functional $\mathcal{A}[\gamma] = \int_{t_1}^{t_2} (p\,\dot{q} - H(q,p))\,dt$ is stationary on true orbits (Hamilton's variational principle).

**Theorem 14.6.2 (Mather's Variational Theory).** Define the *Mather set* as the support of the minimizing measure for the action functional. Mather sets generalize KAM tori to the non-perturbative regime and provide invariant objects (action-minimizing orbits) even when KAM breaks down.

---

## 14.7 Connections to Quantum Mechanics

**Correspondence Principle:** Classical Hamiltonian mechanics is the $\hbar \to 0$ limit of quantum mechanics. Under quantization:
- Classical phase space $(q, p)$ $\to$ Hilbert space $L^2({\mathbb R}^n)$
- Classical Hamiltonian $H(q, p)$ $\to$ Schrödinger operator $\hat{H} = -\hbar^2 \nabla^2/2m + V(q)$
- Classical flow $\Phi_t$ $\to$ Unitary group $e^{-it\hat{H}/\hbar}$

**Quantum Chaos:** The quantum mechanics of classically chaotic systems shows distinctive spectral statistics:
- Classically integrable: eigenvalue spacings follow Poisson statistics
- Classically chaotic: eigenvalue spacings follow GUE (Gaussian Unitary Ensemble) statistics from random matrix theory (Bohigas-Giannoni-Schmit conjecture)

**Theorem 14.7.1 (Quantum Ergodicity — Shnirelman, Zelditch, Colin de Verdière).** If the geodesic flow on a compact Riemannian manifold is ergodic, then almost all eigenfunctions of the Laplacian equidistribute — their squared moduli converge to Liouville measure.

---

## Exercises

**Exercise 14.1.** Verify that the standard symplectic form $\omega_0 = \sum_i dq_i \wedge dp_i$ is closed ($d\omega_0 = 0$) and nondegenerate. Write out Hamilton's equations for $H = |p|^2/2 + V(q)$.

**Exercise 14.2.** (Liouville-Arnold) For the 2D harmonic oscillator $H = (p_1^2 + q_1^2)/2 + (p_2^2 + q_2^2)/2$, find the action-angle coordinates $(I_1, I_2, \theta_1, \theta_2)$. Show $H = I_1 + I_2$.

**Exercise 14.3.** Compute the standard map $f(q, p) = (q + p + K\sin q, p + K\sin q) \pmod{2\pi}$ for $K = 0$ (integrable). Show that for small $K$, the circles $\{p = \text{const}\}$ are perturbed to invariant curves. Estimate the critical $K$ where the last KAM torus breaks down (it is approximately $K_c \approx 0.9716...$).

**Exercise 14.4.** (Poincaré-Birkhoff) For a twist map with rotation number $p/q$ (rational), Poincaré-Birkhoff theorem guarantees at least two periodic orbits of period $q$. Verify this for the standard map at $K = 0$ for the orbits with $p/q = 1/2$.

**Exercise 14.5.** For the pendulum $H = p^2/2 - \cos q$: (a) sketch the phase portrait; (b) find all equilibria; (c) identify the separatrix (the curve connecting the saddle to itself); (d) compute the period of libration orbits as a function of amplitude; (e) show the system is integrable. What happens when we add a small periodic perturbation $\varepsilon\sin(q - t)$?

**Exercise 14.6.** (Quantum Ergodicity) For the quantum harmonic oscillator $\hat{H} = -d^2/dx^2 + x^2$ on ${\mathbb R}$, the eigenfunctions are Hermite functions $\psi_n$. Does $|\psi_n|^2 \to $ Lebesgue measure? (The answer is no — but Shnirelman's theorem applies to compact manifolds with ergodic geodesic flow.)

---

## Chapter Notes

The foundational text is Arnold's *Mathematical Methods of Classical Mechanics* — probably the most beautiful mathematics textbook ever written. It develops Hamiltonian mechanics from the perspective of differential geometry and symplectic topology.

For KAM theory: the original papers are Kolmogorov (1954), Arnold (1963), and Moser (1962). The modern rigorous treatment is in de la Llave's lecture notes and Chierchia-Gallavotti. For Aubry-Mather theory: Mather's original papers and the survey by Mather-Forni are the references.

For the connection to quantum mechanics and quantum chaos: Haake's *Quantum Signatures of Chaos* is the physicist's reference; Zelditch's papers on quantum ergodicity are the mathematical treatment.

Arnold diffusion is an active research area. The survey by Kaloshin-Levi (*Arnold Diffusion for Smooth Systems and a Counterexample to the Generalized Diffusion Condition*) and the book by Arnold-Kozlov-Neishtadt give different perspectives.
