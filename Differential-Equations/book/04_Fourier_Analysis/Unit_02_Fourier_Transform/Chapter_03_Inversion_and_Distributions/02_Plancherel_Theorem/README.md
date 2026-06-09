# Plancherel's Theorem

Plancherel's theorem is the $L^2$ analog of Parseval's identity for Fourier series. It asserts that the Fourier transform is an isometry on $L^2(\mathbb{R})$: it preserves the $L^2$ norm and the inner product. In geometric language, the Fourier transform is a unitary operator on the Hilbert space $L^2(\mathbb{R})$, acting like a rotation in an infinite-dimensional function space.

## Statement

**Theorem (Plancherel, 1910).** The Fourier transform $\mathcal{F}$ extends from $L^1(\mathbb{R}) \cap L^2(\mathbb{R})$ to a unitary operator on $L^2(\mathbb{R})$. That is, there exists a unique bounded linear operator $\mathcal{F} : L^2(\mathbb{R}) \to L^2(\mathbb{R})$ satisfying:
1. For $f \in L^1 \cap L^2$, $\mathcal{F}[f](\xi) = \int_{-\infty}^\infty f(x)e^{-2\pi i\xi x}\,dx$.
2. **Isometry:** $\|\mathcal{F}[f]\|_2 = \|f\|_2$ for all $f \in L^2$.
3. **Surjectivity:** $\mathcal{F}$ maps $L^2(\mathbb{R})$ onto $L^2(\mathbb{R})$.
4. **Inverse:** $\mathcal{F}^{-1} = \mathcal{F}^*$ (the adjoint, which is the transform with the conjugated/negated exponent).

The isometry property $\|\hat{f}\|_2 = \|f\|_2$ is equivalent to the **Parseval identity for the Fourier transform**:
$$\int_{-\infty}^\infty |\hat{f}(\xi)|^2\,d\xi = \int_{-\infty}^\infty |f(x)|^2\,dx.$$

## Proof of the Isometry for $f \in L^1 \cap L^2$

For $f \in L^1 \cap L^2$, we compute directly:
$$\int_{-\infty}^\infty |\hat{f}(\xi)|^2\,d\xi = \int_{-\infty}^\infty \hat{f}(\xi)\overline{\hat{f}(\xi)}\,d\xi.$$
Note $\overline{\hat{f}(\xi)} = \int_{-\infty}^\infty \overline{f(x)}e^{2\pi i\xi x}\,dx = \widehat{\overline{f}^*}(\xi)$ where $\overline{f}^*(x) = \overline{f(-x)}$... A cleaner approach uses the convolution-and-inversion route:

Define $g = f * \tilde{f}$ where $\tilde{f}(x) = \overline{f(-x)}$. Then $\hat{g} = \hat{f}\cdot\overline{\hat{f}} = |\hat{f}|^2$. By the convolution theorem and inversion:
$$\int|\hat{f}|^2\,d\xi = \hat{g}(0)\cdot 1 = g(0) \text{ (by inversion at }x=0\text{)} = \int_{-\infty}^\infty f(t)\overline{f(t)}\,dt = \|f\|_2^2.$$
(Here inversion gives $g(0) = \int \hat{g}(\xi)e^{0}\,d\xi = \int |\hat{f}|^2\,d\xi$, and $g(0) = \int f(t)\overline{f(0-t)}\,dt$... this requires $g \in L^1$, which holds when $f \in L^1 \cap L^2$.) The argument is most cleanly executed in the Schwartz space and then extended by density.

## Extension to $L^2$

For $f \in L^2 \setminus L^1$, the integral $\int f(x)e^{-2\pi i\xi x}\,dx$ need not converge absolutely. The extension proceeds as follows:

1. Choose a sequence $f_n \in L^1 \cap L^2$ with $\|f_n - f\|_2 \to 0$ (e.g., $f_n = f \cdot\mathbf{1}_{[-n,n]}$).
2. The isometry gives $\|\hat{f}_m - \hat{f}_n\|_2 = \|f_m - f_n\|_2 \to 0$, so $\{\hat{f}_n\}$ is Cauchy in $L^2$.
3. Since $L^2$ is complete, $\hat{f}_n \to \hat{f}$ in $L^2$ for some $\hat{f} \in L^2$.
4. The limit $\hat{f}$ does not depend on the choice of approximating sequence (standard argument), so $\mathcal{F}[f] := \hat{f}$ is well-defined.

This extension is the unique bounded linear operator on $L^2$ that agrees with the integral formula on $L^1 \cap L^2$.

## Parseval's Identity (Inner Product Form)

For $f, g \in L^2(\mathbb{R})$:
$$\langle \hat{f}, \hat{g}\rangle = \langle f, g\rangle, \quad \text{i.e.,} \quad \int_{-\infty}^\infty \hat{f}(\xi)\overline{\hat{g}(\xi)}\,d\xi = \int_{-\infty}^\infty f(x)\overline{g(x)}\,dx.$$
This follows from the polarization identity $\langle f,g\rangle = \frac{1}{4}(\|f+g\|^2 - \|f-g\|^2 + i\|f+ig\|^2 - i\|f-ig\|^2)$ and the isometry.

## Physical Interpretation: Energy Conservation

In signal processing, $|f(t)|^2$ is the instantaneous power and $\|f\|_2^2 = \int|f|^2$ is the total energy of the signal. Plancherel's theorem says $\int|\hat{f}(\xi)|^2\,d\xi = \int|f(t)|^2\,dt$: the total energy computed in the time domain equals the total energy computed in the frequency domain. The Fourier transform is a lossless change of representation.

The integrand $|\hat{f}(\xi)|^2$ is the **power spectral density**: the energy per unit frequency at frequency $\xi$. Integrating over any frequency band $[\xi_1, \xi_2]$ gives the energy in that band.

## Unitarity

The isometry and surjectivity of $\mathcal{F}$ make it a **unitary operator** on $L^2(\mathbb{R})$. The adjoint $\mathcal{F}^* = \mathcal{F}^{-1}$, which is given by the inverse transform formula. For a unitary operator, $\mathcal{F}^*\mathcal{F} = \mathcal{F}\mathcal{F}^* = \text{Id}$.

The spectrum of a unitary operator lies on the unit circle. For $\mathcal{F}$, the eigenvalues are $\{1, -1, i, -i\}$, corresponding to functions invariant under the Fourier transform up to a root of unity. The eigenfunctions (Hermite functions $\psi_n$) form a complete orthonormal basis of $L^2(\mathbb{R})$, providing a concrete orthonormal basis for which $\mathcal{F}$ acts diagonally.

## Connection to Fourier Series

The Fourier series analog of Plancherel's theorem is the statement (from Unit 01, Chapter 02) that $\frac{1}{2\pi}\|f\|_2^2 = \sum_{n=-\infty}^\infty |c_n|^2$. Both are instances of the same abstract theorem: if $\{e_n\}$ is a complete orthonormal set in a Hilbert space $H$, then $\sum |\langle f, e_n\rangle|^2 = \|f\|^2$. In the Fourier series case, $e_n = e^{inx}/\sqrt{2\pi}$ and the sum is over $n \in \mathbb{Z}$. In the Fourier transform case, the "orthonormal set" is a continuum $\{e_\xi\}_{\xi \in \mathbb{R}}$ and the sum becomes an integral.
