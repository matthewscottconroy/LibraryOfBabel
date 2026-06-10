# Covering Spaces

## The Basic Idea

A *covering space* of $X$ is a space $E$ that "covers" $X$ by laying multiple copies of the local structure of $X$ over each point. Imagine looking at a spiral staircase from above: what you see is a circle, but the staircase itself winds around, with multiple levels lying over each point of the circle below. The staircase is a covering of the circle.

More precisely:

**Definition.** A continuous map $p : E \to B$ is a *covering map* if every point $b \in B$ has an open neighborhood $U$ (an *evenly covered* neighborhood) such that $p^{-1}(U)$ is a disjoint union of open sets $\bigsqcup_\alpha V_\alpha$ in $E$, with each restriction $p|_{V_\alpha} : V_\alpha \to U$ a homeomorphism.

The space $E$ is the *covering space* (or *total space*), $B$ is the *base space*, $p$ is the *covering map*, and each $p^{-1}(b) = \{e \in E : p(e) = b\}$ is the *fiber* over $b$. For a connected base, all fibers have the same cardinality (the *number of sheets*).

**The spiral staircase:** The covering $p : \mathbb{R} \to S^1$ given by $p(t) = e^{2\pi i t}$ is the universal example. The real line covers the circle; each point of the circle has preimage $\mathbb{Z}$ (all the integers, lying one above each other on the staircase). Each small arc $U$ of the circle is evenly covered by the infinite family of arcs $U + n$ for $n \in \mathbb{Z}$.

## Path Lifting

The key property of covering maps is that paths (and homotopies) in the base lift uniquely to paths (and homotopies) in the total space.

**Theorem (Path Lifting).** Let $p : E \to B$ be a covering map. For any path $\gamma : [0,1] \to B$ and any point $e_0 \in p^{-1}(\gamma(0))$, there exists a unique path $\tilde{\gamma} : [0,1] \to E$ with $\tilde{\gamma}(0) = e_0$ and $p \circ \tilde{\gamma} = \gamma$.

*Proof.* Cover $[0,1]$ by preimages of evenly covered neighborhoods; by compactness, take a Lebesgue number and subdivide. Lift inductively on each subinterval, using the homeomorphism $p|_{V_\alpha}$ to define the lift uniquely.

**Theorem (Homotopy Lifting).** Let $p : E \to B$ be a covering map. Any homotopy $H : [0,1] \times [0,1] \to B$ lifts to a unique homotopy $\tilde{H} : [0,1] \times [0,1] \to E$ once the lift of $H|_{t=0}$ is specified.

These lifting properties are what make covering spaces the model for fibrations in homotopy theory — and for $\Pi$-types in HoTT.

## The Fundamental Group Acts on Fibers

Fix a basepoint $b_0 \in B$ and $e_0 \in p^{-1}(b_0)$. Every loop $\gamma : [0,1] \to B$ based at $b_0$ lifts to a unique path $\tilde{\gamma}$ in $E$ starting at $e_0$. The endpoint $\tilde{\gamma}(1)$ lies in $p^{-1}(b_0)$ (since $p(\tilde{\gamma}(1)) = \gamma(1) = b_0$). If $\gamma \simeq \delta$ rel basepoint, then their lifts end at the same point (by homotopy lifting). So we get a well-defined map:
$$\mu : \pi_1(B, b_0) \times p^{-1}(b_0) \to p^{-1}(b_0), \quad ([\gamma], e) \mapsto \tilde{\gamma}_e(1)$$
where $\tilde{\gamma}_e$ is the lift of $\gamma$ starting at $e$.

This is a *right action* of $\pi_1(B, b_0)$ on the fiber $p^{-1}(b_0)$ (note the reversal: $\mu([\delta], \mu([\gamma], e)) = \mu([\gamma \cdot \delta], e)$, matching the right action convention). The action is by *monodromy*: traveling along a loop in the base permutes the sheets of the covering.

**For the staircase covering** $p : \mathbb{R} \to S^1$: the fiber over $1 \in S^1$ is $\mathbb{Z} \subseteq \mathbb{R}$. The generator $[\ell]$ of $\pi_1(S^1) = \mathbb{Z}$ (the loop that goes around once) acts on the fiber by $n \mapsto n + 1$. The fundamental group $\mathbb{Z}$ acts on itself by translation — the monodromy is exactly the deck transformation.

## The Galois Correspondence

The profound theorem of covering space theory is a perfect analogy of Galois theory for field extensions.

**Theorem (Classification of Covering Spaces).** Let $B$ be a connected, locally path-connected, semi-locally simply connected space with basepoint $b_0$. There is a bijective correspondence:
$$\left\{\text{connected covering spaces of } B, \text{ up to isomorphism}\right\} \longleftrightarrow \left\{\text{subgroups of } \pi_1(B, b_0)\right\}$$

The correspondence: a covering $p : E \to B$ corresponds to the subgroup $p_*(\pi_1(E, e_0)) \subseteq \pi_1(B, b_0)$ (the image of the induced map on fundamental groups). The larger the subgroup, the "smaller" the covering (fewer sheets).

Special cases:
- The subgroup $\pi_1(B, b_0)$ itself corresponds to the trivial covering $\mathsf{id}_B : B \to B$.
- The trivial subgroup $\{e\}$ corresponds to the *universal cover* $\tilde{B}$: the unique simply-connected cover of $B$.
- A subgroup $H \subseteq \pi_1(B, b_0)$ corresponds to an intermediate cover $B_H$ with $\pi_1(B_H) \cong H$.

**For the circle:** $\pi_1(S^1) = \mathbb{Z}$. The subgroups of $\mathbb{Z}$ are $n\mathbb{Z}$ for $n \geq 0$. The subgroup $n\mathbb{Z}$ corresponds to the $n$-sheeted cover $p_n : S^1 \to S^1$ given by $z \mapsto z^n$ (wrapping the circle $n$ times). The subgroup $\{0\}$ corresponds to the universal cover $\mathbb{R}$.

This is Galois theory: covering spaces of $B$ correspond to subgroups of $\pi_1(B)$, just as field extensions of $K$ correspond to subgroups of the Galois group $\text{Gal}(L/K)$.

## The Universal Cover

The *universal cover* $\tilde{B}$ of $B$ is the covering corresponding to the trivial subgroup of $\pi_1(B, b_0)$. It is simply connected: $\pi_1(\tilde{B}) = 0$.

Construction: as a set, $\tilde{B}$ consists of homotopy classes of paths in $B$ starting at $b_0$ (homotopy relative to endpoints). A point of $\tilde{B}$ is a "path up to deformation" starting at the basepoint. The covering map $p : \tilde{B} \to B$ sends a path class to its endpoint: $p([\gamma]) = \gamma(1)$.

The topology on $\tilde{B}$ is defined by the path-lifting property: small neighborhoods of $[\gamma]$ in $\tilde{B}$ are defined using small neighborhoods of $\gamma(1)$ in $B$.

The fiber $p^{-1}(b_0)$ is the set of homotopy classes of loops at $b_0$ — exactly $\pi_1(B, b_0)$. The fundamental group acts freely and transitively on the fiber of its own universal cover.

**Examples:**
- Universal cover of $S^1$ is $\mathbb{R}$ (the staircase).
- Universal cover of $T^2 = \mathbb{R}^2/\mathbb{Z}^2$ is $\mathbb{R}^2$.
- Universal cover of $\mathbb{RP}^n$ is $S^n$ (for $n \geq 1$).

## Deck Transformations

A *deck transformation* (or *covering transformation*) of $p : E \to B$ is a homeomorphism $\phi : E \to E$ with $p \circ \phi = p$: a homeomorphism of the total space that preserves the covering map.

The group of deck transformations $\text{Deck}(E/B)$ acts on the fibers of $p$. For the universal cover, $\text{Deck}(\tilde{B}/B) \cong \pi_1(B, b_0)$ — the fundamental group is realized as a group of homeomorphisms of the universal cover.

For the staircase: $\text{Deck}(\mathbb{R}/S^1) = \{n \mapsto n + k : k \in \mathbb{Z}\} \cong \mathbb{Z}$. Each deck transformation is a translation of the real line by an integer.

## Connection to HoTT: $\Pi$-Types as Covering Spaces

The covering space theory has a beautiful HoTT analog. In HoTT, a *fibration* over a type $B$ is a dependent type family $P : B \to \mathcal{U}$. The total space $\sum_{b:B} P(b)$ covers $B$ via the projection map.

For covering spaces specifically — discrete fibers — the correspondence is:
- A covering space of $B$ with fiber $F$ corresponds to a type family $P : B \to \text{Set}$ (a family of sets over $B$).
- The fundamental group action on the fiber corresponds to the action of the loop space $\Omega(B, b_0)$ on $P(b_0)$.

This is the HoTT version of the Galois correspondence: type families over $B$ correspond to $\pi_1(B)$-sets. The universal cover corresponds to the "identity type family" $P(b) = (b_0 = b)$, whose total space $\sum_{b:B} (b_0 = b)$ is contractible (it is the based path space of $B$, which is always contractible).

This is not just an analogy: in the simplicial set model of HoTT, this correspondence is a theorem. Covering spaces of a Kan complex $B$ are exactly the functors from the fundamental groupoid of $B$ to sets, and dependent type families over $B$ are exactly the functors from the fundamental $\infty$-groupoid of $B$ to types.
