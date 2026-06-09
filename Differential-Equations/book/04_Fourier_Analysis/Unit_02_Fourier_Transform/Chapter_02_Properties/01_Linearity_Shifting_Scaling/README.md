# Linearity, Shifting, and Scaling

The most immediately useful properties of the Fourier transform are its behavior under the most basic operations on functions: forming linear combinations, translating by a constant, and rescaling the argument. Each property has a clean mathematical statement and a direct physical interpretation.

## Linearity

**Theorem.** For $f, g \in L^1(\mathbb{R})$ and constants $\alpha, \beta \in \mathbb{C}$:
$$\mathcal{F}[\alpha f + \beta g] = \alpha\mathcal{F}[f] + \beta\mathcal{F}[g].$$

**Proof.** Immediate from the linearity of the integral:
$$\mathcal{F}[\alpha f + \beta g](\xi) = \int_{-\infty}^\infty (\alpha f(x) + \beta g(x))e^{-2\pi i\xi x}\,dx = \alpha\hat{f}(\xi) + \beta\hat{g}(\xi).$$

This property, while obvious, is foundational: it means the Fourier transform is a linear operator, and the entire theory of linear operators applies. The transform of a sum is the sum of transforms, so we can decompose a complicated function into simpler pieces, transform each piece separately, and add the results.

## Time-Shifting (Translation in the Time Domain)

**Theorem.** For $f \in L^1(\mathbb{R})$ and $a \in \mathbb{R}$, let $f_a(x) = f(x-a)$. Then
$$\hat{f}_a(\xi) = e^{-2\pi i a\xi}\hat{f}(\xi).$$
Equivalently: $\mathcal{F}[f(\cdot - a)](\xi) = e^{-2\pi ia\xi}\hat{f}(\xi)$.

**Proof.** Substitute $u = x - a$:
$$\hat{f}_a(\xi) = \int_{-\infty}^\infty f(x-a)e^{-2\pi i\xi x}\,dx = \int_{-\infty}^\infty f(u)e^{-2\pi i\xi(u+a)}\,du = e^{-2\pi ia\xi}\int_{-\infty}^\infty f(u)e^{-2\pi i\xi u}\,du = e^{-2\pi ia\xi}\hat{f}(\xi).$$

**Interpretation.** Shifting a function in time (by $a$ units) multiplies its Fourier transform by $e^{-2\pi ia\xi}$. The magnitude $|\hat{f}_a(\xi)| = |\hat{f}(\xi)|$ is unchanged: the amplitude spectrum is translation-invariant. Only the phase changes: the phase of $\hat{f}_a$ at frequency $\xi$ is the phase of $\hat{f}$ at $\xi$ plus $-2\pi a\xi$, a linear function of frequency.

**Example.** The rectangular pulse centered at $a$, $f(x) = \mathbf{1}_{[a-1/2, a+1/2]}(x) = \mathbf{1}_{[-1/2,1/2]}(x - a)$, has Fourier transform $e^{-2\pi ia\xi}\text{sinc}(\xi)$.

## Frequency-Shifting (Modulation)

**Theorem.** For $f \in L^1(\mathbb{R})$ and $\nu_0 \in \mathbb{R}$:
$$\mathcal{F}[e^{2\pi i\nu_0 x}f(x)](\xi) = \hat{f}(\xi - \nu_0).$$

**Proof.** Direct computation:
$$\int_{-\infty}^\infty e^{2\pi i\nu_0 x}f(x)e^{-2\pi i\xi x}\,dx = \int_{-\infty}^\infty f(x)e^{-2\pi i(\xi-\nu_0)x}\,dx = \hat{f}(\xi - \nu_0).$$

**Interpretation.** Multiplying a function by a complex exponential $e^{2\pi i\nu_0 x}$ (modulation at carrier frequency $\nu_0$) shifts its spectrum by $\nu_0$. This is the mathematical basis of amplitude modulation (AM) in radio: a baseband signal $f(x)$ is transmitted at carrier frequency $\nu_0$ by forming $\cos(2\pi\nu_0 x)\cdot f(x)$, which shifts the spectrum to be centered at $\pm\nu_0$.

For real modulation, $\cos(2\pi\nu_0 x) = (e^{2\pi i\nu_0 x} + e^{-2\pi i\nu_0 x})/2$, giving:
$$\mathcal{F}[\cos(2\pi\nu_0 x)f(x)](\xi) = \frac{1}{2}[\hat{f}(\xi - \nu_0) + \hat{f}(\xi + \nu_0)].$$

## Scaling

**Theorem.** For $f \in L^1(\mathbb{R})$ and $a \neq 0$:
$$\mathcal{F}[f(ax)](\xi) = \frac{1}{|a|}\hat{f}\!\left(\frac{\xi}{a}\right).$$

**Proof.** If $a > 0$, substitute $u = ax$ (so $du = a\,dx$):
$$\int_{-\infty}^\infty f(ax)e^{-2\pi i\xi x}\,dx = \int_{-\infty}^\infty f(u)e^{-2\pi i\xi u/a}\frac{du}{a} = \frac{1}{a}\hat{f}\!\left(\frac{\xi}{a}\right).$$
If $a < 0$, the substitution reverses limits and introduces a sign, giving the $|a|$ in the denominator.

**Interpretation: Time-Frequency Uncertainty.** The scaling property quantifies the uncertainty principle. If $a > 1$, then $f(ax)$ is a compressed version of $f$ (faster oscillations, narrower support). Its transform $\frac{1}{a}\hat{f}(\xi/a)$ is dilated by $a$ in $\xi$ and reduced in amplitude by $1/a$ — the spectrum spreads out. Conversely, compressing in time spreads in frequency. You cannot simultaneously have a narrow time-domain function and a narrow frequency-domain function.

**Example.** The Gaussian $e^{-\pi\alpha x^2}$ for $\alpha > 0$: applying the scaling property with $f(x) = e^{-\pi x^2}$ (so $f(ax) = e^{-\pi a^2 x^2} = e^{-\pi\alpha x^2}$ with $a = \sqrt{\alpha}$):
$$\mathcal{F}[e^{-\pi\alpha x^2}](\xi) = \frac{1}{\sqrt{\alpha}}\hat{f}\!\left(\frac{\xi}{\sqrt{\alpha}}\right) = \frac{1}{\sqrt{\alpha}}e^{-\pi\xi^2/\alpha}.$$
As $\alpha \to \infty$, the time-domain Gaussian narrows and the frequency-domain Gaussian broadens, consistent with the uncertainty principle.

## Duality

A remarkable feature of the Fourier transform (with the symmetric normalization $1/\sqrt{2\pi}$ in both transform and inverse) is the **duality symmetry**: the transform of the transform returns the original function (possibly reflected). With our normalization $\hat{f}(\xi) = \int f(x)e^{-2\pi i\xi x}\,dx$:
$$\mathcal{F}[\hat{f}](x) = f(-x).$$

**Proof.** By the inversion formula, $\hat{f}(-x) = \int \hat{f}(\xi)e^{-2\pi i\xi x}\,d\xi = \mathcal{F}[\hat{f}](-x)$... more cleanly: the inversion formula says $\int \hat{f}(\xi)e^{2\pi i\xi x}\,d\xi = f(x)$, i.e., $\mathcal{F}^{-1}[\hat{f}] = f$. But $\mathcal{F}^{-1}[g](x) = \mathcal{F}[g](-x)$ (the inverse transform is the forward transform evaluated at $-x$). So $\mathcal{F}[\hat{f}](-x) = f(x)$, i.e., $\mathcal{F}[\hat{f}](x) = f(-x)$.

**Application.** Duality allows us to derive new transform pairs from known ones. For instance, since $\mathcal{F}[e^{-a|x|}](\xi) = \frac{2a}{a^2 + 4\pi^2\xi^2}$, applying duality gives $\mathcal{F}\!\left[\frac{2a}{a^2 + 4\pi^2 x^2}\right](\xi) = e^{-a|\xi|}$.

## Combined Application

**Problem.** Find $\mathcal{F}[g]$ where $g(x) = e^{-3|x-2|}$.

**Solution.** Write $g(x) = f(x-2)$ where $f(x) = e^{-3|x|}$. We know $\hat{f}(\xi) = \frac{6}{9 + 4\pi^2\xi^2}$ (from the decaying exponential formula with $a = 3$). By the time-shifting property:
$$\hat{g}(\xi) = e^{-4\pi i\xi}\hat{f}(\xi) = \frac{6e^{-4\pi i\xi}}{9 + 4\pi^2\xi^2}.$$

The amplitude spectrum $|\hat{g}(\xi)| = \frac{6}{9 + 4\pi^2\xi^2}$ is the same as that of $f$ (translation doesn't change the spectral content), but the phase is rotated by $-4\pi\xi$ at each frequency.
