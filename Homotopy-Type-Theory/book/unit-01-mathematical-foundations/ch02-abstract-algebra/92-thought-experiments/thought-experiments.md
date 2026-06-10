# Thought Experiments: Abstract Algebra

## 1. The Alien Symmetry Group

Suppose an alien civilization has developed a concept of "symmetry" that is genuinely different from ours. Their symmetries satisfy something like our group axioms, but their "inverse" operation does not satisfy aa⁻¹ = e — instead, it satisfies (a⁻¹)⁻¹ = a and (ab)⁻¹ = a⁻¹b⁻¹ (instead of our (ab)⁻¹ = b⁻¹a⁻¹). Call this an "alien group."

Question: are alien groups different from groups, or just isomorphic to groups with a different convention? If you can exhibit a bijection that turns any alien group into a group (preserving some structure), then they are "the same." If no such bijection exists for some alien groups, they are genuinely different.

This thought experiment is essentially the question: what structure is being captured by the group axioms? If you change the axioms slightly, do you get a different theory or the same theory in different notation? This is a question about the *expressiveness* and *completeness* of an axiom system.

In HoTT, this becomes a question about what structure a type must have to "be" a group in the appropriate sense. The answer is given by the type-theoretic definition of a group structure, and two group structures on the same type are the same iff they are connected by a path in the type of group structures.

## 2. The Free Group as Infinite Memory

A free group F(S) on generators S is, in a sense, an infinite memory device. Every finite sequence of generators (with repetition and inversion) is an element of F(S). The group "remembers" every choice ever made — every generator applied, in order, with no simplification except mandatory cancellations.

Contrast this with a quotient group F(S)/N: we have "forgotten" some information, declaring certain words to be the same as the identity. The group ℤ/nℤ = ⟨a | aⁿ = e⟩ has "forgotten" how many full cycles of a we have completed.

Question: is there a useful notion of "information content" of a group? A free group retains maximal information; a quotient retains less. The trivial group retains no information. Is this a genuine measure — something like entropy or Kolmogorov complexity — for groups?

This is related to the concept of *group cohomology*, which measures certain "hidden structure" in group extensions. And in the type-theoretic setting, it is related to the question of *proof relevance*: how much information does a proof of a proposition carry? A mere proposition (h-proposition) carries no information beyond its truth. A set carries the information of which element it is. A higher type carries more. The analogy between "information in a group quotient" and "information in a type truncation" is not accidental.

## 3. Non-Associative Algebra

The group axioms require associativity: (ab)c = a(bc). This seems natural, even obvious. But many important algebraic structures are non-associative:

- *Lie algebras*: [a,b] = -[b,a] and [[a,b],c] + [[b,c],a] + [[c,a],b] = 0 (Jacobi identity). Not associative in general.
- *Octonions*: An 8-dimensional normed division algebra. Division algebras must satisfy |ab| = |a||b|. The reals, complex numbers, and quaternions are associative. The octonions are not.
- *Alternative algebras*: a(ab) = (aa)b and (ab)b = a(bb). A weakening of associativity.

Question: which parts of group theory depend essentially on associativity? The proof of uniqueness of identity uses associativity. The proof of uniqueness of inverses uses associativity. Lagrange's theorem uses cosets, which require the associativity to define correctly.

What would a "non-associative fundamental group" look like? In higher homotopy theory, path composition is only associative *up to homotopy*. In HoTT, the identity type associativity law (p · (q · r) = (p · q) · r) holds only up to a 2-path — a path of paths. The weakening of associativity is not a defect; it is the precise structure of ∞-groupoids.

## 4. The Simplicity of A₅

The alternating group A₅ (even permutations of 5 elements, order 60) is the smallest non-abelian *simple group* — a group with no proper normal subgroups. Its simplicity means it cannot be decomposed further: it has no "smaller parts."

The Galois group of a general degree-5 polynomial is S₅ (symmetric group on 5 elements), and the fact that A₅ is simple (non-abelian simple) is the precise reason the quintic is not solvable by radicals. A composition series for S₅ includes A₅, and A₅ is simple but non-abelian — the composition factor that fails the solvability test.

Question: what is "simplicity" in a group? Why does it obstruct solvability? Can you see, intuitively, why "no normal subgroups" means "cannot be broken down further"? 

Now transport this to type theory: what is the analogue of a simple group in HoTT? A "simple" higher inductive type would be one with no non-trivial quotient maps. The classification of simple groups (the massive theorem completed in 1983 after decades of work) is the classification of the "atoms" of finite group theory. What would classifying the "atoms" of higher-dimensional HoTT types look like?

## 5. The Word Problem

Every group has a *word problem*: given two words in the generators and their inverses, are they equal in the group? For free groups, the word problem is decidable: reduce both words and check if the reductions are equal. For groups with relations, it may not be.

In 1955, Novikov and Boone independently proved that there exist finitely presented groups — groups given by finitely many generators and finitely many relations — whose word problem is *undecidable*. No algorithm can determine, for arbitrary words in the generators, whether they represent the same group element.

This is remarkable: the word problem is logically simple (are two words equal?), and the group is finite to describe (finitely many generators and relations), yet the problem is undecidable. It is as hard as the Halting Problem.

Question: what does this tell us about the relationship between algebra and computation? If we want to formalize group theory in a proof assistant, we need to compute with group elements. But if the word problem is undecidable, we cannot always determine equality of group elements algorithmically. How do proof assistants like Lean handle this? (The answer involves definitional equality vs propositional equality — and is directly relevant to the equality issues in HoTT.)

## 6. Categorical Groups and Higher Symmetry

A group is a category with one object and all morphisms invertible. This categorical perspective — due to Eilenberg and Mac Lane in the 1940s — opens up the possibility of *higher groups*.

A *2-group* is a category where all morphisms are invertible, with a strict or weak group structure on both objects and morphisms. A *∞-group* is the corresponding structure at all heights — an ∞-groupoid with one object.

In HoTT, every type with a basepoint has a *loop space* Ω(A, a) = (a =_A a), which is the type of loops. The loop space is a group (up to higher homotopy). The double loop space ΩΩ(A, a) is abelian (by Eckmann-Hilton). This is the Eckmann-Hilton argument: two compatible operations on a structure must be equal and commutative.

Question: what does "symmetry" mean when you allow not just invertible operations, but invertible operations between operations, invertible operations between those, and so on? The answer — ∞-groups and ∞-groupoids — is one of the central structures of modern mathematics. HoTT provides the language to reason about them formally. What new kinds of symmetry does this higher structure capture that ordinary groups miss?
