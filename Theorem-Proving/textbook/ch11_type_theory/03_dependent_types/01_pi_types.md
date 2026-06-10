# Dependent Types: Pi Types

> "A Π-type is a function whose return type can depend on the value of its argument. This is the type-theoretic way of saying 'for all x of type A, P(x)' — it is simultaneously a type and a logical quantifier."
> — Type theorist's summary

## The Key Insight: Types Can Depend on Values

In simple type theory, a function from $A$ to $B$ has type $A \to B$, and $B$ does not vary with the argument. The function "double" has type $\mathbb{N} \to \mathbb{N}$ — it always returns a natural number regardless of input.

But consider a function that returns a **vector of a specific length**. If we feed it the number $n$, it should return a vector with exactly $n$ elements — and the *type* of the output should reflect this: `Vec n`. Here, the return type `Vec n` depends on the *value* $n$ passed as input.

This is a **dependent type**: a type that depends on a value. The most fundamental dependent type-forming operation is the **$\Pi$-type** (Pi type), also called the **dependent function type** or **dependent product**.

## Definition

Let $A$ be a type and $B : A \to \mathsf{Type}$ be a *family of types* indexed by $A$. The **$\Pi$-type**:
$$\prod_{x : A} B(x) \quad \text{(written } \Pi\, (x : A),\, B(x) \text{ in Lean)}$$

is the type of functions $f$ such that:
- $f$ takes an argument $a : A$
- $f(a)$ has type $B(a)$ — the *return type depends on the argument*

**When $B$ is constant** ($B(x) = C$ for all $x$), the $\Pi$-type reduces to the ordinary function type $A \to C$.

**In logic** (via Curry-Howard): $\prod_{x : A} B(x)$ corresponds to the universal statement $\forall x \in A,\, B(x)$. A *proof* of $\forall x, B(x)$ is a function that, given any $x$, produces a proof that $B(x)$ holds.

## Examples in Lean 4

```lean
-- Simple function type (non-dependent): ℕ → ℕ
def double : ℕ → ℕ := fun n => 2 * n

-- Dependent function type: the return type depends on n
-- replicate n x : Vector α n (a vector of length n, filled with x)
def replicate : (n : ℕ) → α → Vector α n
  | 0,   _ => Vector.nil
  | n+1, x => Vector.cons x (replicate n x)

-- The type of replicate is: (n : ℕ) → α → Vector α n
-- This is Π (n : ℕ), α → Vector α n
-- The return type Vector α n depends on the value n

-- Another example: a function that picks the i-th element of a vector
-- The index must be bounded by the vector's length
def get : (v : Vector α n) → Fin n → α
  | Vector.cons x _, ⟨0, _⟩     => x
  | Vector.cons _ v, ⟨k+1, hk⟩  => get v ⟨k, Nat.lt_of_succ_lt_succ hk⟩

-- Fin n is the type {0, 1, ..., n-1} -- a type-level bound on the index!
-- No runtime bounds check needed: the type system guarantees safety.
```

## Pi Types as Universal Quantifiers

The Curry-Howard correspondence identifies:
$$\prod_{x : A} B(x) \;\leftrightarrow\; \forall x : A,\, B(x)$$

A term $f : \prod_{x : A} B(x)$ is simultaneously:
- A *function* that maps each $a : A$ to a term $f(a) : B(a)$
- A *proof* that for every $a : A$, the proposition $B(a)$ holds

**Example in Lean**: Proving commutativity of addition.

```lean
-- This is both a function AND a proof
theorem add_comm : ∀ (m n : ℕ), m + n = n + m := by
  intro m
  induction m with
  | zero => simp
  | succ k ih => simp [Nat.succ_add, Nat.add_succ, ih]

-- The proof term looks like:
-- fun m => Nat.rec (by simp) (fun k ih => by simp [ih]) m
-- This is a Π-type: Π (m n : ℕ), m + n = n + m
```

The type `∀ (m n : ℕ), m + n = n + m` is exactly $\prod_{m : \mathbb{N}} \prod_{n : \mathbb{N}} (m + n = n + m)$ — a Pi type where the return type is a *proposition* (which is a type, since propositions are types via Curry-Howard).

## Formation, Introduction, Elimination

Type theory is structured around three rules for each type former:

**Formation**: $\prod_{x : A} B(x)$ is a type if $A$ is a type and, for each $a : A$, $B(a)$ is a type.

**Introduction** (lambda abstraction): To construct a term of type $\prod_{x : A} B(x)$, give a body $e$ (possibly involving $x$) of type $B(x)$ for each $x$:
$$\frac{\Gamma, x : A \vdash e : B(x)}{\Gamma \vdash \lambda x.\, e : \prod_{x : A} B(x)}$$

**Elimination** (function application): Given $f : \prod_{x : A} B(x)$ and $a : A$:
$$\frac{\Gamma \vdash f : \prod_{x : A} B(x) \quad \Gamma \vdash a : A}{\Gamma \vdash f\; a : B(a)}$$

**Computation** ($\beta$-reduction): $(\lambda x.\, e)\; a \;\equiv_\beta\; e[a/x]$

These three rules — formation, introduction, elimination — with their accompanying computation rule, completely characterize the $\Pi$-type. This is the standard presentation in dependent type theory (Martin-Löf type theory, CIC).

## Vectors: A Running Example

The type `Vector α n` — a list of exactly $n$ elements of type `α` — is the canonical example of a dependent type. Its length is tracked in the type:

```lean
inductive Vector (α : Type) : ℕ → Type where
  | nil  : Vector α 0
  | cons : α → Vector α n → Vector α (n + 1)

-- Type-safe append: lengths add
def append : Vector α m → Vector α n → Vector α (m + n)
  | Vector.nil,      v => v
  | Vector.cons x u, v => Vector.cons x (append u v)

-- The type (m n : ℕ) → Vector α m → Vector α n → Vector α (m + n)
-- is a Pi type where the return type depends on m and n

-- Type-safe head: only callable on non-empty vectors
def head : Vector α (n + 1) → α
  | Vector.cons x _ => x

-- head is total: Lean won't let you call it on Vector α 0
-- No runtime error possible!
```

With dependent types, many runtime errors become *type errors* — caught at compile time. This is one of the main practical benefits of dependent type systems.

## Connection to Formal Verification

Dependent types are the foundation of all major proof assistants:

**Lean 4** uses the Calculus of Inductive Constructions (CIC), which is essentially the simply-typed lambda calculus extended with:
- $\Pi$-types (dependent function types)
- $\Sigma$-types (dependent pairs)
- Inductive types
- Universe hierarchy

**Coq** uses a very similar CIC. The type-checking algorithm in both systems is essentially: normalize types, check equality of normal forms. This is decidable in CIC (unlike in some more powerful systems).

**Agda** uses a similar but slightly different dependent type theory, without an automatic elimination of pattern matching — forcing the user to supply more structure.

## Why Dependent Types Matter for Mathematics

In dependent type theory, the distinction between "writing a program" and "proving a theorem" dissolves:
- A function `sort : List ℕ → List ℕ` just sorts lists
- A function `sort : (l : List ℕ) → {l' : List ℕ // Sorted l' ∧ Permutation l l'}` *sorts the list AND proves the output is sorted and a permutation of the input*

The second version packs the correctness proof into the type. Writing the function forces you to prove it correct simultaneously — a proof cannot be faked (Lean's type checker would reject it).

This is why Lean, Coq, and Agda are used for **formal verification** — certifying software and mathematical proofs to a level of rigor that human review cannot match.

## Exercises
See [problems/ch11_type_theory/02_dependent_type_exercises.md](../../../problems/ch11_type_theory/02_dependent_type_exercises.md)
