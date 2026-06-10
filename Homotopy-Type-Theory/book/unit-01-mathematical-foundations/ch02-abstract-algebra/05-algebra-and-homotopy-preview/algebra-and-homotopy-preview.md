# Algebra and Homotopy: A Preview

## The Dictionary

The fundamental insight of algebraic topology is this: topological spaces have algebraic invariants, and algebraic operations on the invariants reflect topological operations on the spaces. Algebraically isomorphic invariants imply topologically equivalent spaces.

Here is the correspondence table that we build toward throughout this curriculum:

| Algebra | Topology | Homotopy Type Theory |
|---------|----------|----------------------|
| Group G | Fundamental group π₁(X, x₀) | Loop space Ω(A, a) |
| Group element g | Loop at x₀ | Term p : a =_A a |
| Multiplication g · h | Path concatenation | Composition of identity proofs |
| Identity e | Constant loop | refl_a : a = a |
| Inverse g⁻¹ | Reversed loop | Symmetry of identity |
| Free group F(S) | π₁ of bouquet of circles | Identity type of HIT |
| Presentation ⟨S|R⟩ | CW complex | Higher inductive type |
| Normal subgroup N | Covering space | Fiber of a map |
| Quotient G/N | Deck transformation quotient | Quotient type |
| Homomorphism φ: G → H | Continuous map (on π₁) | Function (on identity types) |
| Isomorphism | Homotopy equivalence | Type equivalence |

Each row is a theorem in algebraic topology, not merely an analogy. In HoTT, it becomes a *definition*.

## The Fundamental Group as a Group

Let X be a topological space and x₀ ∈ X a basepoint. A *loop* at x₀ is a continuous function γ: [0,1] → X with γ(0) = γ(1) = x₀. Two loops are *homotopic* if one can be continuously deformed into the other while keeping the endpoints fixed.

**Definition.** The *fundamental group* π₁(X, x₀) is the set of homotopy classes [γ] of loops at x₀, with multiplication defined by concatenation: [γ] · [δ] = [γ * δ], where:

(γ * δ)(t) = γ(2t) for t ∈ [0, 1/2], δ(2t-1) for t ∈ [1/2, 1].

**Verification that this is a group:**
- *Identity:* The constant loop [c_{x₀}]: c(t) = x₀ for all t. Clearly [c * γ] = [γ] = [γ * c].
- *Inverses:* The reversed loop γ̄(t) = γ(1-t). Then γ * γ̄ ≃ c (homotopy by "shrinking" the path back to its start).
- *Associativity:* (γ * δ) * ε ≃ γ * (δ * ε) (homotopy by reparametrizing speed). Concatenation is associative *up to homotopy*.

The "up to homotopy" is crucial. Path concatenation is not strictly associative — the two sides traverse the loops in the same order but at different speeds. The homotopy adjusts the speeds. This is why π₁ is a *group*, not just a monoid: we must quotient by homotopy to get strict associativity.

**Example computations:**

π₁(ℝⁿ, x₀) = {e}. Any loop in ℝⁿ can be contracted to a constant: the straight-line homotopy H(t, s) = (1-s)γ(t) + sx₀ works.

π₁(S¹, x₀) = ℤ. Loops on the circle are classified by their *winding number*: how many times the loop goes around the circle (with sign for direction). The homotopy class of a loop is exactly its winding number. This is the key computation in algebraic topology, and it is a theorem in HoTT proved using the higher inductive type definition of S¹.

π₁(S², x₀) = {e}. The 2-sphere is simply connected: every loop can be contracted, because you can push the loop over the "top" of the sphere.

π₁(Torus, x₀) = ℤ × ℤ. Loops on the torus can wind independently in two directions (around the hole and around the tube). These two generators commute.

π₁(Figure eight, x₀) = F₂. Loops on the figure eight can go around either circle, in any order, without any commutativity relation. This is exactly the free group on two generators.

π₁(RP², x₀) = ℤ/2ℤ. The real projective plane has a single non-trivial loop that becomes trivial when traversed twice.

## The Seifert-van Kampen Theorem

This theorem computes fundamental groups of spaces built from simpler pieces.

**Theorem (Seifert-van Kampen).** If X = A ∪ B where A, B are open, A ∩ B is path-connected, and A, B, A ∩ B are each path-connected with basepoint x₀ ∈ A ∩ B, then:

π₁(X, x₀) = π₁(A, x₀) *_{π₁(A∩B, x₀)} π₁(B, x₀)

The right side is the *amalgamated free product*: take the free product π₁(A) * π₁(B), then impose the relations iₐ(γ) = i_b(γ) for all γ ∈ π₁(A ∩ B), where iₐ and i_b are the inclusions.

**Example.** Figure eight = S¹ ∪ S¹ at a single point. The intersection is the single basepoint with trivial fundamental group. Van Kampen gives π₁(S¹ ∨ S¹) = ℤ * ℤ = F₂. Correct.

**Example.** Torus = S¹ × S¹. Using van Kampen on the torus viewed as a square with edges identified: π₁(Torus) = ⟨a, b | aba⁻¹b⁻¹ = e⟩ = ℤ × ℤ. The relation aba⁻¹b⁻¹ = e says ab = ba — the generators commute.

Van Kampen is the bridge between algebra and topology: given a topological description of a space, it produces a group-theoretic description of its fundamental group. And the group-theoretic description (a presentation ⟨S|R⟩) is exactly the higher inductive type description of the corresponding type in HoTT.

## Higher Homotopy Groups

The fundamental group captures one-dimensional topology. Higher-dimensional topology requires *higher homotopy groups*.

**Definition.** For n ≥ 1, the n-th *homotopy group* πₙ(X, x₀) is the set of homotopy classes of continuous maps f: Sⁿ → X sending the basepoint of Sⁿ to x₀, with group operation given by "concatenation" of spheres.

π₁: classes of loops (maps S¹ → X).
π₂: classes of "bubbles" (maps S² → X).
π₃: classes of "3-sphere maps" (maps S³ → X).

**Key facts:**
- πₙ is abelian for n ≥ 2 (Eckmann-Hilton argument: two operations compatible in a specific sense must be equal and commutative).
- Computation is notoriously hard: πₙ(Sᵐ) for n > m is mostly unknown, despite being non-trivial.
- π₃(S²) = ℤ (the Hopf fibration, discovered 1931 — the first example of a non-trivial higher homotopy group of a sphere).

## The ∞-Groupoid Structure

Groups capture one-dimensional symmetry: elements, one operation, inverses. But the full structure of homotopy theory is higher-dimensional.

- Loops: 1-dimensional paths.
- Homotopies between loops: 2-dimensional paths-between-paths.
- Homotopies between homotopies: 3-dimensional.
- And so on.

The algebraic structure capturing all of this is an *∞-groupoid*: a structure with morphisms at every dimension, all invertible. Groups are ∞-groupoids with only one object and only non-trivial morphisms in dimension 1.

In HoTT, every type is an ∞-groupoid:
- 0-dimensional: the terms.
- 1-dimensional: the identity proofs (paths between terms).
- 2-dimensional: paths between identity proofs.
- n-dimensional: paths between (n-1)-dimensional paths.

The *n-truncated* types are those where all n+1-dimensional and higher paths are trivial:
- (-1)-truncated types (propositions/mere propositions): at most one term.
- 0-truncated types (sets): terms, but all identity proofs are trivially equal.
- 1-truncated types (groupoids): terms and paths, but all 2-paths are trivial.
- General types: the full ∞-groupoid structure.

The classification of types by homotopy level — the *homotopy n-types* — is a central organizing principle of HoTT. The algebra of this chapter (groups, quotients, free groups) is the algebra of 1-types. The algebra of higher types is still being developed, and HoTT is one of the main tools for making progress.

The fundamental group is where algebra meets topology. The ∞-groupoid structure is where it matures into the full homotopy-type-theoretic setting. Everything in this chapter has been preparation for seeing that the algebraic structures we studied — free groups, presentations, quotients, actions — are the low-dimensional shadows of a richer infinite-dimensional story.
