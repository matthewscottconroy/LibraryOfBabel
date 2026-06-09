# Vector Algebra

Consider a ship navigating across open water. Its position can be described by two numbers — latitude and longitude — and a displacement from one port to another requires specifying both a magnitude and a direction. Neither piece of information alone is sufficient. This simple observation motivates the central object of multivariable mathematics: the vector.

## Definitions

A **vector** in $\mathbb{R}^n$ is an ordered $n$-tuple of real numbers,

$$\mathbf{v} = (v_1, v_2, \ldots, v_n),$$

where each $v_i \in \mathbb{R}$ is called the $i$-th **component** of $\mathbf{v}$. The set of all such $n$-tuples is denoted $\mathbb{R}^n$. Two vectors are equal if and only if all their corresponding components are equal.

The zero vector $\mathbf{0} = (0, 0, \ldots, 0)$ plays the role of the additive identity.

**Vector addition** is defined componentwise: if $\mathbf{u} = (u_1, \ldots, u_n)$ and $\mathbf{v} = (v_1, \ldots, v_n)$, then

$$\mathbf{u} + \mathbf{v} = (u_1 + v_1, u_2 + v_2, \ldots, u_n + v_n).$$

**Scalar multiplication** by $c \in \mathbb{R}$ is likewise componentwise:

$$c\mathbf{v} = (cv_1, cv_2, \ldots, cv_n).$$

These two operations make $\mathbb{R}^n$ a **vector space** over $\mathbb{R}$: they satisfy commutativity and associativity of addition, the existence of additive inverses, and the distributive laws relating scalar multiplication to both operations.

## The Standard Basis

The **standard basis** of $\mathbb{R}^n$ consists of the $n$ vectors

$$\mathbf{e}_1 = (1, 0, 0, \ldots, 0), \quad \mathbf{e}_2 = (0, 1, 0, \ldots, 0), \quad \ldots, \quad \mathbf{e}_n = (0, 0, \ldots, 0, 1).$$

Every vector in $\mathbb{R}^n$ has a unique representation as a linear combination of these basis vectors:

$$\mathbf{v} = v_1 \mathbf{e}_1 + v_2 \mathbf{e}_2 + \cdots + v_n \mathbf{e}_n.$$

In $\mathbb{R}^3$, the standard basis vectors are often written $\mathbf{i}, \mathbf{j}, \mathbf{k}$, and this notation appears frequently in physics and engineering texts.

## The Euclidean Norm

The **Euclidean norm** (or **length**) of a vector $\mathbf{v} \in \mathbb{R}^n$ is

$$\|\mathbf{v}\| = \sqrt{v_1^2 + v_2^2 + \cdots + v_n^2}.$$

This generalizes the Pythagorean theorem. In $\mathbb{R}^2$, the norm of $(v_1, v_2)$ is simply the length of the hypotenuse of a right triangle with legs $|v_1|$ and $|v_2|$.

The norm satisfies three fundamental properties that make it a valid notion of length:
1. **Positive definiteness**: $\|\mathbf{v}\| \geq 0$, with equality if and only if $\mathbf{v} = \mathbf{0}$.
2. **Homogeneity**: $\|c\mathbf{v}\| = |c|\|\mathbf{v}\|$ for any scalar $c$.
3. **Triangle inequality**: $\|\mathbf{u} + \mathbf{v}\| \leq \|\mathbf{u}\| + \|\mathbf{v}\|$.

The triangle inequality is geometrically obvious in $\mathbb{R}^2$ (the straight-line distance between two points is at most the sum of two sides of any triangle connecting them), but its algebraic proof in $\mathbb{R}^n$ requires the Cauchy-Schwarz inequality, which is developed in the next section.

A vector of norm 1 is called a **unit vector**. Given any nonzero vector $\mathbf{v}$, the unit vector in the same direction is $\hat{\mathbf{v}} = \mathbf{v}/\|\mathbf{v}\|$. The process of computing $\hat{\mathbf{v}}$ from $\mathbf{v}$ is called **normalization**.

## Linear Combinations and Span

A **linear combination** of vectors $\mathbf{v}_1, \mathbf{v}_2, \ldots, \mathbf{v}_k \in \mathbb{R}^n$ with coefficients $c_1, c_2, \ldots, c_k \in \mathbb{R}$ is the vector

$$c_1 \mathbf{v}_1 + c_2 \mathbf{v}_2 + \cdots + c_k \mathbf{v}_k.$$

The set of all linear combinations of a collection of vectors is called their **span**. In $\mathbb{R}^2$, two nonparallel vectors span the whole plane; a single nonzero vector spans a line through the origin.

A set of vectors is **linearly independent** if no vector in the set can be written as a linear combination of the others — equivalently, if the equation $c_1 \mathbf{v}_1 + \cdots + c_k \mathbf{v}_k = \mathbf{0}$ implies $c_1 = c_2 = \cdots = c_k = 0$. The standard basis vectors are linearly independent and span all of $\mathbb{R}^n$, so they form a **basis** of $\mathbb{R}^n$.

## Geometric Interpretation

Geometrically, a vector in $\mathbb{R}^n$ can be thought of as an arrow from the origin to the point $(v_1, \ldots, v_n)$, or equivalently as a displacement: an instruction to move $v_1$ units in the $x_1$-direction, $v_2$ units in the $x_2$-direction, and so on. Vector addition then corresponds to sequential displacements, and scalar multiplication stretches or reverses the arrow. The zero vector corresponds to no displacement.

This geometric picture is most vivid in $\mathbb{R}^2$ and $\mathbb{R}^3$, but it guides intuition in higher dimensions as well.

## Worked Example

Let $\mathbf{u} = (1, -2, 3)$ and $\mathbf{v} = (4, 0, -1)$ in $\mathbb{R}^3$.

- $\mathbf{u} + \mathbf{v} = (1+4, -2+0, 3-1) = (5, -2, 2)$.
- $3\mathbf{u} - 2\mathbf{v} = (3, -6, 9) - (8, 0, -2) = (-5, -6, 11)$.
- $\|\mathbf{u}\| = \sqrt{1 + 4 + 9} = \sqrt{14}$.
- The unit vector in the direction of $\mathbf{u}$ is $\hat{\mathbf{u}} = \frac{1}{\sqrt{14}}(1, -2, 3)$.

## Common Pitfalls

Students sometimes confuse the zero vector $\mathbf{0}$ with the scalar $0$. The zero vector has $n$ components, all zero; the scalar $0$ is a single number. The equation $c\mathbf{v} = \mathbf{0}$ holds if either $c = 0$ (the scalar) or $\mathbf{v} = \mathbf{0}$ (the zero vector), but one must be careful about which case applies.

Another common error is treating the norm as if it were linear: $\|\mathbf{u} + \mathbf{v}\| \neq \|\mathbf{u}\| + \|\mathbf{v}\|$ in general. The triangle inequality gives only an upper bound. Equality holds precisely when $\mathbf{u}$ and $\mathbf{v}$ point in the same direction, i.e., when one is a non-negative scalar multiple of the other.

## Connections to Other Areas

The vector space structure introduced here is the same structure that underlies linear algebra, functional analysis, and quantum mechanics. The notion of a normed vector space — a vector space equipped with a norm satisfying the three properties above — is a central object in analysis. The specific norm used here, the Euclidean norm arising from the dot product, gives $\mathbb{R}^n$ the structure of an inner product space, which is introduced in the next section. Later in the course, vector algebra is the language in which the gradient, Jacobian, and curl are all expressed.
