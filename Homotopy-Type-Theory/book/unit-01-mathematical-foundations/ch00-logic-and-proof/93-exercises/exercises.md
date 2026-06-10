# Exercises: Logic and Proof

## Section 1: Propositional Logic

**1.1.** (Routine) Determine which of the following are well-formed formulas of propositional logic. For those that are not, explain why.

(a) P ∧ Q → R  
(b) ¬¬P  
(c) → P Q  
(d) P ∧  
(e) (P → (Q → P))  
(f) ¬(P ↔ (Q ∧ ¬R))  

**1.2.** (Routine) List all subformulas of the following formula: (P → Q) ∧ (¬Q ∨ R).

**1.3.** (Routine) Construct truth tables for:

(a) P → (Q → P)  
(b) (P ∧ Q) → (P ∨ Q)  
(c) (P → Q) ↔ (¬P ∨ Q)  
(d) ¬(P ∧ Q) ↔ (¬P ∨ ¬Q)  

Identify which are tautologies.

**1.4.** (Routine) Use truth tables to determine whether {P → Q, Q → R} ⊨ P → R.

**1.5.** (Computational) Determine whether the following formula is satisfiable: (P ∨ Q) ∧ (¬P ∨ R) ∧ (¬Q ∨ ¬R) ∧ (P ∨ ¬R). If satisfiable, exhibit a satisfying assignment.

**1.6.** (Standard) Show that the biconditional is definable in terms of ∧ and ¬. That is, find a formula using only ∧ and ¬ that is logically equivalent to P ↔ Q.

**1.7.** (Standard) Show that all five connectives (¬, ∧, ∨, →, ↔) can be defined in terms of NAND alone, where NAND(P, Q) is defined as ¬(P ∧ Q). Provide the defining equivalences.

**1.8.** (Proof) Prove that any formula is logically equivalent to a formula in conjunctive normal form (CNF). (Hint: proceed by structural induction on formulas. Handle negations using De Morgan's laws and double negation.)

**1.9.** (Proof-level) The *Sheffer stroke* P|Q (read "P nand Q") satisfies NAND(P,Q). The *Peirce arrow* P↓Q satisfies NOR(P,Q) = ¬(P ∨ Q). Show that {|} alone and {↓} alone are each *functionally complete* — every propositional formula is logically equivalent to a formula using only that connective.

## Section 2: Proof Techniques

**2.1.** (Routine) Give a direct proof of: for all integers n, if n is odd, then n² is odd.

**2.2.** (Routine) Give a proof by contrapositive of: for all integers a, b, if a + b is even, then a and b have the same parity.

**2.3.** (Standard) Give a proof by contradiction that √3 is irrational.

**2.4.** (Standard) Prove: for any integers a, b, c, if a ∤ b (a does not divide b) and a ∤ c, it is not necessarily the case that a ∤ bc. Give a counterexample, then find a condition under which the implication holds.

**2.5.** (Standard) Prove or disprove: for all positive integers a, b, gcd(a, b) = gcd(a, a + b).

**2.6.** (Proof) Prove: there is no largest prime number. Your proof should be careful about the distinction between the infinitely-many-primes conclusion and the no-largest-prime conclusion.

**2.7.** (Proof) Prove: for any prime p and integers a, b, if p ∣ ab then p ∣ a or p ∣ b. (This is Euclid's lemma. Hint: use the fact that gcd(p, a) is either 1 or p, and Bézout's identity.)

**2.8.** (Proof) Give two different proofs that every positive integer is either even or odd. One proof should use strong induction. The other should use only the properties of integer division.

## Section 3: Mathematical Induction

**3.1.** (Routine) Use simple induction to prove: 1² + 2² + ... + n² = n(n+1)(2n+1)/6 for all n ≥ 1.

**3.2.** (Routine) Use simple induction to prove: for all n ≥ 1, 3 ∣ n³ - n.

**3.3.** (Standard) Use strong induction to prove: every integer n ≥ 2 is divisible by some prime.

**3.4.** (Standard) Define the sequence a₀ = 1, a₁ = 3, aₙ = 2aₙ₋₁ + aₙ₋₂. Use strong induction to prove: aₙ < 3ⁿ for all n ≥ 0.

**3.5.** (Standard) Use structural induction on the set of propositional formulas to prove: the number of occurrences of binary connectives in a formula φ equals the number of atomic propositions minus 1.

**3.6.** (Standard) Define full binary trees inductively: a leaf is a full binary tree; a node with two full binary tree subtrees is a full binary tree. Prove by structural induction: every full binary tree has an odd number of nodes.

**3.7.** (Proof) The *Ackermann function* is defined by:
- A(0, n) = n + 1
- A(m, 0) = A(m - 1, 1) for m > 0
- A(m, n) = A(m - 1, A(m, n - 1)) for m, n > 0

Prove that A(m, n) is defined for all m, n ∈ ℕ by well-founded induction on the lexicographic order on ℕ × ℕ.

**3.8.** (Proof) State the *Principle of Complete Induction* as a formal logical theorem (a single sentence of predicate logic) and prove it from the *Principle of Simple Induction* using the property of natural numbers that every non-empty set has a least element.

**3.9.** (Proof-level) Suppose we have a set S with an inductive definition: S contains a base element 0, and if n ∈ S then S(n) ∈ S. Suppose also that S contains no other elements. Define a *motive* P: S → Prop and an induction principle for S. Prove: for any motive P, if P(0) holds and P(n) → P(S(n)) for all n ∈ S, then P holds for all elements of S.

## Section 4: Predicate Logic

**4.1.** (Routine) Determine FV(φ) for each of the following formulas:

(a) ∀x. P(x, y)  
(b) ∃y. (P(x, y) → ∀z. R(y, z))  
(c) ∀x. ∃y. (x < y)  
(d) P(x) ∧ ∀x. Q(x, z)  

**4.2.** (Routine) Compute the substitution φ[t/x] for:

(a) (∀y. P(x, y))[f(z)/x]  
(b) (∃x. R(x, y))[a/y]  
(c) (∀y. y < x)[y/x] — explain why this requires renaming.  

**4.3.** (Standard) Express each of the following in predicate logic. Identify the domain and the predicates.

(a) "Every even integer greater than 2 is the sum of two primes." (Goldbach's conjecture)  
(b) "There is no largest natural number."  
(c) "The sequence (aₙ) converges to L."  

**4.4.** (Standard) Prove the duality of quantifiers using the semantic definition: show that M,σ ⊨ ¬∀x. φ iff M,σ ⊨ ∃x. ¬φ.

**4.5.** (Proof) Use natural deduction to derive: ∀x. (P(x) → Q(x)), ∀x. P(x) ⊢ ∀x. Q(x). Write out the derivation tree explicitly.

**4.6.** (Proof) Show that the following is not a valid first-order inference: ∃x. P(x), ∃x. Q(x) ⊢ ∃x. (P(x) ∧ Q(x)). Give a concrete counterexample (a structure and interpretation in which the premises are true but the conclusion false).

**4.7.** (Proof-level) State and prove: if φ is a first-order sentence and T is a theory (a set of sentences), then T ⊨ φ iff T ∪ {¬φ} is unsatisfiable. (This is the *refutation completeness* of FOL and is used in automated theorem proving.)

## Section 5: Compactness and Completeness

**5.1.** (Standard) Use the compactness theorem to show: the theory of dense linear orders without endpoints has no finite models, yet every finite subset of its axioms has a finite model.

**5.2.** (Standard) Use compactness to show: if a first-order theory T has arbitrarily large finite models, then T has an infinite model. (Hint: add sentences saying "there are at least n elements" for every n.)

**5.3.** (Proof) State the Compactness Theorem for propositional logic and give a complete proof. Your proof should construct the satisfying assignment explicitly, as done in the text.

**5.4.** (Proof) Prove: every set of propositional formulas that is *finitely satisfiable* can be extended to a *maximal finitely satisfiable* set. (A maximal such set contains, for every formula φ, either φ or ¬φ.) This is used in the model-theoretic completeness proof.

**5.5.** (Proof-level) The *upward Löwenheim-Skolem theorem* states: if a first-order theory has an infinite model, it has models of every infinite cardinality. Use the compactness theorem to prove the upward direction: if T has an infinite model, T has a model of cardinality κ for any infinite κ ≥ |T|.

**5.6.** (Research-level) The *interpolation theorem* (Craig, 1957) states: if Γ ⊨ φ → ψ, then there is a formula χ (the *interpolant*) involving only predicates appearing in both φ and ψ such that Γ ⊨ φ → χ and Γ ⊨ χ → ψ. State this precisely, give a proof sketch, and explain its significance for modular reasoning and automated verification.
