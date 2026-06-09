# Algebra of Complex Numbers

The complex number system $\mathbb{C}$ can be motivated by a concrete problem: the polynomial $p(x) = x^2 + 1$ has no real roots, because $x^2 \geq 0$ for all $x \in \mathbb{R}$. To remedy this, we formally adjoin a symbol $i$ satisfying $i^2 = -1$ and close the resulting set under addition and multiplication. The outcome is a field that contains $\mathbb{R}$ as a subfield and in which every polynomial has a root — a fact whose proof requires the full power of Liouville's theorem and belongs to Unit 03. This section develops the algebra of $\mathbb{C}$ from the ground up.

## Definition of Complex Numbers

**Definition.** A complex number is an expression of the form $z = x + iy$ where $x, y \in \mathbb{R}$ and $i$ is a formal symbol satisfying $i^2 = -1$. The real part of $z$ is $\mathrm{Re}(z) = x$ and the imaginary part is $\mathrm{Im}(z) = y$. The set of all complex numbers is denoted $\mathbb{C}$.

Concretely, $\mathbb{C}$ may be identified with $\mathbb{R}^2$ via $z = x + iy \leftrightarrow (x, y)$. The imaginary unit $i$ corresponds to $(0,1)$, and real numbers embed as $x \leftrightarrow (x, 0)$.

## Field Operations

**Addition** is defined component-wise:
$$(x_1 + iy_1) + (x_2 + iy_2) = (x_1 + x_2) + i(y_1 + y_2).$$

**Multiplication** is defined by expanding formally and using $i^2 = -1$:
$$(x_1 + iy_1)(x_2 + iy_2) = (x_1 x_2 - y_1 y_2) + i(x_1 y_2 + x_2 y_1).$$

This multiplication can also be written in matrix form: the map $z \mapsto az$ where $a = \alpha + i\beta$ acts on $\mathbb{R}^2$ as the linear transformation with matrix $\begin{pmatrix} \alpha & -\beta \\ \beta & \alpha \end{pmatrix}$, which is a composition of rotation and scaling. This observation connects the algebra of $\mathbb{C}$ to its geometry.

**Theorem.** $(\mathbb{C}, +, \cdot)$ is a field. In particular:
- Addition is commutative and associative, with identity $0 = 0 + i0$ and inverse $-(x+iy) = -x + i(-y)$.
- Multiplication is commutative and associative, with identity $1 = 1 + i0$.
- The distributive law $z(w_1 + w_2) = zw_1 + zw_2$ holds.
- Every nonzero element has a multiplicative inverse.

## Complex Conjugate and Modulus

**Definition.** The complex conjugate of $z = x + iy$ is $\bar{z} = x - iy$. The modulus (absolute value) of $z$ is $|z| = \sqrt{x^2 + y^2}$.

These two operations are the workhorses of complex arithmetic. Their key properties are:

$$\overline{z + w} = \bar{z} + \bar{w}, \qquad \overline{zw} = \bar{z}\bar{w}, \qquad \overline{\bar{z}} = z.$$
$$|zw| = |z||w|, \qquad |z + w| \leq |z| + |w|, \qquad |z| = |\bar{z}|.$$
$$z \bar{z} = |z|^2, \qquad \mathrm{Re}(z) = \frac{z + \bar{z}}{2}, \qquad \mathrm{Im}(z) = \frac{z - \bar{z}}{2i}.$$

The identity $z\bar{z} = |z|^2$ is the key to computing multiplicative inverses.

## Multiplicative Inverse

For $z = x + iy \neq 0$, the inverse is
$$z^{-1} = \frac{\bar{z}}{|z|^2} = \frac{x}{x^2 + y^2} - i\frac{y}{x^2 + y^2}.$$

**Worked example.** Compute $\dfrac{3 + 4i}{1 - 2i}$.

Multiply numerator and denominator by the conjugate of the denominator:
$$\frac{(3 + 4i)(1 + 2i)}{(1 - 2i)(1 + 2i)} = \frac{3 + 6i + 4i + 8i^2}{1 + 4} = \frac{3 + 10i - 8}{5} = \frac{-5 + 10i}{5} = -1 + 2i.$$

## The Triangle Inequality

**Theorem.** For all $z, w \in \mathbb{C}$, $|z + w| \leq |z| + |w|$.

**Proof.** We compute:
$$|z + w|^2 = (z + w)\overline{(z + w)} = (z + w)(\bar{z} + \bar{w}) = |z|^2 + 2\mathrm{Re}(z\bar{w}) + |w|^2.$$
Since $\mathrm{Re}(z\bar{w}) \leq |z\bar{w}| = |z||w|$, we get
$$|z + w|^2 \leq |z|^2 + 2|z||w| + |w|^2 = (|z| + |w|)^2.$$
Taking square roots gives the result. $\square$

A closely related inequality is the reverse triangle inequality: $\bigl||z| - |w|\bigr| \leq |z - w|$. This follows by applying the triangle inequality to $(z - w) + w$ and to $(w - z) + z$.

**Worked example.** Show that $|z^2 - 1| \geq |z|^2 - 1$ for $|z| \geq 1$.

By the reverse triangle inequality, $|z^2 - 1| \geq |z^2| - |1| = |z|^2 - 1$. $\square$

## Algebraic Structure and Topology

As a field, $\mathbb{C}$ is algebraically closed: every nonconstant polynomial with complex coefficients has a root in $\mathbb{C}$. This is the Fundamental Theorem of Algebra, which will be proved in Unit 03.

As a metric space, $\mathbb{C}$ with the metric $d(z, w) = |z - w|$ is complete: every Cauchy sequence converges. The open disk $D(z_0, r) = \{z : |z - z_0| < r\}$ plays the role of an open interval in real analysis. A subset of $\mathbb{C}$ is open if it contains an open disk around each of its points; it is connected if it cannot be written as the disjoint union of two nonempty open sets. A domain is a nonempty open connected subset of $\mathbb{C}$, and domains are the natural setting for analytic function theory.

## Worked Example: Powers of $i$

Since $i^2 = -1$, the powers of $i$ cycle with period 4:
$$i^0 = 1, \quad i^1 = i, \quad i^2 = -1, \quad i^3 = -i, \quad i^4 = 1, \quad \ldots$$
In general, $i^n = i^{n \bmod 4}$. For instance, $i^{47} = i^3 = -i$.

## Connection to Real Analysis

The field $\mathbb{C}$ is not an ordered field: there is no notion of "positive" complex number that is compatible with the field operations. This is the price paid for algebraic closure. In real analysis, the order structure underpins the mean value theorem and many other results; in complex analysis, these are replaced by arguments that use the rich geometric and topological structure of $\mathbb{C}$.

The identification of $\mathbb{C}$ with $\mathbb{R}^2$ means that many results from two-variable real analysis apply directly. However, as Unit 02 will show, imposing the additional requirement of complex differentiability forces $u$ and $v$ to satisfy constraints — the Cauchy-Riemann equations — that have no real-variable analogue and that give the theory its distinctive character.
