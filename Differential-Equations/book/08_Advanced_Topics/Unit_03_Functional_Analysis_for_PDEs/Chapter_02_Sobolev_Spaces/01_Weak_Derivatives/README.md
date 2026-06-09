# Weak Derivatives

The notion of a weak (or distributional) derivative extends differentiation to functions that are not differentiable in the classical sense. This extension is the cornerstone of Sobolev space theory and the rigorous foundation for the variational approach to PDEs. A function has a weak derivative if it satisfies the integration by parts identity that the classical derivative would satisfy—without requiring the classical derivative to exist.

## Motivation: Integration by Parts

If $f \in C^1(\Omega)$ (for an open set $\Omega \subset \mathbb{R}^n$) and $\phi \in C_c^\infty(\Omega)$ (compactly supported test function), then integration by parts gives:

$$\int_\Omega f \partial_{x_i}\phi \, dx = -\int_\Omega (\partial_{x_i} f)\phi \, dx,$$

with no boundary terms (since $\phi = 0$ on $\partial\Omega$). This identity relates the classical derivative $\partial_{x_i} f$ to $f$ through a pairing with test functions.

The idea of the weak derivative is to use this identity as the definition—to say that $g$ is the weak partial derivative of $f$ if the identity $\int f\partial_{x_i}\phi = -\int g\phi$ holds for all test functions $\phi$, regardless of whether $f$ is classically differentiable.

## Definition

Let $\Omega \subset \mathbb{R}^n$ be an open set, $f \in L^1_{\text{loc}}(\Omega)$, and $\alpha = (\alpha_1, \ldots, \alpha_n)$ a multi-index. A function $g \in L^1_{\text{loc}}(\Omega)$ is a **weak $\alpha$-th partial derivative** of $f$ if:

$$\int_\Omega f \, D^\alpha\phi \, dx = (-1)^{|\alpha|} \int_\Omega g \phi \, dx \quad \text{for all } \phi \in C_c^\infty(\Omega).$$

If such $g$ exists, it is unique (up to sets of measure zero: if $g$ and $\tilde{g}$ both satisfy the identity, then $\int (g - \tilde{g})\phi = 0$ for all $\phi$, implying $g = \tilde{g}$ a.e.). We write $D^\alpha f = g$.

## Examples

**Absolute value on $\mathbb{R}$:** Consider $f(x) = |x|$ on $\mathbb{R}$. The function $g(x) = \text{sgn}(x) = \begin{cases}1 & x > 0 \\ -1 & x < 0\end{cases}$ is the weak derivative:

$$\int_{-\infty}^\infty |x|\phi'(x) \, dx = \int_0^\infty x\phi'(x) \, dx + \int_{-\infty}^0 (-x)\phi'(x) \, dx.$$

Integrating by parts on each interval: $= -\int_0^\infty \phi \, dx + x\phi|_0^\infty + \int_{-\infty}^0 \phi \, dx - x\phi|_{-\infty}^0 = -\int_0^\infty \phi + \int_{-\infty}^0 \phi = -\int \text{sgn}(x)\phi$. (Since $\phi$ is compactly supported, all boundary evaluations at $\pm\infty$ vanish, and the evaluation at 0 from both sides gives no contribution because $0 \cdot \phi(0) = 0$.)

So $|x|' = \text{sgn}(x)$ weakly. Note $\text{sgn}(x)$ is itself not differentiable at $x = 0$ in the classical sense; its weak derivative is $2\delta(0)$, but $2\delta$ is a distribution, not an $L^1_{\text{loc}}$ function—so $\text{sgn}(x)$ does not have a weak derivative in $L^1$.

**Step function:** $H(x) = \mathbf{1}_{[0,\infty)}$ does not have a weak derivative in $L^1_{\text{loc}}$. For $\int H\phi' = -\phi(0)$ is not of the form $-\int g\phi$ for any $g \in L^1_{\text{loc}}$ (as shown in the distribution theory sections). The weak derivative of $H$ is the Dirac delta, which is not an $L^1$ function.

**A function in $H^1$ but not $C^1$:** Consider $f(x) = \max(0, x)$ on $(-1, 1)$. Classically, $f$ is not differentiable at $x = 0$. But $g(x) = H(x) = \mathbf{1}_{(0,1)}$ is a weak derivative: for any $\phi \in C_c^\infty((-1,1))$:

$$\int_{-1}^1 \max(0,x)\phi' \, dx = \int_0^1 x\phi'(x) \, dx = -\int_0^1 \phi(x) \, dx + [x\phi(x)]_0^1 = -\int_0^1 \phi = -\int_{-1}^1 H(x)\phi(x) \, dx.$$

So $f' = H$ weakly, and since $H \in L^2(-1,1)$, the function $f \in H^1(-1,1)$ but $f \notin C^1(-1,1)$.

## Properties of Weak Derivatives

**Linearity.** If $D^\alpha f = g$ and $D^\alpha h = k$ weakly, then $D^\alpha(af + bh) = ag + bk$ weakly.

**Consistency.** If $f \in C^{|\alpha|}(\Omega)$, then its weak $\alpha$-derivative equals its classical $\alpha$-derivative (a.e.).

**Chain rule.** If $f \in W^{1,p}(\Omega)$ and $F \in C^1(\mathbb{R})$ with $F' \in L^\infty$, then $F \circ f \in W^{1,p}(\Omega)$ and $D_i(F \circ f) = F'(f) D_i f$ weakly.

**Product rule.** If $f \in W^{1,p}(\Omega)$ and $g \in C^1(\bar\Omega)$, then $fg \in W^{1,p}(\Omega)$ and $D_i(fg) = (D_i f)g + f(D_i g)$ weakly.

## Weak vs. Distributional Derivative

The **distributional derivative** of $f \in L^1_{\text{loc}}$ always exists (as a distribution). The **weak derivative** requires the distributional derivative to be representable by an $L^1_{\text{loc}}$ function. Concretely: $f$ has a weak $\alpha$-derivative in $L^p$ if and only if the distributional derivative $D^\alpha T_f$ is a regular distribution $T_g$ for some $g \in L^p$.

The Sobolev space $W^{k,p}$ singles out functions whose distributional derivatives are in $L^p$—neither more nor less. This is exactly the right regularity for the variational formulation of elliptic PDEs.

## Sobolev's Inequality and First Results

**Lemma (Poincaré inequality on bounded domains).** Let $\Omega \subset \mathbb{R}^n$ be a bounded open set. For every $f \in H^1_0(\Omega) = W^{1,2}_0(\Omega)$ (i.e., $f$ has zero boundary values in the weak sense):

$$\|f\|_{L^2(\Omega)} \leq C_\Omega \|\nabla f\|_{L^2(\Omega)},$$

where $C_\Omega$ depends only on $\Omega$.

**Proof idea.** For $f \in C_c^\infty(\Omega)$, write $f(x) = \int_0^{x_1} \partial_1 f(t, x_2, \ldots, x_n) \, dt$ (extending by zero outside $\Omega$). The Cauchy-Schwarz inequality and integration over $\Omega$ give the result, with $C_\Omega$ proportional to the diameter of $\Omega$. $\square$

The Poincaré inequality is the reason the seminorm $\|\nabla f\|_{L^2}$ is equivalent to the full $H^1$ norm on $H^1_0(\Omega)$ for bounded domains. This coercivity is what makes the Dirichlet Laplacian (with zero boundary conditions) invertible, and is the key hypothesis of the Lax-Milgram theorem as applied to the Poisson equation.
