# Unit V: Classical Photonic Computing — From Linear Algebra to Neural Networks

> *"The question is not whether we can build a computer using light, but what kind of computer light is naturally good at being."*
>
> — Paraphrase of a theme running through the photonic computing literature, ca. 2017–present

---

## What This Unit Is About

The first four units of this book built the physical foundation: what light is (Unit I), how it is generated and detected (Unit II), how it is guided and controlled (Unit III), and how information is encoded in it and transmitted across networks (Unit IV). Now we reach the central question: can light *compute*?

The answer, as of 2025, is: yes, for specific things, with important qualifications. Light can perform certain mathematical operations — particularly matrix-vector multiplication — faster, more energy-efficiently, or at a larger scale than conventional electronics under certain conditions. It cannot do everything a computer can do (logical branching, memory access, nonlinear operations) without converting to electronics. The art of photonic computing is matching the operations that light naturally performs to the operations that a useful computation requires.

This unit focuses on *classical* photonic computing — the case where the photons are treated as classical electromagnetic waves carrying analog information, without quantum entanglement or single-photon effects. Classical photonic computing is the most mature part of the field, with demonstrated systems performing real machine learning inference, signal processing, and optimization. It is also the most immediate commercial opportunity, which is why companies like Lightmatter, Lightelligence, Optalysys, and others have organized around it.

The physics behind classical photonic computing comes directly from Units I and III. Interference between optical waves (Chapter 2) is the mechanism by which Mach-Zehnder interferometers perform complex-valued multiplication. Diffraction (Chapter 2) is the mechanism by which free-space optical systems perform Fourier transforms and matrix-vector products in a single optical layer. The refractive index (Chapter 3) and plasma dispersion effect (Chapter 7) are the mechanisms by which electrical signals encode the matrix weights.

---

## Four Chapters, Four Questions

**Chapter 11: Fourier Optics and Classical Optical Computing** asks: *What can a lens do that a computer cannot?* The 4f optical processor performs a two-dimensional Fourier transform in the time it takes light to travel between two lenses — nanoseconds, regardless of the transform size. Microwave photonics uses this principle to process radio-frequency signals at bandwidths that electronic ADCs cannot achieve. We also examine why optical logic gates — using light to compute Boolean functions — have largely failed despite decades of effort, and what makes optical *analog* computing fundamentally different from digital logic.

**Chapter 12: Matrix-Vector Multiplication with Light** asks: *How does an array of interferometers perform a matrix computation?* The Mach-Zehnder interferometer mesh, the singular value decomposition, the Reck and Clements decompositions of unitary matrices — these are the mathematical structures that underlie the optical neural networks demonstrated by Shen et al. (2017) and commercialized by Lightmatter and others. We derive the transfer matrix of the MZI mesh from first principles and analyze the precision, noise, and programming errors that limit practical performance.

**Chapter 13: Photonic Neural Networks** asks: *What happens when you try to implement a deep neural network in optics?* The linear algebra is elegant; the nonlinearity is the problem. Neural networks require nonlinear activation functions, and optical nonlinearities are either too weak (at low power) or too lossy (at high power) to compete with simple electronic implementations. We examine the proposed solutions — electro-optic nonlinearity, saturable absorbers, PCM-based activation — and the training approaches (offline, in-situ, hardware-in-the-loop) that address the reality of imperfect optical components. We also cover photonic reservoir computing and the emerging question of photonic transformers.

**Chapter 14: Diffractive Neural Networks** asks: *Can a stack of passive diffractive layers compute?* The D²NN (deep diffractive neural network) architecture uses trained diffraction patterns to route light from an input plane to output neurons. Chapter 8 introduced the concept; here we develop the mathematical model, training procedure, experimental demonstrations, and fundamental limits in detail. The key tension — that passive linear layers cannot provide nonlinear representational power — reappears here with full mathematical justification, and we examine the proposed solutions (nonlinear materials, optoelectronic feedback) honestly.

---

## The Standard of Evidence in This Unit

Photonic computing attracts both genuine excitement and significant hype. Publications in this space sometimes compare optical performance to worst-case electrical baselines, omit the energy cost of the digital-to-analog converters and analog-to-digital converters that interface optical systems to the real world, or describe demonstrations at laboratory scales that do not obviously extrapolate to useful systems.

This unit applies the same standard of evidence used throughout the book: every performance claim is accompanied by its conditions, every advantage is accompanied by its constraints, and fundamental limits are stated clearly even when they are inconvenient for the narrative. The goal is to leave the reader with a genuine understanding of where photonic computing is today, where it can realistically go, and where the claims exceed the physics.

---

## Prerequisites and Connections

This unit builds heavily on:
- **Section 2.2** (Interference) and **Section 2.3** (Diffraction) — the wave optics of the MZI and the diffractive layer
- **Section 7.3** (Silicon photonic modulators) — the device physics of encoding weights
- **Chapter 9** (Information theory) — for analyzing the precision and SNR of analog optical computations
- **Chapter 10** (Optical interconnects) — for understanding the system context in which photonic processors are deployed

---

## References for the Unit Introduction

[1] Shen, Y., et al. (2017). "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11, 441–446. [The paper that established the MZI mesh neural network as a viable architecture and triggered the current wave of photonic deep learning research.]

[2] Lin, X., et al. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008. [The D²NN paper; demonstrates passive diffractive layers for optical classification at THz frequencies.]

[3] Wetzstein, G., et al. (2020). "Inference in artificial intelligence with deep optics and neural networks." *Nature*, 588, 39–47. [Broad review of optical neural network approaches; good framing of the state of the art.]

[4] Shastri, B.J., et al. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15, 102–114. [The field review that covers both classical and neuromorphic photonic computing; the best single reference for the entire unit.]
