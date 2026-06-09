# Second Order Approximation

The first-order Taylor approximation $f(\mathbf{a}+\mathbf{h}) \approx f(\mathbf{a}) + \nabla f(\mathbf{a})\cdot\mathbf{h}$ is the best linear approximation, but it provides no information about the curvature of $f$ near $\mathbf{a}$. In particular, at a critical point where $\nabla f(\mathbf{a}) = \mathbf{0}$, the linear approximation reduces to the constant $f(\mathbf{a})$ — useless for determining whether the point is a minimum, maximum, or saddle. The **second-order Taylor approximation** fills this gap, adding a quadratic term built from the Hessian matrix that captures the local curvature.

## The Second-Order Taylor Expansion

**Theorem (Taylor's Theorem, Second Order).** Let $f: D\subseteq\mathbb{R}^n\to\mathbb{R}$ be $C^2$ on an open set containing the line segment $[\mathbf{a}, \mathbf{a}+\mathbf{h}]$. Then

$$f(\mathbf{a}+\mathbf{h}) = f(\mathbf{a}) + \nabla f(\mathbf{a})\cdot\mathbf{h} + \frac{1}{2}\mathbf{h}^T H_f(\mathbf{a})\mathbf{h} + R_2(\mathbf{h}),$$

where the remainder satisfies $R_2(\mathbf{h}) = o(\|\mathbf{h}\|^2)$, i.e., $R_2(\mathbf{h})/\|\mathbf{h}\|^2\to 0$ as $\mathbf{h}\to\mathbf{0}$.

**Proof.** Define $g(t) = f(\mathbf{a}+t\mathbf{h})$ for $t\in[0,1]$. By the one-variable Taylor theorem:

$g(1) = g(0) + g'(0) + \frac{1}{2}g''(0) + \frac{1}{6}g'''(\xi)$ for some $\xi\in(0,1)$.

Compute: $g'(t) = \nabla f(\mathbf{a}+t\mathbf{h})\cdot\mathbf{h} = \sum_i f_{x_i}(\mathbf{a}+t\mathbf{h})\,h_i$.

$g''(t) = \sum_{i,j} f_{x_ix_j}(\mathbf{a}+t\mathbf{h})\,h_ih_j = \mathbf{h}^T H_f(\mathbf{a}+t\mathbf{h})\mathbf{h}$.

At $t=0$: $g'(0) = \nabla f(\mathbf{a})\cdot\mathbf{h}$ and $g''(0) = \mathbf{h}^T H_f(\mathbf{a})\mathbf{h}$.

The third-order remainder is $\frac{1}{6}g'''(\xi) = O(\|\mathbf{h}\|^3) = o(\|\mathbf{h}\|^2)$, giving the result.

## The Quadratic Term

The quadratic form $Q(\mathbf{h}) = \frac{1}{2}\mathbf{h}^T H_f(\mathbf{a})\mathbf{h}$ can be expanded:

$$Q(\mathbf{h}) = \frac{1}{2}\sum_{i,j} f_{x_ix_j}(\mathbf{a})\,h_ih_j.$$

For $f:\mathbb{R}^2\to\mathbb{R}$ with $\mathbf{h} = (h, k)$:

$$Q(h,k) = \frac{1}{2}(f_{xx}h^2 + 2f_{xy}hk + f_{yy}k^2).$$

This is the second-order part of the approximation. Together with the linear part:

$$f(a+h, b+k) \approx f(a,b) + f_x(a,b)\,h + f_y(a,b)\,k + \frac{1}{2}\left[f_{xx}h^2 + 2f_{xy}hk + f_{yy}k^2\right].$$

## At a Critical Point

When $\nabla f(\mathbf{a}) = \mathbf{0}$, the expansion simplifies dramatically:

$$f(\mathbf{a}+\mathbf{h}) = f(\mathbf{a}) + \frac{1}{2}\mathbf{h}^T H_f(\mathbf{a})\mathbf{h} + o(\|\mathbf{h}\|^2).$$

For small $\mathbf{h}$, the sign of $f(\mathbf{a}+\mathbf{h}) - f(\mathbf{a})$ is determined by the sign of $\mathbf{h}^T H_f(\mathbf{a})\mathbf{h}$:
- If $H_f(\mathbf{a})$ is positive definite: $\mathbf{h}^T H_f\mathbf{h} > 0$ for all $\mathbf{h}\neq\mathbf{0}$, so $f(\mathbf{a}+\mathbf{h}) > f(\mathbf{a})$ near $\mathbf{a}$: $\mathbf{a}$ is a **local minimum**.
- If $H_f(\mathbf{a})$ is negative definite: $\mathbf{h}^T H_f\mathbf{h} < 0$ for all $\mathbf{h}\neq\mathbf{0}$: $\mathbf{a}$ is a **local maximum**.
- If $H_f(\mathbf{a})$ is indefinite: $\mathbf{a}$ is a **saddle point** — neither a min nor a max.

This is the content of the **second derivative test** (Chapter 6), derived here from the Taylor expansion.

## Worked Example

$f(x,y) = x^2+xy+y^2$ near $(0,0)$.

$f(0,0) = 0$, $\nabla f = (2x+y, x+2y) = (0,0)$ at the origin.

$H_f = \begin{pmatrix}2&1\\1&2\end{pmatrix}$, determinant $= 4-1 = 3 > 0$, trace $= 4 > 0$, so positive definite.

Second-order approximation at $(0,0)$: $f(h,k) \approx \frac{1}{2}(2h^2+2hk+2k^2) = h^2+hk+k^2$.

This equals $f$ exactly (it's a quadratic), confirming the approximation is perfect for polynomials of degree $\leq 2$.

The minimum is at the origin, as $h^2+hk+k^2 = (h+k/2)^2 + 3k^2/4 \geq 0$.

## The Remainder Bound

For $f\in C^3$ near $\mathbf{a}$, the Lagrange form of the remainder is $R_2(\mathbf{h}) = \frac{1}{6}g'''(\xi)$ where $g(t) = f(\mathbf{a}+t\mathbf{h})$. One has $|g'''(\xi)| \leq C\|\mathbf{h}\|^3$ for some constant $C$, so $|R_2(\mathbf{h})| \leq \frac{C}{6}\|\mathbf{h}\|^3$. This means the quadratic approximation is accurate to third-order error — it is "off" by at most $O(\|\mathbf{h}\|^3)$.

## Connection to the Shape of the Level Sets

Near a critical point $\mathbf{a}$, the level sets of $f$ look like the level sets of the quadratic form $Q(\mathbf{h}) = \frac{1}{2}\mathbf{h}^T H_f(\mathbf{a})\mathbf{h}$. If $H_f$ is positive definite, the level sets near $\mathbf{a}$ are ellipses (in 2D) or ellipsoids (in 3D). If $H_f$ is indefinite, the level sets are hyperbolas, and the origin is a saddle. This geometric picture — ellipses around a minimum, hyperbolas around a saddle — is directly visible in the quadric surface classification of Unit 1.
