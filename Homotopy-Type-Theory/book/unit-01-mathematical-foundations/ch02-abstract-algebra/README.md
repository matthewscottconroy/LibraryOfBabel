# Chapter 2: Abstract Algebra

## The Symmetry of an Equilateral Triangle

Pick up an equilateral triangle. You can rotate it by 0°, 120°, or 240° and it looks the same. You can flip it across any of three axes of symmetry and it looks the same. Six operations total, each one a symmetry — and you can combine them: rotate, then flip, and you get another symmetry. Flip twice and you're back where you started. The identity operation (do nothing) is itself a symmetry.

This collection of six operations forms a *group*: the dihedral group D₃. It is one of the smallest non-trivial examples of a mathematical structure that runs through all of modern mathematics, from the classification of crystalline structures (the 230 space groups), to the study of particle physics (the Standard Model is built on the groups SU(3) × SU(2) × U(1)), to the theory of knots and surfaces and the fundamental theorem of Galois theory.

What makes groups so central? The abstraction. Once you see that rotations of a triangle, permutations of a set, residues modulo n, invertible matrices, and homotopy classes of loops all have the same algebraic structure — an associative operation, an identity, inverses — you have proved results about all of them simultaneously. Abstract algebra is the art of finding the right structure and exploiting it.

## The Free Group Is the Algebra of Paths

There is a specific reason group theory is foundational to HoTT. It is not just that topology uses groups (though it does — the fundamental group of a space measures its one-dimensional holes). It is that the most basic algebraic object in group theory, the *free group*, is the algebra of paths.

A free group is built from generators and their inverses, concatenated into words, with no imposed relations except the one you cannot avoid: a generator followed by its inverse is the identity. The free group on one generator is the integers: ..., a⁻², a⁻¹, e, a, a², .... The free group on two generators a, b is a much richer structure: words like a²b⁻¹aba⁻¹b³.

Now consider paths in a topological space. A path in a space X is a continuous function from [0,1] to X. You can concatenate two paths that share an endpoint: follow the first, then follow the second. You can reverse a path. The constant path is the identity. These operations do not exactly satisfy group axioms — path concatenation is associative only *up to homotopy*, not on the nose. And this is precisely the structure of a *groupoid*: a structure where composition and inverses hold up to higher equivalence.

In HoTT, this connection becomes a *definition*. The identity type a =_A b — the type of proofs that a equals b — is modeled by paths from a to b in the space A. Proof composition (transitivity of equality) is path concatenation. Reflexivity is the constant path. This is not an analogy: it is the axiom that motivates the entire theory.

The free group you study in this chapter is the fundamental group of a bouquet of circles — literally, the first higher inductive type you encounter when you start computing homotopy groups. And when HoTT proves π₁(S¹) = ℤ, it is using the universal property of the free group in the guise of the induction principle for the circle type.

## What We Cover

**Groups.** The axioms: associativity, identity, inverses. Immediate consequences (unique identity, unique inverses, cancellation). A zoo of examples: ℤ, ℤ/nℤ, Sₙ (symmetric groups), GL_n (invertible matrices), D_n (dihedral groups). Subgroups and their properties.

**Quotient Groups and Isomorphism Theorems.** Normal subgroups are the right notion of "symmetric" subgroup. Quotient groups collapse a subgroup to the identity. The three isomorphism theorems describe how quotients and homomorphisms interact. These are among the most-used results in all of algebra.

**Free Groups.** Words on generators, reduction, concatenation. The universal property: a homomorphism from F(S) is determined by where the generators go. Group presentations: every group is a quotient of a free group. The free group on S is the fundamental group of the wedge sum of |S| circles.

**Group Actions.** A group acting on a set: each group element is a permutation of the set. The orbit-stabilizer theorem relates the size of the orbit to the index of the stabilizer. Cayley's theorem: every group acts on itself by multiplication, so every group is a subgroup of a symmetric group.

**Rings and Modules.** Just enough: rings have two operations (addition and multiplication), modules are the analogue of vector spaces over a ring, and we need enough to understand homology groups (abelian groups = ℤ-modules) and later algebraic topology.

**The Bridge to Homotopy.** The fundamental group of a topological space is a group. The algebra of this chapter — quotient groups, free groups, presentations — is exactly the algebra of fundamental groups. Van Kampen's theorem computes π₁ of a pushout as the free product of groups. Higher homotopy groups are abelian (the Eckmann-Hilton argument). And in HoTT, all of this becomes definitional rather than merely analogical.

## The Style of Algebraic Reasoning

Abstract algebra has a distinctive proof style: arguments from axioms, without reference to specific elements. We prove that "the identity is unique" by showing that any element satisfying the identity axiom must equal the designated identity — using only the axioms, not any specific computation.

This is exactly the style of proof in a proof assistant. When you prove a theorem in Lean, you argue from hypotheses and rules. You do not compute with specific values; you reason structurally. Abstract algebra is the best training for this mode of thought, and the training matters because every theorem about groups that you prove here is a theorem you could, with some labor, formalize in Lean.

The bridge from algebra to type theory is not over a chasm. It is a gentle slope. By the end of this chapter, you will be standing near the top, looking at the type-theoretic landscape below.
