# 3.1 Σ Types: Dependent Pair Types

## The Problem Σ Types Solve

In STLC, the product type $A \times B$ gives you pairs: the first component has type $A$, the second has type $B$, and the two types are independent.

But consider this common pattern in mathematics: a *group* consists of a set $G$ together with a binary operation $\cdot : G \times G \to G$, an identity element $e : G$, and inverses — satisfying the group axioms. The operation, identity, and axioms all depend on the underlying set $G$. You can't separate the type of the carrier set from the type of the structure built on it.

More simply: suppose you want a type for "a natural number together with a proof that it's even." The proof that $n$ is even is a statement *about* $n$ — its type depends on the value $n$. A regular pair $\mathbb{N} \times \mathsf{IsEven}$ would require a type `IsEven` that's the same for all numbers. But the predicate "is even" should apply to a *specific* number.

The Σ type solves this: $\sum_{n:\mathbb{N}} \mathsf{IsEven}(n)$ is the type of pairs $(n, p)$ where $n : \mathbb{N}$ and $p : \mathsf{IsEven}(n)$ — a proof that *that specific $n$* is even.

## The Σ Type Formation Rule

Given a type $A : \mathsf{Type}$ and a type family $B : A \to \mathsf{Type}$, the Σ type (also called the *dependent sum* or *dependent pair type*) is:

$$\frac{\Gamma \vdash A : \mathsf{Type} \quad \Gamma, x : A \vdash B(x) : \mathsf{Type}}{\Gamma \vdash \sum_{x:A} B(x) : \mathsf{Type}} \quad (\Sigma\text{-Form})$$

When $B$ is a constant family $B(x) = C$, we get $\sum_{x:A} C = A \times C$: the ordinary product type.

**Other notation:** Lean 4 uses `⟨a, b⟩ : Σ A B`; Agda uses `Σ A B` or `Σ[ x ∈ A ] B x`; mathematicians sometimes write $\bigsqcup_{a:A} B(a)$ (disjoint union of fibers). The HoTT Book uses $\sum_{x:A} B(x)$.

## The Introduction Rule: Dependent Pairs

To construct an element of $\sum_{x:A} B(x)$:

$$\frac{\Gamma \vdash a : A \quad \Gamma \vdash b : B(a)}{\Gamma \vdash (a, b) : \sum_{x:A} B(x)} \quad (\Sigma\text{-Intro})$$

A pair $(a, b)$ belongs to $\sum_{x:A} B(x)$ when $a : A$ and $b : B(a)$ — the second component must have the type dictated by the specific first component.

**Example.** In $\sum_{n:\mathbb{N}} \mathsf{IsEven}(n)$:
- $(4, \text{proof that 4 is even}) : \sum_{n:\mathbb{N}} \mathsf{IsEven}(n)$ ✓
- $(7, ?)$ fails — there's no proof that 7 is even, so no valid second component

The type system rejects invalid pairs at compile time. This is the power of dependent types: the constraint "the proof must match the number" is enforced by typing.

## The Elimination Rules: Projections

To use an element $p : \sum_{x:A} B(x)$, you project out its components:

$$\frac{\Gamma \vdash p : \sum_{x:A} B(x)}{\Gamma \vdash \pi_1\, p : A} \quad (\Sigma\text{-Elim}_1)$$

$$\frac{\Gamma \vdash p : \sum_{x:A} B(x)}{\Gamma \vdash \pi_2\, p : B(\pi_1\, p)} \quad (\Sigma\text{-Elim}_2)$$

The first projection $\pi_1\, p$ gives the $A$-component, as expected. The second projection $\pi_2\, p$ gives the $B$-component — but notice its *type*: it's $B(\pi_1\, p)$, the type family evaluated at the first component of $p$. The type of the second projection depends on what the first projection returns.

This is where the dependence lives: in a non-dependent product $A \times B$, the second projection always has type $B$. In a Σ type, the second projection has type $B(\pi_1\, p)$ — a type that depends on a specific term.

## The Computation Rules

$$\pi_1\, (a, b) \equiv a \quad (\Sigma\text{-}\beta_1)$$
$$\pi_2\, (a, b) \equiv b \quad (\Sigma\text{-}\beta_2)$$

Projecting from a pair reduces as expected. And the uniqueness ($\eta$) rule:
$$p \equiv (\pi_1\, p, \pi_2\, p) \quad (\Sigma\text{-}\eta)$$

Every element of a Σ type is (definitionally equal to) a pair of its two projections.

## The General Eliminator: Dependent Pattern Matching

Actually, the projection rules are a special case of a more general elimination principle. To define a function *out of* a Σ type into a type family $C : (\sum_{x:A} B(x)) \to \mathsf{Type}$:

$$\mathsf{ind}_\Sigma : \prod_{C : (\sum_{x:A} B(x)) \to \mathsf{Type}} \left(\prod_{a:A} \prod_{b:B(a)} C(a, b)\right) \to \prod_{p : \sum_{x:A} B(x)} C(p)$$

This says: to prove $C(p)$ for all $p : \sum_{x:A} B(x)$, it suffices to prove $C(a, b)$ for all pairs $(a, b)$ with $a : A$ and $b : B(a)$. This is *dependent pattern matching*: when you match on $p = (a, b)$, you can use the fact that $p = (a, b)$ in the type of your result.

The projections are recovered as special cases: $\pi_1 = \lambda p. \mathsf{ind}_\Sigma\, (\lambda\_. A)\, (\lambda a. \lambda b. a)\, p$ and similarly for $\pi_2$.

## Σ Types as Existential Quantifiers

Under Curry-Howard:

| Logic | Type Theory |
|---|---|
| $\exists x \in A, P(x)$ | $\sum_{x:A} B(x)$ |
| Proof of $\exists x, P(x)$ | Pair $(a, p)$ with $a : A$ and $p : B(a)$ |
| "Witness" of the existential | $\pi_1\, (a, p) = a$ |
| "Evidence" of the property | $\pi_2\, (a, p) : B(a)$ |

An existential statement $\exists x, P(x)$ is proved by producing a *witness* $a$ and a *proof* $p$ that $P(a)$ holds. In type theory, this is exactly a pair $(a, p) : \sum_{x:A} B(x)$.

**Example.** The statement "some natural number is even" becomes:
$$\sum_{n:\mathbb{N}} \mathsf{IsEven}(n)$$
A proof is a pair $(4, \text{proof that 4 is even})$: we produce the witness (4) and the evidence (the proof).

**But note:** In classical mathematics, $\exists x, P(x)$ means $P$ holds somewhere, but you might not know where. In constructive type theory, a proof of $\sum_{x:A} B(x)$ gives you an *explicit witness*. You can always extract the witness by taking $\pi_1$. This is the **existence property**: constructive proofs are programs that compute witnesses.

## Subtype Notation: $\{x : A \mid P(x)\}$

A common use of Σ types is to cut out a *subtype*: all elements of $A$ satisfying a predicate $P$. We write:

$$\{x : A \mid P(x)\} = \sum_{x:A} P(x)$$

This is the type of pairs $(a, p)$ where $a : A$ and $p : P(a)$ (a proof that $a$ satisfies $P$).

**Examples:**
- $\{n : \mathbb{N} \mid \mathsf{IsEven}(n)\}$ — even natural numbers
- $\{f : A \to B \mid \mathsf{IsInjective}(f)\}$ — injective functions from $A$ to $B$
- $\{n : \mathbb{N} \mid n < k\} = \mathsf{Fin}(k)$ — the type with $k$ elements

In set theory, $\{x \in A \mid P(x)\}$ is defined by Separation/Comprehension. In type theory, the Σ type plays the same role, with the extra structure that you always remember which proof of $P$ you're using.

## Algebraic Structures as Σ Types

Mathematically, Σ types let you internalize the definition of algebraic structures. A *group* in type theory is:

$$\mathsf{Group} = \sum_{G : \mathsf{Type}} \sum_{\_\cdot\_ : G \to G \to G} \sum_{e : G} \sum_{\mathsf{inv} : G \to G} \mathsf{GroupAxioms}(G, \cdot, e, \mathsf{inv})$$

where $\mathsf{GroupAxioms}$ is a type expressing associativity, identity laws, and inverse laws. An element of $\mathsf{Group}$ is a tuple $(G, \cdot, e, \mathsf{inv}, \text{proofs})$ — a group in the usual sense.

This works perfectly in dependent type theory because each component's type can depend on the earlier components (e.g., the type of the multiplication $\cdot$ depends on the carrier $G$). You couldn't write this as a non-dependent tuple.

**Ring, field, topological space, category:** All definable as Σ types over the appropriate data, with proof obligations in the final components. This is how Lean 4's Mathlib and Agda's standard library define algebraic structures.

## The Currying Isomorphism (Revisited)

Earlier we saw dependent currying:
$$\left(\sum_{a:A} B(a)\right) \to C \cong \prod_{a:A} B(a) \to C$$

This is an isomorphism of types (for any $C$ that doesn't depend on the Σ type). It says: a function from dependent pairs is the same as a curried function that takes the components separately.

But there's a more subtle version. When $C$ depends on the Σ type:
$$\prod_{p : \sum_{a:A} B(a)} C(p) \cong \prod_{a:A} \prod_{b:B(a)} C(a, b)$$

This is the *dependent* version: a dependent function over pairs is equivalent to a doubly-dependent function. This equivalence is given by:
- Forward: $f \mapsto \lambda a. \lambda b. f\, (a, b)$
- Backward: $g \mapsto \lambda p. g\, (\pi_1 p)\, (\pi_2 p)$

Both directions reduce by $\beta$/$\eta$ rules, giving definitional equalities.

## Associativity and Iterated Σ Types

Iterated Σ types don't require parentheses in the "obvious" way:
$$\sum_{a:A} \sum_{b:B(a)} C(a, b) \cong \sum_{p:\sum_{a:A} B(a)} C(\pi_1 p, \pi_2 p)$$

Both express "triples $(a, b, c)$ where $a : A$, $b : B(a)$, $c : C(a, b)$." The isomorphism is just rebracketing.

In practice, we write $\sum_{a:A} \sum_{b:B(a)} C(a, b)$ and think of it as a dependent triple. This iterated Σ structure underlies the definition of algebraic structures and contexts in type theory.

## Total Spaces and Fibrations

In the geometric/HoTT interpretation:
- $\sum_{a:A} B(a)$ is the *total space* of the fibration $B$ over $A$
- Elements are pairs $(a, b)$ with $a$ in the base and $b$ in the fiber over $a$
- The projection $\pi_1 : \sum_{a:A} B(a) \to A$ is the *fibration map*

This is literally the total space construction from topology. The fiber over a point $a$ is $\{p : \sum_{x:A} B(x) \mid \pi_1 p = a\}$, which is equivalent to $B(a)$ (by the path-lifting property / transport).

In HoTT:
- If $B(a) = (b = a)$ for some fixed $b$, the total space $\sum_{a:A} (b = a)$ is *contractible* (it has a unique center of contraction at $(b, \mathsf{refl})$). This is the *path space fibration*.
- The fundamental theorem of HoTT identity types says: to characterize $a = b$ in $A$, it suffices to show the corresponding Σ type is contractible.

## The Constructive Axiom of Choice

Here's a striking theorem in dependent type theory:

**Theorem (Constructive Axiom of Choice, trivial).** For any $A$, $B : A \to \mathsf{Type}$, $C : \prod_{a:A} B(a) \to \mathsf{Type}$:
$$\left(\prod_{a:A} \sum_{b:B(a)} C(a, b)\right) \to \sum_{f : \prod_{a:A} B(a)} \prod_{a:A} C(a, f\, a)$$

In logic, this says: "if for every $a$ there exists a $b$ such that $C(a, b)$, then there exists a function $f$ assigning to each $a$ a $b$ such that $C(a, f\, a)$." This is the Axiom of Choice!

In type theory, this is not an axiom — it's a trivial theorem proved by:
$$\lambda h. (\lambda a. \pi_1 (h\, a), \lambda a. \pi_2 (h\, a))$$

The "choice function" $f$ is $\lambda a. \pi_1(h\, a)$: take the witness from the pair given by $h$. The proof that $C$ holds is $\lambda a. \pi_2(h\, a)$.

The reason AC is trivial here: in constructive type theory, a proof of $\sum_{b:B(a)} C(a, b)$ always comes with an explicit witness. There's no non-constructive "existence without witness." So "for every $a$, there exists a $b$" literally gives you a *function* from $a$ to witnesses.

This is a major difference from classical set theory, where the Axiom of Choice is independent of ZF and philosophically controversial. In dependent type theory, choice is built into the proof structure.

## Summary

The Σ type is:
- The **generalization of product types** to the dependent case
- The **existential quantifier** under Curry-Howard
- The **total space** of a fibration in geometry
- The **subtype constructor**: $\{x : A \mid P(x)\} = \sum_{x:A} P(x)$
- The basis for **algebraic structures** in type theory

Key rules:
- **Formation:** $A : \mathsf{Type}$, $(x : A \vdash B(x) : \mathsf{Type})$ gives $\sum_{x:A} B(x) : \mathsf{Type}$
- **Introduction:** $a : A$, $b : B(a)$ gives $(a, b) : \sum_{x:A} B(x)$
- **Elimination:** $p : \sum_{x:A} B(x)$ gives $\pi_1 p : A$ and $\pi_2 p : B(\pi_1 p)$
- **Computation:** $\pi_1(a,b) \equiv a$, $\pi_2(a,b) \equiv b$
- **Uniqueness:** $p \equiv (\pi_1 p, \pi_2 p)$

Together with Π types, Σ types give dependent type theory its logical completeness: the ability to express any mathematical statement, with proofs as programs that compute witnesses.
