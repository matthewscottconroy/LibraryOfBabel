# Proofs as Programs

The Curry-Howard correspondence is one of the deepest insights in the foundations of mathematics and computer science: *proofs and programs are the same thing*, just viewed from different angles.

## The Basic Dictionary

| Logic | Programming |
|-------|-------------|
| Proposition | Type |
| Proof of P | Term of type P |
| P ∧ Q | Product type A × B |
| P ∨ Q | Sum type A + B |
| P → Q | Function type A → B |
| ⊥ (False) | Empty type (Void) |
| ¬P = P → ⊥ | A → Void |
| ∀x:A. P(x) | Π-type: (x:A) → P(x) |
| ∃x:A. P(x) | Σ-type: Σ(x:A), P(x) |

Each proposition corresponds to a type, and each proof of that proposition corresponds to a *program* of that type. Constructing a proof is identical to writing a program.

## Examples

**Proving P → P (identity)**:

In logic: assume P, conclude P. (Axiom rule.)

In programming: `λx. x : A → A` (identity function).

**Proving (P ∧ Q) → P (left projection)**:

In logic: from P ∧ Q, conclude P by ∧-elimination.

In programming: `λp. fst p : A × B → A`.

**Proving (P → Q) → (Q → R) → (P → R) (transitivity)**:

In logic: function composition applied to two implications.

In programming: `λf. λg. λx. g (f x) : (A → B) → (B → C) → (A → C)`.

## Proof Reduction = Program Execution

When you simplify a proof — eliminating detours like "introduce ∧, then immediately eliminate ∧" — you perform the same operation as beta reduction.

```
Proof:   (introduce P ∧ Q, then apply left projection)
Program: (λp. fst p)(pair a b)  →_β  fst (pair a b)  →  a
```

*Cut elimination* in sequent calculus corresponds exactly to *beta reduction* in lambda calculus. Normalization of proofs — finding the most direct proof with no detours — corresponds to evaluating programs to normal form.

Gentzen's *Hauptsatz* (normalization of proofs) and the *strong normalization* theorem for typed lambda calculus are the same result, expressed in two languages.

## Why This Matters

**Verification**: To verify a program `f : A → B` is correct, you construct a proof that the specification holds. The program *is* the proof. There is no separate verification step — it's baked into the type.

**Extraction**: If you prove a theorem constructively in Coq, you can *extract* a certified program in OCaml or Haskell that is correct by construction. The CompCert C compiler was built this way — a proof of compiler correctness yields the compiler itself.

**Intuitionistic logic**: The Curry-Howard correspondence reveals why classical logic (with the law of excluded middle P ∨ ¬P) is harder to compute with. In programming terms, `P ∨ ¬P` would require a decision procedure for every proposition, which is impossible in general. Classical proofs can't always be turned into programs — they sometimes prove existence without construction.

Adding classical logic to type theory requires features like *continuations* (call/cc in Scheme), *control operators*, or *double-negation translation*. The computational content of a classical proof is a program with access to the call stack.

## The Extended Correspondence

The correspondence extends further:

| Proof theory | Type theory | Category theory |
|-------------|-------------|----------------|
| Propositions | Types | Objects |
| Proofs | Terms | Morphisms |
| Cut elimination | Beta reduction | Composition |
| Normalization | Evaluation | … |
| Gentzen's NK | STLC | Cartesian closed categories |
| Intuitionistic logic | Dependent types | Locally cartesian closed categories |
| Modal logic | Staged computation / comonads | … |

This three-way correspondence — the "holy trinity" of logic, types, and categories — suggests they are three descriptions of a single underlying mathematical structure we don't yet fully understand.

## In Practice

When you write a Lean proof:

```lean
theorem modus_ponens {P Q : Prop} (h1 : P → Q) (h2 : P) : Q :=
  h1 h2
```

You are writing a program. `h1 h2` is function application — apply the function `h1 : P → Q` to the argument `h2 : P` to get a result of type `Q`. The proof term *is* the program; Lean's kernel *runs* it to verify the type checks.

The line between "writing a proof" and "writing a program" evaporates. Mathematical reasoning and computation are one.
