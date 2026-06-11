# Chapter 6: The Curry-Howard Correspondence

## Introduction

The Curry-Howard correspondence is one of the deepest and most surprising discoveries in the foundations of mathematics and computer science. It states that:

> **Propositions are types. Proofs are programs.**

This is not a metaphor or an analogy. It is a precise mathematical isomorphism between:
- Intuitionistic propositional logic (Chapter 5) and simply typed lambda calculus (Chapter 7)
- Natural deduction (Chapter 4) and type theory
- Proof normalization and program evaluation

The correspondence was discovered independently by Haskell Curry (1934: he noticed a correspondence between combinators and axioms of implicational logic), William Howard (1969: unpublished manuscript extending the correspondence to natural deduction), and Per Martin-Löf (1970s: the full extension to dependent types and predicate logic).

For us, the Curry-Howard correspondence is not just an intellectual curiosity. It is the *reason* that proof assistants exist: a program in Lean 4 or Agda is simultaneously a mathematical proof and a computable function. The types are the theorems; the programs are the proofs; running the program is checking the proof; normalizing the program is simplifying the proof.

---

## 1. The Central Dictionary

The correspondence maps every connective and rule of natural deduction to a construct of typed lambda calculus:

| **Logic** | **Type Theory** |
|---|---|
| Proposition $P$ | Type $A$ |
| Proof of $P$ | Term $t : A$ |
| Hypothesis $[P]$ | Variable $x : A$ |
| $P \wedge Q$ | Product type $A \times B$ |
| $P \vee Q$ | Sum type $A + B$ (coproduct) |
| $P \to Q$ | Function type $A \to B$ |
| $\bot$ (false) | Empty type $\mathbf{0}$ |
| $\top$ (true) | Unit type $\mathbf{1}$ |
| $\neg P$ | $A \to \mathbf{0}$ |
| $\wedge$-Introduction | Pair constructor $\mathsf{pair}(t, s)$ |
| $\wedge$-Elimination | Projections $\mathsf{fst}$, $\mathsf{snd}$ |
| $\to$-Introduction | Lambda abstraction $\lambda x : A,\, t$ |
| $\to$-Elimination (modus ponens) | Function application $f\, t$ |
| $\vee$-Introduction | Injections $\mathsf{inl}(t)$, $\mathsf{inr}(s)$ |
| $\vee$-Elimination (case analysis) | $\mathsf{case}\, e\, \mathsf{of}\, \mathsf{inl}(x) \Rightarrow t \mid \mathsf{inr}(y) \Rightarrow s$ |
| $\bot$-Elimination (ex falso) | $\mathsf{absurd} : \mathbf{0} \to A$ |
| Detour (intro then elim) | $\beta$-redex: $(\lambda x, t)\, s$ |
| Detour elimination | $\beta$-reduction: $(\lambda x, t)\, s \to t[s/x]$ |
| Normal form proof | Normal form term (value) |
| Modus ponens | Function application |
| Hypothesis discharge | Variable binding |

**Example 6.1 (Identity).** The proof of $P \to P$ in natural deduction (assume $P$, conclude $P$) corresponds to the identity function $\lambda x : A,\, x$ of type $A \to A$.

**Example 6.2 (Composition).** The proof of $(P \to Q) \to (Q \to R) \to (P \to R)$ corresponds to the function composition program:
$$\lambda f : A \to B,\; \lambda g : B \to C,\; \lambda x : A,\; g\,(f\, x)$$
Type: $(A \to B) \to (B \to C) \to (A \to C)$.

**Example 6.3 (Conjunction introduction).** The proof of $P \to Q \to P \wedge Q$ (from $P$ and $Q$, produce the pair) corresponds to:
$$\lambda x : A,\; \lambda y : B,\; (x, y)$$
Type: $A \to B \to A \times B$.

---

## 2. Simply Typed Lambda Calculus (Syntax)

The formal system on the programming side of the correspondence is the *simply typed lambda calculus* (STLC). We preview it here; Chapter 7 treats it in full detail.

### 2.1 Types

Types $A, B, C$ are built from:
- *Base types*: $o$ (or constants like $\mathsf{Bool}$, $\mathsf{Nat}$)
- *Function types*: $A \to B$ (functions from $A$ to $B$)
- *Product types*: $A \times B$ (pairs)
- *Sum types*: $A + B$ (disjoint unions)
- *Unit type*: $\mathbf{1}$ (one element)
- *Empty type*: $\mathbf{0}$ (no elements)

### 2.2 Terms

Terms $t, s, u$ are:
- *Variables*: $x, y, z, \ldots$
- *Lambda abstraction*: $\lambda x : A,\, t$ (a function taking $x$ of type $A$ and returning $t$)
- *Application*: $t\, s$ (apply function $t$ to argument $s$)
- *Pairs*: $(t, s)$ with projections $\mathsf{fst}(t)$ and $\mathsf{snd}(t)$
- *Injections*: $\mathsf{inl}(t)$ and $\mathsf{inr}(s)$ with case elimination $\mathsf{case}\, e\, \mathsf{of}\, \ldots$
- *Unit*: $\star : \mathbf{1}$
- *Absurdity*: $\mathsf{absurd}(t) : A$ when $t : \mathbf{0}$

### 2.3 Typing Rules

A *typing context* $\Gamma = x_1 : A_1, \ldots, x_n : A_n$ assigns types to variables.

The typing judgment $\Gamma \vdash t : A$ says "in context $\Gamma$, term $t$ has type $A$."

$$\frac{}{\Gamma, x : A \vdash x : A} (\text{Var}) \qquad \frac{\Gamma, x : A \vdash t : B}{\Gamma \vdash \lambda x : A,\, t : A \to B} (\text{Abs})$$

$$\frac{\Gamma \vdash t : A \to B \quad \Gamma \vdash s : A}{\Gamma \vdash t\, s : B} (\text{App}) \qquad \frac{\Gamma \vdash t : A \quad \Gamma \vdash s : B}{\Gamma \vdash (t, s) : A \times B} (\text{Pair})$$

These are *identical in structure* to the natural deduction rules for $\to$ and $\wedge$ — with propositions replaced by types and proof terms annotating each rule.

---

## 3. Reduction and Computation

### 3.1 Beta Reduction

The central computation rule of lambda calculus is *$\beta$-reduction*:
$$(\lambda x : A,\, t)\, s \to_\beta t[s/x]$$

Here $t[s/x]$ means "substitute $s$ for $x$ throughout $t$." This is:
- Logically: detour elimination (the intro-then-elim pattern)
- Computationally: function call / substitution

**Example 6.4.** $(\lambda x : \mathbb{N},\, x + 1)\, 3 \to_\beta 3 + 1 \to_\beta 4$.

**Example 6.5.** $((\lambda f : A \to B,\, \lambda x : A,\, f(f\, x))\, (\lambda y : A,\, y))\, a$
$$\to_\beta (\lambda x : A,\, (\lambda y : A,\, y)((\lambda y : A,\, y)\, x))\, a$$
$$\to_\beta (\lambda y : A,\, y)((\lambda y : A,\, y)\, a)$$
$$\to_\beta (\lambda y : A,\, y)\, a \to_\beta a$$

### 3.2 Eta Reduction

A second rule: $\lambda x,\, f\, x \to_\eta f$ (if $x$ does not appear in $f$). This corresponds to the *uniqueness of extensional functions*: two functions agreeing on all inputs are equal.

### 3.3 Computational Interpretation of Proofs

The proofs in our logic correspond to programs that *run*. When we write a proof $t$ of proposition $P$ (type $A$), $t$ is a program that witnesses $P$. Running $t$ (reducing to normal form) is the process of *extracting the computational content*.

**Theorem 6.6 (Curry-Howard, informal).** Under the correspondence:
- Typechecking a proof term = verifying correctness of a proof
- Beta-normalizing a proof term = simplifying a proof to remove detours
- The type of a normal form = the verified conclusion
- A closed term of type $\mathbf{0}$ = a proof of $\bot$ = does not exist (consistency)

---

## 4. Extending to Predicate Logic: Dependent Types

The Curry-Howard correspondence extends from propositional logic to predicate logic when we introduce *dependent types*.

| **Predicate Logic** | **Dependent Type Theory** |
|---|---|
| $\forall x : A,\; P(x)$ | $\Pi_{x:A} P(x)$ (dependent function type) |
| $\exists x : A,\; P(x)$ | $\Sigma_{x:A} P(x)$ (dependent pair type) |
| Proof of $\forall x : A,\; P(x)$ | Function $f$ such that $f(a) : P(a)$ for all $a : A$ |
| Proof of $\exists x : A,\; P(x)$ | Pair $(a, p)$ with $a : A$ and $p : P(a)$ |

The dependent function type $\Pi_{x:A} B(x)$ generalizes $A \to B$: when $B$ does not depend on $x$, it is $A \to B$. When it does, it is the type of functions $f$ where $f(a) : B(a)$ for each $a : A$.

**Example 6.7 (Dependent function).** The function `Vector : ℕ → Type` that maps a natural number $n$ to the type of lists of length $n$ is a dependent function. The statement "for every $n$, every vector of length $n$ has a well-defined length" has type $\Pi_{n : \mathbb{N}} \Pi_{v : \mathsf{Vector}\,n} (|v| = n)$ — a dependent type expressing a universally quantified equality.

**Example 6.8 (Dependent pair).** The type $\Sigma_{n : \mathbb{N}} \mathsf{Even}(n)$ is the type of pairs $(n, p)$ where $n$ is a natural number and $p$ is a proof that $n$ is even. This is the constructive version of $\exists n : \mathbb{N}, \mathsf{Even}(n)$.

---

## 5. Normalization and Consistency

**Theorem 6.9 (Strong Normalization for STLC).** Every well-typed term of the simply typed lambda calculus reduces to a unique normal form in finitely many steps.

*Proof idea.* Assign to each type $A$ a set of *strongly normalizing* terms $\text{Red}(A)$:
- $\text{Red}(o)$: all strongly normalizing terms of base type.
- $\text{Red}(A \to B)$: terms $t$ such that for all $s \in \text{Red}(A)$, $t\, s \in \text{Red}(B)$.

Show by induction on the typing derivation that every well-typed term is in $\text{Red}(A)$, and that $\text{Red}(A)$ consists of strongly normalizing terms. $\square$

**Corollary 6.10 (Consistency).** There is no closed term of type $\mathbf{0}$ in STLC. Equivalently, there is no proof of $\bot$ from no hypotheses in intuitionistic propositional logic.

*Proof.* A closed term of type $\mathbf{0}$ would have a normal form. But the normal form of a closed term of empty type would have to be a variable (impossible — there are no variables in scope) or an absurdity elimination (but that requires a term of type $\mathbf{0}$ to start — circularity). $\square$

This is one of the most important foundational results: *a type-theoretic proof assistant is consistent* (assuming its type theory is correctly implemented) because any proof of False would be a program of the empty type, which does not exist.

---

## 6. The Full Correspondence: Proof Assistants

Modern proof assistants implement the Curry-Howard correspondence directly:

**Lean 4:**
```lean
-- The proposition: P → Q → P (a tautology)
-- The program (proof term): λ hp : P, λ hq : Q, hp
theorem const_proof (P Q : Prop) (hp : P) (hq : Q) : P := hp

-- Conjunction introduction:
-- The proposition: P → Q → P ∧ Q
-- The program: λ hp, λ hq, ⟨hp, hq⟩
theorem and_intro (P Q : Prop) (hp : P) (hq : Q) : P ∧ Q := ⟨hp, hq⟩
```

**Agda:**
```agda
-- The proposition P → Q → P
const : {P Q : Set} → P → Q → P
const p q = p

-- Conjunction introduction
_,_ : {P Q : Set} → P → Q → P × Q
p , q = (p , q)
```

In both cases, the *type signature* is the proposition, and the *term* is the proof. The type checker verifies that the term has the claimed type — which is exactly proof checking.

---

## 7. Looking Forward: Identity Types

In STLC and simple type theory, equality of terms is *definitional*: $t = s$ iff $t$ and $s$ reduce to the same normal form.

In Martin-Löf type theory and HoTT, equality of *elements of a type* is a *type itself*:
$$a =_A b \text{ is a type}$$

A proof of $a = b$ is a *term* of type $a =_A b$. Under Curry-Howard, such a proof is a *path* from $a$ to $b$ in the space $A$.

This is the seed of homotopy type theory: the identity type is the type of paths, and terms of identity types are the fundamental morphisms of the type's higher groupoid structure.

---

## Exercises

**6.1.** Write the proof term (lambda calculus expression) for each of the following propositions:
  - $(P \wedge Q) \to (Q \wedge P)$ (commutativity of $\wedge$)
  - $(P \to Q) \to (Q \to R) \to (P \to R)$
  - $P \vee Q \to Q \vee P$
  - $(P \wedge (P \to Q)) \to Q$

**6.2.** Verify that the terms you wrote in Exercise 6.1 typecheck by checking the typing rules.

**6.3.** Reduce each of the following terms to normal form:
  - $(\lambda x : A, \lambda y : B, x) a\, b$ where $a : A$, $b : B$
  - $(\lambda f : A \to A, f\, (f\, a))\, (\lambda x : A, x)$ where $a : A$
  - $\mathsf{fst}((\lambda x : A, \lambda y : B, (x, y))\, a\, b)$

**6.4.** The *S, K, I combinators* are terms:
  - $I = \lambda x, x$ (identity)
  - $K = \lambda x, \lambda y, x$ (constant)
  - $S = \lambda x, \lambda y, \lambda z, (x\, z)\, (y\, z)$ (substitution)
  
  Give types for $I$, $K$, and $S$ in STLC. What propositions do their types correspond to?

**6.5.** The type $((A \to B) \to A) \to A$ is Peirce's law. Can it be proven in STLC (without extra axioms)? What does this tell us about the computational content (or lack thereof) of Peirce's law?

**6.6.** Consider the proposition $\neg\neg P \to P$ (double negation elimination). In STLC (without classical axioms), this would be a term of type $((A \to \mathbf{0}) \to \mathbf{0}) \to A$. Show that no such term exists (in the sense that there is no closed term of this type in STLC). What would such a term "compute"?

**6.7.** In Lean 4, write proofs (as terms, not using tactics) for:
  - `fun p : P ∧ Q => (p.2, p.1)` (swapping a pair) — verify the type
  - Function composition: write the term and its type

**6.8 (Challenge).** The *simply typed combinator basis*: Show that every simply typed lambda term can be translated into a term using only $S$, $K$, $I$, and variables (no $\lambda$-abstractions). (*Hint:* This translation is the *bracket abstraction* algorithm. It proceeds by structural induction on terms.)

---

## See Also

**In chapters/:**
- `ch04-proof-theory` — Prerequisite. The natural deduction rules whose types are the Curry-Howard counterparts of the typing rules of STLC. Cut elimination corresponds to normalization; the subformula property corresponds to the types of subterms.
- `ch05-intuitionistic-logic` — The propositional fragment: `IPC ≅ STLC`. Every propositional intuitionistic tautology corresponds to an inhabited type in STLC. Classical tautologies that are not intuitionistic correspond to types with no closed inhabitants.
- `ch07-stlc-system-f` — The direct continuation: System F adds universal quantification over types, corresponding to second-order propositional logic. Parametricity (Reynolds) is the computational analog of the proof-theoretic uniformity enforced by the absence of non-logical axioms.
- `ch08-dependent-types` — The generalization: Π-types `(x : A) → B x` extend the function type `A → B` to the dependent case. `∀x. P(x)` becomes `Π(x : A), P x`. The Curry-Howard correspondence extends fully: `Π` is dependent universal quantification and dependent product; `Σ` is dependent existential quantification.
- `ch09-mltt` — MLTT is the full dependent Curry-Howard correspondence, including the identity type `a =_A b` as the type of proofs of equality (paths). The J eliminator is the proof rule for identity-type elimination; the β-reduction for J is the computation rule.

**In book/:**
- `book/unit-02-logic-and-computation/ch06-curry-howard/` — Extended narrative treatment emphasizing the philosophical significance of "propositions as types." Includes discussion of the BHK interpretation and its relationship to intuitionism.

**In demos/:**
- `demos/demo_curry_howard.py` — Interactive visualization of the Curry-Howard correspondence. Enter a proposition in IPC; receive a type in STLC; enter a proof; receive a lambda term.
- `demos/demo_bhk.py` — The Brouwer-Heyting-Kolmogorov interpretation, which the Curry-Howard correspondence makes precise.
- `demos/demo_proof_basics.py` — Basic proof construction corresponding to simple typed programs.

**Key type-theoretic notation:**
- `P ∧ Q ↔ A × B` (product type; `(p, q) : A × B` is the pair constructor)
- `P ∨ Q ↔ A + B` (sum type; `inl a : A + B` and `inr b : A + B`)
- `P → Q ↔ A → B` (function type; `λx.t : A → B`)
- `⊥ ↔ 𝟘` (empty type; no introduction rule; elimination `exfalso : 𝟘 → A`)
- `¬P ↔ P → 𝟘`
- `∀x:A. P x ↔ Π(x : A), P x` (dependent function type — ch08)
- `∃x:A. P x ↔ Σ(x : A), P x` (dependent pair type — ch08)
