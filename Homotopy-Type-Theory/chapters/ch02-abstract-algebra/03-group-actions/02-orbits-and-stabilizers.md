# 3.2 Orbits and Stabilizers

## Partitioning the Set

When a group $G$ acts on a set $X$, not every element of $X$ needs to be reachable from every other. Some elements might be "connected" by the action, and others might be completely independent. This gives a natural partition of $X$ into *orbits* — classes of elements that can be reached from each other.

## Orbits

**Definition (Orbit).** For a group $G$ acting on a set $X$ and a point $x \in X$, the *orbit* of $x$ under $G$ is:
$$G \cdot x = \{g \cdot x \mid g \in G\}$$

The orbit is "everything reachable from $x$."

**Lemma.** The orbits partition $X$.

*Proof.* Define the relation $x \sim y$ if there exists $g \in G$ with $g \cdot x = y$. This is an equivalence relation:
- *Reflexive:* $e \cdot x = x$, so $x \sim x$.
- *Symmetric:* If $g \cdot x = y$, then $g^{-1} \cdot y = g^{-1} \cdot (g \cdot x) = (g^{-1}g) \cdot x = e \cdot x = x$, so $y \sim x$.
- *Transitive:* If $g \cdot x = y$ and $h \cdot y = z$, then $(hg) \cdot x = h \cdot (g \cdot x) = h \cdot y = z$, so $x \sim z$.

The equivalence classes of $\sim$ are exactly the orbits $G \cdot x$. $\square$

**Examples:**
- For the left regular action $G \curvearrowright G$: the single orbit is $G$ itself (the action is transitive).
- For $S_3 \curvearrowright \{1, 2, 3\}$: one orbit $\{1, 2, 3\}$ (transitive).
- For $\mathbb{Z}$ acting on $\mathbb{Z}$ by even shifts ($n \cdot k = k + 2n$): two orbits, the even integers and the odd integers.
- For $SO(3)$ acting on $\mathbb{R}^3$: the orbits are concentric spheres $\{v \mid |v| = r\}$ for each $r \geq 0$, plus the origin.

## Stabilizers

**Definition (Stabilizer).** For $x \in X$, the *stabilizer* of $x$ (also called the *isotropy group* or *little group*) is:
$$\text{Stab}_G(x) = G_x = \{g \in G \mid g \cdot x = x\}$$

The stabilizer consists of all group elements that "fix" $x$ — that leave it unchanged.

**Lemma.** $\text{Stab}_G(x) \leq G$.

*Proof.* 
- $e \cdot x = x$, so $e \in \text{Stab}(x)$ (nonempty).
- If $g, h \in \text{Stab}(x)$: $(gh) \cdot x = g \cdot (h \cdot x) = g \cdot x = x$, so $gh \in \text{Stab}(x)$ (closed).
- If $g \in \text{Stab}(x)$: $g^{-1} \cdot x = g^{-1} \cdot (g \cdot x) = (g^{-1}g) \cdot x = e \cdot x = x$, so $g^{-1} \in \text{Stab}(x)$ (inverses). $\square$

**Examples:**
- For $SO(3) \curvearrowright S^2$ (rotations of the sphere): $\text{Stab}(\text{north pole}) = SO(2)$ (rotations about the vertical axis).
- For $S_n \curvearrowright \{1, \ldots, n\}$: $\text{Stab}(i) = \{\sigma \in S_n \mid \sigma(i) = i\} \cong S_{n-1}$.
- For the left regular action $G \curvearrowright G$: $\text{Stab}(g) = \{h \mid hg = g\} = \{e\}$. Trivial stabilizer.
- For the conjugation action $G \curvearrowright G$: $\text{Stab}(g) = \{h \mid hgh^{-1} = g\} = \{h \mid hg = gh\} = C_G(g)$, the *centralizer* of $g$.

## The Orbit-Stabilizer Theorem

The orbit and stabilizer of a point are related by a beautiful counting theorem.

**Theorem (Orbit-Stabilizer).** For a group $G$ acting on a set $X$ and a point $x \in X$:
$$|G \cdot x| = [G : \text{Stab}(x)]$$

For finite $G$: $|G| = |G \cdot x| \cdot |\text{Stab}(x)|$.

*Proof.* Define a function $\phi: G/\text{Stab}(x) \to G \cdot x$ by $\phi(g\text{Stab}(x)) = g \cdot x$.

*Well-defined:* If $g\text{Stab}(x) = h\text{Stab}(x)$, then $h^{-1}g \in \text{Stab}(x)$, so $h^{-1}g \cdot x = x$, so $g \cdot x = h \cdot x$.

*Injective:* If $\phi(gS) = \phi(hS)$, then $g \cdot x = h \cdot x$, so $h^{-1}g \cdot x = x$, so $h^{-1}g \in S = \text{Stab}(x)$, so $gS = hS$.

*Surjective:* For any $g \cdot x \in G \cdot x$, $\phi(g\text{Stab}(x)) = g \cdot x$.

So $\phi$ is a bijection, and $|G \cdot x| = |G/\text{Stab}(x)| = [G : \text{Stab}(x)]$. $\square$

## Applications of the Orbit-Stabilizer Theorem

**Application 1: Counting orbits.** The total number of elements in $X$ equals the sum of orbit sizes:
$$|X| = \sum_{\text{orbits}} |G \cdot x| = \sum_{\text{orbits}} \frac{|G|}{|\text{Stab}(x)|}$$

**Application 2: Sphere as a homogeneous space.** $SO(3)$ acts on $S^2$ transitively. Pick the north pole $p$. The stabilizer $\text{Stab}(p) = SO(2)$. By orbit-stabilizer:
$$|S^2| = |SO(3) \cdot p| = [SO(3) : SO(2)] = |SO(3)| / |SO(2)|$$

For Lie groups, "size" means dimension: $\dim(S^2) = \dim(SO(3)) - \dim(SO(2)) = 3 - 1 = 2$. This is exact!

More generally, this gives a *fibration sequence*: $SO(2) \to SO(3) \to S^2$, where $SO(2)$ is the "fiber" (stabilizer) and $S^2$ is the "base" (orbit). Such sequences are fundamental in algebraic topology.

**Application 3: Counting colorings.** How many distinct ways can we color the faces of a cube with $k$ colors, where two colorings are "the same" if one can be rotated to the other?

The rotation group of the cube $G$ (order 24) acts on the set of colorings. Two colorings are equivalent iff they're in the same orbit. We want to count orbits.

Burnside's Lemma: the number of orbits is $\frac{1}{|G|} \sum_{g \in G} |X^g|$, where $X^g$ is the set of colorings fixed by $g$. We compute $|X^g|$ for each type of rotation and sum. This is a beautiful application of group actions to combinatorics.

**Application 4: The class equation.** For the conjugation action $G \curvearrowright G$:
- Orbits are *conjugacy classes* $\{ghg^{-1} \mid g \in G\}$.
- Stabilizer of $h$ is the centralizer $C_G(h)$.
- Orbit-stabilizer: $|\text{conj. class of } h| = [G : C_G(h)]$.

Elements of $Z(G)$ (the center) form singleton conjugacy classes. All other elements form larger classes. Summing:
$$|G| = |Z(G)| + \sum_{[h] \text{ non-central}} [G : C_G(h)]$$

This is the *class equation*, and it's surprisingly powerful. It implies:

**Corollary (p-groups have non-trivial centers).** If $|G| = p^n$ for a prime $p$ and $n \geq 1$, then $Z(G) \neq \{e\}$.

*Proof.* Each term $[G : C_G(h)]$ divides $|G| = p^n$, hence is a power of $p$. For non-central elements, the term is at least $p$. So $|Z(G)| = p^n - \sum p^{k_i}$ is divisible by $p$, hence $|Z(G)| \geq p > 1$. $\square$

This is a key step in proving that every group of order $p^2$ is abelian (and classifying such groups).

## Stabilizers of Different Points in an Orbit

If $y = g \cdot x$ (i.e., $y$ is in the orbit of $x$), how are $\text{Stab}(x)$ and $\text{Stab}(y)$ related?

**Lemma.** $\text{Stab}(g \cdot x) = g \cdot \text{Stab}(x) \cdot g^{-1}$.

*Proof.* $h \in \text{Stab}(g \cdot x) \iff h \cdot (g \cdot x) = g \cdot x \iff g^{-1}hg \cdot x = x \iff g^{-1}hg \in \text{Stab}(x) \iff h \in g \cdot \text{Stab}(x) \cdot g^{-1}$. $\square$

Stabilizers of different points in the same orbit are *conjugate subgroups*. If the action is transitive, all stabilizers are conjugate — they're all "the same subgroup" up to conjugation.

This has a topological interpretation: for a fiber bundle $p: \tilde{X} \to X$ with fiber $F = p^{-1}(x_0)$, the "holonomy" around different basepoints gives conjugate subgroups of the structure group. The "conjugacy class of a subgroup" is better behaved than any particular subgroup because it doesn't depend on the choice of basepoint.

## The Action on Left Cosets and the Core

Here's a key construction: $G$ acts on the set $G/H$ of left cosets by left multiplication: $g \cdot (aH) = (ga)H$. This action is always transitive.

The kernel of this action (elements that fix all cosets) is:
$$\text{Core}_G(H) = \bigcap_{g \in G} gHg^{-1}$$
the *core* of $H$, the largest normal subgroup of $G$ contained in $H$.

**Theorem.** If $[G : H] = n$, then $G/\text{Core}(H)$ embeds in $S_n$.

*Proof.* The action of $G$ on $G/H$ (which has $n$ elements) gives a homomorphism $G \to S_n$. The kernel is $\text{Core}(H)$. By the First Isomorphism Theorem, $G/\text{Core}(H) \cong \text{Im}(\phi) \leq S_n$. $\square$

**Corollary.** If $[G : H] = n$ and $G$ has no normal subgroups of index dividing $n!$, then... (various consequences follow for small-index subgroups).

This theorem is how one proves things like "a simple group of order 60 is isomorphic to $A_5$": it must embed in a small symmetric group, and checking all possible embeddings forces $A_5$.

## Summary

The orbit-stabilizer theorem is a counting theorem in disguise — it says the "size" of an orbit (how many places $x$ can go) is determined by how much of $G$ fixes $x$. The more symmetry $x$ has (the larger its stabilizer), the smaller its orbit (the fewer places it can go). This trade-off between symmetry and mobility is a fundamental theme in geometry and physics.
