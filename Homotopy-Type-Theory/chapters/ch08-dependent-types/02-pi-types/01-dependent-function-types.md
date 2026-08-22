# 2.1 Π Types: Dependent Function Types

## From Arrow Types to Dependent Functions

In STLC, a function type $A \to B$ means: a function that takes an $A$ and returns a $B$. The output type $B$ is fixed — it doesn't change based on what input you give.

The Π type (pronounced "pi type" or "dependent product type") generalizes this. Given a type $A$ and a type family $B : A \to \mathsf{Type}$, the Π type $\prod_{x:A} B(x)$ is the type of functions that, given $a : A$, return an element of $B(a)$. The output *type* can depend on the input *value*.

The notation $\prod_{x:A} B(x)$ comes from the $\prod$ symbol for a (generalized) product — you're taking a "product" indexed by all the elements of $A$. Some authors write $\Pi(x : A). B(x)$ or $(x : A) \to B(x)$ (the latter being Lean 4 and Agda syntax).

**When does the output type actually depend on the input?** Whenever $B$ is a non-constant family. For `Vec(A)`:
- Input: $n : \mathbb{N}$
- Output type: $\mathsf{Vec}(A, n)$ — different for each $n$

So a function of type $\prod_{n:\mathbb{N}} \mathsf{Vec}(A, n)$ is a function that takes a number $n$ and returns a vector of *exactly* $n$ elements. That's a meaningful dependent type.

## The Formation Rule

To form a Π type, you need:
1. A type $A : \mathsf{Type}$
2. A type family $B : A \to \mathsf{Type}$ (i.e., a judgment $x : A \vdash B(x) : \mathsf{Type}$)

$$\frac{\Gamma \vdash A : \mathsf{Type} \quad \Gamma, x : A \vdash B(x) : \mathsf{Type}}{\Gamma \vdash \prod_{x:A} B(x) : \mathsf{Type}} \quad (\Pi\text{-Form})$$

The formation rule says: if $A$ is a type in context $\Gamma$, and $B(x)$ is a type whenever $x : A$ (in the extended context), then $\prod_{x:A} B(x)$ is a type in context $\Gamma$.

Notice that $x$ appears in the type expression $B(x)$ but is *bound* by the $\prod$ — it's a formal variable ranging over $A$. The resulting type $\prod_{x:A} B(x)$ doesn't contain $x$ freely.

**When $B$ is constant:** If $B(x) = C$ for all $x$ (i.e., $B$ is the constant family at $C$), then $\prod_{x:A} B(x) = \prod_{x:A} C = A \to C$. The non-dependent function type is a special case.

## The Introduction Rule: Lambda Abstraction

To construct an element of $\prod_{x:A} B(x)$, you write a lambda abstraction:

$$\frac{\Gamma, x : A \vdash t(x) : B(x)}{\Gamma \vdash \lambda x. t(x) : \prod_{x:A} B(x)} \quad (\Pi\text{-Intro})$$

Given a term $t(x)$ of type $B(x)$ in the context extended by $x : A$, we can abstract over $x$ to form $\lambda x. t(x)$ of type $\prod_{x:A} B(x)$.

This is the same syntactic form as the lambda abstraction in STLC — the difference is that the type $B(x)$ can depend on $x$. When you define $f = \lambda x. t(x)$, you're saying: "for any $x : A$, the result is $t(x)$, which has type $B(x)$."

**Example.** Define a function that takes $n : \mathbb{N}$ and returns the zero vector of length $n$:
$$\mathsf{zeros} = \lambda n. \underbrace{\langle 0, 0, \ldots, 0 \rangle}_{n \text{ times}} : \prod_{n:\mathbb{N}} \mathsf{Vec}(\mathbb{N}, n)$$

The function $\mathsf{zeros}$ has a dependent type: the return type depends on the argument.

## The Elimination Rule: Function Application

To use an element $f : \prod_{x:A} B(x)$, you apply it to an argument:

$$\frac{\Gamma \vdash f : \prod_{x:A} B(x) \quad \Gamma \vdash a : A}{\Gamma \vdash f\, a : B(a)} \quad (\Pi\text{-Elim})$$

If $f$ has a dependent function type and $a : A$, then $f\, a$ has type $B(a)$ — the type family evaluated at the specific argument $a$.

Compare to the non-dependent case: if $f : A \to C$ and $a : A$, then $f\, a : C$. The difference is that now the return type $B(a)$ depends on $a$.

**The crucial point:** When you apply $f$ to a *specific* value $a$, the type system knows you got $B(a)$ back — not just "some instance of $B$," but $B$ evaluated at the exact value $a$. This is how dependent types track precise information.

## The Computation Rule: β-Reduction

$$(\lambda x. t)\, a \equiv t[a/x] : B(a) \quad (\Pi\text{-}\beta)$$

Application of a lambda to an argument reduces by substitution. The type of the result is $B(a)$ — the type family evaluated at $a$, which is also what we get by substituting $a$ for $x$ in $B(x)$.

This is the same $\beta$-reduction as in lambda calculus. The novelty is that it must also work at the type level: the *type* $B(a)$ is computed by substituting $a$ for $x$ in $B(x)$, and this substitution is definitional equality in the type theory.

**Definitional vs. propositional equality:** Two terms are *definitionally equal* if they reduce to the same normal form (or are related by the computation rules). This is a judgment-level notion: $\Gamma \vdash t \equiv s : A$. Propositional equality is a type: $\mathsf{Id}_A(t, s)$. Every definitional equality gives a propositional equality (by reflexivity), but not vice versa.

## The Uniqueness Principle: η-Expansion

$$f \equiv \lambda x. f\, x : \prod_{x:A} B(x) \quad (\Pi\text{-}\eta)$$

Every element of a Π type is (definitionally equal to) a lambda abstraction. This is the $\eta$-law: a function is determined by what it does to arguments.

The $\eta$-law is the uniqueness principle for Π types. It says there's only one way to be an element of a Π type — you have to be a function. This corresponds to the categorical notion that a product is characterized by its projection maps.

In proof assistants, the $\eta$-law is sometimes *definitional* (it holds by computation, without proof) and sometimes *propositional* (it must be stated and proved). Martin-Löf's original theory has $\eta$ as definitional for Π types, which is convenient but makes type checking harder. HoTT includes $\eta$ definitionally for Π and Σ types.

## Polymorphic Identity and Dependent Types

The polymorphic identity function:
$$\mathsf{id} = \Lambda \alpha. \lambda x : \alpha. x$$

In System F, this has type $\forall \alpha. \alpha \to \alpha$. In dependent type theory, there's no separate type abstraction — we use Π:

$$\mathsf{id} = \lambda A. \lambda x. x : \prod_{A : \mathsf{Type}} A \to A$$

Here $A$ is just an argument (of type $\mathsf{Type}$), and the return type $A \to A$ depends on this argument. The type abstraction $\Lambda \alpha$ in System F becomes ordinary lambda abstraction $\lambda A$ in dependent type theory.

This is the unification of polymorphism and dependent types: in a dependently typed system, you don't need a separate "type abstraction" construct — quantifying over types is just a special case of Π types where the domain is a universe.

## Dependent Function Types as Universal Quantifiers

Under the Curry-Howard correspondence:

| Logic | Type Theory |
|---|---|
| Universal proposition $\forall x \in A, P(x)$ | $\prod_{x:A} B(x)$ |
| Proof of $\forall x \in A, P(x)$ | $f : \prod_{x:A} B(x)$ |
| Universal instantiation: $P(a)$ from $\forall x, P(x)$ | $f\, a : B(a)$ from $f : \prod_{x:A} B(x)$ |
| Universal introduction: to prove $\forall x, P(x)$... | Introduce $f = \lambda x. (\ldots)$ |

The Π type is literally the type-theoretic rendering of universal quantification. This isn't just an analogy — it's a precise correspondence. The typing rules for Π types are exactly the natural deduction rules for $\forall$ (intro and elim).

**Example.** Goldbach's conjecture becomes: prove there exists a term of type
$$\prod_{n:\mathbb{N}} (n > 2 \land n \text{ even}) \to \exists p\, q : \mathbb{P}, n = p + q$$

where $\exists$ is itself a Σ type (Section 3). The entire statement, including the quantifiers, is encoded in the type system.

## Functions into Type Families: Motive Argument

One subtle aspect of dependent types: when you pattern-match on an element of an inductive type and the *return type* depends on that element, you need to specify a *motive* — a type family that describes what type you're returning for each constructor.

For example, to define a function by recursion on $\mathbb{N}$ with a dependent return type:

$$\mathsf{rec}_{\mathbb{N}} : \prod_{P : \mathbb{N} \to \mathsf{Type}} P(0) \to \left(\prod_{n:\mathbb{N}} P(n) \to P(\mathsf{succ}(n))\right) \to \prod_{n:\mathbb{N}} P(n)$$

The first argument $P : \mathbb{N} \to \mathsf{Type}$ is the *motive* — the type family you want to produce. The recursor then has two cases: a base case in $P(0)$ and an inductive step taking you from $P(n)$ to $P(\mathsf{succ}(n))$, producing a function $\prod_{n:\mathbb{N}} P(n)$.

When $P$ is constant (say $P(n) = C$), this reduces to ordinary primitive recursion. But when $P$ is non-constant, you can define functions where the return type changes based on the index — like extracting the $k$-th element from a length-$n$ vector, where $k < n$ must be enforced at the type level.

## Implicit Arguments and Implicit Π

In practice, many type arguments can be *inferred* by the type checker. For instance, in `id A x`, the type $A$ can often be inferred from $x$. Most dependently typed languages allow *implicit arguments* — Pi types where the argument is filled in automatically.

In Lean 4:
```lean
def id : {A : Type} → A → A := fun x => x
-- The braces {} mark A as implicit
-- You write (id x) and Lean infers A from x
```

In Agda:
```agda
id : {A : Set} → A → A
id x = x
```

Formally, implicit arguments are still Π types — just with a notation that says "you don't have to write this argument explicitly." The type theory is the same; only the syntax differs.

## Dependent Currying

One elegant consequence of Π types is a dependent version of currying. In STLC:
$$A \times B \to C \cong A \to (B \to C)$$

In dependent type theory, with $B : A \to \mathsf{Type}$:
$$\left(\sum_{a:A} B(a)\right) \to C \cong \prod_{a:A} B(a) \to C$$

A function from a Σ type (dependent pair) to $C$ is equivalent to a function that takes the two components separately (the first component of type $A$, the second of type $B(a)$ depending on the first). This is the dependent generalization of currying.

This equivalence is given by:
- Forward: $f \mapsto \lambda a. \lambda b. f\, (a, b)$
- Backward: $g \mapsto \lambda p. g\, (\pi_1 p)\, (\pi_2 p)$

And these round-trip through $\beta$-reduction, giving definitional equality.

## Strong Normalization and Consistency

As with STLC and System F, Martin-Löf Type Theory (MLTT) with Π types, Σ types, and a universe hierarchy satisfies strong normalization: every well-typed term reduces to a unique normal form.

This is harder to prove for MLTT than for STLC, because:
1. Π types can quantify over universes (so types can be inputs to functions)
2. The computation rules interact with substitution in complex ways
3. Universes add stratification that must be respected throughout

The proof uses a generalization of the reducibility method (logical relations), carried out in a semantic setting. Strong normalization implies consistency: there's no closed term of type $\mathbf{0}$ (the empty type), so the logic is not trivial.

## Summary

The Π type is:
- The **generalization of function types** to the dependent case
- The **universal quantifier** under Curry-Howard
- The **section type** of a fibration in the geometric interpretation
- The **introduction** of true type-theoretic expressiveness

Every operation in the rest of dependent type theory will involve Π types: the recursor for inductive types has a Π type for its motive; the identity type's induction principle ($J$) has Π types everywhere; universes are inhabited by types, and functions between types are Π types.

The key rules to remember:
- **Formation:** $A : \mathsf{Type}$, $(x : A \vdash B(x) : \mathsf{Type})$ gives $\prod_{x:A} B(x) : \mathsf{Type}$
- **Introduction:** $(x : A \vdash t : B(x))$ gives $\lambda x. t : \prod_{x:A} B(x)$
- **Elimination:** $f : \prod_{x:A} B(x)$, $a : A$ gives $f\, a : B(a)$
- **Computation:** $(\lambda x. t)\, a \equiv t[a/x]$
- **Uniqueness:** $f \equiv \lambda x. f\, x$
