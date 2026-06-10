# The Four Judgments of MLTT

## What a Judgment Is

A formal system works by deriving *judgments* — basic forms of assertion that the system recognizes as provable. In propositional calculus, there is one judgment form: "P is provable." In first-order logic, there is "P is provable (under assumptions Γ)." In MLTT, there are four.

Four is not an accident. Four is the minimum needed to capture the relationship between types and terms, between definitional and propositional equality, and between individual judgments and the contexts in which they are made. Reduce to fewer and you lose precision. Add more and you get redundancy. Four is what the theory requires.

## The Four Forms

All four judgments are made relative to a *context* Γ — a list of variable declarations. We write:

**J1.** Γ ⊢ A type — "A is a well-formed type in context Γ."

**J2.** Γ ⊢ A ≡ B type — "A and B are definitionally equal types in context Γ."

**J3.** Γ ⊢ a : A — "a is a term of type A in context Γ."

**J4.** Γ ⊢ a ≡ b : A — "a and b are definitionally equal terms of type A in context Γ."

The symbol ≡ is definitional (or judgmental) equality. It is not the identity type — that is propositional equality, written a = b, which is a type. Definitional equality is a *judgment*, something the type checker verifies automatically by computation. Propositional equality is a *type*, something you prove by constructing a term of that type.

This distinction is the most important technical fact in the chapter.

## Contexts: Telescopes of Dependencies

A *context* Γ is a finite list of variable declarations:

$$\Gamma = (x_1 : A_1,\, x_2 : A_2(x_1),\, x_3 : A_3(x_1, x_2),\, \ldots,\, x_n : A_n(x_1, \ldots, x_{n-1}))$$

Each type A_i may depend on the variables declared before it. This is what makes the context *telescopic* — it extends one dependency at a time.

**Examples:**
- () — the empty context, no assumptions
- (n : ℕ) — one variable, a natural number
- (A : Type₀, a : A) — a type and one of its elements
- (A : Type₀, x : A, y : A, p : x =_A y) — a type, two elements, and a proof of their equality
- (n : ℕ, v : Vec A n) — a number and a vector of that length

In the last example, the type of v (namely Vec A n) mentions the variable n declared before it. This is the dependence. The context is not just a set of assumptions — it is an *ordered* list where later types can see earlier values.

**Context validity** is itself a judgment: we say Γ ctx to mean Γ is a well-formed context. The rules:
- () ctx — the empty context is valid
- If Γ ctx and Γ ⊢ A type, then (Γ, x : A) ctx for fresh x

Building a valid context is not automatic. You must verify that each successive type is well-formed under the existing declarations.

## Why We Need J1 and J2 Separately

J1 (Γ ⊢ A type) asserts that A is a well-formed type. In a non-dependent type system, this is trivial — types are syntactic expressions and every syntactically well-formed type expression is a type. But in dependent type theory, a type expression can be ill-formed:

- Vec A n requires n : ℕ. If n : Bool instead, Vec A n is not a valid type.
- a =_A b requires a, b : A. If a : ℕ and b : String, the expression a = b is not a valid type.

The judgment J1 says: after checking all the dependencies, this expression is a genuine type.

J2 (Γ ⊢ A ≡ B type) says that two types are definitionally equal. This is needed for the *conversion rule*: if Γ ⊢ a : A and Γ ⊢ A ≡ B type, then Γ ⊢ a : B. A term of type A is automatically a term of any type definitionally equal to A. The type checker applies this silently, which is why you can write `a : Vec ℕ (2 + 3)` and the type checker accepts it as an element of `Vec ℕ 5` — the two types are definitionally equal.

## Why We Need J3 and J4 Separately

J3 (Γ ⊢ a : A) asserts that a is a term of type A. This is the basic typing judgment.

J4 (Γ ⊢ a ≡ b : A) asserts that two terms are definitionally equal at type A. This is needed because terms reduce: `(λx. x + 1) 4` and `5` are definitionally equal (they both compute to the same value). The type checker must identify definitionally equal terms, and having J4 as a primitive judgment makes this precise.

The conversion rule for terms: if Γ ⊢ a ≡ b : A, then anything that is true of a is also true of b (definitionally). In particular, if Γ ⊢ f a : B and Γ ⊢ a ≡ a' : A, then Γ ⊢ f a ≡ f a' : B.

## The Structural Rules

The following rules hold for all four judgment forms and do not depend on any specific type former.

**Variable rule:**
$$\frac{x : A \in \Gamma}{\Gamma \vdash x : A}$$
Any variable in scope can be used as a term of its declared type.

**Weakening:**
$$\frac{\Gamma \vdash J \quad \Gamma' \supseteq \Gamma}{\Gamma' \vdash J}$$
Adding more variables to the context does not invalidate existing judgments.

**Substitution:** If Γ ⊢ a : A and Γ, x : A, Γ' ⊢ J, then Γ, Γ'[a/x] ⊢ J[a/x]. Substituting a term for a variable is valid.

**Conversion (type conversion):**
$$\frac{\Gamma \vdash a : A \quad \Gamma \vdash A \equiv B\ \mathsf{type}}{\Gamma \vdash a : B}$$
If two types are definitionally equal, a term of one is a term of the other.

**Reflexivity, symmetry, transitivity of ≡:** Definitional equality is an equivalence relation on both types and terms.

**Congruence:** Definitional equality respects all type-forming operations. If f ≡ f' and a ≡ a', then f a ≡ f' a' (and similarly for lambda abstraction, pairs, etc.).

## The Relationship Between ≡ and =

Definitional equality (≡) implies propositional equality (=). Concretely: from Γ ⊢ a ≡ b : A, we can derive Γ ⊢ refl_a : a =_A b (since refl_a : a =_A a and a ≡ b : A, by conversion refl_a : a =_A b).

Propositional equality (=) does **not** imply definitional equality in intensional MLTT. You can have a proof p : a = b where a and b do not reduce to the same term. This is the crucial point: propositional equality is weaker than definitional equality.

**Example:** `succ(n) + m` and `succ(n + m)` are definitionally equal (this follows from the computation rules for addition). But `n + m` and `m + n` are only propositionally equal — commutativity requires a proof by induction.

The table:

| | Definitional equality (≡) | Propositional equality (=) |
|---|---|---|
| What it is | A judgment, checked by the type checker | A type, proved by constructing a term |
| Strength | Stronger (implies propositional) | Weaker |
| Decidability | Decidable in intensional MLTT | Propositional equality proofs may be complex |
| Example holds | 2 + 3 ≡ 5 | n + m = m + n (proved by induction) |
| Example fails | n + m ≡ m + n (in general) | — |

## Definitional Equality: The Computation Rules

The computation rules for each type former define what reduces to what at the definitional level. The key rules are:

**β-reduction (Π):** (λx. t) a ≡ t[a/x]
**β-reduction (Σ):** fst(a, b) ≡ a; snd(a, b) ≡ b
**ι-reduction (ℕ):** ind_ℕ(C, c_z, c_s, zero) ≡ c_z; ind_ℕ(C, c_z, c_s, succ(n)) ≡ c_s(n, ind_ℕ(C, c_z, c_s, n))
**β-reduction (identity):** J(C, d, a, refl_a) ≡ d

These rules define the reduction relation →. Definitional equality (≡) is then the equivalence relation generated by → (closed under reflexivity, symmetry, transitivity, and congruence). The type checker decides ≡ by normalizing both sides and comparing normal forms.

**Church-Rosser and strong normalization:** In MLTT (without Type : Type), the reduction relation is confluent (Church-Rosser) and strongly normalizing: every term has a unique normal form and reduction terminates. This is what makes the type checker decidable: it normalizes both sides of a definitional equality and checks if the normal forms are syntactically equal.

## The Context as the Foundation

Every judgment in MLTT is relativized to a context. The context encodes everything that is assumed — what types exist, what terms have been bound, what equalities hold definitionally. When you write a proof in Agda or Lean, the context is the list of things to the left of the ⊢ sign at any point in the proof.

When you write:
```agda
myLemma : (n : ℕ) (v : Vec A n) → Vec A (n + 0)
```
the context during the proof body is (n : ℕ, v : Vec A n). The type Vec A (n + 0) in the conclusion is judged well-formed in this context (because n : ℕ is in scope), and Vec A (n + 0) ≡ Vec A n (since n + 0 ≡ n by the computation rules), so the term v : Vec A n is accepted as a term of type Vec A (n + 0) by the conversion rule.

This is not magic. It is the four judgments, working together.
