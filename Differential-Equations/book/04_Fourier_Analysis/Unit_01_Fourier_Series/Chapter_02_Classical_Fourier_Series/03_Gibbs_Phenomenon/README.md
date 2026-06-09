# The Gibbs Phenomenon

Dirichlet's theorem tells us that the partial sums $S_N f$ converge to $f$ at every point of continuity and to the midpoint of jumps. But convergence at individual points does not tell the whole story of how the partial sums behave near a discontinuity. If you plot $S_N f$ near a jump and increase $N$, a curious phenomenon is visible: the partial sums overshoot the function value near the jump, and this overshoot does not diminish as $N \to \infty$. It merely narrows, concentrating into a narrower and narrower spike, while maintaining essentially constant height.

## The Overshoot Does Not Vanish

Consider the square wave $f(x) = \text{sgn}(x)$ on $(-\pi, \pi)$, with a jump from $-1$ to $+1$ at $x = 0$. The Fourier series is
$$S_N f(x) = \frac{4}{\pi}\sum_{k=0}^{N'} \frac{\sin((2k+1)x)}{2k+1},$$
where the sum runs over odd integers up to $N$ (we write $N'$ for the largest odd integer $\leq N$).

The partial sum $S_N f(x)$ has a local maximum somewhere near $x = 0$ (but to the right of it). To find where, differentiate and set to zero:
$$\frac{d}{dx}S_N f(x) = \frac{4}{\pi}\sum_{k=0}^{N'} \cos((2k+1)x) = \frac{4}{\pi} \cdot \frac{\sin((N+1)x)}{2\sin(x/2)} \cdot \frac{1}{2} \cdot (\text{a related expression}).$$
More cleanly, using the full Fourier series through $N$ terms for all frequencies (not just odd ones, to simplify the analysis), the maximum of $S_N$ occurs near $x = \pi/N$. At this point,
$$S_N f(\pi/N) \approx \frac{2}{\pi}\int_0^\pi \frac{\sin u}{u}\,du.$$
This integral is the **sine integral** $\text{Si}(\pi)$. Its value is approximately $1.8519$, which is about $9.0\%$ above $1$. Scaling back: the overshoot above the value $+1$ approaches $\frac{2}{\pi}\text{Si}(\pi) - 1 \approx 0.0895$, or about $8.95\%$ of the total jump height of $2$.

## The Gibbs Constant

The precise statement is:

**Theorem (Gibbs Phenomenon).** Let $f$ be piecewise smooth with a jump discontinuity of height $J = f(x_0^+) - f(x_0^-)$ at $x_0$. Then the partial sums $S_N f$ overshoot the value $f(x_0^+)$ by approximately
$$\delta = \frac{J}{2}\left(\frac{2}{\pi}\int_0^\pi \frac{\sin u}{u}\,du - 1\right) \approx 0.0895 \cdot J,$$
and undershoot $f(x_0^-)$ by the same amount, as $N \to \infty$. The width of the overshoot region is $O(1/N)$.

The quantity $g = \frac{1}{\pi}\int_0^\pi \frac{\sin u}{u}\,du = \frac{\text{Si}(\pi)}{\pi} \approx 0.5895$ is sometimes called the **Gibbs constant**. The overshoot ratio is $2g - 1 \approx 0.179$ (relative to the full jump), or about $0.0895$ times the jump above $f(x_0^+)$.

## Derivation Sketch

The key computation involves a change of variables. For the square wave, the $N$-th partial sum evaluated near the jump is
$$S_N f(\pi/N) = \frac{4}{\pi}\sum_{k=0}^{N'} \frac{\sin((2k+1)\pi/N)}{2k+1}.$$
This is a Riemann sum for $\frac{2}{\pi}\int_0^\pi \frac{\sin t}{t}\,dt$ as $N \to \infty$ (substitute $t = (2k+1)\pi/N$, so $\Delta t \approx 2\pi/N$). Hence the maximum value approaches $\frac{2}{\pi}\text{Si}(\pi)$ regardless of $N$.

The location of the maximum shifts toward $x = 0$ as $N \to \infty$ (at rate $1/N$), so the overshoot becomes concentrated in an increasingly small region near the jump, but its height remains constant.

## Why the Gibbs Phenomenon Cannot Be Eliminated by Partial Sums

The Fourier coefficients $b_n \sim 4/(n\pi)$ of the square wave decay only as $1/n$, so the Fourier series has significant energy at all frequencies. Truncating at $N$ terms introduces a sharp cutoff in frequency space: we use all frequencies up to $N$ and none above. A sharp cutoff in frequency space corresponds (via the convolution theorem) to convolution with the Dirichlet kernel in physical space. The Dirichlet kernel itself has oscillating lobes, and it is these lobes that produce the overshoot.

This is an intrinsic limitation: no choice of $N$ eliminates the overshoot, because the Dirichlet kernel always has sidelobes.

## Remedies: Summability Methods

There are several ways to mitigate the Gibbs phenomenon, all involving replacing the sharp truncation with a smoother one:

**Cesaro summation** replaces the partial sums $S_N f$ with their arithmetic means:
$$\sigma_N f(x) = \frac{1}{N+1}\sum_{k=0}^N S_k f(x).$$
The Cesaro sum uses the **Fejér kernel** $F_N(u) = \frac{1}{N}\left(\frac{\sin(Nu/2)}{\sin(u/2)}\right)^2$ instead of the Dirichlet kernel. The Fejér kernel is non-negative, which eliminates the sidelobes and therefore eliminates the Gibbs overshoot. However, the approximation is less sharp at the jump (wider transition region).

**Lanczos $\sigma$-factor**: multiply the $n$-th Fourier coefficient by the factor $\sigma(n/N) = \text{sinc}(n/N) = \sin(n\pi/N)/(n\pi/N)$. This smoothly tapers the coefficients to zero, reducing oscillations near discontinuities.

**Jackson's theorem** quantifies how well the Cesaro means approximate a continuous function: if $f$ is continuous and $2\pi$-periodic, the maximum error of $\sigma_N f$ is bounded by a multiple of the modulus of continuity of $f$ at scale $1/N$.

## Practical Significance

The Gibbs phenomenon is not merely a mathematical curiosity. It is of direct engineering importance in digital signal processing: any bandlimited filter (one that sets all frequencies above some cutoff to zero) will produce overshoots near discontinuities. This affects image compression (JPEG artifacts near sharp edges), audio processing, and numerical methods. The standard mitigations — windowing functions like the Hann, Hamming, or Blackman windows — are precisely implementations of smooth tapering of the Fourier coefficients, analogous to the Lanczos $\sigma$-factor.

In numerical PDE, Gibbs-like oscillations appear in spectral methods applied to problems with discontinuous data, and special treatment (such as filtering or the ENO/WENO schemes) is required to handle them.
