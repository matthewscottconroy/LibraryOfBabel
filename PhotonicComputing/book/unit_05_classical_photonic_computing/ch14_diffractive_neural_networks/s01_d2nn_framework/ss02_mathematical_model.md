# Subsection 14.1.2: Mathematical Model

## Orientation

The physical picture of Subsection 14.1.1 becomes a computable model once we write the layer operation as a matrix. This subsection derives the Rayleigh–Sommerfeld layer update, shows it is a dense but *structured* linear map — a convolution followed by a diagonal modulation — gives the fast angular-spectrum (FFT) form used in every practical simulator, and counts the degrees of freedom against the space–bandwidth product.

---

## 14.1.2.1 The Rayleigh–Sommerfeld Layer Update

Modelling each neuron as a Huygens secondary source, the field at point $(x',y')$ of layer $l{+}1$ is the sum of modulated wavelets from every neuron of layer $l$:

$$U^{l+1}(x',y') = \sum_{x,y} t^l(x,y)\,U^l(x,y)\,h(x'-x,\,y'-y,\,d),$$

with the Rayleigh–Sommerfeld impulse response

$$h(x,y,z) = \frac{z}{r^2}\left(\frac{1}{2\pi r} + \frac{1}{i\lambda}\right)e^{ikr}, \qquad r = \sqrt{x^2+y^2+z^2}, \quad k = \frac{2\pi}{\lambda}.$$

The factor $z/r = \cos\theta$ is the obliquity factor; the two terms in parentheses are the near-field ($\propto 1/r$) and radiative ($\propto 1/\lambda$) contributions. For $r \gg \lambda$ the second term dominates and $h \to \frac{1}{i\lambda}\frac{e^{ikr}}{r}\cos\theta$, the familiar Rayleigh–Sommerfeld kernel of Goodman. Because $h$ depends only on the coordinate *differences* $(x'-x,\,y'-y)$, the sum is a **2D convolution**: propagation is shift-invariant.

---

## 14.1.2.2 The Layer as a Structured Linear Operator

Flatten each layer's field into a vector. The update is then

$$U^{l+1} = H_d\,\mathrm{diag}(t^l)\,U^l,$$

where $\mathrm{diag}(t^l)$ is the diagonal *modulation* matrix (the trainable part) and $H_d$ is the *propagation* matrix. Since propagation is a convolution, $H_d$ is (block-)Toeplitz: dense — every output couples to every input, the all-to-all wiring of Subsection 14.1.1 — but fixed by the single kernel $h(\cdot,\cdot,d)$, not by $N^2$ independent entries. A diffractive layer is therefore exactly a fully-connected linear layer with *structured* weights: convolution by a frozen propagation kernel, then a diagonal modulation the training is free to choose. Cascading,

$$U^{\text{out}} = H_d\,\mathrm{diag}(t^L)\,\cdots\,H_d\,\mathrm{diag}(t^1)\,H_d\,U^{\text{in}}$$

— a product of linear operators, hence itself a single linear operator $W_{\text{eff}}$. Nothing in the optical stack breaks this linearity; only the detector's $|\cdot|^2$ does (Subsections 14.1.1 and 13.1.1).

---

## 14.1.2.3 Fast Computation: The Angular-Spectrum Method

Evaluating the convolution directly costs $O(N^2)$ per layer for $N$ neurons — $1.6\times10^9$ complex products for $N = 40{,}000$. The convolution theorem cuts this to $O(N\log N)$: propagate in the Fourier domain, where convolution becomes multiplication by a transfer function,

$$U^{l+1} = \mathcal{F}^{-1}\Big\{\,\mathcal{F}\{t^l U^l\}\cdot \tilde H_d(f_x,f_y)\,\Big\}, \qquad \tilde H_d(f_x,f_y) = \exp\!\Big(i k d\sqrt{1-(\lambda f_x)^2-(\lambda f_y)^2}\Big).$$

This $\tilde H_d$ is the *angular-spectrum* propagator: it decomposes the field into plane waves $e^{i2\pi(f_x x + f_y y)}$ and advances each by its axial phase over the distance $d$. When $(\lambda f_x)^2+(\lambda f_y)^2 > 1$ the square root is imaginary and the wave is *evanescent* — decaying rather than propagating — which is the frequency-domain statement that features finer than $\sim\lambda$ do not survive the gap. On an $M\times M$ grid each layer costs two FFTs, $O(M^2\log M)$.

---

## 14.1.2.4 Far Field, Degrees of Freedom, and Space–Bandwidth

**Far-field limit.** When the gap is large enough that the quadratic-phase (Fresnel) term is negligible — the Fraunhofer regime — the layer update reduces to a scaled Fourier transform of $t^l U^l$. This is precisely the lens-free version of the Fourier-transforming property Chapter 11 obtained *with* a lens; a D2NN operated in the far field is a trainable, multi-plane generalization of the 4f processor of Subsection 11.1.1. Real D2NNs, however, run in the *near* field (see the example), where the full RS kernel — not its Fourier limit — is required.

**Counting parameters.** The trainable degrees of freedom per layer equal the number of neurons, $N = M^2$; the whole stack has $LN$ free phases. This is bounded above by the **space–bandwidth product** of the optics (Subsection 11.1.1): a layer of aperture $a$ resolving features of size $\sim\lambda$ supports on the order of $(a/\lambda)^2$ independent modes, so a physically larger or shorter-wavelength plate holds more neurons. Kulce et al. sharpened this into an information-capacity statement — the number of independent points of a linear transform a diffractive stack can approximate scales with its total neuron count — so expressivity is bought with pixels and planes.

---

## Worked Example: Two Layers in the Near Field

**Fresnel number.** Take two $200\times200$ layers of aperture $a = 6$ cm (half-aperture $a_0 = 3$ cm) separated by $d = 3$ cm $\approx 40\lambda$ at $\lambda = 0.75$ mm. The Fresnel number is

$$N_F = \frac{a_0^2}{\lambda d} = \frac{(0.03)^2}{(0.75\times10^{-3})(0.03)} = \frac{9\times10^{-4}}{2.25\times10^{-5}} \approx 40.$$

$N_F \approx 40 \gg 1$ places the propagation deep in the *near field*: Fraunhofer/Fourier approximations are invalid, and the exact angular-spectrum kernel must be used — the reason D2NN simulators propagate with $\tilde H_d$ rather than an FFT-as-Fourier-transform shortcut.

**Computational cost.** On the $M = 200$ grid, one propagation is two 2D FFTs, $\sim 2M^2\log_2(M^2) \approx 2(4\times10^4)(15.3) \approx 1.2\times10^6$ complex operations, versus $\sim 1.6\times10^9$ for the naive convolution — the $O(N\log N)$ versus $O(N^2)$ gap that makes training (Subsection 14.1.3) tractable on a single GPU.

---

## References

[1] Goodman, J.W. (2017). *Introduction to Fourier Optics* (4th ed.). W.H. Freeman. [The Rayleigh–Sommerfeld kernel, the angular-spectrum transfer function, the Fresnel/Fraunhofer regimes, and the space–bandwidth product; Chapters 3–5.]

[2] Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008. [States the diffractive layer update as the secondary-source sum used in 14.1.2.1 and the modulation-plus-propagation decomposition.]

[3] Kulce, O., Mengu, D., Rivenson, Y., & Ozcan, A. (2021). "All-optical information-processing capacity of diffractive surfaces." *Light: Science & Applications*, 10, 25. [Relates the neuron count and number of planes to the dimensionality of the linear transforms a stack can realize.]
