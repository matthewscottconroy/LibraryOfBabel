# Group Actions

## Symmetry as Action

A group is an abstraction of symmetry. A group *action* makes this concrete: it is the precise formulation of what it means for a group to act as the symmetry group of some object.

**Definition.** A *left action* of a group G on a set X is a function · : G × X → X satisfying:
- **Identity:** e · x = x for all x ∈ X.
- **Compatibility:** (gh) · x = g · (h · x) for all g, h ∈ G and x ∈ X.

We say G *acts on* X, and write (G, X, ·) or just G ↷ X.

Equivalently, a group action is a group homomorphism φ: G → Sym(X) (the symmetric group of all bijections X → X), where φ(g) is the bijection x ↦ g · x. The identity axiom says φ(e) = idₓ; the compatibility axiom says φ(gh) = φ(g) ∘ φ(h).

This equivalence is illuminating: a group action is the same data as a way of representing G as a group of symmetries (bijections) of some set.

## Examples

**Symmetric group acting on {1, ..., n}.** The group Sₙ acts on {1, ..., n} by σ · i = σ(i). This is the "tautological" action.

**Group acting on itself by left multiplication.** Any group G acts on itself: g · h = gh. This is the *regular representation*.

**Group acting on itself by conjugation.** g · h = ghg⁻¹. Each g acts as an automorphism of G.

**Rotations of the sphere.** SO(3), the group of rotation matrices in ℝ³, acts on the 2-sphere S² by matrix-vector multiplication. Each rotation is a symmetry of the sphere.

**Deck transformations.** If p: E → B is a covering space, the group of deck transformations acts on E (fixing B). This action is free and properly discontinuous, and the quotient E/G ≅ B. This is one of the key connections between group theory and topology.

**Galois group acting on roots.** Given a polynomial p(x) over ℚ, its Galois group Gal(K/ℚ) acts on the roots of p — permuting them while preserving all algebraic relations. The structure of this action determines whether p is solvable by radicals.

## Orbits and Stabilizers

For a group G acting on X and a point x ∈ X:

**Definition.** The *orbit* of x is Orb(x) = {g · x | g ∈ G} — the set of all images of x under group elements.

**Definition.** The *stabilizer* of x is Stab(x) = {g ∈ G | g · x = x} — the subgroup of elements fixing x.

**Theorem (Orbit-Stabilizer Theorem).** If G is a finite group acting on X and x ∈ X, then |G| = |Orb(x)| · |Stab(x)|.

*Proof.* Consider the map f: G → Orb(x) defined by f(g) = g · x. This is surjective by definition. Two elements g, h ∈ G satisfy f(g) = f(h) iff g · x = h · x iff g⁻¹h · x = x iff g⁻¹h ∈ Stab(x) iff g and h lie in the same left coset of Stab(x). So f induces a bijection between the cosets of Stab(x) in G and the orbit Orb(x). The number of cosets is |G|/|Stab(x)| (by Lagrange). □

**Corollary.** The orbit of any point x has size |G|/|Stab(x)|, which divides |G|.

**Example.** Sₙ acts on {1, ..., n}. The orbit of i is {1, ..., n} (any permutation can send i anywhere). So |Orb(i)| = n and |Stab(i)| = |Sₙ|/n = n!/n = (n-1)!. Indeed, Stab(i) = Sₙ₋₁ (permutations that fix i), which has (n-1)! elements.

**The orbit equation.** If G acts on a finite set X, and we pick one representative from each orbit, then:

|X| = Σ |Orb(x)| = Σ [G : Stab(x)]

This is the *class equation*, which has many applications in finite group theory.

**Burnside's Lemma.** The number of orbits of G acting on X equals (1/|G|) Σ_{g∈G} |X^g|, where X^g = {x ∈ X | g · x = x} is the fixed-point set of g.

Burnside's lemma is used in combinatorics to count distinct configurations up to symmetry — for example, the number of distinct colorings of a necklace with n beads using k colors, where two colorings are "the same" if one is a rotation of the other.

## Cayley's Theorem

**Theorem (Cayley, 1854).** Every group G is isomorphic to a subgroup of a symmetric group.

*Proof.* Consider the regular action: G acts on itself by left multiplication. Define φ: G → Sym(G) by φ(g) = (h ↦ gh). This is a group homomorphism (φ(gh)(x) = ghx = φ(g)(φ(h)(x))). It is injective: if φ(g) = φ(g'), then for all h ∈ G, gh = g'h, so g = g' by cancellation. So G ≅ Im(φ) ≤ Sym(G). □

**Consequences.** Every finite group of order n embeds in Sₙ (and actually in S_{n/2} for n ≥ 3). Every abstract group is a concrete group of permutations — there is no gap between "abstract" and "concrete" symmetry.

Cayley's theorem shows that the symmetric group Sₙ is "universal" for finite groups. It also shows that every group has a faithful action on itself — one where no non-identity element acts trivially.

## Group Actions and Covering Spaces

The deepest application of group actions in topology is the theory of covering spaces. A *covering space* is a continuous surjection p: E → B such that every point b ∈ B has an evenly covered neighborhood U: p⁻¹(U) is a disjoint union of open sets, each mapped homeomorphically onto U by p.

The fundamental group π₁(B, b₀) acts on the fiber p⁻¹(b₀) by *monodromy*: if γ is a loop at b₀ and e ∈ p⁻¹(b₀), then the unique lift of γ starting at e ends at another point γ · e in the fiber. This defines a group action.

**Key theorem.** There is a bijection between:
- Connected covering spaces of B (up to isomorphism)
- Subgroups of π₁(B, b₀) (up to conjugation)

Under this bijection, the *universal cover* (a simply connected covering space, if it exists) corresponds to the trivial subgroup {e}. The degree of the covering equals the index [π₁(B, b₀) : H] of the corresponding subgroup.

**Example.** The real line ℝ covers the circle S¹ via p(t) = (cos 2πt, sin 2πt). The fiber over any point is a copy of ℤ (the integer translates of any preimage). The deck transformation group (group of homeomorphisms of ℝ that commute with p) is ℤ, acting by t ↦ t + n. This corresponds to the fact that π₁(S¹) = ℤ, and the universal cover corresponds to the trivial subgroup.

In HoTT, covering spaces are modeled by *fibrations* — maps between types whose fibers are sets (0-truncated types). The action of π₁(B) on the fiber is encoded in the type-theoretic transport operation: given a path p: b₀ = b₁ in B and a term e in the fiber over b₀, transport along p gives a term in the fiber over b₁. This is the type-theoretic monodromy.

## Orbifolds and Quotient Constructions

When a group G acts on a space X, the quotient X/G — the set of orbits — often has interesting geometric structure. If the action is *free* (no non-identity element has any fixed points), X/G is a manifold and the quotient map p: X → X/G is a covering space.

When the action is not free, X/G is an *orbifold*: a space that looks locally like ℝⁿ/H for some finite group H (the stabilizer of the corresponding point in X). Orbifolds appear in the classification of surfaces and in string theory.

In HoTT, the quotient X/G corresponds to the *orbit type* — the type of orbits under a group action. For good group actions, this is the pushout of the action maps. The study of orbit types in HoTT connects to the theory of *n-truncations* and *groupoid quotients*, which are the type-theoretic foundation of equivariant homotopy theory.

Group actions are where abstract algebra meets geometry. The orbit-stabilizer theorem, Cayley's theorem, covering space theory, and the HoTT treatment of orbit types are all instances of the same principle: symmetry groups act on spaces, and the structure of the action reflects the topology of the space.
