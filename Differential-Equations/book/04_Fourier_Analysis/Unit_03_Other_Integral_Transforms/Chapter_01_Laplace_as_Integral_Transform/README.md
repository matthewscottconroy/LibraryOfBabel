# Chapter 01: Laplace as an Integral Transform

The Laplace transform was introduced earlier in this course primarily as a computational tool for solving linear ODEs with constant coefficients and initial conditions. Here we return to it from a broader perspective: as a member of the family of integral transforms, related to the Fourier transform by a complex change of variable, and with its own inversion theory that connects to complex analysis.

## Chapter Overview

**Section 01: Relation to the Fourier Transform** makes the connection between the Fourier transform and the Laplace transform explicit. The (one-sided) Laplace transform $\mathcal{L}[f](s) = \int_0^\infty f(t)e^{-st}\,dt$ is the Fourier transform of the causal function $f(t)\mathbf{1}_{[0,\infty)}(t)$ evaluated at $-i$ times the imaginary part of $s$, after exponential damping by the real part of $s$. More precisely, $\mathcal{L}[f](\sigma + i\omega) = \mathcal{F}[f(t)e^{-\sigma t}\mathbf{1}_{[0,\infty)}](\omega/(2\pi))$. This relationship explains why the Laplace transform is defined for functions that may grow (as long as they don't grow faster than some exponential), while the Fourier transform requires decay.

**Section 02: Bilateral Laplace** covers the two-sided Laplace transform $\mathcal{B}[f](s) = \int_{-\infty}^\infty f(t)e^{-st}\,dt$. Unlike the one-sided transform, the bilateral version is defined for functions on all of $\mathbb{R}$. Its inversion formula involves the Bromwich integral — an integral along a vertical contour $\sigma + i\omega$ in the complex plane, evaluated using the residue theorem from complex analysis. The one-sided Laplace transform is the special case where $f(t) = 0$ for $t < 0$.

## Key Theme

The Laplace transform lives in the complex $s$-plane. Its region of convergence — the set of complex $s$ for which the defining integral converges absolutely — is a vertical half-plane $\{\text{Re}(s) > \sigma_0\}$ or a vertical strip. The boundary of this region is related to the singularities of $\mathcal{L}[f]$ as a function of $s$. The Fourier transform is recovered by restricting to the imaginary axis $\text{Re}(s) = 0$, when convergence allows.
