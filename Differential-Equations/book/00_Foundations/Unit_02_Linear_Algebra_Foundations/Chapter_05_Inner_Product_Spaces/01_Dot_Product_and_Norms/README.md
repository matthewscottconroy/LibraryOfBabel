# Dot Product and Norms

Length and angle are the primitive concepts of Euclidean geometry. In the abstract setting of vector spaces, they are captured by the inner product: a bilinear, symmetric, positive definite map from pairs of vectors to scalars. The inner product gives a vector space its geometry — the ability to measure sizes and angles, define perpendicularity, and develop the rich theory of projections and orthonormal bases.

## The Standard Dot Product on $\mathbb{R}^n$

For $u = (u_1, \ldots, u_n)$ and $v = (v_1, \ldots, v_n)$ in $\mathbb{R}^n$, the **dot product** is
$$u \cdot v = \langle u, v\rangle = \sum_{i=1}^n u_i v_i = u^T v.$$

The **Euclidean norm** of $v$ is $\|v\| = \sqrt{\langle v, v\rangle} = \sqrt{\sum_i v_i^2}$.

The **angle** between nonzero $u$ and $v$ satisfies $\cos\theta = \frac{\langle u,v\rangle}{\|u\|\|v\|}$, which is well-defined by the Cauchy-Schwarz inequality.

## Abstract Inner Products

**Definition.** An **inner product** on a real vector space $V$ is a function $\langle \cdot, \cdot\rangle: V\times V \to \mathbb{R}$ satisfying:

(IP1) **Symmetry:** $\langle u, v\rangle = \langle v, u\rangle$.

(IP2) **Bilinearity:** $\langle \alpha u + \beta w, v\rangle = \alpha\langle u,v\rangle + \beta\langle w,v\rangle$ (linear in first argument; by symmetry, also in the second).

(IP3) **Positive definiteness:** $\langle v, v\rangle \geq 0$ for all $v$, with equality iff $v = \mathbf{0}$.

An **inner product space** is a vector space equipped with an inner product. The **norm** induced by the inner product is $\|v\| = \sqrt{\langle v,v\rangle}$.

## Key Inequalities

**Theorem (Cauchy-Schwarz).** $|\langle u, v\rangle| \leq \|u\|\cdot\|v\|$, with equality iff $u$ and $v$ are linearly dependent.

*Proof.* For any $t \in \mathbb{R}$: $0 \leq \|u + tv\|^2 = \|u\|^2 + 2t\langle u,v\rangle + t^2\|v\|^2$. This is a quadratic in $t$ with nonnegative values everywhere, so its discriminant is $\leq 0$: $4\langle u,v\rangle^2 - 4\|u\|^2\|v\|^2 \leq 0$. $\square$

**Theorem (Triangle Inequality).** $\|u + v\| \leq \|u\| + \|v\|$.

*Proof.* $\|u+v\|^2 = \|u\|^2 + 2\langle u,v\rangle + \|v\|^2 \leq \|u\|^2 + 2\|u\|\|v\| + \|v\|^2 = (\|u\|+\|v\|)^2$. $\square$

**Parallelogram Law.** $\|u+v\|^2 + \|u-v\|^2 = 2(\|u\|^2 + \|v\|^2)$.

This law characterizes norms that come from inner products: a norm on a vector space is an inner product norm iff it satisfies the parallelogram law.

## Examples of Inner Products

**$C([a,b])$:** $\langle f, g\rangle = \int_a^b f(x)g(x)\,dx$. This is symmetric (Fubini trivially), bilinear (linearity of integration), and positive definite: $\int_a^b f(x)^2\,dx \geq 0$, with equality iff $f \equiv 0$ on $[a,b]$ (by continuity).

**Weighted inner product:** $\langle f,g\rangle_w = \int_a^b f(x)g(x)w(x)\,dx$ for a positive weight function $w > 0$. This arises in Sturm-Liouville theory.

**$\mathbb{R}^n$ with weighted norm:** $\langle u,v\rangle_W = u^TWv$ for a positive definite matrix $W$.

## Orthogonality

**Definition.** Vectors $u, v$ are **orthogonal**, written $u \perp v$, if $\langle u,v\rangle = 0$.

Note: the zero vector is orthogonal to every vector.

**Theorem (Pythagorean Theorem).** If $u \perp v$, then $\|u + v\|^2 = \|u\|^2 + \|v\|^2$.

*Proof.* $\|u+v\|^2 = \|u\|^2 + 2\langle u,v\rangle + \|v\|^2 = \|u\|^2 + \|v\|^2$. $\square$

**Definition.** A set $\{v_1, \ldots, v_k\}$ is:
- **Orthogonal** if $\langle v_i, v_j\rangle = 0$ for $i \neq j$.
- **Orthonormal** if additionally $\|v_i\| = 1$ for all $i$.

**Theorem.** Every orthogonal set of nonzero vectors is linearly independent.

*Proof.* If $\sum c_i v_i = 0$, take the inner product with $v_j$: $c_j\|v_j\|^2 = 0$, so $c_j = 0$. $\square$

## Norms Not from Inner Products

Not every norm on a vector space comes from an inner product. The $\ell^1$ norm $\|v\|_1 = \sum|v_i|$ and the $\ell^\infty$ norm $\|v\|_\infty = \max|v_i|$ fail the parallelogram law for $n \geq 2$. The $\ell^2$ norm $\|v\|_2 = \sqrt{\sum v_i^2}$ does come from an inner product. Only the Euclidean norm (and its scalar multiples) is an inner product norm on $\mathbb{R}^n$.

## The Induced Matrix Norm

For a matrix $A \in M_{m\times n}$, the **operator norm** induced by the Euclidean norm is
$$\|A\| = \sup_{\|x\|=1}\|Ax\| = \sigma_1,$$
the largest singular value of $A$ (discussed in Section 5). This norm satisfies $\|AB\| \leq \|A\|\|B\|$ (submultiplicativity) and is the natural norm for analyzing the matrix exponential: $\|e^{At}\| \leq e^{\|A\|t}$.

## Connection to Differential Equations

The $L^2$ inner product $\langle f,g\rangle = \int_a^b fg$ is the foundation for Fourier analysis and Sturm-Liouville theory. Eigenfunctions of a self-adjoint differential operator are orthogonal with respect to this inner product (or a weighted variant), and any function can be expanded in the orthonormal eigenbasis — this is the Fourier series or, more generally, the eigenfunction expansion. The Parseval identity $\|f\|_{L^2}^2 = \sum_n |\langle f, \phi_n\rangle|^2$ (where $\phi_n$ are the orthonormal eigenfunctions) is the infinite-dimensional analog of the Pythagorean theorem.
