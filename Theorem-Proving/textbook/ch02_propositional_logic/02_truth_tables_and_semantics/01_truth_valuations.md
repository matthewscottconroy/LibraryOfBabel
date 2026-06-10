# Truth Valuations: The Semantics Machine

> *"The logical constants are not representatives; there can be no representatives of the logic of facts."*
> — Wittgenstein, *Tractatus*, 4.0312

---

Here is a powerful idea that sits at the heart of propositional logic: *the meaning of a complex formula is a function of the meanings of its parts.* There is no mystery, no emergent meaning, no context-dependence — just a mechanical computation. Give me the truth values of the atoms, and I will give you the truth value of any formula built from those atoms, by applying the truth clauses for each connective in turn.

This **compositionality** principle sounds almost trivially obvious once stated. But it is actually a very strong constraint, and the fact that it holds for propositional logic is what makes propositional logic *tractable* — and what distinguishes it from the full complexity of natural language.

## Valuations: Assigning Meaning to Atoms

Let P = {p₁, p₂, p₃, ...} be a countably infinite set of propositional variables. A **valuation** (or **assignment**, or **interpretation**) is a function:

$$v : P \rightarrow \{0, 1\}$$

that assigns a truth value (0 for false, 1 for true) to each propositional variable. Once we have v, we can compute the truth value of any formula by applying the recursive truth clauses.

A valuation is completely arbitrary: any assignment of 0s and 1s to variables counts as a legal valuation. There are no constraints, no "meaning" to the variables beyond what v assigns. The variable p₁ might stand for "it is raining" under one valuation and for "the president is left-handed" under another. The logical machinery does not care.

This indifference to content is not a limitation — it is a feature. It means that every logical law we prove holds *regardless* of what the propositional variables happen to stand for. When we show that p → (q → p) is a tautology, we have shown it for all possible substitutions of p and q, in all possible contexts, forever.

## The Recursive Truth Clauses

Given a valuation v, we extend it to all formulas by the following recursive definition. We write ⟦φ⟧_v for the truth value of formula φ under valuation v:

$$\llbracket p_i \rrbracket_v = v(p_i)$$

$$\llbracket \top \rrbracket_v = 1 \quad \llbracket \bot \rrbracket_v = 0$$

$$\llbracket \neg\phi \rrbracket_v = 1 - \llbracket\phi\rrbracket_v$$

$$\llbracket \phi \wedge \psi \rrbracket_v = \min(\llbracket\phi\rrbracket_v, \llbracket\psi\rrbracket_v)$$

$$\llbracket \phi \vee \psi \rrbracket_v = \max(\llbracket\phi\rrbracket_v, \llbracket\psi\rrbracket_v)$$

$$\llbracket \phi \rightarrow \psi \rrbracket_v = \max(1 - \llbracket\phi\rrbracket_v, \llbracket\psi\rrbracket_v)$$

$$\llbracket \phi \leftrightarrow \psi \rrbracket_v = 1 \text{ iff } \llbracket\phi\rrbracket_v = \llbracket\psi\rrbracket_v$$

These clauses are the complete semantics of propositional logic. Nothing else is needed. The truth value of any formula, no matter how complex, can be computed from the truth values of its atoms by following these rules recursively, like unwinding a recursive function call.

> **Computation as Proof**: Notice that computing ⟦φ⟧_v is a recursive computation over the structure of φ. This is structural induction in action (Chapter 7): the correctness of the computation is proved by showing that the base cases (atoms) are handled correctly, and that each recursive case (connective application) is handled correctly assuming the subformulas are correctly evaluated. The proof that this evaluation function is well-defined is essentially the Unique Readability Theorem mentioned in the previous chapter.

## The Size of the Semantic Space

If a formula has n distinct propositional variables, there are 2ⁿ possible valuations — one for each function from n variables to {0, 1}. A truth table is a complete enumeration of all 2ⁿ valuations together with the formula's truth value under each.

For n = 1: 2 valuations.
For n = 2: 4 valuations.
For n = 10: 1,024 valuations.
For n = 20: 1,048,576 valuations.
For n = 100: more atoms than there are in the observable universe.

This exponential blowup is why truth tables are not a practical method for large formulas. It is also why the **SAT problem** — "is this formula satisfiable?" — is NP-complete: no algorithm is known that solves it in polynomial time in the number of variables in the worst case, and most computer scientists believe no such algorithm exists.

Modern **SAT solvers** (see Chapter 13) use sophisticated heuristics and learned clause pruning to solve SAT instances with millions of variables in practice — but this is a triumph of engineering over theory, not a solution to the underlying complexity.

## Tautologies as Universal Truths

A formula φ is a **tautology** if ⟦φ⟧_v = 1 for every valuation v. Tautologies are the logical laws of propositional logic — statements that are true regardless of the interpretation of the variables.

Classic tautologies:

- **Law of Excluded Middle (LEM)**: p ∨ ¬p
- **Non-Contradiction**: ¬(p ∧ ¬p)
- **Double Negation**: ¬¬p ↔ p
- **Modus Ponens schema**: (p ∧ (p → q)) → q
- **Hypothetical Syllogism**: (p → q) ∧ (q → r) → (p → r)
- **De Morgan (conjunction)**: ¬(p ∧ q) ↔ (¬p ∨ ¬q)

Each of these is a *logically valid* pattern: it holds no matter what p, q, and r stand for. When we use modus ponens in a proof, we are applying the tautology `(p ∧ (p → q)) → q` — substituting the actual propositions for the variables and instantiating the universal.

The law of excluded middle deserves special attention. It says that for every proposition, either it or its negation is true. This seems obviously correct — how could a proposition be neither true nor false? — but intuitionists deny it as a *logical* law (see Chapter 5). For them, a proposition is true only if you have a proof of it, and there are propositions for which neither a proof nor a disproof exists.

## From Semantics to Proof

We have now set up the semantic machinery for propositional logic. A formula is true in a valuation if ⟦φ⟧_v = 1. A formula is a tautology if it is true in all valuations. A set of formulas Γ semantically entails φ (Γ ⊨ φ) if every valuation making all members of Γ true also makes φ true.

The big question for the next several chapters: how do we *prove* tautologies and entailments without checking all 2ⁿ valuations? The answer is a **proof system** — a set of axioms and inference rules from which tautologies can be derived syntactically. The profound connection between the semantic notion (validity) and the syntactic notion (provability) is the subject of the **soundness** and **completeness** theorems (Chapter 9).

---

*Next: How to construct truth tables systematically, and what they tell us.*
