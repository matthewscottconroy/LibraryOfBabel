# Important Thinkers: Identity Types and Paths

## Per Martin-Lof (1942–)

Per Martin-Lof is the architect of the framework within which all of this chapter's mathematics lives. His constructive type theory, developed through the 1970s and published in mature form in 1984, introduced the identity type with its formation, introduction, and elimination rules. The J rule — path induction — is Martin-Lof's contribution.

Martin-Lof's original interpretation of the identity type was intensional: proofs of equality are terms, and different proof terms are not automatically equal. This was in deliberate contrast to the extensional interpretation, where all proofs of equality are identified. Martin-Lof's intensional interpretation is what makes HoTT possible — it preserves the information in equality proofs rather than collapsing it.

It is worth pausing on the philosophical audacity of Martin-Lof's program. He was not simply developing a convenient notation for logic. He was arguing that type theory provides a *foundation* for mathematics — one where proofs are mathematical objects, where truth and existence coincide, and where the distinction between syntax and semantics is dissolved. His four lectures "Intuitionistic Type Theory" (1980) remain essential reading.

## Vladimir Voevodsky (1966–2017)

Voevodsky's contribution to the material of this chapter is the *homotopy interpretation*. Although the identity type had been in type theory for decades, it was Voevodsky who first clearly understood that identity types are path spaces, and that this interpretation is not merely a metaphor but a precise mathematical fact, validated by the simplicial set model.

Voevodsky was a Fields Medal winner (2002) for his work in algebraic geometry and motivic cohomology — work that itself required sophisticated homotopy theory. When he turned to foundations around 2005, he brought with him the full machinery of modern homotopy theory and asked: what is the correct foundational framework for homotopy-invariant mathematics?

The answer he arrived at — univalent foundations, built on Martin-Lof type theory with the Univalence Axiom — is the subject of this book. But the key move, the one that unlocks everything, is the homotopy interpretation of the identity type. That move is Voevodsky's.

His untimely death in 2017 was a profound loss. The Univalence Foundation program continues, but without its originator.

## Martin Hofmann and Thomas Streicher (1990s)

Hofmann and Streicher deserve mention for the *groupoid model* — the first model that demonstrated that identity types need not be propositional. In their 1994 paper "The Groupoid Interpretation of Type Theory," they constructed a model where every type is interpreted as a groupoid (a category where every morphism is invertible), and identity proofs are morphisms.

The groupoid model showed two things: first, that UIP (the axiom that all proofs of equality are equal) is independent of Martin-Lof type theory — it cannot be proved from J alone. Second, that identity types can have genuinely non-trivial structure. This was the first glimpse of what Voevodsky would later develop fully: types have homotopy-theoretic structure at every dimension.

## Christine Paulin-Mohring and Thierry Coquand (1990s–2000s)

Much of the practical theory of path induction — the precise form of the J rule, its interaction with other eliminators, the details of computation rules — was worked out in the context of the Coq proof assistant and the Calculus of Inductive Constructions. Paulin-Mohring's inductive types and Coquand's contributions to dependent type theory provided the computational foundation on which HoTT's path computations rest.

## The HoTT Book Authors (2013)

The formalization and systematization of the ideas in this chapter was largely accomplished by the collective effort that produced the Homotopy Type Theory Book in 2013. Written collaboratively by researchers including Steve Awodey, Peter Aczel, Carlo Angiuli, Benedikt Ahrens, and many others, the HoTT Book established the canonical presentation of identity types in the homotopy-theoretic setting.

In particular, the formalization of path operations — concatenation, inversion, all five groupoid laws, transport, ap, and apd — was done with full formal precision in this collaborative work, making it possible to formalize and verify all of these constructions in proof assistants.
