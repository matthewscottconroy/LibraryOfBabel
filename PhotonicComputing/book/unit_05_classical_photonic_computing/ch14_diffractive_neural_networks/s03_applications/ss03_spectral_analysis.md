# Subsection 14.3.3: Spectral Analysis and Object Detection

## Orientation

Diffraction is dispersive: both the free-space propagation kernel and the height-encoded phase mask $\phi = \frac{2\pi}{\lambda}(n-1)h$ depend on wavelength. A diffractive network trained across a band therefore does different things to different colours — a nuisance for a single-wavelength classifier, but the entire point when the task is spectral. This subsection covers broadband D2NNs as spectral filters and spectrometers, their use as terahertz pulse shapers, and their role as computational-imaging front ends.

---

## 14.3.3.1 Broadband Diffractive Networks

A monochromatic D2NN is trained at one $\lambda$; a broadband D2NN is trained with the wavelength dependence of diffraction included, over a continuous band, so that its response is a *designed* function of $\lambda$ (Luo et al. 2019). This turns the network into a spectral engineer. It can be trained as a wavelength filter passing a chosen band to a chosen location, as a spatial-spectral mapper that sends each wavelength to its own output position (a single-shot spectral encoder), or as a task-specific spectral system that realizes some target spectral transfer function. Because the mapping is fixed in passive hardware, the spectral operation happens at the speed of light with no moving parts and no dispersive scanning.

## 14.3.3.2 The D2NN as a Grating Spectrometer

A diffractive aperture that sorts wavelengths by position is, in its resolving power, a diffraction grating. The standard figure of merit is $R = \lambda/\Delta\lambda$, the ratio of wavelength to the smallest resolvable wavelength difference. For a grating $R = m\,N$, the diffraction order times the number of illuminated periods; for a diffractive layer the analogue is set by the number of resolution-limited features across the aperture, $N_x = D/\Delta x$ for aperture width $D$ and pixel pitch $\Delta x$. More features means a longer path-length difference between the aperture's edges, hence finer spectral discrimination — and it caps the number of independent spectral channels the network can map to distinct outputs at roughly $R$.

## 14.3.3.3 Worked Example: Spectral Channels at Terahertz

Take a terahertz diffractive layer at the scale of Section 14.2.1: aperture $D = 8$ cm, pixel pitch $\Delta x = 0.4$ mm, so

$$N_x = \frac{D}{\Delta x} = \frac{80\ \text{mm}}{0.4\ \text{mm}} = 200,$$

giving a first-order resolving power $R \approx 200$. At $f = 0.4$ THz ($\lambda = 0.75$ mm), the resolvable wavelength step is

$$\Delta\lambda \approx \frac{\lambda}{R} = \frac{0.75\ \text{mm}}{200} \approx 3.8\ \mu\text{m},$$

or, in frequency, $\Delta f \approx f/R = 400\ \text{GHz}/200 = 2$ GHz. To map, say, $N = 10$ spectral bins onto ten separate output detectors, the band must span at least $N\,\Delta f = 20$ GHz — a band from $0.39$ to $0.41$ THz sorted into ten 2-GHz channels, each steered to its own location. The ceiling is $R \approx 200$ channels; beyond that, adjacent wavelengths overlap at the output. This is the same space-bandwidth budget as before, now spent on spectral rather than spatial degrees of freedom — a wider aperture buys either more pixels or more colours, but the product is fixed (Section 14.4).

## 14.3.3.4 Pulse Shaping and Computational Imaging

Two further application families use the same physics. In **pulse shaping**, Veli et al. (2021) trained diffractive surfaces to synthesize a target terahertz temporal waveform: because a pulse is a coherent superposition of frequencies, engineering the network's complex response across the band — its amplitude and phase at each $\lambda$ — sculpts the output pulse in time, an all-optical, passive alternative to electronic pulse synthesis. In **computational imaging**, diffractive front ends have been trained for object detection, wide-field and quantitative-phase imaging, and all-optical image processing such as edge enhancement and spatial filtering, performing the first layer of a vision pipeline in the optics before any photon is digitized. In each case the diffractive network does what it does best from this section's opening argument: computing on light that is already there, natively and at capture time, rather than digitizing first and computing after.

---

## References

[1] Luo, Y., Mengu, D., Yardimci, N.T., Rivenson, Y., Veli, M., Jarrahi, M., & Ozcan, A. (2019). "Design of task-specific optical systems using broadband diffractive neural networks." *Light: Science & Applications*, 8, 112. [Broadband diffractive networks with engineered wavelength-dependent response — spectral filters and task-specific spectral systems; the central reference here.]

[2] Veli, M., Mengu, D., Yardimci, N.T., Luo, Y., Li, J., Rivenson, Y., Jarrahi, M., & Ozcan, A. (2021). "Terahertz pulse shaping using diffractive surfaces." *Nature Communications*, 12, 37. [Diffractive surfaces synthesizing target terahertz temporal/spectral pulse profiles.]

[3] Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008. [The diffractive framework and terahertz scale used in the spectral-resolution worked example.]
