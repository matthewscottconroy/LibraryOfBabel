# Chapter 4 — Ordinary Differential Equations

> *A flow is a group homomorphism from time into diffeomorphisms. An ODE is the infinitesimal generator of this group. Mastering ODEs means learning to think in flows.*

**Prerequisites:** Chapters 1 (metric spaces, Banach spaces, Contraction Mapping Theorem), 3 (smooth manifolds, tangent bundles).

**What this chapter builds:** Existence and uniqueness of solutions (Picard-Lindelöf, Carathéodory); the global perspective of the *flow* as a family of diffeomorphisms; linear systems and their spectral classification; the Hartman-Grobman theorem (linearization at hyperbolic equilibria); stable and unstable manifolds; and the Poincaré-Bendixson theory for planar systems.

---

## 4.1 Existence and Uniqueness

### 4.1.1 The Initial Value Problem

**Setup.** Let $U \subseteq {\mathbb R}^n$ be open and $f: U \to {\mathbb R}^n$ a smooth (or merely Lipschitz) vector field. The *initial value problem (IVP)* is:
$$\dot{x} = f(x), \quad x(0) = x_0 \in U.$$

A *solution* on an interval $I \ni 0$ is a differentiable map $\varphi: I \to U$ satisfying $\dot{\varphi}(t) = f(\varphi(t))$ for all $t \in I$ and $\varphi(0) = x_0$.

**Integral Formulation.** The IVP is equivalent to the integral equation:
$$\varphi(t) = x_0 + \int_0^t f(\varphi(s))\,ds.$$

This is the key reformulation: solutions are *fixed points* of the operator $T[\varphi](t) = x_0 + \int_0^t f(\varphi(s))\,ds$.

### 4.1.2 Picard-Lindelöf Theorem

**Definition 4.1.1.** $f: U \to {\mathbb R}^n$ is *locally Lipschitz* if for each compact $K \subseteq U$ there is $L = L(K) > 0$ with $\|f(x) - f(y)\| \leq L\|x - y\|$ for all $x, y \in K$.

*Note:* $C^1$ vector fields are locally Lipschitz, so Picard-Lindelöf applies to all $C^1$ vector fields.

**Theorem 4.1.2 (Picard-Lindelöf / Cauchy-Lipschitz).** Let $f: U \to {\mathbb R}^n$ be locally Lipschitz with Lipschitz constant $L$ on $\bar{B}(x_0, r)$. Set $M = \sup_{x \in \bar{B}(x_0,r)} \|f(x)\|$ and $T = \min(r/M, 1/(2L))$. Then there exists a unique solution $\varphi: [-T, T] \to \bar{B}(x_0, r)$ to the IVP.

*(proof)* The operator $T[\varphi](t) = x_0 + \int_0^t f(\varphi(s))\,ds$ maps the complete metric space $\mathcal{X} = \{\varphi \in C([-T,T], \bar{B}(x_0,r))\}$ to itself (for small enough $T$), and is a contraction:
$$\|T[\varphi] - T[\psi]\|_\infty \leq L \cdot T \cdot \|\varphi - \psi\|_\infty \leq \frac{1}{2}\|\varphi - \psi\|_\infty.$$
The Banach Fixed Point Theorem gives the unique fixed point.

**Theorem 4.1.3 (Maximal Solutions).** Under the hypotheses of Picard-Lindelöf, for each $x_0 \in U$ there exists a unique *maximal solution* $\varphi: (t^-, t^+) \to U$ (with $-\infty \leq t^- < 0 < t^+ \leq +\infty$) that cannot be extended to a larger interval. If $t^+ < \infty$, the solution *escapes to infinity*: $\|\varphi(t)\| \to \infty$ or $\varphi(t) \to \partial U$ as $t \nearrow t^+$.

**Corollary 4.1.4.** On compact manifolds, every smooth vector field generates a *complete flow* (defined for all $t \in {\mathbb R}$).

### 4.1.3 Dependence on Initial Conditions and Parameters

**Theorem 4.1.5 (Smooth Dependence).** Let $f: U \to {\mathbb R}^n$ be $C^k$ ($k \geq 1$). Then the solution $\varphi(t, x_0)$ is $C^k$ jointly in $(t, x_0)$. The derivative $D_{x_0}\varphi(t, x_0)$ satisfies the *variational equation* (matrix ODE):
$$\frac{d}{dt} D_{x_0}\varphi = Df(\varphi(t, x_0)) \cdot D_{x_0}\varphi, \quad D_{x_0}\varphi(0) = I.$$

*This is the fundamental theorem connecting flows and linearizations.*

---

## 4.2 The Flow as a Family of Diffeomorphisms

**Definition 4.2.1.** The *flow* of a complete vector field $f: M \to TM$ is the map $\Phi: {\mathbb R} \times M \to M$ defined by $\Phi(t, p) = \varphi_p(t)$ where $\varphi_p$ is the unique solution with initial condition $p$.

**Proposition 4.2.2 (Flow Properties).**
1. $\Phi_0 = \text{id}_M$
2. $\Phi_t \circ \Phi_s = \Phi_{t+s}$ for all $s, t \in {\mathbb R}$ (group homomorphism: ${\mathbb R} \to \text{Diff}(M)$)
3. $\Phi_t$ is a diffeomorphism for each fixed $t$
4. $\frac{d}{dt}\Big|_{t=0} \Phi_t(p) = f(p)$ (the vector field is the infinitesimal generator)

*The group property (2) follows from uniqueness: $t \mapsto \Phi_{t+s}(p)$ and $t \mapsto \Phi_t(\Phi_s(p))$ both satisfy $\dot{x} = f(x)$ with initial condition $\Phi_s(p)$.*

**Remark 4.2.3.** Property (2) is called the *cocycle property* or *1-cocycle condition*. Discrete dynamical systems $f: M \to M$ have an analogue: the iterates $f^n$ satisfy $f^{m+n} = f^m \circ f^n$ (using integer time instead of real time).

---

## 4.3 Linear Systems

### 4.3.1 The Matrix Exponential

The linear IVP $\dot{x} = Ax$, $x(0) = x_0$ (with $A \in M_n({\mathbb R})$) has the explicit solution:
$$x(t) = e^{tA} x_0, \quad \text{where } e^{tA} = \sum_{k=0}^\infty \frac{t^k A^k}{k!}.$$

**Properties of the Matrix Exponential:**
- $e^{0} = I$
- $\frac{d}{dt} e^{tA} = A e^{tA}$
- $e^{(t+s)A} = e^{tA} e^{sA}$
- $\det(e^{tA}) = e^{t \cdot \text{tr}(A)}$ (Jacobi's formula)
- If $A = PJP^{-1}$ (Jordan form), then $e^{tA} = Pe^{tJ}P^{-1}$

### 4.3.2 Stability of Linear Systems

**Definition 4.3.1.** The equilibrium $x = 0$ of $\dot{x} = Ax$ is:
- *stable* if for all $\varepsilon > 0$ there exists $\delta > 0$: $\|x_0\| < \delta \Rightarrow \|e^{tA}x_0\| < \varepsilon$ for all $t \geq 0$
- *asymptotically stable* if stable and $e^{tA}x_0 \to 0$ as $t \to \infty$ for all initial conditions
- *exponentially stable* if $\|e^{tA}\| \leq Ce^{-\lambda t}$ for some $C, \lambda > 0$

**Theorem 4.3.2 (Stability Classification of Linear Systems).** Let $\sigma(A) = \{\lambda_1, \ldots, \lambda_n\}$ be the spectrum of $A$ (eigenvalues counted with multiplicity).

- $x = 0$ is **exponentially stable** iff all $\text{Re}(\lambda_i) < 0$
- $x = 0$ is **stable** iff all $\text{Re}(\lambda_i) \leq 0$ and eigenvalues with $\text{Re}(\lambda_i) = 0$ have Jordan blocks of size 1
- $x = 0$ is **unstable** otherwise

**Definition 4.3.3.** $A \in M_n({\mathbb R})$ is *hyperbolic* if no eigenvalue has zero real part, i.e., $\sigma(A) \cap i{\mathbb R} = \emptyset$.

For hyperbolic $A$, ${\mathbb R}^n = E^s \oplus E^u$ where $E^s$ = span of generalized eigenvectors for eigenvalues with $\text{Re}(\lambda) < 0$ (stable subspace) and $E^u$ = span of those with $\text{Re}(\lambda) > 0$ (unstable subspace).

---

## 4.4 Linearization and the Hartman-Grobman Theorem

### 4.4.1 Equilibria and Linearization

**Definition 4.4.1.** A point $p \in M$ is an *equilibrium* (or *fixed point*) of $\dot{x} = f(x)$ if $f(p) = 0$.

Near a equilibrium $p$, Taylor expand: $f(x) = Df(p)(x-p) + O(\|x-p\|^2)$. The *linearization* at $p$ is the linear system $\dot{y} = Df(p) y$ (where $y = x - p$).

**Question:** To what extent does the linearization determine the behavior of the nonlinear system near $p$?

**Definition 4.4.2.** An equilibrium $p$ is *hyperbolic* if $Df(p)$ is a hyperbolic matrix (all eigenvalues have nonzero real part).

### 4.4.2 Hartman-Grobman Theorem

**Theorem 4.4.3 (Hartman-Grobman).** Let $p$ be a hyperbolic equilibrium of the $C^1$ vector field $f$. Then there exists a homeomorphism $h: U \to V$ (between neighborhoods of $p$ and $0$) that *conjugates* the nonlinear flow to the linear flow:
$$h \circ \Phi_t = e^{tA} \circ h \quad \text{on } U \cap \Phi_{[-T,T]}(U)$$
where $A = Df(p)$.

*In words: near a hyperbolic equilibrium, the nonlinear flow is topologically conjugate to its linearization.*

**Remark 4.4.4.** The homeomorphism $h$ is in general not differentiable at $p$. Smooth conjugacy requires Sternberg's theorem and nonresonance conditions on the eigenvalues.

### 4.4.3 Stable and Unstable Manifolds

**Theorem 4.4.5 (Stable Manifold Theorem / Hadamard-Perron).** Let $p$ be a hyperbolic equilibrium with $A = Df(p)$ having stable subspace $E^s$ and unstable subspace $E^u$. Then there exist:
- A *local stable manifold* $W^s_{\text{loc}}(p)$: a $C^1$ submanifold tangent to $E^s$ at $p$, with $\Phi_t(x) \to p$ exponentially as $t \to +\infty$ for $x \in W^s_{\text{loc}}(p)$.
- A *local unstable manifold* $W^u_{\text{loc}}(p)$: tangent to $E^u$ at $p$, with $\Phi_t(x) \to p$ exponentially as $t \to -\infty$ for $x \in W^u_{\text{loc}}(p)$.

The *global stable manifold* is $W^s(p) = \bigcup_{t \leq 0} \Phi_t(W^s_{\text{loc}}(p))$ and similarly for $W^u(p)$.

*(proof sketch)* The stable manifold is the fixed point of a graph transform operator. Write the flow in the splitting $E^s \oplus E^u$. A graph over $E^s$ is mapped to a graph under the flow if and only if it is a fixed point of a certain contraction. The contraction mapping theorem gives the unique fixed point, which is the stable manifold.

**Application:** The stable and unstable manifolds of hyperbolic fixed points are the "skeleton" around which dynamics organizes. Their intersections (homoclinic and heteroclinic points) generate chaos (Chapter 9).

---

## 4.5 The Center Manifold Theorem

When an equilibrium is *not* hyperbolic (some eigenvalues on the imaginary axis), the linear analysis is inconclusive. The center manifold captures the nonlinear dynamics on the critical modes.

**Definition 4.5.1.** For $A = Df(p)$ with eigenvalues of zero, negative, and positive real part, decompose ${\mathbb R}^n = E^c \oplus E^s \oplus E^u$ (center, stable, unstable subspaces).

**Theorem 4.5.2 (Center Manifold Theorem).** There exists a $C^k$ *center manifold* $W^c_{\text{loc}}(p)$ tangent to $E^c$ at $p$, invariant under the flow. The long-time dynamics near $p$ is governed by the restriction of $f$ to $W^c$.

**Application:** Bifurcation analysis (Chapter 10) takes place on the center manifold. When parameters vary, the eigenvalues move through the imaginary axis, and the center manifold theorem reduces the infinite-dimensional (or high-dimensional) bifurcation problem to a low-dimensional one.

---

## 4.6 Phase Portraits and Qualitative Analysis

### 4.6.1 Equilibria, Limit Cycles, and Heteroclinic Orbits

**Definition 4.6.1.** For a flow $\Phi_t$ on a phase space $M$:
- The *orbit* of $p$: $\mathcal{O}(p) = \{\Phi_t(p) : t \in {\mathbb R}\}$
- The *omega-limit set*: $\omega(p) = \bigcap_{T>0} \overline{\{\Phi_t(p) : t > T\}}$ (accumulation points as $t \to +\infty$)
- The *alpha-limit set*: $\alpha(p) = \bigcap_{T>0} \overline{\{\Phi_t(p) : t < -T\}}$ (as $t \to -\infty$)
- A *periodic orbit* (limit cycle): $\Phi_T(p) = p$ for some minimal $T > 0$
- A *homoclinic orbit*: an orbit in $W^s(p) \cap W^u(p)$ for the same equilibrium $p$
- A *heteroclinic orbit*: an orbit in $W^s(q) \cap W^u(p)$ for distinct equilibria $p, q$

**Properties of Omega-Limit Sets:**
1. $\omega(p)$ is closed and positively invariant (the flow maps it to itself)
2. $\omega(p)$ is connected if the orbit of $p$ is bounded
3. $\omega(p) = \emptyset$ iff the orbit escapes to infinity

### 4.6.2 Poincaré Maps

**Definition 4.6.2.** Let $\Sigma$ be a smooth hypersurface transverse to the flow near a periodic orbit $\gamma$. The *Poincaré return map* is $P: \Sigma \to \Sigma$ defined by $P(x) = \Phi_{\tau(x)}(x)$, where $\tau(x)$ is the first return time.

Studying the periodic orbit $\gamma$ reduces to studying the fixed point of $P$. The stability of $\gamma$ is determined by the eigenvalues of $DP(p)$ at the fixed point $p = \gamma \cap \Sigma$ — these are the *Floquet multipliers*.

---

## 4.7 Gradient and Hamiltonian Systems

### 4.7.1 Gradient Systems

**Definition 4.7.1.** A *gradient system* on a Riemannian manifold $(M, g)$ is $\dot{x} = -\text{grad}_g(V)(x)$ for a smooth *potential function* $V: M \to {\mathbb R}$.

**Properties:**
- $V$ decreases along orbits: $\frac{d}{dt} V(\Phi_t(x)) = -\|\text{grad}(V)\|^2 \leq 0$
- Omega-limit sets consist of equilibria: $\omega(p) \subseteq \{q : \text{grad}(V)(q) = 0\}$ (by LaSalle's invariance principle)
- Gradient systems have no periodic orbits (since $V$ is strictly decreasing along non-constant orbits)

### 4.7.2 Hamiltonian Systems

**Definition 4.7.2.** A *Hamiltonian system* on a symplectic manifold $(M, \omega)$ is determined by a smooth function $H: M \to {\mathbb R}$ (the Hamiltonian). The vector field $X_H$ satisfies $\omega(X_H, \cdot) = dH$.

In local Darboux coordinates $(q_1, \ldots, q_n, p_1, \ldots, p_n)$ with $\omega = \sum_i dq_i \wedge dp_i$:
$$\dot{q}_i = \frac{\partial H}{\partial p_i}, \quad \dot{p}_i = -\frac{\partial H}{\partial q_i}.$$

**Conservation Laws:**
- $H$ is conserved: $\frac{d}{dt} H = 0$ (energy conservation)
- *Liouville's theorem*: the flow preserves phase space volume (the Liouville measure $\omega^n$)

---

## Exercises

**Exercise 4.1.** Use Picard iteration to find the solution of $\dot{x} = x^2$, $x(0) = 1$. Show the solution blows up in finite time. What does this illustrate about maximal solutions?

**Exercise 4.2.** Classify all equilibria of the harmonic oscillator $\ddot{x} + x = 0$ (written as a planar system). Compute the matrix exponential $e^{tA}$ and draw the phase portrait.

**Exercise 4.3.** For $\dot{x} = -x + x^3$: (a) find all equilibria; (b) classify them (using the Jacobian); (c) draw the phase portrait on ${\mathbb R}$; (d) find the stable and unstable manifolds explicitly.

**Exercise 4.4.** The van der Pol oscillator: $\dot{x} = y$, $\dot{y} = \mu(1-x^2)y - x$ for $\mu > 0$.
(a) Show that the origin is an unstable equilibrium.
(b) Argue (without computing explicitly) that there must be a limit cycle. (*Hint:* Use Poincaré-Bendixson and construct an annular trapping region.)

**Exercise 4.5.** (Variational Equations) For $\dot{x} = f(x)$ with flow $\Phi_t$, prove that $J(t) = D_{x_0}\Phi_t$ satisfies $\dot{J} = Df(\Phi_t(x_0)) J$ with $J(0) = I$. Compute $\det(J(t))$ using the formula $\frac{d}{dt}\det(J) = \text{tr}(Df) \cdot \det(J)$.

**Exercise 4.6.** Show that a 2D Hamiltonian system $\dot{q} = \partial H/\partial p$, $\dot{p} = -\partial H/\partial q$ cannot have asymptotically stable equilibria. (*Hint:* Use Liouville's theorem: the flow preserves area, so volumes cannot contract.)

**Exercise 4.7.** (Center Manifold) Consider $\dot{x} = xy$, $\dot{y} = -y + x^2$ near the origin. The linearization at $(0,0)$ has eigenvalues $0$ and $-1$. The center manifold has the form $y = h(x)$ for small $x$. Find $h(x)$ to second order by substituting into the invariance equation.

**Exercise 4.8.** Let $f: {\mathbb T}^2 \to {\mathbb T}^2$ be the flow of $\dot{\theta}_1 = 1$, $\dot{\theta}_2 = \alpha$. Show every orbit is dense iff $\alpha \in {\mathbb R} \setminus {\mathbb Q}$. If $\alpha \in {\mathbb Q}$, show every orbit is periodic.

---

## Chapter Notes

For the classical theory of ODEs, Arnold's *Ordinary Differential Equations* is essential — it develops the geometric viewpoint (flows, phase portraits, structural stability) rather than focusing on solution formulas. Hirsch, Smale, and Devaney's *Differential Equations, Dynamical Systems, and an Introduction to Chaos* is the modern textbook that connects ODE theory to dynamical systems. Perko's *Differential Equations and Dynamical Systems* is a thorough treatment at the graduate level.

The Stable Manifold Theorem (Theorem 4.4.5) is proven via the *Hadamard graph transform method* in Katok-Hasselblatt's *Introduction to the Modern Theory of Dynamical Systems* (Appendix 4). The center manifold theorem is in Carr's *Applications of Centre Manifold Theory* and in Guckenheimer-Holmes' *Nonlinear Oscillations, Dynamical Systems, and Bifurcations of Vector Fields*.

The connection between the flow $\Phi_t$ and the vector field $f$ is the starting point for the theory of Lie groups: the exponential map $\exp: \mathfrak{g} \to G$ sends a Lie algebra element (infinitesimal generator) to a group element (one-parameter subgroup). Chapter 14 develops this further in the Hamiltonian setting.
