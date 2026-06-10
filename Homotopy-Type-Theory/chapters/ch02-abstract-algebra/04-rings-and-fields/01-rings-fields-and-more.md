# 4.1 Rings, Fields, and Beyond

## From One Operation to Two

Groups have one operation. But many mathematical structures we care about — integers, polynomials, matrices, functions — come with *two* operations: something like addition and multiplication. The study of these structures is the next layer of algebra beyond group theory.

The central objects here are *rings* and *fields*. They extend the group axioms to two operations, with each operation satisfying its own rules and the two operations interacting via distributivity.

## Rings

**Definition (Ring).** A *ring* $(R, +, \cdot)$ is a set $R$ with two binary operations, $+$ (addition) and $\cdot$ (multiplication), such that:

1. $(R, +)$ is an abelian group (with additive identity $0$ and additive inverses $-r$)
2. $\cdot$ is associative: $(r \cdot s) \cdot t = r \cdot (s \cdot t)$
3. $\cdot$ distributes over $+$: $r \cdot (s + t) = r \cdot s + r \cdot t$ and $(r + s) \cdot t = r \cdot t + s \cdot t$

Many authors also require a multiplicative identity $1$ (a *unital* ring or *ring with unity*). When we say "ring" here, we assume unitality: there exists $1 \in R$ with $1 \cdot r = r \cdot 1 = r$.

**What's not required:**
- Multiplicative inverses (unlike a group, you can't generally "divide" in a ring)
- Commutativity of multiplication ($rs = sr$ is a special property)

**Standard examples of rings:**
- $(\mathbb{Z}, +, \cdot)$: the integers. The prototype ring.
- $(\mathbb{Z}/n\mathbb{Z}, +, \cdot)$: integers mod $n$.
- $(\mathbb{Q}, +, \cdot)$, $(\mathbb{R}, +, \cdot)$, $(\mathbb{C}, +, \cdot)$: number fields (also fields, below).
- $M_n(\mathbb{R})$: $n \times n$ real matrices under matrix addition and multiplication. Non-commutative for $n \geq 2$.
- $\mathbb{Z}[x]$: polynomials with integer coefficients. Addition and multiplication of polynomials.
- $C([0,1])$: continuous real-valued functions on $[0,1]$ with pointwise operations. A commutative ring.
- Any group ring $\mathbb{Z}[G]$ for a group $G$.

**Immediate consequences of the ring axioms:**
- $0 \cdot r = 0$ for all $r$ (multiply the equation $0 + 0 = 0$ on the right by $r$)
- $(-r) \cdot s = -(r \cdot s)$ (additive inverse distributes through multiplication)
- $(-1) \cdot r = -r$ (if the ring has unity)
- $0 = 1$ implies $R = \{0\}$ (the *zero ring*)

## Special Classes of Rings

Rings can have additional properties:

**Commutative ring:** $rs = sr$ for all $r, s$. Examples: $\mathbb{Z}$, $\mathbb{Z}[x]$, $\mathbb{R}$.

**Integral domain:** A commutative ring with $1 \neq 0$ (nontrivial) and no *zero divisors* — that is, $rs = 0$ implies $r = 0$ or $s = 0$. Examples: $\mathbb{Z}$, $\mathbb{Z}[x]$, any field.

In an integral domain, you can cancel: $rs = rt$ and $r \neq 0$ implies $s = t$ (multiply both sides by $r^{-1}$... but wait, $r^{-1}$ might not exist in a ring! Instead: $r(s-t) = 0$ and $r \neq 0$ implies $s - t = 0$).

**Division ring (skew field):** Every nonzero element has a multiplicative inverse. Examples: $\mathbb{Q}, \mathbb{R}, \mathbb{C}$, and also the *quaternions* $\mathbb{H}$ (a non-commutative division ring).

## Fields

**Definition (Field).** A *field* is a commutative ring $(F, +, \cdot)$ in which every nonzero element has a multiplicative inverse: for each $r \neq 0$, there exists $r^{-1}$ with $r \cdot r^{-1} = 1$.

Equivalently: $(F, +)$ is an abelian group, $(F \setminus \{0\}, \cdot)$ is an abelian group, and multiplication distributes over addition.

**Examples:**
- $\mathbb{Q}$: the rationals. The smallest field of characteristic 0.
- $\mathbb{R}$: the reals.
- $\mathbb{C}$: the complex numbers.
- $\mathbb{F}_p = \mathbb{Z}/p\mathbb{Z}$ for prime $p$: the *finite field* with $p$ elements.
- $\mathbb{F}_{p^n}$: finite fields with $p^n$ elements (exist for every prime power $p^n$, unique up to isomorphism).
- $\mathbb{Q}(\sqrt{2}) = \{a + b\sqrt{2} \mid a, b \in \mathbb{Q}\}$: a *number field*.

**The characteristic.** The *characteristic* of a field $F$ is the smallest positive integer $n$ with $n \cdot 1 = 0$ (where $n \cdot 1 = \underbrace{1 + \cdots + 1}_{n}$), or $0$ if no such $n$ exists. It's always a prime or $0$:
- $\text{char}(\mathbb{Q}) = \text{char}(\mathbb{R}) = \text{char}(\mathbb{C}) = 0$
- $\text{char}(\mathbb{F}_p) = p$
- Any field of characteristic $p > 0$ has $\mathbb{F}_p$ as a subfield.

## Why Rings and Fields Matter Here

You might wonder why we need rings and fields in a course on Homotopy Type Theory. Several reasons:

**Homology groups and cohomology rings.** Algebraic topology assigns to each topological space a sequence of *homology groups* $H_n(X)$ (measuring "$n$-dimensional holes") and a *cohomology ring* $H^*(X)$ (a ring, not just a group, because cohomology has a multiplicative structure called the *cup product*).

For example:
- $H_0(S^1) = \mathbb{Z}$, $H_1(S^1) = \mathbb{Z}$, all higher $H_n = 0$.
- $H^*(T^2; \mathbb{Z}) = \mathbb{Z}[\alpha, \beta] / (\alpha^2, \beta^2)$ (exterior algebra): the cohomology ring of the torus, capturing the intersection structure of cycles.
- Over a field $k$, the cohomology ring $H^*(X; k)$ is a graded $k$-algebra, and its algebraic structure encodes geometric information.

**Linear algebra over fields.** Vector spaces are modules over fields, and linear algebra is the study of these. Many geometric and topological constructions involve linearization: the tangent space at a point is a vector space, homology with field coefficients is a vector space, etc.

**Polynomial rings.** The ring $\mathbb{Z}[x]$ (or $\mathbb{R}[x]$, etc.) of polynomials is the free commutative ring on one generator, analogous to the free group. Understanding polynomial rings is foundational for algebraic geometry (studying spaces defined by polynomial equations).

**Galois theory.** The relationship between field extensions and groups (Galois groups) is one of the deepest connections in mathematics. Galois theory explains why the quintic polynomial is not solvable by radicals — using the language of groups and fields together.

## Ideals and Quotient Rings

Just as groups have normal subgroups, rings have *ideals* — the right notion of "substructure to mod out by."

**Definition (Ideal).** A subset $I \subseteq R$ is a (*two-sided*) *ideal* if:
1. $(I, +)$ is a subgroup of $(R, +)$
2. $rI \subseteq I$ and $Ir \subseteq I$ for all $r \in R$ (i.e., multiplying by any ring element stays in $I$)

**Quotient ring.** If $I$ is an ideal of $R$, the quotient $R/I$ is the set of cosets $\{r + I\}$ with $(r + I) + (s + I) = (r + s) + I$ and $(r + I)(s + I) = (rs) + I$. This is the ring analog of a quotient group.

**Example.** In $\mathbb{Z}$, ideals are exactly $n\mathbb{Z}$ for $n \geq 0$. The quotient $\mathbb{Z}/n\mathbb{Z}$ is the ring of integers mod $n$.

**Example.** In $\mathbb{R}[x]$, the ideal $(x^2 + 1)$ consists of all multiples of $x^2 + 1$. The quotient $\mathbb{R}[x]/(x^2 + 1) \cong \mathbb{C}$ (since we're "declaring" $x^2 = -1$, so $x$ plays the role of $i$). This is how you construct the complex numbers from the reals.

The *First Isomorphism Theorem for Rings* holds analogously: if $\phi: R \to S$ is a ring homomorphism (preserving both operations and the identity), then $R/\ker\phi \cong \text{Im}(\phi)$.

## Modules

A *module* over a ring $R$ is an abelian group $(M, +)$ with a *scalar multiplication* $R \times M \to M$ satisfying the usual axioms (linearity, distributivity, etc.). Modules generalize vector spaces: a module over a field is exactly a vector space.

Modules over $\mathbb{Z}$ are just abelian groups — every abelian group is a $\mathbb{Z}$-module.

Homology groups of topological spaces are often viewed as $\mathbb{Z}$-modules (or modules over other rings). The module structure carries geometric information.

## A Brief Dictionary

Here's how the ring-theory concepts parallel group theory:

| Group theory | Ring theory |
|---|---|
| Group $G$ | Ring $R$ |
| Abelian group | Commutative ring |
| Normal subgroup | Ideal |
| Quotient group $G/N$ | Quotient ring $R/I$ |
| Group homomorphism | Ring homomorphism |
| First isomorphism theorem | First isomorphism theorem |
| Free group $F(S)$ | Free commutative ring = $\mathbb{Z}[S]$ (polynomial ring) |
| Cayley's theorem | Cayley-Hamilton theorem (for modules) |

The parallel structure is not a coincidence — both theories are instances of *universal algebra*, which studies algebraic structures axiomatically and proves theorems about all of them simultaneously.

In category theory, groups, rings, modules, and more are all *algebras* over *monads* in appropriate categories. The isomorphism theorems hold at this level of generality. And in HoTT, these algebraic structures are studied as types with additional operations, with the isomorphism theorems becoming theorems about type equivalences.
