# Change of Variables and Jacobian

In single-variable integration, the substitution rule $\int_a^b f(g(x))g'(x)\,dx = \int_{g(a)}^{g(b)} f(u)\,du$ converts one integral into another by changing variables. The factor $g'(x)$ corrects for the stretching of the interval under the map $u = g(x)$. In two dimensions, the analogous formula involves a transformation $(x,y) = \mathbf{g}(u,v)$ of the plane, and the "stretching factor" is the absolute value of the Jacobian determinant $|\det J_\mathbf{g}|$. This formula is both theoretically central — it is where the Jacobian of Unit 2 makes its most dramatic appearance — and practically essential for converting difficult integrals into simple ones.

## Statement of the Theorem

**Theorem (Change of Variables in Double Integrals).** Let $D^*\subseteq\mathbb{R}^2$ be a region in the $(u,v)$-plane, and let $\mathbf{g}: D^*\to D\subseteq\mathbb{R}^2$ be a $C^1$ bijection with nonzero Jacobian determinant (except possibly on a set of zero area). Write $(x,y) = \mathbf{g}(u,v) = (g_1(u,v), g_2(u,v))$. Then for any continuous $f:D\to\mathbb{R}$:

$$\iint_D f(x,y)\,dx\,dy = \iint_{D^*} f(g_1(u,v), g_2(u,v))\,\left|\frac{\partial(x,y)}{\partial(u,v)}\right|\,du\,dv,$$

where $\left|\frac{\partial(x,y)}{\partial(u,v)}\right| = |\det J_\mathbf{g}(u,v)|$ is the absolute value of the Jacobian determinant:

$$\frac{\partial(x,y)}{\partial(u,v)} = \det\begin{pmatrix}\partial x/\partial u & \partial x/\partial v \\ \partial y/\partial u & \partial y/\partial v\end{pmatrix} = \frac{\partial x}{\partial u}\frac{\partial y}{\partial v} - \frac{\partial x}{\partial v}\frac{\partial y}{\partial u}.$$

## Geometric Meaning of the Jacobian

The Jacobian determinant $|\partial(x,y)/\partial(u,v)|$ at the point $(u_0,v_0)$ is the factor by which the transformation $\mathbf{g}$ scales areas near $(u_0,v_0)$. Precisely: a small rectangle $[u_0, u_0+\Delta u]\times[v_0, v_0+\Delta v]$ in the $(u,v)$-plane maps to a small parallelogram in the $(x,y)$-plane with area approximately $|\det J_\mathbf{g}(u_0,v_0)|\cdot\Delta u\,\Delta v$.

This is the content of the inverse function theorem: the linearization of $\mathbf{g}$ at $(u_0,v_0)$ is $J_\mathbf{g}(u_0,v_0)$, and linear maps scale areas by their determinant.

## Derivation Sketch

Partition $D^*$ into small rectangles $R^*_k$ of area $\Delta u\,\Delta v$. Their images $R_k = \mathbf{g}(R^*_k)$ partition $D$ approximately (up to boundary issues). The area of $R_k$ is approximately $|\det J_\mathbf{g}(u_k,v_k)|\Delta u\,\Delta v$. The Riemann sum for $\iint_D f\,dA$ becomes:

$\sum_k f(g(u_k,v_k))\cdot\text{Area}(R_k)\approx\sum_k f(g(u_k,v_k))|\det J_\mathbf{g}(u_k,v_k)|\Delta u\,\Delta v,$

which is a Riemann sum for $\iint_{D^*}f(\mathbf{g}(u,v))|\det J_\mathbf{g}(u,v)|\,du\,dv$. Taking the limit gives the formula.

## Standard Example: Polar Coordinates

$(x,y) = (r\cos\theta, r\sin\theta)$, so $J_\mathbf{g} = \begin{pmatrix}\cos\theta&-r\sin\theta\\\sin\theta&r\cos\theta\end{pmatrix}$, $\det J_\mathbf{g} = r$.

$$\iint_D f(x,y)\,dx\,dy = \iint_{D^*} f(r\cos\theta, r\sin\theta)\,r\,dr\,d\theta.$$

The factor $r\,dr\,d\theta$ is the area element in polar coordinates.

## Example: Linear Transformation

$(x,y) = (au+bv, cu+dv)$ (a linear map). The Jacobian is $\begin{pmatrix}a&b\\c&d\end{pmatrix}$, determinant $ad-bc$.

$\iint_D f(x,y)\,dx\,dy = |ad-bc|\iint_{D^*}f(au+bv,cu+dv)\,du\,dv$.

If the linear map has $|ad-bc|=1$ (it preserves area), then the area element is unchanged.

## Example: Elliptical Region

Evaluate $\iint_D e^{-(x^2+y^2)}\,dA$ where $D$ is the ellipse $x^2/a^2+y^2/b^2\leq 1$.

Substitute $x = ar\cos\theta$, $y = br\sin\theta$ (mapping the disk $r\leq 1$ to the ellipse). Jacobian: $\det J = ab r$ (the $a,b$ come from the chain rule).

$\iint_D e^{-(a^2r^2\cos^2\theta+b^2r^2\sin^2\theta)}\cdot abr\,dr\,d\theta$.

For the specific case $a=b$ (circle): $\int_0^{2\pi}\int_0^1 e^{-r^2}\cdot r\,dr\,d\theta = 2\pi\cdot\frac{1-e^{-1}}{2} = \pi(1-e^{-1})$.

## The Inverse Jacobian

If $\mathbf{g}: D^*\to D$ has Jacobian $J_\mathbf{g}$ and $\mathbf{g}^{-1}: D\to D^*$ has Jacobian $J_{\mathbf{g}^{-1}}$, then $J_\mathbf{g}\cdot J_{\mathbf{g}^{-1}} = I$, so $\det J_{\mathbf{g}^{-1}} = 1/\det J_\mathbf{g}$, and the two change-of-variables formulas (one for $\mathbf{g}$, one for $\mathbf{g}^{-1}$) are consistent: applying both gives the identity.

## Common Pitfalls

The Jacobian must be the determinant of the Jacobian matrix of $\mathbf{g}$, taken in absolute value. It is easy to forget the absolute value, leading to a sign error.

Also, the transformation $\mathbf{g}$ must be one-to-one (bijective) on $D^*$, except possibly on a set of zero area. If $\mathbf{g}$ is 2-to-1, the formula double-counts the domain, giving twice the correct value. For polar coordinates, the boundary $r=0$ and the line $\theta=0$ (shared by $\theta=0$ and $\theta=2\pi$) form a zero-area set, so the bijection condition is satisfied in the interior.
