# Chapter 8 — Stability Theory and Lyapunov Methods

> *Lyapunov's direct method asks: can you find a function that decreases along trajectories? If yes, the system is stable — without solving the equations.*

**Prerequisites:** Chapter 4 (ODEs, flows, equilibria), Chapter 5 (spectral theory).

**What this chapter builds:** Lyapunov stability theory for continuous and discrete systems; the direct (second) method of Lyapunov; LaSalle's invariance principle; input-output stability; and Lyapunov exponents as the global nonlinear generalization.

---

## 8.1 Stability Definitions

**Definition 8.1.1.** The equilibrium $x^* = 0$ of $\dot{x} = f(x)$ (or map $x \mapsto f(x)$) is:
- *Lyapunov stable*: $\forall \varepsilon > 0\ \exists \delta > 0$: $\|x(0)\| < \delta \Rightarrow \|x(t)\| < \varepsilon$ for all $t \geq 0$
- *Asymptotically stable*: Lyapunov stable and $x(t) \to 0$ as $t \to \infty$
- *Exponentially stable*: $\exists C, \lambda > 0$: $\|x(t)\| \leq C e^{-\lambda t} \|x(0)\|$ for all $t \geq 0$
- *Unstable*: not Lyapunov stable
- *Globally asymptotically stable (GAS)*: asymptotically stable with basin of attraction = whole space

---

## 8.2 Lyapunov's Direct Method

**Definition 8.2.1.** A *Lyapunov function* for $\dot{x} = f(x)$ near the origin is a $C^1$ function $V: U \to [0, \infty)$ (on some open $U \ni 0$) satisfying:
1. $V(0) = 0$ and $V(x) > 0$ for $x \neq 0$ (positive definite)
2. $\dot{V}(x) = \nabla V(x) \cdot f(x) \leq 0$ along trajectories (non-increasing)

**Theorem 8.2.2 (Lyapunov Stability Theorem).**
- If (1) and (2) hold with $\dot{V} \leq 0$: the origin is Lyapunov stable.
- If (1) holds and $\dot{V}(x) < 0$ for $x \neq 0$ (negative definite): the origin is asymptotically stable.
- If (1) holds and $\dot{V}(x) \leq -\alpha V(x)$ for some $\alpha > 0$: the origin is exponentially stable.

*(proof of stability)* Given $\varepsilon$, let $c = \min_{\|x\|=\varepsilon} V(x) > 0$. The sublevel set $\{V \leq c/2\}$ is compact and contains a ball $\{x : \|x\| < \delta\}$. If $\|x(0)\| < \delta$, then $V(x(0)) < c/2 \leq c$, and since $\dot{V} \leq 0$, $V(x(t)) \leq c$ for all $t \geq 0$, so $x(t) \in \{V \leq c\} \subseteq \{\|x\| \leq \varepsilon\}$.

**Example 8.2.3 (Damped Harmonic Oscillator).** $\ddot{q} + c\dot{q} + kq = 0$ ($c, k > 0$). As a first-order system: $\dot{x}_1 = x_2$, $\dot{x}_2 = -kx_1 - cx_2$. Lyapunov function: $V = kx_1^2/2 + x_2^2/2$ (total energy).
$$\dot{V} = kx_1 x_2 + x_2(-kx_1 - cx_2) = -cx_2^2 \leq 0.$$
$\dot{V} = 0$ iff $x_2 = 0$, but then $\dot{x}_2 = -kx_1 \neq 0$ unless $x_1 = 0$ too. So the origin is asymptotically stable.

**Example 8.2.4 (Nonquadratic Lyapunov).** For $\dot{x} = -x^3$: $V(x) = x^2/2$, $\dot{V} = -x^4 \leq 0$. GAS. But note: $\dot{V} \leq -2V^2$, not $-\alpha V$, so stability is not exponential (solutions decay as $t^{-1/2}$, not $e^{-\lambda t}$).

---

## 8.3 LaSalle's Invariance Principle

Lyapunov's theorem requires $\dot{V} < 0$. What if $\dot{V} \leq 0$ but with equality on a nontrivial set?

**Theorem 8.3.1 (LaSalle's Invariance Principle).** Let $V$ be a Lyapunov function with $\dot{V} \leq 0$. Let $E = \{x : \dot{V}(x) = 0\}$ and $M$ = largest positively invariant set contained in $E$. Then every bounded trajectory converges to $M$ as $t \to \infty$.

*(proof)* The orbit lies in the compact sublevel set $\{V \leq V(x(0))\}$. By Birkhoff's theorem (topological version), $\omega(x(0)) \neq \emptyset$. Since $V$ decreases along trajectories, $V|_{\omega(x(0))} = c$ for some constant $c$, so $\dot{V}|_{\omega(x(0))} = 0$, i.e., $\omega(x(0)) \subseteq E$. Since $\omega(x(0))$ is positively invariant, $\omega(x(0)) \subseteq M$.

**Corollary 8.3.2.** If $M = \{0\}$ (the only invariant set in $E$ is the origin), then the origin is asymptotically stable.

**Example 8.3.3.** Return to the damped oscillator. $E = \{x_2 = 0\}$. On $E$: $\dot{x}_2 = -kx_1$, so the trajectory immediately leaves $E$ unless $x_1 = 0$ too. The largest invariant set in $E$ is $\{0\}$, so the origin is GAS.

---

## 8.4 Converse Lyapunov Theorems

A natural question: does every asymptotically stable equilibrium admit a Lyapunov function?

**Theorem 8.4.1 (Massera).** If the origin of $\dot{x} = f(x)$ is uniformly asymptotically stable (on some neighborhood), then there exists a smooth ($C^\infty$) Lyapunov function on that neighborhood.

**Theorem 8.4.2 (Kurzweil — GAS Converse).** If the origin of an autonomous ODE is globally asymptotically stable, there exists a smooth proper Lyapunov function on all of ${\mathbb R}^n$ (with $V(x) \to \infty$ as $\|x\| \to \infty$).

*The converse theorems are less constructive but are crucial for robustness analysis: they show stability is equivalent to the existence of a Lyapunov function, not just implied by it.*

---

## 8.5 Lyapunov Exponents

Lyapunov exponents generalize eigenvalues to nonlinear time-varying systems, measuring the asymptotic rate of separation of nearby trajectories.

### 8.5.1 Finite-Time and Asymptotic Exponents

**Definition 8.5.1.** For the ODE $\dot{x} = f(x)$ with flow $\Phi_t$ and initial condition $x_0$, the *Lyapunov exponent* of the tangent vector $v \in T_{x_0}M$ is:
$$\lambda(x_0, v) = \limsup_{t \to \infty} \frac{1}{t} \log \|D\Phi_t(x_0) v\|.$$

The Lyapunov spectrum consists of the distinct values taken by $\lambda(x_0, \cdot)$.

**Theorem 8.5.2 (Oseledec Multiplicative Ergodic Theorem, 1968).** Let $(X, \mathcal{B}, \mu, f)$ be an ergodic MPT with $\int \log^+ \|Df\|\,d\mu < \infty$. Then for $\mu$-a.e. $x$:
1. There exist $k \leq n$ distinct values $\lambda_1 > \lambda_2 > \cdots > \lambda_k$ (the *Lyapunov exponents*)
2. The filtration $\{0\} = V_0(x) \subset V_1(x) \subset \cdots \subset V_k(x) = T_xM$ with $\dim V_i = d_i$
3. For $v \in V_i \setminus V_{i-1}$: $\lim_{t \to \pm\infty} \frac{1}{t} \log \|D\Phi_t(x) v\| = \lambda_i$

The Lyapunov exponents $\lambda_i$ are $\mu$-a.e. constant (by ergodicity).

**Example 8.5.3.** For a linear map $\dot{x} = Ax$ with $A$ diagonalizable, the Lyapunov exponents are the real parts of the eigenvalues: $\lambda_i = \text{Re}(\lambda_i(A))$.

**Example 8.5.4 (Cat Map).** The Arnold cat map $f_A$ on ${\mathbb T}^2$ with $A = \begin{pmatrix} 2 & 1 \\ 1 & 1\end{pmatrix}$ has eigenvalues $\lambda_\pm = (3 \pm \sqrt{5})/2$. The Lyapunov exponents are $\log \lambda_+$ and $\log \lambda_-$ (negative).

### 8.5.2 Chaos and Positive Lyapunov Exponents

**Definition 8.5.5.** A system is *chaotic* (in the Lyapunov sense) if $\mu$-a.e. orbit has at least one positive Lyapunov exponent.

Positive Lyapunov exponents mean nearby trajectories diverge exponentially — the hallmark of sensitive dependence. Negative exponents mean contraction. For Hamiltonian systems ($\text{tr}(Df) = 0$, Liouville), the exponents sum to zero: positive and negative exponents come in pairs.

**Theorem 8.5.6 (Pesin's Formula, 1977).** For a $C^2$ diffeomorphism $f$ of a compact manifold preserving a smooth measure $\mu$ (absolutely continuous w.r.t. Lebesgue):
$$h_\mu(f) = \int_X \sum_{\lambda_i > 0} \lambda_i(x)\,d\mu(x) = \sum_{\lambda_i > 0} \lambda_i \cdot d_i$$
(the KS entropy equals the sum of positive Lyapunov exponents, counted with multiplicity).

---

## 8.6 Stability of Periodic Orbits — Floquet Theory

**Definition 8.6.1.** For a periodic orbit $\gamma$ of period $T$ (so $\Phi_T(p) = p$ for $p \in \gamma$), the *Floquet multipliers* are the eigenvalues of the linearized return map $D\Phi_T(p): T_pM \to T_pM$.

One eigenvalue is always $1$ (in the direction of the flow). The others determine the transverse stability.

**Theorem 8.6.2 (Floquet).** The variational equation $\dot{J} = A(t)J$ along a $T$-periodic orbit is equivalent (by a periodic change of variables) to a constant-coefficient linear ODE. The monodromy matrix $M = J(T)$ has eigenvalues = Floquet multipliers.

- If all Floquet multipliers satisfy $|\mu_i| < 1$ (except the trivial $1$): the periodic orbit is asymptotically stable.
- If any $|\mu_i| > 1$: unstable.
- If all $|\mu_i| = 1$ (except trivial): marginally stable (requires nonlinear analysis).

---

## 8.7 Stability in Discrete-Time Systems

For a discrete map $x_{n+1} = g(x_n)$:

**Definition 8.7.1.** The equilibrium $x^* = 0$ of $x_{n+1} = g(x_n)$ is:
- *Lyapunov stable*: $\forall \varepsilon > 0\ \exists \delta > 0$: $\|x_0\| < \delta \Rightarrow \|g^n(x_0)\| < \varepsilon\ \forall n \geq 0$
- *Asymptotically stable*: stable and $g^n(x_0) \to 0$ as $n \to \infty$

**Theorem 8.7.2.** The linearization $Dg(0)$ determines local stability when all eigenvalues have $|\lambda| \neq 1$:
- $|\lambda_i| < 1$ for all $i$: asymptotically stable
- $|\lambda_i| > 1$ for some $i$: unstable

For discrete Lyapunov functions: $V \geq 0$, $V(0) = 0$, and $V(g(x)) \leq \alpha V(x)$ for some $\alpha < 1$ gives exponential stability.

---

## Exercises

**Exercise 8.1.** Find a Lyapunov function for the system $\dot{x}_1 = -x_1 + x_2^2$, $\dot{x}_2 = -x_2 - x_1 x_2$. Show the origin is GAS.

**Exercise 8.2.** (LaSalle) Consider $\dot{x}_1 = x_2$, $\dot{x}_2 = -\sin(x_1) - x_2$ (damped pendulum). Take $V = 1 - \cos(x_1) + x_2^2/2$. Apply LaSalle's theorem to show all bounded solutions converge to an equilibrium.

**Exercise 8.3.** For the system $\dot{x} = -x^3 + x^5$: (a) Find the equilibria. (b) Is the origin stable? GAS? Compute the Lyapunov function $V = x^2/2$ and $\dot{V}$. For which initial conditions does the solution diverge?

**Exercise 8.4.** (Floquet) The Mathieu equation $\ddot{x} + (a + b\cos t) x = 0$ is a periodic linear ODE with period $\pi$. For $a = 1$, $b = 0$: find the Floquet multipliers explicitly. For small $b$, describe what happens to stability.

**Exercise 8.5.** (Lyapunov Exponents) For the tent map $T(x) = 1 - |2x-1|$: compute $|DT(x)| = 2$ almost everywhere. Use Birkhoff's theorem to show the Lyapunov exponent of Lebesgue-a.e. orbit is $\log 2$.

**Exercise 8.6.** Prove Pesin's formula for linear toral automorphisms: $h_\mu(f_A) = \sum_{\lambda > 1} \log \lambda$ where $\lambda$ ranges over eigenvalues of $A$ with $|\lambda| > 1$. (*Hint:* Use the Bernoulli generator given by a Markov partition.)

**Exercise 8.7.** (Research) For the Collatz map on ${\mathbb Z}_2$: the map $T$ is piecewise linear. Compute its local expansion rates. What would it mean for the "Lyapunov exponent" of the Collatz map to be positive?

---

## Chapter Notes

Lyapunov's original work (*The General Problem of the Stability of Motion*, 1892) introduced both the linearization method and the direct method. It is the foundation of stability theory for dynamical systems and control theory.

The modern reference is Khalil's *Nonlinear Systems* (3rd edition) for the engineering perspective, and Bhatia-Szegő's *Stability Theory of Dynamical Systems* for the pure mathematics treatment.

Oseledec's theorem (Section 8.5.2) is proven in Mañé's *Ergodic Theory and Differentiable Dynamics*. For the accessible version, see the appendix of Katok-Hasselblatt. Pesin's formula (Theorem 8.5.6) is from Pesin's 1977 paper; the modern treatment is in Liu-Qian's *Smooth Ergodic Theory for Endomorphisms*.
