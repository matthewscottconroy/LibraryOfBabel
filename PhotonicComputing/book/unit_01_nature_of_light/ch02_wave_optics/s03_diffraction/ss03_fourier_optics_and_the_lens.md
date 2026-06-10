# 2.3.3 — Fourier Optics and the Lens

## The Lens as a Fourier Transform Engine

A thin lens does something remarkable in wave optics: it converts angles to positions. A ray (or wave) traveling at angle $\theta$ with respect to the optical axis is focused to a spot at position $x = f\theta$ in the back focal plane (for paraxial angles). In terms of spatial frequencies, a plane wave with spatial frequency $f_x = \sin\theta/\lambda \approx \theta/\lambda$ is focused to $x = f\lambda f_x$.

This means: **a thin lens performs a spatial Fourier transform between its front focal plane and its back focal plane**.

Let us state this precisely. Suppose the input field in the front focal plane (at distance $f$ before the lens) is $E_\text{in}(x', y')$. The field in the back focal plane (at distance $f$ after the lens) is:

$$E_\text{out}(x, y) = \frac{-i}{\lambda f} \iint E_\text{in}(x', y') \, e^{-i\frac{2\pi}{\lambda f}(xx'+yy')} \, dx' \, dy'$$

$$= \frac{-i}{\lambda f} \, \hat{E}_\text{in}\!\left(\frac{x}{\lambda f}, \frac{y}{\lambda f}\right)$$

where $\hat{E}_\text{in}(f_x, f_y)$ is the 2D Fourier transform of $E_\text{in}$, evaluated at spatial frequencies $f_x = x/(\lambda f)$, $f_y = y/(\lambda f)$.

The constant prefactor $-i/(\lambda f)$ is a phase and scaling factor; the magnitude distribution is $|E_\text{out}(x,y)| \propto |\hat{E}_\text{in}(x/\lambda f, y/\lambda f)|$, which is the Fourier transform of $E_\text{in}$.

**This is exact in the paraxial approximation.** The lens does not approximate the Fourier transform; it computes it exactly (paraxially). And it does so in the time it takes light to traverse the lens system — nanoseconds for a small system, essentially instantaneous for practical information processing.

## Derivation

Why does a lens compute a Fourier transform? A thin lens of focal length $f$ multiplies the field by the phase factor:

$$t_\text{lens}(x, y) = e^{-i\frac{k}{2f}(x^2+y^2)}$$

(A lens with this phase profile converts a plane wave to a converging spherical wave focused at $z = f$.) The field just after the lens is $E_\text{in}(x, y) \cdot t_\text{lens}(x, y)$. This field then propagates a distance $f$ via the Fresnel integral:

$$E_\text{out}(u, v) = \frac{e^{ikf}}{i\lambda f} \iint E_\text{in}(x, y) \, e^{-i\frac{k}{2f}(x^2+y^2)} \, e^{i\frac{k}{2f}[(u-x)^2+(v-y)^2]} \, dx \, dy$$

Expanding $(u-x)^2 = u^2 - 2ux + x^2$, the quadratic terms in $x$ cancel between the lens phase and the Fresnel propagator:

$$= \frac{e^{ikf}}{i\lambda f} e^{i\frac{k}{2f}(u^2+v^2)} \iint E_\text{in}(x, y) \, e^{-i\frac{2\pi}{\lambda f}(ux+vy)} \, dx \, dy$$

The cancellation of the quadratic phase terms in $x$ and $y$ is the key step — it is precisely what the lens does: it compensates the quadratic phase accumulated during Fresnel propagation, leaving only the linear phase $e^{-i(2\pi/\lambda f)(ux + vy)}$ which is the Fourier kernel.

This derivation shows that the Fourier transform operation is not a consequence of the Fraunhofer approximation; it is exact (paraxially) because the lens compensates the quadratic phase exactly. The result $E_\text{out} \propto \hat{E}_\text{in}(u/\lambda f, v/\lambda f)$ follows.

## The 4f System

Two lenses, each of focal length $f$, placed so that their focal planes coincide (total system length $4f$) form the *4f system* — the workhorse of Fourier optical processing.

```
Input        Lens 1      Fourier plane    Lens 2      Output
plane        (f=...)     (filter plane)    (f=...)     plane
  |             |              |              |            |
  |<--- f ----->|<----- f ---->|<----- f ---->|<--- f ---->|
```

The field at the Fourier plane is the Fourier transform of the input. A mask placed in the Fourier plane multiplies the Fourier-domain field by the mask's transmission function $H(f_x, f_y)$. The second lens takes the inverse Fourier transform (returning to the spatial domain), giving output:

$$E_\text{out}(x, y) \propto \mathcal{F}^{-1}\{H(f_x, f_y) \cdot \hat{E}_\text{in}(f_x, f_y)\}$$

$$= E_\text{in}(x, y) * h(x, y)$$

where $h(x, y)$ is the Fourier transform of the mask function $H$ — the impulse response of the spatial filter. This is the *convolution theorem*: multiplication in Fourier space is convolution in real space.

**The 4f system is an analog optical convolution engine.** It convolves the input field with any desired impulse response, in parallel for all spatial positions, at the speed of light. This is the foundational principle of:

1. **Optical image processing**: Edge detection, blur, sharpening — implemented by placing appropriate masks in the Fourier plane.
2. **Free-space optical matrix multiplication**: A spatial light modulator (SLM) in the Fourier plane implements a diagonal multiplication in Fourier space, which is a convolution in real space.
3. **Diffractive neural networks**: Multiple cascaded 4f systems, each with a programmable or fixed phase mask, implement a sequence of convolutions — exactly what a convolutional neural network does.

## Spatial Filtering: A 4f Example

**Low-pass filtering**: A circular aperture in the Fourier plane with radius $r_0$ passes only spatial frequencies $|f_x|, |f_y| < r_0/(\lambda f)$. This blurs the image (removes high-frequency details). The blur radius in the output plane is $\sim \lambda f / r_0$.

**High-pass filtering (edge detection)**: A central stop (opaque disk) in the Fourier plane blocks low frequencies, passing only high-frequency components. This is the spatial frequency domain implementation of the Laplacian edge detector.

**Phase contrast microscopy** (Zernike, 1942): A phase-shifting element in the Fourier plane converts phase variations in the input (invisible in intensity) into intensity variations in the output image. Zernike won the 1953 Nobel Prize in Physics for this.

## Computing a Matrix-Vector Product Optically

The most important application for photonic computing: consider an $N$-pixel 1D input signal $E_\text{in}(x')$ represented as a discrete sum $\sum_j a_j \delta(x' - x_j)$. After the 4f system with mask $H(f_x)$:

$$E_\text{out}(x_i) = \sum_j a_j H\left(\frac{x_i}{\lambda f}\right) e^{-i2\pi x_i x_j/(\lambda f)}$$

If the mask $H(f_x) = \sum_k W_k \delta(f_x - k/\Lambda)$ is a weighted grating, the output is a weighted sum of the input amplitudes — a matrix-vector product. More generally, a 2D mask in the Fourier plane with transmission $H(f_x, f_y)$ can implement arbitrary linear transformations on 2D input fields.

The Goodman-Psaltis architecture and the more recent D2NN approach both exploit this principle. The key physical fact: optical Fourier transforms are implemented in *hardware*, at the speed of light, with no electronic switching.

## Limits and Challenges

**Space-bandwidth product (SBP)**: The maximum number of independent spatial modes handled by an optical system is the *space-bandwidth product* $SBP = A/\lambda^2$ (roughly), where $A$ is the aperture area and $\lambda$ is the wavelength. A 4f system with $A = 1$ cm² at $\lambda = 1$ μm has $SBP \sim 10^8$ — far more than any current electronic processor can handle per cycle. But practical SLMs have $10^6$–$10^7$ pixels (limited by fabrication), and the total throughput is limited by detector bandwidth.

**Noise**: Any optical computation has shot noise (from the discrete quantum nature of photons) and electronic noise (from detectors). The precision of an analog optical computation is ultimately limited by the signal-to-noise ratio, which scales as $\sqrt{N_\text{photons}}$. This limits the number of effective bits (precision) achievable per computation.

**Nonlinearity**: The 4f system is a linear optical processor. Implementing nonlinear operations (required for deep neural networks) requires nonlinear optical effects or electro-optic readout and re-encoding, which introduces latency.

## Summary

- A thin lens computes the spatial Fourier transform between its front and back focal planes.
- The derivation shows the quadratic phase from Fresnel propagation is exactly cancelled by the lens phase profile.
- The 4f system (two lenses, $f$-$f$-$f$-$f$ spacing) implements convolution via Fourier-domain multiplication.
- Optical Fourier processing is the physical basis of Fourier-optical computing, spatial filtering, and diffractive neural networks.
- SBP determines the dimensionality of the computation; noise limits analog precision.

---

*References*

[1] Goodman, J.W. (2005). *Introduction to Fourier Optics*, 3rd ed. Roberts & Company. [The definitive textbook on Fourier optics and coherent optical processing; Chapters 5–7.]

[2] VanderLugt, A. (1964). Signal detection by complex spatial filtering. *IEEE Transactions on Information Theory*, 10(2), 139–145. [DOI: 10.1109/TIT.1964.1053650] [Early work on matched filtering using 4f systems.]

[3] Lin, X. et al. (2018). All-optical machine learning using diffractive deep neural networks. *Science*, 361(6406), 1004–1008. [DOI: 10.1126/science.aat8084]
