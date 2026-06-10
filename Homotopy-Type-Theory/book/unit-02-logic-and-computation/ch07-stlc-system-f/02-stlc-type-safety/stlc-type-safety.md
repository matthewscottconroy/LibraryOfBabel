# STLC: Type Safety

## What "Safe" Means

A type system is safe if well-typed programs behave well at runtime. More precisely: a well-typed program never reaches a "stuck" state — a configuration where evaluation should continue but no evaluation rule applies.

Stuck states are the type-theoretic analog of runtime errors. Consider an untyped language: the term `3 + true` is syntactically valid, but evaluating `+` requires two numbers. When the evaluator reaches `3 + true`, it has no rule to apply (addition is not defined on booleans). The term is stuck.

Types prevent stuck states. If a program has type `Nat → Nat` and you apply it to a `Nat`, the result has type `Nat`. The type system guarantees that every subterm is used in a way consistent with its type, ruling out the cases where evaluation would get stuck.

The formal statement of type safety is two theorems:

**Preservation** (subject reduction): if a term has type $A$ and reduces, the result also has type $A$.

**Progress**: a closed, well-typed term is either already a value (computation complete) or can reduce (computation can continue).

Together, these say: evaluation of a well-typed program never gets stuck, and types are maintained throughout.

## Setting the Stage: Values and Reduction

**Values** are the irreducible terms — the results of computation:
$$v ::= \lambda x : A.\, t \mid (v_1, v_2) \mid \mathsf{inl}(v) \mid \mathsf{inr}(v) \mid \star$$

A value is a lambda abstraction (a function), a pair of values, an injection of a value, or the unit element. Values cannot be reduced further; they are the "final answers."

**Small-step operational semantics**: we define a reduction relation $t \to t'$ giving one-step computation.

**Beta reduction** (the fundamental rule):
$$(\lambda x : A.\, t)\, v \to t[v/x]$$

Note: we require the argument to be a *value* $v$ before applying (call-by-value semantics). This means arguments are evaluated fully before being passed to functions.

**Congruence rules** (reduction in context):
$$\frac{t \to t'}{t\, s \to t'\, s} \qquad \frac{t \to t'}{v\, t \to v\, t'}$$

$$\frac{t_1 \to t_1'}{(t_1, t_2) \to (t_1', t_2)} \qquad \frac{t_2 \to t_2'}{(v_1, t_2) \to (v_1, t_2')}$$

These rules say: if a subterm can reduce, reduce it. The congruence rules determine the order of evaluation: evaluate the function before the argument (left-to-right in the first two rules), and evaluate pairs component-by-component.

## The Canonical Forms Lemma

The proof of progress relies on knowing what values of each type look like.

**Lemma (Canonical Forms).** Let $v$ be a value.

(a) If $\vdash v : A \to B$, then $v = \lambda x : A.\, t$ for some $t$.
(b) If $\vdash v : A \times B$, then $v = (v_1, v_2)$ for some values $v_1, v_2$.
(c) If $\vdash v : A + B$, then $v = \mathsf{inl}(v_1)$ or $v = \mathsf{inr}(v_2)$.
(d) If $\vdash v : \mathbf{1}$, then $v = \star$.
(e) There is no value $v$ with $\vdash v : \mathbf{0}$.

*Proof.* By inspection of the value grammar and the typing rules. Only lambda abstractions can have function types (by the Abs rule), only pairs can have product types, etc. $\square$

The lemma for $\mathbf{0}$ is crucial: there is no value of the empty type. This is the statement that the empty type is empty — a trivially true but formally important fact.

## The Substitution Lemma

Before proving the main theorems, we need:

**Lemma (Substitution Preserves Typing).** If $\Gamma, x : A \vdash t : B$ and $\Gamma \vdash s : A$, then $\Gamma \vdash t[s/x] : B$.

*Proof.* By induction on the typing derivation of $\Gamma, x : A \vdash t : B$.

**Case Var**: $t = x$, so $t[s/x] = s$, and $\Gamma \vdash s : A = B$. ✓

$t = y \neq x$: $t[s/x] = y$, which has type $B$ from $\Gamma$. ✓

**Case Abs**: $t = \lambda y : C.\, u$ with $y \neq x$ and $y \notin \text{FV}(s)$ (by alpha-renaming if necessary). By the typing rules, $\Gamma, x : A, y : C \vdash u : D$ where $B = C \to D$. By the induction hypothesis, $\Gamma, y : C \vdash u[s/x] : D$. By Abs, $\Gamma \vdash \lambda y : C.\, u[s/x] : C \to D = B$. ✓

**Case App**: $t = f\, r$ with $\Gamma, x : A \vdash f : C \to B$ and $\Gamma, x : A \vdash r : C$. By IH, $\Gamma \vdash f[s/x] : C \to B$ and $\Gamma \vdash r[s/x] : C$. By App, $\Gamma \vdash f[s/x]\, r[s/x] = (f\, r)[s/x] : B$. ✓ $\square$

## Preservation (Subject Reduction)

**Theorem (Preservation).** If $\Gamma \vdash t : A$ and $t \to t'$, then $\Gamma \vdash t' : A$.

*Proof.* By induction on the derivation of $t \to t'$.

**Case Beta**: $t = (\lambda x : B.\, u)\, v$ reduces to $u[v/x] : A$. From the typing, $\Gamma \vdash \lambda x : B.\, u : B \to A$ (so $\Gamma, x : B \vdash u : A$) and $\Gamma \vdash v : B$. By the Substitution Lemma, $\Gamma \vdash u[v/x] : A$. ✓

**Case Projection**: $\pi_1\,(v_1, v_2) \to v_1$. If $\Gamma \vdash \pi_1\,(v_1, v_2) : A$, then the typing requires $\Gamma \vdash (v_1, v_2) : A \times B$ for some $B$, so $\Gamma \vdash v_1 : A$. ✓

**Congruence cases**: if $t = r\, s$ reduces via $r \to r'$, then $\Gamma \vdash r : B \to A$ and $\Gamma \vdash s : B$. By IH, $\Gamma \vdash r' : B \to A$. So $\Gamma \vdash r'\, s : A$. ✓ Similar arguments for other congruence rules. $\square$

## Progress

**Theorem (Progress).** If $\vdash t : A$ (closed term), then either $t$ is a value or there exists $t'$ with $t \to t'$.

*Proof.* By induction on the typing derivation of $\vdash t : A$.

**Case Var**: impossible — $t$ is a closed term, so no free variables exist.

**Case Abs**: $t = \lambda x : A.\, u$ is already a value. ✓

**Case App**: $t = f\, s$ with $\vdash f : B \to A$ and $\vdash s : B$. By IH:
- If $f \to f'$: then $t \to f'\, s$ by the congruence rule. ✓
- If $f$ is a value and $s \to s'$: then $t \to f\, s'$ by the congruence rule. ✓
- If both $f$ and $s$ are values: by the Canonical Forms lemma, $f = \lambda x : B.\, u$. So $t = (\lambda x : B.\, u)\, s$ reduces by beta to $u[s/x]$. ✓

**Case Pair**: $t = (t_1, t_2)$. If $t_1$ or $t_2$ can reduce, $t$ can reduce. If both are values, $t$ is a value. ✓

**Case $\bot$E** (ex falso): $t = \mathsf{absurd}\, s$ with $\vdash s : \mathbf{0}$. By IH on $s$:
- If $s \to s'$: then $t \to \mathsf{absurd}\, s'$. ✓
- If $s$ is a value: by Canonical Forms, there is no value of type $\mathbf{0}$. Contradiction. ✓ (This case is vacuous — it cannot arise.) $\square$

## The Significance of the $\bot$ Case

The $\bot$E case in the Progress proof is crucial for the connection to consistency.

The proof says: if we have a closed value of type $\mathbf{0}$ (the empty type), we have a contradiction. Since Progress is proved, and $\bot$E applied to a closed value of $\mathbf{0}$ would give a stuck term (no reduction applies if the argument is a value of $\mathbf{0}$), the Progress theorem would fail if any closed value of type $\mathbf{0}$ existed.

Since Progress holds — it is a theorem — no closed value of type $\mathbf{0}$ can exist. This is the consistency of the type theory: $\mathbf{0}$ has no elements, corresponding to $\bot$ being unprovable.

This is a different argument for consistency than the normalization-based argument of Chapter 6. Here, consistency follows from the operational semantics being well-defined — from the fact that evaluation of well-typed programs never gets stuck. The logical and operational perspectives on consistency are equivalent but expressed differently.

## Type Safety as a Design Principle

The framework of progress and preservation has become a standard design principle for programming language type systems. When designing a new type system or language feature, the first question is: do progress and preservation hold?

If a new feature breaks progress (some well-typed programs get stuck), the type system is unsound — it fails to guarantee what it promises. If a feature breaks preservation (some reductions change the type of a term), the type system cannot be trusted across computation steps.

This design principle has been applied to:
- **Reference types and mutation**: a reference cell $\mathsf{ref}(v) : A\, \mathsf{ref}$ contains a value of type $A$. The store is typed, and preservation requires that the store stays well-typed across reductions.
- **Subtyping**: if $A <: B$ and $t : A$, then $t : B$. Preservation must account for widening: if a term of type $A$ reduces to a term, the result still has type $B$ if $A <: B$.
- **Concurrency**: if two threads reduce in parallel, the combined state must remain well-typed. This requires subject reduction for concurrent step relations.
- **Effects and monads**: if $t : M(A)$ (a computation with effect $M$ producing an $A$), then running the computation should preserve the typing through the effect.

In each case, proving progress and preservation is the formal validation that the type system is coherent. Failure reveals design flaws. The progress and preservation framework is one of the most useful tools in programming language theory.
