# Dot Product and Angle

Vector addition and scalar multiplication give $\mathbb{R}^n$ an algebraic structure, but they say nothing about the relationship between two different vectors — whether they point in roughly the same direction, whether they are perpendicular, or how their lengths compare. The dot product fills this gap. It is a single algebraic operation that encodes both lengths and angles, and it is arguably the most important computational tool in all of multivariable mathematics.

## Definition of the Dot Product

Given two vectors $\mathbf{u} = (u_1, u_2, \ldots, u_n)$ and $\mathbf{v} = (v_1, v_2, \ldots, v_n)$ in $\mathbb{R}^n$, their **dot product** (also called the **scalar product** or **inner product**) is the real number

$$\mathbf{u} \cdot \mathbf{v} = u_1 v_1 + u_2 v_2 + \cdots + u_n v_n = \sum_{i=1}^n u_i v_i.$$

The result is a scalar, not a vector. In particular, $\mathbf{v} \cdot \mathbf{v} = v_1^2 + v_2^2 + \cdots + v_n^2 = \|\mathbf{v}\|^2$, so the dot product of a vector with itself recovers the square of its norm. This observation connects the two sections: the Euclidean norm can be defined as $\|\mathbf{v}\| = \sqrt{\mathbf{v} \cdot \mathbf{v}}$.

The dot product is symmetric ($\mathbf{u} \cdot \mathbf{v} = \mathbf{v} \cdot \mathbf{u}$), linear in each argument ($(\mathbf{u} + \mathbf{w}) \cdot \mathbf{v} = \mathbf{u} \cdot \mathbf{v} + \mathbf{w} \cdot \mathbf{v}$ and $(c\mathbf{u}) \cdot \mathbf{v} = c(\mathbf{u} \cdot \mathbf{v})$), and positive definite ($\mathbf{v} \cdot \mathbf{v} \geq 0$ with equality only when $\mathbf{v} = \mathbf{0}$). These three properties make it an **inner product**, and $\mathbb{R}^n$ equipped with this inner product is called **Euclidean $n$-space**.

## The Cauchy-Schwarz Inequality

**Theorem (Cauchy-Schwarz):** For all $\mathbf{u}, \mathbf{v} \in \mathbb{R}^n$,

$$|\mathbf{u} \cdot \mathbf{v}| \leq \|\mathbf{u}\|\|\mathbf{v}\|,$$

with equality if and only if $\mathbf{u}$ and $\mathbf{v}$ are parallel (one is a scalar multiple of the other).

**Proof sketch:** If $\mathbf{v} = \mathbf{0}$ the result is trivial. For $\mathbf{v} \neq \mathbf{0}$, consider the quadratic in $t$:

$$0 \leq \|\mathbf{u} - t\mathbf{v}\|^2 = \mathbf{u}\cdot\mathbf{u} - 2t(\mathbf{u}\cdot\mathbf{v}) + t^2(\mathbf{v}\cdot\mathbf{v}).$$

This is a non-negative quadratic in $t$, so its discriminant must be non-positive: $4(\mathbf{u}\cdot\mathbf{v})^2 - 4(\mathbf{u}\cdot\mathbf{u})(\mathbf{v}\cdot\mathbf{v}) \leq 0$, which gives $(\mathbf{u}\cdot\mathbf{v})^2 \leq \|\mathbf{u}\|^2\|\mathbf{v}\|^2$. Taking square roots completes the proof. Equality holds when the quadratic has a real root, i.e., when $\mathbf{u} = t\mathbf{v}$ for some scalar $t$.

The Cauchy-Schwarz inequality immediately implies the **triangle inequality**: $\|\mathbf{u} + \mathbf{v}\|^2 = \|\mathbf{u}\|^2 + 2(\mathbf{u}\cdot\mathbf{v}) + \|\mathbf{v}\|^2 \leq \|\mathbf{u}\|^2 + 2\|\mathbf{u}\|\|\mathbf{v}\| + \|\mathbf{v}\|^2 = (\|\mathbf{u}\| + \|\mathbf{v}\|)^2$.

## The Angle Formula

Since Cauchy-Schwarz ensures that $-1 \leq \frac{\mathbf{u}\cdot\mathbf{v}}{\|\mathbf{u}\|\|\mathbf{v}\|} \leq 1$ for nonzero vectors, there exists a unique angle $\theta \in [0, \pi]$ such that

$$\cos\theta = \frac{\mathbf{u}\cdot\mathbf{v}}{\|\mathbf{u}\|\|\mathbf{v}\|}.$$

This is the **angle between $\mathbf{u}$ and $\mathbf{v}$**. In $\mathbb{R}^2$ and $\mathbb{R}^3$, this matches the ordinary geometric angle between the two arrows.

From the formula: if $\theta < \pi/2$ then $\cos\theta > 0$ so $\mathbf{u}\cdot\mathbf{v} > 0$ (the vectors point in generally the same direction); if $\theta = \pi/2$ then $\mathbf{u}\cdot\mathbf{v} = 0$ (the vectors are **orthogonal**); if $\theta > \pi/2$ then $\mathbf{u}\cdot\mathbf{v} < 0$ (the vectors point in generally opposite directions).

Orthogonality is one of the most important geometric relationships in the subject. Two vectors $\mathbf{u}$ and $\mathbf{v}$ are orthogonal if and only if $\mathbf{u}\cdot\mathbf{v} = 0$.

## Worked Examples

**Example 1.** Find the angle between $\mathbf{u} = (1, 1, 0)$ and $\mathbf{v} = (0, 1, 1)$ in $\mathbb{R}^3$.

$\mathbf{u}\cdot\mathbf{v} = 0\cdot1 + 1\cdot1 + 0\cdot1 = 1$. Wait — $\mathbf{u}\cdot\mathbf{v} = 1\cdot0 + 1\cdot1 + 0\cdot1 = 1$. Also $\|\mathbf{u}\| = \sqrt{2}$, $\|\mathbf{v}\| = \sqrt{2}$. So $\cos\theta = 1/2$, giving $\theta = \pi/3$ (60 degrees).

**Example 2.** Verify that $\mathbf{u} = (2, -1, 3)$ and $\mathbf{v} = (1, 7, 1)$ are orthogonal.

$\mathbf{u}\cdot\mathbf{v} = 2(1) + (-1)(7) + 3(1) = 2 - 7 + 3 = -2$. These vectors are not orthogonal. To find a vector orthogonal to $\mathbf{u}$, one approach is to use projections (Section 4).

**Example 3 (Work).** A force $\mathbf{F} = (3, 0, -1)$ newtons acts on an object moving through displacement $\mathbf{d} = (2, 2, 2)$ meters. The work done is $W = \mathbf{F}\cdot\mathbf{d} = 6 + 0 - 2 = 4$ joules. This is the most basic physical application of the dot product: work is the component of force in the direction of motion, multiplied by the distance.

## Common Pitfalls

A frequent error is writing $\mathbf{u}\cdot\mathbf{v}\cdot\mathbf{w}$ for three vectors and treating it as associative. The dot product of two vectors is a scalar, so $(\mathbf{u}\cdot\mathbf{v})\cdot\mathbf{w}$ is meaningless — you cannot dot a scalar with a vector. The dot product is defined only for two vectors of the same dimension.

Another pitfall: the angle formula gives $\theta \in [0, \pi]$. There is no notion of a "signed angle" between two vectors in $\mathbb{R}^n$ for $n \geq 3$; the cross product (Section 3) is needed to encode orientation in $\mathbb{R}^3$.

## Connections to Other Areas

The dot product is the computational heart of the gradient (Unit 2): the gradient of a function $f$ at a point $\mathbf{p}$ is the vector $\nabla f(\mathbf{p})$ such that the directional derivative in direction $\hat{\mathbf{u}}$ equals $\nabla f(\mathbf{p}) \cdot \hat{\mathbf{u}}$. The Cauchy-Schwarz inequality shows that this is maximized when $\hat{\mathbf{u}}$ points in the same direction as the gradient, leading directly to the steepest-ascent interpretation.

In linear algebra, the dot product generalizes to an inner product on any vector space, including spaces of functions. The $L^2$ inner product on function spaces, $\langle f, g \rangle = \int_a^b f(x)g(x)\,dx$, is the basis for Fourier analysis, spectral theory, and the theory of partial differential equations.
