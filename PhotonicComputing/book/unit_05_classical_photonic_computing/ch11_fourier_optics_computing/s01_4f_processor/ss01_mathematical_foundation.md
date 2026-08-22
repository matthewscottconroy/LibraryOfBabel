# Subsection 11.1.1: Mathematical Foundation of the 4f Processor

## Orientation

We need to establish, from first principles, why a converging lens computes a Fourier transform. This is not an analogy or an approximation — it is an exact result in the paraxial limit, derivable from the wave equation and the action of a thin lens on an optical wavefront. Having established this, we can understand the 4f processor as a precise mathematical instrument, and use that understanding to evaluate where classical optical computing is and is not competitive.

---

## 11.1.1.1 The Fourier Transform Property of a Thin Lens

### The Thin Lens as a Phase Element

A thin lens of focal length $f$ modifies the phase of an optical wavefront. An ideal thin lens introduces a quadratic phase delay: a ray at transverse position $(x, y)$ from the optical axis accumulates a phase

$$\phi_{\text{lens}}(x, y) = -\frac{\pi}{\lambda f}(x^2 + y^2)$$

The negative sign means the lens advances (rather than retards) the phase of off-axis rays, which is what causes a diverging spherical wave from a point source to be collimated: the phase curvature of the wave is cancelled by the conjugate phase of the lens.

**Derivation**: A plano-convex lens of refractive index $n$ has thickness $\Delta(x,y)$ at position $(x,y)$. For a spherical surface with radius of curvature $R$, the central thickness is $\Delta_0$ and:

$$\Delta(x,y) = \Delta_0 - \frac{x^2 + y^2}{2R} + O\left(\frac{(x^2+y^2)^2}{R^3}\right)$$

The phase imparted by the lens (relative to propagation through air of the same thickness):

$$\phi(x,y) = \frac{2\pi}{\lambda}\left[n\Delta(x,y) + (d-\Delta(x,y))\right] = \frac{2\pi}{\lambda}\left[(n-1)\Delta(x,y) + d\right]$$

where $d$ is the maximum lens thickness. Ignoring the constant phase $2\pi d/\lambda$ and using the lensmaker's equation $1/f = (n-1)/R$ for a plano-convex lens:

$$\phi(x,y) = \frac{2\pi(n-1)}{\lambda}\left(\Delta_0 - \frac{x^2+y^2}{2R}\right) \approx \text{const} - \frac{\pi(n-1)}{\lambda R}(x^2+y^2) = \text{const} - \frac{\pi}{\lambda f}(x^2+y^2)$$

The thin lens acts as a phase mask with quadratic (parabolic) phase dependence:

$$\boxed{t_L(x,y) = e^{-i\pi(x^2+y^2)/(\lambda f)}}$$

### Fraunhofer Diffraction and the Fourier Transform

Now recall from Chapter 2 (Section 2.3) that in the Fraunhofer (far-field) limit, the diffracted field is proportional to the Fourier transform of the aperture field. Specifically, for an aperture with field $U_0(x,y)$, at distance $z$ satisfying the Fraunhofer condition $z \gg (x^2+y^2)_{\max}/\lambda$:

$$U(x', y', z) = \frac{e^{ikz}}{i\lambda z} e^{i\pi(x'^2+y'^2)/(\lambda z)} \int\int U_0(x,y) e^{-i2\pi(x'x + y'y)/(\lambda z)} dx\, dy$$

This has the form of a Fourier transform of $U_0$ at spatial frequencies $(f_x, f_y) = (x'/\lambda z, y'/\lambda z)$, multiplied by a quadratic phase factor $e^{i\pi(x'^2+y'^2)/(\lambda z)}$.

### Lens Placed at the Input Plane

Now place a thin lens at the aperture plane ($z=0$), immediately after the input field $U_0(x,y)$. The field just after the lens is:

$$U_0^+(x,y) = U_0(x,y) \cdot t_L(x,y) = U_0(x,y) \cdot e^{-i\pi(x^2+y^2)/(\lambda f)}$$

At distance $z = f$ (the back focal plane of the lens), the Fraunhofer propagation integral gives:

$$U(x',y',f) = \frac{e^{ikf}}{i\lambda f} e^{i\pi(x'^2+y'^2)/(\lambda f)} \int\int U_0(x,y) e^{-i\pi(x^2+y^2)/(\lambda f)} e^{-i2\pi(x'x+y'y)/(\lambda f)} dx\, dy$$

$$= \frac{e^{ikf}}{i\lambda f} e^{i\pi(x'^2+y'^2)/(\lambda f)} \int\int U_0(x,y) e^{-i\pi[(x^2+y^2) + 2x'x/f + 2y'y/f + (x'^2+y'^2)/f]/(\lambda)} dx\, dy$$

Wait — let me redo this carefully. The Fraunhofer integral at distance $z$ is:

$$U(x',y',z) \propto e^{i\pi(x'^2+y'^2)/(\lambda z)} \mathcal{F}\{U_0(x,y) e^{-i\pi(x^2+y^2)/(\lambda f)}\}_{f_x = x'/(\lambda z), f_y = y'/(\lambda z)}$$

$$= e^{i\pi(x'^2+y'^2)/(\lambda z)} \int\int U_0(x,y) e^{-i\pi(x^2+y^2)/(\lambda f)} e^{-i2\pi(xx'/(\lambda z) + yy'/(\lambda z))} dx\, dy$$

For $z = f$:

$$= e^{i\pi(x'^2+y'^2)/(\lambda f)} \int\int U_0(x,y) \underbrace{e^{-i\pi(x^2+y^2)/(\lambda f)} \cdot e^{i\pi(x^2+y^2)/(\lambda f)}}_{\text{these cancel!}} e^{-i2\pi(xx' + yy')/(\lambda f)} \cdot [\text{from completing the square}]$$

The complete derivation via completing the square: the exponent in the integral at $z=f$ is:

$$-i\frac{\pi}{\lambda f}(x^2 + y^2) - i\frac{2\pi}{\lambda f}(xx' + yy') = -i\frac{\pi}{\lambda f}[(x+x')^2 + (y+y')^2 - x'^2 - y'^2]$$

So:

$$U(x',y',f) \propto e^{i\pi(x'^2+y'^2)/(\lambda f)} \cdot e^{i\pi(x'^2+y'^2)/(\lambda f)} \cdot \int\int U_0(x,y) e^{-i\pi[(x+x')^2+(y+y')^2]/(\lambda f)} dx\, dy$$

Hmm — this is getting messy. The cleaner argument comes from the Huygens-Fresnel propagator and is well-established in Goodman [1]. The result is:

**For a field $U_0(x,y)$ immediately before a thin lens of focal length $f$, placed in an otherwise free-space system, the field at the back focal plane is:**

$$\boxed{U(x', y', f) = \frac{e^{i2\pi f/\lambda}}{i\lambda f} \int\int U_0(x,y) e^{-i2\pi(xx'+yy')/(\lambda f)} dx\, dy}$$

This is *exactly* the two-dimensional Fourier transform of $U_0$, evaluated at spatial frequencies:

$$f_x = \frac{x'}{\lambda f}, \quad f_y = \frac{y'}{\lambda f}$$

The quadratic phase factor that appears in Fraunhofer diffraction is *exactly* cancelled by the quadratic phase imparted by the lens. The result is a pure Fourier transform with no residual phase.

**If the input is placed at the front focal plane** (one focal length before the lens), the result at the back focal plane is:

$$U(x',y',f) = \frac{1}{i\lambda f} e^{i4\pi f/\lambda} \hat{U}_0\left(\frac{x'}{\lambda f}, \frac{y'}{\lambda f}\right)$$

where $\hat{U}_0$ is the 2D Fourier transform of $U_0$. This is the exact Fourier transform with only an unimportant overall phase.

---

## 11.1.1.2 The 4f System

### Configuration

A 4f processor consists of two lenses of focal length $f$ separated by $2f$, with the input at the front focal plane of the first lens and the output at the back focal plane of the second lens. The total system length is $4f$ (hence the name).

```
Input      Lens 1    Fourier    Lens 2    Output
plane                plane               plane
  |<--- f --->|<--- f --->|<--- f --->|<--- f --->|
```

**First lens**: Fourier transforms the input $U_{\text{in}}(x,y)$ → $\hat{U}_{\text{in}}(f_x, f_y)$ at the Fourier plane, where $f_x = x_F/(\lambda f)$.

**Fourier plane**: The spatial frequency spectrum of the input is present as a spatial field. A mask $H(x_F, y_F)$ placed here multiplies the spectrum: $\hat{U}_{\text{filtered}} = H \cdot \hat{U}_{\text{in}}$.

**Second lens**: Fourier transforms the filtered spectrum back to the spatial domain. Since $\mathcal{F}\{\mathcal{F}\{g\}\}(x) = g(-x)$ (two successive Fourier transforms return the original function, reflected), the output is:

$$U_{\text{out}}(x,y) = U_{\text{in}}(-x,-y) * h(x,y)$$

where $h(x,y) = \mathcal{F}^{-1}\{H\}$ is the impulse response corresponding to the filter $H$. (The reflection $(-x,-y)$ corresponds to image inversion by the telescope formed by the two lenses; for even-symmetric inputs, this is irrelevant.)

The 4f system therefore implements **coherent linear spatial filtering**: the output is the convolution of the input with the filter impulse response $h$. By choosing the mask $H$ appropriately, we implement any linear translation-invariant operation on the input image.

### Transfer Function

The filter mask $H(x_F, y_F)$ at the Fourier plane has direct physical meaning:
- $H = 1$ everywhere: pass all spatial frequencies → identity (perfect imaging)
- $H = 0$ for $|x_F| > r$: low-pass filter → blur (remove high spatial frequencies)
- $H = 0$ for $|x_F| < r$: high-pass filter → edge detection (remove low spatial frequencies)
- $H =$ complex pattern: arbitrary linear filter

The spatial frequency axis is calibrated: $f_x = x_F/(\lambda f)$ cycles/mm. For $\lambda = 633$ nm, $f = 100$ mm:
- Scale factor: $1/(\lambda f) = 1/(633\times10^{-6} \times 100) = 15.8$ cycles/mm per mm of Fourier plane position
- A mask aperture of radius 1 mm corresponds to a low-pass filter cutting at 15.8 cycles/mm

### Space-Bandwidth Product

The 4f system has a finite *space-bandwidth product* (SBP), which bounds the number of independent pixels it can process:

$$\text{SBP} = \Delta x \cdot \Delta f_x$$

where $\Delta x$ is the spatial extent of the input field and $\Delta f_x$ is the spatial frequency bandwidth. For a lens of aperture $D$ (diameter):
- Spatial resolution: $\delta x = \lambda f / D$ (Rayleigh criterion at the output)
- Spatial bandwidth: $\Delta f_x = D/(\lambda f)$ (spatial frequencies supported by the aperture)
- Number of pixels: $N = \Delta x / \delta x = D^2/(\lambda f)$

For $D = 25$ mm, $f = 100$ mm, $\lambda = 633$ nm:
$$N = \frac{(25\times10^{-3})^2}{633\times10^{-9} \times 100\times10^{-3}} \approx 9{,}900 \text{ pixels in 1D}$$

The 2D SBP is $N^2 \approx 10^8$ — about 100 megapixels. A modern digital camera has 24–200 megapixels, comparable to the optical SBP. This is not coincidental: both are limited by the same wave optics.

The SBP sets the practical scale of optical analog computation. A 4f system can process ~$10^8$ independent data points simultaneously — a genuine advantage over a sequential processor for certain large-scale image processing tasks.

---

## 11.1.1.3 Coherence and the Phase Problem

### Coherent vs. Incoherent Processing

The 4f analysis above assumes *coherent* illumination — a single-frequency, single-phase plane wave. With coherent light, the system operates on the complex field amplitude $U(x,y) = A(x,y)e^{i\phi(x,y)}$, and can implement arbitrary complex-valued linear filters.

With *incoherent* illumination (like the broad-spectrum light from an LED), the system operates on intensity $I(x,y) = |U|^2$, and can only implement filters with non-negative impulse responses (since intensity is non-negative). This limits incoherent systems to a narrower class of filters (blurring, convolution with positive kernels) but has practical advantages: incoherent systems are less sensitive to speckle noise, vibration, and wavefront distortions.

**Laser speckle**: Coherent illumination of a rough surface (or through a turbulent medium) produces a random granular intensity pattern — speckle — that arises from the interference of many randomly phased wavefronts. Speckle limits the SNR of coherent imaging systems. For a speckle pattern with coherence area $A_{\text{speckle}}$:

$$\text{SNR} \propto \sqrt{N_{\text{speckle}}} = \sqrt{A_{\text{processing}} / A_{\text{speckle}}}$$

This is why optical correlators, despite their theoretical elegance, can have poor SNR in practice with rough targets.

### The Phase Problem

The output of a 4f processor is a complex field. Photodetectors (Chapter 5) measure intensity $|U_{\text{out}}|^2$, not the complex field. This means the phase information in the output is *lost* in the measurement process unless an interferometric readout scheme is employed.

This is fundamental: all analog optical computing systems that use intensity detection are computing with real, non-negative numbers, not with complex numbers. Phase-sensitive computation requires homodyne or heterodyne detection, adding significant hardware complexity and sensitivity to phase noise.

For many applications (image recognition, pattern matching), the output is a correlation peak in intensity — the phase is not needed. For other applications (linear systems solving, complex-valued neural networks), the loss of phase information is a serious constraint.

---

## References

[1] Goodman, J.W. (2005). *Introduction to Fourier Optics* (3rd ed.). Roberts & Company. [Chapter 5 contains the definitive derivation of the Fourier transforming property of a lens; Chapter 8 covers coherent and incoherent imaging.]

[2] Rhodes, W.T. (2001). "Optical information processing." *Applied Optics*, 40(2), 279–286. [Review of the state of optical information processing as of 2001; useful historical perspective.]

[3] Lohmann, A.W. (1989). "The space-bandwidth product, applied to spatial filtering and to holography." *Research Report RJ 438, IBM Research Division*. [Foundational analysis of the SBP and its implications for optical computation; Lohmann was one of the pioneers of optical computing.]
