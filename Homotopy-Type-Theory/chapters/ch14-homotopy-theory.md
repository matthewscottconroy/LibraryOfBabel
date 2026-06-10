# Chapter 14: Homotopy Theory

## Introduction

Homotopy theory asks: when are two topological spaces "essentially the same" for the purposes of algebraic invariants? The answer is: when they are *homotopy equivalent* — related by maps that are "continuously deformable" into inverses of each other, even if they are not homeomorphic.

This chapter develops the core of homotopy theory: homotopy between maps, the fundamental group, covering spaces, higher homotopy groups, and fibrations. These are the classical concepts that HoTT axiomatizes synthetically. Understanding them concretely — with specific examples like the circle, the torus, and lens spaces — gives intuition for what the abstract constructions in HoTT are capturing.

---

## 1. Homotopy Between Maps

**Definition 14.1 (Homotopy).** A *homotopy* between continuous maps $f, g : X \to Y$ is a continuous map:
$$H : X \times [0,1] \to Y$$
with $H(x, 0) = f(x)$ and $H(x, 1) = g(x)$ for all $x \in X$.

We write $f \simeq g$ when $f$ and $g$ are homotopic. Homotopy is an equivalence relation.

**Intuition:** $H$ is a continuous "deformation" of $f$ into $g$. Think of $H(-, t)$ as a movie: at $t = 0$ it shows $f$, at $t = 1$ it shows $g$, and in between, it continuously morphs $f$ into $g$.

**Example 14.2.** Any two maps $f, g : X \to \mathbb{R}^n$ are homotopic via the straight-line homotopy $H(x, t) = (1-t)f(x) + tg(x)$. So the set of homotopy classes $[X, \mathbb{R}^n]$ has exactly one element for any $X$.

**Example 14.3.** Maps $f, g : X \to S^1$ are homotopic iff they have the same *degree* (winding number). So $[X, S^1]$ is nontrivial — and computable.

**Definition 14.4 (Homotopy Equivalence).** A map $f : X \to Y$ is a *homotopy equivalence* if there exists $g : Y \to X$ with $g \circ f \simeq \mathsf{id}_X$ and $f \circ g \simeq \mathsf{id}_Y$. We write $X \simeq Y$.

**Example 14.5.** $\mathbb{R}^n \simeq \{*\}$ (a point): the inclusion $\iota : \{*\} \hookrightarrow \mathbb{R}^n$ and the constant map $c : \mathbb{R}^n \to \{*\}$ are homotopy inverses. The homotopy $H(x, t) = (1-t)x$ contracts $\mathbb{R}^n$ to the origin.

**Example 14.6.** The cylinder $S^1 \times [0,1] \simeq S^1$: include $S^1$ as the bottom, then project back.

**Example 14.7.** A graph $\Gamma$ is homotopy equivalent to a wedge of circles. The number of circles equals $e - v + 1$ (edges minus vertices plus 1, the *Euler characteristic* term).

---

## 2. The Fundamental Group

### 2.1 Paths and Loops

**Definition 14.8.** A *path* in $X$ from $x_0$ to $x_1$ is a continuous function $\gamma : [0,1] \to X$ with $\gamma(0) = x_0$ and $\gamma(1) = x_1$.

A *loop* at $x_0$ is a path with $\gamma(0) = \gamma(1) = x_0$.

Two paths $\gamma_0, \gamma_1$ from $x_0$ to $x_1$ are *path-homotopic* (or *homotopic relative endpoints*) if there is a homotopy $H : [0,1] \times [0,1] \to X$ with $H(s, 0) = \gamma_0(s)$, $H(s, 1) = \gamma_1(s)$, $H(0, t) = x_0$, $H(1, t) = x_1$ for all $t$.

The equivalence class of a path $\gamma$ is written $[\gamma]$.

### 2.2 Group Structure

**Concatenation:** Given $\gamma_1 : x_0 \to x_1$ and $\gamma_2 : x_1 \to x_2$:
$$(\gamma_1 \cdot \gamma_2)(s) = \begin{cases} \gamma_1(2s) & 0 \leq s \leq 1/2 \\ \gamma_2(2s-1) & 1/2 \leq s \leq 1 \end{cases}$$

**Reversal:** $\bar{\gamma}(s) = \gamma(1-s)$ (traverse in reverse).

**Constant loop:** $c_{x_0}(s) = x_0$.

**Theorem 14.9.** The set of homotopy classes of loops at $x_0$, with the operation $[\gamma_1] \cdot [\gamma_2] = [\gamma_1 \cdot \gamma_2]$, forms a group. This is the *fundamental group* $\pi_1(X, x_0)$.

*Verification:*
- *Well-defined:* If $\gamma_1 \simeq \gamma_1'$ and $\gamma_2 \simeq \gamma_2'$ (relative endpoints), then $\gamma_1 \cdot \gamma_2 \simeq \gamma_1' \cdot \gamma_2'$.
- *Associativity:* $[(\gamma_1 \cdot \gamma_2) \cdot \gamma_3] = [\gamma_1 \cdot (\gamma_2 \cdot \gamma_3)]$ — prove via a "reparametrization homotopy."
- *Identity:* $[c_{x_0}]$ is the identity: $c_{x_0} \cdot \gamma \simeq \gamma$ via a reparametrization.
- *Inverse:* $[\gamma] \cdot [\bar{\gamma}] = [c_{x_0}]$ — the loop $\gamma \cdot \bar{\gamma}$ is homotopic to the constant loop.

### 2.3 Dependence on Basepoint

**Theorem 14.10.** If $X$ is path-connected and $x_0, x_1 \in X$, then $\pi_1(X, x_0) \cong \pi_1(X, x_1)$ (non-canonically). The isomorphism depends on a choice of path from $x_0$ to $x_1$.

For this reason, we often write $\pi_1(X)$ for path-connected $X$, understanding that the group is determined up to isomorphism.

**Functoriality:** A continuous map $f : X \to Y$ with $f(x_0) = y_0$ induces a group homomorphism $f_* : \pi_1(X, x_0) \to \pi_1(Y, y_0)$ via $f_*([\gamma]) = [f \circ \gamma]$.

**Theorem 14.11.** $\pi_1 : \mathbf{Top}_* \to \mathbf{Grp}$ is a functor (on the category of pointed topological spaces).

---

## 3. Computing $\pi_1$: Van Kampen's Theorem

**Theorem 14.12 (Seifert-van Kampen Theorem).** Let $X = U \cup V$ where $U, V$ are open, path-connected, and $U \cap V$ is also path-connected. Choose a basepoint $x_0 \in U \cap V$. Then:
$$\pi_1(X, x_0) \cong \pi_1(U, x_0) *_{\pi_1(U \cap V, x_0)} \pi_1(V, x_0)$$
(the *amalgamated free product* — pushout of groups).

More precisely: the inclusions $i_U : U \cap V \hookrightarrow U$ and $i_V : U \cap V \hookrightarrow V$ induce homomorphisms $(i_U)_*$ and $(i_V)_*$, and $\pi_1(X)$ is their pushout in $\mathbf{Grp}$:
$$\pi_1(U) *_{\pi_1(U \cap V)} \pi_1(V) = (\pi_1(U) * \pi_1(V)) / N$$
where $N$ is the normal subgroup generated by $\{(i_U)_*(g) \cdot (i_V)_*(g)^{-1} \mid g \in \pi_1(U \cap V)\}$.

**Example 14.13 ($\pi_1(S^1) = \mathbb{Z}$).**

*Proof using van Kampen:*
- Let $U = S^1 \setminus \{N\}$ (remove north pole): $U \cong \mathbb{R}$, contractible, $\pi_1(U) = 0$.
- Let $V = S^1 \setminus \{S\}$ (remove south pole): $V \cong \mathbb{R}$, contractible, $\pi_1(V) = 0$.
- $U \cap V \cong \mathbb{R} \sqcup \mathbb{R}$: two disjoint arcs. Path-connected? No! (The two arcs are disconnected.)

Better decomposition: use $U$ and $V$ as slightly thickened hemispheres:
- $U = \{e^{i\theta} \mid -\epsilon < \theta < \pi + \epsilon\}$ (upper arc)
- $V = \{e^{i\theta} \mid \pi - \epsilon < \theta < 2\pi + \epsilon\}$ (lower arc)
- $U \cap V = $ two small arcs, homotopic to two points.

The van Kampen theorem with $U \cap V$ disconnected requires care; the correct computation uses the *groupoid version* of van Kampen.

*Alternative:* Use the winding number directly. Define $\mathbb{Z} \to \pi_1(S^1)$ by $n \mapsto [\gamma_n]$ where $\gamma_n(\theta) = e^{2\pi i n \theta}$. Show it is an isomorphism using the universal cover (see covering spaces). $\square$

**Example 14.14 ($\pi_1(\text{Torus}) = \mathbb{Z}^2$).**
- Decompose $T^2$ as the union of two open sets, each homotopy equivalent to $S^1$.
- $U \cap V \simeq S^1 \vee S^1$ (a figure eight).
- Apply van Kampen: generators $a, b$ (the two fundamental loops), relation $aba^{-1}b^{-1} = 1$ (they commute).
- Result: $\pi_1(T^2) = \langle a, b \mid aba^{-1}b^{-1} \rangle = \mathbb{Z}^2$.

---

## 4. Covering Spaces

Covering spaces formalize the universal cover and the action of the fundamental group on "sheets."

### 4.1 Definition

**Definition 14.15 (Covering Space).** A continuous surjection $p : \tilde{X} \to X$ is a *covering map* if every $x \in X$ has an open neighborhood $U$ such that $p^{-1}(U) = \bigsqcup_\alpha V_\alpha$ (disjoint union of open sets) and each restriction $p|_{V_\alpha} : V_\alpha \to U$ is a homeomorphism.

The pair $(\tilde{X}, p)$ is a *covering space* of $X$. The fibers $p^{-1}(x)$ are discrete (since each $V_\alpha$ maps homeomorphically to $U$).

**Example 14.16.** $p : \mathbb{R} \to S^1$ by $p(t) = e^{2\pi i t}$. The fiber over any point is $\mathbb{Z}$ (the integers, as a discrete set).

**Example 14.17.** $p : S^n \to \mathbb{RP}^n$ by $p(x) = [x]$ (antipodal identification). The fiber over any point has 2 elements. For $n \geq 2$, this is the universal cover of $\mathbb{RP}^n$.

### 4.2 Path Lifting and the Fundamental Theorem

**Theorem 14.18 (Path Lifting).** Given $p : \tilde{X} \to X$, a path $\gamma : [0,1] \to X$, and a lift $\tilde{x}_0 \in p^{-1}(\gamma(0))$, there is a unique lift $\tilde{\gamma} : [0,1] \to \tilde{X}$ with $\tilde{\gamma}(0) = \tilde{x}_0$ and $p \circ \tilde{\gamma} = \gamma$.

**Theorem 14.19 (Homotopy Lifting).** Homotopies also lift uniquely.

**Corollary 14.20.** The *monodromy action*: $\pi_1(X, x_0)$ acts on the fiber $p^{-1}(x_0)$ (by lifting loops from $x_0$ and seeing where they end up).

**Theorem 14.21 (Fundamental Theorem of Covering Spaces).** For a path-connected, locally path-connected, and semi-locally simply connected space $X$:
- Covering spaces of $X$ correspond (up to isomorphism over $X$) to subgroups of $\pi_1(X, x_0)$.
- The universal cover corresponds to the trivial subgroup.
- The quotient $\pi_1(X, x_0)/H$ corresponds to the covering with fiber the coset space.

**Example 14.22.** Covering spaces of $S^1$ (with $\pi_1(S^1) = \mathbb{Z}$):
- Subgroups of $\mathbb{Z}$ are $\{n\mathbb{Z} \mid n \geq 0\}$.
- The subgroup $n\mathbb{Z}$ corresponds to the $n$-sheeted covering $e^{2\pi i n t} : S^1 \to S^1$ (wrapping $n$ times).
- The trivial subgroup $\{0\}$ corresponds to the universal cover $\mathbb{R}$.

---

## 5. Higher Homotopy Groups

**Definition 14.23 (Higher Homotopy Groups).** For $n \geq 1$, the *$n$-th homotopy group* $\pi_n(X, x_0)$ is the set of homotopy classes of maps $(S^n, s_0) \to (X, x_0)$ (from the $n$-sphere to $X$, fixing a basepoint), with a group structure given by "pinching" $S^n$.

For $n \geq 2$, $\pi_n(X)$ is *abelian*.

**Examples:**
- $\pi_n(S^n) = \mathbb{Z}$ (the degree of the map), for $n \geq 1$.
- $\pi_k(S^n) = 0$ for $k < n$ (the sphere has no "wrapping" in lower dimensions).
- $\pi_k(S^n)$ for $k > n$: these are notoriously hard to compute and form an active research area.
- $\pi_3(S^2) = \mathbb{Z}$: the Hopf fibration gives a generator (unexpected — $S^2$ has nontrivial $\pi_3$!).
- $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$: computed by Brunerie in HoTT.

### 5.1 Eilenberg-MacLane Spaces

**Definition 14.24.** A space $K(G, n)$ is an *Eilenberg-MacLane space* for a group $G$ and $n \geq 1$ if:
$$\pi_k(K(G,n)) = \begin{cases} G & k = n \\ 0 & k \neq n \end{cases}$$

Such spaces exist (and are unique up to homotopy equivalence) for any abelian group $G$ and $n \geq 2$, or any group $G$ and $n = 1$.

**Examples:**
- $K(\mathbb{Z}, 1) = S^1$ (the circle)
- $K(\mathbb{Z}, 2) = \mathbb{CP}^\infty$ (infinite-dimensional complex projective space)
- $K(\mathbb{Z}/2, 1) = \mathbb{RP}^\infty$

Eilenberg-MacLane spaces are the building blocks of homotopy theory, analogous to prime numbers in number theory. Every space can be "assembled" from Eilenberg-MacLane spaces via the *Postnikov tower*.

---

## 6. Fibrations and the Long Exact Sequence

**Definition 14.25 (Fibration).** A map $p : E \to B$ is a *Hurewicz fibration* if it has the *homotopy lifting property*: for any space $Y$, any map $f : Y \to E$, and any homotopy $H : Y \times [0,1] \to B$ with $H(-,0) = p \circ f$, there is a lift $\tilde{H} : Y \times [0,1] \to E$ with $p \circ \tilde{H} = H$ and $\tilde{H}(-,0) = f$.

**Example 14.26.** Covering maps are fibrations (with discrete fibers). The path-loop fibration $PX \to X$ (with $PX$ the space of paths starting at $x_0$, evaluated at their endpoint) is a fibration with fiber $\Omega X$ (the loop space).

**Theorem 14.27 (Long Exact Sequence of a Fibration).** For a fibration $F \hookrightarrow E \to B$ with fiber $F = p^{-1}(b_0)$:
$$\cdots \to \pi_n(F) \to \pi_n(E) \to \pi_n(B) \to \pi_{n-1}(F) \to \cdots \to \pi_1(B) \to \pi_0(F) \to \pi_0(E) \to \pi_0(B)$$

This is one of the most powerful computational tools in homotopy theory.

**Example 14.28.** The Hopf fibration $S^1 \hookrightarrow S^3 \to S^2$ gives the long exact sequence:
$$\cdots \to \pi_3(S^1) \to \pi_3(S^3) \to \pi_3(S^2) \to \pi_2(S^1) \to \pi_2(S^3) \to \pi_2(S^2) \to \pi_1(S^1) \to \cdots$$
Since $\pi_n(S^1) = 0$ for $n \geq 2$ and $\pi_n(S^3) = 0$ for $n \leq 2$:
$$0 \to \pi_3(S^3) = \mathbb{Z} \to \pi_3(S^2) \to 0$$
So $\pi_3(S^2) = \mathbb{Z}$, generated by the Hopf map.

---

## 7. CW Complexes

CW complexes (introduced by J.H.C. Whitehead) are the most convenient class of spaces for homotopy theory: they are built by a systematic cell-attachment process and every space is homotopy equivalent to a CW complex.

**Definition 14.29 (CW Complex).** A *CW complex* $X$ is built inductively:
- Start with a discrete set $X^0$ (the *0-skeleton*)
- Form $X^n$ from $X^{n-1}$ by attaching $n$-cells: for each $n$-cell $D^n_\alpha$, a *attaching map* $\phi_\alpha : S^{n-1} \to X^{n-1}$, and $X^n = X^{n-1} \cup_{\bigsqcup \phi_\alpha} \bigsqcup D^n_\alpha$
- $X = \bigcup_n X^n$ with the weak topology

**Example 14.30.** CW structures for standard spaces:
- $S^n$: one 0-cell and one $n$-cell
- $T^2$ (torus): one 0-cell, two 1-cells, one 2-cell, with attaching map $aba^{-1}b^{-1}$
- $\mathbb{RP}^n$: one cell in each dimension $0$ through $n$

**Theorem 14.31 (Whitehead's Theorem).** A map $f : X \to Y$ between CW complexes that induces isomorphisms on all homotopy groups is a homotopy equivalence.

Note: this requires both spaces to be CW complexes. A map inducing isomorphisms on all homotopy groups between general spaces need not be a homotopy equivalence.

---

## Exercises

**14.1.** Show that homotopy of maps is an equivalence relation.

**14.2.** Prove that $\pi_1(X \times Y) \cong \pi_1(X) \times \pi_1(Y)$ for any spaces $X, Y$.

**14.3.** Compute $\pi_1(\mathbb{RP}^2) = \mathbb{Z}/2\mathbb{Z}$ using the universal cover $S^2 \to \mathbb{RP}^2$.

**14.4.** Describe all covering spaces of the figure eight $S^1 \vee S^1$ (which has fundamental group $F_2$, the free group on two generators). How many connected 3-sheeted covers are there (up to isomorphism)?

**14.5.** Apply the long exact sequence to the path-loop fibration $\Omega X \to PX \to X$ (where $PX$ is contractible) to derive $\pi_n(X) \cong \pi_{n-1}(\Omega X)$.

**14.6.** The Hopf fibration: verify that the map $S^3 \to S^2$ defined by $(z_1, z_2) \mapsto [z_1 : z_2] \in \mathbb{CP}^1 = S^2$ (in complex coordinates) is indeed a fibration with fiber $S^1$.

**14.7.** Compute the CW structure of $\mathbb{RP}^2$ and verify the attaching map of the 2-cell corresponds to the relation in $\pi_1$.

**14.8 (Challenge).** The *Hurewicz theorem:* For a simply connected space $X$ (i.e., $\pi_0(X) = \pi_1(X) = 0$), the first non-trivial homotopy group $\pi_n(X) = \mathbb{Z}$ iff the first non-trivial homology group $H_n(X) = \mathbb{Z}$, and the Hurewicz map $\pi_n(X) \to H_n(X)$ is an isomorphism. State and explore this theorem; use it to compute $\pi_2(S^2)$.
