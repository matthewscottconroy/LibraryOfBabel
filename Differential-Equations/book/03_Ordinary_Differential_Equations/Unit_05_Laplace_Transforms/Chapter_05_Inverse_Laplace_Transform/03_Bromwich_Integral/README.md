# The Bromwich Integral

The **Bromwich integral** (also called the Mellin-Bromwich inversion integral or Bromwich-Wagner integral) is the formal inversion formula for the Laplace transform:

$$f(t) = \mathcal{L}^{-1}\{F(s)\}(t) = \frac{1}{2\pi i}\int_{\gamma-i\infty}^{\gamma+i\infty}e^{st}F(s)\,ds, \qquad t > 0,$$

where $\gamma > c$ is any real number to the right of all singularities of $F(s)$ in the complex $s$-plane. The integral is taken along the vertical line $\text{Re}(s) = \gamma$.

## Theoretical Basis

The Bromwich integral is a consequence of the theory of the Fourier transform combined with the half-line restriction $t \geq 0$. The Laplace transform $F(s) = \int_0^\infty e^{-st}f(t)\,dt$ can be viewed as the Fourier transform of $e^{-\gamma t}f(t)$ (a one-sided signal multiplied by a decaying exponential), evaluated at $\omega = \text{Im}(s)$ with $\text{Re}(s) = \gamma$. Inverting the Fourier transform gives the Bromwich integral.

**Theorem.** If $f$ is piecewise smooth, of exponential order $c$, and $\gamma > c$, then for $t > 0$:

$$f(t) = \frac{1}{2\pi i}\lim_{T\to\infty}\int_{\gamma-iT}^{\gamma+iT}e^{st}F(s)\,ds.$$

At jump discontinuities of $f$, the integral converges to the average $\frac{1}{2}(f(t^+) + f(t^-))$.

## Evaluation by Residues

For rational or meromorphic $F(s)$ (with poles in the half-plane $\text{Re}(s) < \gamma$), close the Bromwich contour with a large semicircle $C_R$ in the left half-plane $\text{Re}(s) \leq \gamma$. By Jordan's lemma, the integral over $C_R \to 0$ as $R \to \infty$ for $t > 0$ (since $|e^{st}| = e^{\text{Re}(s)\cdot t} \to 0$ on the left semicircle). By the residue theorem:

$$f(t) = \sum_{\text{poles}}\text{Res}[e^{st}F(s)].$$

**Example.** $F(s) = 1/(s^2 + \omega^2)$: poles at $s = \pm i\omega$. Residues:

$$\text{Res}_{s=i\omega}\frac{e^{st}}{s^2+\omega^2} = \frac{e^{i\omega t}}{2i\omega}, \qquad \text{Res}_{s=-i\omega} = \frac{e^{-i\omega t}}{-2i\omega}.$$

Sum: $\frac{e^{i\omega t} - e^{-i\omega t}}{2i\omega} = \frac{\sin\omega t}{\omega}$. Recovers the known inversion.

## Non-Rational Functions and Branch Cuts

When $F(s)$ has branch points (as for $\mathcal{L}\{t^{-1/2}\} = \sqrt{\pi/s}$ or $\mathcal{L}\{J_0(t)\} = 1/\sqrt{s^2+1}$), the Bromwich contour must be modified to avoid the branch cut. The contour is wrapped around the branch cut, and the inversion integral becomes an integral along the branch cut, which can often be evaluated explicitly.

## Practical Significance

The Bromwich integral is rarely used for hand calculations (partial fractions are more efficient). Its value is theoretical: it proves that the inverse transform is unique, provides a formula valid for non-rational transforms, and connects Laplace transform theory to complex analysis. It is the starting point for asymptotic analysis of solutions for large $t$ via saddle-point methods and steepest descent.
