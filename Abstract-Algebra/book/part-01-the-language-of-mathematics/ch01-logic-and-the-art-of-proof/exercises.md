# Chapter 1 — Exercises

## Important Figures

- **Aristotle (384–322 BCE)** — codified syllogistic reasoning in the *Organon*; the first systematic formal logic
- **George Boole (1815–1864)** — *The Laws of Thought* (1854): algebraic treatment of propositional logic; Boolean algebra
- **Gottlob Frege (1848–1925)** — *Begriffsschrift* (1879): invented modern predicate logic and quantification; the first fully rigorous formal system
- **Bertrand Russell (1872–1970)** — with Whitehead, *Principia Mathematica* (1910–1913); discovered Russell's Paradox, forcing the axiomatization of set theory
- **David Hilbert (1862–1943)** — formalist program; articulated the goal of a complete, consistent axiomatization of mathematics; posed the decision problem (*Entscheidungsproblem*)
- **Kurt Gödel (1906–1978)** — Incompleteness Theorems (1931): no consistent, sufficiently strong formal system is complete; the definitive limit of Hilbert's program

## References and Primary Sources

- **Aristotle, *Prior Analytics* (c. 350 BCE)** — first systematic account of syllogistic inference
- **G. Boole, *The Laws of Thought* (1854)** — algebraic formulation of logic; the origin of Boolean algebra
- **G. Frege, *Begriffsschrift* (1879)** — first modern formal logic with quantifiers
- **B. Russell & A.N. Whitehead, *Principia Mathematica* (3 vols., 1910–1913)** — the logical foundations project
- **D. Velleman, *How to Prove It* (3rd ed., Cambridge, 2019)** — accessible introduction to proof techniques; excellent for building intuition
- **P. Suppes, *Introduction to Logic* (1957)** — classical treatment of propositional and predicate logic

## Examples, Applications, and Thought Experiments

- **The contrapositive** — "If $x^2$ is even, then $x$ is even": prove by contrapositive (if $x$ is odd, $x^2$ is odd); the proof is cleaner than direct proof and avoids contradiction; feel the difference between the three strategies
- **Russell's Paradox as proof by contradiction** — let $R = \{x : x \notin x\}$; then $R \in R \iff R \notin R$; a contradiction from apparently natural assumptions; motivates why a formal axiom system is needed
- **The Liar's Paradox** — "This statement is false": if true, it is false; if false, it is true; self-reference creates undecidability; Gödel's Incompleteness Theorem formalizes this loop into a theorem about formal systems
- **Truth tables as decision procedures** — tautologies (always true) can be verified by checking all $2^n$ rows; this is decidable but exponential; the deeper question of efficient decision (P vs. NP) is unsolved

## Exercises

1. Construct the truth table for the compound proposition $(P \Rightarrow Q) \wedge (\neg Q \Rightarrow \neg P)$. Identify which rows confirm that a conditional and its contrapositive always have the same truth value, and explain in one sentence why this logical equivalence justifies the proof strategy of proving the contrapositive in place of the original implication.

2. Write the formal negation of each of the following statements, moving $\neg$ inward as far as possible (i.e., until it applies only to atomic propositions). Then determine, for each, whether the original statement or its negation is true when the domain is the set of integers $\mathbb{Z}$.
   (a) $\forall x\, \exists y\, (x + y = 0)$
   (b) $\exists x\, \forall y\, (xy = y)$
   (c) $\forall x\, \forall y\, (x < y \Rightarrow \exists z\, (x < z < y))$

3. Prove the following by direct proof: if $n$ is an integer such that $n^2$ is divisible by 3, then $n$ is divisible by 3. Then use this result to prove, by contradiction, that $\sqrt{3}$ is irrational. Identify explicitly which step of each proof would fail if we tried to replace 3 by 4 throughout.

4. Suppose $P(n)$ is the statement "$n^3 + 2n$ is divisible by 3." Prove $P(n)$ for all $n \in \mathbb{N}$ using mathematical induction. After giving the inductive proof, also supply a direct proof using modular arithmetic (considering the cases $n \equiv 0$, $n \equiv 1$, and $n \equiv 2 \pmod{3}$) and compare the two approaches: which is more transparent, and why?

5. A theorem of the form $P \Rightarrow (Q \vee R)$ can sometimes be proved by cases: assume $P$ and $\neg Q$ together, then derive $R$ (so that if $Q$ fails, $R$ must hold). Use this strategy to prove: for any two integers $m$ and $n$, if $mn$ is odd, then both $m$ and $n$ are odd. Identify the propositions $P$, $Q$, and $R$ in your proof and confirm that the case structure is logically valid.

6. An axiomatic system is said to be *categorical* if any two models of its axioms are isomorphic — that is, structurally identical up to relabeling. Consider a toy axiomatic system with undefined terms "point" and "line" and the single axiom: "every two distinct points lie on exactly one line, and every two distinct lines meet in exactly one point." Show that this system is not categorical by exhibiting two non-isomorphic models. Then state, without proof, what additional axiom would force the finite model to be essentially unique.

7. The *principle of strong induction* states: if $P(k)$ holds for all $k < n$ implies $P(n)$, then $P(n)$ holds for all $n \in \mathbb{N}$. Prove that strong induction is logically equivalent to ordinary induction (i.e., each implies the other as a theorem, assuming the other as an axiom). Then use strong induction to prove that every integer $n \geq 2$ has a prime factorization.

8. (Challenge) Define a propositional formula to be in *conjunctive normal form* (CNF) if it is a conjunction of clauses, each clause being a disjunction of literals (atomic propositions or their negations). Prove that every propositional formula is logically equivalent to one in CNF. Your proof should be constructive: describe an explicit algorithm that transforms any formula into CNF, justify its correctness using the logical equivalences established in Section 1.1, and identify at what step the transformation may cause an exponential blowup in formula size.
