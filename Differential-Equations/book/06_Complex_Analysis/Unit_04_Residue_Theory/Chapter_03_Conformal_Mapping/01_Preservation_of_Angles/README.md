# Preservation of Angles: Conformal Maps

The word "conformal" comes from the Latin for "same shape," and it refers to the property that a map preserves the angles between curves. For analytic functions with nonvanishing derivative, this property holds at every point, making conformal maps the natural class of geometric transformations in complex analysis. This section proves the angle-preservation property, establishes the connection with the derivative, and explores what happens at critical points where the derivative vanishes.

## Angle Between Curves

**Definition.** The angle from a smooth curve $C_1$ to a smooth curve $C_2$ at a point $z_0$ where they intersect is the angle from the tangent vector of $C_1$ to the tangent vector of $C_2$ at $z_0$, measured counterclockwise.

If $C_k$ is parametrized by $z_k(t)$ with $z_k(0) = z_0$, the tangent direction of $C_k$ at $z_0$ is $\arg z_k'(0)$. The angle between the curves is $\arg z_2'(0) - \arg z_1'(0)$.

## Conformal Maps Preserve Angles

**Theorem.** Let $f$ be analytic at $z_0$ with $f'(z_0) \neq 0$. If $C_1$ and $C_2$ are smooth curves meeting at $z_0$ with angle $\alpha$, then the image curves $\Gamma_k = f(C_k)$ meet at $f(z_0)$ with the same angle $\alpha$.

**Proof.** Let $C_k(t)$ be a parametrization with $C_k(0) = z_0$. The image curve has parametrization $\Gamma_k(t) = f(C_k(t))$, with tangent vector $\Gamma_k'(0) = f'(z_0) C_k'(0)$. The angle between the image curves is:
$$\arg(\Gamma_2'(0)) - \arg(\Gamma_1'(0)) = \arg(f'(z_0) C_2'(0)) - \arg(f'(z_0)C_1'(0))$$
$$= \arg(f'(z_0)) + \arg(C_2'(0)) - \arg(f'(z_0)) - \arg(C_1'(0)) = \arg(C_2'(0)) - \arg(C_1'(0)) = \alpha. \quad \square$$

The factor $f'(z_0)$ contributes a rotation of $\arg f'(z_0)$ to every tangent vector, so the relative angle between any two tangent vectors is preserved.

**Definition.** A function $f$ is conformal at $z_0$ if it is analytic at $z_0$ and $f'(z_0) \neq 0$. A function is conformal on a domain $D$ if it is conformal at every point of $D$.

## Geometric Meaning of $f'(z_0)$

The derivative $f'(z_0) = |f'(z_0)|e^{i\arg f'(z_0)}$ encodes:
- **Scaling factor:** $|f'(z_0)|$ is the factor by which infinitesimal lengths are magnified.
- **Rotation:** $\arg f'(z_0)$ is the angle by which infinitesimal directions are rotated.

The Jacobian of $f$ (viewed as a map $\mathbb{R}^2 \to \mathbb{R}^2$) is:
$$JF = \begin{pmatrix}u_x & u_y \\ v_x & v_y\end{pmatrix} = |f'(z_0)|\begin{pmatrix}\cos\theta & -\sin\theta \\ \sin\theta & \cos\theta\end{pmatrix},$$
where $\theta = \arg f'(z_0)$. This is a rotation matrix scaled by $|f'(z_0)|$, confirming that the map is locally a rotation and scaling.

## The Inverse Function Theorem in the Complex Setting

**Theorem.** If $f$ is analytic at $z_0$ and $f'(z_0) \neq 0$, then $f$ is locally bijective: there exist neighborhoods $U$ of $z_0$ and $V$ of $f(z_0)$ such that $f|_U : U \to V$ is bijective, and its inverse $g = (f|_U)^{-1} : V \to U$ is analytic with $g'(f(z_0)) = 1/f'(z_0)$.

This follows from the real inverse function theorem (the Jacobian is nonsingular) combined with the Cauchy-Riemann equations, which show that the inverse also satisfies the Cauchy-Riemann equations.

## Critical Points: Angle Multiplication

At a point $z_0$ where $f'(z_0) = 0$, the map is not conformal. If $f'$ has a zero of order $k - 1$ at $z_0$ (so $f'(z_0) = f''(z_0) = \cdots = f^{(k-1)}(z_0) = 0$ but $f^{(k)}(z_0) \neq 0$), then $f(z) - f(z_0) \approx a_k(z-z_0)^k$ for $z$ near $z_0$. The map $w = a_k(z-z_0)^k$ multiplies arguments by $k$: the angle between two curves meeting at $z_0$ with angle $\alpha$ becomes $k\alpha$ in the image.

**Worked example.** $f(z) = z^2$ has $f'(0) = 0$ (critical point with $k = 2$). The positive real axis and the positive imaginary axis meet at $0$ with angle $\pi/2$. Their images are the positive real axis and the negative real axis, meeting at $0$ with angle $\pi = 2 \cdot \pi/2$. The angle is doubled. $\square$

## Worked Examples of Conformal Maps

**Example 1.** The map $f(z) = e^z$ is conformal everywhere (since $f'(z) = e^z \neq 0$). It maps horizontal lines $\{y = c\}$ (with $c \in [0, 2\pi)$) to rays $\{\arg w = c\}$, and vertical lines $\{x = c\}$ to circles $\{|w| = e^c\}$. The right angles between horizontal and vertical lines in the $z$-plane become right angles between rays and circles in the $w$-plane. $\square$

**Example 2.** The map $f(z) = z + 1/z$ is analytic with $f'(z) = 1 - 1/z^2 = 0$ at $z = \pm 1$ and $z = \pm i$. Wait — $f'(z) = 1 - 1/z^2 = 0$ gives $z^2 = 1$, so $z = \pm 1$. The critical points are at $z = \pm 1$. Away from these, $f$ is conformal. This is the Joukowski transform, used in aerodynamics to map circles to airfoil profiles.

**Example 3.** Determine where $f(z) = \sin z$ is conformal.

$f'(z) = \cos z = 0$ at $z = \pi/2 + n\pi$, $n \in \mathbb{Z}$. So $f$ is conformal everywhere except at these points.

## The Open Mapping Theorem

**Theorem.** If $f$ is analytic and nonconstant on a domain $D$, then $f$ maps open sets to open sets (i.e., $f$ is an open mapping).

**Proof sketch.** This follows from the local behavior near a point $z_0$ of order $k$ (where $f - f(z_0)$ has a zero of order $k$): the map looks like $w - f(z_0) = a_k(z - z_0)^k + \cdots$, and the image of a small disk around $z_0$ contains a small disk around $f(z_0)$. $\square$

The open mapping theorem implies the maximum modulus principle: if $|f|$ achieves its maximum at an interior point $z_0$, the image of a neighborhood of $z_0$ contains a neighborhood of $f(z_0)$, and in particular contains points with larger modulus than $f(z_0)$ — a contradiction unless $f$ is constant.

## Level Curves and Orthogonality

For an analytic function $f = u + iv$ with $f'(z) \neq 0$, the level curves $\{u = c\}$ and $\{v = d\}$ form two orthogonal families. This is because the gradients $\nabla u$ and $\nabla v$ are perpendicular: $\nabla u \cdot \nabla v = u_x v_x + u_y v_y = u_x(-u_y) + u_y u_x = 0$ (using the Cauchy-Riemann equations $v_x = -u_y$ and $v_y = u_x$). The level curves are tangent to the rotated gradients, so they too are perpendicular. This orthogonality is the geometric content of the Cauchy-Riemann equations.
