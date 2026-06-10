# Exercises

---

## Propositional Logic

**Exercise 0.1.** Using truth tables, verify each of the following is a tautology:
- $P \to (Q \to P)$
- $(P \to (Q \to R)) \to ((P \to Q) \to (P \to R))$
- $(\neg P \to \neg Q) \to (Q \to P)$

These are the three axiom schemes of classical propositional calculus (Łukasiewicz basis). The third is not an intuitionistic tautology — verify that there is a Kripke model (see Chapter 5) that falsifies it.

---

**Exercise 0.2.** For each formula, determine: tautology, satisfiable but not a tautology, or contradiction.

(a) $(P \to Q) \to (\neg Q \to \neg P)$

(b) $(P \to Q) \to (Q \to P)$

(c) $((P \to Q) \to P) \to P$ (Peirce's law)

(d) $(P \wedge Q) \to (P \vee R)$

(e) $\neg(P \leftrightarrow Q) \leftrightarrow (P \wedge \neg Q) \vee (\neg P \wedge Q)$

---

**Exercise 0.3.** Convert the following to conjunctive normal form (CNF):
- $(P \to Q) \wedge (Q \to R)$
- $\neg(P \vee Q) \to R$

---

**Exercise 0.4.** Show that $\{P \to Q, P \to \neg Q\} \models \neg P$. Do this by (a) truth table and (b) a natural deduction derivation.

---

## Proof Techniques

**Exercise 0.5.** Prove: if $a$ and $b$ are integers and $ab$ is odd, then $a$ and $b$ are both odd.

Give both a direct proof and a contrapositive proof.

---

**Exercise 0.6.** Prove: $\sqrt{3}$ is irrational. Model your proof on the irrationality of $\sqrt{2}$.

---

**Exercise 0.7.** Each of the following "proofs" contains a flaw. Identify the flaw precisely.

(a) *"Proof that all positive integers are equal."* By strong induction. Assume all integers less than $n$ are equal. Then... [describe the flaw]

(b) *"Proof that $1 = 2$."* Let $a = b$. Then $a^2 = ab$, so $a^2 - b^2 = ab - b^2$, so $(a+b)(a-b) = b(a-b)$, so $a + b = b$, so $2a = a$, so $2 = 1$.

---

**Exercise 0.8.** Write a fully rigorous proof that the square root of 6 is irrational. Your proof should cite every theorem it uses.

---

**Exercise 0.9.** Prove: for all integers $n \geq 1$, $n! \geq 2^{n-1}$.

*Hint:* Choose the right form of induction.

---

## Mathematical Induction

**Exercise 0.10.** Prove by induction: $\displaystyle\sum_{k=1}^{n} k^2 = \frac{n(n+1)(2n+1)}{6}$.

---

**Exercise 0.11.** Prove by strong induction: every positive integer can be written in binary (as a sum of distinct powers of 2).

---

**Exercise 0.12.** Define the *Tribonacci sequence*: $T_0 = 0, T_1 = 0, T_2 = 1$, and $T_n = T_{n-1} + T_{n-2} + T_{n-3}$ for $n \geq 3$.

(a) Prove by strong induction that $T_n < 2^n$ for all $n \geq 0$.

(b) What base cases do you need for your induction?

---

**Exercise 0.13 (Structural induction).** Define binary trees as in Section 3.2. The *height* of a tree: $h(\text{leaf}) = 0$, $h(\text{node}(T_1, T_2)) = 1 + \max(h(T_1), h(T_2))$.

Prove: a binary tree of height $h$ has at most $2^h$ leaves.

---

**Exercise 0.14.** The *compactness theorem* for propositional logic: if every finite subset of an infinite set $\Gamma$ of formulas is satisfiable, then $\Gamma$ is satisfiable.

Prove this using the following approach: enumerate the atoms $P_1, P_2, P_3, \ldots$; build a valuation by deciding $v(P_n)$ one at a time, always preserving satisfiability of every finite subset. Use the fact that if every finite subset of $\Gamma \cup \{P_n = \mathbf{T}\}$ is satisfiable or every finite subset of $\Gamma \cup \{P_n = \mathbf{F}\}$ is satisfiable (at least one holds), set $v(P_n)$ accordingly.

---

## Predicate Logic

**Exercise 0.15.** Determine the free and bound variables in each formula:

(a) $\forall x, P(x, y)$

(b) $\exists x, (x > y \wedge \forall y, y < x)$

(c) $P(x) \to \forall x, Q(x)$

---

**Exercise 0.16.** In the language of arithmetic, write first-order formulas expressing:

(a) $n$ is prime (hint: "$n > 1$ and the only divisors of $n$ are 1 and $n$")

(b) $n$ is a perfect square

(c) There are infinitely many primes (this requires a sentence saying "for every prime, there is a larger prime")

---

**Exercise 0.17.** Prove: if $\forall x, (P(x) \to Q(x))$ and $\exists x, P(x)$, then $\exists x, Q(x)$.

Write the proof in natural deduction style, citing each rule used.

---

**Exercise 0.18 (Reflection).** Proof by contradiction uses the law of excluded middle. Find a proof in this chapter that uses contradiction and rewrite it as a direct proof. Is this always possible? Identify one proof that you cannot easily de-classicalize.

---

**Exercise 0.19 (Challenge — Gödel preview).** Gödel's First Incompleteness Theorem states: any consistent, effectively axiomatized theory that can express basic arithmetic contains sentences that are true (in the standard model $\mathbb{N}$) but unprovable in the theory.

(a) What does "effectively axiomatized" mean? Why is this condition needed?

(b) Sketch the key idea: a sentence that (informally) says "this sentence is not provable." Why can this sentence not be proved or disproved without contradiction?

(c) What does this say about the limits of formal proof systems for mathematics?
