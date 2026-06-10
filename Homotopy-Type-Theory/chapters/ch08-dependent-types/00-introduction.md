# Chapter 8: Dependent Types

## The Idea That Changes Everything

So far, we've been working with type systems where types and terms live in completely separate worlds. In STLC, a term has a type, and that type is fixed once you write it down. In System F, you can quantify over types — but only in a way where types depend on other types, not on term-level values.

Dependent types collapse this separation. In a dependently typed system, a type can depend on a *value*. This small change has enormous consequences.

Here's the simplest example that shows what's new. Suppose you want a type for "lists of exactly $n$ elements." In STLC or even System F, you can't do this — you can write `List A` (lists of $A$s), but you can't encode the length constraint in the type. You'd have to enforce it at runtime, losing static guarantees.

With dependent types, you write $\mathsf{Vec}(A, n)$: the type of vectors over $A$ with exactly $n$ entries, where $n$ is a *term* of type $\mathbb{N}$. The type depends on the value $n$. This lets you write a type-safe `append` function:

$$\mathsf{append} : \mathsf{Vec}(A, m) \to \mathsf{Vec}(A, n) \to \mathsf{Vec}(A, m + n)$$

The output length is *computed* from the input lengths, all at the type level. If you try to pass a three-element vector where a two-element vector is expected, the type checker rejects it — before your program ever runs.

## Why Dependent Types? The Spectrum of Expressiveness

Each rung on the type-system ladder lets you express more:

- **STLC:** Every well-typed program terminates. Types separate "functions from $A$ to $B$" from "values of type $A$."
- **System F:** Polymorphism. One `id` function works at every type.
- **System Fω:** Type operators. `List` is a function at the type level.
- **Dependent types:** Types that depend on values. The full Curry-Howard correspondence becomes available: *propositions are types, proofs are programs, theorems are types with inhabitants*.

With dependent types, we can finally express things like:

- *Goldbach's conjecture:* A type whose inhabitants would be proofs that every even number $> 2$ is a sum of two primes.
- *The fundamental group:* A type family $\pi_1(X, x_0)$ that depends on a space $X$ and a basepoint $x_0$.
- *Sorting correctness:* The return type of `sort` says the output is sorted and is a permutation of the input.

In each case, what was previously a comment or a runtime check becomes part of the type, verified by the type checker.

## The HoTT Connection

This chapter develops dependent types as they appear in Martin-Löf Type Theory (MLTT). Everything in HoTT is built on this foundation:

- **Identity types** $\mathsf{Id}_A(a, b)$ — the type of proofs that $a = b$ in $A$ — are the central objects of HoTT. They're inductive types in the dependent type framework.
- **Univalence** is a statement about the universe (a type of all types), which itself requires dependent types to even formulate.
- **Higher inductive types** are inductive types with path constructors — again, essentially dependent type machinery.

Chapter 8 is the bridge from the lambda cube to HoTT. We're building the foundation that everything else rests on.

## Chapter Roadmap

**Section 1: Type Families.** A type family is a function from terms to types — the basic building block of dependent types. We'll see how $\mathsf{Vec}(A, n)$ is a family indexed by $n : \mathbb{N}$, and how type families generalize what we've done so far.

**Section 2: Π Types (Dependent Function Types).** The generalization of $A \to B$ to the case where $B$ can depend on the input value. Formation, introduction, elimination, and the $\beta$/$\eta$ laws. The connection to universal quantification ($\forall$) becomes complete here.

**Section 3: Σ Types (Dependent Pair Types).** The generalization of $A \times B$ where the type of the second component can depend on the first. Σ types express existential statements, subsets, structures, and more.

**Section 4: Universes.** Types live in universes. The type of all (small) types — a universe — lets us quantify over types without circularity, but requires care to avoid paradox. The universe hierarchy $\mathsf{Type}_0 : \mathsf{Type}_1 : \mathsf{Type}_2 : \cdots$ keeps everything consistent.

**Section 5: Inductive Types.** Natural numbers, lists, vectors, trees, and more, all defined by specifying constructors and an elimination principle (a recursor). The recursor is the formal counterpart of mathematical induction.

**Section 6: The Full Curry-Howard Correspondence.** With dependent types, the correspondence between logic and type theory is complete. We map every logical connective and quantifier to a type former, and see that proofs in dependent type theory are simply inhabitants of types.

**Section 7: Exercises.**

## A Note on Notation

Different sources use different notation. We'll mostly follow the HoTT Book and Martin-Löf's original papers, but note the variants:

| Concept | Our notation | Lean 4 | Agda |
|---|---|---|---|
| Π type | $\prod_{x : A} B(x)$ | `(x : A) → B x` | `(x : A) → B x` |
| Σ type | $\sum_{x : A} B(x)$ | `⟨a, b⟩ : Σ A B` | `Σ A B` |
| Universe | $\mathsf{Type}_i$ | `Type u` | `Set` / `Type` |
| Lambda | $\lambda x. t$ | `fun x => t` | `λ x → t` |

The ideas are the same across all these systems. The notation varies because type theory has been discovered and rediscovered in multiple communities — mathematics, computer science, philosophy of logic — that developed their own conventions.

Let's begin.
