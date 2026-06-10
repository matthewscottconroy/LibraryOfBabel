# What Is a Mathematical Proof?

> *"A mathematician, like a painter or poet, is a maker of patterns. If his patterns are more permanent than theirs, it is because they are made with ideas."*
> — G.H. Hardy, *A Mathematician's Apology*

---

In 1993, Andrew Wiles announced a proof of Fermat's Last Theorem — the claim that no three positive integers a, b, c satisfy aⁿ + bⁿ = cⁿ for any integer n > 2. This had been an open problem for 358 years, since Pierre de Fermat wrote it in the margin of his copy of Diophantus' *Arithmetica* in 1637. Wiles's proof was over 100 pages long, drew on mathematics (elliptic curves, modular forms, Galois representations) that Fermat could not have dreamed of, and — importantly — contained an error discovered by the referee. Wiles spent another year repairing the gap before the proof was accepted.

What is remarkable about this story is not that there was an error in a 100-page proof. What is remarkable is that the mathematical community could *identify* there was an error and subsequently *verify* the repair was complete. How? What is the standard that makes something a proof, and a gap in a proof a gap?

This question — what is a mathematical proof? — is not as obvious as it might seem, and the answer has changed significantly over the history of mathematics. Understanding the answer, and its evolution, is essential context for everything we will do in this textbook.

## The Informal Picture: Proof as Convincing Argument

The pre-twentieth-century view, still dominant in practice today, is roughly this: a proof is a sequence of statements, each of which either is a premise (assumption or previously established theorem) or follows from previous statements by a step that any competent mathematician would recognize as valid. The proof is complete when the target theorem appears as the last statement.

This is admirably practical but dangerously vague. "Any competent mathematician would recognize as valid" is a social, not a logical, criterion. It means that the standard of proof is whatever the current mathematical community accepts — and that standard changes. Euler's eighteenth-century proofs, freely manipulating infinite sums and products in ways that are now known to be invalid without further justification, were accepted in their day. Cauchy's "proof" of the continuity of limit functions, mentioned in the previous chapter, was accepted for years. The history of mathematics is a history of *tightening standards*.

## The Crisis of Foundations

The crisis came in the 1880s and 1890s. Cantor had introduced set theory and, with it, a dizzying landscape of infinite sets with different sizes. Frege was building arithmetic on logic. Peano was axiomatizing the natural numbers. And then, in 1901, Bertrand Russell wrote a letter to Frege that began:

> "I have discovered a contradiction in your derivation..."

Russell's paradox — the set of all sets that do not contain themselves — showed that Frege's logical foundations were inconsistent. Any inconsistent system can prove any statement, true or false. The entire edifice of "rigorous" mathematics built on these foundations was worthless.

This was not just a technical setback. It was a philosophical earthquake. Mathematicians were forced to ask: on what basis do we trust proofs at all? What are the fundamental logical rules we are using? Are those rules themselves consistent?

The response to this crisis gave us modern mathematical logic: Zermelo-Fraenkel set theory, Peano arithmetic, Hilbert's proof theory, Brouwer's intuitionism, and, ultimately, the formal proof systems that Lean and Coq implement.

## The Formal Picture: Proof as Symbol Manipulation

The formal solution — developed by Hilbert, Gentzen, and others in the early twentieth century — defines a proof as a finite sequence of formulas, each of which is either an axiom or follows from previous formulas by an explicit, finite, mechanically checkable inference rule.

On this definition:
1. Every step can be checked by a machine with no understanding of mathematics
2. Correctness is a syntactic property — it depends only on the shape of the proof
3. There is no gap between "a competent mathematician would accept this" and "this is correct"

A **formal proof** of φ from axioms Γ is a finite sequence φ₁, φ₂, ..., φₙ where φₙ = φ, and each φᵢ is either a member of Γ or follows from previous formulas by a specified inference rule.

Here is a tiny formal proof of q from the hypotheses {p, p → q}:

```
1. p         (hypothesis)
2. p → q     (hypothesis)
3. q         (→-elimination applied to 1 and 2)
```

Each step is completely explicit. A machine can verify this proof without knowing what p and q mean. The chain of justifications is unbroken.

## The Machine-Checked Picture: Proof as Checkable Object

Modern proof assistants go one step further. Not only is each inference rule made explicit — the proof is a data structure that the computer stores, checks, and manipulates.

In Lean 4, a proof of a theorem `T` is a *term* of type `T` in the type theory. The type-checker verifies that the term has the claimed type, checking every subterm against the type system's rules. This is not a stylistic choice — it is mathematically equivalent to formal proof (by the Curry-Howard correspondence, which we will explore in Chapter 11).

The practical consequence: when Lean 4 accepts a proof, you have not just convinced yourself or a peer reviewer. You have produced a certificate that can be mechanically verified by any computer running Lean's kernel. The kernel is small enough (a few thousand lines of code) that its correctness can itself be formally verified.

This creates a remarkable epistemological situation: modern proof assistants give us proofs whose correctness is *not* a matter of social consensus. The four-color theorem, proved in 1976 by Appel and Haken using computer enumeration, was controversial for years because many mathematicians did not trust computer-generated proofs. When Georges Gonthier formalized it in Coq in 2005, that controversy was resolved: the proof was now a Coq term, and anyone who trusts Coq's kernel (which they can read and verify) must accept the proof.

## The Spectrum from Intuition to Formality

There is a spectrum of proof rigor:

```
Intuitive sketch → Informal proof → Rigorous informal proof →
  Formal proof → Machine-checked proof
```

Each step adds more explicitness, more mechanical checkability, and less reliance on human trust and community consensus. Each step also typically makes the proof *longer*: a one-paragraph informal proof might become ten pages of formal derivation, and ten pages of formal derivation might become a hundred lines of Lean code.

This textbook will train you to move fluidly across this spectrum. Understanding informal proofs requires mathematical intuition and conceptual grasp. Understanding formal proofs requires logical precision. Understanding machine-checked proofs requires familiarity with proof assistants. All three are valuable; all three are interrelated.

> **A Comparative Exercise**: Euclid's proof that there are infinitely many primes is about half a page in modern notation. The same proof in Lean 4 (using Mathlib) is a handful of lines of elegant tactic code. The same proof in fully elaborated formal natural deduction would be pages. The same proof in Lean's kernel language (without Mathlib) would be enormous. Same mathematical content, vastly different lengths. Why? What does each representation make visible or hide?

## Proof and Explanation

One last dimension: the difference between proofs that *establish* and proofs that *explain*.

Some proofs show that something is true by exhaustive verification, by indirect argument, or by reduction to known facts — without illuminating *why* it is true. The four-color theorem's original proof is like this: it reduces the problem to checking 1,936 configurations by computer. The theorem is proved, but many mathematicians feel the proof does not explain *why* four colors suffice.

Other proofs reveal underlying structure. When Euler found that the sum of the reciprocals of the perfect squares equals π²/6, he did not just show a numerical fact — he revealed a deep connection between the distribution of zeros of a complex function (the Riemann zeta function) and the structure of the integers. The proof explains as well as establishes.

This distinction matters philosophically and pedagogically. In this textbook, we aim for both: rigorous formal proofs that can be machine-checked, and intuitive explanations that help you understand *why* the theorems are true. These two goals reinforce each other: a good intuitive explanation often suggests the structure of a formal proof, and the discipline of formalization often reveals what the intuition was glossing over.

---

*Next: A tour of the formal tools we will use throughout this textbook.*
