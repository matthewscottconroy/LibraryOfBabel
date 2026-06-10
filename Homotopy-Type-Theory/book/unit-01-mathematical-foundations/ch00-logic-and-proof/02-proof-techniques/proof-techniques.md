# Proof Techniques

## The Art of Persuading a Machine

A proof is not an argument. An argument tries to persuade a person, and people can be persuaded by incomplete chains of reasoning, by analogy, by intuition, by a confident voice. A proof is meant to convince a machine — or, more precisely, to be checkable by an idealized machine that demands explicit justification for every single step.

This may sound like an impoverishment. In fact it is a liberation. If you can convince the machine, you have established something permanently. No subsequent experiment, no new perspective, no future disagreement can overturn it. The proof is either correct or it is not. There is no "mostly right."

The techniques in this section are strategies for constructing proofs. Each one corresponds to a logical principle, and each logical principle corresponds — under the Curry-Howard correspondence — to a type-theoretic construction. We will note these correspondences as we go, not to distract from the mathematics but to make the later connections natural.

## Direct Proof

The most straightforward technique: assume the hypothesis, and reason forward to the conclusion.

**Logical principle:** To prove φ → ψ, assume φ and derive ψ.

**Example.** Prove: if n is even, then n² is even.

*Proof.* Assume n is even. By definition, there exists an integer k such that n = 2k. Then n² = (2k)² = 4k² = 2(2k²). Since 2k² is an integer, n² is divisible by 2, hence even. □

The proof is direct: we assume the hypothesis, unpack its definition, compute, and observe that the conclusion holds. Every step is explicit.

**Type-theoretic reading.** A direct proof of P → Q is a function: given a proof p of P, we construct a proof of Q. In the example, given the integer k witnessing that n is even, we construct 2k² as a witness that n² is even. The function is: k ↦ 2k².

**Example.** Prove: for all integers a, b, if a ∣ b and b ∣ c, then a ∣ c. (Divisibility is transitive.)

*Proof.* Assume a ∣ b and b ∣ c. Then there exist integers j, k with b = aj and c = bk. Substituting: c = (aj)k = a(jk). Since jk is an integer, a ∣ c. □

Notice the structure: we unpacked two existential hypotheses (obtaining witnesses j and k), performed an algebraic manipulation, and repacked into an existential conclusion (jk is the witness). This unpacking-manipulation-repacking pattern appears in nearly every direct proof.

## Proof by Contrapositive

The contrapositive of φ → ψ is ¬ψ → ¬φ. These two statements are logically equivalent: one is a tautology iff the other is. A proof by contrapositive proves the contrapositive instead of the original.

**Logical principle:** To prove φ → ψ, it suffices to prove ¬ψ → ¬φ. Assume ¬ψ and derive ¬φ.

**When to use it.** The contrapositive is useful when the negation of the conclusion is more workable than the hypothesis. Often, "n is not even" (n is odd) gives us more traction than "if n is even" (n = 2k), because we can do case analysis on what "not even" means concretely.

**Example.** Prove: if n² is even, then n is even.

*Proof.* We prove the contrapositive: if n is odd, then n² is odd. Assume n is odd. Then n = 2k + 1 for some integer k. Then n² = (2k+1)² = 4k² + 4k + 1 = 2(2k² + 2k) + 1. Since 2k² + 2k is an integer, n² is odd. □

The original statement — "if n² is even then n is even" — would be harder to attack directly, because being even is not a hypothesis we can easily manipulate. The contrapositive flips the quantitative constraint to a workable form.

**Combined application.** The two examples together give us: n is even ↔ n² is even. This is a classic fact about parity. It is also used in one of the most famous proofs in mathematics: √2 is irrational.

**Proof that √2 is irrational.** Assume for contradiction that √2 = p/q in lowest terms (gcd(p,q) = 1). Then 2 = p²/q², so p² = 2q², so p² is even. By the fact above, p is even. Write p = 2m. Then (2m)² = 2q², so 4m² = 2q², so 2m² = q², so q² is even, so q is even. But then gcd(p, q) ≥ 2, contradicting the assumption that p/q is in lowest terms. □

Wait — that proof slid into a new technique: contradiction. Let us treat it properly.

## Proof by Contradiction

**Logical principle:** To prove φ, assume ¬φ and derive a contradiction (⊥, the false proposition). Since ⊥ implies everything, ¬φ must be false, so φ must be true.

Formally: to prove φ, it suffices to prove ¬φ → ⊥, which is the same as ¬¬φ, which in classical logic is equivalent to φ.

**Type-theoretic note.** Here we encounter a subtle point. In *intuitionistic* type theory — the logic underlying Lean and Agda — ¬¬φ does not imply φ in general. Double negation elimination is a specifically classical principle. Proof by contradiction in its full generality is not available in constructive mathematics. Instead, constructive proofs must exhibit a witness or construction explicitly. This matters deeply for HoTT, which has a constructive foundation.

For now, we work in classical logic. But keep the distinction in mind.

**Example.** Prove: there are infinitely many prime numbers.

*Proof.* Suppose for contradiction that there are only finitely many primes: p₁, p₂, ..., pₙ. Consider N = p₁ · p₂ · ... · pₙ + 1. We claim N has no prime divisor in our list. Indeed, if pᵢ divided N, then pᵢ would divide N - p₁·...·pₙ = 1, which is impossible since pᵢ ≥ 2. But every integer ≥ 2 has at least one prime divisor (by induction). So N has a prime divisor not in our list — contradicting the assumption that the list was complete. □

This proof, due to Euclid, uses contradiction to rule out the assumption that the list of primes is finite. The contradiction is obtained by constructing a number that cannot be on the list — a constructive move within an overall non-constructive argument.

**Example.** Prove: log₂ 3 is irrational.

*Proof.* Suppose log₂ 3 = p/q for positive integers p, q. Then 2^(p/q) = 3, so 2^p = 3^q. But 2^p is even and 3^q is odd (since 3^q = 3·3·...·3 inherits the oddness of 3). An even number cannot equal an odd number. Contradiction. □

## Proof by Cases

**Logical principle:** To prove φ ∨ ψ → χ, it suffices to prove φ → χ and ψ → χ separately. More generally, any exhaustive analysis of possibilities constitutes a proof by cases.

**When to use it.** Use case analysis when you have a disjunction in the hypothesis, when a definition splits into cases, or when you need to handle a variable that can take structurally different forms.

**Example.** Prove: for all integers n, n(n+1) is even.

*Proof.* Let n be an integer. We case-split on whether n is even or odd.

*Case 1: n is even.* Then n = 2k for some integer k. So n(n+1) = 2k(n+1) = 2[k(n+1)], which is divisible by 2.

*Case 2: n is odd.* Then n + 1 = 2m for some integer m (since the successor of an odd number is even). So n(n+1) = n · 2m = 2(nm), which is divisible by 2.

In both cases, n(n+1) is even. □

**Example.** Prove: |ab| = |a| · |b| for real numbers a, b.

*Proof.* We case-split on the signs of a and b: both non-negative, a negative and b non-negative, a non-negative and b negative, both negative. In each case, we verify the equation by expanding the absolute values according to their definition. (We omit the arithmetic, which is routine.) □

**Proof by exhaustion** is a degenerate form: list all cases explicitly. The four-color theorem — every planar map can be colored with at most four colors — was famously proved by Appel and Haken in 1976 by reducing to a finite (but enormous) case analysis checked by computer. This proof was controversial precisely because human verification was infeasible. Today, computer-checked proofs are the norm in formal mathematics.

## Combining the Techniques

Real proofs combine techniques. Here is a worked example using direct proof, cases, and a touch of contradiction.

**Theorem.** For any integers a, b, if a + b is odd, then exactly one of a, b is odd.

*Proof.* We must show two things: (1) at least one of a, b is odd, and (2) at most one is.

For (1): suppose both a and b are even. Then a = 2j, b = 2k, so a + b = 2j + 2k = 2(j + k), which is even — contradicting the hypothesis that a + b is odd. So at least one is odd.

For (2): suppose both a and b are odd. Then a = 2j + 1, b = 2k + 1, so a + b = 2j + 2k + 2 = 2(j + k + 1), which is even — again a contradiction. So at most one is odd.

Therefore exactly one of a, b is odd. □

## A Proof-Writing Standard

Clear mathematical proof has a recognizable structure. We close with a template.

1. **State what you are proving.** The reader should know the claim before seeing the argument.
2. **State your strategy.** "We proceed by contradiction," "We induct on n," "We case-split on whether n is even."
3. **Carry out the argument.** Each step should follow clearly from the previous.
4. **Signal the end.** The symbol □ (or QED) marks the end of the proof.
5. **Be complete without being verbose.** Omit routine computations only when they are genuinely routine. Never omit logical steps.

When you write proofs in a proof assistant, this structure becomes explicit in the tactic state: you see exactly where you are in the proof, what assumptions you have, and what remains to be shown. Practicing human-readable proof writing here makes the proof assistant experience more intelligible and the translation more natural.

The techniques in this section are not tricks. They are the grammar of mathematical argument. Internalize them, and you will find that proofs — even difficult ones — have a recognizable shape. That shape is what we are learning to see.
