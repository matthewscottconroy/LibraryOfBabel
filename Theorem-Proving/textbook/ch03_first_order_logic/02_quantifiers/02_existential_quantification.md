# Existential Quantification: Witnesses and Evidence

> *"In mathematics, existence proofs are at once the most frustrating and the most powerful."*

---

A doctor says: "There exists a treatment that will cure this condition." An optimist says: "There exists a solution to every problem." A mathematician says: "There exists a prime greater than 1,000,000." These are existential claims — claims that something satisfying a certain description exists in the domain.

Existential quantification is the formal machinery for making such claims precise. And its relationship to proof is, in a deep sense, more demanding than universal quantification: to prove that something exists, you cannot just make a general argument — you must (in constructive mathematics, at least) produce the thing.

## The Semantics of ∃

The sentence ∃x P(x) is true in a structure M with domain D if and only if there is *at least one* element a ∈ D such that P(a) is true.

This is an infinite disjunction when D is infinite:
$$\exists x \, P(x) \equiv P(a_1) \vee P(a_2) \vee P(a_3) \vee \cdots$$

For ∃x P(x) to be true, one disjunct must be true — one element must satisfy P. The existential quantifier is falsified only if *every* element of the domain fails to satisfy P.

Notice the duality with universal quantification:
- ∀x P(x) is falsified by **one** counterexample; proved only by covering **all** elements
- ∃x P(x) is proved by **one** witness; falsified only by checking **all** elements

This duality is captured by the quantifier negation laws:
$$\neg\forall x \, P(x) \equiv \exists x \, \neg P(x)$$
$$\neg\exists x \, P(x) \equiv \forall x \, \neg P(x)$$

## Proof by Witness: The Introduction Rule

The natural way to prove ∃x P(x) is to produce a **witness** — a specific term t — and verify that P(t) holds. This is existential introduction:

$$\frac{P(t)}{\exists x \, P(x)}$$

This is the constructive core of existence proofs. To prove "there exists an even prime," I produce the witness 2 and verify that 2 is even (2 = 2·1) and prime (it has no divisors other than 1 and 2).

In Lean 4, this is the pair constructor: `⟨2, by norm_num⟩ : ∃ n : ℕ, n.Prime ∧ n % 2 = 0`.

## Existential Elimination: Using an Existence Claim

What can you do with the knowledge that ∃x P(x)? You cannot conclude P(t) for any specific t — you do not know *which* element witnesses the existence. But you *can* reason about an *arbitrary* element that satisfies P and derive consequences from it.

The elimination rule is:

$$\frac{\exists x \, P(x) \quad [P(a)] \vdash Q}{\quad Q}$$

"From ∃x P(x), and from the fact that if P(a) holds (for a fresh a) then Q holds, conclude Q."

The key constraint is that a must be **fresh** — it must not appear in Q or in any other undischarged assumption. If you are allowed to use properties specific to a, you could smuggle in information about the particular witness, which you are not supposed to have.

In Lean 4:
```lean
-- From h : ∃ x, P x, extract the witness with `obtain`
obtain ⟨a, ha⟩ := h
-- Now a : α and ha : P a are in context
-- a is fresh; we can use ha but cannot assume anything else about a
```

## Non-Constructive Existence: A Philosophical Divide

Here is one of the deepest fault lines in the philosophy of mathematics. Consider this argument:

> **Claim**: There exist irrational numbers a and b such that aᵇ is rational.

**Proof**: Consider √2^√2. Either this number is rational or irrational.
- *Case 1*: √2^√2 is rational. Then a = b = √2 works (both irrational, and aᵇ is rational).
- *Case 2*: √2^√2 is irrational. Then let a = √2^√2 and b = √2. We have aᵇ = (√2^√2)^√2 = √2^(√2·√2) = √2² = 2, which is rational.

In either case, we have produced the required a and b. The theorem is proved. ∎

But wait — this proof does not tell us *which* case holds! It proves existence without exhibiting a specific witness. Is √2^√2 rational or irrational?

(Spoiler: the Gelfond-Schneider theorem (1934) implies √2^√2 is transcendental, hence irrational, so Case 2 is the actual case and a = √2^√2, b = √2 are the witnesses. But the proof above works without knowing this.)

For a classical logician, this is a perfectly valid existence proof. The law of excluded middle guarantees that either Case 1 or Case 2 holds, and in either case we have the required witnesses.

For an **intuitionist** or **constructivist**, this proof is not acceptable. A constructive existence proof must produce an explicit witness — an algorithm that computes the witness. A proof that says "there exists a value, and it's either this or that, but I don't know which" provides no algorithm, and therefore (from the constructive perspective) no real knowledge.

This is not an academic distinction. Proof assistants reflect it:
- In **Coq** (without classical axioms), you cannot use `destruct (classic P)` to case-split on P ∨ ¬P unless you can prove that case-splitting will not introduce an uncomputable witness.
- In **Lean 4** (which uses classical logic by default), you can use `Classical.em` freely and produce non-constructive existence proofs.
- In **Agda** (default: constructive), fully non-constructive proofs are rejected.

## Unique Existence: ∃!

The notation ∃!x P(x) means "there exists a *unique* x satisfying P":
$$\exists!x \, P(x) \;\equiv\; \exists x \, P(x) \wedge \forall x \, \forall y \, (P(x) \wedge P(y) \rightarrow x = y)$$

Unique existence is the standard form for mathematical definitions. "There exists a unique additive identity in a group" asserts both that 0 exists and that no other element serves as an additive identity. Proving ∃!x P(x) requires two things:
1. **Existence**: find a witness satisfying P
2. **Uniqueness**: show that any two witnesses are equal

---

*Next: Nested quantifiers — where the real power (and difficulty) of FOL resides.*
