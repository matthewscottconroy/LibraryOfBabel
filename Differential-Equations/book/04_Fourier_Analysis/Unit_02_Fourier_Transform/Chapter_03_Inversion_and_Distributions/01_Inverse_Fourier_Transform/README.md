# The Inverse Fourier Transform

The Fourier transform $\mathcal{F}$ sends a function $f$ to its frequency representation $\hat{f}$. The inverse question — given $\hat{f}$, recover $f$ — is answered by the inverse Fourier transform formula. This section examines the conditions under which the formula works, what "works" means precisely, and the subtleties that arise when those conditions are not met.

## The Inverse Transform Formula

**Definition.** The **inverse Fourier transform** of a function $g : \mathbb{R} \to \mathbb{C}$ is
$$\mathcal{F}^{-1}[g](x) = \int_{-\infty}^\infty g(\xi)\,e^{2\pi i\xi x}\,d\xi,$$
when this integral exists.

Comparing with $\mathcal{F}[f](\xi) = \int_{-\infty}^\infty f(x)e^{-2\pi i\xi x}\,dx$, the only difference is the sign in the exponent. This symmetry means $\mathcal{F}^{-1}[g](x) = \mathcal{F}[g](-x)$: the inverse transform is the forward transform evaluated at $-x$. In particular, $\mathcal{F}$ and $\mathcal{F}^{-1}$ have the same analytic properties.

## When Does Inversion Work?

**Theorem (Inversion for $L^1 \cap L^2$).** If $f \in L^1(\mathbb{R})$ and $\hat{f} \in L^1(\mathbb{R})$ (both $f$ and its transform are absolutely integrable), then
$$\mathcal{F}^{-1}[\hat{f}](x) = f(x) \quad \text{for a.e. } x.$$
At every point of continuity of $f$, the equality holds exactly.

The condition $\hat{f} \in L^1$ ensures the inversion integral converges absolutely. When $\hat{f} \notin L^1$ (as happens, for example, with $f = \mathbf{1}_{[-1,1]}$, which has $\hat{f} = \text{sinc} \notin L^1$), the inversion integral must be interpreted as a principal value or limit.

**Theorem (Principal Value Inversion).** If $f \in L^1(\mathbb{R})$, then
$$\lim_{R\to\infty}\int_{-R}^R \hat{f}(\xi)e^{2\pi i\xi x}\,d\xi = \frac{f(x^+) + f(x^-)}{2}$$
at every point where $f$ has one-sided limits and satisfies a Dini condition.

## Inversion via the Gaussian Approximation

An important technique for understanding inversion is the **Gauss-Weierstrass regularization**. For any $f \in L^1(\mathbb{R})$:
$$\lim_{\epsilon\to 0^+}\int_{-\infty}^\infty \hat{f}(\xi)e^{-\pi\epsilon\xi^2}e^{2\pi i\xi x}\,d\xi = f(x)$$
at every point of continuity of $f$. The Gaussian factor $e^{-\pi\epsilon\xi^2}$ makes the integral absolutely convergent for each $\epsilon > 0$, and as $\epsilon \to 0$ the approximation becomes exact.

This technique corresponds to convolving $f$ with the heat kernel (a Gaussian in $x$): the integral on the left is $\mathcal{F}^{-1}[\hat{f}\cdot G_\epsilon] = f * \mathcal{F}^{-1}[G_\epsilon]$, where $G_\epsilon$ is the Gaussian at scale $\epsilon$. As $\epsilon\to 0$, the kernel narrows and approaches the delta function, recovering $f$.

## Examples of Inversion

**Example 1: Inversion of $\hat{f}(\xi) = e^{-a|\xi|}$, $a > 0$.**

We know $\mathcal{F}[e^{-a|x|}](\xi) = \frac{2a}{a^2 + 4\pi^2\xi^2}$. By duality ($\mathcal{F}[\hat{f}](x) = f(-x)$), or directly:
$$\mathcal{F}^{-1}[e^{-a|\xi|}](x) = \int_{-\infty}^\infty e^{-a|\xi|}e^{2\pi i\xi x}\,d\xi = \frac{2a}{a^2 + 4\pi^2 x^2}.$$

**Example 2: Inversion of $\hat{f} = \text{sinc}$ (principal value).**

Since $\hat{f}(\xi) = \text{sinc}(\xi) = \frac{\sin(\pi\xi)}{\pi\xi}$ is the transform of the rectangular pulse $\mathbf{1}_{[-1/2,1/2]}$, the inversion should give
$$\int_{-\infty}^\infty \text{sinc}(\xi)e^{2\pi i\xi x}\,d\xi = \mathbf{1}_{(-1/2,1/2)}(x)$$
at points of continuity, and $1/2$ at $x = \pm 1/2$. The integral does not converge absolutely (sinc $\notin L^1$), but it converges conditionally (as a limit of $\int_{-R}^R$) to the values stated.

## The Fourier Transform as a Bijection on $L^2$

The central result — Plancherel's theorem — says that on $L^2(\mathbb{R})$, the Fourier transform is a bijective isometry. In particular:
- $\mathcal{F} : L^2(\mathbb{R}) \to L^2(\mathbb{R})$ is well-defined.
- $\|\hat{f}\|_2 = \|f\|_2$ for all $f \in L^2$.
- $\mathcal{F}^{-1}$ exists on $L^2$ and satisfies $\mathcal{F}^{-1}\mathcal{F} = \mathcal{F}\mathcal{F}^{-1} = \text{Id}$.

The extension to $L^2$ is done via approximation: given $f \in L^2$, define $f_R = f\cdot\mathbf{1}_{[-R,R]}$ (truncation). Then $f_R \in L^1 \cap L^2$ and $\hat{f}_R$ is well-defined. One shows that $\{\hat{f}_R\}$ is Cauchy in $L^2$ and defines $\hat{f} = \lim_{R\to\infty}\hat{f}_R$ in $L^2$. The inversion formula then holds in $L^2$: $\mathcal{F}^{-1}[\hat{f}] = f$ in the $L^2$ sense.

## Consistency of $\mathcal{F}^{-1}$ with $\mathcal{F}$

An elegant consequence of the duality formula is that applying the Fourier transform four times returns to the original function:
$$\mathcal{F}^4[f] = f.$$
This follows from $\mathcal{F}^2[f](x) = f(-x)$ (double transform gives reflection) and $\mathcal{F}^4[f](x) = \mathcal{F}^2[f(-x)] = f(x)$.

The eigenvalues of $\mathcal{F}$ are therefore fourth roots of unity: $\{1, i, -1, -i\}$. The eigenfunctions are the **Hermite functions** $\psi_n(x) = e^{-\pi x^2}H_n(x)$ where $H_n$ is the $n$-th Hermite polynomial. They form a complete orthonormal basis of $L^2(\mathbb{R})$ and satisfy $\mathcal{F}[\psi_n] = (-i)^n\psi_n$.
