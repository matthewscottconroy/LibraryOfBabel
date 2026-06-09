# Chapter 3 — Topology and Smooth Manifolds

> *A dynamical system is a map on a space. The topology of the space determines which behaviors are possible. The differential structure determines how fast they happen.*

**Prerequisites:** Chapter 1 (metric spaces, continuity).

**What this chapter builds:** The abstract topological framework for phase spaces; homotopy and fundamental groups for classifying spaces up to continuous deformation; smooth manifolds as the arenas for flows; differential forms and Stokes' theorem as the language of integration on manifolds; and the key fixed-point theorems that constrain dynamical behavior.

---

## 3.1 Topological Spaces

### 3.1.1 Definitions

**Definition 3.1.1.** A *topological space* is a pair $(X, \tau)$ where $X$ is a set and $\tau \subseteq 2^X$ (the *topology*, whose elements are *open sets*) satisfies:
1. $\emptyset, X \in \tau$
2. Arbitrary unions of elements of $\tau$ are in $\tau$
3. Finite intersections of elements of $\tau$ are in $\tau$

A set $F$ is *closed* if $X \setminus F$ is open.

**Examples 3.1.2.**
- The *discrete topology*: every subset is open. The *indiscrete topology*: only $\emptyset$ and $X$ are open.
- Any metric $(X, d)$ induces a topology: $U$ is open iff for every $x \in U$, some ball $B(x,r) \subseteq U$.
- The *Zariski topology* on ${\mathbb R}^n$: closed sets are zero sets of polynomials. This is important in algebraic geometry but produces non-Hausdorff spaces.

**Definition 3.1.3.** A topological space is:
- *Hausdorff (T₂)*: distinct points have disjoint open neighborhoods
- *Second countable*: there is a countable base for the topology
- *Separable*: there is a countable dense subset

A *Polish space* is a separable completely metrizable topological space. Most spaces in dynamics are Polish.

### 3.1.2 Continuous Maps

**Definition 3.1.4.** $f: X \to Y$ is *continuous* if $f^{-1}(U)$ is open in $X$ for every open $U \subseteq Y$. A *homeomorphism* is a continuous bijection with continuous inverse.

**Definition 3.1.5.** A *basis* for a topology $\tau$ is a collection $\mathcal{B} \subseteq \tau$ such that every open set is a union of elements of $\mathcal{B}$.

---

## 3.2 Compactness and Connectedness

**Definition 3.2.1.** A topological space $X$ is:
- *Compact*: every open cover has a finite subcover
- *Connected*: the only clopen (simultaneously open and closed) sets are $\emptyset$ and $X$
- *Path-connected*: for every $x, y \in X$ there exists a continuous $\gamma: [0,1] \to X$ with $\gamma(0) = x$, $\gamma(1) = y$
- *Locally connected* / *locally path-connected*: every point has a neighborhood base of connected / path-connected sets

**Theorem 3.2.2 (Tychonoff's Theorem).** An arbitrary product of compact spaces is compact (in the product topology).

*This requires the Axiom of Choice in its full generality. For countable products, a simpler argument using diagonal sequences suffices.*

**Theorem 3.2.3.** Let $f: X \to Y$ be continuous with $X$ compact. Then $f(X)$ is compact. If additionally $Y$ is Hausdorff and $f$ is injective, then $f$ is an embedding (homeomorphism onto its image).

**Key Example 3.2.4 (The Cantor Set).** The Cantor set $C = \{0,1\}^{\mathbb N}$ (binary sequences) with the product topology is a compact metrizable space. It is:
- Totally disconnected (no connected subset with more than one point)
- Perfect (closed, no isolated points)
- Homeomorphic to any compact metrizable totally disconnected perfect space (Brouwer's theorem)

The Cantor set is the prototypical compact invariant set in chaotic dynamics.

---

## 3.3 Homotopy and the Fundamental Group

### 3.3.1 Homotopy of Maps

**Definition 3.3.1.** Two continuous maps $f, g: X \to Y$ are *homotopic* (written $f \simeq g$) if there exists a continuous $H: X \times [0,1] \to Y$ with $H(x, 0) = f(x)$ and $H(x, 1) = g(x)$ for all $x$. The map $H$ is called a *homotopy*.

If $f, g: (X, x_0) \to (Y, y_0)$ are maps of pointed spaces and $H(x_0, t) = y_0$ for all $t$, the homotopy is *based*.

**Definition 3.3.2.** A continuous map $f: X \to Y$ is a *homotopy equivalence* if there exists $g: Y \to X$ with $g \circ f \simeq \text{id}_X$ and $f \circ g \simeq \text{id}_Y$. Spaces related by a homotopy equivalence have the same *homotopy type*.

### 3.3.2 The Fundamental Group

**Definition 3.3.3.** A *loop* based at $x_0 \in X$ is a continuous $\gamma: [0,1] \to X$ with $\gamma(0) = \gamma(1) = x_0$. The *fundamental group* $\pi_1(X, x_0)$ is the set of homotopy classes of loops based at $x_0$, with group operation given by concatenation:
$$(\gamma * \delta)(t) = \begin{cases} \gamma(2t) & t \leq 1/2 \\ \delta(2t - 1) & t \geq 1/2. \end{cases}$$

**Examples 3.3.4.**
- $\pi_1({\mathbb R}^n) = \{e\}$ (trivial) — ${\mathbb R}^n$ is simply connected.
- $\pi_1(S^1) \cong {\mathbb Z}$ — loops around the circle are classified by winding number.
- $\pi_1({\mathbb T}^2) \cong {\mathbb Z}^2$ for the 2-torus.
- $\pi_1(\Sigma_g) \cong \langle a_1, b_1, \ldots, a_g, b_g : [a_1,b_1]\cdots[a_g,b_g] = 1\rangle$ for a genus-$g$ surface.

**Application in Dynamics:** The fundamental group constrains which dynamical behaviors are possible. On $S^1$, the Poincaré rotation number is a homomorphism from $\pi_1$-invariant dynamics to ${\mathbb R}/{\mathbb Z}$. On surfaces, the Lefschetz Fixed Point Theorem uses the Euler characteristic (computable from $\pi_1$).

### 3.3.3 Higher Homotopy Groups

**Definition 3.3.5.** The *n-th homotopy group* $\pi_n(X, x_0)$ for $n \geq 1$ consists of homotopy classes of maps $(S^n, *) \to (X, x_0)$. For $n \geq 2$, $\pi_n$ is abelian.

**Remark 3.3.6 (Connection to HoTT).** From the HoTT perspective, the homotopy groups $\pi_n(X)$ are the *truncations* of the $\infty$-groupoid structure of the space $X$. The Hopf fibration $S^3 \to S^2$ is the generator of $\pi_3(S^2) \cong {\mathbb Z}$, and this is one of the basic computations that motivates the machinery of HoTT.

---

## 3.4 Covering Spaces

**Definition 3.4.1.** A *covering space* of $X$ is a space $\tilde{X}$ with a continuous surjection $p: \tilde{X} \to X$ such that every $x \in X$ has an open neighborhood $U$ with $p^{-1}(U) = \bigsqcup_\alpha V_\alpha$ where each $V_\alpha$ is mapped homeomorphically onto $U$ by $p$.

**Theorem 3.4.2 (Classification of Covering Spaces).** For a connected, locally path-connected, semi-locally simply connected space $X$ and basepoint $x_0$:
- There is a bijection between (isomorphism classes of) connected covering spaces and (conjugacy classes of) subgroups of $\pi_1(X, x_0)$.
- The *universal cover* $\tilde{X}$ corresponds to the trivial subgroup and satisfies $\pi_1(\tilde{X}) = \{e\}$.

**Application in Dynamics:** Covering space theory is used to study the lifting of dynamical systems. A map $f: X \to X$ lifts to a map $\tilde{f}: \tilde{X} \to \tilde{X}$ on the universal cover, and the algebraic properties of $\tilde{f}$ (e.g., on ${\mathbb R}$ when $X = S^1$) carry information about $f$'s rotation number and periodic orbit structure.

---

## 3.5 Smooth Manifolds

### 3.5.1 Definition and Charts

**Definition 3.5.1.** A *smooth $n$-manifold* is a Hausdorff second-countable topological space $M$ with a *smooth atlas*: a collection of homeomorphisms $\varphi_\alpha: U_\alpha \to \hat{U}_\alpha \subseteq {\mathbb R}^n$ (called *charts* or *coordinate maps*) with $M = \bigcup_\alpha U_\alpha$, such that the *transition maps* $\varphi_\beta \circ \varphi_\alpha^{-1}: \varphi_\alpha(U_\alpha \cap U_\beta) \to \varphi_\beta(U_\alpha \cap U_\beta)$ are smooth ($C^\infty$).

**Examples 3.5.2.**
- ${\mathbb R}^n$: a single chart
- $S^n$: the $n$-sphere, covered by two stereographic projection charts
- ${\mathbb T}^n = ({\mathbb R}/{\mathbb Z})^n$: the $n$-torus, locally homeomorphic to ${\mathbb R}^n$
- $GL(n, {\mathbb R})$: the group of invertible matrices, an open subset of ${\mathbb R}^{n^2}$

### 3.5.2 Tangent Bundle and Vector Fields

**Definition 3.5.3.** The *tangent space* $T_pM$ at a point $p \in M$ is the vector space of derivations on smooth functions at $p$, or equivalently the space of equivalence classes of smooth curves through $p$ (same first derivative in any chart). The *tangent bundle* is $TM = \bigsqcup_{p \in M} T_pM$ with its natural smooth structure.

**Definition 3.5.4.** A *vector field* $X$ on $M$ is a smooth section of $TM$: a smooth map $X: M \to TM$ with $X(p) \in T_pM$ for each $p$.

**Theorem 3.5.5.** A vector field $X$ on a compact smooth manifold $M$ generates a *flow* $\Phi: {\mathbb R} \times M \to M$ satisfying:
1. $\Phi(0, p) = p$ (identity at $t=0$)
2. $\Phi(t+s, p) = \Phi(t, \Phi(s, p))$ (group homomorphism property)
3. $\frac{d}{dt}\Big|_{t=0} \Phi(t, p) = X(p)$ (vector field generates the flow)

The maps $\Phi_t = \Phi(t, \cdot): M \to M$ are diffeomorphisms.

**Remark 3.5.6.** This theorem is essentially the Picard-Lindelöf theorem (Chapter 4) on manifolds. Compactness ensures the flow is *complete* (exists for all time). On non-compact manifolds, solutions may escape to infinity in finite time.

### 3.5.3 Lie Groups

**Definition 3.5.7.** A *Lie group* is a smooth manifold $G$ that is also a group, with smooth group operations. Its *Lie algebra* $\mathfrak{g} = T_eG$ carries the bracket $[X, Y] = XY - YX$ (as left-invariant vector fields).

**Examples 3.5.8.**
- $GL(n, {\mathbb R})$ with Lie algebra $\mathfrak{gl}(n) = M_n({\mathbb R})$
- $SO(n)$: rotations, Lie algebra $\mathfrak{so}(n)$ = skew-symmetric matrices
- $SL(n, {\mathbb R})$: matrices of determinant 1, Lie algebra = traceless matrices
- ${\mathbb T}^n = ({\mathbb R}/{\mathbb Z})^n$: the compact abelian Lie group

---

## 3.6 Differential Forms and de Rham Cohomology

### 3.6.1 Differential Forms

**Definition 3.6.1.** A *differential $k$-form* on a smooth manifold $M$ is a smooth section of $\bigwedge^k T^*M$ (the $k$-th exterior power of the cotangent bundle). In local coordinates $(x_1, \ldots, x_n)$:
$$\omega = \sum_{i_1 < \cdots < i_k} f_{i_1 \cdots i_k}\,dx_{i_1} \wedge \cdots \wedge dx_{i_k}.$$

The space of $k$-forms is $\Omega^k(M)$.

**Definition 3.6.2.** The *exterior derivative* $d: \Omega^k(M) \to \Omega^{k+1}(M)$ is the unique operator satisfying:
1. $d(f) = \sum_i \frac{\partial f}{\partial x_i} dx_i$ for functions ($0$-forms)
2. $d \circ d = 0$
3. $d(\omega \wedge \eta) = (d\omega) \wedge \eta + (-1)^k \omega \wedge (d\eta)$ for $\omega \in \Omega^k$

**Key examples:**
- $d(f\,dx) = \frac{\partial f}{\partial y}\,dy \wedge dx = -\frac{\partial f}{\partial y}\,dx \wedge dy$ in 2D
- On ${\mathbb R}^3$: $d$ applied to 0-forms gives the gradient; applied to 1-forms gives the curl; applied to 2-forms gives the divergence.

### 3.6.2 Stokes' Theorem and de Rham Cohomology

**Theorem 3.6.3 (Stokes' Theorem).** Let $M$ be a smooth oriented compact $n$-manifold with boundary $\partial M$. For any $(n-1)$-form $\omega$:
$$\int_M d\omega = \int_{\partial M} \omega.$$

This simultaneously generalizes the Fundamental Theorem of Calculus, Green's Theorem, Gauss's Theorem, and the classical Stokes' Theorem.

**Definition 3.6.4.** A form $\omega$ is *closed* if $d\omega = 0$; *exact* if $\omega = d\eta$ for some $\eta$. Since $d^2 = 0$, exact implies closed. The *de Rham cohomology* is
$$H^k_{\text{dR}}(M) = \frac{\ker(d: \Omega^k \to \Omega^{k+1})}{\text{im}(d: \Omega^{k-1} \to \Omega^k)}.$$

**Theorem 3.6.5 (de Rham's Theorem).** $H^k_{\text{dR}}(M) \cong H^k(M; {\mathbb R})$ (singular cohomology with ${\mathbb R}$ coefficients).

**Application in Dynamics:** The *Liouville measure* of a Hamiltonian system is a top-degree form $\omega^n$ (where $\omega$ is the symplectic form). Stokes' theorem implies $d(\omega^n) = 0$, so Liouville measure is preserved by the flow. The de Rham cohomology measures *topological obstructions* to finding global first integrals.

---

## 3.7 Fixed-Point Theorems

### 3.7.1 Brouwer's Theorem

**Theorem 3.7.1 (Brouwer Fixed Point Theorem).** Every continuous map $f: D^n \to D^n$ (the closed $n$-disk) has a fixed point.

*(proof sketch, $n=2$)* Suppose $f$ has no fixed point. Define $g(x) = $ the point on $\partial D^2$ where the ray from $f(x)$ through $x$ exits the disk. Then $g: D^2 \to \partial D^2 = S^1$ is a continuous retraction of the disk onto its boundary with $g|_{S^1} = \text{id}$. But $H_2(D^2) = 0$ while $H_1(S^1) = {\mathbb Z}$, and a retraction would give a contradiction on homology. (Alternatively: any map $D^2 \to D^2$ has fixed point by obstruction theory.)

**Corollary 3.7.2.** Every continuous map from a convex compact subset of ${\mathbb R}^n$ to itself has a fixed point.

### 3.7.2 Schauder Fixed Point Theorem

**Theorem 3.7.3 (Schauder).** Let $K$ be a compact convex subset of a Banach space, and $f: K \to K$ continuous. Then $f$ has a fixed point.

**Application:** Many proofs of invariant object existence (equilibria, periodic orbits, invariant measures) reduce to applying Brouwer or Schauder.

### 3.7.3 Poincaré-Hopf and Lefschetz Theorems

**Theorem 3.7.4 (Poincaré-Hopf).** Let $M$ be a compact smooth manifold and $V$ a smooth vector field with finitely many zeros. Then
$$\sum_{V(p)=0} \text{index}(V, p) = \chi(M),$$
where $\chi(M) = \sum_k (-1)^k \text{rank}(H^k(M))$ is the Euler characteristic.

**Corollary 3.7.5.** Every vector field on $S^{2n}$ has a zero. (Since $\chi(S^{2n}) = 2 \neq 0$.) But on $S^{2n+1}$ and ${\mathbb T}^n$ (where $\chi = 0$), nonzero vector fields can exist.

**Theorem 3.7.6 (Lefschetz Fixed Point Theorem).** Let $f: M \to M$ be a continuous map on a compact manifold. The *Lefschetz number*
$$L(f) = \sum_k (-1)^k \text{tr}(f_*: H^k(M; {\mathbb Q}) \to H^k(M; {\mathbb Q}))$$
satisfies: if $L(f) \neq 0$, then $f$ has a fixed point.

**Application:** The Lefschetz number counts fixed points (with sign/multiplicity) of a map. When $f$ is the identity, $L(\text{id}) = \chi(M)$. For a diffeomorphism $f: {\mathbb T}^2 \to {\mathbb T}^2$ given by a matrix $A \in SL(2, {\mathbb Z})$, $L(f) = \text{tr}(A)$ on $H^1$ contributes $- \text{tr}(A)$ to $L(f)$, giving $L(f) = 1 - \text{tr}(A) + 1 = 2 - \text{tr}(A)$. When $|\text{tr}(A)| > 2$, $A$ is hyperbolic and $L(f) \neq 0$, so $f$ has fixed points (which we can verify directly).

---

## 3.8 The Poincaré-Bendixson Theorem

**Theorem 3.8.1 (Poincaré-Bendixson).** Let $f: {\mathbb R}^2 \to {\mathbb R}^2$ be a $C^1$ vector field and $\gamma^+(p) = \{\Phi_t(p) : t \geq 0\}$ the positive orbit of $p$. Suppose $\gamma^+(p)$ is contained in a compact region with no equilibria. Then $\omega(p)$ (the omega-limit set) is a periodic orbit.

**Consequence:** In two dimensions, the only limit behaviors for bounded orbits are: fixed points, periodic orbits, or orbits connecting fixed points (homoclinic and heteroclinic connections). *Chaos is impossible in continuous-time 2D systems*.

This is why chaos requires either a 3-dimensional continuous-time system (Lorenz), a 2-dimensional discrete map (Hénon), or a 1-dimensional system with delay.

---

## Exercises

**Exercise 3.1.** Compute $\pi_1(X)$ for: (a) $X = {\mathbb R}^2 \setminus \{0\}$; (b) $X = {\mathbb T}^2$; (c) $X = {\mathbb R}P^2$ (the real projective plane); (d) $X = S^1 \vee S^1$ (the wedge sum of two circles).

**Exercise 3.2.** Show that $\chi(S^n) = 1 + (-1)^n$ using the CW-complex structure of $S^n$ with one 0-cell and one $n$-cell.

**Exercise 3.3.** Prove that the Poincaré-Hopf theorem implies the Brouwer Fixed Point Theorem. (*Hint:* A fixed-point-free map on $D^n$ defines a retraction onto $\partial D^n$; derive a contradiction using the Euler characteristics.)

**Exercise 3.4.** Let $A = \begin{pmatrix} 2 & 1 \\ 1 & 1 \end{pmatrix} \in SL(2,{\mathbb Z})$ and $f_A: {\mathbb T}^2 \to {\mathbb T}^2$ the induced map. Compute $L(f_A)$ and verify it equals the number of fixed points of $f_A$ (counted with appropriate signs).

**Exercise 3.5.** Verify Stokes' theorem for $\omega = x\,dy$ on the unit disk $D^2 \subseteq {\mathbb R}^2$.

**Exercise 3.6.** Let $\omega = -y\,dx/(x^2+y^2) + x\,dy/(x^2+y^2)$ on ${\mathbb R}^2 \setminus \{0\}$. Show that $d\omega = 0$ but $\omega$ is not exact. (*Hint:* Compute $\int_{S^1} \omega$.) Explain why this is consistent with the Poincaré Lemma.

**Exercise 3.7.** (Application to Dynamics) Use the Poincaré-Hopf theorem to prove that every vector field on $S^2$ must have at least one zero. Interpret this as: *you can't comb a hairy ball flat* (Hairy Ball Theorem).

**Exercise 3.8.** Prove that ${\mathbb T}^n = ({\mathbb R}/{\mathbb Z})^n$ admits a nonvanishing vector field. Construct an explicit flow $\Phi_t$ on ${\mathbb T}^2$ with irrational rotation (every orbit dense) and compute its Euler characteristic.

---

## Chapter Notes

The foundational topology material is in Munkres' *Topology* (Chapters 2, 3, 7, 9). For smooth manifolds, Lee's *Introduction to Smooth Manifolds* is the definitive modern text; Guillemin and Pollack's *Differential Topology* offers a more geometric and problem-centered approach. Milnor's *Topology from the Differentiable Viewpoint* is a beautiful short book that covers transversality, degree theory, and the Hopf theorem.

Poincaré-Hopf (Section 3.7.3) connects topology to dynamics by counting zeros of vector fields. The Lefschetz theorem (Theorem 3.7.6) is the bridge to algebraic topology: it counts fixed points of maps via cohomology. The Nielsen fixed-point theorem (not covered here) gives a stronger count using the fundamental group — see Chapters 6 and 9 for applications.

For the connection to HoTT: every manifold is a type, paths are homotopies, and the homotopy groups $\pi_n(M)$ are the higher inductive types of that type. The de Rham complex is a model for the cohomology of the $\infty$-topos of $M$.
