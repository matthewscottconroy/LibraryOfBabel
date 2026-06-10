# Unit I: The Nature of Light — Classical Electromagnetism and Wave Optics

> *"From a long view of the history of mankind — seen from, say, ten thousand years from now — there can be little doubt that the most significant event of the 19th century will be judged as Maxwell's discovery of the laws of electrodynamics. The American Civil War will pale into provincial insignificance in comparison with this important scientific event of the same decade."*
>
> — Richard Feynman, *The Feynman Lectures on Physics*, Vol. II, Chapter 1

---

## What This Unit Is About

Before we can understand how light computes, we must understand what light *is*. This is not a formality. Every design decision in photonic computing — why we use 1550 nm wavelength light, why we care about polarization, why coherent systems are more sensitive to phase noise than incoherent ones, why optical fibers can carry terabits of data over thousands of kilometers, why a silicon waveguide bends light into a 5-micron-radius curve — every one of these facts flows directly from the physics developed in this unit.

The physics is James Clerk Maxwell's, and it is one of the great intellectual achievements in human history. In four compact equations — equations you can write on a napkin — Maxwell unified electricity, magnetism, and optics into a single theory, and in doing so predicted the existence of electromagnetic waves traveling at the speed of light. He made this prediction in 1865, twenty-three years before Heinrich Hertz experimentally confirmed the existence of radio waves [1]. The theory was correct not because Maxwell had experimental data on electromagnetic waves; he did not. It was correct because the logical structure of the equations *demanded* it.

This story matters for photonic computing for a reason that goes beyond historical appreciation: the structure of Maxwell's equations is what makes photonic computing physically possible. It is the reason that light can carry information without dissipating it as heat. It is the reason that two optical signals can cross each other in free space without interfering destructively. It is the reason that light can be guided in waveguides, modulated by electric fields, and amplified by lasers. If you understand Maxwell's equations — really understand them, not just know how to manipulate them — you understand why photonic computing works at all.

---

## Three Chapters, Three Questions

This unit contains three chapters, each organized around a central question.

**Chapter 1: Maxwell's Equations and Electromagnetic Waves** asks: *What is light, mathematically?* We derive Maxwell's equations from their empirical origins, show how they demand the existence of electromagnetic waves, and explore what those waves carry — energy, momentum, and angular momentum. We pay careful attention to the meaning of each equation, because the meaning is what makes the mathematics generative rather than merely descriptive.

**Chapter 2: Wave Optics — Interference, Diffraction, and Coherence** asks: *How does light behave when its wave nature matters?* Geometric optics is adequate when wavelengths are small compared to structures; wave optics is required when they are not — and in photonic computing, they are never not. Interference is the mechanism by which MZI-based optical processors perform computation. Diffraction is the mechanism by which diffractive neural networks function. Coherence is the property that distinguishes laser light from sunlight and determines what kinds of computation are possible.

**Chapter 3: Light-Matter Interaction** asks: *What happens when light meets matter?* A photon in vacuum is simple. A photon in a dielectric medium, or in a semiconductor, or in a nonlinear crystal, is the beginning of everything useful. The refractive index, absorption, stimulated emission, the Kerr effect, optical solitons — all of these arise from the interaction of the electromagnetic field with the microscopic degrees of freedom of matter.

---

## The Mathematical Language of This Unit

The primary mathematical tools of this unit are:
- **Vector calculus**: gradient, divergence, curl, the divergence theorem, Stokes' theorem
- **Partial differential equations**: the wave equation, the Helmholtz equation
- **Fourier analysis**: decomposing signals into frequency components
- **Complex numbers and phasors**: representing oscillatory quantities
- **Linear algebra**: polarization states as vectors, Jones matrices as linear maps

These are not prerequisites. They are developed as needed, always with physical motivation.

---

## A Note on First Principles

The phrase "first principles" is often used loosely to mean "starting from something fundamental." In physics it has a more specific meaning: starting from empirically established laws, without assuming any result that has not been derived or measured.

This unit is genuinely first-principles. We start from Coulomb's law, Faraday's experiments, and Maxwell's reasoning about the displacement current. We derive the wave equation, solve it, and interpret the solutions. We do not import results from nowhere.

This matters for photonic computing because the field is young enough that some claims about it are not well-established. By building from first principles, we develop the physical intuition needed to evaluate those claims honestly.

---

## References for the Unit Introduction

[1] Hertz, H. (1888). "Über sehr schnelle electrische Schwingungen." *Annalen der Physik und Chemie*, 267(7), 421–448. [The original experimental paper demonstrating electromagnetic waves, confirming Maxwell's prediction.]

[2] Maxwell, J.C. (1865). "A dynamical theory of the electromagnetic field." *Philosophical Transactions of the Royal Society of London*, 155, 459–512. [The paper in which Maxwell predicted electromagnetic waves propagating at the speed of light.]

[3] Feynman, R.P., Leighton, R.B., & Sands, M. (1964). *The Feynman Lectures on Physics*, Vol. II. Addison-Wesley. [Feynman's magnificent introduction to electromagnetism; the quote opening this unit is from Chapter 1.]
