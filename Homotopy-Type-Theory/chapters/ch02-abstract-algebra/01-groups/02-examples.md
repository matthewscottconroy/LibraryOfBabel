# 1.2 Examples of Groups

## Why Examples Matter

Definitions in mathematics are only as useful as the examples that flesh them out. The group axioms are short, but they apply to an enormous range of structures. The examples in this section are not decoration — each one illuminates a different aspect of the definition and will appear again later in the curriculum. Some are abelian and some are not; some are finite and some are infinite; some are familiar and some are surprising.

Pay attention to what's the same across all examples: the structure. Pay attention to what differs: the particular operation, the specific elements, the presence or absence of commutativity. This interplay between the abstract and the concrete is the heart of algebra.

## Number Systems Under Addition

The simplest groups come from number systems.

**The integers $(\mathbb{Z}, +)$.** The integers $\{\ldots, -2, -1, 0, 1, 2, \ldots\}$ under addition form a group. The identity is $0$ (adding zero doesn't change anything), and the inverse of $n$ is $-n$. Addition is associative and commutative, so this is an abelian group. It's infinite.

The integers are particularly special: they are the *free abelian group on one generator*. Every element is a sum of copies of $1$ or $-1$, and there are no constraints on how many copies you can use. We'll see this concept formalized when we discuss free groups.

**Integers modulo $n$: $(\mathbb{Z}/n\mathbb{Z}, +)$.** Fix a positive integer $n$. Define $\mathbb{Z}/n\mathbb{Z}$ as the set $\{[0], [1], [2], \ldots, [n-1]\}$ where $[k]$ represents the residue class of $k$ modulo $n$. Addition is defined by $[j] + [k] = [j+k \bmod n]$.

For example, in $\mathbb{Z}/5\mathbb{Z}$: $[3] + [4] = [7 \bmod 5] = [2]$. The identity is $[0]$, and the inverse of $[k]$ is $[n-k]$ (since $[k] + [n-k] = [n] = [0]$).

This is the algebraic structure underlying clock arithmetic: hours on a 12-hour clock live in $\mathbb{Z}/12\mathbb{Z}$.

$\mathbb{Z}/n\mathbb{Z}$ is abelian and finite (order $n$). These groups are called *cyclic groups* because every element can be written as a multiple of a single generator: $[k] = k \cdot [1]$.

**Rationals, reals, complex numbers under addition.** $(\mathbb{Q}, +)$, $(\mathbb{R}, +)$, and $(\mathbb{C}, +)$ are all abelian groups. The identity is $0$ and the inverse of $x$ is $-x$. These are the "obvious" groups that students encounter before formally meeting the definition.

## Number Systems Under Multiplication

Multiplication is trickier because $0$ has no multiplicative inverse.

**Nonzero rationals/reals/complexes under multiplication.** $(\mathbb{Q}^*, \cdot)$, $(\mathbb{R}^*, \cdot)$, $(\mathbb{C}^*, \cdot)$ — the nonzero elements of each number system under multiplication. Identity is $1$, inverse of $x$ is $1/x$. All abelian, all infinite.

**Integers modulo $n$, nonzero, under multiplication — sometimes.** This is subtle. In $\mathbb{Z}/n\mathbb{Z}$, multiplication is well-defined: $[j] \cdot [k] = [jk \bmod n]$. But does every nonzero element have an inverse? Only if $n$ is prime. In $\mathbb{Z}/6\mathbb{Z}$, the element $[2]$ has no inverse: $[2] \cdot [1] = [2]$, $[2] \cdot [2] = [4]$, $[2] \cdot [3] = [0]$, $[2] \cdot [4] = [2]$, $[2] \cdot [5] = [4]$ — never $[1]$. But in $\mathbb{Z}/5\mathbb{Z}$ (prime), every nonzero element has an inverse: $[2] \cdot [3] = [1]$, $[3] \cdot [2] = [1]$, etc.

When $n = p$ is prime, $(\mathbb{Z}/p\mathbb{Z})^* = \{[1], [2], \ldots, [p-1]\}$ under multiplication is a group of order $p-1$. This is fundamental in number theory and cryptography (RSA, discrete logarithm problems).

## Permutation Groups

**The symmetric group $S_n$.** A *permutation* of $\{1, 2, \ldots, n\}$ is a bijection $\sigma : \{1, \ldots, n\} \to \{1, \ldots, n\}$. The set of all such permutations, under composition, forms the *symmetric group* $S_n$. It has order $n!$.

The identity is the identity function. The inverse of a permutation $\sigma$ is the inverse function $\sigma^{-1}$. Composition of functions is associative.

For $n \geq 3$, $S_n$ is non-abelian. Here's why: consider $S_3$, permutations of $\{1, 2, 3\}$. Let $\sigma = (1\ 2)$ (swap 1 and 2, fix 3) and $\tau = (1\ 2\ 3)$ (send $1 \mapsto 2, 2 \mapsto 3, 3 \mapsto 1$). Then:

$\sigma \circ \tau$: $1 \xrightarrow{\tau} 2 \xrightarrow{\sigma} 1$, $2 \xrightarrow{\tau} 3 \xrightarrow{\sigma} 3$, $3 \xrightarrow{\tau} 1 \xrightarrow{\sigma} 2$. So $\sigma\tau = (2\ 3)$.

$\tau \circ \sigma$: $1 \xrightarrow{\sigma} 2 \xrightarrow{\tau} 3$, $2 \xrightarrow{\sigma} 1 \xrightarrow{\tau} 2$, $3 \xrightarrow{\sigma} 3 \xrightarrow{\tau} 1$. So $\tau\sigma = (1\ 3)$.

Since $(2\ 3) \neq (1\ 3)$, we have $\sigma\tau \neq \tau\sigma$, confirming $S_3$ is non-abelian.

The symmetric group is the most fundamental non-abelian example, and it appears everywhere: solving polynomial equations (Galois theory), classifying crystals, analyzing algorithm complexity.

**The alternating group $A_n$.** Every permutation can be written as a product of 2-element swaps (transpositions), and while this decomposition is not unique, the *parity* of the number of transpositions is always the same. Permutations requiring an even number of transpositions are called *even permutations*, and they form a subgroup $A_n \leq S_n$ of order $n!/2$ (for $n \geq 2$).

$A_n$ is normal in $S_n$ (we'll explain what this means shortly) and for $n \geq 5$, $A_n$ is *simple* — it has no proper normal subgroups. This is the key fact behind the unsolvability of the general quintic polynomial.

## Matrix Groups

**The general linear group $\text{GL}_n(\mathbb{R})$.** The set of all invertible $n \times n$ real matrices under matrix multiplication. The identity is the $n \times n$ identity matrix $I_n$, and the inverse of $M$ is the matrix inverse $M^{-1}$.

Matrix multiplication is associative (it represents composition of linear maps). This group is non-abelian for $n \geq 2$ (matrix multiplication doesn't commute in general). It's infinite and non-compact (matrices can have arbitrarily large entries).

**Special linear group $\text{SL}_n(\mathbb{R}) = \{M \in \text{GL}_n(\mathbb{R}) \mid \det(M) = 1\}$.** Matrices with determinant 1. This is a subgroup of $\text{GL}_n(\mathbb{R})$, because $\det(AB) = \det(A)\det(B)$ and $\det(I) = 1$.

**Orthogonal group $O(n)$.** Matrices $M$ with $M^T M = I$ (orthogonal matrices). These represent rotations and reflections in $\mathbb{R}^n$.

**Special orthogonal group $SO(n)$.** Orthogonal matrices with $\det = 1$, i.e., the rotation group. $SO(2)$ is the circle group (rotations of the plane), and $SO(3)$ is the rotation group of 3-dimensional space — fundamental in physics.

These matrix groups are *Lie groups* — groups that are also smooth manifolds (geometric objects with calculus). They provide the bridge between algebra and differential geometry.

## The Dihedral Group

**Dihedral group $D_n$.** The symmetry group of a regular $n$-gon. It consists of:
- $n$ rotations: by $0°, 360°/n, 2 \cdot 360°/n, \ldots, (n-1) \cdot 360°/n$
- $n$ reflections: along $n$ axes of symmetry

Total: $2n$ elements. So $|D_n| = 2n$.

$D_n$ is generated by two elements: $r$ (rotation by $360°/n$) and $s$ (any one reflection), satisfying:
$$r^n = e, \quad s^2 = e, \quad srs = r^{-1}$$

(The last relation says "conjugation by $s$ inverts $r$," which makes sense geometrically: if you flip, then rotate, then flip back, you get the inverse rotation.)

$D_3 \cong S_3$ (the symmetries of an equilateral triangle are the same as the permutations of its three vertices). $D_4$ (symmetries of a square) has 8 elements. These dihedral groups are the simplest non-abelian groups.

For $n \geq 3$, $D_n$ is non-abelian. For $n = 1$, $D_1 \cong \mathbb{Z}/2\mathbb{Z}$ (just identity and one flip). For $n = 2$, $D_2 \cong \mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/2\mathbb{Z}$.

## The Trivial Group and Small Groups

**The trivial group $\{e\}$.** One element, one operation. This is the "do nothing" group. Every equation $ae = ea = a = e$ is trivially satisfied. It's abelian and finite.

**Groups of order 1, 2, 3, 4.** By Lagrange's theorem (coming soon), groups can only have orders dividing certain numbers. For small orders:
- Order 1: only the trivial group.
- Order 2: only $\mathbb{Z}/2\mathbb{Z}$. (Two elements: identity and one involution.)
- Order 3: only $\mathbb{Z}/3\mathbb{Z}$. (Three elements form a cycle.)
- Order 4: either $\mathbb{Z}/4\mathbb{Z}$ (cyclic) or $\mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/2\mathbb{Z}$ (Klein four-group, every element self-inverse). These are the two distinct groups of order 4, up to isomorphism.

Order 5: only $\mathbb{Z}/5\mathbb{Z}$ (prime order forces cyclic).

Order 6: either $\mathbb{Z}/6\mathbb{Z}$ (abelian) or $S_3 \cong D_3$ (non-abelian). Two non-isomorphic groups of order 6.

The classification of all finite groups is an enormous achievement. The *Classification of Finite Simple Groups* (CFSG), completed in the 1980s after decades of work by hundreds of mathematicians, is one of the longest proofs in mathematics, spanning tens of thousands of journal pages.

## The Symmetry Group of Any Object

**$\text{Sym}(X)$ for a set $X$.** For any set $X$, the set of all bijections $X \to X$ under composition forms a group called the *symmetric group on $X$* or the *permutation group of $X$*. When $X = \{1, \ldots, n\}$, this is $S_n$. But $X$ can be any set — infinite, uncountable, whatever.

This example is the "most general" group. Cayley's theorem (Section 3.3) will show that *every* group is isomorphic to a subgroup of $\text{Sym}(G)$ for some set $G$.

## Groups in Topology (Preview)

The fundamental group $\pi_1(X, x_0)$ of a topological space $X$ at a basepoint $x_0$ is a group. Elements are homotopy classes of loops based at $x_0$. The group operation is concatenation of loops.

For example:
- $\pi_1(S^1) \cong \mathbb{Z}$: loops on the circle are classified by their winding number.
- $\pi_1(\mathbb{R}^n) \cong \{e\}$: $\mathbb{R}^n$ is simply connected.
- $\pi_1(\text{torus}) \cong \mathbb{Z} \times \mathbb{Z}$: two independent directions of looping.
- $\pi_1(\text{Klein bottle})$: a non-abelian group with an interesting presentation.

These are not just analogies — the group axioms are literally satisfied by homotopy classes of loops under concatenation. The group structure on loops is what makes algebraic topology work. And in HoTT, the identity type plays this role directly.

## What Makes a "Good" Example?

Looking at all these examples, we can ask: what makes any particular example worth knowing? The answer is usually one or more of:

1. **It's a source of phenomena.** $S_n$ is non-abelian for $n \geq 3$, which means non-commutativity is not exotic — it's what you expect in most interesting situations.

2. **It appears naturally elsewhere.** $SO(3)$ appears in quantum mechanics, crystallography, robotics. $\mathbb{Z}/n\mathbb{Z}$ appears in clock arithmetic, modular forms, elliptic curves.

3. **It tests hypotheses.** If you think you've found a theorem, checking it against a few well-chosen examples quickly reveals whether your intuition is right or where it needs refinement.

4. **It becomes the general case.** Cayley's theorem says every group embeds in a symmetric group, so in some sense $S_n$ is "every group" for large enough $n$.

With this zoo of examples in mind, we're ready to explore the structure of groups more deeply — starting with groups-within-groups.
