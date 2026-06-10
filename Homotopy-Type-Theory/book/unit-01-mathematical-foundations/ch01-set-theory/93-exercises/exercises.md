# Exercises: Set Theory

## Section 1: Naive Set Theory and Paradoxes

**1.1.** (Routine) Let A = {1, 2, 3}, B = {2, 3, 4, 5}. Compute A ∪ B, A ∩ B, A \ B, B \ A, and 𝒫(A ∩ B).

**1.2.** (Routine) Prove that for any sets A, B, C:
(a) A ∩ (B ∪ C) = (A ∩ B) ∪ (A ∩ C)  
(b) A ∪ (B ∩ C) = (A ∪ B) ∩ (A ∪ C)  
(c) A \ (B ∩ C) = (A \ B) ∪ (A \ C)  

**1.3.** (Routine) Prove using the definition of ⊆: for any sets A, B, (A ∩ B) ⊆ A ⊆ (A ∪ B).

**1.4.** (Standard) The *symmetric difference* A △ B = (A \ B) ∪ (B \ A). Prove:
(a) A △ B = B △ A  
(b) A △ (B △ C) = (A △ B) △ C (associativity)  
(c) A △ ∅ = A  
(d) A △ A = ∅  

(These show that (𝒫(X), △, ∅) is an abelian group for any set X.)

**1.5.** (Proof) State Cantor's paradox precisely. Identify exactly which step goes wrong in the proof that V = "the set of all sets" leads to contradiction, and which axiom of ZFC prevents the paradox.

**1.6.** (Proof) Explain in detail why the Separation axiom prevents Russell's paradox while naive comprehension does not. Explicitly state what goes wrong in the argument "let R = {x ∈ V | x ∉ x}" where V is "the set of all sets" — and why V itself is not a set in ZFC.

**1.7.** (Proof-level) The *Burali-Forti paradox* arises from assuming there is a "set of all ordinals." State the paradox precisely, using the definition of ordinals and the properties of ordinal arithmetic. Identify which axioms of ZFC prevent it.

## Section 2: ZFC Axioms

**2.1.** (Standard) Show that the Kuratowski encoding of ordered pairs, (a, b) = {{a}, {a, b}}, satisfies the fundamental property: (a, b) = (c, d) iff a = c and b = d. (This requires using only the Extensionality, Pairing, and the basic properties of sets.)

**2.2.** (Standard) Using the ZFC axioms, prove the existence of: (a) {∅, {∅}}, (b) ℕ as a set (at least the first 5 elements), (c) the Cartesian product A × B for two given sets A, B.

**2.3.** (Standard) Explain why the Axiom of Infinity is necessary to prove the existence of ω (the natural numbers as a set). Specifically: give an example of all the other ZFC axioms being satisfied in a "universe" that contains only hereditarily finite sets, showing that Infinity cannot be derived from the other axioms.

**2.4.** (Proof) Prove that in ZFC, every set has a unique "transitive closure": the smallest transitive set containing it. Use the Axiom of Union in your construction.

**2.5.** (Proof) The Axiom of Replacement says: the image of a set under a class function is a set. Use this to prove that {ω, ω+1, ω+2, ...} is a set (where ω+n is defined by the successor operation). Why is Separation insufficient for this?

**2.6.** (Proof-level) Prove from the Axiom of Foundation that no set is a member of itself (¬∃x. x ∈ x). Then prove there is no two-element cycle: ¬∃x.∃y. (x ∈ y ∧ y ∈ x).

## Section 3: Ordinals and Cardinals

**3.1.** (Routine) Write out the von Neumann representation of the first 5 natural numbers: 0, 1, 2, 3, 4.

**3.2.** (Routine) Compute in ordinal arithmetic: ω + 3, 3 + ω, ω · 2, 2 · ω. Explain in each case why the answers are what they are, in terms of the "structure" of the resulting well-ordered set.

**3.3.** (Standard) Prove the Cantor-Schröder-Bernstein theorem: if |A| ≤ |B| and |B| ≤ |A|, then |A| = |B|. (The standard proof constructs the bijection explicitly using a fixed-point argument on the power set.)

**3.4.** (Standard) Prove by diagonal argument that |ℕ| < |ℝ|. Your proof should make explicit which specific assumption leads to contradiction.

**3.5.** (Standard) Show that |ℕ × ℕ| = |ℕ| by exhibiting an explicit bijection. Then use this to show that |ℚ| = |ℕ| (the rationals are countable).

**3.6.** (Proof) Prove by transfinite induction: every ordinal α is a transitive set. (That is: if β ∈ α and γ ∈ β, then γ ∈ α.)

**3.7.** (Proof) Prove that there is no set of all ordinals. (This is the Burali-Forti paradox, but now prove it rigorously using the definition of ordinals and the properties of ZFC.) Specifically, show that if Ord were a set, it would itself be an ordinal, leading to Ord ∈ Ord, contradicting Foundation.

**3.8.** (Proof-level) State and prove the *Hartogs lemma*: for any set A, there is a smallest ordinal α not in bijection with any subset of A. (This is used to prove the Well-Ordering Theorem from AC in a different way.)

## Section 4: The Axiom of Choice

**4.1.** (Standard) Show that for finite families of non-empty sets, a choice function always exists without using the Axiom of Choice. What property of finite sets is being used?

**4.2.** (Standard) Prove that the following are equivalent (assuming ZF):
(a) Every surjection f: A → B has a right inverse (a function g: B → A with f ∘ g = id_B).  
(b) Every set of non-empty sets has a choice function.

**4.3.** (Proof) Prove Zorn's Lemma from the Well-Ordering Theorem. Specifically: given a poset P where every chain has an upper bound, use a well-ordering of P to construct a maximal element. (The construction is a transfinite recursion.)

**4.4.** (Proof) Use Zorn's Lemma to prove: every vector space has a basis. (A basis is a maximal linearly independent set. The poset to apply Zorn's Lemma to is the set of all linearly independent subsets, ordered by inclusion.)

**4.5.** (Proof-level) The statement "every infinite set has a countably infinite subset" is equivalent (over ZF) to a weak form of the Axiom of Choice called *Countable Choice for Finite Sets*. Prove the statement assuming full AC, then discuss why it requires some choice principle.

## Section 5: Limits of Set Theory

**5.1.** (Discussion) Benacerraf's problem: the number 2 can be encoded as {∅, {∅}} (von Neumann) or {{∅}} (Zermelo). In both encodings, 2 satisfies the Peano axioms for the second natural number. But the sets are different: in the von Neumann encoding, 1 ∈ 2; in the Zermelo encoding, 1 ∉ 2.

Is there a principled reason to prefer one encoding over the other? What does this tell us about the relationship between mathematical objects and their set-theoretic implementations?

**5.2.** (Discussion) The Dedekind cut construction of ℝ and the Cauchy completion construction of ℝ yield structures that are isomorphic as ordered fields. Are they "the same" real numbers? State precisely:
(a) What "isomorphic as ordered fields" means.
(b) In what sense they are the same (in mathematics as practiced).
(c) In what sense they are different (in ZFC).
(d) How HoTT's Univalence Axiom would handle this.

**5.3.** (Proof) The *Löwenheim-Skolem theorem* states: if ZFC (as a first-order theory) has any infinite model, it has a countable model. But ZFC proves the existence of uncountable sets. Explain this apparent paradox (the "Skolem paradox"). What does "uncountable" mean inside the model versus outside?

**5.4.** (Proof-level) In ZFC, proofs are formal sequences of sentences. A proof of "there exists a natural number with property P" does not tell you *which* number — it only establishes existence. Contrast this with a constructive proof, which would exhibit a witness.

Give an example of a theorem that:
(a) Can be proved in ZFC but where the ZFC proof provides no explicit witness.
(b) Can be proved constructively (with an explicit witness).

Then give an example of a theorem provable in ZFC but not constructively (i.e., a theorem that genuinely requires classical logic or the Axiom of Choice, and for which no constructive proof exists).
