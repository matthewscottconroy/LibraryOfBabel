# The Bilateral Laplace Transform

The standard (unilateral) Laplace transform requires $f(t) = 0$ for $t < 0$. Many signals and functions of interest are defined for all $t \in \mathbb{R}$: bilateral exponentials, two-sided decaying functions, or non-causal filters. The **bilateral Laplace transform** handles these cases and provides the natural framework for understanding the Laplace transform as an analytic continuation of the Fourier transform.

## Definition

**Definition.** The **bilateral Laplace transform** of $f : \mathbb{R} \to \mathbb{C}$ is
$$\mathcal{B}[f](s) = \int_{-\infty}^\infty f(t)\,e^{-st}\,dt, \quad s \in \mathbb{C},$$
for values of $s$ where the integral converges absolutely.

When $f(t) = 0$ for $t < 0$, this reduces to $\mathcal{L}[f](s) = \int_0^\infty f(t)e^{-st}\,dt$, the one-sided Laplace transform.

## Region of Convergence

The integral $\int_{-\infty}^\infty |f(t)|e^{-\sigma t}\,dt$ (with $\sigma = \text{Re}(s)$) splits into two parts:
- $\int_0^\infty |f(t)|e^{-\sigma t}\,dt$ converges for $\sigma > \sigma_{\text{right}}$, where $\sigma_{\text{right}}$ is the exponential growth rate as $t \to +\infty$.
- $\int_{-\infty}^0 |f(t)|e^{-\sigma t}\,dt$ converges for $\sigma < \sigma_{\text{left}}$, where $\sigma_{\text{left}}$ is related to the growth rate as $t \to -\infty$.

The **region of convergence (ROC)** of $\mathcal{B}[f]$ is the vertical strip
$$\{\sigma_{\text{right}} < \text{Re}(s) < \sigma_{\text{left}}\}.$$
This is nonempty only if $\sigma_{\text{right}} < \sigma_{\text{left}}$.

**Examples:**
- $f(t) = e^{-a|t|}$, $a > 0$: ROC is $\{-a < \text{Re}(s) < a\}$.
- $f(t) = e^{-at}\mathbf{1}_{[0,\infty)}(t)$: ROC is $\{\text{Re}(s) > -a\}$ (right half-plane).
- $f(t) = e^{at}\mathbf{1}_{(-\infty,0]}(t)$: ROC is $\{\text{Re}(s) < a\}$ (left half-plane).

## Analyticity

On its ROC, $\mathcal{B}[f](s)$ is an **analytic function** of $s$. This follows from differentiating under the integral sign:
$$\frac{d}{ds}\mathcal{B}[f](s) = \int_{-\infty}^\infty (-t)f(t)e^{-st}\,dt = \mathcal{B}[-tf](s).$$
The boundary of the ROC is determined by the singularities of $\mathcal{B}[f]$ as an analytic function.

## Worked Examples

**Example 1:** $f(t) = e^{-a|t|}$ for $a > 0$.
$$\mathcal{B}[f](s) = \int_{-\infty}^0 e^{at}e^{-st}\,dt + \int_0^\infty e^{-at}e^{-st}\,dt = \frac{1}{s-a}\bigg|_{t=0}^{-\infty}\text{-contribution} + \frac{1}{s+a}.$$
More carefully:
$$\int_{-\infty}^0 e^{(a-s)t}\,dt = \frac{1}{s-a} \quad (\text{Re}(s) < a), \qquad \int_0^\infty e^{-(a+s)t}\,dt = \frac{1}{a+s} \quad (\text{Re}(s) > -a).$$
So $\mathcal{B}[e^{-a|t|}](s) = \frac{1}{s-a} + \frac{1}{a+s}\big|_{\text{combined? No.}}$ Wait: $\int_{-\infty}^0 e^{at}e^{-st} = \int_{-\infty}^0 e^{(a-s)t}\,dt = [e^{(a-s)t}/(a-s)]_{-\infty}^0 = 1/(a-s)$ for $\text{Re}(s) < a$. And $\int_0^\infty e^{-at}e^{-st}\,dt = 1/(a+s)$ for $\text{Re}(s) > -a$. So:
$$\mathcal{B}[e^{-a|t|}](s) = \frac{1}{a-s} + \frac{1}{a+s} = \frac{2a}{a^2 - s^2}, \quad -a < \text{Re}(s) < a.$$
On the imaginary axis $s = i\omega$: $\mathcal{B}(i\omega) = 2a/(a^2 + \omega^2)$, which matches the angular-convention Fourier transform of $e^{-a|t|}$.

**Example 2:** Two functions with the same formula but different ROCs.

$F(s) = 1/(s+2)$ arises from:
- $f_1(t) = e^{-2t}\mathbf{1}_{[0,\infty)}(t)$: ROC $\text{Re}(s) > -2$.
- $f_2(t) = -e^{-2t}\mathbf{1}_{(-\infty,0)}(t)$: ROC $\text{Re}(s) < -2$.

**Conclusion:** The ROC is part of the definition. Different functions can have the same algebraic formula for $\mathcal{B}[f]$ but in different regions of $s$.

## Inversion: The Bromwich Integral

**Theorem.** If $F(s) = \mathcal{B}[f](s)$ converges in the strip $\sigma_1 < \text{Re}(s) < \sigma_2$, and if $F(\sigma + i\cdot) \in L^1(\mathbb{R})$ for $\sigma_1 < \sigma < \sigma_2$, then
$$f(t) = \frac{1}{2\pi i}\int_{\sigma - i\infty}^{\sigma + i\infty} F(s)\,e^{st}\,ds$$
for any fixed $\sigma$ in the strip of convergence.

This integral is along a vertical line in the complex plane. For rational $F$, it is evaluated by closing the contour and applying the residue theorem.

## Relationship to Z-Transform

The substitution $s = \ln z$ converts the bilateral Laplace transform into the (bilateral) Z-transform:
$$\mathcal{B}[f](s) = \sum_n f(nT)e^{-snT} = \sum_n f(nT)(e^{sT})^{-n} = \sum_n f(nT)z^{-n}\bigg|_{z = e^{sT}}.$$
This is exactly the Z-transform of the sampled sequence $f(nT)$, evaluated at $z = e^{sT}$. So the Z-transform is the discrete-time analog of the Laplace transform, just as the DFT is the discrete-time analog of the Fourier transform.
