# Chapter 14: Diffractive Deep Neural Networks (D2NN)

## Matter Becomes the Computation

The photonic processors of Chapters 12 and 13 are machines in the ordinary sense — integrated waveguides, actively tuned phase shifters, modulators clocked at gigahertz, a chip reconfigured for every new weight matrix. A *diffractive deep neural network* (D2NN) is something stranger. It is a short stack of passive engineered surfaces — plates of transparent plastic, panels of liquid crystal, or wafers tiled with subwavelength pillars — through which a beam of coherent light simply falls. Nothing along the inference path switches, amplifies, or draws current. The network is trained in a computer; its learned weights are then frozen into the physical relief of the plates, so that the height of the material at each point *is* a parameter of the model. Illuminate the finished stack with an input pattern and the answer assembles itself at the far side, in the time light needs to cross a few centimetres. The computation is not run on the matter — the matter *is* the computation.

This inversion sharpens every trade-off in the book. The integrated networks of Chapters 12–13 are active, reconfigurable, and cascadable: a single chip can hold any weight matrix and can, in principle, be retrained in place (Section 13.3). Their weakness is that each of those virtues costs power, area, and insertion loss. A D2NN spends nothing on reconfigurability because it has none — once printed, the weights are fixed — and in exchange it buys a staggering degree of parallelism almost for free. A single layer of $200\times200 = 40{,}000$ neurons performs, in one optical transit, a dense linear map that an electronic layer would have to wire with 40,000 fan-in connections per neuron; and the wiring is done by diffraction, which costs no hardware at all. The price of that free connectivity is the subject of the whole chapter: a passive diffractive stack is very nearly a *linear* machine, and linearity is exactly what a deep network is supposed to escape.

---

## What This Chapter Establishes

**The framework: diffraction as an all-to-all linear layer** (Section 14.1). Each neuron is a pixel on a thin transmissive layer that multiplies the incident complex field by a trainable transmission coefficient; free-space Rayleigh–Sommerfeld propagation then couples every neuron to every neuron of the next layer. We build the mathematical model — the layer update as a dense but *structured* (convolutional) linear operator — show that a phase-only diffractive stack is a physical realization of a deep *linear* network whose only nonlinearity is intensity detection, and train it by backpropagation through a differentiable diffraction simulator.

**Implementations across the spectrum** (Section 14.2). The learned phase profile can be embodied in many media: 3D-printed dielectric plates for terahertz and millimetre waves, reconfigurable spatial light modulators (SLMs) for visible and near-infrared prototyping, and compact metasurfaces that fold a full layer into a subwavelength-structured film. Each choice fixes the wavelength, the pixel pitch, the achievable phase range, and whether the network can be edited after fabrication.

**Applications** (Section 14.3). D2NNs have been demonstrated as all-optical image classifiers, as engines for optical logic and deterministic linear transforms, and — playing to the native strength of free-space optics — as broadband and spectral processors that sort wavelengths in parallel.

**The hard limits** (Section 14.4). Passivity forbids gain; intensity detection forbids negative outputs without an encoding trick; a printed plate cannot be edited, so the sim-to-real gap becomes a fabrication-tolerance problem; and the number of independent neurons a stack can support is bounded by the space–bandwidth product of the optics. These constraints, not the elegant framework, decide where D2NNs are competitive.

---

## A Reading Frame

Two tensions organize the chapter. First, **passivity versus expressivity**: a passive diffractive stack has no gain and, in its original form, no nonlinearity between layers — a cascade of linear maps that would collapse to a single matrix (Subsection 13.1.1) were it not for the intensity readout at the output plane. Every attempt to add depth — nonlinear detection, optical nonlinear films, opto-electronic layers between plates — is an attempt to buy back the expressivity that passivity gives away. Second, **free-space versus integrated**: the D2NN and the MZI mesh are two physical embeddings of the *same* abstraction, a trained cascade of linear operators. The mesh integrates and reconfigures at the cost of loss and control complexity; the diffractive stack parallelizes and freezes at the cost of editability. Neither dominates — they occupy opposite corners of one design space, and knowing which corner a task belongs in is the practical payoff of this chapter.

---

## References

[1] Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008. [The founding D2NN paper; the framework, experiment, and results that organize this chapter.]

[2] Mengu, D., Luo, Y., Rivenson, Y., & Ozcan, A. (2019). "Analysis of diffractive optical neural networks and their integration with electronic neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 3700114. [Quantitative analysis of D2NN depth, connectivity, and hybrid diffractive–electronic designs.]

[3] Wetzstein, G., Ozcan, A., Gigan, S., Fan, S., Englund, D., Soljačić, M., Denz, C., Miller, D.A.B., & Psaltis, D. (2020). "Inference in artificial intelligence with deep optics and photonics." *Nature*, 588, 39–47. [Cross-community review placing free-space diffractive networks alongside the integrated approaches of Chapters 12–13.]

[4] Goodman, J.W. (2017). *Introduction to Fourier Optics* (4th ed.). W.H. Freeman. [The diffraction theory — angular spectrum, Rayleigh–Sommerfeld, the Fraunhofer limit — on which the entire framework rests.]
