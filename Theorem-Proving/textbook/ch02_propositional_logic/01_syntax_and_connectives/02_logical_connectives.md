# Logical Connectives: The Algebra of Truth

> *"Every sentence of the language of mathematics is a combination of a few simple statements bound together by logical connectives."*

---

Imagine you are a Boolean circuit. You have two input wires, each carrying either a 0 (off) or a 1 (on), and one output wire. What can you do? You can produce a 1 when both inputs are 1 (AND). You can produce a 1 when at least one input is 1 (OR). You can flip a single input (NOT). With these three operations — AND, OR, NOT — you can implement any Boolean function whatsoever. Every computation your laptop performs, every pixel on your screen, every bit of memory — all of it reduces, at the hardware level, to these three tiny operations.

The connectives of propositional logic are exactly these operations, lifted from electrical engineering into the realm of pure thought. They are the vocabulary with which we build complex propositions from simple ones, and they are the foundation of every formal proof system. Understanding them deeply — not just their truth tables, but their *character*, their philosophical oddities, their role in reasoning — is essential.

## Negation (¬): Flipping Truth

Negation is the simplest connective: it takes one proposition and produces its opposite. ¬φ is true when φ is false, and false when φ is true.

The truth table has just two rows:

| φ | ¬φ |
|---|-----|
| T |  F  |
| F |  T  |

In natural language, negation corresponds to "not," "it is not the case that," "it is false that." But natural language negation is deceptively complex. Consider: "The present king of France is not bald." Is this true? If France has no king, there is no one to be bald or not bald. Bertrand Russell analyzed sentences like this as *false* (not meaningless) by treating "the present king of France" as a disguised existential claim that fails when the existence condition is not met. In formal logic, we sidestep this by working with propositions that *do* have a definite truth value.

A subtler point: negation in classical logic satisfies the **law of double negation**: ¬¬φ ≡ φ. What is doubly negated is the same as the original. This seems obviously true — if it's not the case that you're not tall, then you're tall. But in *intuitionistic logic* (the logic that Lean and Coq use by default), ¬¬φ → φ is not provable without additional axioms. We will return to this remarkable fact in Chapter 5; for now, file it away as a sign that negation is philosophically richer than it appears.

## Conjunction (∧): Logical "And"

φ ∧ ψ is true exactly when both φ and ψ are true. The truth table:

| φ | ψ | φ ∧ ψ |
|---|---|-------|
| T | T |   T   |
| T | F |   F   |
| F | T |   F   |
| F | F |   F   |

Conjunction is the most straightforward connective. In natural language it corresponds to "and," but also to "but," "although," "however," "nevertheless" — wherever two claims are being asserted simultaneously. The contrast words ("but," "although") add a pragmatic implication of contrast that conjunction alone does not capture, but the *logical* content is the same: both claims are being made.

One important feature: conjunction is **commutative** (φ ∧ ψ ≡ ψ ∧ φ) and **associative** ((φ ∧ ψ) ∧ χ ≡ φ ∧ (ψ ∧ χ)). These algebraic properties mean we can write φ ∧ ψ ∧ χ without ambiguity.

**In Lean 4**: A proof of `P ∧ Q` is a *pair* — you must provide a proof of P and a proof of Q. You construct it as `⟨proof_of_P, proof_of_Q⟩`. You extract the components with `.1` and `.2` (or `.left` and `.right`). This is the Curry-Howard correspondence making itself felt: conjunction *is* the product type.

## Disjunction (∨): Logical "Or" and Its Ambiguity

φ ∨ ψ is true exactly when at least one of φ, ψ is true:

| φ | ψ | φ ∨ ψ |
|---|---|-------|
| T | T |   T   |
| T | F |   T   |
| F | T |   T   |
| F | F |   F   |

This is **inclusive or** — true even when both disjuncts are true. This is the standard mathematical "or."

Natural language "or" is sometimes **exclusive**: "You can have cake or ice cream" often implies you can have one *but not both*. Menus, legal contracts, and certain everyday usages employ exclusive or. Mathematicians almost always mean inclusive or, and when they mean exclusive, they say so explicitly.

There is also a pragmatic phenomenon: saying "p or q" often implies that the speaker does not know *which* is true. If you know that Alice passed the exam, you would not normally say "Alice passed the exam or she failed." The fact that "p or q" is *technically* true whenever p is true does not mean it is *appropriate* to assert it when you know more than that. This gap between logical truth conditions and appropriate assertability is called **Gricean implicature**, after philosopher of language H.P. Grice.

For logic and mathematics, we care only about truth conditions, not about appropriateness. This is by design: a formal proof does not need to be rhetorically appropriate; it only needs to be valid.

**In Lean 4**: A proof of `P ∨ Q` is either `Or.inl proof_of_P` or `Or.inr proof_of_Q` — you must commit to which disjunct you are proving. This is the constructive aspect: to prove a disjunction constructively, you must *know* which side is true. Contrast this with classical logic, where you can use the law of excluded middle to assume P ∨ ¬P and reason by cases without knowing which case holds.

## Implication (→): The Connective That Confuses Everyone

The **material conditional** φ → ψ is, by far, the most philosophically contentious connective. Its truth table:

| φ | ψ | φ → ψ |
|---|---|-------|
| T | T |   T   |
| T | F |   F   |
| F | T |   T   |
| F | F |   T   |

The third and fourth rows always give students trouble. "If the moon is made of cheese, then 2+2=4" — this is *materially true*, because the antecedent (the moon is made of cheese) is false. "If Napoleon was born in England, then Napoleon was born in Europe" — also materially true, because the consequent is true regardless of the antecedent.

Why on earth do we define implication this way? Here are three ways to understand it:

**Way 1 — The commitment view**: φ → ψ is a *commitment*: I claim that if φ is ever true, ψ will be too. If φ is false, the commitment was never tested — and an untested commitment cannot be violated. So a false antecedent leaves the conditional vacuously satisfied. A conditional is only *violated* (made false) when the antecedent is true but the consequent is false — when the commitment is tested and fails.

**Way 2 — The disjunction equivalence**: φ → ψ is logically equivalent to ¬φ ∨ ψ. Either φ is not the case (so the condition for the consequent to be needed never arises), or ψ is the case (so the consequent is satisfied). Check this against the truth table: the conditional is false only when φ is true and ψ is false — precisely when ¬φ ∨ ψ is false.

**Way 3 — The minimal coherent definition**: We want implication to satisfy modus ponens: from φ → ψ and φ, we can infer ψ. And we want it to be the *strongest* such connective (the one that fails to hold in the fewest cases). The material conditional is the unique truth-functional connective with these properties.

**The philosophical problem**: Material implication captures something important but not everything we mean by "if...then." "If you drop this glass, it will break" is a causal claim about what *would* happen, not just a truth-functional statement about actual events. "If Julius Caesar had used Twitter, he would have been assassinated sooner" is a counterfactual — it talks about what would have happened in a different possible world. These richer notions of conditionality are studied in **conditional logic** and **counterfactual logic** (developed by Robert Stalnaker and David Lewis in the 1960s-70s), which go beyond material implication.

For mathematics and formal proof, material implication is exactly right: mathematical conditionals are not causal, and the "paradoxes" of material implication (false premise implies everything) are mathematical features, not bugs. In mathematics, if you have a false assumption, all bets are off — and that is exactly what material implication encodes.

## Biconditional (↔): If and Only If

φ ↔ ψ is true exactly when φ and ψ have the same truth value:

| φ | ψ | φ ↔ ψ |
|---|---|-------|
| T | T |   T   |
| T | F |   F   |
| F | T |   F   |
| F | F |   T   |

The biconditional means "if and only if" (abbreviated "iff" in mathematical writing). It asserts that φ implies ψ *and* ψ implies φ — the implication runs both ways.

In mathematics, biconditionals are the standard form for definitions and equivalences. "An integer n is even if and only if n = 2k for some integer k." "A function f is continuous if and only if the preimage of every open set is open." "Γ ⊨ φ if and only if Γ ⊢ φ" (completeness theorem). The iff ensures that neither direction of the equivalence is overlooked.

**Interesting structural fact**: φ ↔ ψ is equivalent to (φ → ψ) ∧ (ψ → φ), and also to (φ ∧ ψ) ∨ (¬φ ∧ ¬ψ). The second equivalence says: they are both true, or they are both false — a nice characterization of "having the same truth value."

## The Missing Connectives: Completeness

Five connectives: ¬, ∧, ∨, →, ↔. Are these all we need? More than we need?

It turns out that {¬, ∧} suffices: φ ∨ ψ ≡ ¬(¬φ ∧ ¬ψ), φ → ψ ≡ ¬(φ ∧ ¬ψ), φ ↔ ψ ≡ ¬(φ ∧ ¬ψ) ∧ ¬(¬φ ∧ ψ). Similarly, {¬, ∨} suffices (by De Morgan's laws). More remarkably, a single two-input connective called **NAND** (not-and, defined as ¬(φ ∧ ψ)) is *functionally complete* on its own — every Boolean function can be expressed using only NAND. This has hardware implications: NAND gates are the universal building block of digital circuits.

> **A Design Exercise**: Why did logicians choose this particular set of connectives? They are not the *minimal* set (NAND alone would do), but they correspond to the most *natural* and *interpretable* combinations. The goal was not minimality but communicability. Similar tradeoffs appear throughout the design of programming languages.

## Truth-Functionality: What Makes Propositional Logic Tractable

All five connectives are **truth-functional**: the truth value of a compound formula is determined entirely by the truth values of its components. This is not true of all logical operators. "Alice believes that p" is not truth-functional: the truth value of the belief claim depends not just on whether p is true but on what Alice's psychological states are.

Truth-functionality is what makes propositional logic **decidable**: given any formula, we can determine its truth value under any valuation by simple calculation, and we can check validity by examining all 2ⁿ valuations (where n is the number of distinct propositional variables). Truth tables are a complete decision procedure for propositional logic.

This is remarkable. We have a formal system — propositional logic — for which there is an algorithm that answers every logical question. This will not be true of first-order logic (Chapter 3) or of arithmetic (Chapter 10). Propositional logic's decidability is a consequence of its expressive *limitations* as well as its expressive power.

---

## Real-World Applications

**Digital circuits**: AND, OR, NOT gates implement conjunction, disjunction, negation on binary signals. Every processor operation — addition, comparison, memory access — is built from these gates. Boolean algebra (the algebraic study of these operations) is the mathematics of digital design.

**SQL databases**: The WHERE clause in SQL uses AND, OR, NOT to filter rows. `SELECT * FROM users WHERE age > 18 AND country = 'US'` is a conjunction of two predicates. SQL's three-valued logic (true, false, NULL) is a non-classical extension that handles missing data — a practical application of the philosophical issues raised by "undefined" truth values.

**Program correctness**: In Hoare logic (Chapter 13), preconditions and postconditions are propositional (or first-order) formulas, and the proof that a program satisfies its specification involves logical reasoning with these connectives.

---

*Next: Well-formed formulas — the recursive structure that gives propositional logic its expressiveness.*
