# Propositions as Types, Revisited

## The Full Correspondence

When we first encountered the Curry-Howard correspondence, it was in the context of propositional logic and the simply typed lambda calculus. Propositions corresponded to types. Proofs corresponded to terms. Implication corresponded to function types. Conjunction corresponded to product types. Disjunction corresponded to sum types. Falsehood corresponded to the empty type.

That was propositional logic. With dependent types, we now have the full first-order correspondence. Every logical connective and quantifier has a type-theoretic counterpart, and the correspondence is not merely analogical — it is an identity.

| Logic | Type theory |
|---|---|
| Proposition P | Type P |
| Proof of P | Term of type P |
| P ⇒ Q | P → Q |
| P ∧ Q | P × Q |
| P ∨ Q | P + Q |
| ⊥ (falsehood) | 𝟘 |
| ⊤ (truth) | 𝟙 |
| ¬P | P → 𝟘 |
| ∀x ∈ A. P(x) | Π(x:A).P(x) |
| ∃x ∈ A. P(x) | Σ(x:A).P(x) |
| x = y (equality) | x =_A y (identity type) |

This table is not a dictionary between two separate languages. It is one language seen from two perspectives. The type theory is the logic. The logic is the type theory.

## Proof Relevance: Carrying the Witness

In classical logic and in most mathematical practice, a proof is a certificate. Once you have a proof that P holds, you discard it. The proof itself does not matter — only the conclusion does. Two proofs of the same statement are, from the mathematician's perspective, identical.

Dependent type theory is different. Proofs are terms. Terms can be examined, extracted, computed with. Two proofs of the same statement are in general different terms and can have genuinely different computational behavior.

This is called *proof relevance*. It is not a bug. Consider:

$$\mathsf{SortedPermutation}(l, l') = \mathsf{IsSorted}(l') \times \mathsf{IsPermutation}(l, l')$$

A proof that l' is a sorted permutation of l is a pair: a proof that l' is sorted, and a proof that l' is a permutation of l. The proof of IsSorted(l') is a data structure — it tells you explicitly that each element is ≤ the next. You can *run* it. You can use it to do a merge in O(n) time because you already know the structure.

In classical logic, "there exists a sorted permutation" does not come with this data. You know one exists, but you do not have it. In MLTT, the existential Σ(l':List A).SortedPermutation(l, l') gives you the permutation *and* the proof *and* all their computational content, in one package.

## The Difference Between ∀ and Π

In classical logic, ∀x ∈ A. P(x) is a proposition. It is true or false. If true, there is (by classical reasoning) some "witness" for each x, but the proof does not specify how to find it.

In MLTT, Π(x:A).P(x) is a function type. A proof of Π(x:A).P(x) is a function that, given x : A, *computes* a proof of P(x). The proof is the computation. It is not merely a certificate that P holds; it is a program that, on any input x, produces a proof of P(x) for that specific x.

This is a stronger notion. In logic, we say ∀n ∈ ℕ. ∃m ∈ ℕ. m > n (for every n there is a larger m) and we prove it non-constructively if we like. In MLTT, the proof of Π(n:ℕ).Σ(m:ℕ).(m > n) must actually *produce* the m. The proof is λn. (n+1, ≤-refl). It computes.

The MLTT version is stronger: it gives you a constructive procedure, not just an existence claim.

## The Difference Between ∃ and Σ

Similarly, the existential quantifier ∃x ∈ A. P(x) says "some x has property P" without specifying which one. In MLTT, Σ(x:A).P(x) contains the witness explicitly.

This matters for computation. If you have a proof h : Σ(n:ℕ).IsPrime(n) ∧ (n > 1000000), you can compute fst(h) and get the actual prime number. You do not need to run a prime search — the witness is right there, embedded in the proof.

This is why the Axiom of Choice is trivial in MLTT (as we saw in Section 3): the existential already carries the witness, so "choosing" is just extracting it.

## Proof-Irrelevant Propositions: Squash/Truncation

Sometimes you genuinely do not want to carry the witness. You want the mere *fact* that something exists, without the computational content. For example, if you are writing a specification that says "there exists a sorting algorithm of complexity O(n log n)," you do not want your specification to be tied to a *specific* algorithm.

For this purpose, HoTT introduces the *propositional truncation* (also called squash or truncation) of a type A:

$$\|A\| = \text{"the mere fact that A is inhabited"}$$

An element of ‖A‖ says: A has some element, but we do not specify which. Two elements of ‖A‖ are provably equal (‖A‖ is a mere proposition). The computational content is erased.

The propositional truncation is a higher inductive type with:
- Constructor: |−| : A → ‖A‖
- Path constructor: ∀(x y : ‖A‖). x = y (all elements are equal)

With truncation, we can recover classical-style existential quantification:

- Σ(x:A).P(x): an element of A together with a proof of P — fully proof-relevant
- ‖Σ(x:A).P(x)‖: the mere fact that such an x exists — proof-irrelevant

Both are expressible. The choice of which to use depends on whether you care about the computational content of the witness.

## What Proof Relevance Means for Equality

The most important consequence of proof relevance is for equality. In dependent type theory, the identity type a =_A b is a type. A proof of a = b is a term of this type. Two proofs of the same equality are two terms of the same type — and they can, in principle, be different terms.

In classical set-theoretic mathematics, equality is a relation. Either a = b or it does not. Two proofs of a = b are "the same proof" — there is nothing to distinguish them. Uniqueness of Identity Proofs (UIP) holds trivially.

In MLTT, UIP is a statement that can be questioned. It is not derivable from the basic rules. And it turns out — with the homotopy interpretation — to be false in general. The identity type a =_A b can have multiple distinct elements, corresponding to multiple distinct paths from a to b in the space A.

This is the door into HoTT. Once you take proof relevance seriously for equality — once you say "proofs of equality are data, and different proofs can be genuinely different" — you have committed to a world where types have topology. Where paths exist, homotopies can exist. Where homotopies exist, homotopies of homotopies can exist. And so on, all the way up.

## Why This Matters: The Computational Content of Mathematics

The propositions-as-types principle does something remarkable: it makes mathematics computational. Every proof is a program. Every theorem is a type. Running a proof means executing a program.

This is not merely a curiosity. Verified software systems like CompCert (a verified C compiler) and seL4 (a verified operating system kernel) are built on this principle. The correctness proofs are not separate from the code — they are the same artifact, typed differently. The compiler or kernel is a term; its correctness specification is its type; the proof of correctness is the fact that the term typechecks.

Similarly, the Coq proof assistant (which proves the Four Color Theorem and the Feit-Thompson theorem) and Agda and Lean are all founded on dependent type theory with the propositions-as-types principle. The proofs in these systems are executable programs in a total functional programming language. Extraction (in Coq) or compilation (in Agda) turns a proof into running code.

The propositions-as-types principle is not a philosophical curiosity. It is the foundation of a new style of mathematics where proof and computation are unified — where verifying a theorem and running a program are the same activity. HoTT extends this into geometry and topology, where paths and homotopies become first-class citizens of the mathematical universe.
