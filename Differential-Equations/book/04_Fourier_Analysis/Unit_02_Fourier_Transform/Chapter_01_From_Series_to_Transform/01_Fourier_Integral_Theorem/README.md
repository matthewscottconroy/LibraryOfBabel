# The Fourier Integral Theorem

The Fourier series represents a periodic function as a discrete sum of harmonics. But most functions of physical interest — the temperature distribution in an infinite rod, the pressure wave from an explosion, a radar pulse — are not periodic. They exist on all of $\mathbb{R}$ without repeating. What becomes of Fourier analysis for such functions?

The answer is the Fourier integral theorem, which asserts that a well-behaved non-periodic function can be represented as a continuous superposition (integral) of complex exponentials over all real frequencies.

## Heuristic Derivation from Fourier Series

Let $f : \mathbb{R} \to \mathbb{C}$ be a function that is concentrated on a bounded region (decays rapidly as $|x| \to \infty$). For any large $L > 0$, define the $2L$-periodic function $f_L$ that equals $f$ on $(-L, L)$ and is extended periodically. The complex Fourier series of $f_L$ is
$$f_L(x) = \sum_{n=-\infty}^\infty c_n^{(L)} e^{in\pi x/L},$$
where
$$c_n^{(L)} = \frac{1}{2L}\int_{-L}^L f_L(x)e^{-in\pi x/L}\,dx \approx \frac{1}{2L}\int_{-\infty}^\infty f(x)e^{-in\pi x/L}\,dx$$
(the approximation improving as $L \to \infty$, since $f$ is small outside $(-L, L)$).

Define the frequency spacing $\Delta\xi = 1/(2L)$ and the sample frequencies $\xi_n = n/(2L) = n\Delta\xi$. Then
$$c_n^{(L)} = \Delta\xi\int_{-\infty}^\infty f(x)e^{-2\pi i\xi_n x}\,dx \approx \Delta\xi\cdot\hat{f}(\xi_n),$$
where we define
$$\hat{f}(\xi) = \int_{-\infty}^\infty f(x)e^{-2\pi i\xi x}\,dx.$$

Substituting back into the Fourier series:
$$f_L(x) = \sum_{n=-\infty}^\infty c_n^{(L)} e^{in\pi x/L} = \sum_{n=-\infty}^\infty \hat{f}(\xi_n) e^{2\pi i\xi_n x}\Delta\xi.$$

This is a Riemann sum for the integral $\int_{-\infty}^\infty \hat{f}(\xi) e^{2\pi i\xi x}\,d\xi$ over $\xi$. As $L \to \infty$, $\Delta\xi \to 0$ and the Riemann sum approaches the integral. Passing to the limit:
$$f(x) = \lim_{L\to\infty} f_L(x) = \int_{-\infty}^\infty \hat{f}(\xi)\,e^{2\pi i\xi x}\,d\xi.$$

## Statement of the Theorem

**Theorem (Fourier Integral Theorem).** Let $f \in L^1(\mathbb{R})$ and suppose $f$ is piecewise smooth (continuous except at isolated points, with one-sided limits and derivatives everywhere). Define
$$\hat{f}(\xi) = \int_{-\infty}^\infty f(x)\,e^{-2\pi i\xi x}\,dx.$$
Then $\hat{f} \in L^\infty(\mathbb{R})$ (bounded) and for every $x$:
$$\int_{-\infty}^\infty \hat{f}(\xi)\,e^{2\pi i\xi x}\,d\xi = \frac{f(x^+) + f(x^-)}{2}.$$
In particular, at points of continuity, the formula recovers $f(x)$ exactly.

**Remarks:**
1. The condition $f \in L^1(\mathbb{R})$ ensures $\hat{f}$ is well-defined (the integral defining $\hat{f}(\xi)$ converges absolutely for all $\xi$).
2. The piecewise smoothness is used to prove the inversion formula; without it, inversion requires more sophisticated arguments.
3. The inversion integral may need to be interpreted as a principal value: $\lim_{R\to\infty}\int_{-R}^R \hat{f}(\xi)e^{2\pi i\xi x}\,d\xi$.

## Proof Sketch

The proof mirrors the Dirichlet kernel proof for Fourier series. Define the **Fourier integral kernel**
$$K_R(u) = \int_{-R}^R e^{2\pi i\xi u}\,d\xi = \frac{\sin(2\pi Ru)}{\pi u}.$$
This is the analog of the Dirichlet kernel. The truncated inversion integral is
$$I_R(x) = \int_{-R}^R \hat{f}(\xi)e^{2\pi i\xi x}\,d\xi = \int_{-\infty}^\infty f(t)\,K_R(x-t)\,dt.$$
As $R \to \infty$, $K_R(u) \to \delta(u)$ (the Dirac delta) in the distributional sense, but more concretely, $K_R$ is an approximate identity: it is concentrated near $0$, has integral $1$, and the Riemann-Lebesgue lemma ensures contributions away from $0$ vanish. The piecewise smoothness of $f$ lets us control the local behavior and prove $I_R(x) \to [f(x^+) + f(x^-)]/2$.

## The Riemann-Lebesgue Lemma for the Fourier Transform

**Lemma.** If $f \in L^1(\mathbb{R})$, then $\hat{f}(\xi) \to 0$ as $|\xi| \to \infty$.

This is the continuous analog of the same result for Fourier series. It says that the Fourier transform of an integrable function vanishes at high frequencies — rapidly oscillating exponentials cancel out when integrated against $f$.

The proof is immediate for functions in the Schwartz class (smooth, rapidly decaying), and extends to $L^1$ by approximation. The Riemann-Lebesgue lemma is used in the proof of the Fourier integral theorem and has important physical consequences: bandlimited functions (those whose Fourier transform has compact support) have rapidly decaying transforms, meaning sharp features in frequency space correspond to slow decay in physical space, and vice versa.

## Convention Discussion

Several different normalization conventions appear in the literature. The three most common are:

| Convention | $\hat{f}(\xi)$ | Inversion |
|---|---|---|
| Frequency ($\nu$) | $\int f(x)e^{-2\pi i\nu x}\,dx$ | $\int\hat{f}(\nu)e^{2\pi i\nu x}\,d\nu$ |
| Angular ($\omega$), symmetric | $\frac{1}{\sqrt{2\pi}}\int f(x)e^{-i\omega x}\,dx$ | $\frac{1}{\sqrt{2\pi}}\int\hat{f}(\omega)e^{i\omega x}\,d\omega$ |
| Angular ($\omega$), asymmetric | $\int f(x)e^{-i\omega x}\,dx$ | $\frac{1}{2\pi}\int\hat{f}(\omega)e^{i\omega x}\,d\omega$ |

This course primarily uses the frequency convention (first row), which places no factors of $2\pi$ in the exponents and makes the transform and its inverse structurally identical. The angular asymmetric convention (third row) is common in physics and engineering. All conventions produce equivalent results; one simply adjusts the constants in identities accordingly.

## Worked Examples

**Example 1: Rectangular pulse.** Let $f(x) = \mathbf{1}_{[-1/2,1/2]}(x)$ (the indicator of $[-1/2,1/2]$). Then
$$\hat{f}(\xi) = \int_{-1/2}^{1/2} e^{-2\pi i\xi x}\,dx = \left[\frac{e^{-2\pi i\xi x}}{-2\pi i\xi}\right]_{-1/2}^{1/2} = \frac{e^{-\pi i\xi} - e^{\pi i\xi}}{-2\pi i\xi} = \frac{\sin(\pi\xi)}{\pi\xi} = \text{sinc}(\xi).$$
The inversion formula gives $f(x) = \int_{-\infty}^\infty \text{sinc}(\xi)e^{2\pi i\xi x}\,d\xi$.

**Example 2: Decaying exponential.** Let $f(x) = e^{-a|x|}$ for $a > 0$. Then
$$\hat{f}(\xi) = \int_{-\infty}^\infty e^{-a|x|}e^{-2\pi i\xi x}\,dx = \int_0^\infty e^{-ax}e^{-2\pi i\xi x}\,dx + \int_{-\infty}^0 e^{ax}e^{-2\pi i\xi x}\,dx$$
$$= \frac{1}{a + 2\pi i\xi} + \frac{1}{a - 2\pi i\xi} = \frac{2a}{a^2 + 4\pi^2\xi^2}.$$
This is a **Lorentzian**, the continuous analog of a geometric series. As $a \to 0$, the function $e^{-a|x|}$ approaches $1$ and the Lorentzian $\to \delta(\xi)/1$ — but this limit is only valid in the distributional sense, treated in Chapter 03.
