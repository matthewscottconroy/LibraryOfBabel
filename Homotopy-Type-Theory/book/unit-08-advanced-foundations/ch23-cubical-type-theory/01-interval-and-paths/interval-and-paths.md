# 23.1 The Interval and Paths

## Paths Are Functions: The Foundational Decision

Every design choice in cubical type theory flows from a single foundational decision: a path from $a$ to $b$ in type $A$ is a function from an interval to $A$ with specified endpoints. Not an inductive type. Not an atom. A function. This is the move that makes computation possible.

In Martin-Löf type theory, the identity type $a =_A b$ is generated inductively by reflexivity. The only canonical element is `refl : a = a`, and J tells you how to eliminate: to prove a property $P$ of all paths, prove $P(\text{refl})$ and get the rest for free by the inductive structure. This is elegant, but it makes paths opaque. You cannot look inside `ua(e)` and find a computation. The path is a formal object, not a function.

Cubical type theory changes the ontology. A path is a map out of a primitive interval type $\mathbb{I}$. The interval is not defined from other constructions — it is added to the theory as a new primitive sort. And its algebra is specifically designed so that every path operation you want — reversal, concatenation, higher coherences — can be expressed as a computation on functions.

## The Interval $\mathbb{I}$: A De Morgan Algebra

The interval $\mathbb{I}$ is a new *sort* in the type theory, separate from the type universe. This distinction matters: you cannot form a $\Pi$-type over $\mathbb{I}$ as if $\mathbb{I}$ were a type, but you can *parametrize* constructions by elements of $\mathbb{I}$. Dimension variables $i, j, k : \mathbb{I}$ range over the interval.

The interval carries a De Morgan algebra structure:

$$0, 1 : \mathbb{I}$$
$$\sim : \mathbb{I} \to \mathbb{I} \quad \text{(complement)}$$
$$\wedge, \vee : \mathbb{I} \times \mathbb{I} \to \mathbb{I} \quad \text{(meet and join)}$$

satisfying the laws:

| Law | Equation |
|-----|----------|
| Complement | $\sim 0 = 1$, $\sim 1 = 0$, $\sim(\sim i) = i$ |
| Meet identity | $i \wedge 0 = 0$, $i \wedge 1 = i$, $i \wedge i = i$ |
| Join identity | $i \vee 0 = i$, $i \vee 1 = 1$, $i \vee i = i$ |
| De Morgan | $\sim(i \wedge j) = \sim i \vee \sim j$ |
| De Morgan | $\sim(i \vee j) = \sim i \wedge \sim j$ |
| Complement laws | $i \wedge \sim i = 0$, $i \vee \sim i = 1$ |

This is exactly the structure of a De Morgan algebra — a distributive lattice with a De Morgan involution. It is not a Boolean algebra, because we do not have the law $i = 0$ or $i = 1$ for all $i$ (excluded middle for interval points). The interval has genuinely interior points.

Why De Morgan? Each operation serves a purpose:

- **Complement $\sim i$**: Path reversal. Given $p : a =_A b$, the reversed path is $\lambda i. \, p(\sim i)$. Check: $(\lambda i. \, p(\sim i))(0) = p(\sim 0) = p(1) = b$ and at endpoint 1, $p(\sim 1) = p(0) = a$. Reversal is a definition, not an axiom.

- **Meet $i \wedge j$**: Left connections. The term $\lambda i\, j. \, p(i \wedge j)$ is a 2-cube witnessing the left unit law: $\text{refl} \cdot p \sim p$.

- **Join $i \vee j$**: Right connections. The term $\lambda i\, j. \, p(i \vee j)$ witnesses the right unit law: $p \cdot \text{refl} \sim p$.

Without complement, path reversal would not be definitional — this is exactly the situation in Cartesian cubical type theory (Section 4). Without meet and join, the groupoid coherences would not have explicit cubical witnesses.

## Dimension Variables and Contexts

The typing context in CCHM extends to include dimension variables:

$$\Gamma ::= \cdot \mid \Gamma, x : A \mid \Gamma, i : \mathbb{I}$$

A *dimension variable* $i : \mathbb{I}$ is conceptually like a free real parameter ranging over the unit interval. In a context containing $i : \mathbb{I}$, a type $A$ may depend on $i$ — giving a family of types parametrized by position in the interval. Such a type $A$ in context $\Gamma, i : \mathbb{I}$ is a path of types: at each $i$, you get a type $A(i)$.

The product of $n$ dimension variables gives an $n$-cube:
- $i : \mathbb{I}$ — a 1-cube (a path, or directed edge)
- $i, j : \mathbb{I}$ — a 2-cube (a square, or homotopy between paths)  
- $i, j, k : \mathbb{I}$ — a 3-cube (a cube, or homotopy between homotopies)

Every dimension variable ranges over the same interval $\mathbb{I}$, and the algebraic operations on $\mathbb{I}$ let you navigate within and between cubes.

## Face Formulas

A *face formula* $\phi$ specifies a subset of cube faces — the portion of the cube where some constraint holds. The grammar:

$$\phi ::= 0 \mid 1 \mid (i = 0) \mid (i = 1) \mid \phi \wedge \psi \mid \phi \vee \psi$$

Think of $\phi$ as a boolean predicate on points $(i_1, \ldots, i_n) \in \mathbb{I}^n$. A face formula evaluates to 1 at the points where it holds, and specifies a face of the cube.

Examples:
- $(i = 0)$: the left face — the sub-cube where the $i$-coordinate is 0
- $(i = 1)$: the right face
- $(i = 0) \vee (j = 1)$: the union of left $i$-face and top $j$-face (an open box, missing the top or bottom)
- $(i = 0) \wedge (j = 0)$: the corner where both $i$ and $j$ are 0
- $1$: the entire cube
- $0$: the empty face (nothing)

Face formulas are the grammatical device that lets you say "this element is defined on this part of the cube." They are the precision instrument that makes partial elements work.

## Partial Elements

A *partial element* of type $A$ under face formula $\phi$ is a term of type $A$ that is only defined when $\phi = 1$. We write the type of partial elements as $[\phi \vdash A]$.

Notation for partial elements uses a case-expression syntax:
$$[i = 0 \mapsto a_0, \; i = 1 \mapsto a_1]$$
for an element that equals $a_0$ at the left endpoint and $a_1$ at the right endpoint.

The key rule: if $u : [\phi \vdash A]$ and $v : [\psi \vdash A]$, and they agree on the overlap (when both $\phi$ and $\psi$ hold), then we can write their union $[u, v] : [\phi \vee \psi \vdash A]$.

Partial elements are the correct notion of "boundary data": they specify what a path, square, or cube must look like on part of its boundary, without committing to the whole.

## The Extension Type

Given a partial element $u : [\phi \vdash A]$ and a full type $A$, the *extension type* is the type of completions:

$$\langle A \mid \phi \mapsto u \rangle$$

An element of this type is a term $a : A$ such that $a = u$ whenever $\phi = 1$. The extension type is the type-theoretic way to say "an element that extends this boundary data."

Extension types appear throughout cubical type theory:
- The path type $a =_A b$ is the extension type $\langle \mathbb{I} \to A \mid (i=0) \mapsto a, \, (i=1) \mapsto b \rangle$
- Homotopies, coherences, and horn-fillings are all instances of extension types

## Path Types as Function Spaces

With these tools in place, the path type $a =_A b$ is defined:

$$a =_A b \;\;:\equiv\;\; \langle \mathbb{I} \to A \mid (i=0) \mapsto a, \, (i=1) \mapsto b \rangle$$

An element of $a =_A b$ is a function $p : \mathbb{I} \to A$ with $p(0) = a$ and $p(1) = b$ — not propositionally equal, not up to a further path, but *definitionally*. The type checker knows that $p(0)$ is literally $a$ by computation.

**Reflexivity**: $\text{refl}_a :\equiv \lambda i. \, a : a =_A a$

**Symmetry**: $\text{sym}(p) :\equiv \lambda i. \, p(\sim i) : b =_A a$ when $p : a =_A b$

**Computation rules** (definitional):
- $(\lambda i. \, t)[0/i] = t[0/i]$
- $(\lambda i. \, t)[1/i] = t[1/i]$
- $p(0) = a$ for any $p : a =_A b$
- $p(1) = b$ for any $p : a =_A b$

These are not propositional equalities — they are definitional reductions. The type checker applies them automatically.

## Higher-Dimensional Paths

Two dimension variables $i, j : \mathbb{I}$ give a 2-cube. A term $H : \mathbb{I}^2 \to A$ (a function of two interval variables) is a square in $A$:

- $H(0, j)$: the left face (a path from $H(0,0)$ to $H(0,1)$)
- $H(1, j)$: the right face
- $H(i, 0)$: the bottom face
- $H(i, 1)$: the top face

A homotopy between paths $p, q : a =_A b$ is a 2-cube $H$ with $H(0, j) = a$, $H(1, j) = b$, $H(i, 0) = p(i)$, $H(i, 1) = q(i)$. This is the path type $p =_{a =_A b} q$ — paths between paths.

In the cubical setting, this is no longer an abstract concept requiring J-induction to understand. A 2-cube is literally a bivariate function $\mathbb{I}^2 \to A$. You can write it down, evaluate it at corners, and compute with it.

## Dependent Path Types (PathP)

For families of types $B : \mathbb{I} \to \mathsf{Type}$ — paths in the universe — we need *dependent* paths:

$$\mathsf{PathP}(B, a_0, a_1) :\equiv \langle (i : \mathbb{I}) \to B(i) \mid (i=0) \mapsto a_0, \, (i=1) \mapsto a_1 \rangle$$

An element of $\mathsf{PathP}(B, a_0, a_1)$ is a function $p : (i : \mathbb{I}) \to B(i)$ with $p(0) = a_0 : B(0)$ and $p(1) = a_1 : B(1)$. The endpoints live in different types ($B(0)$ and $B(1)$), and the path interpolates between them as the type changes.

In Cubical Agda:
```agda
PathP : (B : I → Type ℓ) → B i0 → B i1 → Type ℓ
```

`PathP` is used throughout Cubical Agda whenever transport gives you an element that must be compared across different fibers. Non-dependent paths are the special case: `(a =_A b) ≡ PathP (λ _ → A) a b`.

## J Is Derived, Not Primitive

In MLTT, path induction (J) is a primitive elimination rule: the only tool for reasoning about identity types. In cubical type theory, J is *derived* from the path type definition and transport.

**Theorem.** For any motive $P : (b : A) \to a =_A b \to \mathsf{Type}$ and base case $d : P(a, \text{refl})$, there is a function $J(P, d) : (b : A) \to (p : a =_A b) \to P(b, p)$ with $J(P, d)(a, \text{refl}) = d$ definitionally.

The proof uses `transp`: transport $d : P(a, \text{refl})$ along the path of types $\lambda i. \, P(p(i), \lambda j. \, p(i \wedge j))$. This requires the Kan filling operation of the next section, but the point is that J falls out of the computational structure — it is not imposed from outside.

The J computation rule $J(P, d)(a, \text{refl}) = d$ holds *definitionally* in cubical type theory. This is not a coincidence: it is a consequence of the fact that `refl` is $\lambda i. \, a$, transport along a constant path is the identity, and the composition rules are defined to make this work.

## Why the Interval Design Works

The De Morgan algebra on $\mathbb{I}$, the face formula language, and the definition of paths as functions compose into a single coherent design. The payoff:

1. **Endpoints compute**: $p(0)$ and $p(1)$ reduce to actual values. No stuck terms.
2. **Reversal is a definition**: `sym p = λ i → p (~ i)`, using complement.
3. **ap is composition of functions**: $\text{ap}_f(p) = \lambda i. \, f(p(i))$. Apply $f$ pointwise.
4. **Homotopies are bivariate functions**: $H : \mathbb{I}^2 \to A$, written down explicitly.
5. **Coherences have explicit witnesses**: Meet and join give the connection types that witness unit laws.

The CCHM interval is not arbitrary. It is the minimal algebra that makes all of path theory computational, chosen with surgical precision.
