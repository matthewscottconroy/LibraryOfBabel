# Chapter 2: Wave Optics — Interference, Diffraction, and Coherence

> *"The phenomena of the interference of two portions of light...seem to be decisive in favour of the undulatory theory, and it is to be hoped, that if they are sufficiently examined and considered, they will be found to be equally irreconcilable with any modification of the projectile hypothesis."*
>
> — Thomas Young, *Philosophical Transactions of the Royal Society*, 1804

---

## Why Wave Optics Is Not Optional

In the previous chapter, we established that light is an electromagnetic wave. That is the physics. But knowing that light is a wave does not automatically tell you how it behaves in every situation. Under some circumstances — when wavelengths are much smaller than the structures that guide and redirect the light — the wave nature is largely invisible, and light behaves as if it traveled in straight rays. This is geometric optics: the optics of lenses and mirrors and prisms, adequate for telescopes and cameras and eyeglasses, but blind to the phenomena that make photonic computing possible.

Wave optics becomes indispensable when the wavelength is comparable to the structures involved. In photonic computing, this is always the situation. A silicon waveguide is 450 nm wide and carries light at 1550 nm wavelength. A Mach-Zehnder interferometer operates by combining two optical paths that differ by a fraction of a wavelength. A diffractive optical neural network layer uses diffraction patterns at scales of micrometers. A ring resonator accumulates light through constructive interference with a circumference measured in tens of microns. In every one of these devices, the physics is wave physics, and understanding it requires the tools developed in this chapter.

The central organizing principle of wave optics is *superposition*. Because Maxwell's equations are linear (in vacuum and in linear media), solutions can be added together. When two wave solutions are present simultaneously, the total field is their sum — and the intensity (which is proportional to the square of the total field) is not simply the sum of the individual intensities. The cross terms are the interference terms, and interference is the phenomenon from which all the rest of wave optics flows.

This chapter develops wave optics systematically, with careful attention to what each result means physically and how it applies specifically to photonic computing.

---

## The Arc of This Chapter

**Section 2.1 — Geometric Optics** begins, perhaps surprisingly, with the limit in which wave effects are negligible. Geometric optics is not merely a warm-up; it introduces *Fermat's principle* (that light takes the path of stationary phase), *Snell's law* (derived, not assumed), and *ray transfer matrices*, which are the simplest instance of the linear algebraic description of optical systems. These tools remain useful even in the wave regime, and ray transfer matrices will reappear when we analyze Gaussian beam propagation.

**Section 2.2 — Interference** is the heart of wave optics. We derive the condition for constructive and destructive interference, define the visibility of an interference pattern, analyze Young's double-slit (the canonical demonstration of wave behavior), and then move to the two devices most important for photonic computing: the *Fabry-Pérot resonator* (the foundation of all optical cavities, lasers, and resonator-based modulators) and the *Mach-Zehnder interferometer* (MZI), the fundamental building block of photonic neural networks.

**Section 2.3 — Diffraction and Fourier Optics** develops the wave theory of diffraction, starting with the Huygens-Fresnel principle and arriving at the Fraunhofer far-field limit. The most important result is that a converging lens performs a *Fourier transform*: the field in the back focal plane is the spatial Fourier transform of the field in the front focal plane. This is the physical basis of all Fourier-optical computing, including 4f optical processors, and it is the reason that diffractive neural networks can implement matrix operations.

**Section 2.4 — Polarization** treats the transverse nature of electromagnetic waves in full. Light's polarization state is a two-dimensional complex vector, and optical elements that act on polarization are linear operators on that vector space — *Jones matrices*. We develop Jones calculus completely, describe the Stokes parameters and the Poincaré sphere, analyze birefringence and wave plates, and explain why polarization is an important resource for encoding and processing information in photonic systems.

**Section 2.5 — Coherence** addresses the question of how wave-like a given light source actually is. A laser is highly coherent: its electromagnetic field is nearly a pure sinusoidal wave, stable over long distances and times. A thermal source (an LED, a lamp) is incoherent: its field is a random superposition of many modes. The degree of coherence determines what kinds of interference are visible, and it has profound implications for the kind of computation a photonic system can perform. We develop both temporal coherence (the bandwidth constraint) and spatial coherence (the van Cittert-Zernike theorem), and we analyze why coherent optical computing systems are more sensitive to phase noise than incoherent ones.

**Section 2.6 — Gaussian Beams** develops the paraxial wave equation and its most important solution: the Gaussian beam. Real laser beams are Gaussian to a good approximation, and the Gaussian beam model provides exact analytical expressions for beam radius, wavefront curvature, and divergence as functions of propagation distance. The ABCD matrix formalism — a generalization of the ray transfer matrices from Section 2.1 — allows Gaussian beam propagation through arbitrary optical systems to be computed by matrix multiplication. This is used in Chapter 4 (lasers) and Unit V (photonic neural networks) extensively.

---

## Mathematical Prerequisites for This Chapter

This chapter uses:
- Complex exponentials (from Chapter 1, Section 4.4)
- The concept of spatial frequency and Fourier transforms
- Basic linear algebra (vectors, matrices, eigenvalues)
- The integral calculus of Fourier analysis

The Fourier transform is introduced as needed in Section 2.3, with physical motivation. The linear algebra of Jones calculus is elementary — 2×2 matrices acting on 2-vectors — and is developed from scratch in Section 2.4.

---

## The Computing Connection

Every topic in this chapter has a direct photonic computing application:

- **Ray transfer matrices** → ABCD formalism for Gaussian beams in optical cavities and resonators
- **Fabry-Pérot resonators** → ring resonator weight banks in photonic neural networks
- **Mach-Zehnder interferometers** → MZI mesh neural networks (Unit V, Chapter 11)
- **Fourier optics** → free-space optical matrix multipliers and diffractive neural networks (Unit V, Chapter 13)
- **Polarization and Jones matrices** → polarization-multiplexed photonic computing; polarization as an extra degree of freedom
- **Coherence** → fundamental limits on analog optical computation; noise and precision analysis
- **Gaussian beams** → coupling between lasers, fibers, and chips; mode matching in photonic integrated circuits

Understanding these connections is the point of the chapter. Wave optics is not abstract mathematics; it is the engineering physics of photonic computing hardware.

---

## References for the Chapter Introduction

[1] Young, T. (1804). The Bakerian lecture: Experiments and calculations relative to physical optics. *Philosophical Transactions of the Royal Society of London*, 94, 1–16. [The paper in which Young introduced the principle of interference and used the double-slit experiment to measure the wavelength of light.]

[2] Fresnel, A.-J. (1816). Mémoire sur la diffraction de la lumière. *Annales de Chimie et de Physique*, 1, 239–281. [Fresnel's theory of diffraction, combining Huygens's principle with the principle of interference.]

[3] Born, M. & Wolf, E. (1999). *Principles of Optics*, 7th ed. Cambridge University Press. [The comprehensive reference for classical wave optics; particularly Chapters 7–10 on interference, diffraction, and partial coherence.]

[4] Goodman, J.W. (2005). *Introduction to Fourier Optics*, 3rd ed. Roberts & Company. [The definitive text on Fourier optical systems and their applications to information processing.]
