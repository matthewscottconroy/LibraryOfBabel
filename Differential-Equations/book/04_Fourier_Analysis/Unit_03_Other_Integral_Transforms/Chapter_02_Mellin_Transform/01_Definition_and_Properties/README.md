# The Mellin Transform: Definition and Properties

The Mellin transform takes a function $f : (0, \infty) \to \mathbb{C}$ and maps it to a function of a complex variable $s$. It is the natural integral transform associated with the group $(\mathbb{R}^+, \times)$ under multiplication, in the same way that the Fourier transform is natural for $(\mathbb{R}, +)$ under addition.

## Definition

**Definition.** The **Mellin transform** of $f : (0,\infty) \to \mathbb{C}$ is
$$\mathcal{M}[f](s) = \int_0^\infty f(x)\,x^{s-1}\,dx,$$
for values of $s \in \mathbb{C}$ where the integral converges absolutely.

The kernel $x^{s-1} = e^{(s-1)\ln x}$ is the **multiplicative character** of $\mathbb{R}^+$: it satisfies $K(xy) = K(x)K(y)$ when $K(x) = x^{s-1}$ (since $(xy)^{s-1} = x^{s-1}y^{s-1}$).

The **inverse Mellin transform** is
$$f(x) = \frac{1}{2\pi i}\int_{c-i\infty}^{c+i\infty} \mathcal{M}[f](s)\,x^{-s}\,ds,$$
where $c$ is a real number in the strip of convergence.

## Region of Convergence

The integral splits near $0$ and near $\infty$. For the Mellin transform to converge:
- Near $x = 0$: need $x^{\text{Re}(s)-1}|f(x)|$ integrable, i.e., if $f(x) = O(x^{-\alpha})$ as $x \to 0^+$, then $\text{Re}(s) > \alpha$.
- Near $x = \infty$: need $x^{\text{Re}(s)-1}|f(x)|$ integrable, i.e., if $f(x) = O(x^{-\beta})$ as $x \to \infty$, then $\text{Re}(s) < \beta$.

The ROC is the vertical strip $\alpha < \text{Re}(s) < \beta$, nonempty when $\alpha < \beta$.

## Relation to the Fourier and Laplace Transforms

**Substitution $x = e^t$:** Let $g(t) = f(e^t)$ for $t \in \mathbb{R}$. Then
$$\mathcal{M}[f](s) = \int_0^\infty f(x)x^{s-1}\,dx = \int_{-\infty}^\infty f(e^t)e^{(s-1)t}e^t\,dt = \int_{-\infty}^\infty g(t)e^{st}\,dt = \mathcal{B}[g](-s).$$
(Here $\mathcal{B}$ is the bilateral Laplace transform and $-s$ comes from the sign convention $e^{-st}$ in $\mathcal{B}$; with the sign as above, $\mathcal{M}[f](s) = \mathcal{B}_-[g](s)$ where $\mathcal{B}_-[g](s) = \int g(t)e^{st}\,dt$.)

This converts the Mellin transform into a bilateral Laplace transform in logarithmic coordinates. In particular, all of Laplace transform theory applies to the Mellin transform via this substitution.

## Fundamental Properties

Let $F(s) = \mathcal{M}[f](s)$.

**Scaling:** $\mathcal{M}[f(ax)](s) = a^{-s}F(s)$ for $a > 0$.

**Multiplication by a power:** $\mathcal{M}[x^a f(x)](s) = F(s+a)$.

**Derivative:** $\mathcal{M}[f'(x)](s) = -(s-1)F(s-1)$. (Prove by integrating by parts.)

**Differential operator:** $\mathcal{M}\left[\left(x\frac{d}{dx}\right)^n f\right](s) = (-s)^n F(s)$.

The operator $x\,d/dx$ is the natural derivative for multiplicative problems (it is the infinitesimal generator of the scaling group $f(x) \mapsto f(ax)$), just as $d/dx$ is the generator of translations.

## The Mellin Convolution

**Definition.** The **Mellin convolution** (or multiplicative convolution) of $f$ and $g$ is
$$(f \star g)(x) = \int_0^\infty f(y)\,g\!\left(\frac{x}{y}\right)\frac{dy}{y}.$$

**Theorem.** $\mathcal{M}[f \star g](s) = \mathcal{M}[f](s)\cdot\mathcal{M}[g](s)$.

**Proof.** Substitute $y = xu$:
$$\mathcal{M}[f\star g](s) = \int_0^\infty \left(\int_0^\infty f(y)g(x/y)\frac{dy}{y}\right)x^{s-1}\,dx.$$
Interchange integrals (Fubini), then substitute $x = uy$:
$$= \int_0^\infty f(y)\int_0^\infty g(u)(uy)^{s-1}u\,\frac{du\,dy}{y} = \int_0^\infty f(y)y^{s-1}\,dy \cdot\int_0^\infty g(u)u^{s-1}\,du = F(s)G(s).$$

This is the Mellin analog of the Fourier convolution theorem.

## Key Example: The Gamma Function

The Gamma function is a Mellin transform:
$$\Gamma(s) = \mathcal{M}[e^{-x}](s) = \int_0^\infty e^{-x}x^{s-1}\,dx, \quad \text{Re}(s) > 0.$$

This is one of the most important special functions in mathematics. Properties of $\Gamma$ follow from Mellin transform properties:
- The recurrence $\Gamma(s+1) = s\Gamma(s)$ follows from $\mathcal{M}[(-x e^{-x})'](s) = \ldots$ (integration by parts).
- The reflection formula $\Gamma(s)\Gamma(1-s) = \pi/\sin(\pi s)$ follows from computing the Mellin convolution of $e^{-x}$ with itself in two ways.

## The Beta Function as a Mellin Transform

The Beta function $B(a,b) = \int_0^1 x^{a-1}(1-x)^{b-1}\,dx$ appears as a Mellin transform. Specifically, $B(a,b) = \mathcal{M}[(1-x)^{b-1}\mathbf{1}_{[0,1]}(x)](a)$ for $\text{Re}(a), \text{Re}(b) > 0$. The identity $B(a,b) = \Gamma(a)\Gamma(b)/\Gamma(a+b)$ can be proved by computing the Mellin convolution of $e^{-x}$ with itself.

## Worked Example: $f(x) = 1/(1+x)$

$$\mathcal{M}\left[\frac{1}{1+x}\right](s) = \int_0^\infty \frac{x^{s-1}}{1+x}\,dx.$$

For $0 < \text{Re}(s) < 1$, this integral converges. Substitute $x = t/(1-t)$... or use the Beta function: substitute $x = u/(1-u)$, i.e., $1+x = 1/(1-u)$, $dx = du/(1-u)^2$:

Alternatively, use the result $\int_0^\infty \frac{x^{s-1}}{1+x}\,dx = \frac{\pi}{\sin(\pi s)}$ for $0 < \text{Re}(s) < 1$. This classic result is proved by the residue theorem (summing contributions from the poles at $x = -1 = e^{i\pi}$) or by Euler's reflection formula for the Gamma function.

So $\mathcal{M}[1/(1+x)](s) = \pi/\sin(\pi s)$, valid in the strip $0 < \text{Re}(s) < 1$.
