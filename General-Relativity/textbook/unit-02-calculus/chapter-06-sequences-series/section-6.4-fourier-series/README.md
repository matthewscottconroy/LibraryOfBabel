# Section 6.4: Fourier Series

---

## Section Introduction

Every pure tone in music is a sine wave. Every sound we hear — a spoken word, a struck chord, a thunderclap — is a superposition of pure tones. Fourier's insight, formalized in 1822, is that this decomposition is universal: *any* periodic function can be expressed as an infinite series of sines and cosines. This is the **Fourier series**, and it is one of the most useful and influential ideas in all of mathematics.

The key question is what "expressed as" means precisely. For a smooth, well-behaved function, the Fourier series converges pointwise everywhere. For a discontinuous function — a square wave, a sawtooth — the Fourier series converges everywhere except at the discontinuities, and near a discontinuity there is an overshoot (the **Gibbs phenomenon**) that persists at every level of approximation. Understanding exactly what the Fourier series converges to, and how fast, is an entire branch of harmonic analysis.

The **Fourier transform** generalizes Fourier series from periodic to aperiodic functions, replacing the discrete frequency index with a continuous frequency variable. The transform decomposes a signal into its continuous spectrum. The Fourier transform is the central tool of signal processing, quantum mechanics (the position-space and momentum-space wave functions are Fourier transforms of each other), and PDE theory (where it converts differential equations into algebraic equations in frequency space).

In physics, Fourier analysis is everywhere. The heat equation in Chapter 11 is solved by Fourier series. The hydrogen atom wave functions are expanded in spherical harmonics — the angular analogue of Fourier modes. Gravitational wave detectors (LIGO) use Fourier analysis to extract signals from noise. Cosmological perturbation theory expands fluctuations in Fourier modes to decouple the linear evolution equations. The language of Fourier analysis is so pervasive in physics that it is impossible to proceed without it.

---

## Subsections

- [6.4.1: Periodic Functions and Fourier Coefficients](6.4.1-fourier-coefficients.md)
- [6.4.2: Convergence of Fourier Series](6.4.2-convergence.md)
- [6.4.3: Parseval's Theorem and Energy](6.4.3-parseval.md)
- [6.4.4: The Fourier Transform](6.4.4-fourier-transform.md)
- [6.4.5: Applications to Differential Equations](6.4.5-pde-applications.md)
