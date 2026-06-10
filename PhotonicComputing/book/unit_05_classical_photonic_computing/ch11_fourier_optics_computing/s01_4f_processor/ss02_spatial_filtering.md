# Subsection 11.1.2: Spatial Filtering

## Orientation

With the 4f system established as a physical implementation of coherent linear spatial filtering, we now examine what filters are useful, how they are implemented physically, and what their limitations are in practice. Spatial filtering is not merely an academic exercise — it is the mechanism by which the 4f processor performs computation. The "filter" at the Fourier plane is the program.

---

## 11.1.2.1 The Filter Plane as a Programmable Processor

### Physical Implementation of Filters

A spatial filter in the 4f system is a physical mask placed at the Fourier plane. Several technologies implement programmable (software-controlled) filters:

**Fixed masks**: Photographically produced amplitude masks $H(x_F, y_F)$ on a glass substrate, or chrome-on-glass for high contrast. Cheap, stable, but not reconfigurable.

**Spatial Light Modulators (SLMs)**: As discussed in Section 8.2, an SLM is a programmable 2D array that modulates phase, amplitude, or both. Liquid crystal on silicon (LCoS) SLMs achieve:
- Pixel pitch: 3.74–12 μm (512×512 to 4096×4096 arrays)
- Phase range: 0–$2\pi$ (for phase-only SLMs)
- Update rate: 30–180 Hz (video rate)
- Efficiency: 90–99% reflectivity (phase-only); 25–50% (amplitude, using polarization selection)

An LCoS SLM at the Fourier plane makes the 4f system a programmable analog processor: upload a new mask pattern to the SLM, and the filter changes in one video frame ($\sim 10$ ms). The computational "program" is the SLM pattern; the "data" is the input field.

**Acousto-optic modulators (AOMs)**: A piezoelectric transducer drives acoustic waves in a crystal (TeO₂, LiNbO₃). The acoustic wave creates a periodic density (and thus refractive index) grating that diffracts light. By controlling the acoustic frequency and amplitude, the AOM implements a 1D complex filter with ~100 MHz update bandwidth. Used in microwave photonic signal processing (Section 11.2).

**Digital micromirror devices (DMDs)**: Binary amplitude modulation; each micromirror is $\pm 12°$ (ON or OFF). Very fast (>20 kHz), but only binary — cannot implement arbitrary complex filters.

### Low-Pass and High-Pass Filters

**Low-pass filter** (spatial smoothing): Block high spatial frequencies by placing an aperture (a circular stop) at the center of the Fourier plane. Only $|f_x|, |f_y| < f_{\text{cut}}$ passes. The output is the input convolved with:

$$h(x,y) = \mathcal{F}^{-1}\{\text{circ}(r/r_0)\} = \frac{r_0 J_1(2\pi r_0 r)}{r}$$

where $J_1$ is the first-order Bessel function. This is the Airy disk pattern — the same point spread function that appears in telescope imaging. The filter removes sharp edges (high spatial frequencies), blurring the image. Used in optical coherence tomography, confocal microscopy.

**High-pass filter** (edge detection): Block low spatial frequencies with an opaque stop at the center (a central obstruction). High frequencies pass; the output emphasizes edges and sharp features. For an input image $f(x,y)$:

$$f_{\text{high-pass}}(x,y) = f(x,y) - f_{\text{low-pass}}(x,y)$$

This is the optical analog of a discrete Laplacian or Sobel edge filter.

**Band-pass filter**: A ring-shaped aperture in the Fourier plane passes only spatial frequencies in a specific range. Used in phase-contrast microscopy and surface defect inspection.

**Differential phase contrast**: A "half-plane" filter $H(f_x, f_y) = \text{sgn}(f_x)$ computes the derivative of the input in one direction:
$$U_{\text{out}}(x,y) = \frac{\partial U_{\text{in}}}{\partial x}$$

---

## 11.1.2.2 The VanderLugt Filter

### Complex-Valued Matched Filtering

The most powerful application of spatial filtering is the *matched filter* (also called the *VanderLugt filter* after its inventor [1]), which computes the cross-correlation between the input image and a reference pattern.

Cross-correlation of $f$ and $g$:
$$(f \star g)(x,y) = \int\int f^*(x'-x, y'-y) g(x',y') dx'\, dy'$$

By the correlation theorem of Fourier analysis:
$$\mathcal{F}\{f \star g\} = \hat{f}^*(f_x, f_y) \cdot \hat{g}(f_x, f_y)$$

where $\hat{f}^*$ is the complex conjugate of the Fourier transform of $f$.

To implement this in a 4f system, the filter mask at the Fourier plane must be:
$$H(f_x, f_y) = \hat{f}^*\left(\frac{x_F}{\lambda f}, \frac{y_F}{\lambda f}\right)$$

This is a *complex-valued* filter — it requires both amplitude and phase modulation. For a VanderLugt filter, the filter mask is recorded holographically:

1. Record a hologram of the reference pattern $f$ at the Fourier plane (by interfering the Fourier transform of $f$ with a reference plane wave).
2. During operation, place the input $g$ at the input plane.
3. The output plane shows the cross-correlation $(f \star g)$.

### Pattern Recognition

If the input $g$ contains a version of the reference $f$ translated by $(x_0, y_0)$:
$$g(x,y) = f(x-x_0, y-y_0) + \text{noise}$$

then the cross-correlation output contains a sharp peak at $(x_0, y_0)$:
$$(f \star g)(x,y) = (f \star f)(x-x_0, y-y_0) + (f \star \text{noise})(x,y)$$

The autocorrelation $(f \star f)$ of a complex pattern has a narrow, bright peak at the origin (for patterns with good autocorrelation properties). The output peak position tells us *where* in the input image the reference pattern appears.

This is the optical correlator, described in Subsection 11.1.3. The key property: the correlation is computed *optically*, in parallel for the entire input image, in a single propagation through the 4f system.

### Limitations of the VanderLugt Filter

The VanderLugt filter in its original form has two practical limitations:

**Phase sensitivity**: The filter must be recorded and replayed with phase accuracy better than $\lambda/10$ (i.e., $\sim 60$ nm). Thermal and vibrational instability makes this difficult in practical systems. The shift from holographic film to SLMs helps but does not eliminate the problem.

**Non-negative amplitude constraint**: For amplitude-only SLMs or fixed amplitude masks, the complex-valued $H(f_x, f_y)$ must be approximated. Phase-only SLMs (LCoS) can implement $|H|=1$ with arbitrary phase, but cannot implement arbitrary amplitude modulation. This limits the quality of the matched filter.

---

## 11.1.2.3 Coherent vs. Incoherent Processing Trade-offs

### When Each Is Superior

| Property | Coherent | Incoherent |
|----------|----------|------------|
| Filter type | Complex-valued (amplitude + phase) | Real, non-negative |
| Noise | Speckle-limited | Shot noise + thermal |
| Vibration sensitivity | High (phase noise) | Low |
| Dynamic range | Limited by speckle | Limited by detector |
| Applications | Pattern matching, edge detection | Widefield imaging, microscopy |
| Computation model | Complex linear algebra | Non-negative linear algebra |

For photonic computing applications, the coherence choice has a direct implication: coherent systems can implement complex-valued matrix-vector products (the physically natural operation for neural networks using MZI meshes, Chapter 12), while incoherent systems can only implement non-negative matrix-vector products.

### The Incoherent Advantage for Machine Learning

There is a genuine advantage to incoherent processing for certain neural network architectures. If the weight matrix $W$ has some negative entries, an incoherent system cannot directly implement $W$. But if we represent $W = W^+ - W^-$ (positive minus negative parts), we can implement each part separately (at the cost of doubling the hardware) and subtract the results electrically. This is the "split-photodetector" trick used in WDM wavelength-division neural network architectures (Section 12.4).

Alternatively, one can work entirely in the non-negative domain (as all-incoherent systems do), but this restricts the representational power of the network — not all useful weight matrices are non-negative. This constraint is analyzed in Chapter 14 in the context of diffractive networks.

---

## References

[1] VanderLugt, A. (1964). "Signal detection by complex spatial filtering." *IEEE Transactions on Information Theory*, 10(2), 139–145. [The original matched filter paper; defines what is now called the VanderLugt filter and demonstrates optical pattern recognition.]

[2] Weaver, C.S., & Goodman, J.W. (1966). "A technique for optically convolving two functions." *Applied Optics*, 5(7), 1248–1249. [The joint transform correlator; a simpler implementation that avoids recording a filter hologram.]

[3] Javidi, B., & Horner, J.L. (1994). "Optical pattern recognition for validation and security verification." *Optical Engineering*, 33(6), 1752–1756. [Review of practical applications of optical correlators in security and ID verification.]

[4] Psaltis, D., & Farhat, N. (1985). "Optical information processing based on an associative-memory model of neural nets with thresholding and feedback." *Optics Letters*, 10(2), 98–100. [Early proposal for optical neural networks using Fourier optics; the conceptual ancestor of modern photonic neural networks.]
