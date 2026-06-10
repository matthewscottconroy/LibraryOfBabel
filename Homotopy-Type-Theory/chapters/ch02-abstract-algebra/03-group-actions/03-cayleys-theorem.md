# 3.3 Cayley's Theorem

## Every Group Is a Symmetry Group

We've been studying "abstract" groups — algebraic objects defined by axioms. But group theory grew out of the study of permutations. Are abstract groups genuinely more general than permutation groups? Or is every abstract group secretly a permutation group in disguise?

The answer: **every group is isomorphic to a group of permutations**. There is no such thing as an "abstract" group that can't be realized concretely as permutations of some set. This is Cayley's theorem.

## The Theorem

**Theorem (Cayley's Theorem).** Every group $G$ is isomorphic to a subgroup of $\text{Sym}(G)$ — the symmetric group of all bijections $G \to G$.

In other words, every group $G$ "is" a group of symmetries — specifically, symmetries of its own underlying set.

*Proof.* Define the *left regular action* $\phi: G \to \text{Sym}(G)$ by:
$$\phi(g)(h) = gh \quad \text{(left multiplication by } g\text{)}$$

**Step 1: Each $\phi(g)$ is a bijection.** The map $h \mapsto gh$ has inverse $h \mapsto g^{-1}h$, so it's indeed an element of $\text{Sym}(G)$.

**Step 2: $\phi$ is a homomorphism.** For $g_1, g_2 \in G$:
$$\phi(g_1 g_2)(h) = (g_1 g_2)h = g_1(g_2 h) = g_1 \cdot \phi(g_2)(h) = (\phi(g_1) \circ \phi(g_2))(h)$$
So $\phi(g_1 g_2) = \phi(g_1) \circ \phi(g_2)$. ✓

**Step 3: $\phi$ is injective.** If $\phi(g) = \phi(g')$ (as functions $G \to G$), then in particular $\phi(g)(e) = \phi(g')(e)$, so $ge = g'e$, so $g = g'$. ✓

Therefore $\phi$ is an injective homomorphism, so $G \cong \text{Im}(\phi) \leq \text{Sym}(G)$. $\square$

## What Cayley's Theorem Says

The theorem is reassuring: there are no "exotic" abstract groups that escape the world of permutations. Every group arises as a collection of bijections of some set, closed under composition.

But it's also somewhat weak: it tells you $G$ embeds in $\text{Sym}(G)$, which has order $(|G|)!$ — vastly larger than $G$ itself. For a group of order 10, this embeds it in $S_{10}$, which has order $10! = 3,628,800$. Not a very efficient embedding.

In practice, we often look for smaller representations. If $G$ has a subgroup $H$ of small index $n$, then $G$ embeds in $S_n$ (via the action on cosets), which can be much smaller.

**Example.** $A_5$ has order 60. It acts on itself (Cayley: embeds in $S_{60}$). But $A_5$ also acts on a set of 5 elements — it's a subgroup of $S_5$ already, which has order 120. Much more efficient!

## Cayley's Theorem and Representation Theory

Cayley's theorem is the beginning of *representation theory* — the study of how groups act on sets (more specifically, on vector spaces).

A *representation* of $G$ is a homomorphism $\rho: G \to \text{GL}(V)$ for some vector space $V$ (invertible linear transformations of $V$). Cayley's theorem is the permutation version: every group embeds in a symmetry group.

The natural next question: what are the *irreducible* representations of $G$ — those that can't be broken into smaller pieces? Classifying irreducible representations is one of the central problems of representation theory, and it has deep connections to:
- Fourier analysis (Pontryagin duality for abelian groups)
- Quantum mechanics (representations of $SO(3)$, $SU(2)$, etc.)
- Number theory (Galois representations, Langlands program)
- Combinatorics (representations of $S_n$ and Young tableaux)

Cayley's theorem just says every group has at least one representation (the left regular one). The deeper questions are about which representations exist and how they decompose.

## The Cayley Graph

There's a geometric object associated with Cayley's theorem: the *Cayley graph*.

**Definition.** Given a group $G$ and a generating set $S \subseteq G$ (closed under inverses), the *Cayley graph* $\Gamma(G, S)$ has:
- *Vertices:* elements of $G$
- *Edges:* connect $g$ to $gs$ for each $g \in G$ and $s \in S$

The Cayley graph turns the abstract group $G$ into a geometric object. The group $G$ acts on its own Cayley graph by left multiplication: $g$ sends vertex $h$ to vertex $gh$, and preserves the edge structure. This is Cayley's theorem, geometrized.

**Examples:**
- $\mathbb{Z}$ with $S = \{+1, -1\}$: the Cayley graph is the integer number line $\ldots -2 - 1 - 0 - 1 - 2 \ldots$
- $\mathbb{Z}/n\mathbb{Z}$ with $S = \{+1, -1\}$: a cycle of length $n$.
- $F_2 = \langle a, b \rangle$ with $S = \{a, a^{-1}, b, b^{-1}\}$: an infinite 4-regular tree (every vertex has degree 4).
- $S_3$ with $S = \{(12), (123), (132)\}$: a more complex graph.

Cayley graphs give a way to study groups geometrically. The large-scale geometry of Cayley graphs — how "fast" they expand, whether they have cycles, what their boundaries look like — reveals properties of the group. This is the subject of *geometric group theory*, one of the active areas of modern mathematics.

A famous open problem: does every finite group have a Hamiltonian path in some Cayley graph? (Almost certainly yes, but it's not proven in full generality.)

## Cayley's Theorem in Type Theory

In type theory, there's an analog of Cayley's theorem for types (not just groups).

Given a type $A$ and a function $f: A \to A$ (not necessarily a group element), the *Yoneda embedding* says that $A$ is determined by its "function type" $A \to A$. For groups, this specializes to Cayley's theorem.

More precisely, in category theory, the Yoneda lemma says every category embeds (fully faithfully) into its functor category. For the category with one object (a group), this gives Cayley's theorem.

In HoTT, the Yoneda lemma holds for $(\infty, 1)$-categories, and it has consequences for higher groups (∞-groupoids). Understanding Cayley's theorem is a step toward understanding these higher-categorical analogs.

## Summary

Cayley's theorem closes a circle: we started with permutation groups (historically the first groups studied), abstracted to the group axioms, and then discovered via Cayley's theorem that abstract groups are exactly the same as permutation groups. The abstraction was justified — and also ultimately conservative. Every abstract group is a symmetry group of something.

This is a pattern that recurs in mathematics: you generalize, you prove something about the generalization, and then you discover that the generalization has secretly returned to something concrete. The abstraction was not for its own sake — it clarified the structure and made the theorems easier to prove.
