# Higher Order Terms

The first- and second-order Taylor approximations capture linear and quadratic behavior. But some functions have vanishing gradient and Hessian at a point, making the second-order test inconclusive; the behavior is then governed by higher-order terms. More broadly, the full Taylor series of an analytic function — with all powers of $\mathbf{h}$ — provides an exact representation of the function in a neighborhood of the expansion point. This section introduces the multi-index notation needed to write these higher-order terms compactly, states the general Taylor theorem, and discusses convergence.

## Multi-Index Notation

A **multi-index** is a tuple $\alpha = (\alpha_1, \alpha_2, \ldots, \alpha_n)\in\mathbb{Z}_{\geq 0}^n$. Its key attributes:

- **Order:** $|\alpha| = \alpha_1 + \alpha_2 + \cdots + \alpha_n$.
- **Factorial:** $\alpha! = \alpha_1!\,\alpha_2!\cdots\alpha_n!$.
- **Monomial:** $\mathbf{h}^\alpha = h_1^{\alpha_1}h_2^{\alpha_2}\cdots h_n^{\alpha_n}$.
- **Partial derivative:** $\partial^\alpha f = \frac{\partial^{|\alpha|} f}{\partial x_1^{\alpha_1}\cdots\partial x_n^{\alpha_n}}$.

**Example** (for $n=2$, $\alpha = (2,1)$): $|\alpha| = 3$, $\alpha! = 2$, $\mathbf{h}^\alpha = h_1^2h_2$, $\partial^\alpha f = f_{x_1x_1x_2} = \frac{\partial^3 f}{\partial x_1^2\partial x_2}$.

## The General Taylor Theorem

**Theorem.** Let $f:\mathbb{R}^n\to\mathbb{R}$ be of class $C^{k+1}$ on an open set containing the segment $[\mathbf{a}, \mathbf{a}+\mathbf{h}]$. Then

$$f(\mathbf{a}+\mathbf{h}) = \sum_{|\alpha|\leq k}\frac{\partial^\alpha f(\mathbf{a})}{\alpha!}\mathbf{h}^\alpha + R_k(\mathbf{h}),$$

where $R_k(\mathbf{h}) = o(\|\mathbf{h}\|^k)$ (Peano remainder), or in the Lagrange form:

$$R_k(\mathbf{h}) = \sum_{|\alpha|=k+1}\frac{\partial^\alpha f(\mathbf{a}+\theta\mathbf{h})}{\alpha!}\mathbf{h}^\alpha$$

for some $\theta\in(0,1)$.

**Expansion through order 2** (recovering the previous sections):

$|\alpha|=0$: $\alpha=(0,\ldots,0)$, $\mathbf{h}^\alpha=1$, $\partial^\alpha f = f$. Term: $f(\mathbf{a})$.

$|\alpha|=1$: $\alpha=\mathbf{e}_i$, $\mathbf{h}^\alpha = h_i$, $\partial^\alpha f = f_{x_i}$. Sum: $\sum_i f_{x_i}h_i = \nabla f\cdot\mathbf{h}$.

$|\alpha|=2$: two types. Pure: $\alpha=2\mathbf{e}_i$, $\alpha!=2$, term $\frac{f_{x_ix_i}}{2}h_i^2$. Mixed: $\alpha=\mathbf{e}_i+\mathbf{e}_j$ ($i<j$), $\alpha!=1$, term $f_{x_ix_j}h_ih_j$. Total: $\frac{1}{2}\sum_{i,j}f_{x_ix_j}h_ih_j = \frac{1}{2}\mathbf{h}^TH_f\mathbf{h}$.

## Third-Order Terms in Two Variables

For $f(x,y)$ expanded at $(a,b)$ with $h = x-a$, $k = y-b$:

$$f(a+h,b+k) = f + (f_xh+f_yk) + \frac{1}{2}(f_{xx}h^2+2f_{xy}hk+f_{yy}k^2)$$
$$+ \frac{1}{6}(f_{xxx}h^3+3f_{xxy}h^2k+3f_{xyy}hk^2+f_{yyy}k^3) + \cdots$$

The coefficients $1, 3, 3, 1$ are binomial coefficients, just as in the one-variable expansion of $(h+k)^3$. More precisely, the $k$-th order terms in $n=2$ variables correspond to the binomial expansion of $(hD_x+kD_y)^k f/k!$ where $D_x, D_y$ are partial derivative operators.

## When Higher-Order Terms Matter

**Case 1: Degenerate critical points.** If $f$ has a critical point at $\mathbf{a}$ and the Hessian $H_f(\mathbf{a}) = 0$ (or is semidefinite), the second-order test is inconclusive. The nature of the critical point is determined by the leading nonvanishing term in the Taylor expansion.

**Example.** $f(x,y) = x^4+y^4$. At $(0,0)$: $\nabla f = (0,0)$, $H_f = 0$. Second-order test fails. But $f(h,k) = h^4+k^4 \geq 0$ with equality only at $(0,0)$, so it is a global minimum. This is detected by the fourth-order terms.

**Example.** $f(x,y) = x^3+y^3$. At $(0,0)$: $\nabla f = (0,0)$, $H_f = 0$. Third-order terms: $h^3+k^3$. Taking $h = k > 0$: $f(h,h) = 2h^3 > 0$. Taking $h > 0, k = -h$: $f(h,-h) = 0$. Taking $h < 0, k = -h < 0$... wait, $h < 0$, so $h^3 < 0$ and $(-h)^3 = -h^3 > 0$, giving $0$. Taking both negative: $f(-h,-h) = -2h^3 < 0$. So the function takes both positive and negative values near the origin: saddle point.

**Case 2: Taylor series of analytic functions.** A function $f$ is **real analytic** at $\mathbf{a}$ if its Taylor series converges to $f$ in some open ball $B(\mathbf{a}, r)$. The radius of convergence depends on the function; it is at least the distance to the nearest singularity in the complex domain. For elementary functions (polynomials, $e^x$, $\sin x$, $\ln x$), the Taylor series converges on the natural domain.

**Example.** For $f(x,y) = e^{x+y}$ at $(0,0)$:

$$e^{x+y} = \sum_{k=0}^\infty \frac{(x+y)^k}{k!} = \sum_{k=0}^\infty \frac{1}{k!}\sum_{j=0}^k\binom{k}{j}x^jy^{k-j} = \sum_{\alpha_1+\alpha_2=0,1,2,\ldots}\frac{x^{\alpha_1}y^{\alpha_2}}{\alpha_1!\alpha_2!},$$

which matches the multi-index formula exactly: $\sum_\alpha \frac{\partial^\alpha f(0)}{\alpha!}x^\alpha = \sum_\alpha \frac{x^\alpha}{\alpha!}$ (since $\partial^\alpha e^{x+y}\big|_{(0,0)} = 1$).

## Multinomial Theorem and Symmetry

The $k$-th order terms in the Taylor expansion of $f(\mathbf{a}+\mathbf{h})$ form a homogeneous polynomial of degree $k$ in $\mathbf{h}$, called the **$k$-th order part** of the Taylor expansion. By Clairaut's theorem (for $f\in C^k$), all permutations of the order of differentiation give the same result, so the coefficients of this polynomial are symmetric. This symmetry is exactly what makes the multi-index formula well-defined.

## Practical Implications

In practice, Taylor series to order 2 or 3 suffice for most purposes. The key use cases:
1. **Optimization:** Second-order terms detect the type of critical point.
2. **Numerical integration:** Higher-order terms bound the error in numerical methods (Runge-Kutta methods for ODEs use Taylor expansions of the solution).
3. **Perturbation theory:** In physics and engineering, the response to a small perturbation is a Taylor series in the perturbation parameter.
4. **Asymptotic analysis:** Understanding the behavior of solutions to differential equations near special points (fixed points, branch points) often requires Taylor expansion to several orders.
