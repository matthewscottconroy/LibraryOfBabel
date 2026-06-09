# Properties and Zeros of Bessel Functions

The qualitative behavior of Bessel functions — their oscillation, their zeros, their interlacing structure, and their asymptotic decay — is as important for applications as the exact series definition. In particular, the zeros $j_{\nu,n}$ of $J_\nu$ are the eigenvalues of the radial Sturm-Liouville problem on a disk, and knowing their approximate values, their spacing, and their growth rate is essential for computing modes of vibration, heat decay rates, and waveguide cutoff frequencies. This section develops the theory of Bessel function properties systematically.

## Qualitative Behavior

A Bessel function $J_\nu(x)$ for $\nu \geq 0$ and $x > 0$ looks like a damped cosine. More precisely:

- **Near the origin:** $J_\nu(x) \sim (x/2)^\nu/\Gamma(\nu+1) \to 0$ for $\nu > 0$; $J_0(0) = 1$.
- **For large $x$:** $J_\nu(x) \sim \sqrt{2/(\pi x)}\cos(x - \nu\pi/2 - \pi/4)$.

The function oscillates with period approaching $2\pi$ and amplitude decaying as $1/\sqrt{x}$. The phase shift $-\nu\pi/2 - \pi/4$ means $J_0$ has its first zero near $2.405$, not at $\pi \approx 3.14$ as cosine would. The $1/\sqrt{x}$ decay reflects the spreading of cylindrical waves: energy in a cylindrical wave front is conserved, but the wave front grows like $2\pi r$, so amplitude decays like $1/\sqrt{r}$.

**Comparison with trigonometric functions.** Bessel's equation $y'' + \frac{1}{x}y' + (1-\nu^2/x^2)y = 0$ can be converted to a Schrödinger-type equation via $y = u/\sqrt{x}$: the function $u(x) = \sqrt{x}\,J_\nu(x)$ satisfies:

$$u'' + \left(1 - \frac{\nu^2 - 1/4}{x^2}\right)u = 0.$$

For large $x$, the term $(\nu^2-1/4)/x^2 \to 0$, and the equation approaches $u'' + u = 0$, confirming the asymptotic cosine behavior. For $\nu = 1/2$ exactly, $(\nu^2-1/4)/x^2 = 0$ and the equation is exactly $u'' + u = 0$:

$$J_{1/2}(x) = \sqrt{\frac{2}{\pi x}}\sin x, \qquad J_{-1/2}(x) = \sqrt{\frac{2}{\pi x}}\cos x.$$

These are **elementary** Bessel functions — a special feature of half-integer order.

## The Zeros of $J_\nu$

**Theorem.** For each $\nu \geq 0$, the function $J_\nu(x)$ has infinitely many positive zeros. Label them $0 < j_{\nu,1} < j_{\nu,2} < j_{\nu,3} < \cdots$. These zeros are all simple (i.e., $J_\nu'(j_{\nu,n}) \neq 0$).

*Proof sketch.* From the Schrödinger form, when $x > \nu + 1/2$, the effective potential is negative and the equation is oscillatory (Sturm comparison with $u'' + u = 0$). The oscillation theorem for Sturm-Liouville problems then guarantees infinitely many zeros.

**First few zeros** (tabulated values):

| $n$ | $j_{0,n}$ | $j_{1,n}$ | $j_{2,n}$ |
|---|---|---|---|
| 1 | 2.4048 | 3.8317 | 5.1356 |
| 2 | 5.5201 | 7.0156 | 8.4172 |
| 3 | 8.6537 | 10.1735 | 11.6198 |
| 4 | 11.7915 | 13.3237 | 14.7960 |

**Asymptotic formula.** The McMahon expansion for large $n$:

$$j_{\nu,n} \approx \left(n + \frac{\nu}{2} - \frac{1}{4}\right)\pi - \frac{4\nu^2-1}{8\pi(n+\nu/2-1/4)} - \cdots$$

The leading term $j_{\nu,n} \approx (n + \nu/2 - 1/4)\pi$ shows the zeros are approximately evenly spaced with gap $\approx \pi$, and the offset from integer multiples of $\pi$ is $(\nu/2-1/4)\pi$.

**Physical significance.** The $n$-th natural frequency of a circular membrane of radius $a$ with fixed boundary is $\omega_{mn} = c\, j_{m,n}/a$ (for azimuthal mode number $m$). The ratio of the $(m,n)$-th frequency to the fundamental $\omega_{01} = c\,j_{0,1}/a$ is $j_{m,n}/j_{0,1}$ — irrational for most $(m,n)$, making the spectrum inharmonic. This is why drums sound "unpitched" compared to strings.

## Interlacing of Zeros

**Theorem (interlacing).** The zeros of $J_\nu$ and $J_{\nu+1}$ interlace:

$$j_{\nu,n} < j_{\nu+1,n} < j_{\nu,n+1}.$$

Moreover, the zeros of $J_\nu$ and $J_\nu'$ interlace:

$$0 < j_{\nu,1}' < j_{\nu,1} < j_{\nu,2}' < j_{\nu,2} < \cdots$$

(where $j_{\nu,n}'$ denotes the $n$-th positive zero of $J_\nu'$).

*Proof.* The Wronskian identity $J_\nu J_{\nu+1}' - J_\nu' J_{\nu+1} = -2/(\pi x)$ is nonzero, so $J_\nu$ and $J_{\nu+1}$ cannot have common zeros. The Sturm comparison theorem (comparing Bessel equations for $\nu$ and $\nu+1$) gives the interlacing.

**Corollary.** Between consecutive zeros of $J_\nu$ lies exactly one zero of $J_{\nu+1}$.

## Recursion and Differentiation Formulas

The recursion relations derived in Section 1 yield useful differentiation formulas:

$$J_\nu'(x) = \frac{1}{2}[J_{\nu-1}(x) - J_{\nu+1}(x)], \qquad J_0'(x) = -J_1(x).$$

**Differentiation of $J_0$:** Since $J_0' = -J_1$, the zeros of $J_1$ (which are the zeros of $J_0'$) lie between consecutive zeros of $J_0$, as the interlacing theorem guarantees.

**Integration by parts:** The formula $\int_0^a J_0(\alpha r)r\,dr = (a/\alpha)J_1(\alpha a)$ is obtained by integrating $(d/dr)[rJ_1(\alpha r)] = \alpha r J_0(\alpha r)$. More generally:

$$\int_0^a J_\nu(\alpha r)r^{\nu+1}\,dr = \frac{a^{\nu+1}}{\alpha}J_{\nu+1}(\alpha a).$$

This integral is the key ingredient in computing Bessel-Fourier coefficients.

## Neumann Functions and Their Zeros

The Neumann function $Y_\nu(x)$ (second kind) has zeros $y_{\nu,n}$ that interlace with those of $J_\nu$:

$$j_{\nu,n} < y_{\nu,n} < j_{\nu,n+1}.$$

For an annular region $R_1 < r < R_2$, the radial eigenfunctions are linear combinations $\alpha J_\nu(\mu r) + \beta Y_\nu(\mu r)$ where the constants $\alpha, \beta$ are determined by the boundary conditions at $r = R_1$ and $r = R_2$. The eigenvalues $\mu$ are zeros of a $2\times 2$ determinant involving $J_\nu$ and $Y_\nu$ at $R_1$ and $R_2$, computed numerically.

## Bessel Functions of Large Order

For fixed $x$ and large $\nu$:

$$J_\nu(x) \sim \frac{1}{\sqrt{2\pi\nu}}\left(\frac{ex}{2\nu}\right)^\nu \quad \text{as } \nu \to \infty.$$

This exponential decay in $\nu$ means that for a fixed point $r < 1$ on the unit disk, the higher azimuthal modes ($m$ large) contribute negligibly — the Bessel-Fourier series converges rapidly in the azimuthal direction.

The transition region $\nu \approx x$ has a more complex description in terms of Airy functions: $J_\nu(\nu + t\nu^{1/3}) \approx 2^{1/3}\nu^{-1/3}\text{Ai}(-2^{1/3}t)$ where $\text{Ai}$ is the Airy function. This is the boundary between the "oscillatory" regime ($x > \nu$) and the "exponential" regime ($x < \nu$).

## Integral Representations

The Poisson integral formula gives $J_\nu$ for $\nu > -1/2$:

$$J_\nu(x) = \frac{(x/2)^\nu}{\sqrt{\pi}\,\Gamma(\nu+1/2)}\int_0^\pi\cos(x\cos\theta)\sin^{2\nu}\theta\,d\theta.$$

For $\nu = 0$: $J_0(x) = \frac{1}{\pi}\int_0^\pi\cos(x\cos\theta)\,d\theta$ (Bessel's original representation). For $\nu = 1/2$: the weight $\sin\theta$ gives $J_{1/2}(x) = \sqrt{2/\pi x}\sin x$ as expected.

This integral representation has a probabilistic interpretation: if $\mathbf{U}$ is a unit vector uniformly distributed on the unit circle in $\mathbb{R}^{2\nu+2}$ (after appropriate normalization), then $J_\nu(x) \propto \mathbb{E}[e^{ix\mathbf{U}\cdot\mathbf{e}_1}]$ — $J_\nu$ is the characteristic function of the projection of a uniform distribution on a sphere in $\mathbb{R}^{2\nu+2}$. This connects Bessel functions to probability theory and random walks.

## Application: Diffraction Pattern of a Circular Aperture

A plane wave $e^{ikz}$ incident on a circular aperture of radius $a$ in an opaque screen produces a far-field diffraction pattern proportional to:

$$A(\theta) = 2\pi a^2 \frac{J_1(ka\sin\theta)}{ka\sin\theta},$$

where $\theta$ is the diffraction angle. This is the **Airy pattern**. The first dark ring occurs at $ka\sin\theta = j_{1,1} \approx 3.832$, giving $\sin\theta \approx 1.22\lambda/(2a)$ — the Rayleigh criterion for angular resolution of a circular aperture. Telescopes, camera lenses, and the human eye are all limited by this formula, which is derived directly from the first zero of $J_1$.
