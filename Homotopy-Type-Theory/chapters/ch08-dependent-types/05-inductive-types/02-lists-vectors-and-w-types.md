# 5.2 Lists, Vectors, and W-Types

## Lists: Inductively Defined

The type of lists over $A$ is one of the most fundamental inductive types. It has two constructors:

$$\mathsf{nil} : \mathsf{List}(A)$$
$$\mathsf{cons} : A \to \mathsf{List}(A) \to \mathsf{List}(A)$$

A list is either empty ($\mathsf{nil}$) or a head element (of type $A$) consed onto a tail (another list). This is the inductive definition: $\mathsf{cons}$ takes an existing list (the recursive component) and extends it.

The elimination principle (dependent recursor):

$$\mathsf{ind}_{\mathsf{List}(A)} : \prod_{P : \mathsf{List}(A) \to \mathsf{Type}} P(\mathsf{nil}) \to \left(\prod_{a:A} \prod_{l:\mathsf{List}(A)} P(l) \to P(\mathsf{cons}(a, l))\right) \to \prod_{l:\mathsf{List}(A)} P(l)$$

This says: to prove $P$ for every list, prove it for $\mathsf{nil}$ and prove it for $\mathsf{cons}(a, l)$ given $P(l)$. This is structural induction on lists.

Computation rules:
$$\mathsf{ind}\, P\, p_\mathsf{nil}\, p_\mathsf{cons}\, \mathsf{nil} \equiv p_\mathsf{nil}$$
$$\mathsf{ind}\, P\, p_\mathsf{nil}\, p_\mathsf{cons}\, (\mathsf{cons}(a, l)) \equiv p_\mathsf{cons}\, a\, l\, (\mathsf{ind}\, P\, p_\mathsf{nil}\, p_\mathsf{cons}\, l)$$

## Defining Functions on Lists

The `foldr` pattern from functional programming is just the non-dependent recursor:

$$\mathsf{foldr} : (A \to B \to B) \to B \to \mathsf{List}(A) \to B$$
$$\mathsf{foldr} = \mathsf{rec}_{\mathsf{List}(A)}$$

From `foldr`, we get:
- **Length:** $\mathsf{length} = \mathsf{foldr}\, (\lambda \_ n. \mathsf{succ}(n))\, 0$
- **Map:** $\mathsf{map}\, f = \mathsf{foldr}\, (\lambda a\, l. \mathsf{cons}(f\, a, l))\, \mathsf{nil}$
- **Append:** $l_1 \mathbin{++} l_2 = \mathsf{foldr}\, \mathsf{cons}\, l_2\, l_1$
- **Reverse:** $\mathsf{rev} = \mathsf{foldr}\, (\lambda a\, r. r \mathbin{++} [a])\, \mathsf{nil}$

Proofs about these functions follow by induction. For example:

**Theorem:** $\mathsf{length}(l_1 \mathbin{++} l_2) = \mathsf{length}(l_1) + \mathsf{length}(l_2)$.

Proof by induction on $l_1$:
- Base case: $\mathsf{length}(\mathsf{nil} \mathbin{++} l_2) = \mathsf{length}(l_2) = 0 + \mathsf{length}(l_2)$ ✓
- Inductive step: $\mathsf{length}(\mathsf{cons}(a, l_1) \mathbin{++} l_2) = 1 + \mathsf{length}(l_1 \mathbin{++} l_2)$ (by definition of `++` and `length`) $= 1 + \mathsf{length}(l_1) + \mathsf{length}(l_2)$ (by IH) $= \mathsf{length}(\mathsf{cons}(a, l_1)) + \mathsf{length}(l_2)$ ✓

## Vectors: Length-Indexed Lists

The vector type $\mathsf{Vec}(A, n)$ is a *dependent* inductive type: it's indexed by a natural number $n$, and the type changes based on that index. This is the type family we've been promising since the introduction.

$$\mathsf{nil} : \mathsf{Vec}(A, 0)$$
$$\mathsf{cons} : A \to \mathsf{Vec}(A, n) \to \mathsf{Vec}(A, \mathsf{succ}(n))$$

Notice: the `cons` constructor specifies the precise length of both its input and output. A vector of length $n$ consed with an element gives a vector of length $n+1$. The type carries this information.

The elimination principle:

$$\mathsf{ind}_{\mathsf{Vec}(A)} : \prod_{P : \prod_{n:\mathbb{N}} \mathsf{Vec}(A, n) \to \mathsf{Type}} P(0, \mathsf{nil}) \to \left(\prod_{n:\mathbb{N}} \prod_{a:A} \prod_{v:\mathsf{Vec}(A,n)} P(n, v) \to P(\mathsf{succ}(n), \mathsf{cons}(a, v))\right) \to \prod_{n:\mathbb{N}} \prod_{v:\mathsf{Vec}(A,n)} P(n, v)$$

The motive $P$ now depends on *both* the length $n$ and the vector $v$. This allows proving properties that mention the length.

## Type-Safe Vector Operations

The value of vectors over lists is compile-time safety for length-sensitive operations.

**Safe head:** Extract the first element of a non-empty vector. The type guarantees non-emptiness:
$$\mathsf{head} : \mathsf{Vec}(A, \mathsf{succ}(n)) \to A$$
There's no need to handle the empty case — the type rules it out. The function is:
$$\mathsf{head}\, (\mathsf{cons}\, a\, \_) = a$$

**Safe tail:** Remove the first element:
$$\mathsf{tail} : \mathsf{Vec}(A, \mathsf{succ}(n)) \to \mathsf{Vec}(A, n)$$
Again, no empty case needed.

**Type-safe append:** Concatenation adds lengths:
$$\mathsf{append} : \mathsf{Vec}(A, m) \to \mathsf{Vec}(A, n) \to \mathsf{Vec}(A, m + n)$$

In Agda:
```agda
append : {A : Set} {m n : ℕ} → Vec A m → Vec A n → Vec A (m + n)
append [] ys = ys
append (x ∷ xs) ys = x ∷ append xs ys
```

The `m + n` in the return type is literally arithmetic on natural numbers, computed at type-checking time. If you accidentally write `n + m` instead of `m + n`, the type checker will complain (since `m + n` and `n + m` are not definitionally equal — they require a proof of commutativity).

**Safe lookup:** Access the $k$-th element where $k < n$ is enforced:
$$\mathsf{lookup} : \mathsf{Fin}(n) \to \mathsf{Vec}(A, n) \to A$$

where $\mathsf{Fin}(n) = \{0, 1, \ldots, n-1\}$ is the type of indices for an $n$-element vector (defined inductively with constructors $\mathsf{fzero} : \mathsf{Fin}(\mathsf{succ}(n))$ and $\mathsf{fsucc} : \mathsf{Fin}(n) \to \mathsf{Fin}(\mathsf{succ}(n))$). An out-of-bounds index has the wrong type — it's a type error, not a runtime error.

## Dependent Inductive Types in General

The vector type is an example of an *indexed inductive family*: a type family $\mathsf{Vec}(A) : \mathbb{N} \to \mathsf{Type}$ where the type at each index is defined inductively, with constructors that specify the index precisely.

The general pattern: an indexed inductive family $T : I \to \mathsf{Type}$ over an index type $I$ has:
- Constructors that produce elements of $T(i)$ for specific indices $i$ (possibly computed from constructor arguments)
- An elimination principle that handles all constructors and propagates the index information

This is the type-theoretic analog of inductive families in universal algebra, or indexed families of sets in category theory.

## W-Types: The Universal Inductive Type

Here's a surprising theorem: all inductive types can be encoded using a single type former called the *W-type* (W for "wellfounded" or "well-ordered tree").

The W-type $W_{a:A} B(a)$ is defined by one constructor:

$$\mathsf{sup} : \prod_{a:A} (B(a) \to W_{x:A} B(x)) \to W_{x:A} B(x)$$

Intuitively: $\mathsf{sup}(a, f)$ is a tree node with "label" $a : A$ and "branching factor" $B(a)$ — each branch is indexed by an element of $B(a)$, and the subtree at branch $b : B(a)$ is $f(b)$.

The elimination principle:

$$\mathsf{ind}_W : \prod_{P : W_{a:A} B(a) \to \mathsf{Type}} \left(\prod_{a:A} \prod_{f:B(a) \to W} \prod_{g: \prod_{b:B(a)} P(f(b))} P(\mathsf{sup}(a, f))\right) \to \prod_{w : W_{a:A} B(a)} P(w)$$

Given: for every $a$, any subtree function $f$, and given $P$ holds for all subtrees $f(b)$, prove $P$ holds for $\mathsf{sup}(a, f)$. Then $P$ holds for all W-trees. This is wellfounded induction.

## Encoding Inductive Types as W-Types

**Natural numbers:** Take $A = \mathbb{B}$ (booleans) and:
$$B(\mathsf{false}) = \mathbf{0} \quad \text{(zero branches — leaf node)}$$
$$B(\mathsf{true}) = \mathbf{1} \quad \text{(one branch — unary tree)}$$

Then $W_{b:\mathbb{B}} B(b)$ has:
- Leaf nodes: $\mathsf{sup}(\mathsf{false}, \mathsf{absurd})$ — no branches (label false, function from $\mathbf{0}$, which is vacuous). This is $\mathsf{zero}$.
- Unary nodes: $\mathsf{sup}(\mathsf{true}, f)$ where $f : \mathbf{1} \to W$ gives one child. This is $\mathsf{succ}$ applied to $f(\mathsf{tt})$.

So $W_{\mathbb{B}} B \cong \mathbb{N}$: the W-type with these parameters is isomorphic to the natural numbers.

**Lists over $A$:** Take the index set to be $A + \mathbf{1}$ (either an element of $A$ or "nil"):
$$B(\mathsf{inr}(\mathsf{tt})) = \mathbf{0} \quad \text{(nil is a leaf)}$$
$$B(\mathsf{inl}(a)) = \mathbf{1} \quad \text{(cons has one child — the tail)}$$

Then $W_{A+\mathbf{1}} B \cong \mathsf{List}(A)$.

**Binary trees with $A$-labeled leaves:** Take $A$ for leaf labels and one binary-branching constructor:
$$B(\mathsf{inl}(a)) = \mathbf{0} \quad \text{(leaves have no children)}$$
$$B(\mathsf{inr}(\mathsf{tt})) = \mathbb{B} \quad \text{(binary nodes have 2 children: left and right)}$$

Then $W_{A+\mathbf{1}} B \cong \mathsf{BinTree}(A)$.

The W-type is a *universal* wellfounded tree type: by choosing $A$ (what labels nodes) and $B$ (how many branches each label gives), you get any wellfounded inductive type.

## Why W-Types Matter

W-types are important for foundational reasons:

1. **Minimality:** You can prove theorems about all inductive types by proving them for W-types once, then encoding.

2. **Universe independence:** W-types can be defined in a type theory with only Π and Σ types and a universe, without a primitive notion of "general inductive type." This shows that dependent type theory doesn't need inductive types as a primitive — they're derivable (at least in a weak sense).

3. **Wellfoundedness:** The elimination principle for W-types is exactly wellfounded (Noetherian) induction: you can prove anything about W-trees by showing that properties propagate from subtrees to trees. This connects type theory to the classical notion of well-ordered sets.

4. **Termination:** Every function defined by the W-type recursor terminates, because W-trees are finite (their depth is a natural number, and the elimination decreases depth at each step). This justifies the use of structural recursion.

## Inductive-Recursive Definitions

An advanced feature in Agda (and studied in type theory research): *inductive-recursive* definitions, where an inductive type and a function on it are defined simultaneously.

Example: a universe $U$ closed under Π types, defined inductively along with its decoding function $\mathsf{El} : U \to \mathsf{Type}$:

```agda
data U : Set where
  nat   : U
  pi    : (a : U) → (El a → U) → U

El : U → Set
El nat       = ℕ
El (pi a b)  = (x : El a) → El (b x)
```

Here $U$ and $\mathsf{El}$ are defined simultaneously: $U$ uses $\mathsf{El}$ in its constructor (for $\mathsf{pi}$, the family type $b : \mathsf{El}(a) \to U$ uses $\mathsf{El}$), and $\mathsf{El}$ is defined by recursion on $U$.

Inductive-recursive definitions are important for modeling universes inside type theory, and for Dybjer-Setzer's formalization of Martin-Löf's universe construction.

## Connection to HoTT: Higher Inductive Types

Standard inductive types have constructors that produce elements of the type. Higher Inductive Types (HITs) — the subject of Chapter 14 — also have *path constructors* that produce equalities (paths) between elements.

For example, the circle $S^1$ is a HIT:
- Point constructor: $\mathsf{base} : S^1$
- Path constructor: $\mathsf{loop} : \mathsf{base} = \mathsf{base}$

The loop constructor says there's a non-trivial path from $\mathsf{base}$ to itself — the fundamental loop of the circle. The elimination principle for $S^1$ says: to define a function out of $S^1$ into a type $P$, choose an image for $\mathsf{base}$ and a path over $\mathsf{loop}$.

HITs generalize the inductive type framework by allowing path constructors. All of algebraic topology can be done synthetically using HITs in HoTT.

## Summary

| Type | Constructors | Key Use |
|---|---|---|
| $\mathbb{N}$ | $\mathsf{zero}$, $\mathsf{succ}$ | Natural numbers, iteration |
| $\mathsf{List}(A)$ | $\mathsf{nil}$, $\mathsf{cons}$ | Variable-length sequences |
| $\mathsf{Vec}(A, n)$ | $\mathsf{nil}_0$, $\mathsf{cons}_n$ | Length-indexed sequences |
| $\mathsf{Fin}(n)$ | $\mathsf{fzero}$, $\mathsf{fsucc}$ | Bounded indices |
| $W_{a:A} B(a)$ | $\mathsf{sup}$ | Universal wellfounded trees |
| HIT (e.g., $S^1$) | points + paths | Spaces in HoTT |

The pattern throughout: define constructors, derive the elimination principle, state computation rules. The elimination principle is the precise statement of the induction/recursion principle, making proofs and definitions mechanical and verifiable.

W-types show that all of this can be grounded in a very simple universal construction — wellfounded trees. And HITs extend the pattern to higher dimensions, forming the cornerstone of HoTT's synthetic approach to topology.
