# Propositional Logic

## Before Meaning, Before Truth

There is something strange about mathematics that we rarely pause to notice. When a mathematician writes "P ∧ Q → R," they have not yet said anything true or false. The symbols are marks on paper. The question of what they *mean* — whether they are true, whether they follow from other things — comes later. First, we must agree on what counts as a legal expression.

This two-stage approach — syntax before semantics, form before interpretation — is not pedantry. It reflects a genuine and important distinction. Syntax tells us what strings of symbols are well-formed. Semantics tells us what those well-formed strings mean. Keeping them separate lets us ask, precisely, which meanings a given formal system can express and which inferences it validates.

Propositional logic is the simplest place to practice this discipline. We introduce it here not because it is the destination — predicate logic, type theory, and dependent types are the destinations — but because the formal habits it builds are exactly what every later system requires.

## Atomic Propositions

We begin with the smallest possible units: *atomic propositions*, also called *propositional variables* or *atoms*. These are unanalyzed statements — they have no internal structure as far as propositional logic is concerned.

We write atoms as uppercase letters: P, Q, R, S, or P₁, P₂, P₃, ... We assume infinitely many of them are available. Examples of statements we might encode as atoms:

- P: "It is raining."
- Q: "The ground is wet."
- R: "There is a rainbow."
- S: "The lecture has started."

Propositional logic does not ask why P is true, or what it means for rain to fall. It only tracks whether P is true or false, and how that truth value interacts with the truth values of other atoms via logical connectives.

## The Five Connectives

From atoms, we build complex propositions using *logical connectives*. There are five standard connectives in classical propositional logic:

| Symbol | Name | Pronunciation | Arity |
|--------|------|---------------|-------|
| ¬ | Negation | "not" | 1 (unary) |
| ∧ | Conjunction | "and" | 2 (binary) |
| ∨ | Disjunction | "or" | 2 (binary) |
| → | Implication | "if ... then" | 2 (binary) |
| ↔ | Biconditional | "if and only if" | 2 (binary) |

A word on implication: the formula φ → ψ is false only when φ is true and ψ is false. This surprises many students who encounter it first. "If it's raining, the ground is wet" — how can that be true when it's not raining? The answer is that material implication does not claim causation. It claims: *it never happens that φ is true while ψ is false*. When φ is false, that claim is vacuously satisfied.

A word on disjunction: in English, "or" is sometimes exclusive (one or the other but not both). In logic, ∨ is always *inclusive*: φ ∨ ψ is true when at least one of φ, ψ is true, including when both are.

## The Inductive Definition of Formulas

We now give the precise definition of a *well-formed formula* (wff).

**Definition.** The set Form of well-formed formulas of propositional logic is defined inductively:

**Base case.** Every atomic proposition P is a wff.

**Inductive cases.** If φ and ψ are wffs, then so are:
- ¬φ
- (φ ∧ ψ)
- (φ ∨ ψ)
- (φ → ψ)
- (φ ↔ ψ)

**Closure.** Nothing else is a wff.

The closure clause is crucial. It says the definition is *exhaustive*: a string of symbols is a wff only if it can be built by the rules above, not merely because it looks formula-like. This makes the definition precise: Form is the smallest set satisfying the clauses, or equivalently, the intersection of all sets closed under them.

This is an *inductive definition*, a pattern you will encounter repeatedly. Inductive types in dependent type theory are defined by exactly the same structure: base cases and construction rules, with everything else excluded. Get comfortable with this pattern now.

## Precedence Conventions

Strictly speaking, every binary connective should be surrounded by parentheses: ((P ∧ Q) → R) rather than P ∧ Q → R. The parentheses guarantee unambiguous parsing.

In practice, precedence conventions reduce the clutter:

1. ¬ binds most tightly (applied first, prefix)
2. ∧ (left-associative)
3. ∨ (left-associative)
4. → (right-associative)
5. ↔ (binds least tightly, left-associative)

So P ∨ Q → R ∧ S parses as (P ∨ Q) → (R ∧ S), and P → Q → R parses as P → (Q → R).

**Convention note.** Different texts use different conventions, and some authors omit the biconditional from the primitive connectives (defining φ ↔ ψ as (φ → ψ) ∧ (ψ → φ)). When reading any source, identify its conventions before working with formulas.

## Subformulas

**Definition.** The *subformulas* of a formula φ are defined by structural recursion:
- If φ = P (atomic), then Sub(P) = {P}.
- If φ = ¬ψ, then Sub(¬ψ) = {¬ψ} ∪ Sub(ψ).
- If φ = (ψ ★ χ) for any binary connective ★, then Sub(φ) = {φ} ∪ Sub(ψ) ∪ Sub(χ).

Every formula is a subformula of itself. A *proper* subformula is one strictly smaller. The subformula relation is well-founded: every formula has finitely many subformulas, all of smaller complexity. This is why structural induction on formulas always terminates.

## Truth Tables: Semantics via Assignments

An *assignment* (or *valuation*) is a function v: Atoms → {T, F} that assigns a truth value to every atomic proposition. An assignment extends to all formulas by the following truth tables:

**Negation:**
| φ | ¬φ |
|---|-----|
| T | F |
| F | T |

**Conjunction:**
| φ | ψ | φ ∧ ψ |
|---|---|--------|
| T | T | T |
| T | F | F |
| F | T | F |
| F | F | F |

**Disjunction:**
| φ | ψ | φ ∨ ψ |
|---|---|--------|
| T | T | T |
| T | F | T |
| F | T | T |
| F | F | F |

**Implication:**
| φ | ψ | φ → ψ |
|---|---|--------|
| T | T | T |
| T | F | F |
| F | T | T |
| F | F | T |

**Biconditional:**
| φ | ψ | φ ↔ ψ |
|---|---|--------|
| T | T | T |
| T | F | F |
| F | T | F |
| F | F | T |

These tables are *definitions*. The connectives mean exactly what the tables say. If you want a different notion of "or" or a different notion of "if-then," you need a different connective.

Given an assignment v, we write v ⊨ φ (read: "v satisfies φ" or "φ is true under v") to mean the truth value of φ under v is T.

## Tautologies and Contradictions

**Definition.** A formula φ is a *tautology* (or *logical truth*) if v ⊨ φ for every assignment v. It is a *contradiction* (or *unsatisfiable*) if v ⊭ φ for every assignment v. It is *satisfiable* if v ⊨ φ for at least one v.

**Examples of tautologies:**
- P → P (trivially, anything implies itself)
- P ∨ ¬P (the law of excluded middle)
- ¬(P ∧ ¬P) (the law of non-contradiction)
- (P → Q) → (¬Q → ¬P) (contrapositive)
- ((P → Q) ∧ (Q → R)) → (P → R) (hypothetical syllogism)
- (P ∧ (P → Q)) → Q (modus ponens as a tautology)

To check whether a formula is a tautology, we check all 2ⁿ assignments of truth values to its n atomic propositions. For formulas with many atoms, this is computationally expensive — in fact, the problem of determining whether a propositional formula is satisfiable (SAT) is NP-complete, one of the most famous problems in theoretical computer science.

## Semantic Consequence

**Definition.** A set of formulas Γ *semantically entails* φ, written Γ ⊨ φ, if every assignment that satisfies all formulas in Γ also satisfies φ.

When Γ = ∅, we have ⊨ φ, which means φ is a tautology.

**Examples:**
- {P, P → Q} ⊨ Q. Proof: suppose v ⊨ P and v ⊨ P → Q. Then since v(P) = T and v(P → Q) = T, we cannot have v(Q) = F (that would make P → Q false). So v(Q) = T.
- {P ∨ Q, ¬P} ⊨ Q. Proof: if P ∨ Q is true and P is false, then Q must be true.
- {P → Q, ¬Q} ⊨ ¬P. This is the *contrapositive* form of modus tollens.

Semantic consequence captures the *content* of logical inference: an argument is valid when its premises force its conclusion to be true.

## Syntactic Consequence and Proof Systems

There is another notion of consequence, purely syntactic: we say Γ ⊢ φ (read: "φ is provable from Γ") if there is a formal *proof* — a finite sequence of steps, each justified by an inference rule — deriving φ from Γ.

What are the inference rules? There are several equivalent choices: Hilbert-style axiom systems, sequent calculus, natural deduction. We adopt natural deduction, introduced by Gentzen in 1935, because it most closely mirrors how mathematicians actually reason.

The key meta-theorem connecting syntax and semantics is:

**Soundness:** If Γ ⊢ φ then Γ ⊨ φ. Every provable formula is valid.

**Completeness:** If Γ ⊨ φ then Γ ⊢ φ. Every valid formula is provable.

Together, soundness and completeness say that the proof system captures *exactly* the valid inferences — no more, no less. We prove the completeness theorem (in the predicate logic setting) in Section 5.

## Logical Equivalence

Two formulas φ and ψ are *logically equivalent*, written φ ≡ ψ, if φ ↔ ψ is a tautology — equivalently, if v ⊨ φ iff v ⊨ ψ for every assignment v.

Important logical equivalences:

**De Morgan's laws:**
- ¬(φ ∧ ψ) ≡ ¬φ ∨ ¬ψ
- ¬(φ ∨ ψ) ≡ ¬φ ∧ ¬ψ

**Contrapositive:** φ → ψ ≡ ¬ψ → ¬φ

**Double negation:** ¬¬φ ≡ φ

**Distribution:**
- φ ∧ (ψ ∨ χ) ≡ (φ ∧ ψ) ∨ (φ ∧ χ)
- φ ∨ (ψ ∧ χ) ≡ (φ ∨ ψ) ∧ (φ ∨ χ)

**Exportation:** (φ ∧ ψ) → χ ≡ φ → (ψ → χ)

The last equivalence — exportation — deserves special attention. In type theory, it says that a function of two arguments is the same as a curried function: a function that takes one argument and returns another function. The isomorphism between A × B → C and A → (B → C) is called *currying*, after Haskell Curry, and it is one of the deepest structural facts in the Curry-Howard correspondence.

## Normal Forms

Any formula can be converted to a canonical form that makes certain analyses easier.

**Conjunctive Normal Form (CNF).** A formula is in CNF if it is a conjunction of *clauses*, where each clause is a disjunction of *literals* (atoms or negated atoms). Example: (P ∨ ¬Q) ∧ (¬P ∨ R) ∧ Q.

**Disjunctive Normal Form (DNF).** A formula is in DNF if it is a disjunction of *cubes*, where each cube is a conjunction of literals. Example: (P ∧ ¬Q) ∨ (¬P ∧ R).

Every formula is logically equivalent to a formula in CNF, and to a formula in DNF. The conversion uses De Morgan's laws and the distribution laws.

CNF is particularly important in computer science: the SAT problem asks whether a CNF formula is satisfiable. Modern SAT solvers, which can handle millions of variables, are used in hardware verification, software analysis, and AI planning.

## The Connection to Type Theory

In dependent type theory, every propositional connective has a type-theoretic counterpart:

| Propositional Logic | Type Theory |
|---------------------|-------------|
| P ∧ Q | Product type A × B |
| P ∨ Q | Sum type A + B (coproduct) |
| P → Q | Function type A → B |
| ¬P | Function type A → ⊥ |
| ⊤ (truth) | Unit type 1 |
| ⊥ (falsehood) | Empty type 0 |

Under this correspondence (the Curry-Howard correspondence), a *proof* of the proposition P is a *term* of the type P. Proving P ∧ Q means constructing a pair (a, b) where a is a proof of P and b is a proof of Q. Proving P → Q means constructing a function that takes a proof of P and returns a proof of Q.

This is not an analogy. In a dependent type theory, propositions *are* types and proofs *are* terms. The distinction between "logic" and "programming" dissolves. What we study in this chapter as propositional logic, we will later see as the simply-typed lambda calculus. The formulas are types. The proofs are programs.

This perspective — logic as programming — is the animating idea of all proof assistants. Every time you state a lemma in Lean and prove it, you are constructing a term of a type. The type-checker verifies the proof by type-checking the term. Propositional logic is the simplest setting to see this structure, before the additional complexity of dependent types enters.

Study the connectives here. The programs they encode will be yours to write.
