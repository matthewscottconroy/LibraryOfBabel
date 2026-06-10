# Propositions as Types

## The Central Identification

The Curry-Howard correspondence rests on one precise claim: a proposition $P$ is identified with the type of its proofs.

This is not a loose metaphor. If $P$ is the proposition "$A$ and $B$," then the type of proofs of $P$ is the product type $A \times B$. A proof of $P$ is a pair $(a, b)$ where $a$ proves $A$ and $b$ proves $B$. If $P$ is the proposition "if $A$ then $B$," then the type of proofs is the function type $A \to B$. A proof of $P$ is a function that converts any proof of $A$ into a proof of $B$.

Under this identification, every logical connective corresponds to a type constructor, and every inference rule corresponds to a typing rule. We work through the dictionary in full.

## Conjunction as Product Type

**Proposition**: $A \wedge B$. **Type**: $A \times B$ (product type, also written $A \times B$ or Pair$(A, B)$).

**Proof**: a pair $(a, b)$ where $a : A$ and $b : B$.

**Natural deduction rule** (Introduction):
$$\frac{\Gamma \vdash A \quad \Gamma \vdash B}{\Gamma \vdash A \wedge B} \quad (\wedge\text{I})$$

**Typing rule** (Pair formation):
$$\frac{\Gamma \vdash a : A \quad \Gamma \vdash b : B}{\Gamma \vdash (a, b) : A \times B} \quad (\text{Pair})$$

These are formally identical. The turnstile judgment $\Gamma \vdash A$ (logical) is the same as $\Gamma \vdash a : A$ (type-theoretic) — the latter just makes the proof term $a$ explicit.

**Elimination rules**:

Logically: from $A \wedge B$, extract $A$ or extract $B$.
Type-theoretically: from a pair $(a, b) : A \times B$, project out $\pi_1(p) : A$ or $\pi_2(p) : B$.

**Computation rule** (beta reduction):
$$\pi_1((a, b)) \to_\beta a \qquad \pi_2((a, b)) \to_\beta b$$

This corresponds to the detour reduction in Section 3: introducing a conjunction and immediately eliminating it simplifies to just having the component.

## Implication as Function Type

**Proposition**: $A \to B$. **Type**: $A \to B$ (function type).

**Proof**: a function $f : A \to B$ that converts any proof $a : A$ into a proof $f(a) : B$.

**Natural deduction** (Introduction — hypothetical reasoning):
$$\frac{\Gamma, A \vdash B}{\Gamma \vdash A \to B} \quad (\to\text{I})$$

**Typing rule** (Lambda abstraction):
$$\frac{\Gamma, x : A \vdash t : B}{\Gamma \vdash \lambda x.\, t : A \to B} \quad (\text{Abs})$$

Again formally identical. The hypothetical reasoning "assume $A$, derive $B$, discharge to get $A \to B$" is exactly lambda abstraction: "bind variable $x : A$, build term $t : B$, abstract to get $\lambda x.\, t : A \to B$."

**Elimination** (Modus ponens / Application):
$$\frac{\Gamma \vdash f : A \to B \quad \Gamma \vdash a : A}{\Gamma \vdash f\, a : B}$$

**Computation rule**:
$$(\lambda x.\, t)\, a \to_\beta t[a/x]$$

Beta reduction is substitution of the argument for the bound variable — exactly the detour reduction for implication.

## Disjunction as Sum Type

**Proposition**: $A \vee B$. **Type**: $A + B$ (coproduct or sum type, tagged union).

**Elements**: either $\mathsf{inl}(a)$ (a tagged proof of $A$) or $\mathsf{inr}(b)$ (a tagged proof of $B$).

**Introduction rules** map to constructors; **elimination** (case analysis) maps to pattern matching:

```
case e of
  inl x => t₁
  inr y => t₂
```

This is the computational realization of proof by cases: given a proof of $A \vee B$, handle both cases by specifying what to do with each.

**Computation rules**:
$$\text{case}(\mathsf{inl}(a), x.t_1, y.t_2) \to_\beta t_1[a/x]$$
$$\text{case}(\mathsf{inr}(b), x.t_1, y.t_2) \to_\beta t_2[b/y]$$

Since we know which disjunct holds, we take the appropriate branch directly.

## Empty Type and Unit Type

**False ($\bot$)** corresponds to the **empty type $\mathbf{0}$** — the type with no elements.

Elimination of $\bot$ (ex falso, derive anything from absurdity) corresponds to pattern matching on the empty type, which has no cases: any output type is vacuously satisfied.

```
absurd : 𝟎 → A
absurd e = case e of {}  -- no cases
```

**Truth ($\top$)** corresponds to the **unit type $\mathbf{1}$** — the type with exactly one element $\star : \mathbf{1}$.

The trivial proof of $\top$ is $\star$. No information is carried; the proof has only one value.

## Negation

**$\neg A$** is defined as $A \to \bot$. Type-theoretically: $A \to \mathbf{0}$.

A proof of $\neg A$ is a function $f : A \to \mathbf{0}$. If $A$ has no elements (is "false"), then this is a function with empty domain — trivially. If $A$ has an element $a$, then $f(a) : \mathbf{0}$ would give an element of the empty type, which is impossible. So $A \to \mathbf{0}$ is inhabited exactly when $A$ is not inhabited.

This is the computational content of negation: "I can derive absurdity from any proof of $A$."

## Universal Quantification as Dependent Product

Moving to predicate logic (and dependent types):

**$\forall x : A, B(x)$** corresponds to the **dependent product type $\Pi_{x:A} B(x)$**.

An element of $\Pi_{x:A} B(x)$ is a function $f$ that, given any $a : A$, produces an element $f(a) : B(a)$. The output type $B(a)$ depends on the input $a$. This is a *dependent function*: the type of the output varies with the input value.

Formation, introduction, elimination, and computation rules:

- **Formation**: if $A : \mathsf{Type}$ and $x : A \vdash B(x) : \mathsf{Type}$, then $\Pi_{x:A} B(x) : \mathsf{Type}$.
- **Introduction**: if $x : A \vdash t : B(x)$, then $\lambda x.\, t : \Pi_{x:A} B(x)$.
- **Elimination**: if $f : \Pi_{x:A} B(x)$ and $a : A$, then $f\, a : B(a)$.
- **Computation**: $(\lambda x.\, t)\, a \to_\beta t[a/x]$.

When $B(x)$ does not depend on $x$ (constant $B$), $\Pi_{x:A} B = A \to B$ — the ordinary function type. Dependent products generalize ordinary functions to the case where the output type varies with the input.

## Existential Quantification as Dependent Sum

**$\exists x : A, B(x)$** corresponds to the **dependent sum type $\Sigma_{x:A} B(x)$**.

An element of $\Sigma_{x:A} B(x)$ is a pair $(a, b)$ where $a : A$ and $b : B(a)$. The second component has a type that depends on the first component's value.

Formation, introduction, elimination, computation:

- **Formation**: if $A : \mathsf{Type}$ and $x : A \vdash B(x) : \mathsf{Type}$, then $\Sigma_{x:A} B(x) : \mathsf{Type}$.
- **Introduction**: if $a : A$ and $b : B(a)$, then $(a, b) : \Sigma_{x:A} B(x)$.
- **Elimination**: if $p : \Sigma_{x:A} B(x)$, then $\pi_1(p) : A$ and $\pi_2(p) : B(\pi_1(p))$.
- **Computation**: $\pi_1((a, b)) = a$, $\pi_2((a, b)) = b$.

When $B(x)$ does not depend on $x$, $\Sigma_{x:A} B = A \times B$ — the ordinary product type. Dependent sums generalize ordinary products.

## The Proof Rules Are Typing Rules

The most striking way to see the correspondence is side by side. Take the natural deduction rules for $\to$ and the typing rules for function types:

**Natural deduction** ($\to$I): if $\Gamma, A \vdash B$, then $\Gamma \vdash A \to B$.

**Typing** (Abs): if $\Gamma, x : A \vdash t : B$, then $\Gamma \vdash \lambda x.\, t : A \to B$.

These are the same rule. In both cases: if the consequent/body can be derived given an extra assumption/variable, then we can form an implication/lambda abstraction in the original context.

The logical notation writes hypotheses as bare propositions; the type-theoretic notation writes them as typed variables $x : A$. The difference is that type theory makes the proof term ($x$ in the hypothesis, $\lambda x.\, t$ in the conclusion) explicit, while natural deduction leaves the proof implicit.

**Natural deduction** ($\to$E): from $\Gamma \vdash A \to B$ and $\Gamma \vdash A$, derive $\Gamma \vdash B$.

**Typing** (App): from $\Gamma \vdash f : A \to B$ and $\Gamma \vdash a : A$, derive $\Gamma \vdash f\, a : B$.

Same rule, with the proof terms made explicit: the proof $f$ of $A \to B$ is applied to the proof $a$ of $A$ to produce the proof $f\, a$ of $B$.

## Classical Logic and Its Computational Cost

What about classical logic? Where does LEM fit in this picture?

LEM would be a term of type $A + (A \to \mathbf{0})$ — for every type $A$, either an element of $A$ or a proof that $A$ is empty. In a *normalizing* type theory, no such term can exist for an arbitrary type $A$: if it did, we would have a program that, for any type, decides whether that type is inhabited.

Classical logic, under Curry-Howard, corresponds to programming languages with *control operators*: call/cc (call-with-current-continuation), throw and catch (for exceptions), or the $\mu$-abstraction of Parigot's $\lambda\mu$-calculus. These are valid computational constructs, but they do not "compute a value" in the ordinary sense — they can jump out of the current computation context.

The Curry-Howard correspondence for classical logic gives: Peirce's law $((A \to B) \to A) \to A$ corresponds to the call/cc operator `call-with-current-continuation : ((A → B) → A) → A`. This is an operator that takes a "potential continuation" and uses it to escape the current context with a value of type $A$. It is a standard feature of Scheme and is related to exceptions in other languages.

Classical axioms thus have computational content — but non-standard computational content. This explains why proof assistants built on constructive type theory (Coq, Agda, Lean) treat classical axioms as optional additions, and why adding them breaks certain computational properties (like strong normalization of the internal reduction).

## The Correspondence Across the Curriculum

The Curry-Howard correspondence is not just a result about propositional logic. It extends:

- **Predicate logic** ↔ **Dependent type theory** (Martin-Löf, 1972): $\forall$ and $\exists$ correspond to $\Pi$ and $\Sigma$ types.
- **Second-order logic** ↔ **System F** (Girard, 1971): quantification over propositions/types corresponds to polymorphism.
- **Modal logic** ↔ **Monadic type theory**: the $\square$ and $\diamond$ modalities correspond to type constructors for effects.
- **Linear logic** ↔ **Session types / linear type theory** (Wadler, Caires): resource-sensitivity in logic corresponds to uniqueness of type-linear values.
- **Equality propositions** ↔ **Identity types** (Martin-Löf, 1973; HoTT): proofs of $a = b$ are elements of $\mathsf{Id}_A(a, b)$.

The last extension is the most significant. In ordinary type theory, the identity type $a =_A b$ has at most one element (by Streicher's K axiom or Uniqueness of Identity Proofs). In HoTT, the identity type can have multiple distinct elements — paths — and these paths can themselves have higher identifications — homotopies. This is the Curry-Howard correspondence taken to its logical conclusion: if proofs are types, and propositions of equality are types, then the space of equality proofs is a type with its own internal structure. That structure is what HoTT studies.
