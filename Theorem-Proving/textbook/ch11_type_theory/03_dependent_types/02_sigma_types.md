# Dependent Types: Sigma Types

> "A Σ-type is a pair whose second component's type depends on the first component's value. This is simultaneously an existential quantifier and a dependent pair type."
> — Type theorist's summary

## Paired with Dependency

In ordinary type theory, the product type $A \times B$ contains pairs $(a, b)$ where $a : A$ and $b : B$ — the types of both components are fixed in advance.

A **$\Sigma$-type** (Sigma type, also called a **dependent pair type** or **dependent sum**) generalizes this: the type of the second component can *depend on the value of the first*.

$$\sum_{x : A} B(x) \quad \text{(written } \Sigma\, (x : A),\, B(x) \text{ in Lean)}$$

A term of this type is a pair $(a, b)$ where:
- $a : A$
- $b : B(a)$ — the type of $b$ depends on the specific value $a$

## As Existential Quantifiers

Via the Curry-Howard correspondence:
$$\sum_{x : A} B(x) \;\leftrightarrow\; \exists x : A,\; B(x)$$

A term $(a, b) : \sum_{x : A} B(x)$ is:
- A *pair* (computationally): first component is $a : A$, second is evidence $b : B(a)$
- A *proof* (logically): the witness is $a$, the proof that $a$ satisfies the predicate is $b$

This is the **witness-evidence** interpretation of existential statements: to prove $\exists x, P(x)$, you must exhibit a specific witness and prove it satisfies $P$.

## Examples in Lean 4

```lean
-- Basic existential: "there exists an even number"
example : ∃ n : ℕ, n % 2 = 0 := ⟨4, by decide⟩
-- The pair: witness n = 4, evidence 4 % 2 = 0

-- Σ-type for a bounded natural number (n < 10)
def bounded_nat := Σ n : ℕ, n < 10
-- A term: ⟨7, by decide⟩ : Σ n : ℕ, n < 10

-- Subtype: a set described by a predicate
-- { n : ℕ // n.Prime } -- natural numbers that are prime
def primes := {n : ℕ // n.Prime}
example : primes := ⟨7, by decide⟩

-- Vector with dependent length (Σ-type view)
-- A "dynamically sized" list with its length stored:
def SizedList (α : Type) := Σ n : ℕ, Vector α n
def example_list : SizedList ℕ := ⟨3, #[1, 2, 3]⟩

-- The type of primes with a proof of primality
def prime_example : Σ n : ℕ, Nat.Prime n := ⟨17, by decide⟩
```

## Formation, Introduction, Elimination

**Formation**: $\sum_{x : A} B(x)$ is a type if $A$ is a type and $B : A \to \mathsf{Type}$ is a family.

**Introduction** (pairing): To construct a term, provide the witness and the evidence:
$$\frac{\Gamma \vdash a : A \qquad \Gamma \vdash b : B(a)}{\Gamma \vdash (a, b) : \sum_{x : A} B(x)}$$

**Elimination** (projection): Given $p : \sum_{x : A} B(x)$:
$$\text{fst}(p) : A \qquad \text{snd}(p) : B(\text{fst}(p))$$

The typing of `snd(p)` is itself dependent: its type $B(\text{fst}(p))$ depends on the runtime value of `fst(p)`.

## When $B$ is Constant: Ordinary Products

If $B(x) = C$ is constant (does not depend on $x$), then $\sum_{x : A} B(x) = A \times C$ — ordinary cartesian product.

So Σ-types generalize ordinary product types, just as Π-types generalize ordinary function types.

## Specifications as Types

Σ-types let us encode **specifications** directly in types:

```lean
-- A sorting function with a correctness guarantee baked in:
def sort_spec (l : List ℕ) : Σ l' : List ℕ, Sorted l' ∧ l' ~ l :=
  -- Must return: a list l', plus a proof that l' is sorted AND a permutation of l
  sorry

-- A prime factorization function with a uniqueness guarantee:
def factorize (n : ℕ) (hn : n ≥ 2) :
    Σ factors : Multiset ℕ, (∀ p ∈ factors, Nat.Prime p) ∧ factors.prod = n :=
  sorry
```

Writing the function forces you to prove its specification simultaneously — the type is the contract, and Lean's type-checker enforces it.

## Subtypes: The Common Special Case

A **subtype** $\{x : A \mid P(x)\}$ (written `{x : A // P x}` in Lean) is a Σ-type where the second component is a proposition (proof):
$$\{x : A \mid P(x)\} = \sum_{x : A} P(x)$$

Terms of this type are pairs (element, proof), but since the proof is a proposition (with at most one "value" up to propositional equality), the terms look just like elements of $A$ satisfying $P$.

```lean
-- Even numbers as a subtype
def EvenNat := {n : ℕ // n % 2 = 0}

def double_even (n : ℕ) : EvenNat := ⟨2 * n, by omega⟩

-- Accessing the underlying value:
def val_example : EvenNat := ⟨6, by decide⟩
#eval val_example.val  -- 6
```

## Exercises
See [problems/ch11_type_theory/02_dependent_type_exercises.md](../../../problems/ch11_type_theory/02_dependent_type_exercises.md)
