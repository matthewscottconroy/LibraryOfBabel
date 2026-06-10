# Syntax and Semantics: Structure and Meaning

> *"The sign is arbitrary, the meaning is not."*
> — Ferdinand de Saussure

---

Here is a peculiar fact. The string of symbols

$$\forall x \, (D(x) \rightarrow M(x))$$

is syntactically identical whether D means "is a dog" and M means "is a mammal," or whether D means "is a Democrat" and M means "is mortal," or whether D and M have no specified meaning at all. The *syntax* — the shape of the expression, its structural properties — is the same in all three cases. The *semantics* — what the expression *means*, what it is *about*, whether it is *true* — depends entirely on how we choose to interpret D and M.

This separation between syntax and semantics is one of the deepest and most fruitful ideas in all of logic. Understanding it clearly is the first step toward understanding why formal logic works, what proof assistants are actually doing, and what the relationship is between mathematical truth and mathematical proof.

## The Distinction, Precisely

**Syntax** is the study of *form*: which expressions are well-formed, how expressions are structured, which expressions are built from which others. Syntactic properties are determined by the symbol sequence alone, without reference to meaning.

**Semantics** is the study of *meaning*: what expressions refer to, under which conditions they are true or false, how they relate to the world (or to abstract mathematical structures). Semantic properties require specifying an interpretation.

The same formal expression can be given wildly different semantic interpretations. The formula `∀x(D(x) → M(x))` is true in one interpretation and false in another. Whether it is provable from a given set of axioms is a *syntactic* question — one we can answer by manipulating symbols mechanically. Whether it is *true* is a *semantic* question — one we can only answer by specifying what D and M mean.

> **A Revealing Experiment**: Take the formula `P → (Q → P)`. Is this formula:
> (a) Syntactically well-formed? (b) True? (c) A tautology?
>
> For (a), you need only check that it conforms to the grammar — no interpretation needed.
> For (b), you need to know what P and Q mean in some context.
> For (c), you need to check *all* possible interpretations — is it true no matter what P and Q mean?
>
> Notice that (a) and (c) can be answered purely mechanically, without knowing anything about the "content" of P and Q. That is the power of separating syntax from semantics.

## Interpretations and Valuations

In propositional logic, an **interpretation** (also called a **valuation**) is an assignment of truth values to the propositional variables. A valuation v assigns either 1 (true) or 0 (false) to each variable: v(p) = 1, v(q) = 0, etc.

The semantics is then defined by a set of recursive **truth clauses** that compute the truth value of any formula from the truth values of its subformulas:

| Formula | True iff... |
|---------|-------------|
| p | v(p) = 1 |
| ¬φ | φ is false under v |
| φ ∧ ψ | both φ and ψ are true under v |
| φ ∨ ψ | at least one of φ, ψ is true under v |
| φ → ψ | φ is false under v, or ψ is true under v |
| φ ↔ ψ | φ and ψ have the same truth value under v |

These clauses are the entire semantics of propositional logic. They are purely compositional: the meaning of a complex formula is *determined entirely* by the meanings of its parts. This **compositionality** principle is what makes formal semantics tractable — and also, incidentally, what makes formal languages so different from natural language, where the meaning of a whole is often more (or less) than the sum of its parts.

## Satisfaction, Validity, and Entailment

With semantics in hand, we can define the three most important semantic concepts in logic.

A formula φ is **satisfiable** if there exists at least one valuation under which it is true. It is **valid** (or a **tautology**) if it is true under *every* valuation. It is **unsatisfiable** (or a **contradiction**) if it is false under every valuation.

Think of valuations as possible worlds. A satisfiable formula is one that is true in at least one possible world — it describes something coherent. A valid formula is one that is true in *all* possible worlds — it is a logical law, something that holds regardless of the way things are. An unsatisfiable formula is one that is true in no possible world — it describes something incoherent.

The concept of **logical entailment** — written Γ ⊨ φ — generalizes this: a set of formulas Γ entails a formula φ iff every valuation that makes all members of Γ true also makes φ true. In other words, φ must be true whenever the "assumptions" in Γ are true. Entailment is the semantic counterpart of proof: Γ ⊢ φ (there is a formal proof of φ from Γ) should correspond to Γ ⊨ φ.

This correspondence — the deep connection between provability (syntax) and validity (semantics) — is called **completeness**, and proving it rigorously is one of the central theorems of mathematical logic. We will return to it in Chapter 9.

## The Liar Paradox and Tarski's Hierarchy

Here is a sentence that causes serious trouble:

> "This sentence is false."

Call it L. If L is true, then what L says is the case — but L says it is false, so L must be false. Contradiction. If L is false, then what L says is not the case — but L says it is false, so it is not false, so it is true. Contradiction again. L seems to be true if and only if it is false.

This is the **Liar Paradox**, known since antiquity (the Cretan Epimenides says "All Cretans are liars" — is he lying?). For centuries it was treated as a curiosity. Alfred Tarski showed in 1933 that it is a symptom of a genuine structural problem: if a sufficiently rich language can refer to its own sentences and their truth values, then it will produce Liar-like contradictions.

Tarski's solution is elegant and important: **stratify** language into levels. The **object language** L₀ contains ordinary sentences but cannot talk about its own truth. The **metalanguage** L₁ contains sentences of L₀ plus a truth predicate `True₀` that applies to sentences of L₀. A further metalanguage L₂ contains a truth predicate for L₁, and so on. A sentence at level n can only attribute truth to sentences at lower levels, preventing self-reference.

Tarski's **T-schema** specifies the meaning of truth at each level:

$$\text{True}_0(\ulcorner \phi \urcorner) \iff \phi$$

Here ⌜φ⌝ is the *name* (formal encoding) of the sentence φ, and the biconditional expresses that the name of φ is true iff φ itself is true. For example:

$$\text{True}_0(\ulcorner \text{"Snow is white"} \urcorner) \iff \text{Snow is white.}$$

This is Tarski's famous example. It looks trivially obvious — and it is! The T-schema says: truth is not some mysterious property floating free of the world; to say a sentence is true *just is* to assert that sentence. This is the core of the **correspondence theory of truth** dressed in formal clothes.

The Liar paradox is blocked because `L` is at the same level as `True₀` — it tries to apply the truth predicate to a sentence at its own level, which Tarski's hierarchy forbids.

## Syntax Without Semantics: Is It Useful?

You might wonder: if logic is ultimately about *truth*, why spend so much time on syntax alone? Can syntax be useful without semantics?

The surprising answer is *yes*, and this insight is the foundation of the Hilbert program and of modern proof theory. When you manipulate a Lean 4 proof — applying tactics, restructuring goals, applying lemmas — you are, from one perspective, doing purely syntactic manipulation: transforming one string of symbols into another according to fixed rules, without ever asking whether those symbols are "true." The type-checker's job is purely syntactic: does the resulting term typecheck? It never asks "but is it *really* correct?"

The miracle — and it is a miracle, established by the soundness theorem — is that syntactic correctness implies semantic truth. If your Lean proof compiles, the theorem is true. This is not obvious; it requires proof. But once established, it means that the laborious semantic verification (checking truth against all possible interpretations) is replaced by syntactic verification (does the proof term typecheck?), which a machine can do instantly.

This is the deepest reason why formal languages and proof assistants are valuable. They convert the hard, semantic question "Is this true?" into the easier, syntactic question "Does this typecheck?" — and the soundness theorem guarantees the two questions have the same answer.

---

## Tool Connections

**Tarski's World / Carnap**: In Tarski's World, you see the syntax/semantics distinction made concrete. The sentence `∀x(Cube(x) → Large(x))` is a syntactic object — a string of symbols. The blocks world in front of you is the semantic interpretation. When you press "Evaluate," the software checks whether the syntactic object is true in the semantic world. This is semantics in action.

**Z3 (Python)**: A satisfiability modulo theories (SMT) solver like Z3 works at the semantic level. It searches for a model (an interpretation) that satisfies a given formula. When Z3 returns SAT with a model, it is exhibiting a *witness* — a specific semantic interpretation that makes the formula true. When it returns UNSAT, it has verified that no such interpretation exists — the formula is unsatisfiable.

**Lean 4**: The type theory underlying Lean is, among other things, a formal semantics for proofs. A term `t : P` is simultaneously syntactic (a data structure in memory) and semantic (a proof of the proposition P). The elaborator is a semantic interpreter running in real time as you type.

---

*Next: We turn from language to the central object of study — propositions and what makes something true or false.*
