# 5.1 Inductive Types: Natural Numbers and Recursors

## What Inductive Types Are

So far, we've introduced Π types and Σ types — ways to form new types from existing ones. But where do the basic types come from? What are $\mathbb{N}$, $\mathbb{B}$ (booleans), $\mathbf{0}$ (the empty type), $\mathbf{1}$ (the unit type)?

These are *inductive types*: types defined by specifying their *constructors* — the ways to build elements from simpler pieces — and their *elimination principle* — the way to consume or analyze elements.

The principle of inductive types is: tell me how to build elements, and I'll tell you how to use them. The elimination principle says exactly what you need to prove or compute for each constructor case, ensuring that your function or proof is well-defined on all possible inputs.

This is a profound design choice. In ZFC set theory, $\mathbb{N}$ is defined as a specific set (von Neumann ordinals or Dedekind-Peano axioms impose conditions from outside). In type theory, $\mathbb{N}$ is defined by its *interface* — its constructors and eliminators. There's no ambiguity about what $\mathbb{N}$ is, because the type is fully characterized by how you interact with it.

## Natural Numbers: Constructors

The natural numbers $\mathbb{N}$ have two constructors:

$$\mathsf{zero} : \mathbb{N}$$
$$\mathsf{succ} : \mathbb{N} \to \mathbb{N}$$

That's it. Every natural number is either $\mathsf{zero}$ or the successor of some other natural number:
$$0 = \mathsf{zero}$$
$$1 = \mathsf{succ}(\mathsf{zero})$$
$$2 = \mathsf{succ}(\mathsf{succ}(\mathsf{zero}))$$
$$\vdots$$

The natural numbers are the *smallest* type with these two constructors — there's nothing else in $\mathbb{N}$. This "smallest type" condition is enforced by the elimination principle.

## The Non-Dependent Recursor

The simplest way to define a function out of $\mathbb{N}$ (when you don't need the return type to depend on the natural number) is by *primitive recursion*:

$$\mathsf{rec}_{\mathbb{N}} : C \to (C \to C) \to \mathbb{N} \to C$$

Given:
- A base case $c_0 : C$ (the value at $\mathsf{zero}$)
- A step function $c_s : C \to C$ (how to go from the value at $n$ to the value at $\mathsf{succ}(n)$)
- A natural number $n : \mathbb{N}$

The recursor returns an element of $C$:
$$\mathsf{rec}_{\mathbb{N}}\, c_0\, c_s\, \mathsf{zero} \equiv c_0$$
$$\mathsf{rec}_{\mathbb{N}}\, c_0\, c_s\, (\mathsf{succ}(n)) \equiv c_s\, (\mathsf{rec}_{\mathbb{N}}\, c_0\, c_s\, n)$$

**Example: addition.** Addition $m + n$ iterates the successor function $m$ times starting from $n$:
$$m + n = \mathsf{rec}_{\mathbb{N}}\, n\, \mathsf{succ}\, m$$
Check:
- $0 + n = \mathsf{rec}_{\mathbb{N}}\, n\, \mathsf{succ}\, \mathsf{zero} \equiv n$ ✓
- $\mathsf{succ}(m) + n = \mathsf{rec}_{\mathbb{N}}\, n\, \mathsf{succ}\, (\mathsf{succ}(m)) \equiv \mathsf{succ}(\mathsf{rec}_{\mathbb{N}}\, n\, \mathsf{succ}\, m) = \mathsf{succ}(m + n)$ ✓

**Example: multiplication.** $m \times n = \mathsf{rec}_{\mathbb{N}}\, 0\, (\lambda k. k + n)\, m$. Check:
- $0 \times n = 0$ ✓
- $\mathsf{succ}(m) \times n = (m \times n) + n$ ✓

**Example: exponentiation.** $m^n = \mathsf{rec}_{\mathbb{N}}\, 1\, (\lambda k. k \times m)\, n$. Check:
- $m^0 = 1$ ✓
- $m^{\mathsf{succ}(n)} = m^n \times m$ ✓

The recursor is the uniform way to define all primitive recursive functions on $\mathbb{N}$.

## The Dependent Recursor (Induction Principle)

The full power of inductive types comes from the *dependent* recursor, which allows the return type to depend on the natural number. This is the formal statement of mathematical induction:

$$\mathsf{ind}_{\mathbb{N}} : \prod_{P : \mathbb{N} \to \mathsf{Type}} P(0) \to \left(\prod_{n:\mathbb{N}} P(n) \to P(\mathsf{succ}(n))\right) \to \prod_{n:\mathbb{N}} P(n)$$

Given:
- A *motive* $P : \mathbb{N} \to \mathsf{Type}$ (the property/type family you're producing)
- A base case $p_0 : P(0)$ (proof/element for the base case)
- An inductive step $p_s : \prod_{n:\mathbb{N}} P(n) \to P(\mathsf{succ}(n))$ (given a proof at $n$, produce a proof at $\mathsf{succ}(n)$)

This gives $\prod_{n:\mathbb{N}} P(n)$: a proof for every natural number.

The computation rules:
$$\mathsf{ind}_{\mathbb{N}}\, P\, p_0\, p_s\, \mathsf{zero} \equiv p_0$$
$$\mathsf{ind}_{\mathbb{N}}\, P\, p_0\, p_s\, (\mathsf{succ}(n)) \equiv p_s\, n\, (\mathsf{ind}_{\mathbb{N}}\, P\, p_0\, p_s\, n)$$

**This is mathematical induction.** The motive $P$ is the induction hypothesis; $p_0$ is the base case; $p_s$ is the inductive step. The type-theoretic presentation makes the structure explicit:
- $p_s$ takes two arguments: the number $n$ itself, and the inductive hypothesis $P(n)$
- The result is a proof of $P(\mathsf{succ}(n))$, which may use both $n$ and the inductive hypothesis

When $P$ is a proposition (a type with at most one element), this is exactly the standard induction principle. When $P$ is a type family producing non-propositional types (like computing a vector of length $n$), it's dependent recursion over a family.

## Seeing Induction as a Type

The power of the Curry-Howard perspective: the induction principle for $\mathbb{N}$ is itself a term with a specific type. The type:

$$\prod_{P : \mathbb{N} \to \mathsf{Type}} P(0) \to \left(\prod_{n:\mathbb{N}} P(n) \to P(\mathsf{succ}(n))\right) \to \prod_{n:\mathbb{N}} P(n)$$

is precisely the Peano induction axiom, stated as a type. And $\mathsf{ind}_{\mathbb{N}}$ is a *proof term* — a program of this type.

In a proof assistant, when you write a proof by induction on $n : \mathbb{N}$, you're constructing a term of this type. The proof checker verifies that your construction is type-correct.

## Example: Proving $n + 0 = n$

Let's use the induction principle to prove that $n + 0 = n$ for all $n : \mathbb{N}$.

The motive: $P(n) = (n + 0 = n)$.

Base case: $P(0) = (0 + 0 = 0)$. By the definition of addition, $0 + 0 = 0$ by the base case rule. So $p_0 = \mathsf{refl}_0 : 0 + 0 = 0$.

Inductive step: Assume $p_n : n + 0 = n$ (the induction hypothesis). We need $P(\mathsf{succ}(n)) = (\mathsf{succ}(n) + 0 = \mathsf{succ}(n))$.

By the definition of addition: $\mathsf{succ}(n) + 0 = \mathsf{succ}(n + 0)$.
By the induction hypothesis $p_n$: $n + 0 = n$.
So $\mathsf{succ}(n + 0) = \mathsf{succ}(n)$ by applying $\mathsf{succ}$ to both sides.
Combining: $\mathsf{succ}(n) + 0 = \mathsf{succ}(n)$.

In type theory, this proof term is:
$$\mathsf{ind}_{\mathbb{N}}\, (n \mapsto n + 0 = n)\, \mathsf{refl}_0\, (n \mapsto p_n \mapsto \mathsf{ap}_\mathsf{succ}\, p_n)$$

where $\mathsf{ap}_\mathsf{succ} : n + 0 = n \to \mathsf{succ}(n + 0) = \mathsf{succ}(n)$ applies the function $\mathsf{succ}$ to both sides of an equality.

## Why Not Just Define Recursion by Equations?

In Haskell or ML, you define functions on natural numbers by equations:
```haskell
add 0 n = n
add (succ m) n = succ (add m n)
```

Why bother with the recursor? Because pattern matching equations in Haskell hide a termination problem: Haskell allows potentially non-terminating recursive definitions. The recursor guarantees that your recursion is *structural* — it always decreases, and so always terminates.

In Agda and Lean 4, pattern matching is syntactic sugar for the recursor (plus a termination check). You can write:
```agda
add : ℕ → ℕ → ℕ
add zero n = n
add (suc m) n = suc (add m n)
```
and Agda translates this to the recursor internally. The termination checker ensures that every recursive call decreases in a well-founded ordering.

## Universes and the Recursor

The recursor $\mathsf{ind}_{\mathbb{N}}$ takes the motive $P : \mathbb{N} \to \mathsf{Type}$. But which universe does $\mathsf{Type}$ live in? This is where universe polymorphism matters:

$$\mathsf{ind}_{\mathbb{N}}^{\ell} : \prod_{P : \mathbb{N} \to \mathsf{Type}_\ell} P(0) \to \left(\prod_{n:\mathbb{N}} P(n) \to P(\mathsf{succ}(n))\right) \to \prod_{n:\mathbb{N}} P(n)$$

There's a separate recursor for each universe level $\ell$. Universe polymorphism lets you state this uniformly, and proof assistants handle the level inference automatically.

## The Relationship to Peano Arithmetic

The natural numbers in type theory satisfy the Peano axioms as theorems:

1. $0 : \mathbb{N}$ — zero is a natural number ✓ (by $\mathsf{zero}$)
2. $\mathsf{succ} : \mathbb{N} \to \mathbb{N}$ — every natural number has a successor ✓ (by $\mathsf{succ}$)
3. $\mathsf{succ}(n) \neq 0$ for all $n : \mathbb{N}$ — provable from the recursor (define a function that returns $\mathbf{0}$ on $\mathsf{zero}$ and $\mathbf{1}$ on $\mathsf{succ}(\_)$)
4. $\mathsf{succ}(m) = \mathsf{succ}(n) \to m = n$ — injectivity of $\mathsf{succ}$, provable from the identity type and recursor
5. Induction: $\mathsf{ind}_{\mathbb{N}}$ ✓ (built in)

So the type $\mathbb{N}$ defined by constructors $\mathsf{zero}$ and $\mathsf{succ}$ with the induction principle satisfies all five Peano axioms. This isn't a coincidence — the inductive definition *is* the Peano axiomatization in type theory.

## Other Inductive Types: Boolean and Unit

Booleans are an inductive type with two constructors and no recursive components:

$$\mathsf{true} : \mathbb{B}$$
$$\mathsf{false} : \mathbb{B}$$

Recursor:
$$\mathsf{rec}_{\mathbb{B}} : \prod_{P : \mathbb{B} \to \mathsf{Type}} P(\mathsf{true}) \to P(\mathsf{false}) \to \prod_{b:\mathbb{B}} P(b)$$

This is just a two-case match: given values for the $\mathsf{true}$ case and the $\mathsf{false}$ case, produce a value for any $b$.

The unit type $\mathbf{1}$ has one constructor:

$$\mathsf{tt} : \mathbf{1}$$

Recursor: $\mathsf{rec}_\mathbf{1} : \prod_{P : \mathbf{1} \to \mathsf{Type}} P(\mathsf{tt}) \to \prod_{u:\mathbf{1}} P(u)$.

The empty type $\mathbf{0}$ has *no* constructors:

Recursor: $\mathsf{rec}_\mathbf{0} : \prod_{P : \mathbf{0} \to \mathsf{Type}} \prod_{x:\mathbf{0}} P(x)$.

This is the principle of explosion (ex falso quodlibet): from a proof of $\mathbf{0}$, you can prove anything. The recursor for $\mathbf{0}$ requires no cases — there are no constructors to handle. It's sometimes called $\mathsf{absurd}$.

## Strong Normalization for Inductive Types

Adding inductive types to Π and Σ types preserves strong normalization, provided:
1. The recursors compute on constructors as specified by the $\beta$-rules
2. The motives in recursors live in the same universe as the type being eliminated (no universe inconsistency)
3. The inductive definitions are *positive* (the type being defined doesn't appear in a negative position in its own constructors — see below)

**Positivity condition.** Consider a bad "inductive" type:

$$\mathsf{Bad} \text{ with constructor } \mathsf{bad} : (\mathsf{Bad} \to \mathsf{Bad}) \to \mathsf{Bad}$$

This is a self-referential type where the constructor's argument has $\mathsf{Bad}$ in a negative position. If allowed, we could define:

$$\omega = \mathsf{bad}\, (\lambda x. (\mathsf{rec}_\mathsf{Bad}\, \ldots)\, x\, x)$$

and then $\omega\, \omega$ would diverge — breaking strong normalization. The positivity condition forbids constructors where the type appears to the left of a function arrow in a constructor argument.

All standard inductive types (ℕ, lists, trees, vectors) satisfy positivity. Agda and Lean 4 check this automatically.

## The Recursor vs. Pattern Matching

Two ways to define functions by cases on an inductive type:

**Recursor style** (low-level, always terminates by construction):
```agda
double : ℕ → ℕ
double = recℕ 0 (λ n r → succ (succ r))
```

**Pattern matching style** (high-level, Agda checks termination):
```agda
double : ℕ → ℕ
double zero = zero
double (suc n) = suc (suc (double n))
```

Both are equivalent; pattern matching is syntactic sugar for the recursor. In practice, everyone uses pattern matching — it's more readable. But the *semantics* is given by the recursor, which is why the termination checker can verify that pattern-matching definitions are correct.

## Summary

Inductive types are defined by:
1. **Constructors:** The ways to build elements
2. **Elimination principle (recursor):** The uniform way to use elements, one case per constructor

For $\mathbb{N}$:
- Constructors: $\mathsf{zero} : \mathbb{N}$ and $\mathsf{succ} : \mathbb{N} \to \mathbb{N}$
- Recursor (non-dependent): $C \to (C \to C) \to \mathbb{N} \to C$
- Recursor (dependent, induction): $\prod_{P:\mathbb{N}\to\mathsf{Type}} P(0) \to (\prod_{n:\mathbb{N}} P(n) \to P(n+1)) \to \prod_{n:\mathbb{N}} P(n)$
- Computation: the recursor reduces on $\mathsf{zero}$ and $\mathsf{succ}$ as specified

The non-dependent recursor gives all primitive recursive functions; the dependent recursor gives mathematical induction.

Everything else — lists, vectors, trees, identity types, even the universe itself (in a sense) — follows the same pattern: specify constructors, derive the elimination principle, state the computation rules.
