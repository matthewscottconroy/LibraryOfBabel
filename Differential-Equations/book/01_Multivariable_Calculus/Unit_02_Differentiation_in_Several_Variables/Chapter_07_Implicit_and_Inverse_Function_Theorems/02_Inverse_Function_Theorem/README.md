# Inverse Function Theorem

A function of one variable is locally invertible near a point $a$ if and only if $f'(a)\neq 0$: the function is either strictly increasing or strictly decreasing there, so it is one-to-one in a neighborhood. In several variables, the analogous condition is that the Jacobian matrix $J_f(\mathbf{a})$ is invertible (nonsingular). The **inverse function theorem** formalizes this: under this hypothesis, $f$ has a smooth local inverse near $\mathbf{a}$, and the Jacobian of the inverse is the inverse of the Jacobian.

## Statement

**Theorem (Inverse Function Theorem).** Let $f: D\subseteq\mathbb{R}^n\to\mathbb{R}^n$ be $C^1$ on an open set $D$, and suppose $J_f(\mathbf{a})$ is invertible at some $\mathbf{a}\in D$. Then there exist open neighborhoods $U\ni\mathbf{a}$ and $V\ni f(\mathbf{a})$ such that:

1. $f|_U: U\to V$ is a bijection.
2. The inverse map $f^{-1}: V\to U$ is $C^1$.
3. For all $\mathbf{y}\in V$: $J_{f^{-1}}(\mathbf{y}) = \left[J_f(f^{-1}(\mathbf{y}))\right]^{-1}$.

If $f$ is $C^k$ for $k\geq 1$, then $f^{-1}$ is also $C^k$.

## Interpretation

The theorem says:
- **Locally injective:** A function with nonsingular Jacobian at $\mathbf{a}$ is locally one-to-one near $\mathbf{a}$.
- **Locally surjective:** Every point near $f(\mathbf{a})$ is in the image of $f$ near $\mathbf{a}$.
- **Smooth inverse:** The local inverse is as smooth as $f$ itself.

The condition $\det J_f(\mathbf{a})\neq 0$ is exactly the condition that the linearization $L(\mathbf{x}) = f(\mathbf{a})+J_f(\mathbf{a})(\mathbf{x}-\mathbf{a})$ of $f$ near $\mathbf{a}$ is invertible. Since $f$ is locally approximated by its linearization, $f$ itself is locally invertible.

## The Jacobian of the Inverse

The formula $J_{f^{-1}}(\mathbf{y}) = [J_f(f^{-1}(\mathbf{y}))]^{-1}$ is the multivariable chain rule applied to $f\circ f^{-1} = \text{id}$:

$J_{f}(f^{-1}(\mathbf{y}))\cdot J_{f^{-1}}(\mathbf{y}) = J_{\text{id}}(\mathbf{y}) = I$.

So $J_{f^{-1}} = (J_f)^{-1}$. In one dimension, this reduces to $(f^{-1})'(y) = 1/f'(f^{-1}(y))$, the familiar formula from single-variable calculus.

## Worked Example: Polar Coordinates

Consider the polar coordinate map $f(r,\theta) = (r\cos\theta, r\sin\theta) = (x,y)$.

$J_f = \begin{pmatrix}\cos\theta & -r\sin\theta \\ \sin\theta & r\cos\theta\end{pmatrix}$, $\det J_f = r\cos^2\theta + r\sin^2\theta = r$.

The Jacobian is nonsingular iff $r\neq 0$. Therefore: near any point with $r > 0$, the polar coordinate map is locally invertible (one can solve for $(r,\theta)$ from $(x,y)$). At $r=0$ (the origin), the Jacobian is singular, and indeed polar coordinates are not invertible near the origin (every $(0,\theta)$ maps to the same point $(0,0)$).

The inverse Jacobian: $J_{f^{-1}} = (J_f)^{-1} = \frac{1}{r}\begin{pmatrix}r\cos\theta & r\sin\theta \\ -\sin\theta & \cos\theta\end{pmatrix} = \begin{pmatrix}\cos\theta & \sin\theta \\ -\sin\theta/r & \cos\theta/r\end{pmatrix}$.

This is the Jacobian of the map $(x,y)\mapsto(r,\theta) = (\sqrt{x^2+y^2}, \arctan(y/x))$, which can be verified directly.

## Worked Example: Nonlinear System

Let $f(x,y) = (x^2-y^2, 2xy)$. (This is complex squaring: $z\mapsto z^2$ with $z=x+iy$.)

$J_f = \begin{pmatrix}2x & -2y \\ 2y & 2x\end{pmatrix}$, $\det J_f = 4x^2+4y^2 = 4(x^2+y^2)$.

Nonsingular iff $(x,y)\neq(0,0)$. Near any nonzero point, $f$ is locally invertible. One can verify: $f$ maps the circle $x^2+y^2=r^2$ to the circle $u^2+v^2 = r^4$ (with $u=x^2-y^2$, $v=2xy$), going around twice. So $f$ is 2-to-1 globally ($(x,y)$ and $(-x,-y)$ have the same image), but locally 1-to-1 away from the origin.

## Proof Sketch

The proof constructs the inverse map using the **contraction mapping theorem**. Given $\mathbf{y}$ near $f(\mathbf{a})$, one solves $f(\mathbf{x})=\mathbf{y}$ iteratively: $\mathbf{x}_{k+1} = \mathbf{x}_k - [J_f(\mathbf{a})]^{-1}(f(\mathbf{x}_k)-\mathbf{y})$. Since $J_f$ is continuous and nonsingular at $\mathbf{a}$, the operator $\mathbf{x}\mapsto \mathbf{x}-[J_f(\mathbf{a})]^{-1}(f(\mathbf{x})-\mathbf{y})$ is a contraction on a small ball around $f^{-1}(\mathbf{y})$ (by the mean value theorem), so it converges to a unique fixed point, which is the desired $f^{-1}(\mathbf{y})$.

## Consequence: Coordinate Change in Integration

The inverse function theorem justifies the change-of-variables formula in multiple integrals. If $\mathbf{g}:\mathbb{R}^n\to\mathbb{R}^n$ is a $C^1$ bijection with nonsingular Jacobian, then:

$$\int_{g(D)}f(\mathbf{x})\,d^n\mathbf{x} = \int_D f(\mathbf{g}(\mathbf{u}))\,|\det J_\mathbf{g}(\mathbf{u})|\,d^n\mathbf{u}.$$

The hypothesis that $\mathbf{g}$ is locally invertible (guaranteed by $\det J_\mathbf{g}\neq 0$) ensures the change of variables is valid.

## Global vs. Local Invertibility

The inverse function theorem is a **local** result: it only guarantees a local inverse. A map can be locally invertible everywhere (every point has an invertible Jacobian) but not globally injective.

**Example.** $f:\mathbb{R}\to\mathbb{R}$, $f(x) = e^x$. Then $f'(x) = e^x > 0$ everywhere, so $f$ is locally invertible at every point. And indeed $f$ is globally invertible (it is strictly increasing). But $f:\mathbb{R}\to\mathbb{R}^2$, $f(t) = (\cos t, \sin t)$ is locally invertible everywhere ($\|f'(t)\| = 1\neq 0$) but globally many-to-one ($f(t) = f(t+2\pi)$).

For global invertibility, one needs additional conditions (e.g., the map is proper and has degree $\pm 1$, or the domain is simply connected and the map is locally injective).
