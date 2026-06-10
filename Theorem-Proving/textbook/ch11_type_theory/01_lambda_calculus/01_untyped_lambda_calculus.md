# Untyped Lambda Calculus

> "I believe in calculation. When I prove a theorem, I want to know that the calculation is right, not that I feel good about it."
> — Alonzo Church

## A Language for Pure Computation

In the early 1930s, Alonzo Church was trying to create a formal foundation for mathematics — a system in which all mathematical objects and functions could be represented. His approach: **lambda calculus**, a tiny formal language with three kinds of expressions and two kinds of computation steps.

Church's original goal (a complete foundation for mathematics) ran into trouble — the pure system was too powerful and led to paradoxes. But as a language for expressing *computable functions*, it turned out to be perfect. When Turing independently developed Turing machines and showed that Church's lambda calculus computes exactly the same functions, it was recognized as a profound discovery: **computation has a universal, abstract nature** independent of the particular formalism.

Today, lambda calculus is:
- The theoretical core of functional programming (Haskell, ML, OCaml, Scala)
- The foundation for type theory and proof assistants (Lean 4, Coq, Agda)
- The standard model for the semantics of programming languages
- Half of the Curry-Howard correspondence (the other half being logic)

## The Syntax: Three Constructs

Lambda calculus has the smallest possible syntax:

$$e ::= x \quad|\quad \lambda x.\, e \quad|\quad e\; e$$

- **Variable** $x$: a name, representing an unspecified value
- **Abstraction** $\lambda x.\, e$: a function with parameter $x$ and body $e$
  - "Given $x$, compute $e$"
  - This is anonymous: $\lambda x.\, x + 1$ is the successor function without a name
- **Application** $e_1\; e_2$: apply function $e_1$ to argument $e_2$

**Parsing conventions**:
- Application is left-associative: $f\; x\; y = (f\; x)\; y$
- Abstraction extends as far right as possible: $\lambda x.\, f\; x = \lambda x.\, (f\; x)$, not $(\lambda x.\, f)\; x$
- Consecutive abstractions: $\lambda x y.\, e = \lambda x.\, \lambda y.\, e$ (currying)

Every function in lambda calculus is a **unary function** of one argument. Multi-argument functions are *curried*: $f(x, y)$ becomes $\lambda x.\, \lambda y.\, e$, applied as $(f\; a)\; b$.

## Free and Bound Variables

In $\lambda x.\, e$, the variable $x$ is **bound** — it is a placeholder whose name is arbitrary. In $\lambda x.\, y$, the variable $y$ is **free** — it refers to something outside the expression.

**Example**: In $\lambda x.\, (x\; y)$, $x$ is bound and $y$ is free.

**$\alpha$-equivalence**: Two expressions are $\alpha$-equivalent (the same up to renaming of bound variables) if one can be obtained from the other by consistently renaming bound variables:
$$\lambda x.\, x \; =_\alpha \; \lambda y.\, y \; =_\alpha \; \lambda z.\, z$$

These are all "the identity function" — the name of the parameter does not matter.

## Reduction: How Computation Happens

**$\beta$-reduction** is the fundamental computation step — applying a function to an argument:
$$(\lambda x.\, e)\; t \;\longrightarrow_\beta\; e[t/x]$$

where $e[t/x]$ means "substitute $t$ for all free occurrences of $x$ in $e$," being careful to rename bound variables if necessary (to avoid **variable capture**).

**Example**:
$$(\lambda x.\, x\; x)\; (\lambda y.\, y) \;\longrightarrow_\beta\; (\lambda y.\, y)\; (\lambda y.\, y) \;\longrightarrow_\beta\; \lambda y.\, y$$

The redex (reducible expression) is a $\beta$-redex; one application of $\beta$-reduction is a **reduction step**.

**$\eta$-reduction**: $\lambda x.\, (f\; x) \;\longrightarrow_\eta\; f$ (if $x \notin \text{FV}(f)$)

This says: a function that simply passes its argument to $f$ is the same as $f$ itself. Useful for simplification.

A term with no $\beta$-redexes is in **$\beta$-normal form** — it cannot be reduced further. Not every term has a normal form: the term $\Omega = (\lambda x.\, x\; x)\; (\lambda x.\, x\; x)$ reduces to itself and loops forever.

## Church Encodings: Computing with Pure Functions

The remarkable fact: lambda calculus can represent *all* mathematical structures using nothing but functions. There are no primitive data types — only functions.

**Booleans**:
$$\text{TRUE} = \lambda t.\, \lambda f.\, t$$
$$\text{FALSE} = \lambda t.\, \lambda f.\, f$$
$$\text{IF} = \lambda b.\, \lambda t.\, \lambda f.\, b\; t\; f$$

Notice: $\text{IF}\; \text{TRUE}\; A\; B \to \text{TRUE}\; A\; B = (\lambda t.\, \lambda f.\, t)\; A\; B \to A$. The boolean $\text{TRUE}$ *is* the function that selects its first argument.

**Natural numbers** (Church numerals):
$$\bar{0} = \lambda f.\, \lambda x.\, x$$
$$\bar{1} = \lambda f.\, \lambda x.\, f\; x$$
$$\bar{2} = \lambda f.\, \lambda x.\, f\; (f\; x)$$
$$\bar{n} = \lambda f.\, \lambda x.\, \underbrace{f\; (f\; (\cdots (f}_{n}\; x)\cdots))$$

A Church numeral $\bar{n}$ is the function that applies its argument $f$ exactly $n$ times to $x$. This is iterated application as a value.

**Arithmetic**:
$$\text{SUCC} = \lambda n.\, \lambda f.\, \lambda x.\, f\; (n\; f\; x)$$
$$\text{ADD} = \lambda m.\, \lambda n.\, \lambda f.\, \lambda x.\, m\; f\; (n\; f\; x)$$
$$\text{MULT} = \lambda m.\, \lambda n.\, \lambda f.\, m\; (n\; f)$$

**Pairs**:
$$\text{PAIR} = \lambda x.\, \lambda y.\, \lambda s.\, s\; x\; y$$
$$\text{FST} = \lambda p.\, p\; \text{TRUE}$$
$$\text{SND} = \lambda p.\, p\; \text{FALSE}$$

## Fixed Points and Recursion

Can lambda calculus express recursion? Seemingly not: a lambda abstraction $\lambda x.\, e$ has no name, so the body $e$ cannot refer to the function itself.

The solution is the **fixed-point combinator**:
$$Y = \lambda f.\, (\lambda x.\, f\; (x\; x))\; (\lambda x.\, f\; (x\; x))$$

**The key property**: $Y\; F \;\longrightarrow_\beta\; F\; (Y\; F)$ — $Y\; F$ is a fixed point of $F$.

This means: $Y$ can compute the fixed point of any function $F$. If $F$ encodes a recursive definition ("to compute the factorial of $n$, multiply $n$ by the factorial of $n-1$"), then $Y\; F$ is the actual factorial function.

```haskell
-- In Haskell (which is based on lambda calculus):
-- Every recursive function implicitly uses a fixed-point combinator

-- Without fix:
factorial :: Int -> Int
factorial = fix (\rec n -> if n == 0 then 1 else n * rec (n - 1))
  where fix f = let x = f x in x

-- With explicit recursion (Haskell allows self-reference by name):
factorial' :: Int -> Int
factorial' 0 = 1
factorial' n = n * factorial' (n - 1)
```

The $Y$ combinator makes recursion *derivable* — it is not a primitive; it emerges from the basic combinatorial structure of lambda calculus.

## The Church-Turing Thesis

Lambda calculus and Turing machines define the same class of computable functions. This is a theorem — proven by showing that any Turing machine can be encoded as a lambda expression and vice versa.

But the **Church-Turing Thesis** goes further: it claims that *any* function computable by any reasonable formal model of computation is already computable by Turing machines (and hence by lambda calculus). This is not a theorem but a *thesis* — a claim about the nature of computation itself. It has resisted all known counterexamples and is universally accepted.

The Church-Turing Thesis means lambda calculus is not just a theoretical curiosity — it is a universal model of computation. Every algorithm ever conceived, every program ever written, can in principle be expressed as a lambda term.

## Connection to Modern Programming

Haskell's type system is directly derived from the typed lambda calculus. Lean 4 and Coq are proof assistants whose proof terms *are* lambda expressions. The Curry-Howard correspondence (ch11/02) shows that:
- Lambda terms = proofs
- Types = propositions
- $\beta$-reduction = proof normalization

When you write a Lean proof, you are writing a lambda term. When Lean verifies the proof, it $\beta$-reduces and type-checks the term.

```lean
-- In Lean 4, functions literally are lambda terms:
def id_fun : α → α := fun x => x
-- This is λx.x

def compose : (β → γ) → (α → β) → (α → γ) := fun f g x => f (g x)
-- This is λf.λg.λx. f (g x) — the S combinator in combinatory logic
```

## Exercises
See [problems/ch11_type_theory/01_lambda_calculus_exercises.md](../../../problems/ch11_type_theory/01_lambda_calculus_exercises.md)
