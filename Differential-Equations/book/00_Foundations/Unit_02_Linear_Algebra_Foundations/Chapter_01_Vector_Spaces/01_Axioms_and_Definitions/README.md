# Axioms and Definitions of Vector Spaces

Two operations define the structure of a vector space: addition (combining two elements to get a third) and scalar multiplication (stretching or compressing an element by a field element). The axioms govern how these operations interact. Any object satisfying these axioms — regardless of what its elements "look like" — is a vector space, and all the theorems of linear algebra apply to it.

## The Axioms

**Definition.** A **vector space** over a field $F$ is a set $V$ together with two operations:
- Addition: $V \times V \to V$, denoted $(u, v) \mapsto u + v$
- Scalar multiplication: $F \times V \to V$, denoted $(\alpha, v) \mapsto \alpha v$

satisfying the following eight axioms for all $u, v, w \in V$ and $\alpha, \beta \in F$:

(V1) $u + v = v + u$ (commutativity of addition)

(V2) $(u + v) + w = u + (v + w)$ (associativity of addition)

(V3) There exists $\mathbf{0} \in V$ such that $v + \mathbf{0} = v$ for all $v$ (additive identity)

(V4) For each $v \in V$, there exists $-v \in V$ with $v + (-v) = \mathbf{0}$ (additive inverse)

(V5) $1 \cdot v = v$ (multiplicative identity)

(V6) $\alpha(\beta v) = (\alpha\beta) v$ (compatibility of scalar multiplication)

(V7) $\alpha(u + v) = \alpha u + \alpha v$ (distributivity over vector addition)

(V8) $(\alpha + \beta) v = \alpha v + \beta v$ (distributivity over scalar addition)

## Consequences of the Axioms

These axioms are not independent — some follow from combinations of others — but they form a convenient complete list. From them, one derives:

**Theorem.** The zero vector $\mathbf{0}$ is unique, and the additive inverse $-v$ is unique for each $v$.

**Theorem.** $0 \cdot v = \mathbf{0}$ and $(-1) \cdot v = -v$ for all $v \in V$.

*Proof.* $0 \cdot v = (0 + 0) \cdot v = 0 \cdot v + 0 \cdot v$. Adding $-(0\cdot v)$ to both sides: $\mathbf{0} = 0 \cdot v$. Then $(-1)v + v = (-1+1)v = 0\cdot v = \mathbf{0}$, so $(-1)v = -v$. $\square$

**Theorem.** If $\alpha v = \mathbf{0}$, then $\alpha = 0$ or $v = \mathbf{0}$.

## Examples

The power of the abstract definition is that all of the following are vector spaces over $\mathbb{R}$:

**$\mathbb{R}^n$:** Vectors are $n$-tuples of real numbers, with componentwise addition and scalar multiplication. This is the prototype.

**$C([a,b])$:** Continuous functions on $[a,b]$, with addition $(f+g)(x) = f(x)+g(x)$ and scalar multiplication $(\alpha f)(x) = \alpha f(x)$. The zero vector is the zero function $\mathbf{0}(x) \equiv 0$.

**$P_n$:** Polynomials of degree at most $n$, with standard polynomial addition and scalar multiplication. This is a $(n+1)$-dimensional vector space.

**$M_{m\times n}$:** $m \times n$ matrices with real entries, with entry-wise addition and scalar multiplication.

**$C^k(I)$:** Functions with $k$ continuous derivatives on an interval $I$. This is a subspace of $C(I)$.

**Solution spaces of ODEs:** The set $S$ of all solutions to $L[y] = a_n y^{(n)} + \cdots + a_0 y = 0$ on an interval $I$ is a vector space: if $y_1, y_2 \in S$, then $L[\alpha y_1 + \beta y_2] = \alpha L[y_1] + \beta L[y_2] = 0$, so $\alpha y_1 + \beta y_2 \in S$.

## Non-Examples

Not every set with these operations forms a vector space. The set of functions satisfying $f'' + f' + 1 = 0$ is **not** a vector space (it does not contain $\mathbf{0}$ — if $y_1, y_2$ are solutions, $\alpha y_1$ is generally not). The set of vectors in $\mathbb{R}^2$ with positive first component is not a vector space because it is not closed under scalar multiplication by negative scalars.

## Vector Spaces over $\mathbb{C}$

When $F = \mathbb{C}$, the scalar field is the complex numbers. Complex vector spaces arise naturally in ODE theory: the characteristic polynomial of a real matrix may have complex roots, and the corresponding eigenvectors have complex components. The solution of $y'' + y = 0$ can be written using complex exponentials $e^{it}$ and $e^{-it}$ before taking real and imaginary parts to get $\cos t$ and $\sin t$.

## The Role of the Field

Different choices of field $F$ produce different theories. Over $\mathbb{R}$, inner products can be defined (Chapter 5). Over $\mathbb{C}$, one needs Hermitian inner products. Over finite fields $\mathbb{Z}/p\mathbb{Z}$, one gets the theory of error-correcting codes. This unit works primarily over $\mathbb{R}$, with complex scalars appearing in eigenvalue computations.

## Connection to Differential Equations

The fact that the solution space of a homogeneous linear ODE is a vector space (shown above) is the most important structural observation in linear ODE theory. It means that:
- Finding solutions is not just about finding particular curves, but about finding a basis for a vector space.
- The superposition principle holds: any linear combination of solutions is a solution.
- The "general solution" is not a formula but a parametric description of every element of a vector space, with parameters $c_1, \ldots, c_n$ being the coordinates in the basis of solutions.

This perspective transforms ODE solving from a collection of techniques into a coherent algebraic theory.
