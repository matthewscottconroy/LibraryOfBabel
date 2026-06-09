# Chapter 11 — Chaos Theory

> *Chaos is not disorder. It is deterministic complexity — systems with a perfectly definite future that are nonetheless impossible to predict. The mathematics of chaos is about measuring and explaining this paradox.*

**Prerequisites:** Chapters 7 (ergodic theory, entropy), 8 (Lyapunov exponents, Oseledec), 9 (hyperbolic dynamics, Markov partitions).

**What this chapter builds:** Precise characterizations of chaos (Devaney, Lyapunov, Li-Yorke); the Lorenz system and strange attractors; fractal geometry and dimension theory; the logistic map family and the universality of period-doubling; multifractal analysis; and the relationship between Lyapunov exponents, entropy, and information production.

---

## 11.1 What is Chaos?

Chaos is a word with multiple mathematical definitions, each capturing a different aspect of complex dynamics.

### 11.1.1 Three Definitions Compared

**Definition 11.1.1 (Devaney Chaos).** A continuous map $f: X \to X$ is *Devaney chaotic* if: (1) topologically transitive, (2) periodic points dense, (3) sensitive dependence on initial conditions (SDIC). (As shown in Chapter 6, (3) follows from (1)+(2).)

**Definition 11.1.2 (Li-Yorke Chaos).** $f$ is *Li-Yorke chaotic* if there exists an uncountable scrambled set $S$ (pairs in $S$ are simultaneously proximal and distal).

**Definition 11.1.3 (Positive Entropy Chaos).** $f$ is chaotic if $h_{\text{top}}(f) > 0$.

**Relationships:** Positive topological entropy implies Li-Yorke chaos (Blanchard-Glasner-Kolyada-Maass). Li-Yorke chaos does not imply positive entropy. Devaney chaos on an infinite space implies Li-Yorke chaos and positive entropy.

---

## 11.2 The Lorenz System

**The Equations:** Derived by Edward Lorenz (1963) as a simplified model of atmospheric convection:
$$\dot{x} = \sigma(y - x), \quad \dot{y} = x(\rho - z) - y, \quad \dot{z} = xy - \beta z.$$
Standard parameters: $\sigma = 10$, $\rho = 28$, $\beta = 8/3$.

**Properties:**
- The system has three equilibria: $(0,0,0)$ and $(\pm\sqrt{\beta(\rho-1)}, \pm\sqrt{\beta(\rho-1)}, \rho-1)$.
- For standard parameters, all three equilibria are unstable.
- The system is dissipative: $\nabla \cdot F = -\sigma - 1 - \beta = -41/3 < 0$ (phase volume shrinks).
- Solutions are globally bounded: $V = x^2 + y^2 + (z-\rho-\sigma)^2$ decreases outside a large ellipsoid.

**The Lorenz Attractor:** The omega-limit set of Lebesgue-a.e. initial condition in the bounding ellipsoid is a *strange attractor* — a compact invariant set that is neither a fixed point nor a periodic orbit, on which the dynamics is chaotic.

**Theorem 11.2.1 (Tucker, 2002 — Computer-Assisted Proof).** The Lorenz system with standard parameters has a robust chaotic attractor — a uniformly hyperbolic attractor — confirming Lorenz's numerical observations rigorously.

*(The proof uses rigorous interval arithmetic to construct a Poincaré map on a cross-section and verify its hyperbolic properties.)*

### 11.2.1 The Lorenz Map

The Poincaré map of the Lorenz system on the section $\{z = \rho - 1\}$ produces a one-dimensional map $F: [0,1] \to [0,1]$ (the *Lorenz map*) that is:
- Monotone increasing on $(0, 1/2)$ and $(1/2, 1)$
- Has a discontinuity at $x = 1/2$ with $F(1/2^-) = 1$ and $F(1/2^+) = 0$
- Has slopes $> 1$ everywhere

This map is chaotic (SDIC, dense periodic orbits, positive entropy $\approx \log 2$).

---

## 11.3 Strange Attractors

**Definition 11.3.1.** An *attractor* of a dynamical system is a compact invariant set $\Lambda$ such that some open neighborhood $U \supseteq \Lambda$ has $\bigcap_{t \geq 0} \Phi_t(U) = \Lambda$ (all nearby orbits converge to $\Lambda$).

An attractor is *strange* if it is fractal (non-integer Hausdorff dimension) and has sensitive dependence.

**Example 11.3.2 (Hénon Map).** $H_{a,b}(x,y) = (1 - ax^2 + y, bx)$ for $a = 1.4$, $b = 0.3$ has a strange attractor in ${\mathbb R}^2$. The attractor has fractal structure: zooming in reveals self-similar patterns.

**Theorem 11.3.3 (Benedicks-Carleson, 1991).** For Lebesgue-a.e. $b$ close to $0$ and for $a$ in a positive measure set near $a = 2$, the Hénon map has a strange attractor with a unique SRB measure.

---

## 11.4 Fractal Geometry

### 11.4.1 Hausdorff Dimension

**Definition 11.4.1.** The *Hausdorff $d$-measure* of a set $A \subseteq {\mathbb R}^n$ is:
$$\mathcal{H}^d(A) = \lim_{\delta \to 0} \inf\left\{\sum_i |U_i|^d : A \subseteq \bigcup_i U_i,\ |U_i| \leq \delta\right\}.$$

The *Hausdorff dimension* of $A$ is $\dim_H(A) = \inf\{d : \mathcal{H}^d(A) = 0\} = \sup\{d : \mathcal{H}^d(A) = \infty\}$.

**Theorem 11.4.2 (Basic Properties).**
1. $\dim_H(A) \in [0, n]$ for $A \subseteq {\mathbb R}^n$
2. $\dim_H(A) \leq \dim_H(B)$ if $A \subseteq B$ (monotonicity)
3. $\dim_H(\bigcup_n A_n) = \sup_n \dim_H(A_n)$ for countable unions
4. $\dim_H({\mathbb R}^n) = n$, $\dim_H$ of smooth $k$-manifold $= k$

**Examples:**
- Cantor set $C$: $\dim_H(C) = \log 2 / \log 3 \approx 0.631$
- Lorenz attractor: $\dim_H \approx 2.06$
- Hénon attractor: $\dim_H \approx 1.26$

### 11.4.2 Box-Counting Dimension

**Definition 11.4.3.** The *box-counting dimension* (or *Minkowski dimension*) is:
$$\dim_B(A) = \lim_{\varepsilon \to 0} \frac{\log N(A, \varepsilon)}{\log(1/\varepsilon)},$$
where $N(A, \varepsilon)$ is the minimum number of balls of radius $\varepsilon$ needed to cover $A$.

Box-counting dimension is easier to estimate numerically than Hausdorff dimension. For "nice" fractal sets (self-similar attractors), $\dim_H = \dim_B$.

### 11.4.3 Information Dimension and the Kaplan-Yorke Conjecture

**Definition 11.4.4.** The *information dimension* of a measure $\mu$ is:
$$d_1(\mu) = \lim_{\varepsilon \to 0} \frac{\int \log \mu(B(x,\varepsilon))\,d\mu(x)}{\log \varepsilon}.$$

**Conjecture 11.4.5 (Kaplan-Yorke / Lyapunov Dimension).** For an SRB measure $\mu$ on a chaotic attractor with Lyapunov exponents $\lambda_1 \geq \lambda_2 \geq \cdots \geq \lambda_n$, the information dimension is:
$$d_1(\mu) = j + \frac{\lambda_1 + \cdots + \lambda_j}{|\lambda_{j+1}|},$$
where $j$ is the largest index with $\lambda_1 + \cdots + \lambda_j \geq 0$.

This conjecture (the *Kaplan-Yorke conjecture*) has been proven in several special cases (e.g., for Axiom A attractors by Ledrappier and Young) but remains open in general.

---

## 11.5 The Logistic Map: A Case Study

**The Family:** $f_\mu: [0,1] \to [0,1]$, $f_\mu(x) = \mu x(1-x)$, for $\mu \in [0, 4]$.

### 11.5.1 Complete Picture at $\mu = 4$

**Theorem 11.5.1.** $f_4$ is topologically conjugate to the tent map $T(x) = 1 - |2x-1|$ via $h(x) = \sin^2(\pi x/2)$ (or $h(x) = (2/\pi)\arcsin(\sqrt{x})$).

*Proof:* $f_4(h(\theta)) = 4 \sin^2(\pi\theta/2)(1 - \sin^2(\pi\theta/2)) = 4 \sin^2(\pi\theta/2)\cos^2(\pi\theta/2) = \sin^2(\pi\theta) = h(2\theta \mod 1) = h(T(\theta))$.

**Consequences of Conjugacy:**
- $f_4$ is topologically conjugate to the doubling map (via $T \sim$ doubling)
- Topological entropy: $h_{\text{top}}(f_4) = \log 2$
- Lyapunov exponent for Lebesgue-a.e. orbit: $\lambda = \log 2$
- Invariant measure: $(h_*)^{-1}$(Lebesgue) = $\frac{dx}{\pi\sqrt{x(1-x)}}$ (arcsine distribution)
- Dense periodic orbits, transitive, ergodic with respect to arcsine measure

### 11.5.2 The Parameter Space

- $\mu \in (0, 1)$: all orbits converge to $0$
- $\mu \in (1, 3)$: unique stable fixed point $x^* = 1 - 1/\mu$
- $\mu = 3$: Hopf/pitchfork bifurcation — fixed point loses stability, period-2 orbit born
- $\mu \in (3, 3.449\ldots)$: stable period-2 orbit
- $\mu_\infty \approx 3.5699\ldots$: onset of chaos
- $\mu \in (\mu_\infty, 4)$: chaotic behavior interspersed with periodic windows
- $\mu = 4$: fully developed chaos

---

## 11.6 Multifractal Analysis

Standard dimension theory assigns a single number to a set. Multifractal analysis studies the *spectrum of dimensions* within a dynamical measure.

**Definition 11.6.1.** For a measure $\mu$ on an attractor, the *local dimension* at $x$ is $\alpha(x) = \lim_{\varepsilon \to 0} \log \mu(B(x,\varepsilon)) / \log \varepsilon$.

The *multifractal spectrum* (or *$f(\alpha)$ spectrum*) is:
$$f(\alpha) = \dim_H\{x : \alpha(x) = \alpha\}.$$

**Theorem 11.6.2 (Ruelle, Pesin, Eckmann-Procaccia).** For hyperbolic attractors, $f(\alpha)$ is a concave function of $\alpha$, with maximum $f(\alpha_{\text{typ}}) = $ Hausdorff dimension of the attractor, achieved at the typical dimension $\alpha_{\text{typ}}$.

**Legendre Transform:** $f(\alpha)$ is related to the *Rényi dimension spectrum* $D_q$ (a function of the Rényi order $q$) via the Legendre transform:
$$D_q = \frac{1}{q-1} \inf_\alpha [q\alpha - f(\alpha) + 1].$$
Conversely $f(\alpha) = \inf_q [q\alpha - (q-1)D_q + 1]$.

This connects multifractal analysis directly to Rényi entropies (Chapter 17).

---

## 11.7 Chaos and Information Production

**Theorem 11.7.1.** For an ergodic dynamical system with positive Lyapunov exponent $\lambda > 0$:
- Nearby orbits separate at rate $e^{\lambda t}$: to predict the orbit at time $T$ to accuracy $\varepsilon$ from initial accuracy $\delta$, requires $\lambda T \leq \log(\delta/\varepsilon)$ — a finite prediction horizon.
- The system produces information at rate $\lambda$ bits per unit time (in appropriate units).
- Pesin's formula $h_\mu(f) = \sum_{\lambda_i > 0} \lambda_i$ identifies the entropy rate with the total information production.

**Definition 11.7.2 (Predictability Horizon).** For a system with maximal Lyapunov exponent $\lambda$, the *predictability horizon* (time at which prediction error reaches the attractor scale $L$ from initial error $\varepsilon$) is:
$$T_{\text{pred}} \approx \frac{1}{\lambda} \log\frac{L}{\varepsilon}.$$

For the atmosphere: $\lambda \approx 0.5/\text{day}$, $L/\varepsilon \approx 10^6$ gives $T_{\text{pred}} \approx 14 \log 10 / 0.5 \approx 14$ days — consistent with the "two-week barrier" for weather prediction.

---

## Exercises

**Exercise 11.1.** Show that the tent map $T(x) = 1 - |2x-1|$ on $[0,1]$ is Devaney chaotic by verifying all three conditions explicitly.

**Exercise 11.2.** (Hausdorff Dimension) Compute $\dim_H(C_\lambda)$ where $C_\lambda$ is the Cantor set formed by removing the middle $\lambda$-fraction at each stage. (*Hint:* Use the self-similarity: $C_\lambda$ is covered by $2^n$ intervals of length $((1-\lambda)/2)^n$.)

**Exercise 11.3.** For the Hénon map at $a = 1.4$, $b = 0.3$: compute numerically the two Lyapunov exponents $\lambda_1 > 0 > \lambda_2$ and estimate $\lambda_1 + \lambda_2$ (should be $\log |b| = \log 0.3 \approx -1.2$). Estimate the Kaplan-Yorke dimension.

**Exercise 11.4.** Show that the logistic map $f_4$ with the arcsine invariant measure satisfies Pesin's formula: compute $\int \log |f_4'(x)|\,d\mu_{\text{arc}}(x)$ and show it equals $\log 2$.

**Exercise 11.5.** (Multifractal) For a Bernoulli measure $\mu_p = (p, 1-p)$ on $\{0,1\}^{\mathbb N}$ (the doubling map attractor): show the local dimension of $\mu_p$ at a point $x$ with asymptotic frequency $\rho$ of 1s is $\alpha(x) = -\rho \log p - (1-\rho)\log(1-p)$. Compute $f(\alpha) = \dim_H\{\alpha(x) = \alpha\}$ and show it is the entropy function $-\rho\log\rho - (1-\rho)\log(1-\rho)$ (a Legendre transform of $\alpha$).

**Exercise 11.6.** Derive the Lorenz equations from the Navier-Stokes equations via the Galerkin truncation (Fourier modes). What does the truncation ignore?

**Exercise 11.7.** (Predictability) An ODE has maximal Lyapunov exponent $\lambda = 1/\text{day}$. You can measure initial conditions to accuracy $\varepsilon = 10^{-6}$ (in normalized units), and need prediction accuracy $L = 0.1$. What is the predictability horizon? By what factor would you need to improve measurement accuracy to double the predictability horizon?

---

## Chapter Notes

The Lorenz system (Section 11.2) is from Lorenz's 1963 paper *Deterministic Nonperiodic Flow* in the *Journal of Atmospheric Science* — one of the most cited scientific papers ever. Tucker's rigorous proof of the strange attractor (2002) uses interval arithmetic and is in *Foundations of Computational Mathematics*.

For fractal geometry: Falconer's *Fractal Geometry: Mathematical Foundations and Applications* is the standard text. For multifractal analysis, Pesin's *Dimension Theory in Dynamical Systems* is the mathematical treatment. Eckmann-Ruelle's *Ergodic theory of chaos and strange attractors* (*Reviews of Modern Physics*, 1985) is the key survey paper.

The connection between chaos and information theory (Section 11.7) is deep and explicit through Pesin's formula: the KS entropy equals the sum of positive Lyapunov exponents. This will be made even more precise in the ergodic information theory of Chapter 23.
