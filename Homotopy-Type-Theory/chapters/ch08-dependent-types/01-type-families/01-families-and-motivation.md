# 1.1 Type Families and Motivation

## What's Missing in Non-Dependent Type Systems

Let's start with a concrete frustration. Suppose you're writing a safe matrix multiplication function. You want to say: "this function takes an $m \times n$ matrix and an $n \times p$ matrix and returns an $m \times p$ matrix." The type should *enforce* that the inner dimensions match.

In STLC or Haskell (without GHC extensions), the best you can do is something like `Matrix → Matrix → Matrix`. The dimension constraint lives in a comment, not the type. The compiler won't catch it if you call the function with incompatible dimensions.

With dependent types, you can write:

$$\mathsf{matmul} : \mathsf{Mat}(m, n) \to \mathsf{Mat}(n, p) \to \mathsf{Mat}(m, p)$$

where $m$, $n$, $p$ are terms of type $\mathbb{N}$. The function is indexed by three specific numbers, and the type system ensures they're used consistently.

The key new ingredient is a *type family* — a function that takes a value and returns a type.

## Type Families: The Basic Concept

**Definition.** A *type family* over a type $A$ is a function $B : A \to \mathsf{Type}$ — a function that takes an element $a : A$ and returns a type $B(a)$.

This is not a type constructor like `List` (which takes a type and returns a type). A type family takes a *term* and returns a type.

**Examples:**

**The vector family.** $\mathsf{Vec}(A) : \mathbb{N} \to \mathsf{Type}$ defined by:
- $\mathsf{Vec}(A, 0) = \mathbf{1}$ (the unit type — only the empty vector)
- $\mathsf{Vec}(A, n+1) = A \times \mathsf{Vec}(A, n)$ (a head element paired with a shorter vector)

This is a type family indexed by natural numbers. Different values of $n$ give genuinely different types.

**The finite set family.** $\mathsf{Fin} : \mathbb{N} \to \mathsf{Type}$ where $\mathsf{Fin}(n)$ has exactly $n$ elements: $\{0, 1, \ldots, n-1\}$. This is useful for safe array indexing: if your array has type $\mathsf{Vec}(A, n)$, then a valid index has type $\mathsf{Fin}(n)$, making out-of-bounds access a type error.

**Goldbach as a type family.** For each $n : \mathbb{N}$, define $\mathsf{Goldbach}(n)$ to be the type:
$$\mathsf{Goldbach}(n) = \{(p, q) : \mathbb{P} \times \mathbb{P} \mid n = p + q\}$$
(where $\mathbb{P}$ is the type of prime numbers). Then $\prod_{n : \mathbb{N}} (n > 2 \land n \text{ even}) \to \mathsf{Goldbach}(n)$ is the type of a proof of Goldbach's conjecture.

The type depends on a value $n$. Different values give different (and interesting!) mathematical claims.

**The identity type family.** For a fixed $a : A$, define $\mathsf{Id}_A(a) : A \to \mathsf{Type}$ by $\mathsf{Id}_A(a, b)$ = "the type of proofs that $a$ equals $b$ in $A$." This is the central type family in HoTT.

## Why This Generalizes Everything We've Done

Non-dependent function types $A \to B$ are the special case of type families where $B$ doesn't actually depend on the input. If $B : A \to \mathsf{Type}$ is the constant family $B(a) = C$ for all $a$, then a function that takes $a : A$ and returns an element of $B(a) = C$ is just a function $A \to C$.

So type families strictly generalize arrow types. They also generalize polymorphism: in System F, a polymorphic term has type $\forall \alpha. A(\alpha)$, quantifying over types. In dependent type theory, this becomes a type family $A : \mathsf{Type} \to \mathsf{Type}$, and the quantification is $\prod_{\alpha : \mathsf{Type}} A(\alpha)$.

The unification of "type-level quantification" (polymorphism) and "value-level dependent types" into a single framework is one of the most elegant aspects of dependent type theory.

## Type Families in Context: The Judgment $\Gamma \vdash B(x) : \mathsf{Type}\ (x : A)$

In a dependent type theory, we need to be careful about what it means to form a type family. The family $B$ is a function from $A$ to types, so we need $A$ and $B$ to be well-formed in context. The key judgment is:

$$\Gamma, x : A \vdash B(x) : \mathsf{Type}$$

This says: "in context $\Gamma$, extended with a variable $x$ of type $A$, the expression $B(x)$ is a well-formed type." This is the formal statement that $B$ is a type family over $A$.

The variable $x$ in this judgment is a *free variable* — it doesn't have a specific value. When we substitute a specific term $a : A$ for $x$, we get the specific type $B(a)$.

**Example.** The vector family is:
$$n : \mathbb{N} \vdash \mathsf{Vec}(A, n) : \mathsf{Type}$$
This says that given any specific natural number $n$, we get a well-formed type $\mathsf{Vec}(A, n)$.

## Substituting into Type Families

If $B$ is a type family over $A$ and $a : A$ is a specific element, we get the *fiber* of $B$ at $a$: the type $B(a)$.

This substitution is a key operation: $B[a/x]$ means "substitute $a$ for $x$ in $B$." For type families, this gives the specific type at that index.

**Example.** If $B(n) = \mathsf{Vec}(A, n)$ and we substitute $n := 3$, we get $B(3) = \mathsf{Vec}(A, 3)$, the type of three-element vectors over $A$.

The substitution works because types and terms live in the same language — unlike in STLC where the type layer and the term layer are strictly separated. In dependent type theory, the judgment $a : A$ and the type expression $B(a)$ are in the same syntactic category.

## Dependent Types and the Logic of Predicates

The connection to first-order logic becomes clear when you read type families as predicates.

A predicate $P$ on a set $A$ is a function $P : A \to \{\top, \bot\}$ — for each element $a$, it says whether $P(a)$ holds. In dependent type theory, a type family $B : A \to \mathsf{Type}$ plays the same role: $B(a)$ is the type of proofs that the predicate holds at $a$. It's inhabited if the predicate holds, empty if it doesn't.

| Logic | Type Theory |
|---|---|
| Predicate $P$ on $A$ | Type family $B : A \to \mathsf{Type}$ |
| $P(a)$ holds | $B(a)$ is inhabited |
| $P(a)$ fails | $B(a) = \mathbf{0}$ (empty type) |
| $\forall x \in A, P(x)$ | $\prod_{x:A} B(x)$ (Π type) |
| $\exists x \in A, P(x)$ | $\sum_{x:A} B(x)$ (Σ type) |

This table is the key to the full Curry-Howard correspondence for first-order logic. The Π type (Section 2) formalizes the universal quantifier; the Σ type (Section 3) formalizes the existential.

## Transport: Moving Between Fibers

Here's something that doesn't exist in non-dependent type theory. If $B$ is a type family over $A$, and $p : a = b$ is a proof that $a$ equals $b$ in $A$, then we can *transport* an element of $B(a)$ to an element of $B(b)$:

$$\mathsf{transport} : \prod_{a\, b : A} (a = b) \to B(a) \to B(b)$$

This might look simple, but it's profound. It says: if $a$ and $b$ are identified (equal), then anything true of $a$ (in the sense of $B(a)$ being inhabited) is also true of $b$. This is the *substitution of equals for equals*, formalized as a type-theoretic operation.

In HoTT, the equality type $a = b$ is interpreted as a *path* from $a$ to $b$ in the space $A$. Transport then says: a path in the base space $A$ induces a function between fibers $B(a)$ and $B(b)$. This is the type-theoretic version of *parallel transport* in differential geometry — a geometric operation that turns out to be a consequence of the fundamental induction principle for identity types.

We'll prove this in Section 5 when we study the identity type as an inductive type. For now, the point is that type families are not just for indexing by numbers — they interact deeply with equality.

## Fibrations: Type Families as Topology

The HoTT perspective interprets type families geometrically. If $A$ is a space (a type), a type family $B : A \to \mathsf{Type}$ is a *fibration* over $A$: for each point $a$ in the base space $A$, we have a fiber $B(a)$ sitting over it.

The total space is $\sum_{a:A} B(a)$ (the Σ type, Section 3) — the space of all pairs $(a, b)$ where $a : A$ and $b : B(a)$. The projection map $\pi_1 : \sum_{a:A} B(a) \to A$ sends $(a, b)$ to $a$.

A *section* of the fibration is a function $f : \prod_{a:A} B(a)$ — for each point $a$, choose an element $f(a)$ in the fiber over $a$. Sections are Π types.

This is the dictionary:
| Topology | Type Theory |
|---|---|
| Fibration $E \to B$ | Type family $B : A \to \mathsf{Type}$ |
| Fiber over $b$ | $B(a)$ |
| Total space | $\sum_{a:A} B(a)$ |
| Section | $f : \prod_{a:A} B(a)$ |
| Parallel transport | $\mathsf{transport}$ |

We're not just using topology as a metaphor — in HoTT, the identity type $a = b$ genuinely behaves like a path space, and type families genuinely behave like fibrations. This is the geometric content of HoTT, and it all starts here, with the concept of a type family.

## Looking Ahead

Type families are the raw material. In Section 2, we'll see how to form functions into type families (Π types). In Section 3, we'll form pairs where the type of the second component depends on the first (Σ types). These are the two fundamental type formers in dependent type theory, and everything else — identity types, universes, inductive types — is built from them or alongside them.

The vector type $\mathsf{Vec}(A, n)$ that started this section will reappear in Section 5, defined as an inductive type. At that point, we'll have all the machinery to write `append`, `lookup`, and other length-indexed operations with full type safety.
