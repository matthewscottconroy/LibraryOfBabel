# Chapter 7 Overview: Induction and Recursion

---

## Central Question

When is a definition by recursion well-defined? When does "define $f$ by: $f(0) = c$ and $f(n+1) = g(n, f(n))$" actually give a unique function? And when can we generalise from induction on $\mathbb{N}$ to induction on *any* well-founded structure?

---

## Why This Chapter Matters

Induction and recursion are the engines of discrete mathematics and theoretical computer science. Every data type in a functional programming language is an inductive type; every function defined on it is defined by structural recursion; every property proved about it is proved by structural induction. Lean 4 and Coq are built around inductive types and structural recursion. Understanding the logical foundations — why these definitions and proofs are valid — is essential for rigorous programming and proof.

---

## Key Definitions

**Well-founded relation.** A binary relation $\prec$ on a set $A$ is *well-founded* if every non-empty subset of $A$ has a $\prec$-minimal element. Equivalently (in ZFC): there is no infinite $\prec$-descending chain $a_0 \succ a_1 \succ a_2 \succ \cdots$.

**Accessible element.** An element $a \in A$ is *accessible* (with respect to $\prec$) if every $\prec$-descending sequence starting at $a$ terminates. The accessible elements form the largest subset on which $\prec$ is well-founded.

**Well-founded induction principle.** If $\prec$ is well-founded on $A$, and $P$ is a property such that: for all $a \in A$, if $P(b)$ holds for all $b \prec a$, then $P(a)$ holds — then $P(a)$ holds for all $a \in A$.

**Structural induction.** Induction on an inductively defined type, where the ordering is "subterm." For a formula $\phi$: $\phi'$ is a proper subformula of $\phi$ if $\phi'$ is one of the immediate components used to build $\phi$.

**The recursion theorem (on $\mathbb{N}$).** For any set $A$, element $c \in A$, and function $g: \mathbb{N} \times A \to A$, there is a unique function $f: \mathbb{N} \to A$ such that $f(0) = c$ and $f(n+1) = g(n, f(n))$.

---

## Main Theorems

### The Recursion Theorem

**Theorem.** Let $A$ be a set, $c \in A$, and $g: \mathbb{N} \times A \to A$. Then there is a unique function $f: \mathbb{N} \to A$ satisfying the primitive recursion equations $f(0) = c$ and $f(n+1) = g(n, f(n))$.

**Proof sketch.** Define a "finite approximation" $t: \{0, \ldots, k\} \to A$ to be an approximation if it satisfies the recursion equations up to $k$. One shows:
1. Every $n$ is in the domain of some approximation.
2. Any two approximations agree wherever both are defined.
Define $f(n) = t(n)$ for any approximation $t$ with $n$ in its domain. This is well-defined by (2), total by (1), and satisfies the equations. Uniqueness follows by induction. $\square$

**Why the proof is nontrivial:** The definition is circular on its face ("define $f$ using $f$"). The proof resolves the circularity by constructing $f$ as the union of compatible partial functions, each defined only for finitely many values.

### Well-Founded Recursion Theorem

**Theorem.** Let $\prec$ be a well-founded relation on $A$. Let $h: A \times (\text{partial functions}) \to B$ be a function (called the "step function"). Then there is a unique function $f: A \to B$ such that for every $a \in A$:

$$f(a) = h(a, f\restriction_{\{b : b \prec a\}})$$

where $f\restriction_S$ is the restriction of $f$ to $S$.

**Proof.** By well-founded induction: define $f(a)$ by assuming $f$ is already defined on all $b \prec a$. Well-foundedness guarantees this induction terminates (no infinite descending chain). $\square$

**Applications:**
- The Ackermann function (total but not primitive recursive) is defined by well-founded recursion on pairs of natural numbers with the lexicographic order.
- Quicksort terminates because the list length strictly decreases in each recursive call.
- All Lean 4 and Coq functions defined by structural recursion implicitly invoke this theorem.

### Proof by Well-Founded Induction

**Principle.** Let $\prec$ be well-founded on $A$. To prove $\forall a \in A, P(a)$:

*Inductive step only:* Assume $P(b)$ holds for all $b \prec a$ (the "inductive hypothesis"), and prove $P(a)$.

This replaces both the base case and the inductive step of ordinary induction. The base case arises automatically: for $\prec$-minimal elements, there are no $b \prec a$, so the hypothesis is vacuous, and we must prove $P(a)$ from scratch.

**Example:** Proof of Euclid's algorithm terminates.

The Euclidean algorithm computes $\gcd(a, b)$ by repeated application of: $\gcd(a, b) = \gcd(b, a \bmod b)$ (for $b \neq 0$). Terminates because $a \bmod b < b$, so the second argument strictly decreases. By well-founded induction on $b$, the algorithm terminates for all inputs. $\square$

---

## Inductive Types and Structural Recursion

An *inductive type* is defined by its constructors. For example, the type of natural numbers:

```lean4
inductive Nat : Type where
  | zero : Nat
  | succ : Nat → Nat
```

This defines `Nat` as the *smallest* type containing `zero` and closed under `succ`. Every natural number is either `zero` or `succ n` for a unique smaller `n`.

The *recursion principle* for `Nat` is exactly the recursion theorem: to define a function $f: \texttt{Nat} \to A$, provide $f(\texttt{zero}) = c$ and $f(\texttt{succ}\ n) = g(n, f(n))$.

The *induction principle* for `Nat` follows the same pattern: to prove $P$ holds for all `Nat`, prove it for `zero` and prove the step.

**For binary trees:**

```lean4
inductive Tree (α : Type) : Type where
  | leaf : Tree α
  | node : Tree α → α → Tree α → Tree α
```

Structural recursion on `Tree α` processes each `node` by recursing on both subtrees.

---

## Historical Context

**Richard Dedekind (1888)** published *Was sind und was sollen die Zahlen?* ("What are numbers and what should they be?"), in which he proved the recursion theorem for natural numbers — the first rigorous proof that recursive definitions are well-founded. This is the first appearance of the idea that the natural numbers are an initial algebra (Peano's axioms, stated the same year, provide the axioms; Dedekind proved the recursion theorem).

**Giuseppe Peano (1889)** introduced the axioms for arithmetic (now called the Peano axioms, though largely due to Dedekind): 0 is a natural number, every natural number has a unique successor, 0 is not a successor, the induction principle. These axioms characterise the natural numbers up to isomorphism.

**Dana Scott and Gordon Plotkin (1970s)** developed domain theory for denotational semantics, which provides the semantic foundation for well-founded recursion in programming language theory.

**Per Martin-Löf (1975, 1984)** introduced intuitionistic type theory with inductive types defined by elimination rules — the direct ancestor of Lean 4 and Coq's inductive type mechanism.

---

## Common Pitfalls

**Non-well-founded recursion:** The definition $f(n) = f(n+1)$ does not define a function: there is no "descent" — the recursion goes upward, not downward.

**Non-structural recursion:** In Lean 4, a function defined by recursion must be *provably terminating*. Lean checks this automatically for structural recursion (recursing on structurally smaller arguments). For other recursive patterns, you must provide a well-founded measure (a function into a well-ordered set that decreases with each call).

**Induction vs. recursion:** They are logically equivalent (and related by Curry-Howard), but they arise in different contexts. Induction is for *proving*; recursion is for *defining*. In dependent type theory, they are the same thing.

---

## Connections to Other Chapters

- **Chapter 6** (Set Theory) defines well-foundedness using sets and provides the set-theoretic proof of the recursion theorem.
- **Chapter 8** (Number Theory) uses mathematical induction extensively for proofs about divisibility, primes, and modular arithmetic.
- **Chapter 10** (Computability) defines primitive recursive and general recursive functions using exactly the recursion theorem.
- **Chapter 11** (Type Theory) makes the connection between inductive types and structural recursion/induction precise via the Curry-Howard correspondence.
- **Chapter 13** (Formal Verification) implements structural recursion and induction as the core of Lean 4 and Coq's definitional mechanisms.
