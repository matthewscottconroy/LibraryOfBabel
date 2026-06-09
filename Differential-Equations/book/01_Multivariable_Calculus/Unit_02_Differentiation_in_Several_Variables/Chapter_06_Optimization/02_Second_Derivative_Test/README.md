# Second Derivative Test

Having found a critical point of $f$ — a point where $\nabla f = \mathbf{0}$ — the next question is whether it is a local minimum, local maximum, or saddle point. In one variable, the second derivative test answers this: $f''(a) > 0$ means minimum, $f''(a) < 0$ means maximum, $f''(a) = 0$ is inconclusive. In several variables, the second-order Taylor expansion shows that the local behavior near a critical point is governed by the Hessian matrix $H_f(\mathbf{a})$ — specifically, by whether the quadratic form $\mathbf{h}^T H_f(\mathbf{a})\mathbf{h}$ is always positive, always negative, or takes both signs.

## Statement of the Test

**Theorem (Second Derivative Test).** Let $f:\mathbb{R}^n\to\mathbb{R}$ be $C^2$ near a critical point $\mathbf{a}$ (so $\nabla f(\mathbf{a})=\mathbf{0}$). Let $H = H_f(\mathbf{a})$ be the Hessian at $\mathbf{a}$.

1. If $H$ is **positive definite**, then $\mathbf{a}$ is a **strict local minimum**.
2. If $H$ is **negative definite**, then $\mathbf{a}$ is a **strict local maximum**.
3. If $H$ is **indefinite** (has both positive and negative eigenvalues), then $\mathbf{a}$ is a **saddle point**.
4. If $H$ is **positive or negative semidefinite** (has a zero eigenvalue), then the test is **inconclusive**: $\mathbf{a}$ could be a min, max, or saddle.

**Proof of case 1.** Since $H$ is positive definite, the minimum eigenvalue $\lambda_{\min} > 0$, so $\mathbf{h}^T H\mathbf{h} \geq \lambda_{\min}\|\mathbf{h}\|^2$ for all $\mathbf{h}$. By the second-order Taylor expansion:

$f(\mathbf{a}+\mathbf{h}) - f(\mathbf{a}) = \frac{1}{2}\mathbf{h}^T H\mathbf{h} + o(\|\mathbf{h}\|^2) \geq \frac{\lambda_{\min}}{2}\|\mathbf{h}\|^2 + o(\|\mathbf{h}\|^2)$.

For small $\|\mathbf{h}\|$, the $o(\|\mathbf{h}\|^2)$ term is bounded in absolute value by $(\lambda_{\min}/4)\|\mathbf{h}\|^2$, so $f(\mathbf{a}+\mathbf{h}) - f(\mathbf{a}) \geq \frac{\lambda_{\min}}{4}\|\mathbf{h}\|^2 > 0$ for $\mathbf{h}\neq\mathbf{0}$. Hence $\mathbf{a}$ is a strict local minimum.

## The Two-Variable Test

For $f:\mathbb{R}^2\to\mathbb{R}$ with critical point $(a,b)$, let $A = f_{xx}(a,b)$, $B = f_{xy}(a,b)$, $C = f_{yy}(a,b)$, and $D = AC - B^2$ (the discriminant, equal to $\det H_f$).

- $D > 0$ and $A > 0$: local minimum. ($H$ is positive definite.)
- $D > 0$ and $A < 0$: local maximum. ($H$ is negative definite.)
- $D < 0$: saddle point. ($H$ is indefinite.)
- $D = 0$: inconclusive.

**Why $D$ determines definiteness for $2\times 2$ matrices.** The eigenvalues $\lambda_{1,2}$ of $\begin{pmatrix}A&B\\B&C\end{pmatrix}$ satisfy $\lambda_1+\lambda_2 = A+C$ (trace) and $\lambda_1\lambda_2 = AC-B^2 = D$ (determinant). If $D > 0$: both eigenvalues have the same sign; the sign is determined by $A+C$. If $A > 0$ and $D > 0$: trace is positive (both eigenvalues positive), so positive definite. If $D < 0$: eigenvalues have opposite signs, so indefinite.

## Worked Example

$f(x,y) = x^3 - 3x + y^2 - 4y$.

Critical points from the previous section: $(1,2)$ and $(-1,2)$.

$f_{xx} = 6x$, $f_{xy} = 0$, $f_{yy} = 2$.

At $(1,2)$: $A=6$, $B=0$, $C=2$, $D = 12 > 0$, $A > 0$: **local minimum**. $f(1,2) = -6$.

At $(-1,2)$: $A=-6$, $B=0$, $C=2$, $D = -12 < 0$: **saddle point**.

## Higher-Dimensional Test via Eigenvalues

In $\mathbb{R}^n$, the definiteness of $H_f(\mathbf{a})$ is determined by:
1. **Eigenvalues:** compute all $n$ eigenvalues $\lambda_1,\ldots,\lambda_n$ of $H_f$. All positive: positive definite (minimum). All negative: negative definite (maximum). Mixed signs: indefinite (saddle). Any zero eigenvalue: inconclusive.

2. **Sylvester's criterion:** $H$ is positive definite iff all leading principal minors $\Delta_k = \det\begin{pmatrix}H_{ij}\end{pmatrix}_{1\leq i,j\leq k}$ are positive. $H$ is negative definite iff the leading minors alternate in sign: $\Delta_1 < 0$, $\Delta_2 > 0$, $\Delta_3 < 0$, etc.

**Example ($n=3$).** $f(x,y,z) = x^2 + 2y^2 + 3z^2 + xy$.

$H_f = \begin{pmatrix}2&1&0\\1&4&0\\0&0&6\end{pmatrix}$.

$\Delta_1 = 2 > 0$. $\Delta_2 = 8-1 = 7 > 0$. $\Delta_3 = 6\cdot 7 = 42 > 0$.

By Sylvester: positive definite, so the origin is a local minimum.

## Inconclusive Cases

When $D = 0$ (for $n=2$) or $\det H_f = 0$ more generally, one must look at higher-order terms. There is no universal test.

**Example 1.** $f(x,y) = x^4+y^4$: $\nabla f(0) = (0,0)$, $H_f(0) = 0$, $D=0$. But $f(x,y) = x^4+y^4 \geq 0$: global minimum.

**Example 2.** $f(x,y) = x^4-y^4$: $\nabla f(0) = (0,0)$, $H_f(0) = 0$, $D=0$. But $f(h,0) = h^4 > 0$ and $f(0,k) = -k^4 < 0$: saddle point.

**Example 3.** $f(x,y) = -(x^4+y^4)$: global maximum at origin, $D=0$.

## Connection to Differential Equations

The second derivative test is the foundation for the linear stability analysis of equilibria of ODEs. Consider $\dot{x} = -f'(x)$ (gradient flow in 1D). A critical point $x^*$ with $f'(x^*) = 0$ is a stable equilibrium of the ODE iff $f''(x^*) > 0$ (minimum of $f$) and unstable iff $f''(x^*) < 0$ (maximum of $f$). In $n$ dimensions, $\dot{\mathbf{x}} = -\nabla f(\mathbf{x})$ has a stable equilibrium at $\mathbf{a}$ iff $H_f(\mathbf{a})$ is positive definite (iff $\mathbf{a}$ is a local minimum of $f$). This connects optimization theory directly to the stability theory of dynamical systems.
