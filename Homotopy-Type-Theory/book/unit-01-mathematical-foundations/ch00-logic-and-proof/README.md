# Chapter 0: Logic and Proof

## A Strange Fact

Here is a strange fact: every theorem you will ever prove in mathematics can be written as a program.

Not metaphorically. Literally. There is a precise correspondence — discovered independently by Haskell Curry in the 1930s and William Howard in the 1960s, now called the Curry-Howard correspondence — between logical proofs and typed programs. A proof of the proposition "P implies Q" is a function that takes a proof of P and returns a proof of Q. A proof of "P and Q" is a pair consisting of a proof of P and a proof of Q. A proof by induction on the natural numbers is a program that recurses on the natural number structure.

This correspondence is not an amusing coincidence. It is the foundational insight behind every modern proof assistant: Lean, Coq, Agda, Isabelle. When you write a proof in Lean, you are literally writing a program. The type-checker that verifies your proof is the same mechanism as a compiler checking type safety.

But here is what makes this chapter more than just preparation for later: the correspondence runs the other direction too. Programs can be read as proofs. The fact that a function compiles — that it type-checks — is itself a theorem, and the function is its proof. When we study the logic in this chapter, we are studying the type system of a programming language. When we study proof rules, we are studying typing rules.

This is why logic is not just the starting point of the curriculum. It is present throughout, in every proof we write, in every type we inhabit.

## The Historical Moment

Logic as a mathematical discipline was born twice.

The first birth was Aristotle's *Organon*, written around 350 BCE. Aristotle identified the *syllogism* — "All men are mortal; Socrates is a man; therefore Socrates is mortal" — as the fundamental unit of deductive inference. He classified valid syllogisms, identified fallacies, and articulated the principles of non-contradiction and excluded middle. For two thousand years, Aristotelian logic was logic.

The second birth was Gottlob Frege's *Begriffsschrift* of 1879. In 83 pages, Frege invented modern logic: quantifiers, variables, formal rules of inference, the distinction between syntax and semantics. He was trying to prove that arithmetic was a branch of pure logic — a program called *logicism* — and he needed a logical language expressive enough to carry all of mathematics. The language he invented did the job.

But Frege's logicism collapsed. In 1902, Bertrand Russell wrote Frege a letter: let R be the set of all sets that do not contain themselves. Does R contain itself? If it does, it doesn't. If it doesn't, it does. Frege's Basic Law V — the principle that permits forming the set {x | φ(x)} for any property φ — is inconsistent. The *Grundgesetze der Arithmetik*, whose second volume was at the printer, was fatally flawed.

Russell's paradox is not just an interesting puzzle. It reveals that unrestricted set comprehension is dangerous — that not every property defines a set — and that a formal system needs to be carefully designed to avoid such traps. Russell's own response was a type theory: a hierarchy of types where self-referential constructions are simply not well-formed. The idea that formal systems should enforce typing constraints to prevent paradox is older than Lean by a century.

## What We Build

This chapter develops two things in parallel: the *syntax* of logical reasoning and the *practice* of proof.

The syntax first. A well-formed formula is built from atomic propositions and logical connectives: negation (¬), conjunction (∧), disjunction (∨), implication (→), and biconditional (↔). The rules for building formulas are inductive — which means proofs about all formulas proceed by structural induction, a pattern that recurs constantly in type theory.

Semantics assigns meaning to syntax: a formula is true or false under an assignment of truth values to atomic propositions. The truth tables for each connective are definitions, not observations. Logical consequence — the relation Γ ⊨ φ meaning "every assignment that satisfies all of Γ satisfies φ" — captures the semantic notion of validity.

But truth tables are not proofs. The syntactic side of the story is natural deduction: a collection of inference rules that capture valid reasoning directly in terms of proof structure, with no reference to truth values. Natural deduction has introduction rules (how to prove each connective) and elimination rules (how to use each connective once you have it). The introduction-elimination duality is not accidental — it is the design principle that ensures the system is coherent and that makes the Curry-Howard correspondence precise.

Predicate logic extends propositional logic with quantifiers: ∀ (for all) and ∃ (there exists). These quantifiers are where mathematics actually lives. Nearly every mathematical theorem uses quantifiers: "for every ε > 0, there exists δ > 0 such that..." Predicate logic has its own rules of inference, its own notion of satisfaction, and its own analogue of the completeness theorem.

Mathematical induction runs through the chapter like a thread. We study simple induction, strong induction, structural induction, and well-founded induction. Each form is a theorem scheme — a pattern of reasoning that applies in infinitely many cases. And the most general form, well-founded induction, is exactly the eliminator for inductive types in dependent type theory. The connection is not analogy; it is identity.

## Connections Forward

When we study propositional connectives here, we are studying the type-forming operations of simple type theory. Conjunction is the product type A × B. Disjunction is the sum type A + B. Implication is the function type A → B. Negation is the function type A → ⊥, where ⊥ is the empty type.

When we study quantifiers, we are approaching dependent types. The universal quantifier ∀x∈A. P(x) becomes the dependent product Π(x:A).P(x). The existential quantifier ∃x∈A. P(x) becomes the dependent sum Σ(x:A).P(x). These are not analogies — they are the same construction at different levels of abstraction.

When we study natural deduction proof trees, we are studying terms in the lambda calculus. A proof of P → Q is a lambda-abstraction. A proof of P ∧ Q is a pair. Application of modus ponens is function application.

None of this is visible yet at the level of this chapter. The goal here is to build fluency with logical reasoning, not to encode it in a type theory. But knowing the destination changes how you see the road. Study the introduction rule for implication not just as "how to prove an if-then statement" but as "how to define a function." Study induction not just as "how to prove things about natural numbers" but as "the recursor for the natural number type."

The map between logic and programming is the deepest single idea in the curriculum. This chapter is where you first lay hands on it.
