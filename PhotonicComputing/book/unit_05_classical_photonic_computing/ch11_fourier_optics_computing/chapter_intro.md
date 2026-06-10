# Chapter 11: Fourier Optics and Classical Optical Computing

## The Natural Computation of a Lens

A converging lens focuses light. This is what every student learns in a first optics course, and it is true. But it is incomplete. A converging lens also performs a mathematical operation that is, depending on how you count, among the most computationally significant in all of applied mathematics: it computes the *two-dimensional Fourier transform* of the optical field at its front focal plane, producing the transform at the back focal plane — instantaneously, in parallel, for every spatial frequency simultaneously.

The Fourier transform of a 1000 × 1000 image using a fast Fourier transform (FFT) algorithm on a CPU takes roughly $N^2 \log_2 N \approx 10^7$ operations. At 10^12 operations/second (a modern CPU core), this takes $\sim 10$ microseconds. A lens performs the same 2D Fourier transform in the time it takes light to travel a focal length — nanoseconds for a 10 cm lens. And it performs the transform for any image, regardless of content, with no programming required.

This is the original "photonic computing" — it predates the phrase by decades. Optical correlation systems using 4f processors were developed in the 1960s for character recognition and pattern matching [1]. Military imaging systems used optical processors for target identification. The fundamental principle — that light naturally computes Fourier transforms, and many useful operations can be expressed as Fourier operations — remains as valid today as it was then.

Why, then, didn't optical computing take over in the 1970s or 1980s? And why is it relevant again now? These questions frame the three sections of this chapter.

---

## Three Sections, Three Domains

**Section 11.1: The 4f Optical Processor** develops the mathematical foundation: the Fourier transforming property of a thin lens, the 4f system, and spatial filtering. We derive from first principles (the Fraunhofer diffraction integral, introduced in Chapter 2) why a lens Fourier transforms the field. We then examine the optical correlator — a 4f system that computes the cross-correlation between an input image and a reference, in a single optical operation — and evaluate honestly when it is competitive with digital FFT-based correlation.

**Section 11.2: Microwave Photonics** examines a domain where optical analog processing is genuinely competitive with electronics today: radio-frequency (RF) signal processing. At frequencies above 20–40 GHz, electronic ADCs cannot digitize fast enough to process signals directly; optical systems, exploiting the far higher bandwidth of optical carriers, can photonically process these signals before (or instead of) digitizing. Photonic ADCs, true time delay beamforming, and photonic radar signal processing are mature enough to be in defense systems currently.

**Section 11.3: Optical Logic Gates** examines why optical Boolean logic has been a research topic for 40 years and has never been deployed. The physics of optical logic — why nonlinearity is required, why the energy-latency product is unfavorable, why SOA-based all-optical gates cannot compete with CMOS — is a cautionary tale about the limits of photonic computing that every student of the field should understand thoroughly before reading optimistic claims about future optical computers.

---

## Why This Chapter Matters for Photonic Computing

The 4f processor and its descendants are the historical and conceptual foundation of photonic computing. Every modern photonic processor — whether an MZI mesh, a diffractive neural network, or a reservoir computer — can be traced back to the same physical principle: that a carefully designed optical system can perform a specific mathematical operation on an input field in the time it takes light to traverse the system.

Understanding the 4f processor deeply also inoculates against one of the most common errors in photonic computing claims: the mistake of comparing optical "computational" operations (Fourier transforms, matrix-vector products performed by diffraction or interference) to sequential electronic operations, without accounting for the preprocessing (input encoding), postprocessing (output readout), and memory operations that any complete computation requires. The 4f processor is genuinely fast at its specific operation. It is not "faster than a computer" at general-purpose computing, because it is not a general-purpose computer.

---

## References

[1] VanderLugt, A. (1964). "Signal detection by complex spatial filtering." *IEEE Transactions on Information Theory*, 10(2), 139–145. [The original VanderLugt filter paper defining the optical correlator; one of the foundational papers in optical computing.]

[2] Goodman, J.W. (2005). *Introduction to Fourier Optics* (3rd ed.). Roberts & Company. [The standard reference for Fourier optics; Goodman's derivation of the lens Fourier transform property is definitive.]

[3] Capmany, J., & Novak, D. (2007). "Microwave photonics combines two worlds." *Nature Photonics*, 1, 319–330. [The review paper that established microwave photonics as a distinct subfield; the best introduction to RF photonic processing.]
