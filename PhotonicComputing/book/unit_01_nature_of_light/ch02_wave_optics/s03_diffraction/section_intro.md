# Section 2.3: Diffraction and Fourier Optics

If interference is what two waves do at a point, diffraction is what a single wave does as it bends around obstacles and through apertures. The word "diffraction" suggests something exotic or peripheral — a correction to be applied when geometric optics breaks down. But the reality is deeper: diffraction is not a separate phenomenon added on top of wave propagation. It is wave propagation, seen from the perspective of a wave encountering a finite aperture or obstacle. The wave does not "know" it should travel in a straight ray; it propagates in all directions consistent with the wave equation. What we observe as diffraction is the interference of the infinite collection of Huygens wavelets emitted by every point of the wavefront.

The deeper insight — and the one most important for photonic computing — is the connection between diffraction and the Fourier transform. In the far field (Fraunhofer diffraction limit), the diffracted field is precisely the spatial Fourier transform of the aperture field. And a lens, by focusing the far-field pattern onto a finite focal plane, implements a Fourier transform optically: the field in the back focal plane of a lens is the Fourier transform of the field in the front focal plane.

This connection between lenses and Fourier transforms is the foundation of Fourier optics and free-space optical computing. It means that a physical system — two lenses separated by twice their focal length — performs, on an optical field, the same operation that a computer performs in software when it runs an FFT. The difference is that the optical Fourier transform is performed at the speed of light, in parallel for all spatial frequencies simultaneously, with no multiplications — just propagation.

## Subsections

- **2.3.1 — The Huygens-Fresnel Principle**: The physical basis of diffraction; Kirchhoff's integral formula; Fresnel vs. Fraunhofer limits.
- **2.3.2 — Fraunhofer Diffraction**: The far-field diffraction pattern; single slit; rectangular and circular apertures; the sinc and Airy disk patterns.
- **2.3.3 — Fourier Optics and the Lens**: The lens as a Fourier transform engine; the 4f system; spatial filtering; applications to optical computing.
- **2.3.4 — Diffraction Gratings**: Multiple-slit interference; grating equation; wavelength demultiplexing; echelle gratings; diffractive optical elements.
