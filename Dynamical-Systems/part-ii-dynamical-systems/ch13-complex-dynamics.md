# Chapter 13 — Complex Dynamics

> *The Mandelbrot set is the most complex object in mathematics — infinitely intricate at every scale, yet defined by a single quadratic polynomial. Complex dynamics explains why.*

**Prerequisites:** Complex analysis (holomorphic functions, Riemann surfaces), Chapter 6 (topological dynamics), Chapter 11 (chaos, fractal dimension).

**What this chapter builds:** The Julia and Fatou sets as the fundamental dichotomy for complex iteration; the Mandelbrot set as parameter space; the No Wandering Domains theorem; quasiconformal surgery; renormalization in complex dynamics; and the connections to hyperbolic geometry and Teichmüller theory.

---

## 13.1 Iteration of Complex Maps

**Setup.** Let $f: \hat{{\mathbb C}} \to \hat{{\mathbb C}}$ be a rational map of degree $d \geq 2$ on the Riemann sphere $\hat{{\mathbb C}} = {\mathbb C} \cup \{\infty\}$.

**Definition 13.1.1.** The *Fatou set* $\mathcal{F}(f) \subseteq \hat{{\mathbb C}}$ is the largest open set on which the family of iterates $\{f^n\}_{n \geq 0}$ is *normal* (every subsequence has a locally uniformly convergent sub-subsequence). The *Julia set* is $\mathcal{J}(f) = \hat{{\mathbb C}} \setminus \mathcal{F}(f)$.

**Intuition:** The Fatou set is where nearby orbits behave similarly; the Julia set is where orbits diverge chaotically.

### 13.1.1 Basic Properties

**Theorem 13.1.2.**
1. $\mathcal{J}(f)$ is closed, nonempty, and $f(\mathcal{J}(f)) = \mathcal{J}(f)$ (invariant).
2. $\mathcal{J}(f)$ is perfect (no isolated points) — unless $\mathcal{J}(f) = \hat{{\mathbb C}}$.
3. $f|_{\mathcal{J}(f)}$ is topologically mixing.
4. Repelling periodic orbits are dense in $\mathcal{J}(f)$.
5. $\mathcal{J}(f)$ has empty interior, or $\mathcal{J}(f) = \hat{{\mathbb C}}$.

**Critical Points:** The critical points of $f$ (where $f' = 0$) are crucial. For degree $d$, there are $2d-2$ critical points (counted with multiplicity) on $\hat{{\mathbb C}}$.

**Theorem 13.1.3 (Fatou-Julia).** If all critical points of $f$ have bounded orbits (converge to attracting cycles or are in Siegel disks/Herman rings), then $\mathcal{J}(f)$ is connected. Otherwise, $\mathcal{J}(f)$ is a Cantor set.

---

## 13.2 Polynomial Dynamics

### 13.2.1 Filled Julia Sets

For a polynomial $p: {\mathbb C} \to {\mathbb C}$ of degree $d$:

**Definition 13.2.1.** The *filled Julia set* is $\mathcal{K}(p) = \{z \in {\mathbb C} : p^n(z) \not\to \infty\}$. The Julia set $\mathcal{J}(p) = \partial \mathcal{K}(p)$.

**Theorem 13.2.2.** $\mathcal{K}(p)$ is compact and $\mathcal{J}(p) = \mathcal{K}(p)$ iff $\mathcal{K}(p)$ has empty interior.

### 13.2.2 Böttcher Coordinates

For polynomials, infinity is a superattracting fixed point of degree $d$. The basin of attraction $\mathcal{A}(\infty) = \hat{{\mathbb C}} \setminus \mathcal{K}(p)$ is conformally equivalent to the exterior of the unit disk.

**Theorem 13.2.3 (Böttcher).** Near infinity, there is a unique conformal isomorphism (Böttcher coordinate):
$$\phi: \mathcal{A}(\infty) \xrightarrow{\sim} \hat{{\mathbb C}} \setminus \bar{{\mathbb D}}, \quad \phi(p(z)) = \phi(z)^d.$$

The *Green's function* $G_p(z) = \log|\phi(z)| = \lim_{n \to \infty} d^{-n} \log|p^n(z)|$ measures the "height" of $z$ above the filled Julia set.

**External Rays:** The preimages of radial lines $\{re^{2\pi i\theta} : r > 1\}$ under $\phi$ are *external rays* of angle $\theta$. For $\mathcal{J}(p)$ locally connected, external rays land on boundary points, providing a combinatorial model of $\mathcal{J}(p)$.

---

## 13.3 Quadratic Polynomials: $f_c(z) = z^2 + c$

The family $\{f_c : c \in {\mathbb C}\}$ is the simplest nontrivial family of polynomials.

**Definition 13.3.1.** $f_c$ has a unique critical point at $z = 0$. The *critical orbit* is $0 \mapsto c \mapsto c^2+c \mapsto \cdots$

**Dichotomy:** $\mathcal{K}(f_c)$ is connected iff the critical orbit is bounded. This gives the characterization of the Mandelbrot set.

### 13.3.1 The Mandelbrot Set

**Definition 13.3.2.** The *Mandelbrot set* is:
$$\mathcal{M} = \{c \in {\mathbb C} : \mathcal{K}(f_c) \text{ is connected}\} = \{c \in {\mathbb C} : f_c^n(0) \not\to \infty\}.$$

**Theorem 13.3.3 (Basic Properties of $\mathcal{M}$).**
1. $\mathcal{M}$ is compact, connected, and full (no holes).
2. $\mathcal{M}$ is symmetric under complex conjugation.
3. The *main cardioid* $\{c : f_c$ has an attracting fixed point$\}$ and the *period-2 bulb* are the most prominent regions.
4. The boundary $\partial\mathcal{M}$ has Hausdorff dimension $2$ (Shishikura's theorem).

**The MLC Conjecture:** Is $\mathcal{M}$ locally connected? This is one of the central open problems in complex dynamics. A positive answer would give a complete combinatorial description of the parameter space.

**Theorem 13.3.4 (Yoccoz, 1990).** $\mathcal{M}$ is locally connected at all *finitely renormalizable* parameters (parameters $c$ where $f_c$ is not infinitely renormalizable in any period).

---

## 13.4 Classification of Fatou Components

**Definition 13.4.1.** A *Fatou component* is a connected component of $\mathcal{F}(f)$. The possible types are:

1. *Attracting basin*: the basin of attraction of an attracting periodic cycle. $f^n \to$ the periodic orbit uniformly.
2. *Parabolic basin*: basin of a fixed point with eigenvalue $e^{2\pi i p/q}$ (neutral, rational). Orbit converges tangentially.
3. *Siegel disk*: a Fatou component on which $f$ is conformally conjugate to an irrational rotation. Requires Diophantine condition on the rotation number (see Siegel's linearization theorem).
4. *Herman ring*: an annulus on which $f$ is conjugate to an irrational rotation. (Only occurs for rational maps, not polynomials.)
5. *Böttcher domain*: basin of a superattracting cycle.

**Theorem 13.4.2 (Sullivan's No Wandering Domains, 1985).** There are no *wandering* Fatou components: every Fatou component is preperiodic (eventually maps into a periodic Fatou component of one of the five types above).

*(proof)* Uses *quasiconformal deformation theory*: any wandering domain would give a nontrivial deformation of $f$ in its quasiconformal conjugacy class, but Teichmüller theory shows these deformations are parameterized by a finite-dimensional space (determined by the critical points). A wandering domain provides an infinite-dimensional deformation — contradiction.

---

## 13.5 Quasiconformal Maps

Quasiconformal maps are the key tool for deforming complex dynamical systems.

**Definition 13.5.1.** A homeomorphism $\phi: U \to V$ between open sets in ${\mathbb C}$ is *$K$-quasiconformal* ($K \geq 1$) if it is ACL (absolutely continuous on lines) and $|\bar{\partial}\phi| \leq k |\partial\phi|$ a.e. where $k = (K-1)/(K+1) < 1$. Here $\partial = \frac{1}{2}(\frac{\partial}{\partial x} - i\frac{\partial}{\partial y})$ and $\bar{\partial} = \frac{1}{2}(\frac{\partial}{\partial x} + i\frac{\partial}{\partial y})$.

Conformal maps are $1$-quasiconformal.

**Theorem 13.5.2 (Measurable Riemann Mapping Theorem).** Let $\mu: {\mathbb C} \to {\mathbb D}$ be measurable with $\|\mu\|_\infty \leq k < 1$ (a *Beltrami coefficient*). Then there exists a unique quasiconformal homeomorphism $\phi: {\mathbb C} \to {\mathbb C}$ (fixing $0, 1, \infty$) solving the *Beltrami equation* $\bar{\partial}\phi = \mu \partial\phi$.

This theorem — the analytic core of Sullivan's proof and of most modern complex dynamics — allows one to *construct* conformal conjugacies by first finding quasiconformal ones.

---

## 13.6 Renormalization in Complex Dynamics

**Definition 13.6.1.** $f_c$ is *renormalizable at period $n$* if there exists a disk $U \ni 0$ such that $f_c^n: U \to U$ is polynomial-like (proper map of degree 2).

**Definition 13.6.2 (Douady-Hubbard Polynomial-Like Maps).** A *polynomial-like map* of degree $d$ is a proper holomorphic map $f: U \to V$ of degree $d$, where $U \Subset V$ are topological disks. The *filled Julia set* $K_f = \bigcap_n f^{-n}(V)$ is well-defined.

**Theorem 13.6.3 (Straightening Theorem).** Every polynomial-like map of degree $d$ is quasiconformally conjugate to a genuine polynomial of degree $d$.

**Renormalization Operator:** The renormalization operator sends $f_c$ to the polynomial-like map $f_c^n|_U$, straightened (via the Straightening Theorem) to a new polynomial $f_{c'}$. This defines a map $\mathcal{R}: \mathcal{M}_n \to \mathcal{M}$ from the set of period-$n$-renormalizable parameters to $\mathcal{M}$.

**Theorem 13.6.4 (Douady-Hubbard).** The "small Mandelbrot copies" inside $\mathcal{M}$ correspond exactly to renormalizable parameters. Each baby Mandelbrot is homeomorphic to $\mathcal{M}$ itself (via the renormalization operator).

---

## 13.7 Entropy of Complex Maps

**Theorem 13.7.1.** For a polynomial of degree $d$:
$$h_{\text{top}}(f|_{\mathcal{J}(f)}) = \log d.$$

For $f_c$ with $c \in \mathcal{M}$: $h_{\text{top}}(f_c|_{\mathcal{K}(f_c)}) = \log 2$ (since $\mathcal{K}$ is connected and $f_c$ is degree 2).

**Theorem 13.7.2 (Misiurewicz-Szlenk).** For a real quadratic $f_\mu: x \mapsto \mu x(1-x)$ viewed as a map on ${\mathbb C}$:
$$h_{\text{top}}(f_\mu) = \log(\text{leading coefficient of minimal polynomial of } f_\mu^n(0))$$
is a monotone function of $\mu$ on $[0, 4]$.

---

## Exercises

**Exercise 13.1.** Prove that the Julia set of $f_c(z) = z^2 + c$ is the boundary of the basin of attraction of $\infty$. Show that for $|c| > 2$, the Julia set is a Cantor set.

**Exercise 13.2.** For $c = 0$: $f_0(z) = z^2$. Compute the Julia set, filled Julia set, and Fatou set explicitly. Classify each Fatou component.

**Exercise 13.3.** For $c = -2$: $f_{-2}(z) = z^2 - 2$. Show the Julia set is the interval $[-2, 2] \subseteq {\mathbb R}$ and $f_{-2}$ is conjugate to the Chebyshev polynomial $T_2(\cos\theta) = \cos(2\theta)$ on $[-1,1]$. What is the topological entropy?

**Exercise 13.4.** (Mandelbrot) Show that if $|c| > 2$, then $c \notin \mathcal{M}$. (*Hint:* Show $|f_c^n(0)| \to \infty$.) Find the largest $r$ such that $\{|c| \leq r\} \subseteq \mathcal{M}$.

**Exercise 13.5.** Compute the Hausdorff dimension of the Julia set of $f_c$ for $c$ on the boundary of the main cardioid near $c = 0$ and near $c = -2$. (Use the Bowen formula: $\dim_H(\mathcal{J}(f_c)) = 1 + \lambda^2/(4 \log d) + O(\lambda^4)$ where $\lambda$ is the multiplier of the fixed point.)

**Exercise 13.6.** State Sullivan's No Wandering Domains theorem carefully. Why does the proof not apply to transcendental entire functions (like $e^z$)? (Indeed, wandering domains exist for $z \mapsto e^z$.)

---

## Chapter Notes

The foundational reference is Milnor's *Dynamics in One Complex Variable* — beautifully written, at just the right level, and free online. Beardon's *Iteration of Rational Functions* and Carleson-Gamelin's *Complex Dynamics* are the other standard texts.

For the Mandelbrot set specifically: Douady-Hubbard's original publications (the *Orsay Notes*) are available scanned. The MLC conjecture remains the central open problem — see Lyubich's survey articles.

Sullivan's No Wandering Domains theorem (1985) revolutionized complex dynamics by ending decades of uncertainty about Fatou component classification. The proof via the Measurable Riemann Mapping Theorem (Section 13.5) is in Sullivan's original paper in *Annals of Mathematics*.

Shishikura's theorem ($\dim_H(\partial\mathcal{M}) = 2$) is from his 1998 paper — a stunning result showing that while $\partial\mathcal{M}$ looks like a smooth curve, it is maximally rough in the fractal sense.
