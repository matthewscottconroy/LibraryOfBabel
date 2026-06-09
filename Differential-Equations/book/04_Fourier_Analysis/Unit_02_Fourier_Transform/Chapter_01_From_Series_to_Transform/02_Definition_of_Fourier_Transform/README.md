# Definition of the Fourier Transform

The Fourier integral theorem motivates a specific mapping: given a function $f$, associate to it a new function $\hat{f}$ that encodes how much of each frequency is present in $f$. This mapping is the Fourier transform. Making its definition precise requires specifying a domain (for which functions is the transform defined?), a normalization convention, and a framework for verifying when the inverse formula applies.

## Formal Definition

**Definition.** For $f \in L^1(\mathbb{R})$, the **Fourier transform** of $f$ is the function $\hat{f} : \mathbb{R} \to \mathbb{C}$ defined by
$$\hat{f}(\xi) = \mathcal{F}[f](\xi) = \int_{-\infty}^\infty f(x)\,e^{-2\pi i\xi x}\,dx.$$

The integral converges absolutely for every $\xi \in \mathbb{R}$, since $|e^{-2\pi i\xi x}| = 1$ and $f \in L^1$:
$$|\hat{f}(\xi)| \leq \int_{-\infty}^\infty |f(x)||e^{-2\pi i\xi x}|\,dx = \int_{-\infty}^\infty |f(x)|\,dx = \|f\|_1 < \infty.$$

The **inverse Fourier transform** is
$$\mathcal{F}^{-1}[g](x) = \int_{-\infty}^\infty g(\xi)\,e^{2\pi i\xi x}\,d\xi.$$

Under the conditions of the Fourier integral theorem ($f \in L^1$ and piecewise smooth), $\mathcal{F}^{-1}[\mathcal{F}[f]] = f$ at points of continuity.

## Properties of $\hat{f}$ for $f \in L^1$

If $f \in L^1(\mathbb{R})$, then:
1. $\hat{f}$ is well-defined, bounded by $\|\hat{f}\|_\infty \leq \|f\|_1$.
2. $\hat{f}$ is uniformly continuous on $\mathbb{R}$.
3. $\hat{f}(\xi) \to 0$ as $|\xi| \to \infty$ (Riemann-Lebesgue lemma).
4. $\hat{f}$ need not be in $L^1(\mathbb{R})$, so the inversion integral $\int \hat{f}(\xi)e^{2\pi i\xi x}\,d\xi$ may fail to converge absolutely.

The failure of $\hat{f}$ to be in $L^1$ even when $f \in L^1$ is a technical inconvenience. The Schwartz space (rapidly decaying smooth functions) is the natural domain where the Fourier transform is especially well-behaved.

## The Gaussian: A Fundamental Example

The most important single example is the **Gaussian** $f(x) = e^{-\pi x^2}$. This function is its own Fourier transform.

**Computation:**
$$\hat{f}(\xi) = \int_{-\infty}^\infty e^{-\pi x^2}e^{-2\pi i\xi x}\,dx.$$
Complete the square in the exponent: $-\pi x^2 - 2\pi i\xi x = -\pi(x + i\xi)^2 - \pi\xi^2$.
$$\hat{f}(\xi) = e^{-\pi\xi^2}\int_{-\infty}^\infty e^{-\pi(x+i\xi)^2}\,dx.$$
The integral $\int_{-\infty}^\infty e^{-\pi(x+i\xi)^2}\,dx$ can be evaluated by contour integration (shifting the contour of integration from the real axis to the line $\text{Im}(z) = \xi$, which contributes no residues since the integrand is entire) to give $\int_{-\infty}^\infty e^{-\pi u^2}\,du = 1$ (the standard Gaussian integral with the normalization $\int e^{-\pi x^2}\,dx = 1$).

Therefore $\hat{f}(\xi) = e^{-\pi\xi^2}$. The Gaussian is a **fixed point** of the Fourier transform.

More generally, $\mathcal{F}[e^{-\pi\alpha x^2}](\xi) = \alpha^{-1/2}e^{-\pi\xi^2/\alpha}$ for $\alpha > 0$. A narrow Gaussian (large $\alpha$) transforms to a broad one (small $\alpha$), illustrating the time-frequency uncertainty principle.

## The Uncertainty Principle

The width of a function and the width of its Fourier transform cannot both be made arbitrarily small simultaneously. This is the **Heisenberg uncertainty principle** in mathematics (corresponding to the physical principle in quantum mechanics, where position and momentum are related by the Fourier transform).

**Theorem.** For $f \in L^2(\mathbb{R})$ with $\|f\|_2 = 1$, let $\Delta x^2 = \int x^2|f(x)|^2\,dx$ and $\Delta\xi^2 = \int \xi^2|\hat{f}(\xi)|^2\,d\xi$. Then
$$\Delta x \cdot \Delta\xi \geq \frac{1}{4\pi}.$$
Equality holds if and only if $f(x) = Ce^{-\pi\alpha x^2}$ for some $\alpha > 0$ and constant $C$ — the Gaussian.

## Further Examples

**Rectangular pulse:** $f(x) = \mathbf{1}_{[a,b]}(x)$.
$$\hat{f}(\xi) = \int_a^b e^{-2\pi i\xi x}\,dx = \frac{e^{-2\pi ia\xi} - e^{-2\pi ib\xi}}{2\pi i\xi} = (b-a)\,\text{sinc}((b-a)\xi)\,e^{-\pi i(a+b)\xi}.$$
For the symmetric case $a=-T/2$, $b=T/2$: $\hat{f}(\xi) = T\,\text{sinc}(T\xi)$.

**One-sided exponential:** $f(x) = e^{-ax}\mathbf{1}_{[0,\infty)}(x)$, $a > 0$.
$$\hat{f}(\xi) = \int_0^\infty e^{-ax}e^{-2\pi i\xi x}\,dx = \int_0^\infty e^{-(a+2\pi i\xi)x}\,dx = \frac{1}{a + 2\pi i\xi}.$$

**Triangle function:** $\Lambda(x) = \max(1-|x|, 0)$. Note $\Lambda = \mathbf{1}_{[-1/2,1/2]} * \mathbf{1}_{[-1/2,1/2]}$ (convolution). By the convolution theorem (Chapter 02), $\hat{\Lambda}(\xi) = \text{sinc}^2(\xi)$.

## The Schwartz Space

The natural domain for the Fourier transform, where everything works optimally, is the **Schwartz space** $\mathcal{S}(\mathbb{R})$: the space of smooth functions $f$ for which $\sup_x |x^k f^{(j)}(x)| < \infty$ for all $k, j \geq 0$.

The Schwartz space has several important properties:
- $\mathcal{S}(\mathbb{R}) \subset L^1(\mathbb{R}) \cap L^2(\mathbb{R})$.
- The Fourier transform maps $\mathcal{S}(\mathbb{R})$ to itself bijectively.
- On $\mathcal{S}(\mathbb{R})$, the inversion formula holds pointwise and absolutely.
- $\mathcal{S}(\mathbb{R})$ is dense in $L^p(\mathbb{R})$ for $1 \leq p < \infty$.

The Gaussian $e^{-\pi x^2}$ is in $\mathcal{S}$, as are all functions of the form $p(x)e^{-\pi x^2}$ for polynomial $p$. The rectangular pulse $\mathbf{1}_{[-1,1]}$ is not in $\mathcal{S}$ (not smooth), and its transform sinc is not in $L^1$ (not absolutely integrable), illustrating why the $L^1$ theory requires care.
