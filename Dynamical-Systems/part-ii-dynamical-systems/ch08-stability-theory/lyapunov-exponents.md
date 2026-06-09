# 8.5 Lyapunov Exponents

So far, we've studied stability of equilibria — the behavior of a system near a fixed point. But what about the long-term stability of an arbitrary orbit? If you start two nearby initial conditions and let them evolve, do they stay close or diverge?

Lyapunov exponents answer this question. They generalize eigenvalues (which answer it for linear systems near equilibria) to nonlinear time-varying systems, measuring the asymptotic rate of separation of nearby trajectories.

Lyapunov exponents generalize eigenvalues to nonlinear time-varying systems, measuring the asymptotic rate of separation of nearby trajectories.

---

## Finite-Time and Asymptotic Exponents

**Definition 8.5.1.** For the ODE $\dot{x} = f(x)$ with flow $\Phi_t$ and initial condition $x_0$, the *Lyapunov exponent* of the tangent vector $v \in T_{x_0}M$ is:
$$\lambda(x_0, v) = \limsup_{t \to \infty} \frac{1}{t} \log \|D\Phi_t(x_0) v\|.$$

The Lyapunov spectrum consists of the distinct values taken by $\lambda(x_0, \cdot)$.

Here $D\Phi_t(x_0)$ is the derivative of the flow: it tells you how a small perturbation $v$ at $x_0$ evolves under the linearized dynamics. If $\|D\Phi_t(x_0) v\| \approx e^{\lambda t} \|v\|$, then $\lambda$ is the Lyapunov exponent in the direction $v$.

Positive Lyapunov exponent: nearby orbits diverge exponentially. Negative: they converge. Zero: they neither diverge nor converge (marginally stable, like the Hamiltonian case).

The great theorem organizing all of this is due to Oseledec, proved in 1968.

**Theorem 8.5.2 (Oseledec Multiplicative Ergodic Theorem, 1968).** Let $(X, \mathcal{B}, \mu, f)$ be an ergodic MPT with $\int \log^+ \|Df\|\,d\mu < \infty$. Then for $\mu$-a.e. $x$:
1. There exist $k \leq n$ distinct values $\lambda_1 > \lambda_2 > \cdots > \lambda_k$ (the *Lyapunov exponents*)
2. The filtration $\{0\} = V_0(x) \subset V_1(x) \subset \cdots \subset V_k(x) = T_xM$ with $\dim V_i = d_i$
3. For $v \in V_i \setminus V_{i-1}$: $\lim_{t \to \pm\infty} \frac{1}{t} \log \|D\Phi_t(x) v\| = \lambda_i$

The Lyapunov exponents $\lambda_i$ are $\mu$-a.e. constant (by ergodicity).

What this is really saying: for almost every initial condition, the tangent space $T_xM$ splits into invariant subspaces, one for each Lyapunov exponent. A tangent vector in the $i$-th subspace grows (or shrinks) at exactly the rate $e^{\lambda_i t}$ — and this is a genuine limit, not just a $\limsup$. The exponents are constant almost everywhere because the system is ergodic: the "asymptotic expansion rates" don't depend on where you start (generically).

**Example 8.5.3.** For a linear map $\dot{x} = Ax$ with $A$ diagonalizable, the Lyapunov exponents are the real parts of the eigenvalues: $\lambda_i = \text{Re}(\lambda_i(A))$.

**Example 8.5.4 (Cat Map).** The Arnold cat map $f_A$ on ${\mathbb T}^2$ with $A = \begin{pmatrix} 2 & 1 \\ 1 & 1\end{pmatrix}$ has eigenvalues $\lambda_\pm = (3 \pm \sqrt{5})/2$. The Lyapunov exponents are $\log \lambda_+$ and $\log \lambda_-$ (negative).

For the cat map, the linearization is constant (the map is linear on the torus), so the Lyapunov exponents are just the logs of the eigenvalue moduli. The positive exponent $\log \lambda_+ \approx 0.962$ tells you that nearby trajectories diverge at roughly $e^{0.962}$ per iterate in the unstable direction.

---

## Chaos and Positive Lyapunov Exponents

**Definition 8.5.5.** A system is *chaotic* (in the Lyapunov sense) if $\mu$-a.e. orbit has at least one positive Lyapunov exponent.

Positive Lyapunov exponents mean nearby trajectories diverge exponentially — the hallmark of sensitive dependence. Negative exponents mean contraction. For Hamiltonian systems ($\text{tr}(Df) = 0$, Liouville), the exponents sum to zero: positive and negative exponents come in pairs.

---

## Pesin's Formula

The deepest result connecting Lyapunov exponents to entropy is Pesin's formula, proved in 1977.

**Theorem 8.5.6 (Pesin's Formula, 1977).** For a $C^2$ diffeomorphism $f$ of a compact manifold preserving a smooth measure $\mu$ (absolutely continuous w.r.t. Lebesgue):
$$h_\mu(f) = \int_X \sum_{\lambda_i > 0} \lambda_i(x)\,d\mu(x) = \sum_{\lambda_i > 0} \lambda_i \cdot d_i$$
(the KS entropy equals the sum of positive Lyapunov exponents, counted with multiplicity).

Pesin's formula is one of the most beautiful results in the subject. It says: the entropy of a system (how fast information is generated) equals the total expansion rate (sum of positive Lyapunov exponents). Expansion creates new information: two nearby orbits that diverge exponentially at rate $\lambda$ generate $\lambda$ bits per second of information about which one is which.

The formula connects two objects that were defined very differently — entropy via partitions and information (Section 7.7) and Lyapunov exponents via tangent vectors and linearization. They turn out to measure the same thing, from two different angles. This is a deep structural fact about smooth ergodic theory.

In the next section, we ask about stability not of equilibria but of periodic orbits.
