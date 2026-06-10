# Connectedness and Paths

## Two Kinds of Connectedness

There are two natural notions of a space being "in one piece," and they are not equivalent.

**Connectedness** asks: can the space be split into two disjoint non-empty open pieces? If not, the space is connected.

**Path-connectedness** asks: can any two points be joined by a path? If yes, the space is path-connected.

Path-connectedness implies connectedness, but not conversely. The example to keep in mind: the *topologist's sine curve* — the closure of the graph of sin(1/x) for x > 0. This space is connected (it cannot be split into two disjoint open pieces) but not path-connected (you cannot draw a path from a point on the y-axis to a point on the wavy part).

For "nice" spaces (manifolds, CW complexes), the two notions coincide. For pathological spaces, they differ. In this curriculum, we mostly work with nice spaces, and path-connectedness is the right notion for homotopy theory.

## Connectedness

**Definition.** A topological space X is *connected* if it cannot be written as X = U ∪ V where U, V are non-empty, disjoint, and open.

Equivalently: X is connected iff the only subsets of X that are both open and closed are ∅ and X.

**Examples:**
- ℝ is connected. (Proof: suppose ℝ = U ∪ V with U, V non-empty, disjoint, open. Pick a ∈ U and b ∈ V. WLOG a < b. Let c = sup(U ∩ [a,b]). Then c ∈ cl(U) ⊆ U (since U is closed as the complement of V). But also c is a limit of points in V (if not, some interval around c is in U, contradicting c = sup(U ∩ [a,b])). Since V is closed, c ∈ V. Contradiction: U and V are disjoint.)
- [a, b] is connected for any a ≤ b (same proof, on [a,b]).
- ℝ \ {0} = (-∞, 0) ∪ (0, ∞) is disconnected.
- ℚ is disconnected (it contains no intervals, so every rational number is isolated by irrationals).
- S¹ is connected. Sⁿ for any n ≥ 1 is connected.

**Theorem.** Continuous images of connected spaces are connected.

*Proof.* Let f: X → Y be continuous with X connected. Suppose f(X) = U ∪ V with U, V disjoint non-empty open in f(X). Then f⁻¹(U) and f⁻¹(V) are disjoint non-empty open in X with f⁻¹(U) ∪ f⁻¹(V) = X. This contradicts connectedness of X. □

**Intermediate Value Theorem (from connectedness).** If f: X → ℝ is continuous and X is connected, then f(X) is an interval (possibly infinite or degenerate). In particular, if f(x) = a and f(y) = b, then f takes every value between a and b.

*Proof.* f(X) is a connected subset of ℝ. Connected subsets of ℝ are intervals (proof: if f(X) is not an interval, there exist a < c < b with a, b ∈ f(X) but c ∉ f(X). Then f(X) = f(X) ∩ (-∞, c) ∪ f(X) ∩ (c, ∞) is a disconnection.) □

This is the cleanest proof of IVT: it reduces to the claim that connected subsets of ℝ are intervals, which is geometric and obvious.

## Path-Connectedness

**Definition.** A space X is *path-connected* if for every x, y ∈ X, there exists a path from x to y: a continuous function γ: [0, 1] → X with γ(0) = x and γ(1) = y.

**Definition.** A *path* in X from x to y is a continuous function γ: [0, 1] → X with γ(0) = x and γ(1) = y.

Paths are the fundamental objects of homotopy theory. Every definition about paths, homotopies, and the fundamental group starts here.

**Examples of path-connected spaces:**
- ℝⁿ is path-connected: the straight-line path γ(t) = x + t(y - x) connects any x to y.
- Any convex subset of ℝⁿ is path-connected.
- S¹ is path-connected: the arc from one point to another.
- The complement of finitely many points in ℝⁿ for n ≥ 2 is path-connected.

**Examples of connected but not path-connected spaces:**
- The topologist's sine curve (described above).
- Certain fractal sets.

For this curriculum, we work almost exclusively with path-connected spaces (manifolds, CW complexes), where connectedness and path-connectedness coincide.

## The Path: The Central Object

Let us focus on the path γ: [0,1] → X.

**Constant path.** The constant path at x is c_x(t) = x for all t. It is a path from x to x.

**Path reversal.** Given a path γ from x to y, the reversed path is γ̄(t) = γ(1-t). It is a path from y to x.

**Path concatenation.** Given paths γ from x to y and δ from y to z, their concatenation is:

(γ ∗ δ)(t) = γ(2t) for t ∈ [0, 1/2], δ(2t - 1) for t ∈ [1/2, 1].

This is continuous: γ(2·(1/2)) = γ(1) = y = δ(0) = δ(2·(1/2) - 1), so the two pieces agree at t = 1/2. The Pasting Lemma ensures the concatenation is continuous.

**Pasting Lemma.** If X = A ∪ B where A and B are closed, and f: A → Y and g: B → Y are continuous with f = g on A ∩ B, then the function that equals f on A and g on B is continuous.

## The Fundamental Group (Informal)

Fix a basepoint x₀ ∈ X. A *loop* based at x₀ is a path γ with γ(0) = γ(1) = x₀.

Two loops γ, δ are *homotopic* (rel endpoints) if there is a continuous H: [0,1]² → X with:
- H(t, 0) = γ(t) and H(t, 1) = δ(t) for all t (the homotopy starts at γ and ends at δ).
- H(0, s) = H(1, s) = x₀ for all s (endpoints stay fixed).

Homotopy is an equivalence relation. The equivalence class of γ is [γ].

**The fundamental group** π₁(X, x₀) is the set of homotopy classes of loops at x₀, with multiplication [γ] · [δ] = [γ ∗ δ].

This is a group:
- **Identity:** [c_{x₀}], the class of the constant loop.
- **Inverses:** [γ]⁻¹ = [γ̄], the reversed loop (homotopic to the constant because you can contract γ ∗ γ̄ to c_{x₀} by "shrinking the loop back").
- **Associativity:** [γ] · ([δ] · [ε]) = [(γ ∗ δ) ∗ ε] ≃ [γ ∗ (δ ∗ ε)] = [γ] · ([δ] · [ε]). The homotopy is a reparametrization: both concatenations traverse the same loops in the same order, just at different speeds.

The key point: **associativity holds only up to homotopy**, not on the nose. The paths (γ ∗ δ) ∗ ε and γ ∗ (δ ∗ ε) are different functions (they have different speeds at each time t), but they are homotopic. The fundamental group is well-defined only because we work with homotopy classes.

## π₁ and ∞-Groupoids

The "only up to homotopy" character of path composition is not a defect — it is the structure.

In HoTT, the type of loops Ω(X, x₀) = (x₀ =_X x₀) has exactly this structure: composition is associative up to a *path of paths* (a 2-path), and these higher paths satisfy their own coherence laws up to 3-paths, and so on. This is an *∞-groupoid*: a structure where composition and identities hold at each level only up to higher-level equivalences.

The fundamental group π₁(X, x₀) is the *1-truncation* of this ∞-groupoid: we declare all 2-paths and higher to be trivial, leaving only the group structure. This is the *0-truncation at the loop level* — setting all proofs of equality between paths to be equal.

Different spaces have ∞-groupoids with different amounts of non-trivial higher structure:
- *n-types* (homotopy n-types): spaces where all (n+1)-dimensional and higher path structure is trivial.
- *Sets* (0-types): paths between paths are all equal.
- *Groupoids* (1-types): paths between paths between paths are all equal. The fundamental groupoid (paths between all points, not just loops) captures this.
- *General types*: the full ∞-groupoid structure.

This is the hierarchy of types in HoTT. The real analysis of this section — defining paths precisely, computing with them, observing that composition is only associative up to homotopy — is the geometric foundation for this hierarchy.

Every time you prove that two proofs of the same proposition are equal in HoTT, you are constructing a path between paths — a homotopy. Every time you prove that two proofs of equality of proofs are equal, you are constructing a 3-dimensional path. The analysis of this section is where that higher-dimensional structure comes from.
