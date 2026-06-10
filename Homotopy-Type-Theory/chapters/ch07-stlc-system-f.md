# Chapter 7: Simply Typed Lambda Calculus and System F

## Introduction

The lambda calculus, invented by Alonzo Church in the 1930s, is perhaps the simplest Turing-complete programming language. It has three constructs: variables, function formation, and function application. Everything else — numbers, booleans, lists, recursion — can be *encoded*.

Adding types to the lambda calculus imposes discipline: not every term can be applied to every other term; types constrain which programs are well-formed. This discipline has two effects. First, it rules out many "bad" programs (infinite loops, type errors). Second — and surprisingly — it makes the remaining programs *computationally better behaved*: every well-typed term terminates.

This chapter develops two typed systems:
1. **Simply Typed Lambda Calculus (STLC):** The foundational system. Types are formed from base types and function types. Corresponds to intuitionistic propositional logic (Chapter 6).
2. **System F:** Adds universal quantification over types (polymorphism). Corresponds to second-order propositional logic. Contains encodings of all basic data types.

Both are *precursors* to the dependent types of Chapter 8, which subsume and extend them.

---

## 1. Untyped Lambda Calculus (Background)

Before adding types, we briefly recall the untyped lambda calculus, to see what types constrain.

**Definition 7.1 (Untyped Lambda Terms).** The set $\Lambda$ of lambda terms is defined inductively:
- Every variable $x$ is a term.
- If $t \in \Lambda$ and $x$ is a variable, then $\lambda x,\, t$ is a term.
- If $t, s \in \Lambda$, then $t\, s$ is a term.

Application is left-associative: $f\, a\, b = (f\, a)\, b$.

**Definition 7.2 (Substitution and Beta Reduction).**
- $t[s/x]$: the term obtained by substituting $s$ for all free occurrences of $x$ in $t$ (with appropriate renaming of bound variables to avoid capture).
- **$\beta$-reduction:** $(\lambda x,\, t)\, s \to_\beta t[s/x]$.

**The problem with untyped lambda calculus:**
- Self-application $(\lambda x,\, x\, x)\, (\lambda x,\, x\, x) \to_\beta (\lambda x,\, x\, x)\, (\lambda x,\, x\, x) \to_\beta \cdots$ diverges.
- The Y combinator $Y = \lambda f,\, (\lambda x,\, f\, (x\, x))\, (\lambda x,\, f\, (x\, x))$ implements recursion — but $Y\, f$ does not terminate for most $f$.
- There is no notion of "type error": $\mathsf{true}\, \mathsf{true}$ is a well-formed term, though it makes no sense.

Types eliminate all of these pathologies.

---

## 2. Simply Typed Lambda Calculus (STLC)

### 2.1 Types

**Definition 7.3 (Simple Types).** Fix a set of *base types* $\mathcal{B} = \{o, \iota, \ldots\}$. Simple types are:
$$A, B ::= \alpha \mid A \to B \mid A \times B \mid A + B \mid \mathbf{1} \mid \mathbf{0}$$
where $\alpha \in \mathcal{B}$.

The function type $A \to B$ is right-associative: $A \to B \to C = A \to (B \to C)$.

### 2.2 Terms and Typing Contexts

A *typing context* $\Gamma = x_1 : A_1, \ldots, x_n : A_n$ declares types for a finite set of variables. The typing judgment $\Gamma \vdash t : A$ says "term $t$ has type $A$ in context $\Gamma$."

**Typing rules:**

**Variables:**
$$\frac{x : A \in \Gamma}{\Gamma \vdash x : A}$$

**Function types:**
$$\frac{\Gamma, x : A \vdash t : B}{\Gamma \vdash \lambda x : A,\, t : A \to B} \qquad \frac{\Gamma \vdash t : A \to B \quad \Gamma \vdash s : A}{\Gamma \vdash t\, s : B}$$

**Products:**
$$\frac{\Gamma \vdash t : A \quad \Gamma \vdash s : B}{\Gamma \vdash (t, s) : A \times B} \quad \frac{\Gamma \vdash t : A \times B}{\Gamma \vdash \mathsf{fst}\, t : A} \quad \frac{\Gamma \vdash t : A \times B}{\Gamma \vdash \mathsf{snd}\, t : B}$$

**Sums:**
$$\frac{\Gamma \vdash t : A}{\Gamma \vdash \mathsf{inl}\, t : A + B} \quad \frac{\Gamma \vdash s : B}{\Gamma \vdash \mathsf{inr}\, s : A + B}$$

$$\frac{\Gamma \vdash e : A + B \quad \Gamma, x : A \vdash t : C \quad \Gamma, y : B \vdash s : C}{\Gamma \vdash \mathsf{case}\, e\, \mathsf{of}\, \mathsf{inl}(x) \Rightarrow t \mid \mathsf{inr}(y) \Rightarrow s : C}$$

**Unit and Empty:**
$$\frac{}{\Gamma \vdash \star : \mathbf{1}} \qquad \frac{\Gamma \vdash t : \mathbf{0}}{\Gamma \vdash \mathsf{absurd}\, t : A}$$

### 2.3 Computation Rules

**$\beta$-reduction** (the primary computation rule):
- $(\lambda x : A,\, t)\, s \to_\beta t[s/x]$
- $\mathsf{fst}(t, s) \to_\beta t$
- $\mathsf{snd}(t, s) \to_\beta s$
- $\mathsf{case}\, (\mathsf{inl}\, v)\, \mathsf{of}\, \mathsf{inl}(x) \Rightarrow t \mid \mathsf{inr}(y) \Rightarrow s \to_\beta t[v/x]$
- $\mathsf{case}\, (\mathsf{inr}\, w)\, \mathsf{of}\, \mathsf{inl}(x) \Rightarrow t \mid \mathsf{inr}(y) \Rightarrow s \to_\beta s[w/y]$

**$\eta$-expansion** (the extensionality rule):
- $t : A \to B$ is $\eta$-equivalent to $\lambda x : A,\, t\, x$ (for $x$ fresh)

### 2.4 Type Safety

**Definition 7.4.** A term $t$ is a *value* if it is a lambda abstraction, a pair of values, an injection of a value, or the unit $\star$.

**Theorem 7.5 (Progress).** If $\vdash t : A$ (closed, well-typed), then either $t$ is a value or there exists $t'$ with $t \to_\beta t'$.

**Theorem 7.6 (Preservation).** If $\Gamma \vdash t : A$ and $t \to_\beta t'$, then $\Gamma \vdash t' : A$.

*Proof of Preservation.* By induction on the derivation of $t \to_\beta t'$. The key case is $(\lambda x, t)\, s \to t[s/x]$: we need to show that if $\Gamma, x : A \vdash t : B$ and $\Gamma \vdash s : A$, then $\Gamma \vdash t[s/x] : B$. This follows from the *substitution lemma*: substitution preserves typing. $\square$

### 2.5 Strong Normalization

**Theorem 7.7 (Strong Normalization).** Every well-typed term in STLC is strongly normalizing: there are no infinite $\beta$-reduction sequences.

*Proof (sketch, via Logical Relations).* Define, for each type $A$, a set $\mathcal{R}(A)$ of *reducible* terms:
- $\mathcal{R}(\alpha)$: closed terms of base type that are strongly normalizing.
- $\mathcal{R}(A \to B)$: closed terms $t : A \to B$ such that for all $s \in \mathcal{R}(A)$, we have $t\, s \in \mathcal{R}(B)$.
- $\mathcal{R}(A \times B)$: terms $t$ such that $\mathsf{fst}\, t \in \mathcal{R}(A)$ and $\mathsf{snd}\, t \in \mathcal{R}(B)$.

Show by induction on typing derivations that every well-typed closed term is reducible. Show that every reducible term is strongly normalizing. Combining: every well-typed term normalizes. $\square$

**Consequence:** The simply typed lambda calculus is *not* Turing-complete. The function $n \mapsto n!$ is typeable; the Ackermann function is *not* typeable in STLC. There is no self-referential term (since $\lambda x,\, x\, x$ cannot be typed — it would require $A \to B = A$, which has no finite solution).

---

## 3. Church Encodings

Despite not being Turing-complete, STLC can encode many data types through *Church encodings*. This shows that the type structure is rich.

### 3.1 Church Booleans

$$\mathsf{Bool} = \forall \alpha,\, \alpha \to \alpha \to \alpha$$
Wait — this uses a type quantifier, which is System F. In pure STLC, fix a base type:
$$\mathsf{true} : A \to A \to A = \lambda x,\, \lambda y,\, x$$
$$\mathsf{false} : A \to A \to A = \lambda x,\, \lambda y,\, y$$
$$\mathsf{if}\, b\, t\, f = b\, t\, f$$

### 3.2 Church Numerals (in untyped or System F)

The Church numeral for $n$ is the function that applies $f$ to $x$ exactly $n$ times:
$$\underline{0} = \lambda f,\, \lambda x,\, x$$
$$\underline{1} = \lambda f,\, \lambda x,\, f\, x$$
$$\underline{n} = \lambda f,\, \lambda x,\, f^n(x)$$

Addition: $\mathsf{plus}\, m\, n = \lambda f,\, \lambda x,\, m\, f\, (n\, f\, x)$.
Multiplication: $\mathsf{mult}\, m\, n = \lambda f,\, m\, (n\, f)$.

In System F, Church numerals have type $\Pi \alpha,\, (\alpha \to \alpha) \to \alpha \to \alpha$.

---

## 4. System F: Polymorphism

System F (Girard 1972, Reynolds 1974) extends STLC with *quantification over types*.

### 4.1 Type System F

**Types:**
$$A, B ::= \alpha \mid A \to B \mid \forall \alpha,\, A$$
(We can also add products and sums.)

**Terms:**
- Type abstraction: $\Lambda \alpha,\, t$ (abstract over a type variable $\alpha$)
- Type application: $t\, [A]$ (instantiate a type-polymorphic term with type $A$)

**Typing rules:**
$$\frac{\Gamma \vdash t : A \quad \alpha \notin \text{FTV}(\Gamma)}{\Gamma \vdash \Lambda \alpha,\, t : \forall \alpha,\, A} \qquad \frac{\Gamma \vdash t : \forall \alpha,\, A}{\Gamma \vdash t\, [B] : A[\alpha := B]}$$

**Reduction:** $(\Lambda \alpha,\, t)\, [B] \to_\beta t[\alpha := B]$ (*type-level beta reduction*).

### 4.2 Polymorphic Types as Second-Order Logic

The correspondence (Curry-Howard) extends: System F corresponds to second-order intuitionistic propositional logic (2IPC), where $\forall \alpha, A$ corresponds to "for all propositions $\alpha$, $A$ holds."

| **2IPC** | **System F** |
|---|---|
| $\forall \alpha,\, A$ | $\forall \alpha,\, A$ |
| Universal instantiation | Type application $t\, [B]$ |
| Universal introduction | Type abstraction $\Lambda \alpha,\, t$ |

**Example 7.8.** The *identity* at any type:
$$\mathsf{id} = \Lambda \alpha,\, \lambda x : \alpha,\, x : \forall \alpha,\, \alpha \to \alpha$$

The type $\forall \alpha,\, \alpha \to \alpha$ is the System F version of $\forall P, P \to P$ in second-order logic.

**Example 7.9.** Church booleans in System F:
$$\mathsf{Bool} = \forall \alpha,\, \alpha \to \alpha \to \alpha$$
$$\mathsf{true} = \Lambda \alpha,\, \lambda x : \alpha,\, \lambda y : \alpha,\, x : \mathsf{Bool}$$
$$\mathsf{false} = \Lambda \alpha,\, \lambda x : \alpha,\, \lambda y : \alpha,\, y : \mathsf{Bool}$$

**Example 7.10.** Church naturals:
$$\mathsf{Nat} = \forall \alpha,\, (\alpha \to \alpha) \to \alpha \to \alpha$$
$$\underline{n} = \Lambda \alpha,\, \lambda f : \alpha \to \alpha,\, \lambda x : \alpha,\, f^n(x) : \mathsf{Nat}$$

From $\mathsf{Nat}$, we can define addition, multiplication, and even predecessor — making System F Turing-complete (at the level of normalizable functions, which includes all total recursive functions and more).

### 4.3 Strong Normalization for System F

**Theorem 7.11 (Girard).** System F is strongly normalizing.

The proof is significantly harder than for STLC: the logical relations argument breaks down at function types because the domain of $\mathcal{R}(\forall \alpha, A)$ requires knowing $\mathcal{R}(A[\alpha := B])$ for all $B$ at once. Girard's proof introduces *candidates of reducibility* (or *saturated sets*) to handle this.

**Consequence:** System F is not Turing-complete — despite encoding all primitive recursive functions, it cannot encode every recursive function. (The Halting Problem cannot be solved in System F.)

### 4.4 Parametricity

**Theorem 7.12 (Parametricity / Reynolds 1983).** If $f : \forall \alpha,\, A$ in System F, then $f$ is *parametric*: it behaves "the same way" at every type instantiation.

This is formalized by *relational parametricity*: for any relation $R$ between types $B_1$ and $B_2$, the instantiation $f\, [B_1]$ and $f\, [B_2]$ are related by the *lifted relation* $A[R/\alpha]$.

**Free theorems (Wadler 1989):** Parametricity gives theorems "for free" — just from the type of a polymorphic function:
- A term of type $\forall \alpha,\, \alpha \to \alpha$ must be the identity $\mathsf{id}$.
- A term of type $\forall \alpha,\, \alpha \to \alpha \to \alpha$ must be either $\mathsf{true}$ or $\mathsf{false}$ (the two projections).
- A term of type $\forall \alpha,\, \mathsf{List}\, \alpha \to \mathsf{List}\, \alpha$ must be a function that only rearranges elements (it cannot inspect their values).

---

## 5. System $F_\omega$: Type Operators

System $F_\omega$ extends System F with *type operators* — functions from types to types.

**Kinds:** Types in $F_\omega$ are classified by *kinds*:
$$\kappa ::= \star \mid \kappa \to \kappa$$
where $\star$ is the kind of proper types and $\kappa \to \kappa'$ is the kind of type-level functions.

**Type-level lambda abstraction:** $\lambda \alpha : \kappa,\, A$ has kind $\kappa \to \kappa'$ if $A : \kappa'$.

**Example 7.13.** The functor $\mathsf{Maybe} = \lambda \alpha : \star,\, \mathbf{1} + \alpha$ has kind $\star \to \star$. $\mathsf{Maybe}\, \mathsf{Nat} = \mathbf{1} + \mathsf{Nat}$ (option type for naturals).

System $F_\omega$ is the core of Haskell's type system (generalized algebraic data types, type classes). The *Calculus of Constructions* (CoC) combines $F_\omega$ with dependent types — it is the foundation of Coq/Rocq.

---

## Exercises

**7.1.** Type-check the following terms in STLC. For each, determine the most general type.
  - $\lambda f : A \to B,\, \lambda g : B \to C,\, \lambda x : A,\, g\, (f\, x)$
  - $\lambda p : A \times B,\, (\mathsf{snd}\, p, \mathsf{fst}\, p)$
  - $\lambda e : A + B,\, \mathsf{case}\, e\, \mathsf{of}\, \mathsf{inl}(x) \Rightarrow \mathsf{inr}(x) \mid \mathsf{inr}(y) \Rightarrow \mathsf{inl}(y)$

**7.2.** Reduce the following to normal form, showing each step:
  - $(\lambda x : A \to A,\, \lambda y : A,\, x\, (x\, y))\, (\lambda z : A,\, z)$
  - $\mathsf{fst}\, ((\lambda x : \mathbb{N},\, (x, x + 1))\, 3)$

**7.3.** Show that $\lambda x,\, x\, x$ cannot be given a type in STLC. (*Hint:* What would the type of $x$ have to be? Show this leads to an infinite type equation.)

**7.4.** In System F, write terms for:
  - Church boolean $\mathsf{and} : \mathsf{Bool} \to \mathsf{Bool} \to \mathsf{Bool}$
  - Church natural $\mathsf{plus} : \mathsf{Nat} \to \mathsf{Nat} \to \mathsf{Nat}$
  - Church natural $\mathsf{iszero} : \mathsf{Nat} \to \mathsf{Bool}$

**7.5.** What free theorem does the type $\forall \alpha,\, \alpha \to \alpha \to \alpha$ give you? Write down precisely what parametricity says about a term of this type.

**7.6.** In Agda or Lean 4, implement the following:
  - A polymorphic identity function
  - A polymorphic function `swap : ∀ {A B : Type}, A × B → B × A`
  - A polymorphic function `curry : ∀ {A B C : Type}, (A × B → C) → A → B → C`

**7.7 (Challenge).** The *Girard-Reynolds isomorphism:* System F types classify second-order formulas. In particular, $\mathsf{Nat} = \forall \alpha,\, (\alpha \to \alpha) \to \alpha \to \alpha$ corresponds to the Peano second-order axiom defining $\mathbb{N}$. Show that Church naturals satisfy the Peano axioms (zero, successor, induction) at the type level in System F.
