# Exercises: Abstract Algebra

## Section 1: Groups

**1.1.** (Routine) Verify that each of the following is a group:
(a) (ℤ, +, 0)  
(b) (ℝ \ {0}, ·, 1)  
(c) (GL₂(ℝ), ·, I) — 2×2 invertible real matrices under multiplication.  

For each, identify the identity and describe the inverse operation.

**1.2.** (Routine) Which of the following are groups? Explain.
(a) (ℕ, +, 0)  
(b) (ℤ, ·, 1)  
(c) ({e, a, b} with multiplication table: e·e=e, e·a=a, e·b=b, a·a=e, a·b=b, b·b=e, b·a=a)

**1.3.** (Standard) Prove: in any group G, (ab)⁻¹ = b⁻¹a⁻¹ for all a, b ∈ G.

**1.4.** (Standard) Let G be a group and a ∈ G. Prove: if a² = e, then a = a⁻¹. If every element of G satisfies a² = e, prove G is abelian.

**1.5.** (Standard) Prove Lagrange's theorem: if H ≤ G is a subgroup of a finite group G, then |H| divides |G|. Your proof should explicitly identify the partition of G into cosets and show the cosets have equal size.

**1.6.** (Proof) Let G be a group of order p (prime). Prove G is cyclic and isomorphic to ℤ/pℤ. (Hint: every non-identity element has order dividing p, hence order p.)

**1.7.** (Proof) The *center* of a group G is Z(G) = {z ∈ G | zg = gz for all g ∈ G}. Prove:
(a) Z(G) is a normal subgroup of G.  
(b) G/Z(G) cyclic implies G abelian.  
(c) If G has order p² for prime p, then G is abelian.

**1.8.** (Proof) Prove the First Isomorphism Theorem: if φ: G → H is a group homomorphism, then G/ker(φ) ≅ Im(φ). Your proof should explicitly define the isomorphism and verify it is well-defined, a homomorphism, injective, and surjective.

## Section 2: Free Groups

**2.1.** (Routine) Reduce the following words in F({a, b}):
(a) aba⁻¹a  
(b) ab⁻¹ba⁻¹  
(c) a⁻¹aa⁻¹  
(d) abba⁻¹b⁻¹a⁻¹  

**2.2.** (Standard) Show that the free group F({a}) is isomorphic to ℤ. Exhibit an explicit isomorphism and verify the homomorphism property.

**2.3.** (Standard) Using the universal property of F({a, b}), construct a homomorphism φ: F({a, b}) → S₃ such that φ(a) = (12) and φ(b) = (123). Compute φ(ab), φ(a²b⁻¹), φ(aba⁻¹).

**2.4.** (Proof) Prove that F(S) as defined (reduced words under concatenation-then-reduction) satisfies the universal property stated in the text. Specifically: define the map φ explicitly and prove it is the unique homomorphism extending f: S → G.

**2.5.** (Proof) Prove that every subgroup of a free group is free. (This is the Nielsen-Schreier theorem. You may sketch the proof by thinking about how subgroups of a free group correspond to covering spaces of a wedge of circles.)

**2.6.** (Proof-level) Show that the word problem for free groups is decidable: describe an algorithm that, given two words in F(S), determines whether they represent the same element. Prove your algorithm is correct.

## Section 3: Group Actions

**3.1.** (Standard) Let D₄ be the dihedral group of symmetries of the square. Describe the action of D₄ on the set of vertices {1, 2, 3, 4}. Write out the permutation representation φ: D₄ → S₄.

**3.2.** (Standard) Let G act on X and let x ∈ X. Prove:
(a) Stab(x) is a subgroup of G.  
(b) For g ∈ G, Stab(g·x) = g·Stab(x)·g⁻¹ (conjugate of the stabilizer).

**3.3.** (Standard) Use the orbit-stabilizer theorem to count: how many distinct necklaces can be made using 6 beads, each colored one of 3 colors, up to rotation? (Use Burnside's lemma.)

**3.4.** (Proof) Prove Cayley's theorem: every group G is isomorphic to a subgroup of Sym(G). Be explicit about the action used and the homomorphism φ: G → Sym(G).

**3.5.** (Proof) A group action is *free* if the only element g ∈ G with g·x = x for some x ∈ X is g = e (the only element with any fixed points is the identity). Show that if G acts freely on a finite set X and |G| = n, then |G| | |X| and X decomposes into |X|/n orbits of size exactly n.

**3.6.** (Proof-level) Describe the correspondence between connected covering spaces of a graph (1-dimensional CW complex) and subgroups of its fundamental group. Give an example: find the three connected covering spaces of the figure-eight graph corresponding to the three subgroups of index 2 in F₂.

## Section 4: Rings and Modules

**4.1.** (Routine) For each of the following, determine if it is a ring. If so, determine if it is commutative, has unity, is an integral domain, or is a field.
(a) (ℤ, +, ·)  
(b) (2ℤ, +, ·) — even integers  
(c) (ℚ, +, ·)  
(d) (M₂(ℝ), +, ·) — 2×2 real matrices  

**4.2.** (Standard) Determine all ideals in ℤ. Prove your answer. Which are maximal? Which are prime?

**4.3.** (Standard) Prove that if φ: R → S is a ring homomorphism, then ker(φ) is an ideal of R and Im(φ) is a subring of S. State and prove the First Isomorphism Theorem for rings.

**4.4.** (Standard) Prove that ℝ[x]/⟨x²+1⟩ ≅ ℂ. Define the isomorphism explicitly.

**4.5.** (Proof) A commutative ring R is an *integral domain* iff the only zero-divisors are 0. Prove: ℤ[x] is an integral domain. (Hint: consider leading coefficients.)

**4.6.** (Proof-level) Prove the structure theorem for finitely generated abelian groups: every finitely generated abelian group is isomorphic to ℤ^r ⊕ ℤ/n₁ℤ ⊕ ... ⊕ ℤ/n_kℤ where n₁ | n₂ | ... | n_k. (You may use the Smith normal form of integer matrices.)

## Section 5: Connections to Topology and HoTT

**5.1.** (Standard) Compute the fundamental group of the following spaces using van Kampen's theorem:
(a) S¹ (the circle)  
(b) S¹ ∨ S¹ (the figure eight)  
(c) The torus T = S¹ × S¹  
(d) The Klein bottle (describe the CW structure and apply van Kampen)  

**5.2.** (Standard) The presentation of a group ⟨a, b | a², b², (ab)ⁿ⟩ is the dihedral group D_n. Verify this by showing the presented group satisfies the dihedral group axioms and has order 2n.

**5.3.** (Proof) In HoTT, the fundamental group of S¹ (the circle HIT with one base point and one loop) is ℤ. Sketch the proof of π₁(S¹) ≅ ℤ using the universal property of the circle type:
- State the universal property (induction principle).
- Define the map π₁(S¹) → ℤ by "winding number."
- Define the map ℤ → π₁(S¹) by "loop^n."
- Argue why these are inverses.

**5.4.** (Proof-level) The Eckmann-Hilton argument shows that a set with two compatible binary operations (each is a unital morphism with respect to the other) is actually commutative and the two operations are equal. State and prove the Eckmann-Hilton argument. Then explain why this implies π₂(X) is abelian for any topological space X.
