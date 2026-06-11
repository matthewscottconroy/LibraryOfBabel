# Chapter 8: Dependent Type Theory

## Introduction

In simply typed lambda calculus and System F, types are fixed in advance: a function has type $A \to B$, where $A$ and $B$ are determined without reference to the function's arguments. In *dependent type theory*, this restriction is lifted: the type of the output of a function may *depend* on the value of its input.

This single extension — allowing types to depend on terms — unlocks the full expressive power of formal mathematics. Propositions become types (via Curry-Howard), and quantified statements become dependent types:
- "For all natural numbers $n$, there exists a prime greater than $n$" becomes a *type*, and a proof is a *program*.
- "The length of the concatenation of two vectors equals the sum of their lengths" becomes a *statement with a type-level proof*.
- Mathematical structures (groups, rings, categories) become *types with additional structure*, and theorems about them become *elements of those types*.

Dependent type theory is the language in which Lean 4, Agda, and Coq are written, and it is the direct predecessor of HoTT.

---

## 1. The Key Idea: Types Depending on Terms

### 1.1 Motivation: Vectors

Consider the type of *lists of natural numbers of a specific length*. In a simple type system, we might have `List Nat`, but we cannot express "a list of exactly $n$ elements" — the length $n$ would have to be a value, not a type.

With dependent types, we can define:
$$\mathsf{Vec} : \mathbb{N} \to \mathsf{Type}$$
so that $\mathsf{Vec}\, n$ is the type of lists of length exactly $n$.

Now:
- $[1, 2, 3] : \mathsf{Vec}\, 3$
- The *concatenation function* has type $\mathsf{Vec}\, m \to \mathsf{Vec}\, n \to \mathsf{Vec}\, (m + n)$
- The *head function* has type $\mathsf{Vec}\, (n+1) \to \mathbb{N}$ — it is not defined on empty lists (type-enforced!)

The type of `head` *statically* ensures it is never called on an empty vector. This is not a runtime check — it is enforced at compile time by the type system.

### 1.2 Families of Types

**Definition 8.1.** A *type family* over $A$ is a function $B : A \to \mathsf{Type}$ that assigns a type $B(a)$ to each element $a : A$. We say $B$ is a *type family* indexed by $A$.

**Examples:**
- $\mathsf{Vec} : \mathbb{N} \to \mathsf{Type}$: vectors indexed by length.
- $\mathsf{Fin} : \mathbb{N} \to \mathsf{Type}$: $\mathsf{Fin}\, n$ is the type of natural numbers less than $n$.
- $\mathsf{Even} : \mathbb{N} \to \mathsf{Prop}$: $\mathsf{Even}\, n$ is the proposition that $n$ is even (either a type with a proof, or an empty type).
- $\mathsf{Vec}\, (\text{-}) : \mathbb{N} \to \mathsf{Type}$ or $B : A \times A \to \mathsf{Type}$ are both type families over different index types.

---

## 2. Dependent Function Types (Π Types)

### 2.1 Formation and Introduction

**Definition 8.2 (Π Type).** Given a type $A$ and a type family $B : A \to \mathsf{Type}$, the *dependent function type* (or *Π type*) is:
$$\Pi_{x : A}\, B(x) \quad \text{also written} \quad (x : A) \to B(x) \quad \text{or} \quad \forall (x : A),\, B(x)$$

An element $f : \Pi_{x:A} B(x)$ is a function that, given any $a : A$, returns a term $f(a) : B(a)$.

**Typing rules:**

**Formation:**
$$\frac{\Gamma \vdash A : \mathsf{Type} \quad \Gamma, x : A \vdash B(x) : \mathsf{Type}}{\Gamma \vdash \Pi_{x:A} B(x) : \mathsf{Type}}$$

**Introduction:**
$$\frac{\Gamma, x : A \vdash t : B(x)}{\Gamma \vdash \lambda x : A,\, t : \Pi_{x:A} B(x)}$$

**Elimination:**
$$\frac{\Gamma \vdash f : \Pi_{x:A} B(x) \quad \Gamma \vdash a : A}{\Gamma \vdash f\, a : B(a)}$$

**Computation ($\beta$-rule):**
$$(\lambda x : A,\, t)\, a \equiv t[a/x] : B(a)$$

**Uniqueness ($\eta$-rule):**
$$f \equiv \lambda x,\, f\, x \quad \text{for } f : \Pi_{x:A} B(x)$$

### 2.2 Non-Dependent Special Case

When $B$ does not depend on $x$ (i.e., $B(x) = B$ for a fixed type $B$), we recover the ordinary function type:
$$\Pi_{x:A} B = A \to B$$

So Π types are a *conservative extension* of function types.

### 2.3 Examples

**Example 8.3 (Polymorphic identity).** The identity function at any type:
$$\mathsf{id} : \Pi_{A : \mathsf{Type}}\, A \to A = \lambda A,\, \lambda x : A,\, x$$
This is like `id : ∀ A, A → A` in System F, but here $A$ ranges over types *in the universe*, not just type variables.

**Example 8.4 (Head of a vector).** 
$$\mathsf{head} : \Pi_{n : \mathbb{N}}\, \mathsf{Vec}\, (n + 1) \to \mathbb{N}$$
The type system ensures `head` can only be called on non-empty vectors.

**Example 8.5 (Universal statement as Π type).** The statement "every even number is the sum of two primes" (Goldbach's conjecture) is:
$$\Pi_{n : \mathbb{N}}\, \mathsf{Even}(n) \to \mathsf{IsSumOfTwoPrimes}(n)$$
A proof would be a function that, given any $n : \mathbb{N}$ and any proof $h : \mathsf{Even}(n)$, returns a term witnessing that $n$ is a sum of two primes.

---

## 3. Dependent Pair Types (Σ Types)

### 3.1 Formation and Introduction

**Definition 8.6 (Σ Type).** Given $A : \mathsf{Type}$ and $B : A \to \mathsf{Type}$, the *dependent pair type* (or *Σ type*) is:
$$\Sigma_{x:A}\, B(x) \quad \text{also written} \quad \{x : A \mid B(x)\} \quad \text{or} \quad \exists (x : A),\, B(x)$$

An element of $\Sigma_{x:A} B(x)$ is a *dependent pair* $(a, b)$ where $a : A$ and $b : B(a)$. The type of the second component *depends* on the value of the first.

**Typing rules:**

**Formation:**
$$\frac{\Gamma \vdash A : \mathsf{Type} \quad \Gamma, x : A \vdash B(x) : \mathsf{Type}}{\Gamma \vdash \Sigma_{x:A} B(x) : \mathsf{Type}}$$

**Introduction:**
$$\frac{\Gamma \vdash a : A \quad \Gamma \vdash b : B(a)}{\Gamma \vdash (a, b) : \Sigma_{x:A} B(x)}$$

**Elimination (projections):**
$$\frac{\Gamma \vdash p : \Sigma_{x:A} B(x)}{\Gamma \vdash \mathsf{fst}\, p : A} \qquad \frac{\Gamma \vdash p : \Sigma_{x:A} B(x)}{\Gamma \vdash \mathsf{snd}\, p : B(\mathsf{fst}\, p)}$$

Note: the type of $\mathsf{snd}\, p$ depends on the *value* of $\mathsf{fst}\, p$.

**Computation:**
$$\mathsf{fst}(a, b) \equiv a \qquad \mathsf{snd}(a, b) \equiv b$$

### 3.2 Non-Dependent Special Case

When $B(x) = B$ for a fixed type, $\Sigma_{x:A} B = A \times B$ (the ordinary product type).

### 3.3 Examples

**Example 8.7 (Subset type).** $\Sigma_{n : \mathbb{N}}\, \mathsf{Even}(n)$ is the type of pairs $(n, h)$ where $n$ is a natural number and $h$ is a proof that $n$ is even. This is the *type of even natural numbers*, or the subset $\{n : \mathbb{N} \mid n \text{ is even}\}$.

**Example 8.8 (Existential statement).** A proof of $\exists n : \mathbb{N},\, \mathsf{Prime}(n) \wedge n > 100$ is an element of $\Sigma_{n:\mathbb{N}}\, (\mathsf{Prime}(n) \times n > 100)$ — a specific prime $n > 100$ together with proofs of both properties.

**Example 8.9 (Groups as Σ types).** A group structure on a type $G$ is:
$$\mathsf{Group}(G) = \Sigma_{\mu : G \to G \to G}\, \Sigma_{e : G}\, (\Pi_{a:G}\, \mu\, e\, a = a) \times (\Pi_{a:G}\, \Sigma_{b:G}\, \mu\, a\, b = e) \times \ldots$$
A group is a type $G$ together with an element of $\mathsf{Group}(G)$. The type of groups is $\Sigma_{G : \mathsf{Type}}\, \mathsf{Group}(G)$.

---

## 4. Universes

### 4.1 The Need for Universes

We have been writing $A : \mathsf{Type}$, but what type does $\mathsf{Type}$ itself have? If $\mathsf{Type} : \mathsf{Type}$ (*type-in-type*), then:
- Girard's paradox holds: this is inconsistent (analogous to Russell's paradox).
- We can derive a proof of $\bot$.

The solution (Martin-Löf): a *universe hierarchy*.

### 4.2 The Universe Hierarchy

**Definition 8.10 (Universes).** We have a sequence of *universe types*:
$$\mathsf{Type}_0 : \mathsf{Type}_1 : \mathsf{Type}_2 : \cdots$$

Each universe $\mathsf{Type}_i$ contains all "small" types at level $i$: all types $A$ such that $A : \mathsf{Type}_i$. The next universe $\mathsf{Type}_{i+1}$ contains $\mathsf{Type}_i$ as an element.

**Cumulativity:** If $A : \mathsf{Type}_i$, then $A : \mathsf{Type}_{i+1}$. (Lower universes embed into higher ones.) This means we can always "lift" a type to a larger universe.

**Russell vs. Tarski style:**
- *Russell style:* No explicit coercions; terms of type $\mathsf{Type}_i$ are literally types.
- *Tarski style:* Each universe is a "code type" with a decoding function; terms must be explicitly decoded to be used as types.

Lean 4 and Agda use Russell-style universes with automatic lifting (universe polymorphism).

### 4.3 Universe Polymorphism

A function that works at any universe level is *universe polymorphic*. In Lean 4:
```lean
def id.{u} (A : Type u) (a : A) : A := a
-- Here u is a universe level variable
```

In Agda:
```agda
id : {ℓ : Level} → {A : Set ℓ} → A → A
id x = x
```

---

## 5. Inductive Types

Dependent type theory includes a powerful mechanism for defining data types: *inductive types*. An inductive type $T$ is defined by specifying its *constructors* — the ways to build elements of $T$.

### 5.1 Natural Numbers

$$\frac{}{\vdash \mathsf{zero} : \mathbb{N}} \qquad \frac{\Gamma \vdash n : \mathbb{N}}{\Gamma \vdash \mathsf{succ}\, n : \mathbb{N}}$$

Every natural number is either $\mathsf{zero}$ or $\mathsf{succ}$ of another natural number.

**Eliminator (induction principle):**
$$\frac{\Gamma \vdash P : \mathbb{N} \to \mathsf{Type} \quad \Gamma \vdash p_0 : P(\mathsf{zero}) \quad \Gamma \vdash p_s : \Pi_{n:\mathbb{N}}\, P(n) \to P(\mathsf{succ}\, n) \quad \Gamma \vdash n : \mathbb{N}}{\Gamma \vdash \mathsf{rec}_\mathbb{N}(P, p_0, p_s, n) : P(n)}$$

**Computation rules:**
$$\mathsf{rec}_\mathbb{N}(P, p_0, p_s, \mathsf{zero}) \equiv p_0$$
$$\mathsf{rec}_\mathbb{N}(P, p_0, p_s, \mathsf{succ}\, n) \equiv p_s\, n\, (\mathsf{rec}_\mathbb{N}(P, p_0, p_s, n))$$

This is *dependent recursion*: the motive $P$ can depend on the natural number, so we can compute both data and proofs by recursion on $\mathbb{N}$.

**Example 8.11.** Addition $\mathsf{add} : \mathbb{N} \to \mathbb{N} \to \mathbb{N}$:
$$\mathsf{add}\, m\, n = \mathsf{rec}_\mathbb{N}(\lambda \_, \mathbb{N},\, n,\, \lambda k,\, \lambda r,\, \mathsf{succ}\, r,\, m)$$
This computes $m + n$ by recursion on $m$: base case $0 + n = n$, step case $(\mathsf{succ}\, k) + n = \mathsf{succ}\, (k + n)$.

**Example 8.12 (Proof by induction).** Proof that $0$ is a right identity for addition, i.e., $\Pi_{n:\mathbb{N}}\, n + 0 = n$:
$$\mathsf{rec}_\mathbb{N}(\lambda n,\, n + 0 = n,\, \mathsf{refl}_0,\, \lambda k,\, \lambda h : k + 0 = k,\, \mathsf{ap}_\mathsf{succ}\, h)$$
(Where $\mathsf{refl}_0 : 0 + 0 = 0$ and $\mathsf{ap}_\mathsf{succ}$ applies $\mathsf{succ}$ to both sides of an equation.)

### 5.2 Lists and Vectors

**Lists:**
$$\mathsf{List}(A) : \mathsf{Type} \quad \text{constructors: } [] : \mathsf{List}(A), \quad (x :: xs) : \mathsf{List}(A) \text{ for } x : A, xs : \mathsf{List}(A)$$

**Vectors (length-indexed lists):**
$$\mathsf{Vec}(A) : \mathbb{N} \to \mathsf{Type}$$
$$[] : \mathsf{Vec}(A, 0) \qquad \mathsf{cons}(h, t) : \mathsf{Vec}(A, n+1) \text{ for } h : A,\, t : \mathsf{Vec}(A, n)$$

The head and tail functions are total on vectors (never fail):
$$\mathsf{head} : \mathsf{Vec}(A, n+1) \to A \qquad \mathsf{tail} : \mathsf{Vec}(A, n+1) \to \mathsf{Vec}(A, n)$$

### 5.3 Well-Founded Trees (W-Types)

W-types provide a general framework for *well-founded* inductive types.

**Definition 8.13.** Given $A : \mathsf{Type}$ and $B : A \to \mathsf{Type}$, the *W-type* $\mathsf{W}_{x:A} B(x)$ is defined with:
- Constructor: $\mathsf{sup}(a, f) : \mathsf{W}_{x:A} B(x)$ for $a : A$ and $f : B(a) \to \mathsf{W}_{x:A} B(x)$

Intuitively: $a$ labels the root node, and $f$ picks the subtrees indexed by $B(a)$.

**Example 8.14.** Natural numbers as a W-type: $A = \{0, 1\}$ (the "shape"), $B(0) = \mathbf{0}$ (zero has no children), $B(1) = \mathbf{1}$ (successor has one child).
- $\mathsf{zero} = \mathsf{sup}(0, \mathsf{absurd})$
- $\mathsf{succ}\, n = \mathsf{sup}(1, \lambda \star, n)$

W-types show that all well-founded inductive types can be built from a single primitive. This is important for giving uniform semantics to type theory.

---

## 6. The Propositions-as-Types Principle, Revisited

With dependent types, the Curry-Howard correspondence extends fully to predicate logic:

| **Predicate Logic** | **Dependent Type Theory** |
|---|---|
| $\forall x : A,\, P(x)$ | $\Pi_{x:A} P(x)$ |
| $\exists x : A,\, P(x)$ | $\Sigma_{x:A} P(x)$ |
| Proof of $\forall x, P(x)$ | Function $f$ with $f(a) : P(a)$ for all $a$ |
| Proof of $\exists x, P(x)$ | Pair $(a, p)$ with $a : A$, $p : P(a)$ |
| Mathematical theorem | Type $T$ |
| Proof of theorem | Term $t : T$ |
| Mathematical object | Term $a : A$ |
| Mathematical structure | Type with extra structure |

The full correspondence makes dependent type theory the unified foundation for mathematics and programming:
- **Mathematics:** propositions are types, proofs are programs, mathematical structures are types
- **Computer science:** programs are proofs, types are specifications, type checking is proof checking

---

## Exercises

**8.1.** In Agda or Lean 4, define:
  - `Vec : ℕ → Type` (length-indexed vectors)
  - `head : Vec (n+1) → A`
  - `tail : Vec (n+1) → Vec n`
  - `append : Vec m → Vec n → Vec (m+n)`

**8.2.** Using the Π type, write the type of a proof that "for all $n : \mathbb{N}$, $n + 0 = n$." Then write the proof term explicitly (using the eliminator for $\mathbb{N}$).

**8.3.** Write the type of a proof of "every prime number greater than 2 is odd." You don't need to prove it; just give the type.

**8.4.** Show that $\Sigma_{n:\mathbb{N}} \mathsf{Vec}\, A\, n$ (a vector of any length) is equivalent to $\mathsf{List}\, A$.

**8.5.** The *diagonal* of a dependent pair: define a function
$$\mathsf{diag} : \Pi_{A : \mathsf{Type}}\, A \to \Sigma_{B : \mathsf{Type}}\, B$$
What is this function? What does it witness?

**8.6.** Implement the factorial function as a term using the $\mathbb{N}$-eliminator (not using `match` or `induction` tactic sugar).

**8.7.** In Lean 4, state and prove (in tactic mode):
  - `theorem add_zero : ∀ n : ℕ, n + 0 = n`
  - `theorem add_comm : ∀ m n : ℕ, m + n = n + m`
  Now inspect the proof term (using `#print add_comm`). Identify the Π types and Σ types in the term.

**8.8 (W-types).** Express binary trees as a W-type. The branching type should be: a leaf has 0 children, a node has 2 children.

**8.9 (Challenge).** The *axiom of choice* in dependent type theory is:
$$\mathsf{AC} : \Pi_{A B : \mathsf{Type}}\, \Pi_{R : A \to B \to \mathsf{Type}}\, (\Pi_{x:A}\, \Sigma_{y:B}\, R(x,y)) \to \Sigma_{f : A \to B}\, \Pi_{x:A}\, R(x, f(x))$$
Prove this as a theorem in dependent type theory (no axiom needed!). What does this say about the relationship between choice and dependent types?

---

## See Also

**In chapters/:**
- `ch06-curry-howard` — Prerequisite. The Curry-Howard correspondence for simple types is extended here to dependent types: `Π(x : A), B x` corresponds to `∀x:A. P(x)` and `Σ(x : A), B x` corresponds to `∃x:A. P(x)`.
- `ch09-mltt` — The direct continuation: MLTT adds the identity type `Id_A(a, b)` to the dependent type theory introduced here, completing the foundation for HoTT. The J eliminator is the crucial new rule.
- `ch11-categorical-logic` — The categorical semantics of dependent types: Π-types correspond to right adjoints of the pullback functor in a locally Cartesian closed category (LCCC). `Π_f B := (f* ⊣ Π_f)` is the categorical reading of the dependent product.
- `ch16-identity-types` — The identity type `Id_A(a, b)` is the third fundamental type former, after Π and Σ. Together, these three type formers generate the full structure of HoTT. The exercise 8.9 above (AC is provable) fails when propositional truncation is added (ch17): `Π_{x:A} ‖Σ_{y:B} R(x,y)‖ → ‖Σ_{f : A→B} Π_{x:A} R(x, f(x))‖` is not provable.

**In book/:**
- `book/unit-03-dependent-types/ch08-dependent-types/` — Extended narrative treatment, with emphasis on the mathematical significance of dependent types: vectors, matrices, well-founded trees, and the propositions-as-types reading of quantified statements.

**In demos/:**
- `demos/demo_dependent_types.py` — Interactive illustrations of Π-types and Σ-types. Demonstrates the difference between `A → B` and `Π(x : A), B x`, and between `A × B` and `Σ(x : A), B x`.
- `demos/demo_universes.py` — Universe levels and universe polymorphism.
- `demos/demo_w_types.py` — W-types (well-founded trees) as a general framework for inductive types.

**Key notation established here (used throughout all subsequent chapters):**
- Π-type: `Π(x : A), B x` or `(x : A) → B x`
- Σ-type: `Σ(x : A), B x` or `⟨a, b⟩ : Σ(x : A), B x`
- Universe: `A : Type₀ : Type₁ : Type₂ : ...` (Russell style) or `A : 𝒰ᵢ`
- The axiom of choice (as a theorem, without propositional truncation): `Π_{x:A} Σ_{y:B} R(x,y) → Σ_{f:A→B} Π_{x:A} R(x, f(x))`
