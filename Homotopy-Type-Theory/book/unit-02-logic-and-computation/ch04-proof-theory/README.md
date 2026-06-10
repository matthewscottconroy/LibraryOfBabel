# Chapter 4: Proof Theory

## The Consistency Proof That Used More Than It Proved

In 1935, Gerhard Gentzen proved that Peano Arithmetic — the standard axiomatic system for natural number arithmetic — is consistent. No contradiction can be derived from its axioms. This was a remarkable achievement, and it was achieved by a 25-year-old mathematician in a single paper.

There is a catch. To prove that Peano Arithmetic is consistent, Gentzen used transfinite induction up to the ordinal $\varepsilon_0$. Peano Arithmetic itself can prove transfinite induction up to any specific finite ordinal, but it cannot prove transfinite induction up to $\varepsilon_0$ — that principle is stronger than anything Peano Arithmetic can establish on its own. Gentzen's consistency proof was technically valid, but it used a tool slightly beyond the reach of the system it was certifying.

This is not a failure. Gödel had already proved, in 1931, that no consistent system powerful enough to express arithmetic can prove its own consistency. Any consistency proof must use something the system cannot itself verify. Gentzen's proof is the cleanest possible consistency proof: it uses the minimal additional principle required. And what it reveals is not a defect in mathematics but a profound structural fact — that the consistency of a formal system is a property that can be established, but only from a vantage point slightly above the system itself.

This is not a joke. It is a discovery about the nature of proof.

Before we can understand type theory — before we can understand why propositions are types and proofs are programs — we must understand what a proof is. Not informally, not "a convincing argument," but precisely: what structure does a valid proof have, what can we say about that structure, and what does it mean to simplify one?

## What Proof Theory Is About

Proof theory is the mathematical study of proofs as formal objects. It asks the questions that working mathematicians normally ignore:

- What is the minimal structure a valid proof must have?
- Can every proof be simplified into a "canonical" form with no redundant steps?
- What can we infer about a proposition from the structure of its proofs?
- Is there a procedure that can always determine, in finite time, whether a proof is valid?

These are not questions about *which* theorems are true. They are questions about the *architecture* of reasoning itself.

The answers are striking. Every proof in natural deduction can be reduced to a normal form — a form with no "detours," no instances of introducing a logical connective only to immediately eliminate it. The process of removing detours is computation. The normal form of a proof is the result of running it as a program. And the existence of normal forms implies that the system is consistent: if there were a proof of a contradiction, normalizing it would yield a proof with no hypotheses and no introduction rules, which is impossible.

## Sections of This Chapter

**Section 1: Judgments and Derivations.** We begin at the beginning: what is a judgment? A judgment is a formal declaration — "proposition $A$ is true in context $\Gamma$." We study the structure of derivation trees, the distinction between the syntax of proof and its semantics, and the structural rules (weakening, contraction, exchange) that govern how hypotheses behave. This section introduces the notation that will carry us through the rest of the curriculum.

**Section 2: Natural Deduction.** Gentzen's natural deduction is the proof system that most closely mirrors ordinary mathematical reasoning. For each logical connective — conjunction, disjunction, implication, negation, the quantifiers — there are introduction rules (how to prove a proposition with that connective) and elimination rules (how to use such a proved proposition). We develop the full system with worked examples, and examine the balance between introduction and elimination that makes normalization possible.

**Section 3: Normalization.** A proof contains a "detour" when it introduces a connective only to immediately eliminate it — proving $A \wedge B$ from separate proofs of $A$ and $B$, then immediately extracting $A$, when the proof of $A$ was sitting there all along. The normalization theorem (Prawitz 1965) says every such detour can be removed, and every proof can be reduced to a normal form. We prove this and derive its two most important consequences: the subformula property (proofs stay within the vocabulary of their conclusions) and consistency.

**Section 4: Sequent Calculus.** Gentzen's second formalism is more symmetric than natural deduction. Instead of a single conclusion on the right, a sequent has a list of possible conclusions — $\Gamma \Rightarrow \Delta$ — giving the calculus a natural left-right duality. The central theorem is cut elimination (the *Hauptsatz*): any use of an intermediate lemma can be eliminated, producing a proof that works directly with the subformulas of its conclusion. We prove this and unpack its consequences for consistency, decidability, and the relationship between classical and intuitionistic logic.

## The Connection to HoTT

Why does proof theory matter for Homotopy Type Theory?

Because in HoTT, proofs are terms. The identity type $a =_A b$ is the type whose inhabitants are proofs that $a$ and $b$ are equal. A proof of type $p : a =_A b$ is a mathematical object with internal structure — it can be composed, inverted, transported along other equalities. Two proofs of the same equality can be themselves unequal, connected by a higher-dimensional proof, which is itself a term of a type.

This multi-layered structure — the "tower" of identity types — only makes sense if you understand that proofs are not mere certificates but objects with geometry. That understanding begins with proof theory: with Gentzen's insight that proofs have internal structure, that this structure can be simplified, and that the simplification is a form of computation. Once you have internalized that proofs compute, the HoTT picture of paths, homotopies, and higher paths becomes not mysterious but inevitable.

Let us begin.
