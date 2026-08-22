# PHOTONIC COMPUTING: FROM FIRST PRINCIPLES TO THE FRONTIERS OF LIGHT-SPEED INFORMATION PROCESSING

## A Comprehensive Textbook Outline

---

## Preface

*Why write a book about photonic computing now? Because we are standing at an inflection point. For seventy years, computation has meant electrons — electrons shuttling through silicon, carrying bits as voltage levels, switching at the command of billions of transistors etched onto chips no larger than a fingernail. It is a story of stunning triumph, tracing Moore's Law across orders of magnitude until the limits of physics itself are being pressed. But now, at precisely the moment when silicon is straining, a new protagonist is emerging: the photon. Massless, unchargeable, capable of carrying information at the speed of light and across vast bandwidths without crosstalk — photons are not a replacement for electrons so much as a liberation from them. This book is your guide to understanding how light computes, why it matters, and how to build systems that harness it.*

---

## How to Use This Book

This text is organized into ten units progressing from classical electromagnetism through integrated photonics, optical communications, analog and digital photonic computing, neuromorphic photonic systems, and quantum photonic computing, culminating at the research frontier. Each chapter contains:

- **Motivating Introduction** — situating the topic within the larger narrative
- **Mathematical Development** — built from first principles, step-by-step
- **Worked Examples** — fully solved, referenced to real systems
- **Exercises** — mathematical, conceptual, and applied
- **Engineering/Programming Projects** — hands-on builds and simulations
- **Key Researchers** — the people who made this field
- **Further Reading** — primary papers, textbooks, lecture notes

**Prerequisite path:** Calculus → Linear Algebra → Differential Equations → Introductory Physics → Programming (Python). Each unit opens with a review of the mathematical tools it will use.

---

# UNIT I: THE NATURE OF LIGHT — CLASSICAL ELECTROMAGNETISM

> *Imagine you are handed a compass and told to figure out the universe. Hans Christian Ørsted did exactly that in 1820, noticing that a current-carrying wire deflected his compass needle. From that humble observation, through the hands of Ampère, Faraday, and finally James Clerk Maxwell, emerged four equations that unified electricity, magnetism, and light — and inadvertently launched the information age. These equations govern every photon in every optical fiber, every laser pulse in every data center, every qubit in every quantum photonic processor. They are the bedrock on which this entire book is built.*

---

## Chapter 1: Maxwell's Equations and Electromagnetic Waves

### 1.1 The Historical Path to Maxwell
- 1.1.1 Coulomb's law and the inverse-square force
- 1.1.2 Gauss's law as the field-theoretic formulation
- 1.1.3 Biot-Savart and Ampère's law for steady currents
- 1.1.4 Faraday's law of induction: the first hint of light
- 1.1.5 Maxwell's displacement current: the crowning unification
- 1.1.6 Historical context: the ether, Hertz, and the confirmation of electromagnetic waves

**Key Researchers:** Maxwell, Faraday, Hertz, Heaviside (who reformulated Maxwell's 20 equations into the 4 we use today)

### 1.2 Maxwell's Equations in Integral Form

#### 1.2.1 Gauss's Law for Electric Fields
$$\oint_S \mathbf{E} \cdot d\mathbf{A} = \frac{Q_{\text{enc}}}{\varepsilon_0}$$
- Physical meaning: electric flux through a closed surface equals enclosed charge over ε₀
- Deriving field of a point charge, line charge, plane
- Worked example: field inside and outside a uniformly charged sphere

#### 1.2.2 Gauss's Law for Magnetic Fields
$$\oint_S \mathbf{B} \cdot d\mathbf{A} = 0$$
- Physical meaning: no magnetic monopoles; field lines form closed loops
- Contrast with electric case
- Implications for photon polarization states

#### 1.2.3 Faraday's Law of Induction
$$\oint_C \mathbf{E} \cdot d\boldsymbol{\ell} = -\frac{d}{dt}\int_S \mathbf{B} \cdot d\mathbf{A}$$
- A changing magnetic field creates a circulating electric field
- Lenz's law and energy conservation
- Worked example: EMF in a rotating loop

#### 1.2.4 The Ampère-Maxwell Law
$$\oint_C \mathbf{B} \cdot d\boldsymbol{\ell} = \mu_0 I_{\text{enc}} + \mu_0\varepsilon_0\frac{d}{dt}\int_S \mathbf{E} \cdot d\mathbf{A}$$
- Maxwell's displacement current term: physical motivation
- Why symmetry demanded it
- The displacement current inside a charging capacitor (worked example)

### 1.3 Maxwell's Equations in Differential Form

#### 1.3.1 Vector Calculus Review
- Gradient ∇f, divergence ∇·**F**, curl ∇×**F**
- The divergence theorem: relating surface integrals to volume integrals
- Stokes' theorem: relating line integrals to surface integrals
- Worked examples for each operator in Cartesian, cylindrical, and spherical coordinates
- The Laplacian ∇²f

#### 1.3.2 The Differential Maxwell Equations
$$\nabla \cdot \mathbf{E} = \frac{\rho}{\varepsilon_0}$$
$$\nabla \cdot \mathbf{B} = 0$$
$$\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t}$$
$$\nabla \times \mathbf{B} = \mu_0 \mathbf{J} + \mu_0\varepsilon_0\frac{\partial \mathbf{E}}{\partial t}$$

- Derivation of each from the integral form
- Maxwell's equations in matter: **D** = ε**E**, **H** = **B**/μ, polarization **P**, magnetization **M**
- Boundary conditions at interfaces (critical for waveguide theory)

### 1.4 Deriving the Electromagnetic Wave Equation

#### 1.4.1 Step-by-Step Derivation
- Start from curl of Faraday's law: ∇×(∇×**E**) = -∂(∇×**B**)/∂t
- Apply the vector identity: ∇×(∇×**E**) = ∇(∇·**E**) - ∇²**E**
- In free space (ρ=0, J=0): ∇·**E**=0
- Substitute Ampère-Maxwell: obtain
$$\nabla^2 \mathbf{E} = \mu_0\varepsilon_0\frac{\partial^2 \mathbf{E}}{\partial t^2}$$
- Identical equation for **B**
- Identify wave speed: $c = \frac{1}{\sqrt{\mu_0\varepsilon_0}} \approx 3\times10^8$ m/s

#### 1.4.2 Plane Wave Solutions
- Trial solution: **E** = **E₀** exp(i(**k**·**r** - ωt))
- Substitution and dispersion relation: k² = ω²/c², so k = ω/c
- The wavevector **k**, wavelength λ, frequency ν, angular frequency ω
- Transversality condition: **k**·**E₀** = 0
- Relation between **E** and **B**: **B** = (**k**×**E**)/ω
- Phase velocity v_p = ω/k, group velocity v_g = dω/dk

#### 1.4.3 The Electromagnetic Spectrum
- Gamma rays through radio waves
- Optical windows relevant to photonic computing: 850 nm (VCSEL), 1310 nm, 1550 nm (telecom C-band)
- Photon energy E = hν = ℏω
- Worked example: energy per photon at 1550 nm vs. 850 nm

#### 1.4.4 Complex Notation and the Physical Field
- Analytic signal representation
- Time-averaging of Poynting vector
- Why complex exponentials simplify oscillatory mathematics

### 1.5 Energy and Momentum in Electromagnetic Fields

#### 1.5.1 The Poynting Vector
$$\mathbf{S} = \frac{1}{\mu_0}(\mathbf{E} \times \mathbf{B}) = \mathbf{E} \times \mathbf{H}$$
- Energy flux density [W/m²]
- Poynting's theorem: energy conservation in EM fields
- Intensity of a plane wave: I = |**S**| = (1/2)ε₀c|**E₀**|²
- Worked example: intensity of a 1 mW laser focused to 10 μm spot

#### 1.5.2 Radiation Pressure
- Momentum density of EM field: g = S/c²
- Radiation pressure P = I/c (absorbing surface), P = 2I/c (reflecting surface)
- Optical tweezers: trapping particles with light
- Relevance: optical forces on photonic chip structures

#### 1.5.3 Electromagnetic Momentum and Angular Momentum
- Spin angular momentum: σ± = ±ℏ per photon (circular polarization)
- Orbital angular momentum: lℏ per photon (vortex beams)
- Applications in quantum optical computing

### 1.6 Maxwell's Equations in Media

#### 1.6.1 Dielectric Materials
- Electric susceptibility χ_e: **P** = ε₀χ_e**E**
- Dielectric constant ε_r = 1 + χ_e
- Wave speed in medium: v = c/n, where n = √(ε_r μ_r)
- Dispersion: n = n(ω)

#### 1.6.2 Conducting Media and Skin Depth
- Complex permittivity: ε̃ = ε' + iε''
- Attenuation in conductors
- Skin depth δ = √(2/ωμσ)
- Why metals are lossy for optical waveguides

#### 1.6.3 Boundary Conditions (Derivation from Integral Form)
- Tangential **E** continuous across interface
- Normal **D** continuous (no surface charges)
- Tangential **H** continuous (no surface currents)
- Normal **B** continuous
- These four conditions will recur throughout waveguide, fiber, and resonator analysis

### 1.7 Exercises

**Mathematical:**
1. Verify that **E**(z,t) = E₀ x̂ cos(kz - ωt) satisfies the wave equation. Find the corresponding **B** field.
2. Using the divergence theorem, derive the differential form of Gauss's law from the integral form.
3. A plane wave propagates in the +z direction with E₀ = 10⁶ V/m. Calculate: (a) the magnetic field amplitude, (b) the Poynting vector magnitude, (c) the radiation pressure on a perfect mirror.
4. Show that ∇·**B** = 0 is automatically satisfied if **B** = ∇×**A** for some vector potential **A**.
5. Derive the wave equation for **H** in free space starting from Maxwell's equations.

**Conceptual:**
6. Why does Maxwell's displacement current term restore symmetry between Faraday's and Ampère's laws?
7. Light carries momentum. If a 1 mW laser beam is absorbed by a black surface for 1 second, what is the impulse delivered?
8. Explain why electromagnetic waves are transverse. What does this imply for photon polarization states?

**Applied:**
9. A single-mode optical fiber carries 1 mW of 1550 nm light in a mode field diameter of 10 μm. Estimate the peak electric field amplitude in the core.
10. In a silicon waveguide (n = 3.47), what is the wavelength of 1550 nm light? What is the phase accumulated over 1 cm?

### 1.8 Programming Projects

**Project 1.1: FDTD 1D Solver** — Implement a 1D finite-difference time-domain simulation of a plane wave hitting a dielectric interface. Observe reflection and transmission. Verify against Fresnel coefficients. (Python, numpy)

**Project 1.2: Poynting Vector Visualizer** — For a given plane wave polarization, compute and plot the instantaneous and time-averaged Poynting vector field in 2D. Animate the propagation.

**Project 1.3: Dispersion Explorer** — Plot the phase and group velocity for a dispersive medium modeled by a Lorentz oscillator. Show normal and anomalous dispersion regimes.

### 1.9 Further Reading
- **Textbooks:** Griffiths, *Introduction to Electrodynamics* (4th ed.); Jackson, *Classical Electrodynamics* (3rd ed.); Born & Wolf, *Principles of Optics*
- **Papers:** Maxwell (1865), "A Dynamical Theory of the Electromagnetic Field," *Phil. Trans. Roy. Soc.*
- **Online:** MIT OCW 8.03 (Physics III: Vibrations and Waves); Feynman Lectures on Physics Vol. II

---

## Chapter 2: Wave Optics — Interference, Diffraction, and Coherence

> *Before the laser, optics was largely a science of averages — averaged over time, averaged over wavelength, averaged over the thermal chaos of incoherent light sources. The laser changed everything, producing light of extraordinary purity and coherence. But to appreciate what makes laser light special — and why it is the indispensable fuel of photonic computing — you must first understand wave optics in all its richness: interference, diffraction, the Fourier connection, and the deep concept of coherence. This chapter builds those tools.*

### 2.1 Geometric (Ray) Optics

#### 2.1.1 Fermat's Principle
- Light takes the path of stationary optical path length
- Derivation of reflection law from Fermat's principle
- Derivation of Snell's law: n₁ sin θ₁ = n₂ sin θ₂
- When ray optics is valid: λ ≪ feature size

#### 2.1.2 Ray Transfer Matrices (ABCD Matrices)
- Representing ray as column vector [y, θ]ᵀ
- Free propagation matrix M_prop
- Thin lens matrix M_lens
- Interface matrix M_interface
- Concatenation of matrices for optical systems
- Gaussian beam propagation with ABCD matrices
- Worked example: imaging through a two-lens system
- Worked example: finding the focal plane of a compound lens

#### 2.1.3 Total Internal Reflection
- Critical angle: θ_c = arcsin(n₂/n₁), n₁ > n₂
- Evanescent wave beyond the critical angle
- Frustrated total internal reflection
- Basis of fiber optic waveguiding

### 2.2 The Scalar Wave Equation and Huygens' Principle

#### 2.2.1 Scalar Approximation
- When polarization can be ignored
- The scalar wave equation: ∇²U - (1/v²)∂²U/∂t² = 0
- Monochromatic fields: U(**r**,t) = u(**r**)e^{-iωt}
- Helmholtz equation: ∇²u + k²u = 0

#### 2.2.2 Huygens-Fresnel Principle
- Every wavefront point is a secondary source of spherical wavelets
- Fresnel-Kirchhoff diffraction integral
- The obliquity factor

#### 2.2.3 Optical Path Length and Phase
- OPL = n · L
- Phase shift: φ = k · OPL = (2π/λ) · n · L
- Accumulation through a medium of varying n(**r**)

### 2.3 Interference

#### 2.3.1 Superposition of Two Plane Waves
- U_total = U₁ + U₂
- Intensity: I = |U_total|² = I₁ + I₂ + 2√(I₁I₂)cos(Δφ)
- Constructive (Δφ = 2mπ) and destructive (Δφ = (2m+1)π) interference
- Visibility: V = (I_max - I_min)/(I_max + I_min)

#### 2.3.2 Young's Double-Slit Experiment
- Geometry: two slits separated by d, screen at distance L
- Path difference: Δ = d sin θ ≈ dy/L (paraxial)
- Fringe spacing: Δy = λL/d
- Full intensity pattern derivation
- Worked example: fringe spacing for λ=1550 nm, d=10 μm, L=1 m

#### 2.3.3 The Fabry-Pérot Cavity
- Two parallel mirrors, reflectivities R₁, R₂, separation L
- Round-trip phase: δ = 2kL = 4πnL/λ
- Transmission vs. wavelength (Airy function):
$$T = \frac{(1-R)^2}{(1-R)^2 + 4R\sin^2(\delta/2)}$$
- Free spectral range (FSR): Δν_FSR = c/(2nL)
- Finesse: F = π√R/(1-R)
- Linewidth: Δν = FSR/F
- Q factor: Q = ν₀/Δν = ω₀ · (energy stored)/(power lost)
- Worked example: silicon microring resonator (L ~ 100 μm, Q ~ 10⁵)
- Applications: laser cavities, wavelength filters, ring modulators

#### 2.3.4 Mach-Zehnder Interferometer
- Splitting and recombining: arms of length L₁ and L₂
- Output intensity as function of phase difference Δφ
- Transfer function: T = cos²(Δφ/2)
- Electro-optic modulation: tuning Δφ with voltage
- Balanced MZI as 50/50 coupler at Δφ = π/2
- Worked example: compute insertion loss and extinction ratio for a given Δφ

### 2.4 Diffraction and Fourier Optics

#### 2.4.1 Fraunhofer Diffraction
- Far-field (Fraunhofer) condition: z ≫ a²/λ
- Fraunhofer diffraction integral:
$$U(x,y) \propto \iint u(\xi,\eta) e^{-i\frac{2\pi}{\lambda z}(x\xi + y\eta)} d\xi\, d\eta$$
- Recognition as a 2D Fourier transform
- Single slit: sinc pattern
- Circular aperture: Airy disk, resolution limit
- Worked example: diffraction-limited spot size of a focusing lens

#### 2.4.2 The Lens as a Fourier Transform Processor
- A thin lens of focal length f performs a 2D Fourier transform between its front and back focal planes
- The 4f optical system: cascaded Fourier transform and inverse transform
- Spatial filtering in the Fourier plane
- Optical matched filter: correlation detection
- Optical convolution: the basis of diffractive neural networks

#### 2.4.3 Diffraction Gratings
- Grating equation: mλ = d(sin θ_i + sin θ_m)
- Blazed gratings for efficiency
- Arrayed waveguide gratings (AWGs) as on-chip gratings
- Resolving power R = mN

### 2.5 Polarization

#### 2.5.1 States of Polarization
- Linear: **E** oscillates along fixed axis
- Circular: **E** rotates (σ+ right-hand, σ- left-hand)
- Elliptical: general case
- Degree of polarization

#### 2.5.2 Jones Vector and Jones Matrix Formalism
- Jones vector: [E_x, E_y]ᵀ (complex amplitudes)
- Linear horizontal: [1, 0]ᵀ; vertical: [0, 1]ᵀ
- Right circular: [1, i]ᵀ/√2; left: [1, -i]ᵀ/√2
- Linear polarizer (horizontal): [[1,0],[0,0]]
- Quarter-wave plate (fast axis horizontal): [[1,0],[0,-i]]
- Rotation matrix for polarizer at angle θ
- Cascade: M_total = M_N · ... · M_2 · M_1
- Worked example: HWP + polarizer = rotated polarization
- Worked example: QWP converts linear to circular

#### 2.5.3 Stokes Parameters and Poincaré Sphere
- S₀ = I_total, S₁ = I_H - I_V, S₂ = I_45 - I_135, S₃ = I_R - I_L
- Relation to Jones vector: S₀ = |E_x|² + |E_y|²
- Poincaré sphere: all polarization states as points on a sphere
- Poles: circular polarizations; equator: linear polarizations
- Importance for polarization multiplexing in communications

#### 2.5.4 Birefringence and Wave Plates
- Ordinary and extraordinary refractive indices
- Phase retardation Γ = (2π/λ)(n_e - n_o)L
- Half-wave plate (Γ=π): rotates polarization
- Quarter-wave plate (Γ=π/2): converts linear ↔ circular
- Liquid crystal cells as tunable wave plates
- Polarization mode dispersion in fibers

### 2.6 Coherence

#### 2.6.1 Temporal Coherence
- Mutual coherence function: Γ(τ) = ⟨U*(t)U(t+τ)⟩
- Normalized: γ(τ) = Γ(τ)/Γ(0)
- Coherence time τ_c and coherence length L_c = c·τ_c
- Wiener-Khinchin theorem: S(ν) = ∫Γ(τ)e^{i2πντ}dτ
- Relation to spectral linewidth: τ_c ~ 1/Δν
- Worked example: coherence length of a DFB laser (Δν ~ 1 MHz) vs. LED (Δν ~ 10 THz)
- Why coherence matters for MZI-based photonic processors

#### 2.6.2 Spatial Coherence
- Mutual coherence function: Γ(**r₁**, **r₂**, τ)
- van Cittert-Zernike theorem: spatial coherence of an incoherent source
- Spatial coherence of a laser beam

#### 2.6.3 Implications for Photonic Computing
- Coherent systems: interference-based, require phase stability
- Incoherent systems: intensity-based, more robust to phase noise
- Coherence requirements for MZI mesh processors
- Coherence requirements for quantum photonic systems

### 2.7 Gaussian Beams

#### 2.7.1 Paraxial Wave Equation
- Slowly varying envelope: U = u(r)e^{ikz}
- Paraxial approximation: ∂²u/∂z² ≪ k∂u/∂z
- Results in: ∇_T²u + 2ik∂u/∂z = 0

#### 2.7.2 Gaussian Beam Solution
$$u(r,z) = \frac{w_0}{w(z)}\exp\left(-\frac{r^2}{w^2(z)}\right)\exp\left(-ikz - ik\frac{r^2}{2R(z)} + i\zeta(z)\right)$$
- Beam waist w₀ at z=0
- Rayleigh range: z_R = πw₀²/λ
- Beam radius: w(z) = w₀√(1 + (z/z_R)²)
- Radius of curvature: R(z) = z(1 + (z_R/z)²)
- Gouy phase: ζ(z) = arctan(z/z_R)
- Divergence angle: θ = λ/(πw₀)
- Worked example: coupling a diode laser to a fiber

### 2.8 Exercises

**Mathematical:**
1. Derive the Fabry-Pérot transmission function from first principles using the sum of a geometric series for multiple reflections.
2. A lens of focal length f = 50 mm is illuminated by a plane wave through a circular aperture of diameter D = 5 mm. Find the Airy disk radius in the focal plane for λ = 633 nm.
3. Prove that the Jones matrix for a half-wave plate at angle θ to the horizontal rotates linearly polarized light by 2θ.
4. Show from the Fabry-Pérot formula that the finesse F ≈ π√R/(1-R) for R close to 1.
5. A Gaussian beam has waist w₀ = 5 μm at λ = 1550 nm. Calculate the Rayleigh range, the beam radius at z = 1 mm, and the far-field divergence angle.

**Conceptual:**
6. Why does increasing the finesse of a Fabry-Pérot cavity narrow its transmission peaks? What is the trade-off?
7. A Mach-Zehnder interferometer is used as a binary optical switch. What phase difference is needed? How precise must the phase control be for 30 dB extinction ratio?
8. Explain the van Cittert-Zernike theorem in physical terms. Why does a distant incoherent source appear spatially coherent?

**Applied:**
9. A microring resonator has diameter 10 μm, effective index 2.5, group index 4.2. Calculate the FSR and finesse if the power loss per round trip is 1%.
10. An AWG-based demultiplexer must separate 1550 nm and 1551 nm channels. Estimate the required grating order and number of waveguide arms for a 5 cm device.

### 2.9 Programming Projects

**Project 2.1: Interferometer Simulator** — Simulate a Mach-Zehnder interferometer. Plot transmission vs. phase difference. Add thermal noise to the phase and compute the resulting ON/OFF ratio distribution. (Python)

**Project 2.2: Fourier Optics Image Processor** — Implement a 4f optical system in simulation. Apply spatial filters (low-pass, high-pass, notch) to images. Demonstrate edge detection via high-pass filtering. Compare to digital convolution.

**Project 2.3: Diffraction Pattern Calculator** — Compute and animate Fresnel diffraction patterns for various aperture shapes (slit, circle, annulus). Show convergence to Fraunhofer limit as z → ∞.

**Project 2.4: Gaussian Beam Propagator** — Implement Gaussian beam propagation through a sequence of lenses using ABCD matrix method. Optimize coupling efficiency into a single-mode fiber.

**Project 2.5: Fabry-Pérot Resonator Analyzer** — Plot the transmission spectrum of a Fabry-Pérot cavity. Sweep reflectivity and length. Identify longitudinal modes. Simulate a silicon microring filter for WDM channel selection.

### 2.10 Further Reading
- **Textbooks:** Goodman, *Introduction to Fourier Optics* (4th ed.); Saleh & Teich, *Fundamentals of Photonics* (3rd ed.); Hecht, *Optics* (5th ed.)
- **Key Papers:** Kogelnik & Li (1966), "Laser Beams and Resonators," *Applied Optics*
- **Online:** MIT OCW 6.637 (Optical Signals, Devices and Systems)

---

## Chapter 3: Light-Matter Interaction — From Absorption to Nonlinear Optics

> *Light doesn't just pass through matter — it negotiates with it. The negotiation happens at the atomic level, where electrons respond to the electric field of a passing electromagnetic wave, reradiating energy and altering the field's character. For small fields, the response is linear: doubling the field doubles the response. But push hard enough — with the intense fields available from modern lasers — and the response becomes nonlinear. Harmonics are generated, frequencies are mixed, solitons form in fibers, and new wavelengths emerge from nothing. These nonlinear phenomena are both the challenge and the opportunity of photonic computing.*

### 3.1 Classical Model: The Lorentz Oscillator

#### 3.1.1 Driven Harmonic Oscillator
- Electron bound to nucleus modeled as mass on spring
- Equation of motion: mẍ + mγẋ + mω₀²x = -eE(t)
- With E(t) = E₀e^{-iωt}: x(ω) = -eE₀/m · 1/(ω₀² - ω² - iγω)
- Dipole moment: p = -ex
- Polarization: P = Nqx where N = number density

#### 3.1.2 Complex Susceptibility and Refractive Index
- χ(ω) = P/(ε₀E): electric susceptibility
- Explicit form from Lorentz model:
$$\chi(\omega) = \frac{Ne^2}{m\varepsilon_0}\frac{1}{\omega_0^2 - \omega^2 - i\gamma\omega}$$
- Complex refractive index: ñ = n + iκ = √(1 + χ)
- Real part n: phase velocity dispersion
- Imaginary part κ: absorption (κ > 0) or gain (κ < 0)
- Absorption coefficient: α = 2ωκ/c [m⁻¹]
- Beer-Lambert law: I(z) = I₀ e^{-αz}

#### 3.1.3 Normal and Anomalous Dispersion
- Below resonance (ω < ω₀): dn/dω > 0 (normal dispersion)
- Above resonance (ω > ω₀): dn/dω < 0 (anomalous dispersion)
- Group velocity: v_g = c/n_g where n_g = n - λ(dn/dλ)
- Group velocity dispersion (GVD): β₂ = d²k/dω²
- Worked example: silica fiber dispersion at 1550 nm (β₂ ≈ -21 ps²/km)

#### 3.1.4 Kramers-Kronig Relations
- Causality requires Re[χ] and Im[χ] to be related:
$$n(\omega) - 1 = \frac{2}{\pi}\mathcal{P}\int_0^\infty \frac{\omega'\kappa(\omega')}{\omega'^2 - \omega^2}d\omega'$$
- Physical meaning: you cannot have gain without changing the refractive index
- Silicon modulators: plasma dispersion effect ties Δn to Δα
- Derivation via contour integration in the complex frequency plane

### 3.2 Quantum Picture: Einstein Coefficients

#### 3.2.1 Two-Level System
- Energy levels E₁ (ground) and E₂ (excited), separation ΔE = hν₀
- Photon energy must match: hν = E₂ - E₁

#### 3.2.2 Einstein A and B Coefficients
- Spontaneous emission rate: A₂₁ [s⁻¹]
- Stimulated emission rate: B₂₁ · u(ν) [s⁻¹]
- Absorption rate: B₁₂ · u(ν) [s⁻¹]
- Relation from thermodynamic equilibrium:
  - B₁₂ = B₂₁ (for non-degenerate levels)
  - A₂₁/B₂₁ = 8πhν³/c³
- Spontaneous emission: isotropic, random phase → why it's "noise" in lasers

#### 3.2.3 Population Inversion and Gain
- At thermal equilibrium: N₂/N₁ = e^{-hν/kT} ≪ 1 (mostly ground state)
- Population inversion: N₂ > N₁ → stimulated emission dominates → optical gain
- Gain coefficient g(ν) = σ(ν)(N₂ - N₁) where σ is stimulated emission cross section
- This is the operating principle of every laser used in photonic computing

### 3.3 Nonlinear Optics

#### 3.3.1 Nonlinear Polarization
- For large fields, expand polarization in powers of E:
$$\mathbf{P} = \varepsilon_0\left(\chi^{(1)}\mathbf{E} + \chi^{(2)}\mathbf{E}^2 + \chi^{(3)}\mathbf{E}^3 + \cdots\right)$$
- χ^(1): linear susceptibility (normal optics)
- χ^(2): second-order nonlinearity (only in non-centrosymmetric crystals; zero in Si)
- χ^(3): third-order nonlinearity (present in all materials including Si, SiO₂)

#### 3.3.2 Second-Harmonic Generation (SHG)
- Input: E = E₀cos(ωt) → P^(2) = ε₀χ^(2)E₀²cos²(ωt) = (ε₀χ^(2)E₀²/2)(1 + cos(2ωt))
- DC component: optical rectification
- 2ω component: second-harmonic generation
- Phase matching condition: Δk = k(2ω) - 2k(ω) = 0
- Birefringent phase matching; quasi-phase matching (periodically poled LiNbO₃)
- Applications: frequency doubling, lithium niobate modulators (Pockels effect is χ^(2))

#### 3.3.3 Third-Order Nonlinearities in Silicon Photonics
- Kerr effect: Δn = n₂I, where n₂ (Si) ≈ 4×10⁻¹⁸ m²/W
- Self-phase modulation (SPM): spectrum broadening during propagation
- Cross-phase modulation (XPM): one beam modulates another's phase (useful for all-optical switching)
- Four-wave mixing (FWM): three waves mix to produce a fourth; ω₄ = ω₁ + ω₂ - ω₃
- Two-photon absorption (TPA) in silicon: limiting factor for on-chip nonlinear optics

#### 3.3.4 Optical Solitons
- Interplay of GVD and SPM in anomalous dispersion regime
- Nonlinear Schrödinger equation (NLSE):
$$i\frac{\partial A}{\partial z} - \frac{\beta_2}{2}\frac{\partial^2 A}{\partial t^2} + \gamma|A|^2A = 0$$
- Fundamental soliton: N = 1, exact analytic solution
- Soliton period, stability, self-healing
- Temporal solitons in fiber; spatial solitons in waveguides
- Dissipative Kerr solitons in microresonators (frequency combs)

#### 3.3.5 Stimulated Raman and Brillouin Scattering
- Raman scattering: photon-phonon interaction, frequency shift ~13 THz in Si
- Stimulated Raman amplification in silicon
- Brillouin scattering: lower frequency shift (~10 GHz), electrostrictive; backward propagating
- Brillouin lasers and narrow-linewidth light sources

### 3.4 Exercises

**Mathematical:**
1. Using the Lorentz oscillator model, show that the imaginary part of χ peaks at ω = ω₀ and derive the FWHM linewidth in terms of γ.
2. Derive the Beer-Lambert law from the relation between Im[ñ] and the electric field amplitude.
3. For SHG with phase mismatch Δk, show that the SHG power varies as sinc²(ΔkL/2). Find the coherence length L_c = π/Δk.
4. Solve the NLSE numerically for N = 1 soliton propagation over 10 soliton periods using a split-step Fourier method.
5. Using Kramers-Kronig, estimate the refractive index change associated with a gain coefficient of g = 100 cm⁻¹ at ν₀.

**Conceptual:**
6. Why is χ^(2) = 0 in silicon? What symmetry argument rules it out?
7. Why does two-photon absorption in silicon limit the use of high-intensity pulses for on-chip nonlinear optics?
8. A fiber communication system uses WDM channels spaced 50 GHz apart. Which nonlinear effect is most likely to cause channel crosstalk? How can it be mitigated?

### 3.5 Programming Projects

**Project 3.1: Split-Step Fourier Method for Pulse Propagation** — Implement the NLSE split-step solver. Simulate a Gaussian pulse propagating in a fiber with anomalous GVD. Show spectral broadening from SPM, then adjust initial power to form a soliton. (Python, numpy/scipy)

**Project 3.2: Lorentz Oscillator Dispersion Model** — Plot n(ω) and κ(ω) for a Lorentz oscillator. Verify the Kramers-Kronig relation numerically using FFT. Superimpose multiple resonances to model realistic glass.

**Project 3.3: Four-Wave Mixing Crosstalk Estimator** — For a WDM system with N channels, compute the FWM-generated crosstalk power as a function of channel spacing, fiber nonlinear coefficient γ, and dispersion. Plot the crosstalk map.

### 3.6 Further Reading and Key Researchers
- **Textbooks:** Boyd, *Nonlinear Optics* (4th ed.); Agrawal, *Nonlinear Fiber Optics* (6th ed.)
- **Key Researchers:** Bloembergen (Nobel 1981, nonlinear optics), Stolen & Ippen (fiber nonlinearities), Jalali (silicon nonlinear photonics)
- **Key Papers:** Bloembergen & Pershan (1962), "Light Waves at the Boundary of Nonlinear Media"

---
# PHOTONIC COMPUTING — BOOK OUTLINE (Part 2)
# Units II–IV: Lasers, Guided-Wave Photonics, Communications

---

# UNIT II: THE LASER — ENGINE OF PHOTONIC COMPUTING

> *In 1960, Theodore Maiman fired a flashlamp through a ruby rod and produced the first laser pulse — a concentrated beam of red light more intense than the surface of the sun. Within a decade, lasers were everywhere: in surgery theaters, surveying instruments, CD players, and optical fiber networks. Today, a data center router is threaded with hundreds of laser beams, each carrying terabits of information through glass fibers thinner than a human hair. Every photonic computing system — whether it computes with interference, with diffraction, or with quantum superposition — depends on the laser as its light source. This unit explores how lasers work, what makes them special, and how they are being miniaturized onto silicon chips.*

---

## Chapter 4: Laser Physics

### 4.1 Population Inversion and Gain Media

#### 4.1.1 Why Two-Level Systems Cannot Lase
- At best N₂ = N₁ (transparent medium) in a two-level system under steady-state pumping
- Proof: rate equations show N₂/N₁ → 1 as pump increases, never > 1
- Need three or four levels

#### 4.1.2 Three-Level and Four-Level Systems
- Three-level (ruby): rapid decay from pump level to metastable level; ground state is lower laser level
- Four-level (Nd:YAG, Er:fiber): rapid decay populates upper laser level AND rapidly depopulates lower laser level → easier to achieve inversion
- Rate equations for four-level system:
$$\frac{dN_2}{dt} = R_p - \frac{N_2}{\tau} - \sigma(\nu)\frac{I}{h\nu}(N_2 - N_1)$$
- Steady-state solution for population inversion

#### 4.1.3 Gain Coefficient
- Small-signal gain: g₀ = σ(N₂ - N₁)
- Gain saturation: g = g₀/(1 + I/I_sat)
- Homogeneous vs. inhomogeneous broadening
- Gain bandwidth determines tuning range and mode spacing

### 4.2 Optical Resonators

#### 4.2.1 Longitudinal Modes
- Resonance condition: 2nL = mλ, m integer
- Mode spacing: Δν = c/(2nL) = FSR
- For L = 300 μm (laser diode): FSR ≈ 150 GHz at 1550 nm
- Mode spacing determines single-mode vs. multimode operation

#### 4.2.2 Stability of Optical Resonators
- Stability criterion from ABCD matrix of round trip: -1 ≤ (A+D)/2 ≤ 1
- Stability diagram: plot of g₁g₂ = (1 - L/R₁)(1 - L/R₂)
- Plane-plane, concentric, hemispherical, confocal resonators
- Beam waist and mode volume calculation

#### 4.2.3 Q Factor and Finesse
- Q = ω₀(stored energy)/(power dissipated) = ω₀τ_p where τ_p = photon lifetime
- Finesse F = FSR/Δν_FWHM = π√R/(1-R) for equal-mirror cavity
- High-Q cavities: microsphere resonators Q ~ 10⁸, photonic crystal cavities Q ~ 10⁶

### 4.3 Laser Rate Equations and Output Characteristics

#### 4.3.1 Coupled Rate Equations
$$\frac{dN}{dt} = \frac{I_{pump}}{qV} - \frac{N}{\tau_e} - v_g g S$$
$$\frac{dS}{dt} = \Gamma v_g g S - \frac{S}{\tau_p} + \Gamma \beta \frac{N}{\tau_e}$$
- N = carrier density, S = photon density
- Γ = confinement factor, β = spontaneous emission factor
- Threshold: Γv_g g_th = 1/τ_p
- Above threshold: linear increase of S with pump current

#### 4.3.2 Laser Threshold and Slope Efficiency
- Threshold condition: gain equals total loss
- Below threshold: LED-like emission
- Above threshold: stimulated emission dominates, narrow linewidth
- Slope efficiency η_d = differential quantum efficiency
- L-I (light-current) curve analysis

#### 4.3.3 Mode Locking — Ultrashort Pulse Generation
- Locking N longitudinal modes with fixed relative phases
- Pulse duration: Δt ~ 1/(NΔν) ≈ 1/Δν_gain
- Active mode locking: intra-cavity amplitude modulator at f_rep = c/2L
- Passive mode locking: saturable absorber (fast vs. slow)
- Kerr lens mode locking (Ti:sapphire, <10 fs pulses)
- Mode-locked fiber lasers: repetition rates 100 MHz – 10 GHz
- Applications: optical sampling, photonic ADC, optical frequency combs

#### 4.3.4 Laser Noise
- Schawlow-Townes linewidth: $\Delta\nu_{ST} = \frac{h\nu(\Delta\nu_c)^2}{P_{out}}$ — modified by Henry linewidth enhancement factor α
- Phase noise: spectral density S_φ(f)
- Relative intensity noise (RIN): S_RIN(f) [dBc/Hz]
- Why laser linewidth matters for coherent photonic computing (MZI phase stability)
- Injection locking to reduce linewidth

### 4.4 Laser Types for Photonic Computing

#### 4.4.1 Semiconductor Laser Diodes
- Heterostructure confinement: carrier and optical
- Quantum well, wire, dot active regions
- p-n junction pumping
- Fabry-Pérot laser: multimode, broad linewidth
- Ridge waveguide geometry

#### 4.4.2 Distributed Feedback (DFB) Lasers
- Periodic grating provides wavelength-selective feedback
- Bragg condition: λ_B = 2n_eff Λ
- Single longitudinal mode, narrow linewidth
- Side-mode suppression ratio (SMSR) > 40 dB
- Workhorse of optical communication and photonic computing

#### 4.4.3 Vertical-Cavity Surface-Emitting Lasers (VCSELs)
- Epitaxial DBR mirrors top and bottom
- Emission perpendicular to chip surface
- Low threshold current, circular beam, high-speed modulation
- Dense 2D arrays possible
- Applications: data center interconnects, LIDAR, optical neural networks
- 850 nm GaAs VCSELs: dominant for short-reach interconnects

#### 4.4.4 Heterogeneously Integrated Lasers on Silicon
- III-V bonded to SOI wafer
- Hybrid silicon laser (Bowers group, UCSB/Intel)
- Key challenge: thermal mismatch, coupling efficiency
- Micro-transfer printing of laser bars
- Recent demonstration: wafer-scale integration of InP on silicon

#### 4.4.5 On-Chip Frequency Combs
- Microresonator Kerr combs: equidistant lines from a single CW pump laser
- Dissipative Kerr soliton (DKS) state for low-noise combs
- Soliton microcomb as multi-wavelength light source for WDM computing
- 100+ comb lines from a single pump laser
- Line spacing tunable from ~10 GHz to ~1 THz

### 4.5 Exercises

**Mathematical:**
1. Derive the laser threshold condition from the round-trip gain = round-trip loss requirement. Express in terms of mirror reflectivities R₁, R₂, internal loss α_i, and material gain g.
2. From the laser rate equations, find the steady-state photon density S above threshold as a function of pump rate R_p, showing linear dependence above threshold.
3. A mode-locked laser has gain bandwidth Δν = 10 THz. Estimate the minimum pulse duration. If the cavity round-trip time is 10 ns, how many modes are locked?
4. Calculate the Schawlow-Townes linewidth for a DFB laser with: output power 10 mW, cavity photon lifetime 3 ps, linewidth enhancement factor α_H = 3.
5. A VCSEL has threshold current 1 mA, slope efficiency 0.5 W/A, and series resistance 50 Ω. Plot the L-I-V curve and find the bias point for maximum wall-plug efficiency.

**Conceptual:**
6. Why can't a two-level system achieve population inversion under steady-state pumping? Show with rate equations.
7. A DFB laser has SMSR of 45 dB. Is this sufficient for a coherent QAM-16 optical communications system? What SMSR is typically required?
8. Compare passive mode locking with a fast vs. slow saturable absorber. Which produces shorter pulses and why?

### 4.6 Programming Projects

**Project 4.1: Laser Rate Equation Simulator** — Implement the coupled carrier-photon rate equations for a laser diode. Simulate: (a) L-I curve, (b) turn-on transient with relaxation oscillations, (c) small-signal modulation response H(f) to find the -3 dB bandwidth. Compare to experimental data for a commercial DFB laser.

**Project 4.2: Mode-Locked Laser Pulse Shaper** — Simulate active mode locking by propagating an intracavity field through gain, loss, and a sinusoidal amplitude modulator in a round-trip loop. Watch pulses emerge from noise. Study the effect of modulation depth and frequency on pulse width.

**Project 4.3: Microresonator Kerr Comb Simulator** — Implement the Lugiato-Lefever equation (LLE) on a ring resonator to simulate comb formation. Scan the pump detuning and observe the transition from modulation instability to chaotic to soliton states. Plot the comb spectrum.

**Project 4.4: Laser Noise Analyzer** — Simulate laser phase noise using the Langevin rate equations with stochastic terms. Compute the power spectral density of the intensity and phase noise. Verify the Lorentzian lineshape.

### 4.7 Further Reading and Key Researchers
- **Textbooks:** Saleh & Teich, *Fundamentals of Photonics* Ch. 14-16; Coldren, Corzine & Mašanović, *Diode Lasers and Photonic Integrated Circuits*
- **Key Researchers:** Maiman (first laser), Kroemer & Alferov (Nobel 2000, heterostructure lasers), Bowers (silicon photonics lasers), Kippenberg (microcombs)
- **Key Papers:** Hall et al. (1962) and Nathan et al. (1962), semiconductor laser demonstrations; Kippenberg et al. (2018), "Dissipative Kerr solitons," *Science*

---

## Chapter 5: Photodetectors and Optical Receivers

### 5.1 Photodetection Fundamentals

#### 5.1.1 Photoelectric Effect and Quantum Efficiency
- Photon absorbed → electron-hole pair generated (if hν > E_gap)
- Internal quantum efficiency η_i: fraction of absorbed photons producing carriers
- External quantum efficiency η_ext: includes surface reflection losses
- Responsivity: R = ηq/hν [A/W]
- Worked example: ideal Ge detector at 1550 nm, R_max = 1.25 A/W

#### 5.1.2 Bandwidth and Speed
- Drift of carriers across depletion region: transit time limit τ_tr = W/v_sat
- RC time constant limit: τ_RC = R_L C
- Bandwidth: f_3dB ≈ 1/(2π·max(τ_tr, τ_RC))
- Trade-off: wider depletion region W → higher responsivity but slower response

### 5.2 Detector Types

#### 5.2.1 p-i-n Photodiode
- Wide intrinsic (i) region: most absorption occurs here, full depletion
- Electric field across i-region sweeps carriers quickly
- Bias voltage, dark current, responsivity spectrum
- Silicon p-i-n: λ < 1100 nm; Ge p-i-n: λ < 1600 nm

#### 5.2.2 Avalanche Photodiodes (APDs)
- Reverse bias high enough for impact ionization → carrier multiplication
- Multiplication gain M = I_ph/(I_primary)
- Excess noise factor F(M) = kM + (2-1/M)(1-k), k = ionization ratio
- Gain-bandwidth product: M · f_3dB = constant
- Applications: long-haul coherent receivers, LIDAR

#### 5.2.3 Single-Photon Avalanche Diodes (SPADs)
- Biased above breakdown: Geiger mode
- Single photon triggers avalanche
- Quenching circuit resets diode
- Figures of merit: dark count rate, timing jitter, dead time, photon detection efficiency
- CMOS-compatible SPADs: arrays for LiDAR, quantum random number generators

#### 5.2.4 Superconducting Nanowire Single-Photon Detectors (SNSPDs)
- Niobium nitride (NbN) nanowire at cryogenic temperature (2-4 K)
- Photon breaks a Cooper pair → local hot spot → resistance → voltage pulse
- Detection efficiency > 90%, timing jitter < 20 ps, dark count rate < 1 Hz
- Essential for: quantum photonic computing, quantum key distribution, deep-space optical communication

#### 5.2.5 Germanium-on-Silicon Photodetectors
- CMOS-compatible, covers 1310 nm and 1550 nm telecom bands
- Selective epitaxial growth of Ge on Si
- Bandwidth > 50 GHz demonstrated
- Integrated with ring modulators and waveguides in silicon photonics foundry

### 5.3 Noise in Photodetection

#### 5.3.1 Shot Noise
- Photon arrivals are Poisson-distributed
- Shot noise current: $\langle i_{shot}^2\rangle = 2qI_{ph}B$ where B = bandwidth
- Originates from quantum nature of light — cannot be eliminated
- Represents the fundamental limit of photodetection

#### 5.3.2 Thermal (Johnson) Noise
- $\langle i_{thermal}^2\rangle = 4k_BT B/R_L$
- Dominant noise at low signal levels for direct detection systems
- Reduced by: cooling, high-impedance load (at cost of bandwidth), avalanche gain

#### 5.3.3 Signal-to-Noise Ratio and Sensitivity
- SNR = I_ph²/(⟨i_shot²⟩ + ⟨i_thermal²⟩)
- Shot-noise limited: SNR = ηP_opt/(2hνB) = number of photons per bit / 2
- Sensitivity: minimum detectable optical power for BER = 10⁻⁹
- Typical direct detection sensitivity: -20 to -30 dBm
- Coherent detection sensitivity: -40 to -50 dBm (approaches shot-noise limit)

#### 5.3.4 Noise in Photonic Computing
- Photonic matrix multipliers: output signal corrupted by shot noise and thermal noise of detectors
- Signal-to-noise ratio determines the effective bit precision of analog optical computation
- Key challenge: achieving >8-bit effective resolution in photonic analog processors

### 5.4 Exercises

**Mathematical:**
1. A Ge photodiode has quantum efficiency η = 0.8 at 1550 nm. Calculate its responsivity. If it receives 1 μW of optical power, what is the photocurrent?
2. Derive the shot-noise limited SNR for a direct detection system. Show that it equals ηN_ph/2 where N_ph is the number of photons per bit period.
3. An APD has ionization ratio k = 0.3 and multiplication M = 10. Calculate the excess noise factor. Compare the SNR with an ideal p-i-n plus low-noise amplifier.
4. A transimpedance amplifier has noise current density 5 pA/√Hz and input capacitance 100 fF. For bit rate 25 Gbps, find the minimum sensitivity (BER=10⁻¹²).

### 5.5 Programming Projects

**Project 5.1: Optical Receiver Sensitivity Calculator** — Build a complete optical receiver model (detector + TIA + decision circuit). Compute sensitivity as a function of bit rate for OOK, PAM-4, and coherent QPSK. Plot the required received power vs. data rate.

**Project 5.2: SPAD Array Simulator** — Model a 16×16 SPAD array for photon counting imaging. Add dark counts, afterpulsing, and cross-talk. Reconstruct images from photon arrival timestamps. Compare with CCD/CMOS direct detection.

**Project 5.3: Photonic Analog Computation Noise Analysis** — Model the noise chain in a photonic matrix multiplier (laser RIN → modulator noise → waveguide loss → detector shot noise → TIA). Calculate the effective number of bits (ENOB) as a function of optical power and detector bandwidth.

### 5.6 Further Reading and Key Researchers
- **Textbooks:** Saleh & Teich, *Fundamentals of Photonics* Ch. 18; Agrawal, *Fiber-Optic Communication Systems* Ch. 4
- **Key Researchers:** Helms, Eisenstein (APDs); Natarajan & Hadfield (SNSPDs); Kash & Michel (Ge-on-Si detectors)
- **Key Papers:** Marsili et al. (2013), "Detecting single infrared photons with 93% system efficiency," *Nature Photonics*

---

# UNIT III: GUIDED WAVE PHOTONICS AND INTEGRATED CIRCUITS

> *An optical fiber is perhaps the most improbable information conduit ever devised: a thread of glass, purer than anything found in nature, guiding light through continent-spanning distances with losses measured in tenths of a decibel per kilometer. Inside a photonic chip, light is herded through silicon waveguides thinner than a virus, routed around corners, split into dozens of identical copies, and interfered with nanometer precision. This unit is about the science and engineering of guided light — from the towering mathematics of electromagnetic modes to the pragmatic realities of silicon foundry processes. By the end, you will be able to design, simulate, and characterize the fundamental building blocks of every photonic computing system being built today.*

---

## Chapter 6: Optical Waveguides and Fiber Optics

### 6.1 Planar Waveguide Theory

#### 6.1.1 Ray Optic Model: Total Internal Reflection
- Three-layer structure: core (n₁), cladding (n₂ < n₁)
- Guided ray bounces at critical angle: θ_c = arcsin(n₂/n₁)
- Numerical aperture: NA = √(n₁² - n₂²) = n₀ sin θ_acceptance
- Self-consistency condition for guided modes: ray must interfere constructively after one round trip

#### 6.1.2 Wave Optic Model: TE and TM Modes
- Maxwell's equations in layered geometry
- TE modes: E_y component; TM modes: H_y component
- TE eigenvalue equation:
$$\tan(k_x d/2) = \frac{\gamma}{k_x}$$ (symmetric modes)
- Parameters: k_x = transverse wavenumber in core; γ = evanescent decay in cladding
- Dispersion relation: k_x² + β² = k₀²n₁², γ² = β² - k₀²n₂²
- Normalized frequency: V = k₀ d/2 · √(n₁² - n₂²) = k₀ d/2 · NA
- Cutoff condition: V = mπ/2 for mode m
- Single-mode condition: V < π/2

#### 6.1.3 Mode Fields and Confinement Factor
- Transcendental eigenvalue equation solved graphically or numerically
- Mode field profile: cosine in core, exponential decay in cladding
- Confinement factor Γ: fraction of power in core
- Evanescent field: basis of optical coupling and evanescent wave sensors

### 6.2 Optical Fiber

#### 6.2.1 Step-Index and Graded-Index Fiber
- Step-index: abrupt core-cladding boundary
- Graded-index: parabolic refractive index profile n(r) = n₁√(1 - 2Δ(r/a)²)
- Graded-index: all rays have approximately equal travel time → less modal dispersion
- Optimal profile: α = 2 for minimum modal dispersion

#### 6.2.2 Single-Mode Fiber (SMF)
- Condition: V < 2.405 (first zero of J₀ Bessel function)
- SMF-28: a = 4.1 μm, n_core = 1.452, n_clad = 1.447 at 1550 nm, V ≈ 2.1
- Mode field diameter (MFD) ≈ 10 μm
- No modal dispersion: used for all long-haul communication and coherent photonic computing

#### 6.2.3 Fiber Attenuation
- Absorption: OH-ion overtone at 1380 nm; Rayleigh tail extends into infrared
- Rayleigh scattering: α_R ∝ λ⁻⁴ (density fluctuations frozen in during fiber drawing)
- Minimum loss: 0.2 dB/km at 1550 nm (Corning SMF-28)
- Worked example: power budget for 100 km link at 0.2 dB/km

#### 6.2.4 Chromatic Dispersion
- Material dispersion: dn/dλ varies with wavelength
- Waveguide dispersion: β₂ depends on confinement
- Total dispersion parameter D = D_M + D_W [ps/(nm·km)]
- Zero-dispersion wavelength λ_ZD: 1310 nm for SMF-28
- At 1550 nm: D ≈ +17 ps/(nm·km) (anomalous dispersion)
- Pulse broadening: δτ = D · L · Δλ

#### 6.2.5 Erbium-Doped Fiber Amplifiers (EDFAs)
- Er³⁺ ions in silica: gain band 1530-1565 nm (C-band)
- Population inversion with 980 nm or 1480 nm pump laser
- Small-signal gain G = exp(gL) can reach 30-40 dB
- Noise figure: F = 2n_sp (minimum 3 dB for fully inverted amplifier)
- Gain flattening with gain-equalizing filters
- Multi-stage EDFA with mid-stage access for dispersion compensation

### 6.3 Nonlinear Propagation Effects

#### 6.3.1 The Generalized Nonlinear Schrödinger Equation
$$\frac{\partial A}{\partial z} + \frac{\alpha}{2}A + \frac{i\beta_2}{2}\frac{\partial^2 A}{\partial T^2} - \frac{\beta_3}{6}\frac{\partial^3 A}{\partial T^3} = i\gamma\left(|A|^2A + \frac{i}{\omega_0}\frac{\partial(|A|^2 A)}{\partial T} - T_R A\frac{\partial|A|^2}{\partial T}\right)$$
- Each term: loss, GVD, TOD, SPM, self-steepening, Raman

#### 6.3.2 Optical Solitons in Fiber
- Fundamental soliton: N = 1, where $N^2 = \gamma P_0 T_0^2/|\beta_2|$
- Higher-order solitons: periodic evolution
- Soliton fission, supercontinuum generation
- Worked example: 1 pJ, 1 ps pulse in SMF → N calculation

#### 6.3.3 Stimulated Brillouin Scattering Threshold
- Threshold power: P_th ≈ 21 A_eff / (g_B · L_eff)
- g_B(SiO₂) ≈ 2×10⁻¹¹ m/W, Brillouin gain bandwidth ~30 MHz
- SBS limits transmitted power in CW fiber links
- Mitigation: phase dithering, spectrally broadening the laser

### 6.4 Exercises

**Mathematical:**
1. Derive the TE eigenvalue equation for a symmetric planar waveguide of half-width d/2. Find the graphical intersection method for the first two modes.
2. A single-mode fiber with n_core = 1.452, n_clad = 1.447, core radius a = 4.1 μm. Calculate V at 1550 nm and confirm single-mode operation.
3. A pulse with Δλ = 0.1 nm propagates through 80 km of SMF-28 (D = 17 ps/nm/km). Calculate the output pulse broadening. At what bit rate does ISI become significant?
4. Calculate the EDFA gain G for a 10 m fiber with peak gain coefficient g = 5 dB/m, assuming complete inversion and negligible background loss.

**Conceptual:**
5. Why does graded-index multimode fiber have less modal dispersion than step-index? Explain physically using ray paths.
6. Why is 1550 nm chosen as the primary telecom wavelength? What would change if we used 1310 nm?

### 6.5 Programming Projects

**Project 6.1: Mode Solver for Planar Waveguide** — Implement a graphical and numerical solver for TE and TM modes of a symmetric planar waveguide. Plot mode field profiles, effective indices, and confinement factors vs. V number. Extend to asymmetric waveguides.

**Project 6.2: Optical Fiber Pulse Propagation** — Implement the split-step Fourier method for the NLSE with loss, GVD, and SPM. Simulate: (a) linear pulse broadening, (b) SPM-induced spectral broadening, (c) soliton propagation, (d) supercontinuum generation with higher-order effects.

**Project 6.3: EDFA Gain Spectrum Model** — Implement a numerical model of an EDFA using the two-level rate equations along the fiber length. Compute gain and noise figure vs. input signal power. Add a gain-flattening filter and recompute the WDM channel-equalized gain spectrum.

### 6.6 Further Reading and Key Researchers
- **Textbooks:** Agrawal, *Nonlinear Fiber Optics* (6th ed.); Saleh & Teich *Fundamentals of Photonics* Ch. 8-9; Okoshi & Kikuchi, *Coherent Optical Fiber Communications*
- **Key Researchers:** Kao (Nobel 2009, fiber optics), Mears et al. (EDFA), Mollenauer (fiber solitons)
- **Key Papers:** Kao & Hockham (1966), "Dielectric-fibre surface waveguides for optical frequencies"; Mears et al. (1987), "Low-noise erbium-doped fibre amplifier"

---

## Chapter 7: Silicon Photonics and Photonic Integrated Circuits

### 7.1 The Silicon Photonics Platform

#### 7.1.1 Why Silicon?
- CMOS manufacturing infrastructure: nm-precision lithography, high yield
- Silicon-on-insulator (SOI): Si core (n=3.47), SiO₂ box (n=1.44), 400 nm standard thickness
- High index contrast → tight mode confinement → small bending radii (5-10 μm)
- Disadvantages: indirect bandgap (no native laser), two-photon absorption above 0.5 W power
- Transparent window: 1.1–7 μm

#### 7.1.2 The Ecosystem
- Foundries: IMEC (Belgium), AMF (Singapore), AIM Photonics (USA), GlobalFoundries, TSMC
- Process design kits (PDKs): standardized component libraries
- Multi-project wafer (MPW) runs: shared cost prototyping (~\$20k for a chip)
- Open-source PDKs: SiEPIC-EBeam, IHP SG25H, AIM Photonics

### 7.2 Passive Components

#### 7.2.1 Strip Waveguide Design
- 450 nm wide, 220 nm tall Si strip on SiO₂
- Mode solving: numerical (FEM, FDTD) and semi-analytical
- Propagation loss: 2-3 dB/cm (sidewall roughness limited)
- Bending loss vs. radius trade-off
- Fundamental TE and TM modes; birefringence

#### 7.2.2 Directional Couplers
- Two parallel waveguides separated by gap g
- Coupled mode theory: power oscillates between guides
$$\frac{dA_1}{dz} = -i\kappa A_2, \quad \frac{dA_2}{dz} = -i\kappa A_1$$
- Solution: A₁(z) = cos(κz), A₂(z) = -i sin(κz)
- Coupling length L_c = π/(2κ)
- Power splitting ratio: sin²(κL)
- Fabrication sensitivity: κ depends exponentially on gap

#### 7.2.3 Multimode Interference (MMI) Couplers
- Self-imaging principle in a wide multimode waveguide
- 50/50 and 1×N splitters
- More fabrication tolerant than directional couplers
- Used in Mach-Zehnder modulators

#### 7.2.4 Mach-Zehnder Interferometer
- Two MMI couplers connected by two arms of length L₁, L₂
- Transfer function: T = cos²(Δφ/2), where Δφ = (2π/λ)Δ(nL)
- Phase tuning: thermo-optic (ΔT→Δn), electro-optic (V→Δn)
- Used as: modulator, switch, beamsplitter, unitary gate in ONN

#### 7.2.5 Microring Resonators
- Ring of circumference 2πR coupled to bus waveguide
- Resonance condition: 2πRn_eff = mλ
- Transfer function (all-pass ring):
$$T = \frac{a^2 - 2ar\cos\phi + r^2}{1 - 2ar\cos\phi + (ar)^2}$$
- At resonance: T_min = ((a-r)/(1-ar))² where a = round-trip field transmission, r = coupling coefficient
- Critical coupling: a = r → T_min = 0 (complete extinction)
- Add-drop ring: four-port device for wavelength routing
- Thermal tuning: heater above ring, ~10 mW for one FSR

#### 7.2.6 Arrayed Waveguide Gratings (AWGs)
- N input waveguides → free propagation region → N×M array of curved guides → free propagation region → M output waveguides
- Path length difference between adjacent array waveguides: ΔL
- Phase progression creates angularly dispersed output
- Wavelength demultiplexing with channel spacing Δλ = λ²/(n_g · N_arr · ΔL)
- Loss uniformity, crosstalk, thermal stability
- Applications: WDM transceivers, photonic computing weight banks

### 7.3 Active Components: Modulators

#### 7.3.1 The Plasma Dispersion Effect in Silicon
- Free carriers (electrons and holes) change refractive index and absorption
- Soref & Bennett (1987) empirical relations at 1550 nm:
$$\Delta n = -[8.8\times10^{-22}\Delta N_e + 8.5\times10^{-18}\Delta N_h^{0.8}]$$
$$\Delta\alpha = 8.5\times10^{-18}\Delta N_e + 6.0\times10^{-18}\Delta N_h$$
- Carrier injection (PN forward bias): large Δn but slow (ns)
- Carrier depletion (PN reverse bias): smaller Δn but fast (ps, GHz-scale)
- Carrier accumulation (MOS): fast, low loss, but requires oxide

#### 7.3.2 MZI Modulator
- Reverse-biased PN junction in one or both arms
- Applied voltage V → carrier depletion → Δn → phase shift Δφ
- V_π·L figure of merit: drive voltage for π phase shift (extinction)
- Silicon MZI modulator: V_π·L ≈ 3-4 V·cm
- Modulation bandwidth: limited by RC (junction capacitance × load resistance)
- Segmented design, traveling-wave electrode for high bandwidth (>50 GHz)

#### 7.3.3 Lithium Niobate Modulators
- LiNbO₃: large Pockels coefficient r₃₃ = 30 pm/V (χ^(2) material)
- V_π·L ≈ 2 V·cm for X-cut LiNbO₃
- Bandwidth > 100 GHz (no carrier effects, purely electro-optic)
- Thin-film LiNbO₃ on insulator (LNOI): waveguide confinement + low V_π
- Dominant modulator for next-generation coherent transceivers and photonic processors

#### 7.3.4 Microring Modulators
- Reverse-biased PN ring: compact, low capacitance, >50 GHz bandwidth
- Drive voltage ~2 V for 10 dB extinction
- Wavelength sensitive: requires thermal stabilization
- WDM-native: each ring selects its own channel
- Dense arrays: 4, 8, 16 channels on single bus waveguide

### 7.4 Active Components: Phase-Change Switches

#### 7.4.1 Thermo-Optic Switches
- Silicon dn/dT ≈ 1.8×10⁻⁴ K⁻¹
- Metal heater above waveguide: heat diffuses into core
- Switching power: ~10-20 mW for π phase shift (MZI switch)
- Switching time: ~10 μs (thermal time constant)
- Nonvolatile? No — latches require holding power
- Applications: slow reconfiguration of photonic processor weights

#### 7.4.2 Phase-Change Materials: Optical Memory
- GST (Ge₂Sb₂Te₅): amorphous (n=4.0, lossy) ↔ crystalline (n=7.0, higher loss)
- GSST (Ge₂Sb₂Se₄Te₁): lower loss in both states, better contrast
- Optical switching with nanosecond laser pulses
- Nonvolatile: state retained without power → synaptic weight storage
- Multi-level storage: partial crystallization for analog weights
- Key research groups: Bhaskaran (Oxford), Pernice (Münster), Wuttig (Aachen)

#### 7.4.3 MEMS-Based Switches
- Electrostatic actuation moves a waveguide section into/out of evanescent coupling
- Very low on-state insertion loss (< 0.5 dB)
- High extinction ratio (> 30 dB)
- Slower than EO (μs range) but nonvolatile when latched
- Integration with silicon photonics: demonstrated at foundry level

### 7.5 Photonic Platforms

#### 7.5.1 Silicon Nitride (Si₃N₄)
- n = 2.0 at 1550 nm, lower than silicon → less tight confinement
- Ultra-low propagation loss: < 0.1 dB/m demonstrated
- No two-photon absorption at 1550 nm → can handle high power
- Transparent to visible wavelengths (400 nm – 5 μm)
- No electro-optic effect, but excellent for passive circuits and frequency combs
- Anomalous GVD engineering for broadband Kerr combs

#### 7.5.2 Lithium Niobate on Insulator (LNOI)
- Thin-film (~600 nm) LiNbO₃ on SiO₂
- Combines tight confinement with strong Pockels effect
- V_π·L < 2 V·cm with > 100 GHz bandwidth
- Efficient SHG and other χ^(2) processes
- Start-ups: HyperLight, Poet Technologies

#### 7.5.3 Indium Phosphide (InP)
- Native laser gain: InGaAsP quantum wells cover 1300-1600 nm
- Full integration: laser, modulator, SOA, detector on one chip
- Higher cost and lower yield than Si/SiN
- Dominant platform for coherent pluggable transceivers (400G, 800G ZR)
- Research platform for on-chip neuromorphic lasers

### 7.6 Exercises

**Mathematical:**
1. Using coupled-mode theory, derive the complete transmission matrix of a directional coupler with coupling coefficient κ and length L. For what L is the splitting ratio exactly 50/50?
2. A microring resonator has R = 5 μm, n_eff = 2.5, group index n_g = 4.2, and round-trip loss α = 3 dB/cm. Calculate the FSR, finesse, and loaded Q. Find the coupling coefficient r for critical coupling.
3. A silicon MZI modulator has V_π = 5 V and length L = 3 mm. The PN junction capacitance is 100 fF/mm. With 50 Ω termination, find the 3-dB EO bandwidth.
4. In a WDM system with 8 channels using a microring weight bank (FSR = 800 GHz at 1550 nm), calculate the required ring radii for channels spaced 100 GHz apart.

**Conceptual:**
5. What are the three main contributions to propagation loss in silicon photonic waveguides? How does each scale with wavelength?
6. Compare the MZI modulator and microring modulator for use in a WDM photonic neural network weight bank. What are the trade-offs?
7. Why is Si₃N₄ preferred over Si for microresonator frequency combs? What limits the maximum achievable comb span?

### 7.7 Programming Projects

**Project 7.1: Silicon Waveguide Mode Solver** — Implement a 2D FEM mode solver (or use Meep/MPB) for a 450×220 nm Si strip waveguide. Plot the TE₀ and TM₀ mode fields, effective indices, and group indices as a function of waveguide width. Identify the single-mode width range.

**Project 7.2: Directional Coupler Designer** — Using coupled-mode theory, design a 50/50 directional coupler in silicon for 1550 nm. Using Meep FDTD, simulate the coupler and measure the splitting ratio as a function of gap and length. Compare to analytical prediction.

**Project 7.3: Microring Resonator Characterization** — Simulate a silicon microring using the transfer matrix method. Plot the transmission spectrum. Fit the simulated data to extract Q, FSR, finesse, and coupling coefficient. Design a critically coupled ring filter.

**Project 7.4: Silicon Photonic Layout in gdsfactory** — Using the gdsfactory Python library and the SiEPIC-EBeam PDK, design a complete test chip including: straight waveguides, grating couplers, microring resonator, and a 1×8 MMI-based splitter. Generate the GDS file for fabrication.

**Project 7.5: MZI-Based Photonic Processor Cell** — Design and simulate a single MZI cell for use in a photonic neural network. Calculate the transmission matrix for a given phase setting. Sweep the phase error distribution (σ = 0.01 rad) and plot the resulting transmission matrix error statistics.

### 7.8 Further Reading and Key Researchers
- **Textbooks:** Reed & Knights, *Silicon Photonics: An Introduction*; Chrostowski & Hochberg, *Silicon Photonics Design* (free PDF available)
- **Key Researchers:** Soref (silicon photonics pioneer), Lipson (microring resonators, Columbia), Bowers (silicon photonics integration, UCSB), Watts (silicon photonics, MIT), Atabaki/Agarwal (3D integrated photonics, MIT)
- **Key Papers:** Soref & Bennett (1987), "Electrooptical effects in silicon"; Xu et al. (2005), "Micrometre-scale integrated silicon ring modulator," *Nature*; Reed et al. (2010), "Silicon optical modulators," *Nature Photonics*

---

## Chapter 8: Photonic Crystals, Metamaterials, and Nanophotonic Structures

### 8.1 Photonic Crystals

#### 8.1.1 Bragg Reflector — 1D Photonic Crystal
- Alternating layers of high and low refractive index, period Λ
- Bragg condition: 2Λ(n_H d_H + n_L d_L) = mλ → stop band (photonic band gap)
- Reflectance spectrum: band gap width ∝ |n_H - n_L|/(n_H + n_L)
- Transfer matrix method for arbitrary multilayer stack
- Applications: VCSEL mirrors, laser end mirrors, high-Q resonator mirrors

#### 8.1.2 Bloch Theorem for Photonic Crystals
- Periodic dielectric: ε(r) = ε(r + R) for lattice vector R
- Bloch modes: H_k(r) = u_k(r) e^{ik·r}
- Photonic band structure: ωₙ(k), photonic Brillouin zone
- Band gap: frequency range with no propagating modes
- Analogy with electronic band structure (but photons, not electrons)

#### 8.1.3 2D Photonic Crystal Slabs
- Holes etched in a silicon slab: triangular lattice, hexagonal unit cell
- TE-like and TM-like modes (parity with respect to mid-plane)
- Complete photonic band gap for TE-like modes in triangular lattice
- Photonic crystal waveguides: line defect guides light in the gap
- Photonic crystal cavities: point defect → localized mode, ultra-high Q

#### 8.1.4 Slow Light in Photonic Crystals
- Near band edge: group velocity v_g = dω/dk → 0
- Slow light factor S = c/v_g up to 1000× demonstrated
- Enhanced nonlinear interactions in slow-light waveguides
- Applications: delay lines, enhanced modulators, nonlinear photonic computing

### 8.2 Metasurfaces and Flat Optics

#### 8.2.1 Metasurface Concept
- Array of subwavelength resonators (nanoantennas) on a surface
- Each resonator imparts a local phase and amplitude to the transmitted/reflected field
- Total phase coverage 0 to 2π achievable
- Effective medium description breaks down → full-wave simulation needed

#### 8.2.2 Pancharatnam-Berry Phase Metasurfaces
- Rotate resonator angle to accumulate geometric phase: φ = 2θ for circular polarization
- Phase depends only on geometry, not wavelength → broadband
- Achromatic metalens: compensate chromatic phase with resonance tuning

#### 8.2.3 Applications in Photonic Computing
- Ultra-compact lenses for beam steering and free-space ONN
- Metasurface holograms: weight matrices encoded in diffraction patterns
- Reconfigurable metasurfaces: liquid crystal tuning, MEMS tuning
- Diffractive deep neural network (D2NN) layers (see Chapter 14)

### 8.3 Plasmonics

#### 8.3.1 Surface Plasmon Polaritons
- Collective oscillation of conduction electrons coupled to EM field at metal-dielectric interface
- Dispersion relation: $k_{SP} = \frac{\omega}{c}\sqrt{\frac{\varepsilon_m \varepsilon_d}{\varepsilon_m + \varepsilon_d}}$
- k_SP > k₀ → confined mode below diffraction limit
- Propagation length limited by metal loss (ohmic heating)
- Gold SPPs at 1550 nm: propagation length ~100 μm

#### 8.3.2 Subwavelength Confinement and the Opportunity for Computing
- Mode area far below λ²/4: enhanced light-matter interaction
- Plasmonic modulators: ultra-compact, high-speed (>100 GHz)
- Plasmonic photodetectors: subwavelength-integrated on-chip
- Challenge: high loss limits cascadability
- Hybrid plasmonic-dielectric modes: balance confinement and loss

### 8.4 Exercises

1. Calculate the stop-band center wavelength and width of a Si/SiO₂ DBR mirror stack with period Λ = 235 nm and 10 periods. Verify with transfer matrix computation.
2. Using the Bloch theorem, explain why a point defect in a photonic crystal creates a localized mode inside the band gap.
3. A plasmonic MZI modulator has active length 1 μm and requires Δφ = π for switching. The plasmonic mode has effective index n_eff = 2.0. What electro-optic coefficient r (pm/V) is needed with a 1 V drive?

### 8.5 Programming Projects

**Project 8.1: Photonic Band Structure Calculator** — Using MPB (MIT Photonic Bands), compute the TE photonic band structure of a 2D triangular-lattice photonic crystal slab (r/a = 0.3). Identify the band gap. Introduce a line defect and compute the defect waveguide band.

**Project 8.2: Metasurface Lens Design** — Design a transmissive metasurface lens targeting f = 500 μm at λ = 1550 nm. Assign phase profile φ(x,y) = -k₀√(x² + y² + f²). Simulate the far field using a scalar diffraction integral. Compute focusing efficiency.

**Project 8.3: Plasmonic Mode Calculator** — Compute the SPP dispersion relation for a gold-air interface. Plot the mode field profile at 1550 nm. Compare the confinement and propagation length for Au, Ag, and Al at visible and infrared wavelengths.

### 8.6 Further Reading and Key Researchers
- **Textbooks:** Joannopoulos et al., *Photonic Crystals: Molding the Flow of Light* (free PDF); Novotny & Hecht, *Principles of Nano-Optics*; Yu & Capasso, "Flat optics with designer metasurfaces," *Nature Materials* (2014)
- **Key Researchers:** Joannopoulos (photonic crystals, MIT), Vučković (photonic crystal cavities, Stanford), Capasso (metasurfaces, Harvard), Atwater (plasmonics, Caltech)

---

# UNIT IV: INFORMATION THEORY AND OPTICAL COMMUNICATIONS

> *Claude Shannon sat at Bell Labs in the late 1940s and asked a question so simple it seems obvious in retrospect: how much information can a channel carry? The answer he found — the channel capacity theorem — became the foundational theorem of the digital age. Today, optical fiber systems operate within a few decibels of the Shannon limit, carrying petabits of information around the world every second. But information theory is not just the province of communication engineers. It underpins every photonic computing system: how many distinguishable states can an analog photonic processor represent? What is the minimum energy per operation? How much noise can a photonic neural network tolerate before its outputs become meaningless? This unit answers those questions, first from the communications perspective, then from the computing perspective.*

---

## Chapter 9: Information Theory and Optical Modulation

### 9.1 Shannon's Information Theory

#### 9.1.1 Entropy and Information Content
- Self-information: I(x) = -log₂ P(x) [bits]
- Shannon entropy: H(X) = -Σ P(x) log₂ P(x)
- H is maximized by uniform distribution
- Conditional entropy, mutual information I(X;Y) = H(X) - H(X|Y)
- Worked example: capacity of binary symmetric channel

#### 9.1.2 Channel Capacity and Shannon-Hartley Theorem
- Additive white Gaussian noise (AWGN) channel
- Channel capacity: C = B log₂(1 + SNR) [bits/s]
- B = bandwidth [Hz], SNR = signal power / noise power
- Shannon limit: minimum Eb/N₀ for reliable communication
- Optical channel capacity: generalization to Poisson photon statistics

#### 9.1.3 Capacity of the Optical Fiber
- Optical SNR (OSNR) limited by amplifier noise
- Nonlinear Shannon limit: capacity peaks then decreases with power (nonlinear noise)
- Capacity crunch: forecast of global traffic exceeding installed fiber capacity
- Spatial division multiplexing (SDM): few-mode and multicore fiber as next frontier

### 9.2 Optical Modulation Formats

#### 9.2.1 On-Off Keying (OOK) and PAM-4
- OOK: 1 bit/symbol, direct detection
- PAM-4: 4 amplitude levels, 2 bits/symbol, direct detection
- Symbol diagram, minimum distance, BER formula
- PAM-4 sensitivity penalty vs. OOK: ~4.7 dB

#### 9.2.2 Coherent Modulation: BPSK, QPSK, QAM
- Complex constellation: I+Q plane
- BPSK: 2 points on I-axis, 1 bit/symbol
- QPSK: 4 points at ±45°, 2 bits/symbol
- 16-QAM, 64-QAM, 256-QAM: 4, 6, 8 bits/symbol
- BER as function of Eb/N₀ for each format
- Coherent receiver: local oscillator laser, 90° hybrid, balanced photodiodes, DSP

#### 9.2.3 Wavelength Division Multiplexing (WDM)
- DWDM: 50 GHz or 12.5 GHz channel spacing (ITU C-band grid)
- C-band: 1530-1565 nm, 80+ channels at 50 GHz spacing
- L-band: 1565-1625 nm (extended capacity)
- WDM system capacity: N_ch × B_ch × b/s_per_channel
- Current record: >10 Pb/s over single fiber
- Worked example: 80-channel DWDM at 400 Gbps/channel = 32 Tbps total

### 9.3 Optical Amplifiers

#### 9.3.1 EDFA Architecture
- Pre-amplifier: noise-limited (low signal)
- Booster: power-limited (high signal)
- In-line amplifier: gain equals span loss (transparent link)
- Gain tilt and gain-flattening filters

#### 9.3.2 Raman Amplification
- Stimulated Raman scattering provides gain to signal downshifted by ~13 THz
- Distributed Raman amplification: gain in the transmission fiber itself
- Lower noise figure than EDFA
- Backward pump configuration for noise performance

#### 9.3.3 Semiconductor Optical Amplifiers (SOAs)
- Compact, on-chip, broadband gain
- High nonlinear gain saturation: useful for wavelength conversion, optical gates
- High noise figure and polarization sensitivity
- Applications: on-chip photonic switching and computing

### 9.4 Forward Error Correction for Optical Systems

#### 9.4.1 FEC Principles
- Add redundant bits to detect and correct errors
- Code rate R = k/n where k = information bits, n = total bits
- Coding gain: reduction in required OSNR for target BER
- Hard-decision vs. soft-decision FEC

#### 9.4.2 Codes Used in Optical Communications
- Reed-Solomon (RS): widely used in first-generation FEC
- LDPC (Low-Density Parity Check): near-Shannon-limit performance, used in 400G/800G systems
- Polar codes: theoretically capacity-achieving
- Turbo codes: iterative decoding

#### 9.4.3 FEC for Photonic Computing
- Photonic analog processors produce noisy outputs
- Hybrid approach: photonic compute + digital FEC
- Quantifying required SNR for target compute accuracy

### 9.5 Exercises

**Mathematical:**
1. A WDM system uses 80 channels, each carrying coherent 64-QAM at symbol rate 100 GBaud with 16% LDPC overhead. Calculate (a) net bit rate per channel, (b) total system capacity, (c) required OSNR for BER = 10⁻³ pre-FEC.
2. Using the AWGN channel capacity formula, find the spectral efficiency [bits/s/Hz] achievable with SNR = 20 dB. How many QAM levels approximately match this?
3. Prove that soft-decision FEC achieves ~2 dB better coding gain than hard-decision FEC for AWGN channels.
4. A photonic matrix multiplier has optical SNR of 30 dB per output. What is the maximum effective number of bits (ENOB) of precision that can be achieved?

### 9.6 Programming Projects

**Project 9.1: Shannon Limit Visualizer** — Plot optical channel capacity vs. launched power for a long-haul fiber link including: OSNR-limited regime (linear), nonlinear noise-limited regime. Show the peak capacity and compare to current deployed systems.

**Project 9.2: Coherent Receiver Simulator** — Implement a full coherent optical receiver in software: (a) generate 16-QAM symbols, (b) add phase noise and AWGN, (c) implement chromatic dispersion compensation (FDE), (d) carrier phase recovery (Viterbi-Viterbi), (e) compute BER vs. OSNR curve.

**Project 9.3: WDM System Designer** — Build a link budget calculator for a DWDM system. Inputs: fiber loss, EDFA gain/NF, span length, number of spans, modulation format. Output: OSNR at receiver, required margin vs. FEC threshold, maximum reach.

### 9.7 Further Reading and Key Researchers
- **Textbooks:** Agrawal, *Fiber-Optic Communication Systems* (6th ed.); Proakis & Salehi, *Digital Communications*; Cover & Thomas, *Elements of Information Theory*
- **Key Researchers:** Shannon (information theory), Essiambre & Winzer (nonlinear fiber capacity), Ip & Kahn (coherent detection revival)
- **Key Papers:** Shannon (1948), "A Mathematical Theory of Communication"; Essiambre et al. (2010), "Capacity Limits of Optical Fiber Networks," *J. Lightwave Technology*

---

## Chapter 10: Optical Interconnects and Data Center Networks

### 10.1 The Interconnect Bottleneck

#### 10.1.1 Power and Bandwidth Scaling of Electrical Links
- Electrical link energy: ~1-5 pJ/bit at chip-to-chip distances > 5 cm
- Bandwidth × distance product limited by skin effect and dielectric losses
- A100 GPU: 4.8 TB/s total memory bandwidth, but limited by copper SerDes energy
- Optical link energy: ~0.5-1 pJ/bit, distance-independent past a few centimeters

#### 10.1.2 Co-Packaged Optics (CPO)
- Move optical transceivers from front panel into the package
- Eliminate lossy chip-to-QSFP electrical connection
- Ayar Labs TeraPHY: monolithic electronic-photonic chip with optical I/O
- Intel Co-Packaged Optics: heterogeneous integration in EMIB package
- Key challenge: heat management, yield, serviceability

#### 10.1.3 Photonic Network-on-Chip (PNoC)
- Replace electrical buses with optical waveguide networks inside a processor
- WDM channels provide spatial reuse
- Proposed architectures: ATAC, Corona, Firefly, Petabit
- Energy per operation potential: <0.1 pJ/bit at cm scale
- Key barrier: on-chip laser integration

### 10.2 Data Center Network Topologies

#### 10.2.1 Leaf-Spine Architecture
- Two-tier: spine switches and leaf (top-of-rack) switches
- Full bisection bandwidth, predictable latency
- East-west traffic dominates in AI training workloads

#### 10.2.2 Optical Circuit Switching
- OCS (e.g., Google Orion, Microsoft Sirius): establish optical paths for heavy long-lived flows
- MEMS-based OCS: ~10 ms reconfiguration, thousands of ports
- Wavelength selective switch (WSS): reconfigure WDM channels
- Hybrid EPS + OCS architectures

#### 10.2.3 All-Optical Switching
- SOA-based burst-mode switches: ns reconfiguration
- Micro-ring switch fabrics: ns, but narrow bandwidth
- AWGR (AWG Router): wavelength-space routed, truly non-blocking

### 10.3 Exercises

1. An AI training cluster uses 1000 GPUs connected by a leaf-spine fabric. The all-reduce communication pattern requires bisection bandwidth B. If optical links at 400 Gbps replace 100G electrical, by what factor does the communication time decrease?
2. Calculate the power consumed by a 400G QSFP transceiver (8 W typical) vs. a co-packaged optical I/O chip (1.5 pJ/bit, same 400G rate). For a 128-port switch, what is the total power savings?
3. Design a WDM-routed PNoC for a 64-core processor. Each core needs 100 Gbps of total bandwidth. Assign wavelengths to routing paths and estimate the total laser power required.

### 10.4 Programming Projects

**Project 10.1: Data Center Network Traffic Simulator** — Implement a discrete-event simulator for a leaf-spine network. Compare TCP throughput and latency for: all-electrical, hybrid OCS, and all-optical switching under realistic AI training all-reduce traffic patterns.

**Project 10.2: Photonic Network-on-Chip Power Estimator** — Build a model of a WDM PNoC (e.g., Corona architecture). Compute total laser power, modulator power, and detector power as a function of the number of wavelengths, core count, and link distance. Compare to equivalent electrical mesh NoC.

### 10.5 Further Reading and Key Researchers
- **Textbooks:** Vahdat et al., Google's data center network evolution papers; Chowdhury & Boutaba (2010), "A survey of network virtualization"
- **Key Researchers:** Vahdat (Google), Singla (CMU), Bergman (Columbia PNoC), Miller (Stanford, optical interconnects)
- **Key Papers:** Miller (2009), "Device requirements for optical interconnects to silicon chips," *Proc. IEEE*; Farrington et al. (2010), "Helios: A Hybrid Electrical/Optical Switch Architecture," *SIGCOMM*

---
# PHOTONIC COMPUTING — BOOK OUTLINE (Part 3)
# Units V–VI: Classical Photonic Computing and Neuromorphic Photonics

---

# UNIT V: CLASSICAL PHOTONIC COMPUTING

> *The idea of computing with light is almost as old as the laser itself. By the 1980s, optical computing had become a fashionable research program: optical logic gates, optical memory, whole optical CPUs were imagined. The program largely collapsed — optical switching required far too much power per bit, and silicon electronics improved so rapidly that optics couldn't compete. But something changed in the 2010s. Deep learning exploded, demanding not logic gates but matrix multiplications — and matrix multiplication is something that light does naturally, effortlessly, at the speed of propagation. Light passing through a lens performs a Fourier transform. A Mach-Zehnder mesh performs a unitary matrix multiply. A diffractive stack performs a convolutional neural network inference. The question is no longer "can light compute?" but "what exactly should it compute, and at what scale?" This unit explores those questions with rigor.*

---

## Chapter 11: Fourier Optics as Analog Computing

### 11.1 The 4f Optical Processor

#### 11.1.1 Mathematical Foundation
- Input field at front focal plane: U_in(x,y)
- Lens 1 (focal length f): performs Fourier transform at back focal plane
  - Field at Fourier plane: $\tilde{U}(f_x, f_y) = \mathcal{F}\{U_{in}\}$ where f_x = x/(λf), f_y = y/(λf)
- Filter H(f_x, f_y) placed at Fourier plane: multiply in frequency domain
- Lens 2: performs inverse Fourier transform
- Output: U_out(x,y) = h(x,y) * U_in(x,y), where h = FT{H} is the point spread function
- This is an optical convolution machine

#### 11.1.2 Spatial Filtering Operations
- Low-pass filter: circular aperture at Fourier plane → blurring
- High-pass filter: central stop → edge enhancement
- Matched filter: H(f_x, f_y) = S*(f_x, f_y) → correlation with template S
- Phase-only filter: |H| = 1, only phase altered → Zernike phase contrast microscopy
- Worked example: design a 4f matched filter for pattern recognition

#### 11.1.3 Computing Complexity
- The 4f system computes a 2D convolution in O(1) time (propagation time)
- For an N×N image: electronic FFT requires O(N² log N), optical does it in constant time
- Key limitation: input and output are in intensity domain (loss of negative weights)
- Limitation: dynamic range limited by SLM and detector noise

#### 11.1.4 Optical Correlator for Pattern Recognition
- VanderLugt correlator: holographic matched filter
- Joint transform correlator: no need for separate filter fabrication
- Applications: face recognition, fingerprint matching (demonstrated pre-digital era)
- Modern relevance: basis for D2NN and free-space photonic tensor processors

### 11.2 Microwave Photonics

#### 11.2.1 RF Signal Processing with Photonics
- Photonic link: RF → optical modulator → fiber → photodetector → RF
- Avoids the bandwidth limitations of electronic RF processing
- Spurious-free dynamic range (SFDR) of photonic links
- Photonic true-time delay for wideband beamforming

#### 11.2.2 Photonic Radar and LIDAR
- Photonic synthesis of wideband chirp signals
- Coherent LIDAR: frequency-modulated continuous wave (FMCW)
- Range: ΔR = c/(2B) where B = chirp bandwidth
- Silicon photonics LIDAR chips: Aeva, Luminar, Sense Photonics
- Photonic ADC for high-bandwidth digitization of radar returns

#### 11.2.3 Photonic Beamforming
- Optical true-time delay: avoids squint in wideband arrays
- N-element phased array with photonic delay control
- Integration: silicon photonic beamformer chips
- Applications: 5G mmWave, satellite communication, radar

### 11.3 Optical Logic and Switching

#### 11.3.1 Why Optical Logic Is Hard
- Electronic transistor: input controls output with gain and fan-out
- Optical equivalent: need photon to control photon
- Photon-photon interaction requires a medium (nonlinear material) → low efficiency
- Required energy per bit must be < kT ln 2 (Landauer limit) ~ 3×10⁻²¹ J
- Current SOA-based optical gates: ~1 pJ/operation — still orders of magnitude above electronics for logic

#### 11.3.2 SOA-Based Optical Gates
- Cross-gain modulation (XGM): saturating a pump depletes gain for probe
- Cross-phase modulation (XPM): saturating pump changes refractive index for probe
- Four-wave mixing (FWM) in SOA: generates conjugate of signal
- All-optical flip-flop with two coupled SOA-MZI switches (demonstrated by COBRA/COBRA, TU/e)

#### 11.3.3 Phase-Change Optical Logic
- GST-loaded waveguide: amorphous = high transmission, crystalline = high attenuation
- Nonvolatile optical logic: state persists without power
- Logical NOT, AND, OR with PCM-loaded MZI gates
- Multi-level states: beyond binary computation

### 11.4 Optical Analog-to-Digital Conversion

#### 11.4.1 The Photonic Time-Stretch ADC
- Stretch time axis of wideband signal by chirped fiber dispersion
- Digitize the stretched signal with lower-bandwidth electronics
- Effective bandwidth multiplication: B_eff = M × B_electronic where M = stretch factor
- Demonstrated: 10 Tbps optical bandwidth digitized with 10 GHz electronics
- Applications: single-shot radar, high-speed oscilloscopes

#### 11.4.2 Optical Sampling
- Ultrashort mode-locked laser pulses as sampling clock
- Extremely low timing jitter (< 10 fs for fiber frequency combs)
- Photonic ADC demonstrated at >1 Tbaud sampling
- Comparison: electronic jitter-limited bandwidth vs. photonic

### 11.5 Exercises

**Mathematical:**
1. A 4f optical correlator uses a matched filter H = S*(f_x, f_y). Show that the output field at the image plane is the cross-correlation of the input with S.
2. Derive the SFDR of a photonic RF link with modulator Vπ = 3 V and third-order intercept point IIP3. Show SFDR ∝ (IIP3/noise)^(2/3).
3. An optical ADC stretches a 100 GHz-bandwidth signal by a factor M = 100 using dispersive fiber. The effective analog bandwidth is then digitized by a 5 GHz ADC with 10 effective bits. Calculate the effective ADC input bandwidth and total dynamic range.
4. For a PCM-loaded waveguide with 3 dB/cm loss in the amorphous state and 20 dB/cm in crystalline state, over a 200 μm long section, calculate the transmission contrast (extinction ratio) in dB.

**Conceptual:**
5. Why did 1980s all-optical computing fail? Which specific assumptions about photonic devices turned out to be incorrect?
6. Compare optical convolution (4f system) to digital convolution on a GPU. For what image sizes does optical become competitive, assuming the 4f system has 1 ns latency?

### 11.6 Programming Projects

**Project 11.1: 4f Optical Processor Simulator** — Simulate a 4f optical correlator using 2D FFT. Implement: (a) matched filter for face detection, (b) edge-detection high-pass filter, (c) phase contrast filter. Quantify the correlation peak vs. false-positive rate.

**Project 11.2: Photonic Time-Stretch ADC Model** — Model the complete photonic time-stretch ADC chain: mode-locked laser → dispersive stretcher → modulator → dispersive compressor → electronic ADC. Simulate digitization of a 100 GHz chirp signal. Compute SNR and ENOB.

**Project 11.3: SOA-Based Optical Gate Simulation** — Implement the SOA rate equation model including carrier depletion and recovery. Simulate XGM-based wavelength conversion: input a 10 Gbps OOK signal at λ_pump, output inverted OOK at λ_probe. Plot eye diagram quality vs. bias current.

### 11.7 Further Reading and Key Researchers
- **Textbooks:** Goodman, *Introduction to Fourier Optics*; Capmany & Novak (2007), "Microwave photonics combines two worlds," *Nature Photonics*
- **Key Researchers:** Goodman (Fourier optics, Stanford), Jalali (photonic ADC, UCLA), Seeds & Williams (microwave photonics, UCL)
- **Key Papers:** VanderLugt (1964), "Signal detection by complex spatial filtering," *IEEE Trans. Inf. Theory*; Coppinger et al. (1999), "Photonic time stretch and its application to analog-to-digital conversion," *IEEE Trans. Microwave Theory*

---

## Chapter 12: Matrix-Vector Multiplication with Photonics

> *The entire edifice of modern deep learning rests on matrix multiplication. A single forward pass through GPT-4 executes approximately 10¹⁵ floating-point operations, the vast majority of them matrix multiplies. GPU clusters burn megawatts performing these operations electronically. Photons, by contrast, perform the same computation in the time it takes light to cross a waveguide — and they do it in parallel, without heat, without charge transport. This chapter is the technical heart of analog photonic computing: how to encode matrices in light, multiply them, and read out the result.*

### 12.1 The Case for Optical Linear Algebra

#### 12.1.1 Computational Complexity
- Matrix-vector product **y** = **W** · **x**: O(N²) operations for N×N matrix
- Matrix-matrix product: O(N³) → O(N^2.37) with Strassen/Coppersmith-Winograd
- At N = 10,000: 10⁸ multiplications and additions per result vector
- GPU A100: ~312 TFLOPS (FP16), power 400 W → ~0.78 TFLOPS/W
- Photonic claim: >10 TOPS/W at nanosecond latency

#### 12.1.2 Analog vs. Digital Trade-offs
- Analog photonic: no quantization, parallel by nature, but noisy
- Digital electronic: exact, programmable, but serial and energy-hungry
- Precision: analog photonic limited to ~6-8 bits ENOB currently
- Key insight: for neural network inference, 8-bit precision is often sufficient

#### 12.1.3 The Optical MAC Operation
- Multiply-accumulate (MAC): y = Σᵢ wᵢxᵢ
- Encoding: wᵢ as optical amplitude, xᵢ as modulated power
- Accumulation: coherent superposition of fields = amplitude summing
- Or: incoherent summation of intensities = power summing
- Photodetector converts optical power to current: multiplication complete

### 12.2 The Mach-Zehnder Interferometer as a Unitary Gate

#### 12.2.1 MZI Transfer Matrix
- A balanced MZI with phase shift θ in one arm and input coupler angle φ:
$$U_{MZI}(\theta, \phi) = \begin{pmatrix} e^{i\phi}\cos\theta/2 & i\sin\theta/2 \\ i\sin\theta/2 & e^{i\phi}\cos\theta/2 \end{pmatrix}$$
- This is a 2×2 unitary matrix (up to global phase)
- Parameterized by two continuous parameters: basis for arbitrary unitary

#### 12.2.2 Reck Decomposition
- Any N×N unitary matrix U can be decomposed into at most N(N-1)/2 MZI elements
- Reck et al. (1994) proof: triangular mesh architecture
- Total MZIs required: N(N-1)/2
- Depth (layers): 2N-3
- For N=8: 28 MZIs needed

#### 12.2.3 Clements Decomposition
- Alternative rectangular mesh architecture (Clements et al. 2016)
- Same number of MZIs: N(N-1)/2
- Depth reduced to N (columns)
- More resilient to loss: balanced path lengths
- Preferred for silicon photonic implementation

#### 12.2.4 Programming an MZI Mesh
- Target unitary U → find phases {θᵢ, φᵢ} via sequential nulling algorithm
- Forward pass: compute output state for given phases
- Inverse: use QR-like decomposition to find all phases for target U
- Gradient-based optimization: use backpropagation on the phase parameters

#### 12.2.5 Errors and Imperfections
- Fabrication errors: coupler splitting ratio deviates from 50/50 (typically ±5%)
- Phase errors: heater calibration, thermal drift
- Loss imbalance between arms
- Error model: U_actual ≈ U_target · exp(i·E) where E = error matrix
- Impact on matrix fidelity: error grows as O(N·σ_error)
- Correction strategies: in-situ measurement and compensation

### 12.3 Singular Value Decomposition Implementation

#### 12.3.1 SVD and Neural Network Weights
- Any real matrix W = U Σ Vᵀ where U, V are orthogonal and Σ diagonal
- Optical implementation: V† mesh → diagonal attenuators (Σ) → U mesh
- Non-unitary matrix computation: enables all arbitrary linear maps
- Required: amplitude modulators for Σ (ring modulators or MZMs)

#### 12.3.2 The Shen et al. 2017 Experiment (MIT)
- First experimental ONN on chip
- 56 MZIs in silicon nitride for vowel recognition
- 4×4 unitary matrices: 6 MZIs each
- Demonstrated 76.7% accuracy on ORL face dataset
- Key finding: optical errors correctable by training noise into the model

#### 12.3.3 Lightmatter Mars and Envise
- Mars: photonic matrix processor for inference
- Envise: complete AI accelerator with Mars photonic core
- Architecture: WDM parallelism + MZI mesh for matrix computation
- Performance claims: 100× energy efficiency over GPU for inference
- First commercial photonic AI chip for edge applications

### 12.4 Wavelength-Multiplexed Incoherent Computing

#### 12.4.1 Microring Weight Banks
- N microring resonators on a bus waveguide, each resonant at λᵢ
- Each ring thermally tuned to set transmission T(λᵢ) ∈ [0, 1] → weight wᵢ
- N WDM input channels modulated with inputs x₁, x₂, ..., xN
- Each channel power: Pᵢ = wᵢ · xᵢ (after ring filter)
- Photodetector integrates all wavelengths: y = Σ wᵢxᵢ ✓
- This is an incoherent weighted sum (no phase required)

#### 12.4.2 Positive-Only Weights and Differential Detection
- Ring transmission ∈ [0, 1]: only positive weights
- Solution: differential pair — two photodetectors, y = y⁺ - y⁻
- Positive weight ring bank + negative weight ring bank → signed weights
- Differential ring modulator architecture (Tait et al., Princeton)

#### 12.4.3 Broadcast-and-Weight Architecture
- Broadcast input signal to all nodes via bus waveguide
- Each node selects its wavelength with a ring filter
- Weighted sum at each detector → neuron output
- Scale to NxM matrix: N wavelengths, M detectors
- Chip layout and power budget analysis

#### 12.4.4 WDM-Based Matrix-Vector Multiplication at Scale
- Each row of matrix W: one detector with ring weight bank (N rings)
- For M outputs: M such detector banks
- Total rings: N×M
- Total MZMs: N (one per input column)
- Bandwidth: N × channel rate × M outputs/clock
- Power consumption estimate: laser + modulators + heaters + detectors

### 12.5 Time-Division and Space-Division Multiplexing

#### 12.5.1 Temporal Multiplexing for Matrix Rows
- Process one row at a time using shared hardware, time-multiplex
- Reduces hardware but increases latency proportionally
- Suitable for batch inference on fixed hardware

#### 12.5.2 Space-Division Multiplexing with Waveguide Crossbar
- NxN waveguide crossbar: input waveguides × output waveguides
- Each crossing: a Mach-Zehnder cell with amplitude and phase control
- Fully connected: any input to any output
- Loss grows with N (crossbar loss ~ N dB)
- Silicon photonic crossbar demonstrations up to 8×8

### 12.6 Exercises

**Mathematical:**
1. Show that the transfer matrix of a balanced MZI with phase shift θ in the upper arm is unitary. Find the eigenvalues and eigenvectors of this matrix.
2. Decompose the 4×4 DFT matrix using the Clements decomposition. How many MZIs are required? Draw the network diagram.
3. For a ring weight bank with N = 16 channels and ring Q = 10,000, calculate the crosstalk between adjacent channels (spaced 50 GHz) when a ring is tuned off-resonance by one FSR/N.
4. A photonic matrix multiplier has shot noise limited SNR of 35 dB per output element. What is the maximum ENOB? For 8-bit precision, what minimum optical power is required at the detector?
5. The Clements decomposition requires N(N-1)/2 MZIs for an N×N unitary. For N = 64, calculate the total number of phase-tuning elements and estimate chip area (each MZI: 500 μm × 100 μm).

**Conceptual:**
6. Compare coherent (MZI mesh) and incoherent (ring weight bank) photonic matrix multiplication. List the pros and cons of each for: (a) accuracy, (b) reconfiguration speed, (c) power efficiency, (d) negative weight handling.
7. Why do photonic neural networks trained with ideal weights suffer accuracy degradation when deployed on real hardware? What training strategies mitigate this?

### 12.7 Programming Projects

**Project 12.1: MZI Mesh Simulator** — Implement a Clements-decomposition MZI mesh for N=8. Given a target unitary matrix, compute all phase settings. Apply random fabrication errors (Gaussian σ=0.05 rad per MZI) and measure the output fidelity. Implement compensation by gradient descent on the phase parameters.

**Project 12.2: Ring Weight Bank Simulator** — Model an 8-channel WDM ring weight bank in Python. Compute the transmission matrix including crosstalk between channels as a function of ring Q and channel spacing. Show the impact of thermal drift on weight accuracy.

**Project 12.3: Photonic MNIST Classifier** — Train a 4-layer photonic neural network (ONN) in PyTorch using the neuroptica or neurophox library to classify MNIST digits. Compare performance when: (a) perfect hardware, (b) hardware errors σ=0.01 rad, (c) noise-aware training, (d) post-training calibration.

**Project 12.4: Optical Matrix Multiplier Power Model** — Build a component-level power model for a WDM broadcast-and-weight matrix multiplier performing a 64×64 matrix-vector product. Account for: laser wall-plug efficiency, modulator drive power, thermo-optic heater power, and TIA power. Compare to equivalent NVIDIA Tensor Core operation.

**Project 12.5: Reck/Clements Decomposition Library** — Implement both Reck and Clements decomposition algorithms in Python. Given a random N×N unitary, find the MZI phases, reconstruct the unitary, and verify accuracy. Benchmark decomposition time and compare to NumPy matrix multiply.

### 12.8 Further Reading and Key Researchers
- **Textbooks:** No dedicated textbook yet; see review papers
- **Key Researchers:** Gu & Sorin (photonic computing at Lightmatter); Miller (Stanford, optical computing fundamentals); Prucnal (Princeton, neuromorphic photonics); Wetzstein (Stanford, optical neural networks)
- **Key Papers:**
  - Reck et al. (1994), "Experimental realization of any discrete unitary operator"
  - Clements et al. (2016), "Optimal design for universal multiport interferometers"
  - Shen et al. (2017), "Deep learning with coherent nanophotonic circuits," *Nature Photonics*
  - Hamerly et al. (2019), "Experimental investigation of performance differences between coherent and incoherent feedback OPU"

---

## Chapter 13: Photonic Neural Networks — Architecture and Training

> *In 2017, a paper from MIT appeared in Nature Photonics with the arresting title "Deep learning with coherent nanophotonic circuits." It showed a silicon photonic chip performing optical matrix-vector multiplication to recognize vowels — not perfectly, not at scale, but unmistakably. It was proof of principle, and it ignited a research program. In the years since, photonic neural networks have advanced from 4-neuron demonstrations to systems claiming teraops-per-watt efficiencies, from lab curiosities to venture-funded companies. This chapter gives you the mathematical and practical foundation to understand, evaluate, and build these systems.*

### 13.1 Deep Learning Foundations (Photonics-Oriented)

#### 13.1.1 The Feedforward Network as Matrix Operations
- Layer l: activations a^(l) = f(W^(l) a^(l-1) + b^(l))
- W^(l): weight matrix (what the photonic chip computes)
- f(·): nonlinear activation function (what electronics must provide)
- b^(l): bias vector (additional electronic offset)
- In an ONN: W^(l) is realized optically; f and b electronically

#### 13.1.2 Backpropagation
- Forward pass: compute predictions, store activations at each layer
- Loss function L(ŷ, y)
- Backward pass: compute ∂L/∂W^(l) = δ^(l) (a^(l-1))ᵀ where δ^(l) = (W^(l+1))ᵀ δ^(l+1) ⊙ f'(z^(l))
- Weight update: W^(l) ← W^(l) - η ∂L/∂W^(l)
- For photonic hardware: gradients require backpropagation through the physical device model

#### 13.1.3 Hardware Bottleneck for AI
- Training GPT-3 required ~3×10²³ FLOP → 1000 GPU-years at current efficiency
- Inference for LLM: 10¹⁰-10¹² FLOP per request
- Energy cost: each word generated by ChatGPT consumes ~0.001 kWh
- Photonic computing target: 10-100× improvement in FLOPS/Watt for inference

### 13.2 Optical Activation Functions

#### 13.2.1 The Problem
- Linear optical networks (MZI mesh): only compute linear functions
- Neural network power comes from nonlinearity
- Optical nonlinearities are weak at low power → energy cost
- Electronic nonlinearities require optical-electronic-optical conversion

#### 13.2.2 Electro-Optic Nonlinear Activation
- Detect optical output, apply nonlinearity electronically (ReLU, sigmoid), re-encode optically
- Energy cost: O-E-O conversion per layer: 0.5-5 pJ/activation
- Latency: ~100 ps per O-E-O conversion

#### 13.2.3 All-Optical Nonlinear Activation
- Saturable absorption in SOA or graphene
- Cross-phase modulation: control beam shifts signal nonlinearly
- Optical bistability: ring resonator below threshold
- Limitation: nonlinear threshold power >> ideal computing power budget

#### 13.2.4 Computing in the Linear Regime
- Random feature networks: one nonlinear layer + many linear layers
- Reservoir computing: fixed nonlinear dynamics + trainable linear readout
- ELM (Extreme Learning Machine): random weights, train only output layer

### 13.3 Training Photonic Neural Networks

#### 13.3.1 Offline Training with Hardware Simulation
- Train in PyTorch/TensorFlow using a differentiable model of the hardware
- Include noise, fabrication errors, limited bit precision
- Deploy trained weights to hardware
- Challenge: simulation model mismatch → inference accuracy loss

#### 13.3.2 In-Situ Training
- Run forward passes on actual hardware
- Measure output and compute loss
- Backpropagate through hardware: need direct measurement of gradients
- Gradient-free methods: Zeroth-order optimization (SPSA, CMA-ES)
- Gradient through physics: forward and backward optical propagation
- Challenges: slow (sequential updates), requires extra hardware measurements

#### 13.3.3 Noise-Aware Training
- Add noise model into the training forward pass: σ_phase, σ_splitting
- Network learns to be robust to hardware imperfections
- Trade-off: accuracy on ideal hardware vs. accuracy on noisy hardware
- Key reference: Bandyopadhyay et al. (2022), noise-immune ONN training

#### 13.3.4 Hardware-in-the-Loop Training
- Actual photonic chip used in the training loop
- Forward pass: optical → measure output
- Backward pass: estimated gradients (adjoint method or finite differences)
- Demonstrated for small ONNs by Pai et al. (2023), Stanford

### 13.4 Photonic Reservoir Computing

#### 13.4.1 Reservoir Computing Concept
- Fixed nonlinear dynamical system (reservoir): maps input to high-dimensional state
- Only the readout (linear regression) layer is trained
- Reservoir need not be well-understood — just nonlinear and high-dimensional

#### 13.4.2 Single-Node Photonic Reservoir
- Nonlinear node: semiconductor laser or MZM
- Time-delay feedback loop: creates N virtual nodes via temporal multiplexing
- Input x(t) drives the node, output measured at N time points = N-dimensional reservoir state
- Train readout weights via ridge regression
- Demonstrated by Brunner et al. (2013): 77 GBit/s classification

#### 13.4.3 Integrated Photonic Reservoir
- Array of coupled microring resonators as reservoir nodes
- Rich nonlinear dynamics: chaotic regime for large coupling
- Random Gaussian coupling → echo state network in photonics
- Applications: time-series prediction, spoken digit recognition, optical channel equalization

### 13.5 Optical Transformers and Attention Mechanisms

#### 13.5.1 Self-Attention as Matrix Operations
- Q = XW_Q, K = XW_K, V = XW_V (linear projections)
- Attention score: A = softmax(QKᵀ/√d_k)
- Output: Y = AV
- Photonic: W_Q, W_K, W_V computed by MZI meshes; softmax electronically

#### 13.5.2 Photonic Dot-Product Accelerator for Attention
- Inner product QKᵀ: computed photonically row by row
- Bottleneck: O(L²) where L = sequence length
- Photonic attention: L parallel optical dot-product units → O(L) optical time
- Proposal: "FlashAttention" analog in photonics using WDM parallelism

### 13.6 Exercises

**Mathematical:**
1. Show that an N×N MZI mesh with ideal MZIs can represent any N×N unitary. What additional hardware is needed to represent any complex-valued matrix W (not necessarily unitary)?
2. Derive the signal-to-noise ratio for a single photonic neuron (ring weight bank detector) as a function of input power per channel, ring transmission, fiber loss, and detector noise.
3. For a photonic reservoir with N=50 virtual nodes and a time-delay of T=1 ns, estimate the maximum input symbol rate and the reservoir state dimensionality.
4. In noise-aware ONN training, if the phase noise is Gaussian with σ=0.02 rad, derive the expected output fidelity degradation for a 64×64 MZI mesh (Clements architecture) after one forward pass.

**Conceptual:**
5. Why is the energy-per-operation of an ONN potentially much lower than a GPU for inference, but not for training? What would be needed to enable photonic training?
6. An ONN is trained with offline simulation and then deployed on hardware. The inference accuracy drops from 94% to 87%. List five possible causes and rank them by likelihood.

### 13.7 Programming Projects

**Project 13.1: ONN in PyTorch with Hardware Noise** — Using the neurophox library, implement a 4-layer ONN for CIFAR-10 classification with MZI mesh layers. Train with and without noise augmentation (σ_phase = 0.01 rad). Measure accuracy degradation after deployment and improvement from noise-aware training.

**Project 13.2: Photonic Reservoir Computer** — Implement a time-delay reservoir computer in Python: (a) single-node MZM-based reservoir, (b) time-multiplexed virtual nodes, (c) train a ridge regression readout for NARMA-10 time series prediction. Compare performance to an ESN with same number of nodes.

**Project 13.3: Hardware-in-the-Loop ONN Emulator** — Build a software emulator of hardware-in-the-loop training where you intentionally introduce correlated phase errors. Implement SPSA gradient estimation and compare convergence to exact backpropagation. Measure extra epochs needed for equivalent accuracy.

**Project 13.4: Photonic Attention Unit** — Implement a photonic self-attention layer simulator where the Q, K, V projections are computed by ideal MZI meshes. Add phase noise and bit-width quantization. Measure the perplexity of a language model using photonic vs. ideal attention.

### 13.8 Further Reading and Key Researchers
- **Key Researchers:** 
  - Prucnal (Princeton) — neuromorphic photonic networks
  - Wetzstein (Stanford) — optical neural networks, optics for AI
  - Englund (MIT) — photonic processors, quantum photonics
  - Hughes (Lightmatter/Stanford) — training ONNs
  - Brunner (Univ. Burgundy) — photonic reservoir computing
  - Larger (Univ. Burgundy) — delay-based reservoir
- **Key Papers:**
  - LeCun, Bengio & Hinton (2015), "Deep learning," *Nature* (background)
  - Shen et al. (2017), "Deep learning with coherent nanophotonic circuits"
  - Brunner et al. (2013), "Parallel photonic information processing at gigabyte per second data rates"
  - Hughes et al. (2018), "Training of photonic neural networks through in situ backpropagation"
  - Bandyopadhyay et al. (2022), "Single chip photonic deep neural network"
- **Software:** neurophox (MIT), neuroptica, PhoxoniX (Lightmatter), PennyLane (Xanadu)

---

## Chapter 14: Diffractive Deep Neural Networks (D2NN)

> *What if a neural network could be made of nothing more than sheets of plastic? In 2018, a team at UCLA showed exactly this: they designed a stack of diffractively engineered surfaces that, when illuminated with terahertz light, could classify handwritten digits with 91% accuracy — without any electronic components, without any power beyond the incident light itself. The "network" was encoded entirely in the geometry of the diffractive layers. This is perhaps the purest expression of optical analog computing: matter itself becomes the computation.*

### 14.1 The D2NN Framework

#### 14.1.1 Physical Principle
- Each neuron at layer l is a point on a diffractive surface (a pixel)
- It receives light from all pixels of layer l-1 (through free-space propagation)
- It modifies the phase (and optionally amplitude) of transmitted light
- It emits to all pixels of layer l+1 (again through free-space propagation)

#### 14.1.2 Mathematical Model
- Field at pixel (x', y') on layer l+1 due to all pixels at layer l:
$$U^{l+1}(x', y') = \sum_{x,y} w(x,y,l) \cdot t^l(x,y) \cdot U^l(x,y) \cdot h(x'-x, y'-y, d_l)$$
- w(x,y,l) = amplitude weight (from detector geometry)
- t^l(x,y) = complex transmission coefficient (trainable parameter)
- h = Rayleigh-Sommerfeld propagation kernel
- This is a weighted sum over the previous layer → analogous to a neural network layer

#### 14.1.3 Training with Backpropagation
- Forward pass: simulate optical diffraction through all layers
- Loss function: distance between output intensity pattern and target
- Backward pass: backpropagate gradients through the differentiable diffraction model
- Trainable parameters: {t^l(x,y)}: complex transmission coefficients (or just phases if amplitude=1)
- Typically phase-only training: constrain |t^l| = 1

#### 14.1.4 The Original Lin et al. 2018 Experiment
- 5 diffractive layers, 200×200 pixels each
- Terahertz illumination at 0.4 THz (λ = 0.75 mm)
- 3D-printed layers from plastic (phase modulator)
- Trained in simulation, fabricated physically
- Performance: 91.75% test accuracy on MNIST
- Physical inference speed: limited by light propagation (nanoseconds)

### 14.2 Optical Implementations

#### 14.2.1 3D-Printed Diffractive Layers
- Phase profile encoded in varying surface height
- Phase shift: φ(x,y) = 2π(n-1)h(x,y)/λ
- Height h controls phase: dynamic range 0-λ/(n-1) for 0-2π
- Resolution limited by printer voxel size (typically 50-100 μm)
- Suitable for THz and mm-wave D2NN

#### 14.2.2 Spatial Light Modulators (SLMs)
- Liquid crystal SLM: 1080×1920 pixels, phase tunable 0-2π at 60 Hz
- Reconfigurable D2NN: change the trained weights by updating SLM patterns
- Optical calibration required to correct SLM pixel-to-pixel variation
- Wavelength: visible (633/532 nm) or near-IR (1064 nm)

#### 14.2.3 Silicon Photonic Implementation
- Waveguide array: multimode waveguide with controllable scatterers
- Not yet fully demonstrated at scale, but theoretically feasible

#### 14.2.4 Metasurface D2NN
- Each layer: dielectric metasurface with subwavelength pixel pitch
- Compact: each layer is ~λ thick (vs. free-space d ≫ λ)
- Non-reconfigurable but highly compact and CMOS-compatible
- Recent demonstration: metalens-array D2NN for image classification

### 14.3 Physical Limits and Trade-offs

#### 14.3.1 Optical Crosstalk Between Pixels
- Rayleigh-Sommerfeld propagator: near-field pixel-to-pixel crosstalk
- Far-field limit: each pixel contributes to all downstream pixels (all-to-all)
- Optimal layer separation d: balance between connectivity and crosstalk

#### 14.3.2 Depth and Width Trade-offs
- More layers: higher representational power
- Wider layers (more pixels): higher throughput
- Physical constraint: device footprint = number_of_layers × d
- Energy constraint: power proportional to input intensity × area

#### 14.3.3 Non-Negativity Constraint
- D2NN detects intensity |U|²: intrinsically non-negative output
- Loss of phase information at each layer detection event
- Solution: avoid intermediate detection; keep fields complex throughout
- Phase detection (homodyne) adds complexity but recovers negative weights

### 14.4 Applications

#### 14.4.1 Image Classification
- Object recognition, digit recognition (MNIST, Fashion-MNIST)
- Performance vs. depth: deeper → better, diminishing returns
- Comparison to digital CNN at equivalent parameter count

#### 14.4.2 Optical Logic Operations
- XOR, XNOR implemented as D2NN operations
- Optical computing without any electronic elements
- Demonstrated at THz by Lin group (UCLA)

#### 14.4.3 Spectral Analysis and Object Detection
- D2NN as spectrometer: input spectrum → spatial distribution at output
- Object detection with bounding box prediction
- Wide-field computational imaging

### 14.5 Exercises

**Mathematical:**
1. Derive the Rayleigh-Sommerfeld propagation kernel h(x-x', y-y', z) from the Huygens-Fresnel principle. Show that in the far-field it reduces to a Fourier transform.
2. For a D2NN with L layers of N×N pixels each, separated by distance d: estimate the total optical path length, the effective connectivity (fan-in per neuron), and the compute throughput in ops/second for illumination intensity I.
3. Show that training a D2NN via backpropagation requires differentiating through the Rayleigh-Sommerfeld propagator. Write the gradient expression ∂L/∂t^l(x,y) in terms of forward and backward propagating fields.
4. A D2NN uses phase-only modulators with 8-bit phase resolution (256 levels). Estimate the equivalent precision loss compared to continuous phase, and its impact on classification accuracy.

**Conceptual:**
5. Compare a D2NN to a convolutional neural network (CNN). What is the effective receptive field of each D2NN neuron? Is the D2NN connectivity equivalent to a fully connected layer or a convolutional layer?
6. Why is training a D2NN in simulation and deploying physically ("sim-to-real") challenging? What hardware imperfections cause the largest performance gap?

### 14.6 Programming Projects

**Project 14.1: D2NN Simulator from Scratch** — Implement a D2NN forward pass using NumPy: (a) implement Rayleigh-Sommerfeld propagation via FFT, (b) implement the training loop with PyTorch autograd on phase parameters, (c) train a 5-layer D2NN on MNIST, (d) visualize the phase patterns of each trained layer.

**Project 14.2: SLM-Based D2NN with Calibration** — Model the imperfections of a real SLM (phase nonlinearity, pixel cross-talk, beam aberrations). Add these to the D2NN forward model. Retrain with hardware-aware loss. Implement the Gerchberg-Saxton algorithm for phase-only hologram generation.

**Project 14.3: Metasurface D2NN Design Tool** — Given a target phase profile, design a silicon metasurface layer using a pre-computed library of pillar heights vs. phase. Apply the discrete phase values to a trained D2NN layer and measure accuracy degradation vs. fabrication resolution.

### 14.7 Further Reading and Key Researchers
- **Key Researchers:**
  - Ozcan (UCLA) — D2NN inventor
  - Lin (UCLA) — D2NN experiments
  - Psaltis (Caltech/EPFL) — optical computing, holographic neural networks
  - Wetzstein (Stanford) — neural étendue expansion, optics for AI
  - Mengu (UCLA) — D2NN extensions and applications
- **Key Papers:**
  - Lin et al. (2018), "All-optical machine learning using diffractive deep neural networks," *Science*
  - Mengu et al. (2019), "Analysis of Diffractive Optical Neural Networks"
  - Luo et al. (2019), "Design of task-specific optical systems using broadband diffractive neural networks"
  - Shi et al. (2022), "Seeing through defocus with a metalens array D2NN"
- **Textbooks:** Goodman, *Introduction to Fourier Optics* (for propagation physics)

---

# UNIT VI: NEUROMORPHIC PHOTONICS

> *Your brain runs on roughly 20 watts — less than a dim lightbulb — and yet it performs tasks that would require megawatts of GPU compute to approximate. It does this with spikes: brief electrical pulses that race along axons, arriving at synapses where they trigger or suppress subsequent spikes. The computational language of the brain is not floating-point arithmetic but spike timing, not matrix multiplication but coincidence detection. Neuromorphic computing tries to build silicon that speaks the brain's language. Photonics offers a remarkable twist: photonic neurons spike at picosecond timescales — a million times faster than biological neurons — and communicate at the speed of light. What happens when you build a brain out of lasers?*

---

## Chapter 15: Neuromorphic Computing Concepts

### 15.1 Biological Neurons and Spiking Neural Networks

#### 15.1.1 The Biological Neuron
- Dendrites: receive synaptic input (excitatory and inhibitory)
- Soma (cell body): integrates inputs, generates action potentials
- Axon: transmits spikes to downstream neurons
- Synapse: junction between neurons; weight determined by synaptic strength

#### 15.1.2 The Leaky Integrate-and-Fire (LIF) Model
- Membrane potential: $\tau_m \frac{dV}{dt} = -(V - V_{rest}) + RI(t)$
- When V reaches threshold V_th: fire spike, reset to V_rest
- Leaky: voltage decays exponentially toward rest if no input
- Integrate: accumulates input current R·I(t)
- Fire: all-or-nothing spike emission
- Biologically plausible, computationally efficient, hardware-amenable

#### 15.1.3 Spike Timing and Rate Coding
- Rate coding: information in average firing rate f [Hz]
- Temporal coding: information in precise spike timing
- Population coding: information distributed across many neurons
- Time-to-first-spike: fast, low-energy coding
- Photonic neurons naturally produce temporal coding (picosecond spikes)

#### 15.1.4 Spike-Timing-Dependent Plasticity (STDP)
- Hebbian rule: "neurons that fire together, wire together"
- STDP: if pre-synaptic neuron fires just before post-synaptic, strengthen synapse (LTP)
- If pre fires after post, weaken synapse (LTD)
- Weight update: Δw = A_+ exp(-Δt/τ_+) if Δt > 0; Δw = -A_- exp(Δt/τ_-) if Δt < 0
- Unsupervised learning: learns features from input statistics

#### 15.1.5 Neuromorphic Hardware
- Intel Loihi 2: 128 cores, 1M neurons, 120M synapses, 5 TOPS/W
- IBM TrueNorth: 4096 cores, 1M neurons, 256M synapses
- SpiNNaker (Manchester): ARM-based, real-time SNN simulation
- BrainScaleS (Heidelberg): analog accelerated hardware
- Advantage of photonics: ns-to-ps spike timescale vs. ms biological/μs electronic

### 15.2 Why Photonics for Neuromorphics?

#### 15.2.1 Speed Advantage
- Biological neuron: action potential duration ~1 ms, refractory period ~2 ms
- Electronic neuron: ~1-100 ns spike generation
- Photonic neuron: ~1-10 ps spike — 10⁶× faster than biology
- Network inference time scales with number of spike propagation steps: photonic→ 10⁶× faster

#### 15.2.2 Fan-Out and Broadcast
- One laser output → split to N receivers via waveguide splitters
- WDM: N wavelengths, each goes to different neuron
- Electronic fan-out: driver circuit needed for each output

#### 15.2.3 Energy Considerations
- Photonic SNN: energy proportional to number of spikes × energy per spike
- Low-spike-rate operation: very energy efficient
- Energy per spike challenge: current photonic neurons require ~1-100 fJ/spike (goal: <1 fJ)

---

## Chapter 16: Photonic Neurons and Synapses

### 16.1 Excitable Photonic Laser Neurons

#### 16.1.1 The Injection-Locked Laser as Excitable Node
- Semiconductor laser biased just below threshold + optical injection
- Class A laser dynamics: maps to FitzHugh-Nagumo neuron model
- Optical injection above critical power → excitable response: emit single optical spike
- Sub-threshold injection: no spike (integrate-and-forget)
- All-optical IF neuron: demonstrated by Nahmias et al. (2013)

#### 16.1.2 VCSEL-Based Optical Neurons
- VCSELs: compact, 2D arrays, direct modulation
- Polarization dynamics: VCSEL switches between TE and TM modes → photonic neuron
- Demonstrated: polarization switching as spiking mechanism
- Integration: VCSEL array coupled to optical interconnect fabric

#### 16.1.3 Excitable Microring Laser Neurons
- Coupled clockwise/counter-clockwise modes in a ring laser
- Mode competition → excitable dynamics
- On-chip: integrated with Si waveguides
- Energy per spike: ~10-100 fJ

#### 16.1.4 Semiconductor Laser Neuron Rate Equations
- Two coupled differential equations analogous to FitzHugh-Nagumo:
$$\dot{E} = \frac{1}{2}(G - 1)E + F_E$$
$$\dot{N} = \frac{I}{e} - \frac{N}{\tau_s} - G|E|^2$$
- G = g(N - N_tr): gain function
- F_E: Langevin noise term
- Excitable regime: parameter space where single pulse triggers spike

### 16.2 Photonic Synapses: Nonvolatile Optical Weights

#### 16.2.1 Phase-Change Material Optical Synapses
- GST thin film cladding on waveguide: transmission depends on crystallinity
- State change: nanosecond optical pulses from on-chip laser
- Amorphous→crystalline: optical pulse > 500 μW for > 1 ns (SET operation)
- Crystalline→amorphous: optical pulse > 1 mW for < 1 ns (RESET)
- Multi-level: partial crystallization → 16+ distinct transmission levels
- Retention: state holds for >10 years without power (nonvolatile)
- Demonstrated by Ríos et al. (2015), Nature Photonics

#### 16.2.2 GSST: Low-Loss Phase Change Material
- Ge₂Sb₂Se₄Te₁: similar phase change but orders of magnitude lower optical loss
- Transmission contrast at 1550 nm: 10 dB (amorphous) vs. 5 dB (crystalline) insertion loss
- Much better suited for cascadable photonic synaptic networks
- Developed by Zhang et al. (2019), MIT group

#### 16.2.3 In-Memory Photonic Computing
- Weight stored in PCM waveguide: no separate memory read required
- Light passing through the waveguide is automatically weighted by the PCM state
- Completely passive: no power to hold weight
- Ideal for: deployed (fixed) photonic neural networks

#### 16.2.4 Multi-Level Analog Synaptic Storage
- N intermediate crystallization levels: log₂(N) bits per synapse
- Precise optical pulse control for analog weight programming
- Drift in PCM resistance analogy (crystallization drift): long-term weight instability
- Mitigation: periodic refresh, drift-robust encoding

### 16.3 Photonic SNN Architectures

#### 16.3.1 WDM Photonic Spiking Network
- Each neuron has a distinct wavelength λᵢ
- Synaptic connection: ring resonator at λᵢ on receiving neuron's bus
- Ring transmission (tuned by PCM): synaptic weight
- Spike propagates as a brief optical pulse at λᵢ
- Receiver neuron integrates weighted pulses → spike when threshold exceeded

#### 16.3.2 Photonic Spiking Convolutional Layer
- 2D array of photonic neurons
- Shared convolutional kernel weights: same ring filter pattern repeated
- WDM enables weight sharing across spatial positions
- Demonstrated in simulation by Shastri et al. (2021)

### 16.4 Learning in Photonic SNNs

#### 16.4.1 STDP with Optical Pulses
- Pre-synaptic spike: optical pulse at λ_pre arrives at PCM synapse
- Post-synaptic spike: optical pulse at λ_post generated by post-neuron
- LTP implementation: pre then post → PCM crystallization (increase weight)
- LTD: post then pre → PCM amorphization (decrease weight)
- Timing control: propagation delay between nodes determines STDP window

#### 16.4.2 Surrogate Gradient for Photonic SNNs
- Spike function: non-differentiable step function
- Replace with smooth surrogate: σ(V - V_th) for backpropagation
- Enables gradient-based training of spiking photonic networks
- Training in simulation, deployment on hardware

### 16.5 Exercises

**Mathematical:**
1. Linearize the semiconductor laser neuron rate equations around the quiescent point (below threshold). Show that the system has a pair of complex eigenvalues, and determine the parameter range for excitable vs. oscillatory behavior.
2. For a photonic SNN with N neurons connected in an Erdős-Rényi random graph (connection probability p), estimate the average number of spikes per second at steady state as a function of N, p, and the excitatory/inhibitory ratio.
3. A PCM synapse stores a weight in 8 levels between transmission T_min = 0.1 and T_max = 0.9. Compute the weight precision in bits and the expected SNR if the transmission measurement noise is σ = 0.02.
4. Implement the STDP learning rule and show that it leads to competitive learning (winner-take-all) for a set of input patterns with overlapping features.

**Conceptual:**
5. Compare photonic SNN inference energy to GPU inference energy for a 10-layer fully-connected network with 1000 neurons per layer. State all assumptions clearly.
6. Why is a nonvolatile photonic synapse essential for deployed neuromorphic photonic hardware? What happens if the synaptic weights require continuous power to maintain?

### 16.6 Programming Projects

**Project 16.1: Photonic Leaky Integrate-and-Fire Neuron** — Implement the semiconductor laser LIF neuron model from its rate equations using scipy ODE solver. Stimulate with: (a) constant current → identify threshold, (b) sinusoidal input → frequency entrainment, (c) random Poisson spike train → show stochastic spiking. Compare to biological LIF with identical parameters.

**Project 16.2: PCM Synapse Multi-Level Programming Simulator** — Model a GST waveguide synapse with 8 analog levels. Simulate partial crystallization using a stochastic model of nucleation kinetics. Compute the programming accuracy, drift over time, and its impact on classification accuracy in a 3-layer photonic SNN.

**Project 16.3: STDP Photonic SNN on MNIST** — Build a 2-layer photonic SNN (784 input + 100 hidden neurons) with PCM synapses. Train with STDP (rate-coded inputs). Measure classification accuracy on MNIST after training and compare to error-backpropagation-trained analog ANN.

**Project 16.4: Photonic Reservoir SNN** — Implement a random spiking reservoir network of 100 photonic LIF neurons with random excitatory/inhibitory connections. Train a linear readout (SVM or ridge regression) on the spike rates. Test on: spoken digit recognition (N-MNIST) and chaotic time series prediction.

### 16.7 Further Reading and Key Researchers
- **Key Researchers:**
  - Prucnal & Shastri (Princeton) — photonic spiking neural networks
  - Nahmias (Princeton/Luminous) — photonic neuron demonstrations
  - Bhaskaran (Oxford) — PCM optical synapses
  - Ríos (Oxford/Maryland) — all-optical synaptic networks
  - Vandoorne (Ghent) — integrated photonic reservoirs
- **Key Papers:**
  - Nahmias et al. (2013), "A Leaky Integrate-and-Fire Laser Neuron for Ultrafast Cognitive Computing," *IEEE J. Sel. Top. Quantum Electron.*
  - Ríos et al. (2015), "Integrated all-photonic non-volatile multi-level memory," *Nature Photonics*
  - Shastri et al. (2021), "Photonics for artificial intelligence and neuromorphic computing," *Nature Photonics*
  - Vandoorne et al. (2014), "Experimental demonstration of reservoir computing on a silicon photonics chip," *Nature Communications*
- **Textbooks:** Mahowald, *VLSI Analogs of Neural Computation* (for neuromorphic background); Gerstner & Kistler, *Spiking Neuron Models* (free PDF)

---
# PHOTONIC COMPUTING — BOOK OUTLINE (Part 4)
# Units VII–X: Quantum Photonics, Fabrication, Industry, and Frontiers

---

# UNIT VII: QUANTUM PHOTONICS AND QUANTUM COMPUTING

> *In 1935, Einstein, Podolsky, and Rosen published a thought experiment designed to show that quantum mechanics was incomplete. They described two particles that, once they interacted, remained forever correlated — measure one and you instantly know something about the other, no matter how far apart they are. Einstein called it "spooky action at a distance" and considered it a flaw in the theory. Three decades later, John Bell proved that no local hidden-variable theory could reproduce quantum correlations. Another two decades on, experiments confirmed Bell's inequality violations with photons. Today, that spookiness is not a bug but a feature — the fuel of quantum computation, quantum cryptography, and quantum communication. And among all physical systems for quantum computing, photons hold a special place: they travel at light speed, carry quantum information without decoherence at room temperature, and can be manipulated with the tools of integrated photonics. This unit builds quantum photonics from its mathematical foundations to the architectures being fabricated today.*

---

## Chapter 17: Quantum Mechanics Foundations for Photonics

### 17.1 The Postulates of Quantum Mechanics

#### 17.1.1 State Vectors and Hilbert Space
- Pure state: |ψ⟩ ∈ ℋ (complex vector space with inner product)
- Inner product: ⟨φ|ψ⟩ ∈ ℂ
- Normalization: ⟨ψ|ψ⟩ = 1
- Dirac notation: bra ⟨φ|, ket |ψ⟩, outer product |ψ⟩⟨φ|
- Superposition: |ψ⟩ = α|0⟩ + β|1⟩, |α|² + |β|² = 1
- Density matrix: ρ = |ψ⟩⟨ψ| for pure state; ρ = Σᵢ pᵢ|ψᵢ⟩⟨ψᵢ| for mixed state

#### 17.1.2 Observables and Operators
- Observable: Hermitian operator Â = †
- Eigenvalue equation: Â|aₙ⟩ = aₙ|aₙ⟩
- Measurement outcome aₙ with probability |⟨aₙ|ψ⟩|²
- Expectation value: ⟨Â⟩ = ⟨ψ|Â|ψ⟩
- Uncertainty principle: σ_A σ_B ≥ ½|⟨[Â,B̂]⟩|
- Commutator: [Â, B̂] = ÂB̂ - B̂Â

#### 17.1.3 Time Evolution
- Schrödinger equation: iℏ d|ψ⟩/dt = Ĥ|ψ⟩
- Time evolution operator: |ψ(t)⟩ = U(t)|ψ(0)⟩, U(t) = exp(-iĤt/ℏ)
- U(t) is unitary: preserves normalization
- Heisenberg picture: operators evolve, states fixed

### 17.2 The Quantum Harmonic Oscillator

#### 17.2.1 Hamiltonian
$$\hat{H} = \frac{\hat{p}^2}{2m} + \frac{1}{2}m\omega^2\hat{x}^2 = \hbar\omega\left(\hat{a}^\dagger\hat{a} + \frac{1}{2}\right)$$

#### 17.2.2 Creation and Annihilation Operators
$$\hat{a} = \sqrt{\frac{m\omega}{2\hbar}}(\hat{x} + \frac{i\hat{p}}{m\omega}), \quad \hat{a}^\dagger = \sqrt{\frac{m\omega}{2\hbar}}(\hat{x} - \frac{i\hat{p}}{m\omega})$$
- Commutation: [â, â†] = 1
- Number operator: n̂ = â†â; eigenvalues n = 0,1,2,...
- Energy eigenvalues: Eₙ = ℏω(n + ½)
- â|n⟩ = √n|n-1⟩, â†|n⟩ = √(n+1)|n+1⟩
- Vacuum state: â|0⟩ = 0; Fock state: |n⟩ = (â†)ⁿ/√(n!) |0⟩

### 17.3 Quantization of the Electromagnetic Field

#### 17.3.1 Field Quantization
- Electromagnetic field → infinite collection of harmonic oscillators (one per mode **k**, polarization λ)
- Mode operators: â_{**k**,λ} and â†_{**k**,λ}
- Electric field operator:
$$\hat{\mathbf{E}}(\mathbf{r},t) = \sum_{\mathbf{k},\lambda} \mathcal{E}_0 \left(\hat{a}_{\mathbf{k}\lambda} e^{i(\mathbf{k}\cdot\mathbf{r}-\omega_k t)} + \hat{a}^\dagger_{\mathbf{k}\lambda} e^{-i(\mathbf{k}\cdot\mathbf{r}-\omega_k t)}\right)\boldsymbol{\epsilon}_{\mathbf{k}\lambda}$$
- Vacuum fluctuations: ⟨0|Ê²|0⟩ ≠ 0

#### 17.3.2 Fock States (Number States)
- |n⟩: exactly n photons in mode
- Non-classical: Wigner function has negative regions
- Difficult to prepare: require strongly nonlinear optical interaction
- Applications: quantum computing (dual-rail qubit), quantum communication

#### 17.3.3 Coherent States
- Eigenstate of â: â|α⟩ = α|α⟩ where α ∈ ℂ
- Expansion: |α⟩ = e^{-|α|²/2} Σₙ (αⁿ/√n!) |n⟩
- Photon number distribution: Poisson with mean n̄ = |α|²
- Closest quantum analog to classical light: laser above threshold
- Uncertainty: Δx = Δp = 1/2 (minimum uncertainty state)

#### 17.3.4 Squeezed States
- Squeezed vacuum: Δx < 1/2, Δp > 1/2 (or vice versa)
- Squeezed coherent state: displaced squeezed vacuum
- Squeezing operator: Ŝ(ξ) = exp[(ξ*â² - ξ(â†)²)/2]
- Squeezing parameter r: Δx = e^{-r}/2
- Generated by: optical parametric amplification (OPA), four-wave mixing
- Application: gravitational wave detection (LIGO uses 15 dB squeezed light)
- Application: continuous-variable quantum computing

#### 17.3.5 Phase Space Representations
- Wigner function W(x,p): quasi-probability distribution
- Husimi Q-function: always non-negative
- Glauber P-function: diagonal coherent state expansion
- Wigner function of Fock state |1⟩: has negative ring at origin (non-classical signature)

### 17.4 Quantum Entanglement

#### 17.4.1 Composite Systems and Tensor Products
- Two-qubit Hilbert space: ℋ_A ⊗ ℋ_B (dimension 4)
- Product states: |ψ⟩_A ⊗ |φ⟩_B = |ψφ⟩_{AB}
- Entangled states: cannot be written as product
- Example: (|00⟩ + |11⟩)/√2 — cannot be separated

#### 17.4.2 Bell States
$$|\Phi^+\rangle = \frac{1}{\sqrt{2}}(|00\rangle + |11\rangle)$$
$$|\Phi^-\rangle = \frac{1}{\sqrt{2}}(|00\rangle - |11\rangle)$$
$$|\Psi^+\rangle = \frac{1}{\sqrt{2}}(|01\rangle + |10\rangle)$$
$$|\Psi^-\rangle = \frac{1}{\sqrt{2}}(|01\rangle - |10\rangle)$$
- Form a complete orthonormal basis for 2-qubit space
- Maximally entangled: measuring one qubit completely determines other
- Photonic encoding: polarization, path, time-bin

#### 17.4.3 Bell Inequality Violation
- Classical bound (CHSH inequality): |E(a,b) - E(a,b') + E(a',b) + E(a',b')| ≤ 2
- Quantum maximum: 2√2 ≈ 2.83 (Tsirelson bound)
- Aspect et al. (1982): first experimental violation with photons
- Loophole-free Bell tests (2015): Hensen et al., Giustina et al., Shalm et al.
- Implication: quantum mechanics is genuinely non-local (no local hidden variables)

### 17.5 Exercises

**Mathematical:**
1. Show that the coherent state |α⟩ has Poissonian photon statistics. Compute the Mandel Q parameter and show Q = 0 for coherent state.
2. Compute the Wigner function of the vacuum state |0⟩ and the single-photon Fock state |1⟩. Show that W₁(0,0) < 0.
3. Derive that [â, â†] = 1 from the canonical commutation relation [x̂, p̂] = iℏ.
4. Show that the four Bell states form a complete orthonormal basis for the two-qubit Hilbert space.
5. For the CHSH inequality, find the measurement settings (angles) that maximize the quantum violation to 2√2.

**Conceptual:**
6. What is the fundamental difference between a coherent state and a Fock state in terms of phase uncertainty and photon number uncertainty?
7. Why does an entangled state |Φ+⟩ violate the CHSH Bell inequality, while a classical correlated state does not? What does this tell us about nature?

### 17.6 Programming Projects

**Project 17.1: Quantum State Visualizer** — Using QuTiP, compute and visualize the Wigner functions of: vacuum |0⟩, single photon |1⟩, coherent state |α=2⟩, cat state (|α⟩+|-α⟩)/N, and squeezed vacuum. Animate time evolution under the harmonic oscillator Hamiltonian.

**Project 17.2: Bell Inequality Simulation** — Simulate the CHSH experiment with entangled photon pairs (|Φ+⟩). Compute the CHSH parameter S as a function of measurement angle settings. Verify S = 2√2 at optimal angles. Add decoherence (depolarizing channel) and show that S → 2 as noise increases.

**Project 17.3: Quantum Optics Toolbox Exploration** — Using QuTiP or Strawberry Fields, prepare and measure: (a) photon-number squeezed state, (b) optical cat state, (c) two-mode entangled state from a beam splitter acting on |1,0⟩. Compute entanglement entropy.

### 17.7 Further Reading and Key Researchers
- **Textbooks:** Nielsen & Chuang, *Quantum Computation and Quantum Information* (the "bible"); Gerry & Knight, *Introductory Quantum Optics*; Walls & Milburn, *Quantum Optics* (2nd ed.)
- **Key Researchers:** Dirac (foundations), Bell (Bell inequalities), Aspect (photon entanglement tests), Haroche & Wineland (Nobel 2012, quantum optics), Glauber (Nobel 2005, quantum theory of optical coherence)
- **Key Papers:** Bell (1964), "On the Einstein-Podolsky-Rosen paradox"; Aspect et al. (1982), "Experimental Tests of Bell's Inequalities"

---

## Chapter 18: Quantum Optics — From Photon Statistics to Squeezing

### 18.1 Photon Statistics and Non-Classical Light

#### 18.1.1 The Hanbury Brown-Twiss Experiment
- Two detectors at outputs of a 50/50 beam splitter, measuring coincidences
- Second-order coherence function:
$$g^{(2)}(\tau) = \frac{\langle\hat{a}^\dagger(t)\hat{a}^\dagger(t+\tau)\hat{a}(t+\tau)\hat{a}(t)\rangle}{\langle\hat{a}^\dagger\hat{a}\rangle^2}$$
- g^(2)(0) = 1: coherent (Poissonian)
- g^(2)(0) > 1: bunching (thermal/chaotic light)
- g^(2)(0) < 1: antibunching → non-classical → single photon source
- g^(2)(0) = 0: perfect single photon source (no two-photon events)

#### 18.1.2 Single Photon Antibunching
- First demonstration: Kimble, Dagenais & Mandel (1977) — resonance fluorescence from a single atom
- Physical reason: a single emitter can only emit one photon at a time (must re-excite before next emission)
- Modern single photon sources aim for g^(2)(0) < 0.01

### 18.2 The Quantum Beam Splitter

#### 18.2.1 Quantum Beam Splitter Transformation
- Input modes â, b̂; output modes ĉ, d̂
$$\hat{c} = t\hat{a} + r\hat{b}, \quad \hat{d} = r\hat{a} + t\hat{b}$$
- Unitary: |r|² + |t|² = 1, r*t + rt* = 0
- For 50/50: t = 1/√2, r = i/√2

#### 18.2.2 Hong-Ou-Mandel (HOM) Effect
- Two identical photons (same mode in every respect) enter the two input ports of a 50/50 BS
- Input state: â†b̂†|00⟩ = |1,1⟩
- After BS: ĉ†ĉ† - d̂†d̂† (from the algebra: destructive interference for |1,1⟩_out)
- Output: always |2,0⟩ or |0,2⟩ — never |1,1⟩
- **Coincidences vanish**: HOM dip in coincidence counts vs. delay
- HOM visibility V = 1 - g^(2)(0): measures photon indistinguishability
- Critical for: photonic quantum computing (two-photon gate), quantum communication

#### 18.2.3 HOM as a Primitive for Linear Optical Quantum Computing
- Two identical photons at BS inputs → entangled output (always bunched)
- Partial distinguishability: HOM visibility < 1 → gate fidelity degradation
- Requirement for LOQC: V > 99.9% photon indistinguishability

### 18.3 Optical Parametric Processes and Squeezing

#### 18.3.1 Optical Parametric Amplification
- χ^(2) medium with pump at ωₚ → converts pump photons to pairs at ωₛ and ωᵢ
- Energy conservation: ωₚ = ωₛ + ωᵢ (signal + idler)
- OPA evolution: Ŝ(ξ) = exp[ξ*â_s â_i - ξ â†_s â†_i] (two-mode squeezing)
- SPDC (spontaneous parametric down-conversion): starts from vacuum → generates entangled pairs

#### 18.3.2 Entangled Photon Pair Generation by SPDC
- Type-I SPDC: both photons same polarization (degenerate)
- Type-II SPDC: signal and idler orthogonal polarization → polarization-entangled pairs
- Spectral and spatial correlations: pairs must be spectrally pure for HOM visibility
- Heralding: detect idler → know signal photon present
- Pair generation rate: ~10⁶ pairs/s from typical PPKTP crystal at mW pump power

#### 18.3.3 Single-Mode Squeezing
- Degenerate OPA: ωₛ = ωᵢ = ωₚ/2
- Output: squeezed vacuum or squeezed coherent state
- Squeezing levels: 15 dB (-15 dB below vacuum noise) demonstrated at NIST
- Required for: CV quantum computing, quantum-enhanced sensing

#### 18.3.4 LIGO: Squeezed Light in Practice
- 40 km arm-length Michelson interferometer: measures gravitational wave strain h ~ 10⁻²¹
- Standard quantum limit: quantum noise floor from photon shot noise
- Injected 15 dB squeezing reduces shot noise floor by √10× → extends detection range
- Nobel Prize 2017 (Weiss, Barish, Thorne) for gravitational wave detection
- Direct application of quantum optics to real-world sensing

### 18.4 Exercises

**Mathematical:**
1. Using the beam splitter transformation for a 50/50 BS, show that input state |1,1⟩ produces output |2,0⟩ and |0,2⟩ with equal probability, and never |1,1⟩ (Hong-Ou-Mandel effect).
2. Show that the single-mode squeezing operator Ŝ(r) transforms: Ŝ†(r)x̂Ŝ(r) = e^{-r}x̂ and Ŝ†(r)p̂Ŝ(r) = e^{r}p̂. Verify the Heisenberg uncertainty principle is preserved.
3. For a SPDC source generating pairs at rate R with detection efficiency η, calculate the singles rate and coincidence rate. Find the heralding efficiency η_h.
4. The HOM dip visibility V = (C_off - C_min)/C_off where C = coincidences. If g^(2)(0) = 0.02 for each single-photon source, what is the maximum achievable HOM visibility?

**Conceptual:**
5. Squeezed states beat the shot noise limit in one quadrature. Why can't squeezed states break the Heisenberg uncertainty principle?
6. Why is photon indistinguishability so critical for linear optical quantum computing? What happens to gate fidelity if two photons have 1% spectral mismatch?

### 18.5 Programming Projects

**Project 18.1: Hong-Ou-Mandel Simulation** — Simulate the HOM experiment using QuTiP. Input |1,1⟩ to a beam splitter. Compute coincidence counts as a function of temporal delay between input photons (model as Gaussian wavepackets). Plot the HOM dip and fit a Gaussian to extract photon coherence time.

**Project 18.2: SPDC Photon Pair Characterization** — Model a Type-II SPDC source using the joint spectral amplitude (JSA) formalism. Compute the JSA for a PPKTP crystal. Calculate: purity, Schmidt number, and expected HOM visibility. Optimize the pump bandwidth for maximum indistinguishability.

**Project 18.3: Squeezed Light Interferometry** — Simulate a Mach-Zehnder interferometer operating at the shot noise limit, then with 10 dB input squeezing. Compute the phase sensitivity improvement. Apply to a LIGO-like scenario: estimate the detection range improvement for gravitational waves from binary neutron star mergers.

### 18.6 Further Reading and Key Researchers
- **Textbooks:** Walls & Milburn, *Quantum Optics*; Gerry & Knight, *Introductory Quantum Optics*; Barnett, *Quantum Information*
- **Key Researchers:** Glauber (coherent states, Nobel 2005), Hong-Ou-Mandel (1987 experiment), Kimble (cavity QED, quantum networks), Furusawa (CV quantum teleportation), Lvovsky (squeezing)
- **Key Papers:** Hong, Ou & Mandel (1987), "Measurement of subpicosecond time intervals between two photons by interference," *PRL*; Tse et al. (2019), "Quantum-enhanced advanced LIGO detectors"

---

## Chapter 19: Single-Photon Sources and Detectors

### 19.1 Single-Photon Sources

#### 19.1.1 Figures of Merit
- Brightness B: photons collected per excitation pulse
- Purity: g^(2)(0) < 0.01 required for quantum computing
- Indistinguishability I: HOM visibility with identical copy
- Simultaneously: bright + pure + indistinguishable — all three are hard together
- Key trade-off: brightness requires strong coupling, purity requires spectral filtering (reduces brightness)

#### 19.1.2 Semiconductor Quantum Dots
- InGaAs/GaAs or InAs/InP quantum dots: 3D confinement → discrete energy levels → "artificial atoms"
- Emission: 900-1600 nm depending on composition and size
- Purity: g^(2)(0) < 0.001 demonstrated
- Indistinguishability: up to 99.5% demonstrated with resonance fluorescence
- Integration: embedded in photonic crystal cavities, micropillar lasers, ring resonators
- Purcell enhancement: cavity speeds up emission → narrower linewidth → more indistinguishable
- Scaling challenge: each QD is slightly different → cannot yet mass-produce identical emitters

#### 19.1.3 Color Centers in Diamond
- Nitrogen-vacancy (NV) center: N substitution + adjacent vacancy in diamond lattice
- Emission at ~637 nm (ZPL), broad phonon sideband (reduces useful fraction)
- Spin: NV^- has S=1 ground state → optically addressable spin qubit
- Silicon-vacancy (SiV), germanium-vacancy (GeV): better spectral properties, narrower ZPL
- Diamond photonic structures: nanophotonic waveguides, photonic crystal cavities
- Applications: quantum networks, single-photon sources, quantum sensing (magnetometry)

#### 19.1.4 Defects in 2D Materials
- Hexagonal boron nitride (hBN): single-photon emitters at room temperature
- Broad emission spectrum (visible to near-IR)
- Not yet fully reproducible or spectrally identical
- Exciting for ambient-temperature quantum photonics

#### 19.1.5 SPDC Sources (Probabilistic)
- High brightness, room temperature, telecom wavelength
- Key limitation: probabilistic pair generation → multi-photon events limit scalability
- Multiplexed SPDC: N sources, route heralded photon to output → near-deterministic
- PsiQuantum approach: multiplexed SPDC sources for fault-tolerant photonic quantum computing

### 19.2 Superconducting Nanowire Single-Photon Detectors (SNSPDs)

#### 19.2.1 Operating Principle
- NbN or WSi nanowire, 4-5 nm thick, ~100 nm wide, on substrate
- Cooled to 2-4 K (below T_c): superconducting state
- Photon absorbed → local hot spot → resistive barrier → current pulse → voltage output
- Quench pulse detected by cold readout electronics

#### 19.2.2 Performance Parameters
- System detection efficiency (SDE): > 98% demonstrated (MIT LL)
- Dark count rate (DCR): < 1 cps (counts per second)
- Timing jitter: 3-7 ps (world record ~3 ps)
- Reset time: ~10 ns (100 MHz count rate)
- Dead time: limited by inductance of nanowire
- Photon-number resolution: using multi-pixel arrays

#### 19.2.3 Integration with Silicon Photonics
- Waveguide-coupled SNSPD: nanowire on top of Si or SiN waveguide
- Traveling wave detector: absorption distributed over long nanowire → > 90% efficiency
- Demonstrated by Pernice group (Münster): fully integrated Si photonic chip with on-chip SNSPDs
- Key for: chip-scale quantum photonic circuits

### 19.3 Cavity Quantum Electrodynamics (CQED)

#### 19.3.1 The Jaynes-Cummings Model
- Single two-level atom in a single-mode cavity
$$\hat{H}_{JC} = \hbar\omega_c\hat{a}^\dagger\hat{a} + \frac{\hbar\omega_a}{2}\hat{\sigma}_z + \hbar g(\hat{a}^\dagger\hat{\sigma}_- + \hat{a}\hat{\sigma}_+)$$
- g: vacuum Rabi coupling strength
- Vacuum Rabi splitting: 2g in cavity transmission spectrum
- Dressed states: symmetric and antisymmetric superpositions of atom+photon

#### 19.3.2 Strong Coupling Regime
- Condition: g > κ, γ (cavity linewidth, atomic linewidth)
- Clear vacuum Rabi splitting observable
- Single photon nonlinearity: second photon blocked (photon blockade)
- Applications: quantum gate between photon and atom/qubit

#### 19.3.3 Purcell Effect (Weak Coupling)
- Purcell factor: F_P = (3/4π²)(λ/n)³ · Q/V
- Enhanced spontaneous emission rate: Γ_enhanced = F_P · Γ_free
- Increased fraction of photons emitted into the cavity mode (β-factor → 1)
- Critical for: bright single-photon sources (QD in cavity)
- Photonic crystal nanocavities: Q/V > 10⁶/λ³ → extreme Purcell enhancement

### 19.4 Exercises

**Mathematical:**
1. For a QD in a photonic crystal cavity with Q = 30,000 and mode volume V = 0.05 (λ/n)³, calculate the Purcell factor. If the free-space lifetime is 1 ns, what is the cavity-enhanced lifetime? What fraction of photons goes into the cavity mode?
2. An SNSPD has SDE = 90%, DCR = 100 cps, and jitter = 50 ps. For a quantum photonic circuit operating at 1 GHz clock rate with 1 photon per clock expected, compute: (a) missed detection rate, (b) dark count contamination, (c) timing resolution.
3. Derive the eigenvalues of the Jaynes-Cummings Hamiltonian for the one-excitation subspace. Show that the energy splitting is 2ℏg (vacuum Rabi splitting).
4. For a multiplexed SPDC source with N = 100 memories and pair generation probability p = 0.1 per clock, compute the single-photon output probability and the residual multi-photon probability.

**Conceptual:**
5. Why must single-photon sources be both pure (g^(2)(0) → 0) and indistinguishable (HOM visibility → 1) for photonic quantum computing? How do these requirements conflict?
6. Compare SNSPDs and SPADs for use in a room-temperature vs. cryogenic photonic quantum processor. When is each preferred?

### 19.5 Programming Projects

**Project 19.1: Purcell Enhancement Calculator** — For a range of photonic cavity designs (Fabry-Pérot, microsphere, photonic crystal), compute the Purcell factor, β-factor, and expected g^(2)(0) for a QD emitter as a function of Q and V. Optimize the cavity design for maximum source brightness with g^(2)(0) < 0.01.

**Project 19.2: Jaynes-Cummings Dynamics Simulator** — Using QuTiP, simulate the dynamics of the Jaynes-Cummings model. Show: (a) vacuum Rabi oscillations, (b) collapse and revival for coherent state input, (c) photon blockade (two-photon suppression). Animate the Wigner function of the cavity field.

**Project 19.3: SNSPD Detection Efficiency Calculator** — Model a waveguide-coupled SNSPD. Compute detection efficiency as a function of: nanowire length, optical absorption coefficient (function of wavelength), meandering fill factor, and coupling loss. Optimize the design for maximum SDE at 1550 nm.

### 19.6 Further Reading and Key Researchers
- **Key Researchers:** Kimble (CQED, quantum networks), Mookherjee & Shields (QD sources), Zwiller (NV centers), Marsili & Verma (SNSPDs), Pernice (integrated SNSPDs), Vučković (photonic crystal single-photon sources)
- **Key Papers:**
  - Purcell (1946), "Spontaneous emission probabilities at radio frequencies" *PR*
  - Michler et al. (2000), "A quantum dot single-photon turnstile device," *Science*
  - Marsili et al. (2013), "Detecting single infrared photons with 93% system efficiency," *Nature Photonics*
  - Tomm et al. (2021), "A bright and fast source of coherent single photons," *Nature Nanotechnology*

---

## Chapter 20: Linear Optical Quantum Computing

> *Can you build a universal quantum computer using only linear optics — beam splitters, phase shifters, and photodetectors? For years, the answer seemed clearly "no": linear optics cannot create the nonlinear photon-photon interaction needed for deterministic two-qubit gates. Then, in 2001, Knill, Laflamme, and Milburn published a bombshell result: using only linear optics plus single photon sources and photon-number-resolving detectors, you can build a near-deterministic two-qubit gate using quantum teleportation — and therefore, a scalable universal quantum computer. It was the founding document of the entire field of photonic quantum computing.*

### 20.1 The Qubit in Photonics

#### 20.1.1 Dual-Rail Encoding
- Qubit: one photon in one of two modes (paths or polarizations)
- |0⟩_L = |1⟩_a|0⟩_b (photon in mode a)
- |1⟩_L = |0⟩_a|1⟩_b (photon in mode b)
- Logical basis: {|10⟩, |01⟩} in two-mode Fock space
- Superposition: (|10⟩ + |11⟩)/√2 — photon delocalized over two modes

#### 20.1.2 Single-Qubit Gates with Linear Optics
- Beam splitter + phase shifter → arbitrary SU(2) rotation on dual-rail qubit
- Hadamard gate H: 50/50 beam splitter
- Z gate: phase shifter in one arm
- X gate: swap of modes (crossed waveguide)
- Universal single-qubit gate: MZI with two phases

#### 20.1.3 The CNOT Problem
- CNOT requires photon-photon interaction
- Linear optics: each photon evolves independently → no direct photon-photon coupling
- Measurement-induced nonlinearity: post-selection on detector outcomes

### 20.2 The KLM Protocol

#### 20.2.1 The Key Insight: Measurement Creates Effective Nonlinearity
- Ancilla photons + beam splitter network + post-selection on measurement outcomes
- Success probability of nondeterministic CZ gate: 1/4 (original KLM)
- Boosted to 1 - 1/n² with n ancilla photons
- Quantum gate teleportation: offline prepare resource state → probabilistic gate succeeds with higher probability

#### 20.2.2 The Nondeterministic CZ Gate
- Input: two dual-rail qubits (4 modes total) + ancilla state
- Linear optical network (specified MZI mesh)
- Post-select on ancilla measurement outcome: success → applied CZ gate
- Failure: must repeat (requires quantum memory or feed-forward)

#### 20.2.3 Gate Teleportation and Boosting
- Prepare "cat states" offline with probability p_cat
- Use teleportation to apply gate: success probability boosted
- With enough ancilla photons: probability → 1 (resource intensive)
- Key tradeoff: circuit depth vs. success probability vs. ancilla resource cost

### 20.3 Measurement-Based Quantum Computing (MBQC)

#### 20.3.1 Cluster States
- Graph state |G⟩: vertices = qubits, edges = CZ gates applied
- Preparation: start with |+⟩^⊗N = Hadamard on all qubits, apply CZ for each edge
- Linear optical cluster state generation: type-II fusion of photon pairs
- Cluster state is a universal resource for quantum computation

#### 20.3.2 One-Way Quantum Computer
- Computation proceeds by sequential single-qubit measurements
- Measurement basis determines the computation
- Feed-forward corrections using Pauli byproduct operators
- Advantage: measurements are naturally irreversible → no need for reversible gates

#### 20.3.3 Fusion-Based Quantum Computing (FBQC)
- Bartolucci et al. (PsiQuantum, 2021): resource-efficient photonic MBQC
- Fuse small resource states (4-6 photons) using Bell measurements
- Fault-tolerant surface code on fused graph
- Loss thresholds: ~10% per photon (achievable with SNSPDs + active switching)
- PsiQuantum's architecture uses this approach with silicon photonics at cryogenic temperatures

### 20.4 Boson Sampling

#### 20.4.1 The Aaronson-Arkhipov Result
- Feed n photons into N-mode linear optical network (U is random Haar-random)
- Sample from the output photon-number distribution
- Computing this distribution classically: requires computing permanents of submatrices of U
- Permanent computation: #P-hard (believed harder than NP)
- Quantum boson sampler: outputs samples in polynomial time → evidence of quantum advantage

#### 20.4.2 Gaussian Boson Sampling (GBS)
- Input: squeezed states (Gaussian states) instead of Fock states
- Output: harder to classically simulate than original boson sampling
- Advantage: Gaussian state inputs are much easier to prepare experimentally
- Xanadu's Borealis (2022): 216 modes, ~5 orders of magnitude beyond classical simulation
- Applications beyond advantage demonstration: graph problems, quantum chemistry

#### 20.4.3 Experimental Demonstrations
- Aaronson/Arkhipov boson sampling: Broome et al. (2013), Spring et al. (2013), Crespi et al. (2013) — proof-of-principle
- Jiuzhang (Pan group, USTC 2020): 76-photon boson sampling, classical simulation estimated 2.5 billion years
- Jiuzhang 2.0 (2021): 113 photons, 144 modes
- Xanadu Borealis (2022): time-domain multiplex GBS, reconfigurable

### 20.5 Quantum Error Correction for Photonic Systems

#### 20.5.1 Photon Loss — The Dominant Error
- Photon absorbed or lost → qubit in |vac⟩ (not |0⟩ or |1⟩) — erasure error
- Loss rate: waveguide loss + coupler loss + detection inefficiency + source inefficiency
- Target: total photon survival probability > 90%
- Current state-of-art: ~85% photon survival probability

#### 20.5.2 Photonic Qubits and Error Codes
- Dual-rail: photon loss → detectable erasure (distinguishable from |0⟩ or |1⟩)
- Erasure codes for photonic qubits: Grassl-Beth-Pellizzari codes
- GKP encoding: encode qubit in oscillator (continuous variable), correctable with squeezing
- Surface codes on photonic graph states

#### 20.5.3 PsiQuantum's Fault-Tolerant Roadmap
- Goal: millions of physical qubits → thousands of logical qubits
- Physical layer: Si photonics with SNSPDs (cryo), III-V lasers
- Logical layer: FBQC with surface codes
- Target: topological fault threshold ~1% error per gate → need ~10¹⁰ photons/second from sources
- Projected machine: 1 million physical qubit equivalent, modular cryo units

### 20.6 Exercises

**Mathematical:**
1. Show that an arbitrary SU(2) rotation on a dual-rail photonic qubit can be achieved with a single MZI (two beam splitters + one phase shifter).
2. Derive the success probability of the KLM nondeterministic CZ gate (P = 1/4) by computing the post-selection outcomes for a specific ancilla state and optical network.
3. For a cluster state of N qubits on a 2D square lattice, count the number of Bell measurements required to complete a quantum computation of depth d. Compare to the circuit model gate count.
4. In Gaussian boson sampling with n squeezed inputs and N output modes, show that the complexity of classically computing output probabilities scales as O(n² · 2^n), confirming hardness for large n.
5. A photonic qubit has photon loss probability ε = 0.05 per component, and the circuit has depth 100 with 10 components per layer. Compute the total photon survival probability and estimate whether fault-tolerance is achievable.

**Conceptual:**
6. Why does photon loss create an "erasure" error rather than a "Pauli" error in the dual-rail qubit encoding? Why is erasure error actually easier to correct than Pauli error?
7. Compare boson sampling and universal quantum computing: what is the computational relationship between them? Is a boson sampler useful for any practical problem?

### 20.7 Programming Projects

**Project 20.1: Linear Optical CNOT Gate Simulation** — Using the KLM scheme (or a simplified version), implement the nondeterministic CZ gate in simulation. Run 10,000 trials with randomly chosen ancilla measurement outcomes. Verify that success outcomes (probability 1/4) give the correct CZ operation and failure outcomes give garbage.

**Project 20.2: Boson Sampling Simulator** — Implement a classical boson sampling simulator for small n (n ≤ 20). Use Ryser's algorithm for permanent computation. Simulate a random Haar-random interferometer and sample from the output distribution. Compare classical simulation time vs. n.

**Project 20.3: Gaussian Boson Sampling with Strawberry Fields** — Using Xanadu's Strawberry Fields library, implement a GBS circuit on N=10 modes with r=1.0 squeezing. Sample 1000 output patterns. Compute the two-point correlations and verify the Hafnian formula for probabilities.

**Project 20.4: Cluster State Measurement-Based Computation** — Build an MBQC simulator: (a) generate a 1D cluster state of N=8 qubits, (b) implement a logical Hadamard gate via sequential measurements, (c) implement a quantum teleportation circuit using MBQC, (d) verify output fidelity.

### 20.8 Further Reading and Key Researchers
- **Key Researchers:**
  - KLM — Knill, Laflamme, Milburn (2001 protocol)
  - O'Brien (Bristol) — linear optical quantum computing experiments
  - Pan (USTC) — Jiuzhang boson sampling
  - Aaronson (UT Austin) — boson sampling complexity
  - Bartolucci et al. (PsiQuantum) — fusion-based QC
  - Rudolph (PsiQuantum) — photonic quantum computing architecture
- **Key Papers:**
  - Knill, Laflamme & Milburn (2001), "A scheme for efficient quantum computation with linear optics," *Nature*
  - Aaronson & Arkhipov (2013), "The Computational Complexity of Linear Optics," *Theory of Computing*
  - Zhong et al. (2020), "Quantum computational advantage using photons," *Science* (Jiuzhang)
  - Bartolucci et al. (2021), "Fusion-based quantum computation"
  - Madsen et al. (2022), "Quantum computational advantage with a programmable photonic processor (Borealis)," *Nature*

---

## Chapter 21: Continuous-Variable and Xanadu's Quantum Computing

### 21.1 CV Quantum Information

#### 21.1.1 Quadrature Variables
- Quadrature operators: x̂ = (â + â†)/√2, p̂ = (â - â†)/(i√2)
- [x̂, p̂] = i (ℏ=1 units)
- Coherent state: displaced origin in phase space ⟨x̂⟩ = Re(α), ⟨p̂⟩ = Im(α)
- Squeezed state: Δx < 1/√2 (below vacuum noise)

#### 21.1.2 Gaussian States and Operations
- Gaussian states: fully described by first and second moments (μ, σ)
- Gaussian operations: symplectic transformations of (x̂, p̂) → preserve Gaussian character
- Examples: displacement, squeezing, beam splitter, phase rotation
- Efficient classical simulation of Gaussian circuits (no quantum advantage)
- Non-Gaussian elements required for quantum advantage

#### 21.1.3 GKP Encoding
- Gottesman-Kitaev-Preskill encoding: qubit in oscillator
- Logical |0⟩_L: grid of spikes in position quadrature at 0, ±2√π, ±4√π, ...
- Logical |1⟩_L: grid shifted by √π
- Correction: small displacements detectable and correctable
- GKP states require high squeezing to prepare (>13 dB) — recently achieved

### 21.2 Xanadu's Photonic Quantum Computer

#### 21.2.1 The Borealis Architecture
- Time-domain multiplexing: fiber loop delays create multiple time bins
- One squeezed source → time-multiplexed into N modes via fiber switching
- Programmable beam splitters: electro-optic modulators switch between time bins
- Measurement: homodyne or heterodyne (Gaussian measurements)
- 216 modes, fully programmable, room-temperature squeezed light sources

#### 21.2.2 PennyLane Software Framework
- Open-source ML framework for quantum-classical hybrid computing
- Differentiable quantum programming: compute gradients of quantum circuits
- Supports Xanadu hardware, simulation backends, and IBM/Google quantum devices
- Key for: quantum machine learning on photonic hardware

#### 21.2.3 Quantum Machine Learning with CV
- Continuous-variable quantum neural networks: trainable Gaussian + non-Gaussian layers
- Quantum generative models (QGAN): generate quantum states matching a target distribution
- Quantum kernel methods: inner products of quantum feature maps
- Limitation: classically efficient simulation of all-Gaussian circuits limits advantage

### 21.3 Exercises

1. Show that a beam splitter of reflectivity R acts as a symplectic rotation in phase space. What is the symplectic matrix?
2. For a GKP qubit with squeezing parameter Δ = 0.1 (momentum quadrature width), compute the logical error probability for a Gaussian displacement error of σ = Δ/2.
3. Design a CV quantum gate set for a photonic computer: write down the Hamiltonian generators for the displacement gate D(α), the squeezing gate S(r), and the cubic phase gate V(γ). Show that these form a universal gate set.

### 21.4 Programming Projects

**Project 21.1: Strawberry Fields GBS Algorithm** — Using PennyLane + Strawberry Fields, implement a GBS circuit for the graph maximum clique problem. Encode a random graph as a GBS adjacency matrix. Sample the GBS output and use samples to estimate the maximum clique. Compare to classical brute-force.

**Project 21.2: CV-QNN Training** — Implement a continuous-variable quantum neural network in PennyLane. Train it to approximate a target function f(x) = sin(2πx). Use the parameter-shift rule to compute gradients. Compare convergence to a classical neural network.

### 21.5 Further Reading and Key Researchers
- **Key Researchers:** Weedbrook, Pirandola, Braunstein (CV quantum information); Killoran, Bromley, Arrazola (Xanadu); Lloyd & Braunstein (original CV QC proposal)
- **Key Papers:** Lloyd & Braunstein (1999), "Quantum computation over continuous variables"; Madsen et al. (2022), "Quantum computational advantage with a programmable photonic processor," *Nature*

---

## Chapter 22: Quantum Communication and the Quantum Internet

### 22.1 Quantum Key Distribution (QKD)

#### 22.1.1 BB84 Protocol
- Alice prepares photons in one of 4 states: |H⟩, |V⟩, |+⟩, |-⟩
- Bob measures in Z or X basis (randomly)
- Sifted key: outcomes where both chose same basis
- Error rate estimation: reveals eavesdropping
- Privacy amplification: distill secure key from sifted key
- Security proof: information-theoretically secure against any eavesdropper

#### 22.1.2 Photonic QKD Implementations
- Weak coherent pulse (WCP) sources: attenuated laser → approximate single photons
- Decoy-state QKD: use multiple power levels to detect photon-number splitting attacks
- True single-photon QKD: QD source + SNSPD → highest security
- Maximum secure distance: ~300 km (current record with ultra-low-loss fiber + SNSPDs)
- Twin-field QKD: overcome PLOB bound, extend to 600+ km
- Satellite QKD: Micius satellite (Pan group, 2017): QKD over 1200 km

#### 22.1.3 QKD Integration with Classical Networks
- Commercial QKD: ID Quantique, Toshiba, QuantumCTek
- ETSI QKD standards
- Hybrid QKD + post-quantum cryptography
- Practical limitations: distance, key rate, trusted node requirement

### 22.2 Quantum Repeaters

#### 22.2.1 No-Cloning Theorem
- Impossible to copy an unknown quantum state
- Therefore: optical amplifiers cannot be used to extend quantum communication range
- Repeater principle: entanglement swapping extends entanglement over distance

#### 22.2.2 Quantum Memory
- Requirement: store photonic qubit as matter qubit (atom, NV center, rare-earth ion)
- Atomic frequency comb (AFC) memory: spectral hole burning in rare-earth doped crystal
- Efficiency > 90%, storage time ~ ms to s
- Bandwidth: GHz for telecom wavelengths (needed for high key rate)

#### 22.2.3 Entanglement Swapping
- Node A shares entanglement with middle node M; node B shares with M
- Middle node performs Bell measurement on its two qubits
- Result: A and B are now entangled (without direct interaction)
- Teleportation of entanglement across a chain of repeater nodes

#### 22.2.4 The Quantum Internet Roadmap
- Stage 1: QKD networks (deployed today)
- Stage 2: Entanglement distribution between distant nodes
- Stage 3: Remote entanglement with quantum memories
- Stage 4: Quantum network with logical qubits and error correction
- Stage 5: Full quantum internet: distributed quantum computing
- Timeline: Stage 2-3 demonstrators within 5-10 years; full quantum internet: 20-30 years

### 22.3 Exercises
1. Prove that the BB84 protocol is secure against individual attacks using the Csiszár-Körner theorem.
2. For a QKD system with detector efficiency η = 0.9, dark count rate d = 10⁻⁶ per pulse, and fiber attenuation 0.2 dB/km, compute the secret key rate at 100 km distance using the BB84 decoy-state protocol.
3. Calculate the PLOB bound (private capacity of a lossy channel) at 300 km with 0.2 dB/km fiber. Compare to the twin-field QKD rate.

### 22.4 Programming Projects

**Project 22.1: BB84 QKD Simulator** — Implement the full BB84 protocol: Alice prepares qubits, Eve intercepts with probability p_E, Bob measures. Compute: sifted key rate, quantum bit error rate (QBER), and final secure key rate after error correction and privacy amplification. Plot secure key rate vs. QBER.

**Project 22.2: Entanglement Swapping Simulator** — Simulate a 3-node quantum repeater chain. Alice-M₁ and M₁-Bob share Bell pairs (with fidelity F₀). M₁ performs a Bell measurement and Alice-Bob entanglement is swapped. Compute the resulting fidelity as a function of initial fidelity and include depolarizing noise in the quantum memories.

### 22.5 Further Reading and Key Researchers
- **Key Researchers:** Bennett & Brassard (BB84), Ekert (E91), Pan (quantum communication over satellite), Gisin (ID Quantique), Kimble (quantum network vision), Wehner (quantum internet roadmap)
- **Key Papers:** Bennett & Brassard (1984), "Quantum cryptography: public key distribution and coin tossing"; Pan et al. (2017), "Satellite-based entanglement distribution over 1200 kilometers," *Science*

---

# UNIT VIII: FABRICATION AND SIMULATION TOOLS

> *Theory without fabrication is philosophy. This unit bridges the gap between the physics and mathematics of the previous units and the practical reality of building photonic computing systems. You will learn how photons are trapped inside silicon chips, how chips go from design files to working devices, and how to use the computational tools that make both design and discovery possible. Whether your goal is to publish a research paper on a new photonic component or to build a product, this unit gives you the skills.*

---

## Chapter 23: Nanofabrication for Photonics

### 23.1 Cleanroom Fundamentals
- 23.1.1 Contamination and cleanroom classification (ISO 1 through ISO 8)
- 23.1.2 The silicon wafer: crystal growth (Czochralski), doping, SOI fabrication
- 23.1.3 Photolithography: exposure, developer, resist
- 23.1.4 Electron-beam lithography: direct write, 5-10 nm resolution
- 23.1.5 Extreme UV (EUV) lithography at 13.5 nm: enabling sub-7 nm features
- 23.1.6 Etching: isotropic (wet HF) vs. anisotropic (RIE, ICP-RIE) — key for waveguide sidewalls
- 23.1.7 Deposition: PECVD for SiN cladding, ALD for Al₂O₃ gates
- 23.1.8 Chemical mechanical planarization (CMP)
- 23.1.9 Ion implantation for PN junction formation in modulators

### 23.2 Silicon Photonics Foundry Process

#### 23.2.1 Standard SOI Process Flow
- SOI wafer → photoresist → e-beam/DUV lithography → ICP etch → SiO₂ cladding deposition → metallization
- Deep UV lithography at 193 nm: standard at AIM, AMF, IMEC
- 6-inch and 8-inch wafers: cost per die comparison
- PDK-based design: only components in the PDK guaranteed to work

#### 23.2.2 Multi-Project Wafer (MPW) Runs
- Share a wafer with other designs: \$15,000-50,000 per run
- Turnaround time: 4-9 months
- Essential for academic research prototyping
- Providers: IMEC, AMF, AIM Photonics, CMC Microsystems (Canada)

#### 23.2.3 Foundry-Grade Quality and Yield
- Propagation loss variations across wafer
- Critical dimension (CD) variations: ±5 nm typical
- Yield models: Poisson defect density model
- Process characterization and monitoring (PCM) structures

### 23.3 III-V Integration
- 23.3.1 Why III-V? Direct bandgap → efficient lasers (GaAs, InP, GaN)
- 23.3.2 Flip-chip bonding: III-V laser bonded face-down to Si photonic chip
- 23.3.3 Micro-transfer printing: pick-and-place at wafer scale
- 23.3.4 Heterogeneous bonding: direct bonding of InP epitaxial layers to SOI
- 23.3.5 Monolithic III-V on silicon: epitaxial growth on Si via Ge buffer (defect challenge)

### 23.4 Packaging and Testing
- 23.4.1 Fiber-chip coupling: lensed fiber, grating coupler arrays
- 23.4.2 Electrical probing: RF probe tips for modulator bandwidth measurement
- 23.4.3 Photonic chip testing: swept-wavelength measurements, eye diagram testing
- 23.4.4 Electronic-photonic co-packaging: wire bonding, flip-chip to ASIC
- 23.4.5 Thermal management: heat sink, thermoelectric cooler for wavelength stability
- 23.4.6 Reliability and burn-in testing: MTTF for laser and modulator

### 23.5 Exercises
1. Calculate the minimum feature size achievable with 193 nm DUV lithography using numerical aperture NA = 0.75 and k₁ factor = 0.4: CD = k₁λ/NA.
2. A silicon photonic chip has propagation loss 2 dB/cm and must carry 1550 nm light through a 10 cm waveguide. What fraction of input power reaches the end?
3. Design a grating coupler for 1550 nm targeting 10° angle: find the period Λ given n_eff = 2.85, n_clad = 1.0, and the grating equation n_eff - n_clad sin θ = mλ/Λ.

### 23.6 Programming Projects

**Project 23.1: GDS Layout Generator** — Using gdsfactory in Python, design a complete photonic chip layout including: grating couplers, 1×4 MMI splitter, ring resonator, directional coupler test structures, and metal heaters. Add automatic routing of optical waveguides and generate the final GDS file.

**Project 23.2: Process Tolerance Monte Carlo** — Model critical dimension variations (±10 nm) in a directional coupler. Using Monte Carlo simulation (N=1000 trials), compute the distribution of coupling ratio at 1550 nm. Determine what CD variation is acceptable for ±5% coupling variation.

**Project 23.3: Yield Estimator** — Implement a Poisson yield model for a silicon photonic chip. Given die area 10 mm², critical defect density D₀ = 0.1/cm², and three critical process steps, compute the expected yield. Optimize die size vs. number of dies per wafer to maximize revenue.

### 23.7 Further Reading and Key Researchers
- **Textbooks:** Jalali & Fathpour, *Silicon Photonics* (2006); Pavesi & Lockwood, *Silicon Photonics* Vols. I-III; Chrostowski & Hochberg, *Silicon Photonics Design* (Cambridge, free PDF)
- **Key Researchers:** Jalali (silicon photonics, UCLA), Lipson (Cornell/Columbia), Reed (Southampton), Atabaki (MIT-3D integration)
- **Online:** AIM Photonics Academy training; edX silicon photonics courses; ePIXfab training

---

## Chapter 24: Simulation and Design Tools

### 24.1 Numerical Electromagnetic Methods

#### 24.1.1 Finite-Difference Time-Domain (FDTD)
- Yee cell: E and H fields on offset grids
- Time stepping: leapfrog algorithm
- Stability condition: Courant condition Δt ≤ Δx/(c√3)
- Perfectly matched layer (PML) absorbing boundaries
- Convergence: field solutions converge as Δx → 0
- Software: Lumerical FDTD (commercial), Meep (open source, MIT)
- Typical run: 3D, 10×10×10 μm volume, 1000 time steps → several hours on CPU; minutes on GPU

#### 24.1.2 Finite Element Method (FEM)
- Triangular/tetrahedral mesh: adapts to curved boundaries
- Weak formulation of Maxwell's equations
- Frequency-domain solver: eigenvalue problem for modes
- Software: COMSOL Multiphysics, Ansys HFSS, FEniCS (open source)
- Mode solving: FEM computes guided mode profiles and effective indices

#### 24.1.3 Eigenmode Expansion (EME)
- Decompose waveguide into sections, solve modes in each
- Mode overlap integrals for coupling between sections
- Ideal for: long tapers, gratings, arrayed waveguide gratings
- Software: Lumerical MODE, Photon Design FIMMWAVE

#### 24.1.4 Beam Propagation Method (BPM)
- Paraxial approximation: valid for slowly varying structures
- Efficient for long structures but inaccurate for sharp bends or high index contrast
- Applications: fiber-to-chip tapers, AWG design

### 24.2 Photonic Circuit Simulation

#### 24.2.1 Transfer Matrix Method
- Each component: S-matrix (scattering matrix) relating input and output field amplitudes
- Cascade: multiply transfer matrices
- Fast: analytical for idealized components
- Compact model extraction from FDTD simulation → S-parameters → circuit simulator

#### 24.2.2 Photonic SPICE
- Analogous to SPICE for electronics: nodal analysis of photonic circuits
- Lumerical Interconnect: time-domain circuit simulator for photonic systems
- IPKISS (Luceda): layout → circuit simulation automation
- Enables simulation of complete photonic processors with thousands of components

#### 24.2.3 Photonic Processor Simulation Stack
- Full stack: FDTD (component) → S-matrix extraction → circuit model → system-level simulation
- Example: simulate a 64×64 MZI mesh including all component models, loss, thermal crosstalk

### 24.3 Inverse Design

#### 24.3.1 Adjoint Method for Photonic Optimization
- Objective function: F = F(fields) that depends on the design parameters ε(r)
- Forward simulation: compute E(r) for given ε(r)
- Adjoint simulation: one additional FDTD simulation gives ∂F/∂ε(r) for all r simultaneously
- Cost: two FDTD runs regardless of number of design parameters
- Application: optimize waveguide coupler, splitter, taper, grating coupler

#### 24.3.2 Topology Optimization
- Optimize pixel-by-pixel (ε = ε_Si or ε_SiO₂) using adjoint gradients
- Penalty: binarization constraint
- Result: "inverse-designed" compact device (often looks like random noise but works)
- Demonstrated devices: ultra-compact beam splitter (2×2 μm²), polarization splitter, wavelength demultiplexer

#### 24.3.3 Deep Learning for Photonic Inverse Design
- Train neural network to predict device performance from geometry
- Inverse model: predict geometry from target performance
- Tandem network: avoid mode collapse in inverse prediction
- Physics-informed neural networks (PINNs): embed Maxwell's equations in the loss function
- Speed: 1000× faster forward prediction than FDTD → useful for iterative optimization

### 24.4 Exercises

**Mathematical:**
1. Derive the Courant stability condition for the 1D FDTD algorithm. Show that it follows from the requirement that information does not travel faster than c in one time step.
2. Implement the 1D FDTD update equations for E_x and H_y. Show that the leapfrog scheme is time-reversible.
3. For a silicon photonic Y-junction (2×2 μm design region), formulate the adjoint optimization problem: write down the objective function for a 50/50 power splitter and the gradient formula.

### 24.5 Programming Projects

**Project 24.1: Meep FDTD Waveguide Simulation** — Using the Meep Python API, simulate: (a) mode profile of a 450 nm Si waveguide, (b) transmission spectrum of a directional coupler, (c) ring resonator transmission with varying coupling gap. Extract S-parameters and fit to analytical models.

**Project 24.2: Adjoint Optimization of a Y-Junction** — Using Meep + autograd (or the FDTD sensitivity analysis), design an ultra-compact 50/50 Y-junction splitter. Optimize the 2×2 μm design region for maximum transmission balance. Compare your design to the traditional adiabatic Y-junction.

**Project 24.3: MZI-Based Photonic Processor in Lumerical Interconnect** — Build a 4×4 MZI mesh circuit in Lumerical Interconnect (or a Python transfer-matrix simulator). Simulate transmission for a programmed unitary matrix. Add phase noise and measure fidelity. Simulate a full MVM operation for a neural network inference step.

**Project 24.4: Deep Learning Surrogate for Waveguide Mode Solver** — Generate a dataset of 10,000 (width, height) → (n_eff, n_g) pairs using Meep. Train a neural network surrogate. Evaluate prediction accuracy. Use the surrogate to rapidly scan a design space that would take days with FDTD.

### 24.6 Further Reading and Key Researchers
- **Textbooks:** Taflove & Hagness, *Computational Electrodynamics: The FDTD Method* (3rd ed.); Jin, *The Finite Element Method in Electromagnetics*
- **Software Docs:** Meep documentation (meep.readthedocs.io); gdsfactory documentation; Lumerical Knowledge Base
- **Key Researchers:** Taflove (FDTD), Joannopoulos (MIT photonic simulations), Piggott (photonic inverse design, Google), Lu & Vučković (adjoint optimization)
- **Key Papers:** Piggott et al. (2015), "Inverse design and demonstration of a compact and broadband on-chip wavelength demultiplexer," *Nature Photonics*

---

# UNIT IX: BENCHMARKING AND THE COMPUTING LANDSCAPE

## Chapter 25: Electronic-Photonic Co-Design and Benchmarking

### 25.1 The Role of Electronics

#### 25.1.1 Electronic-Photonic Interface Components
- DAC: digital signal → analog drive voltage for modulator (8-12 bit at >50 GHz)
- ADC: photodetector output → digital number (8-12 bit at >50 GHz)  
- TIA: transimpedance amplifier converts photocurrent to voltage
- Clock distribution: frequency-locking modulators and detectors
- Electronic power: dominates total system power budget at current device efficiencies

#### 25.1.2 Co-Design Methodology
- Start with algorithm: what computation is required?
- Partition: what is photonics best for? (linear algebra, high bandwidth)
- What is electronics best for? (nonlinearity, memory, control)
- Hardware-software co-design: jointly optimize algorithm and hardware
- Example: photonic MVM + electronic accumulation + digital softmax → neural network layer

#### 25.1.3 Hybrid Electronic-Photonic Integration
- 2.5D: photonic chip and electronic ASIC in same package, wire bonded
- 3D: electronic chip stacked on photonic chip (TSMC CoWoS-like)
- Monolithic: photonic and electronic layers in same fab process (IBM, Intel)
- Key metric: interconnect bandwidth between photonic and electronic dies

### 25.2 Benchmarking Photonic AI Accelerators

#### 25.2.1 Standard Metrics
- Throughput: GMAC/s (giga multiply-accumulate per second) or TOPS
- Energy efficiency: TOPS/W (peak) and TOPS/W (typical workload)
- Effective bit precision: ENOB from analog noise floor
- Latency: time from input to output
- Reconfigurability: time to change weights

#### 25.2.2 Fair Comparison with GPUs
- A100 GPU: 312 TOPS (TF16), 80 GB HBM, 400 W → ~0.78 TOPS/W
- H100 GPU: 1979 TOPS (INT8), 700 W → ~2.8 TOPS/W
- Photonic claims: up to 100 TOPS/W (at 4-bit precision)
- Caveat: photonic efficiency claims often neglect electronic overhead (DAC/ADC, control)
- FAIR benchmark: total system TOPS/W including all support electronics

#### 25.2.3 Where Photonics Wins
- Fixed-weight inference at low latency: optical path = constant time, no scheduling
- Large batch matrix-vector multiply at low energy
- Bandwidth-limited tasks: WDM parallelism avoids bandwidth bottleneck
- High-speed sensing → computing (LIDAR, radar): ADC avoided if analog compute used directly

#### 25.2.4 Where Electronics Wins (Today)
- High precision (>12 bit): digital arithmetic has guaranteed precision
- Nonlinear activation functions
- Reconfiguration: changing weights (heaters: μs; PCM: ns but limited endurance)
- Memory: photonics has no equivalent of SRAM, DRAM, or HBM
- General purpose: logic, branches, control flow

### 25.3 Exercises

1. Compare the energy per floating-point multiply-accumulate (MAC) for: (a) NVIDIA H100 GPU, (b) a photonic MZI mesh at 1 mW average optical power per neuron, (c) human brain synapse (~10 fJ estimated). Discuss the gap and what it would take to close it.
2. A photonic AI accelerator has 64 optical inputs, 64 outputs, processes 1 MAC per clock cycle, and runs at 50 GHz clock rate. What is its peak throughput in TOPS? If each MZI has 10 mW heater power, what is the heating power for the 64×64 mesh?
3. For a photonic reservoir computer benchmarked on the NARMA-10 task: accuracy = 97%, latency = 10 ns, power = 5 mW. Compare to a digital LSTM of equivalent capacity running on an embedded MCU (accuracy = 97%, latency = 1 μs, power = 100 mW).

### 25.4 Programming Projects

**Project 25.1: Photonic AI Accelerator Design Space Explorer** — Build a parameterized model of a WDM broadcast-and-weight photonic matrix multiplier. Sweep: N_wavelengths (8-128), detector count (8-128), optical power per channel. Compute throughput (GMAC/s) and power (W) for each point. Plot the Pareto frontier of throughput vs. efficiency. Compare to the GPU H100 at the same model size.

**Project 25.2: Full System Power Budget** — For a photonic AI chip performing 64×64 matrix-vector multiplication at 50 GHz clock rate: compute the power consumed by each component (laser, MZMs, ring heaters, TIA, ADC, DAC, digital control). Identify the dominant power consumer. Propose design changes to improve total system efficiency by 2×.

### 25.5 Further Reading and Key Researchers
- **Key Researchers:** Miller (Stanford, interconnect limits), Hamerly (MIT/NTT), Garg & Nanjunda (Lightmatter), Shastri (Princeton), Jalali (UCLA, photonic ADC)
- **Key Papers:** Miller (2017), "Attojoule Optoelectronics for Low-Energy Information Processing and Communications"; Hamerly et al. (2019), "Scaling Advantages of All-to-All Connectivity in Physical Annealers"

---

# UNIT X: COMPANIES, RESEARCH GROUPS, AND THE FRONTIERS OF RESEARCH

> *We close where the textbooks end and the open problems begin. The chapters in this unit are less tutorial and more landscape — a view from the research frontier, where the equations are still being written, the experiments are still failing in interesting ways, and the companies are racing to build something that has never existed before. Read this unit as you would a scientific review article: as a map of a territory still being charted, not as a description of settled land.*

---

## Chapter 26: The Photonic Computing Industry and Research Ecosystem

### 26.1 Photonic AI Hardware Companies

#### 26.1.1 Lightmatter (Cambridge, MA)
- Founded 2017 by graduates of MIT photonics group (Hamerly, Harris, Vu)
- Products: Mars (photonic matrix processor), Envise (full AI accelerator), Passage (photonic network-on-chip)
- Mars: first silicon photonic AI chip for edge inference
- Passage: photonic switch interconnect for multi-chip AI systems
- Funding: Series C ($154M, 2023), valuation >$1B

#### 26.1.2 Lightelligence (Boston/Shanghai)
- PACE chip: first commercial silicon photonic AI chip
- Demonstrates ONN inference on standardized workloads
- Published performance data: 2 TOPS with low power vs. digital ASIC baseline

#### 26.1.3 Luminous Computing
- Large-scale optical tensor processor for large language model training
- Target: compete with GPU clusters for transformer inference
- Architecture: all-optical compute fabric with coherent WDM

#### 26.1.4 Rain Neuromorphics (Berkeley, CA)
- Approach: analog mixed-signal (not purely photonic)
- Combines photonic interconnects with RRAM synaptic weights
- Focus: energy-efficient inference for edge AI

#### 26.1.5 Ayar Labs (Santa Clara, CA)
- Not a compute company: optical I/O for silicon chips
- TeraPHY: monolithic electro-optic chip embeds laser, modulator, detector in CMOS
- Optical I/O replaces SerDes: lower power, higher bandwidth, longer reach
- Major customers: Intel, NVIDIA evaluating for future AI interconnect

### 26.2 Quantum Photonics Companies

#### 26.2.1 PsiQuantum (Palo Alto, CA)
- Goal: fault-tolerant photonic quantum computer with 1M+ physical qubits
- Approach: silicon photonics at scale + SNSPDs at 4K + FBQC error correction
- Partnership: GlobalFoundries for silicon photonic fab, cryo system manufacturers
- Funding: >\$600M (as of 2024)
- Timeline: first useful fault-tolerant QC claimed "in the 2020s"

#### 26.2.2 Xanadu (Toronto, Canada)
- CV quantum computing: squeezed light + linear optics + homodyne detection
- Borealis system: 216-mode GBS demonstrated quantum advantage (2022)
- PennyLane: most widely used quantum ML framework (open source)
- Strategy: near-term GBS applications + long-term fault-tolerant CV-QC

#### 26.2.3 QuiX Quantum (Netherlands)
- Reconfigurable linear optical quantum processors (silicon nitride)
- 20, 50, 100+ mode programmable interferometers
- Platform for boson sampling experiments and quantum photonic research

#### 26.2.4 Quandela (Paris, France)
- Single photon source specialist: QD-in-micropillar cavity (Madeleine source)
- Madeleine: record-breaking brightness + indistinguishability QD source
- Full quantum photonic platform from source to processor

#### 26.2.5 ORCA Computing (London, UK)
- Quantum memory-based temporal demultiplexer
- Room-temperature quantum optical memory using atomic vapor cells
- Focus: practical quantum advantage for machine learning applications

### 26.3 Key Academic Research Groups

- **MIT (Cambridge, MA):** Marin Soljačić (photonic AI, inverse design), Dirk Englund (quantum photonics, diamond NV centers, photonic processors), Karl Bergman (photonic network-on-chip), Anuradha Agarwal (photonic materials)
- **Stanford University:** Jelena Vučković (photonic crystal cavities, inverse design), Shanhui Fan (photonic topological effects, light-matter interactions), Gordon Wetzstein (computational imaging, optical neural networks), Olav Solgaard (MEMS photonics)
- **Caltech:** Alireza Marandi (ultrafast photonic computing, Ising machines, frequency combs), Kerry Vahala (microresonator frequency combs), Andrei Faraon (rare-earth quantum memories)
- **Princeton:** Paul Prucnal (neuromorphic photonics), Nathalie de Leon (diamond NV centers, quantum memories)
- **UCSB:** John Bowers (silicon photonic integration, lasers, high-speed devices)
- **UCLA:** Aydogan Ozcan (D2NN, computational imaging, photonic AI)
- **EPFL (Lausanne):** Tobias Kippenberg (microresonator frequency combs, microcombs), Camille Brès (silicon nitride nonlinear photonics)
- **University of Oxford:** Ian Walmsley (ultrafast quantum optics, quantum memories), Harish Bhaskaran (PCM optical memory)
- **University of Bristol:** Jeremy O'Brien (linear optical quantum computing, silicon photonic QC), Mark Thompson (integrated quantum photonics)
- **USTC (Hefei):** Jian-Wei Pan (quantum communication, boson sampling Jiuzhang, entanglement swapping records)
- **TU Eindhoven (COBRA):** Mehmet Smit (InP photonic integration), Erwin Bente (mode-locked lasers)
- **University of Münster:** Wolfram Pernice (PCM photonic computing, integrated SNSPDs)

---

## Chapter 27: Frontiers of Research

> *The following topics represent the leading edge as of 2025. Papers are still being written, experiments are underway, and some of the most important results have not yet been published. Read these sections as an invitation to contribute, not as a summary of what is known.*

### 27.1 Photonic Ising Machines

#### 27.1.1 The Ising Problem
- Minimize: H = -Σᵢⱼ Jᵢⱼ σᵢ σⱼ - Σᵢ hᵢ σᵢ where σᵢ ∈ {±1}
- NP-hard in general: spin glass (random Jᵢⱼ)
- Maps to: MaxCut, traveling salesman, portfolio optimization, protein folding
- If a photonic machine can efficiently minimize H → practical quantum-inspired advantage

#### 27.1.2 Coherent Ising Machine (CIM)
- Network of degenerate optical parametric oscillators (OPO): each oscillator = one spin
- Coupling: all-to-all via optical delay line feedback
- Spin value encoded in OPO phase: 0 or π
- Natural dynamics: OPO network minimizes a pseudo-Hamiltonian → approximate Ising solver
- Stanford/NTT CIM: 100,000 spin machine demonstrated (2021)
- Performance: often competitive with best classical algorithms, scalable

#### 27.1.3 Opto-Electronic Oscillator-Based Ising Machines
- Electronic feedback loop + opto-electronic nonlinearity
- Faster reconfiguration than all-optical CIM
- Demonstrated: IBM and MIT groups for combinatorial optimization

#### 27.1.4 Open Questions
- When does a CIM outperform best classical solvers (simulated annealing, CPLEX)?
- What problem instances are CIM-hard vs. classically easy?
- Can CIM scale to 10⁶ spins with high connectivity?
- Is there a path to quantum-enhanced Ising machine (beyond classical CIM)?

### 27.2 Integrated Frequency Combs for Computing

#### 27.2.1 Dissipative Kerr Soliton Microcombs
- Pump a microresonator (Si₃N₄ or silica) with CW laser above threshold
- Parametric oscillation → multiple comb teeth from a single pump
- Soliton state: stable, coherent comb with uniform spacing equal to FSR
- Comb tooth spacing: 10-1000 GHz (determined by resonator radius)
- Number of comb teeth: 100-500 across C+L band

#### 27.2.2 Microcomb as Multi-Wavelength Source
- Replace N separate lasers with one laser + one resonator
- Each comb tooth: a WDM channel for photonic computing or communication
- Key requirement: flat comb spectrum (dispersion engineering)
- Challenge: relative intensity noise (RIN) of comb teeth vs. DFB laser
- Demonstrated: microcomb-driven WDM neural network (Feldmann et al. 2021)

#### 27.2.3 Comb-Based Photonic Time-Frequency Computing
- Frequency comb as ruler: ultraprecise optical frequency reference
- Frequency comb as clock: optical frequency divided to microwave → low-noise clock
- Compute in the frequency domain: time-frequency analysis with combs
- Photonic analog of DFT using Talbot effect + soliton microcomb

### 27.3 Free-Space Photonic Computing

#### 27.3.1 Free-Space Matrix Multiplication
- Physical beam splitter network: light paths encode matrix elements
- Spatial light modulator (SLM): 1080×1920 independently programmable phase pixels
- Input vector encoded as pixel intensities; output detected by camera
- Matrix-vector product in one optical propagation step (~1 ns)
- Challenges: diffraction limit, alignment, SLM frame rate (60-1000 Hz)

#### 27.3.2 Optical Tensor Processors
- Multi-layer free-space system with SLMs as reconfigurable weight layers
- Demonstrated by Rafayelyan et al. (2020): random projection optical processor
- Scales to N ~ 10⁶ (SLM pixel count) — far more than MZI mesh
- Energy: input laser power + SLM electrical control (~10 W)
- Latency: limited by SLM update rate, not optical propagation

### 27.4 Optical Computing for Specific Domains

#### 27.4.1 Photonic Accelerator for Molecular Dynamics
- Molecular dynamics: O(N²) all-pairs interaction computation
- Photonic architecture: WDM + broadcast → parallelize interaction summation
- Estimate: 10,000× speedup for protein folding simulation vs. GPU

#### 27.4.2 Photonic Radio-Frequency Machine Learning
- Direct RF-to-optical conversion + photonic neural network inference
- Skip the ADC: photonic network classifies radar/communications signals optically
- Applications: electronic warfare, spectrum sensing, anti-jamming
- Demonstrated: photonic RF classification at 40 GHz bandwidth

#### 27.4.3 Optical Computing for Computational Biology
- Sequence alignment via optical correlation
- Single-cell RNA sequencing analysis via photonic dimensionality reduction
- Optical PCA: diffraction-based principal component extraction

### 27.5 Topological Photonics

#### 27.5.1 Topological Band Theory Applied to Photonics
- Photonic crystal analogs of topological insulators
- SSH model for photons: topologically protected edge states
- Bulk-boundary correspondence: topological invariant → edge modes
- 2D topological photonic crystal: chiral edge states immune to backscattering

#### 27.5.2 Applications
- Robust waveguiding: topological protection against defects and disorder
- Non-reciprocal light transmission: photonic analog of quantum Hall effect
- Topological laser: single-mode lasing at edge states

### 27.6 Photonic Computing with 2D Materials

#### 27.6.1 Graphene for Photonics
- Graphene: broadband absorption, ambipolar gate-tunable conductivity
- Graphene electro-optic modulator: gate voltage tunes Fermi level → tunable absorption
- Graphene photodetector: broadband, ultrafast (100+ GHz)
- Integration with silicon photonics: CVD graphene transferred to waveguide

#### 27.6.2 Transition Metal Dichalcogenides (TMDs)
- MoS₂, WSe₂: direct bandgap in monolayer form (visible emission)
- Strong excitonic effects, valley degree of freedom
- Potential: ultra-compact modulators, on-chip light sources

#### 27.6.3 van der Waals Heterostructures
- Stack 2D materials with atomic precision
- Moiré patterns: superlattice effects → tunable bandgap
- Photonic potential: moiré exciton polaritons for strongly correlated photonic many-body physics

### 27.7 The Path to Fault-Tolerant Photonic Quantum Computing

#### 27.7.1 Resource State Generation at Scale
- FBQC requires 6-photon GHZ-like resource states continuously
- Generation: SPDC sources + heralding + active switching
- Target generation rate: >10⁸ resource states per second for a useful QC
- Loss tolerance: each photon loss ≤ 10%

#### 27.7.2 Photon Loss Thresholds
- Surface code threshold: ~1% error per gate (standard circuit model)
- Photon loss is erasure error: higher threshold for erasure codes (~50%)
- For FBQC: loss threshold ~15% per photon (with appropriate code)
- Current state: ~5-10% loss per photon (limited by source efficiency, coupler loss, detector)
- Gap: need 2-3× improvement across the stack

#### 27.7.3 Timeline and Scale Estimates
- Near-term (2025-2028): demonstration of fault-tolerant logical qubit with photons
- Medium-term (2028-2033): small-scale useful quantum algorithms (chemistry, optimization)
- Long-term (2033+): fault-tolerant universal QC with millions of physical photonic qubits
- PsiQuantum roadmap: production fab partnership → 2029 delivery of first useful system
- Xanadu roadmap: near-term GBS advantage → CV fault-tolerant QC

---

## Chapter 28: Outlook — Open Problems and the Future

### 28.1 Fundamental Physical Limits

#### 28.1.1 Energy per Operation — The Landauer Limit
- Landauer limit: kT ln 2 ≈ 3×10⁻²¹ J at 300 K
- Current photonic MAC: ~0.1-1 pJ (100-10,000× above Landauer)
- Electronic MAC: ~0.1 pJ (similar to photonic, but improving at ~40%/year)
- Path to photonic sub-fJ operation: reduce optical losses, reduce detector bandwidth, lower laser threshold

#### 28.1.2 The Precision-Energy Trade-off
- Shannon information theory applied to analog photonic computing
- Bits of precision available: B = ½ log₂(1 + SNR) where SNR = signal power / noise power
- More precision requires more optical power → fundamental trade-off
- Optimal point: depends on algorithm accuracy requirements
- Photonic ML: 4-8 bits often sufficient → competitive energy-per-bit

#### 28.1.3 Quantum Noise Floor
- Shot noise from photon arrivals: irreducible at room temperature
- Quantum-limited receiver: approaches shot noise limit
- Squeezed-light injection: can beat shot noise in one quadrature
- Ultimate limit for photonic computing: quantum Fisher information bound

### 28.2 Engineering Challenges (Summary)

- **Thermal management:** photonic chips generate heat; thermal crosstalk between ring resonators and heaters; requires μK-level stability for dense WDM systems
- **Calibration and drift:** MZI phase drifts due to temperature, aging; requires continuous recalibration; solution: in-situ monitoring, closed-loop control
- **Electronic-photonic bandwidth:** the photonic core may be fast, but the electronic interface (DAC/ADC) is the bottleneck; co-packaged optics helps but not yet solved at required bandwidth density
- **Scalability of photon sources:** quantum photonic computing requires millions of indistinguishable photons/second — not yet demonstrated at needed fidelity and brightness simultaneously
- **Photonic memory:** no photonic equivalent of RAM; optical delay lines are lossy; PCM memory limited endurance (10⁶ cycles); major open problem

### 28.3 Open Research Questions

1. **Is there a photonic "transistor" equivalent?** A fast, low-loss, low-energy photon-controlled photon switch — the holy grail of optical computing since 1960. Current candidates: photonic crystal cavity with one-photon switching (~100 photons currently, need ~1), EIT-based slow light switches, polariton switches.

2. **End-to-end energy efficiency:** Photonic computing often claims energy advantage in the optical domain, but DAC/ADC and laser overhead may dominate. What is the minimum system energy? What algorithm-hardware co-design minimizes it?

3. **What problems are uniquely suited to optical computing?** Not just "matrix multiply on a chip" but: what algorithm structure maps perfectly to photonic physics with no electronic overhead? Candidates: FMCW LIDAR, optical coherence tomography processing, direct RF neural classification.

4. **Hybrid classical-quantum photonic systems:** Can near-term photonic quantum processors (imperfect, noisy) be combined with classical photonic processors to solve practical problems that neither could solve alone?

5. **The photonic memory problem:** Every practical computing system needs random-access memory. Can an optical or hybrid system be built that provides high-bandwidth, low-latency, nonvolatile memory competitive with DRAM?

### 28.4 The Future of the Field

#### 28.4.1 Convergence of Photonic Computing and Quantum Photonics
- Today: classical photonic computing and quantum photonic computing are separate fields
- Future: hybrid systems — classical optical preprocessing + quantum coherent post-processing
- Example: optical neural network preprocessing → input to quantum sampling circuit → post-process classically
- Requires: quantum-classical interface with minimal decoherence

#### 28.4.2 Photonic Computing for Scientific Simulation
- Molecular dynamics, weather prediction, fluid simulation, protein folding
- These are dominated by linear algebra and nearest-neighbor interactions
- Photonic architecture naturally parallelize all-to-all linear operations
- Not yet demonstrated at sufficient scale and accuracy — major opportunity

#### 28.4.3 The Post-Silicon Computing Landscape
- CMOS transistor scaling ends at ~1 nm feature size (physical limits: quantum tunneling, heat)
- Successors: photonics, spintronics, 2D materials, quantum, neuromorphic, DNA computing
- Most likely: heterogeneous integration — each technology for what it does best
- Photonics' role: high-bandwidth interconnects + specific compute kernels + quantum I/O

#### 28.4.4 The 50-Year View
- 1975: first optical fiber installed; 2025: fiber carries >1 Pbps
- 2025: first commercial photonic AI accelerators; 2075: photonic computing = mainstream?
- The scientific community that will make this happen is being trained today
- The problems worth solving are not "can we build it" but "what should we build, for whom, and why"

---

# APPENDICES

## Appendix A: Mathematical Prerequisites

### A.1 Linear Algebra for Photonic Computing
- A.1.1 Vectors, inner products, norms in ℂⁿ
- A.1.2 Matrix operations: multiplication, inverse, transpose, Hermitian conjugate
- A.1.3 Eigendecomposition: Av = λv; diagonalization A = QΛQ⁻¹
- A.1.4 Unitary matrices: U†U = I; preserves inner products; crucial for MZI mesh analysis
- A.1.5 Singular value decomposition: A = UΣV†; any matrix expressed as two unitaries + diagonal
- A.1.6 Matrix exponentials: e^A; relation to unitary evolution e^(iH) for Hermitian H
- A.1.7 Exercises: 15 problems ranging from basic to SVD applications

### A.2 Complex Analysis
- A.2.1 Complex numbers: polar form, Euler's formula e^(iθ) = cos θ + i sin θ
- A.2.2 Analytic functions: Cauchy-Riemann equations
- A.2.3 Contour integration and residue theorem
- A.2.4 Application: deriving Kramers-Kronig relations via contour integral in upper half-plane
- A.2.5 Exercises: 10 problems

### A.3 Fourier Analysis
- A.3.1 Continuous Fourier transform and inverse: F̃(ν) = ∫f(t)e^{-2πiνt}dt
- A.3.2 Key transform pairs: Gaussian, rect, sinc, comb
- A.3.3 Convolution theorem: FT{f*g} = F̃·G̃
- A.3.4 Parseval's theorem: ∫|f|² = ∫|F̃|²
- A.3.5 Sampling theorem: f_s > 2B_max (Nyquist)
- A.3.6 2D Fourier transform: for Fourier optics
- A.3.7 Discrete Fourier transform and FFT algorithm: O(N log N)
- A.3.8 Exercises: 12 problems including 2D DFT and FFT implementation

### A.4 Probability and Statistics
- A.4.1 Random variables, probability density functions
- A.4.2 Gaussian/normal distribution: moments, central limit theorem
- A.4.3 Poisson distribution: photon counting statistics
- A.4.4 Maximum likelihood estimation
- A.4.5 Monte Carlo methods for photonic system analysis
- A.4.6 Exercises: 10 problems

### A.5 Differential Equations
- A.5.1 First-order ODEs: separation of variables, integrating factor
- A.5.2 Second-order linear ODEs: characteristic equation, particular solution
- A.5.3 Systems of ODEs: matrix exponential, stability analysis (laser rate equations)
- A.5.4 PDEs: wave equation, heat equation, Schrödinger equation
- A.5.5 Separation of variables for the wave equation in cylindrical coordinates (fiber modes)
- A.5.6 Exercises: 12 problems

---

## Appendix B: Quantum Computing Prerequisites

### B.1 Qubit, Gates, and Circuits
- B.1.1 Qubit state space: Bloch sphere
- B.1.2 Single-qubit gates: X (NOT), Y, Z, H (Hadamard), S (phase), T
- B.1.3 Two-qubit gates: CNOT, CZ, SWAP, Toffoli
- B.1.4 Quantum circuit diagrams and notation
- B.1.5 Universality: any unitary approximated by {H, T, CNOT}

### B.2 Quantum Algorithms
- B.2.1 Deutsch-Josza algorithm: introduction to quantum speedup
- B.2.2 Grover's search algorithm: O(√N) vs. O(N) classical
- B.2.3 Shor's factoring algorithm: O(log N)³ — threat to RSA
- B.2.4 Quantum simulation: Hamiltonian simulation, chemistry

### B.3 Quantum Error Correction
- B.3.1 Three-qubit bit-flip code
- B.3.2 Shor's nine-qubit code: corrects any single-qubit error
- B.3.3 Stabilizer formalism
- B.3.4 Surface code: threshold theorem, logical error rate

### B.4 Exercises (20 problems including circuit implementation)

---

## Appendix C: Software Environment Setup

### C.1 Python Environment for Photonics
- Python 3.10+, virtual environment setup
- NumPy, SciPy, Matplotlib, Pandas
- Jupyter Lab for interactive computation

### C.2 Photonic Simulation Tools
- Meep (FDTD): installation, first simulation, waveguide mode
- MPB (Photonic Bands): photonic crystal band structure
- gdsfactory: layout design, PDK integration
- SiEPIC-EBeam PDK: open-source silicon photonics design kit
- Lumerical FDTD (commercial): student license access

### C.3 Quantum Photonics Tools
- QuTiP: quantum optics simulation
- Strawberry Fields (Xanadu): photonic quantum computing simulation
- PennyLane: quantum machine learning
- Qiskit: gate-based quantum circuits (for comparison)

### C.4 Photonic Neural Network Tools
- neurophox (MIT): MZI mesh simulation and training
- neuroptica: optical neural network platform
- PyTorch + custom CUDA extensions for hardware-aware ONN training

### C.5 Step-by-Step Tutorials
- Tutorial 1: Simulate a silicon photonic waveguide in Meep
- Tutorial 2: Design a ring resonator in gdsfactory, send to foundry
- Tutorial 3: Train an ONN on MNIST using neurophox
- Tutorial 4: Simulate a boson sampling circuit in Strawberry Fields
- Tutorial 5: Implement a quantum key distribution protocol in QuTiP

---

## Appendix D: Photonic Material Properties Reference

### D.1 Material Property Tables at 1550 nm
| Material | n | α (dB/cm) | n₂ (m²/W) | Notes |
|----------|---|-----------|------------|-------|
| Si | 3.47 | 2-3 | 4×10⁻¹⁸ | TPA at high power |
| SiO₂ | 1.44 | 0.0001 | 2.2×10⁻²⁰ | Fiber, cladding |
| Si₃N₄ | 2.00 | 0.01 | 2.4×10⁻¹⁹ | Low loss waveguides |
| LiNbO₃ | 2.21 | 0.3 | 1.8×10⁻¹⁹ | r₃₃=30 pm/V (Pockels) |
| InP | 3.16 | variable | 1.5×10⁻¹⁷ | Gain medium |
| GaAs | 3.37 | variable | 1.6×10⁻¹⁷ | Gain medium, 850 nm |
| Ge | 4.28 | variable | N/A | Telecom photodetector |
| GST (amor.) | 4.0 | high | N/A | Phase-change material |

### D.2 Key Constants
- Speed of light: c = 2.998×10⁸ m/s
- Planck constant: h = 6.626×10⁻³⁴ J·s, ℏ = 1.055×10⁻³⁴ J·s
- Boltzmann constant: k_B = 1.381×10⁻²³ J/K
- Elementary charge: q = 1.602×10⁻¹⁹ C
- Vacuum permittivity: ε₀ = 8.854×10⁻¹² F/m
- Vacuum permeability: μ₀ = 4π×10⁻⁷ H/m
- Photon energy at 1550 nm: 0.8 eV = 1.28×10⁻¹⁹ J

---

## Appendix E: Master Glossary

*(200+ terms, from ADC to Zernike polynomials, covering electromagnetism, photonics, quantum optics, quantum computing, machine learning, fabrication, and communications)*

Key entries include:
- **ABCD matrix:** Ray transfer matrix for paraxial optics
- **AWG (Arrayed Waveguide Grating):** WDM demultiplexer using waveguide array
- **Bell state:** One of four maximally entangled two-qubit states
- **Boson sampling:** Sampling from the output distribution of a linear optical network — believed classically hard
- **CIM (Coherent Ising Machine):** Network of OPOs solving the Ising optimization problem
- **D2NN:** Diffractive deep neural network — optical analog compute using diffraction layers
- **EDFA:** Erbium-doped fiber amplifier — optical amplifier for C-band
- **Finesse:** Ratio of FSR to resonance linewidth of a Fabry-Pérot cavity
- **FBQC:** Fusion-based quantum computing — photonic MBQC scheme by PsiQuantum
- **GBS (Gaussian Boson Sampling):** CV photonic quantum advantage experiment
- **GKP encoding:** Qubit encoded in oscillator quadrature for error correction
- **HOM (Hong-Ou-Mandel) effect:** Two identical photons at a BS always exit together
- **KLM protocol:** Linear optical quantum computing with measurement-induced nonlinearity
- **LNOI:** Lithium niobate on insulator — platform for fast electro-optic modulators
- **MZI:** Mach-Zehnder interferometer — basic building block of optical computing circuits
- **PCM:** Phase-change material — nonvolatile optical memory (GST, GSST)
- **Purcell factor:** Enhancement of spontaneous emission rate by a cavity
- **Q factor:** Quality factor of a resonator = energy stored / power dissipated
- **SOI:** Silicon-on-insulator — standard platform for silicon photonics
- **SNSPD:** Superconducting nanowire single-photon detector
- **SPDC:** Spontaneous parametric down-conversion — entangled photon pair source
- **WDM:** Wavelength division multiplexing — parallel channels at different wavelengths

---

## Appendix F: Solutions to Selected Exercises

Worked solutions for one-third of all exercises, with emphasis on the most conceptually important problems.

---

## Master Bibliography

### Essential Textbooks (by Unit)
**Unit I (Electromagnetism and Optics):**
- Griffiths, *Introduction to Electrodynamics*, 4th ed. (2017)
- Born & Wolf, *Principles of Optics*, 7th ed. (1999)
- Goodman, *Introduction to Fourier Optics*, 4th ed. (2017)
- Saleh & Teich, *Fundamentals of Photonics*, 3rd ed. (2019)

**Unit II (Lasers and Detectors):**
- Yariv & Yeh, *Photonics: Optical Electronics in Modern Communications*, 6th ed. (2007)
- Coldren, Corzine & Mašanović, *Diode Lasers and Photonic Integrated Circuits*, 2nd ed. (2012)

**Unit III (Waveguides and Silicon Photonics):**
- Agrawal, *Nonlinear Fiber Optics*, 6th ed. (2019)
- Chrostowski & Hochberg, *Silicon Photonics Design* (2015) [free PDF]

**Unit IV (Communications):**
- Agrawal, *Fiber-Optic Communication Systems*, 6th ed. (2021)
- Cover & Thomas, *Elements of Information Theory*, 2nd ed. (2006)

**Unit V-VI (Photonic Computing):**
- No dedicated textbook exists yet (as of 2025)
- See review papers: Shastri et al. (2021) *Nature Photonics*; Wetzstein et al. (2020) *Nature*

**Unit VII (Quantum Photonics):**
- Nielsen & Chuang, *Quantum Computation and Quantum Information* (2010)
- Walls & Milburn, *Quantum Optics*, 2nd ed. (2008)
- Gerry & Knight, *Introductory Quantum Optics* (2004)

**Unit VIII (Fabrication):**
- Taflove & Hagness, *Computational Electrodynamics*, 3rd ed. (2005)
- Joannopoulos et al., *Photonic Crystals*, 2nd ed. [free PDF at ab-initio.mit.edu]

### 50 Most Important Primary Papers
1. Maxwell (1865), "A Dynamical Theory of the Electromagnetic Field"
2. Kao & Hockham (1966), "Dielectric-fibre surface waveguides for optical frequencies"
3. Maiman (1960), "Stimulated optical radiation in ruby" — first laser
4. Mears et al. (1987), "Low-noise erbium-doped fibre amplifier"
5. Soref & Bennett (1987), "Electrooptical effects in silicon"
6. Reck et al. (1994), "Experimental realization of any discrete unitary operator"
7. Bell (1964), "On the Einstein-Podolsky-Rosen paradox"
8. Hong, Ou & Mandel (1987), "Measurement of subpicosecond time intervals between two photons by interference"
9. Knill, Laflamme & Milburn (2001), "A scheme for efficient quantum computation with linear optics"
10. Aaronson & Arkhipov (2013), "The Computational Complexity of Linear Optics"
11. Bennett & Brassard (1984), "Quantum cryptography: public key distribution and coin tossing"
12. Shen et al. (2017), "Deep learning with coherent nanophotonic circuits," *Nature Photonics*
13. Lin et al. (2018), "All-optical machine learning using diffractive deep neural networks," *Science*
14. Xu et al. (2005), "Micrometre-scale integrated silicon ring modulator," *Nature*
15. Kippenberg et al. (2018), "Dissipative Kerr solitons in optical microresonators," *Science*
16. Ríos et al. (2015), "Integrated all-photonic non-volatile multi-level memory," *Nature Photonics*
17. Marsili et al. (2013), "Detecting single infrared photons with 93% system efficiency," *Nature Photonics*
18. Zhong et al. (2020), "Quantum computational advantage using photons," *Science* (Jiuzhang)
19. Madsen et al. (2022), "Quantum computational advantage with programmable photonic processor," *Nature*
20. Bartolucci et al. (2021), "Fusion-based quantum computation"
21. Clements et al. (2016), "Optimal design for universal multiport interferometers," *Optica*
22. Piggott et al. (2015), "Inverse design and demonstration of a compact wavelength demultiplexer," *Nature Photonics*
23. Miller (2009), "Device requirements for optical interconnects to silicon chips," *Proc. IEEE*
24. Tait et al. (2017), "Neuromorphic photonic networks using silicon photonics weight banks," *Scientific Reports*
25. Nahmias et al. (2013), "A Leaky Integrate-and-Fire Laser Neuron," *IEEE J. STQE*
26. Vandoorne et al. (2014), "Experimental demonstration of reservoir computing on a silicon photonics chip," *Nature Comm.*
27. Shastri et al. (2021), "Photonics for artificial intelligence and neuromorphic computing," *Nature Photonics*
28. Feldmann et al. (2021), "Parallel convolutional processing using an integrated photonic tensor core," *Nature*
29. Feldmann et al. (2019), "All-optical spiking neurosynaptic networks with self-learning capabilities," *Nature*
30. Essiambre et al. (2010), "Capacity Limits of Optical Fiber Networks," *J. Lightwave Tech.*
31. Hamerly et al. (2019), "Experimental Investigation of Performance Differences Between Coherent and Incoherent Feedback OPU"
32. Pan et al. (2017), "Satellite-based entanglement distribution over 1200 km," *Science*
33. Aspect, Grangier & Roger (1982), "Experimental Realization of Einstein-Podolsky-Rosen-Bohm Gedankenexperiment," *PRL*
34. Joannopoulos et al. (1997), "Photonic crystals," *Solid State Comm.*
35. Yu & Capasso (2014), "Flat optics with designer metasurfaces," *Nature Materials*
36. Agrawal (2019), *Nonlinear Fiber Optics* (6th edition as reference)
37. Lugiato & Lefever (1987), "Spatial dissipative structures in passive optical systems," *PRL* (LLE equation)
38. Okamoto (2006), *Fundamentals of Optical Waveguides* (2nd edition as reference)
39. Tomm et al. (2021), "A bright and fast source of coherent single photons," *Nature Nanotechnology*
40. Hughes et al. (2018), "Training of photonic neural networks through in situ backpropagation," *Optica*
41. Brunner et al. (2013), "Parallel photonic information processing at gigabyte per second data rates," *Nature Comm.*
42. Wetzstein et al. (2020), "Inference in artificial intelligence with deep optics and photonics," *Nature*
43. Bandyopadhyay et al. (2022), "Single chip photonic deep neural network with accelerated training," *arXiv*
44. Pai et al. (2023), "Experimentally realized in situ backpropagation for deep learning in photonic neural networks," *Science*
45. Larger et al. (2012), "Photonic information processing beyond Turing: an optoelectronic implementation of reservoir computing," *Opt. Express*
46. Kippenberg, Holzwarth & Diddams (2011), "Microresonator-based optical frequency combs," *Science*
47. Pernice et al. (2012), "High-speed and high-efficiency travelling wave single-photon detectors embedded in nanophotonic circuits"
48. Wehner, Elkouss & Hanson (2018), "Quantum internet: A vision for the road ahead," *Science*
49. Pfeiffer & Kippenberg (2018), "Ultrasmooth silicon nitride waveguides," *Optica*
50. Hensen et al. (2015), "Loophole-free Bell inequality violation," *Nature*

---

## Online Resources and Courses

- **MIT OpenCourseWare:** 6.007 (Electromagnetic Energy), 6.013 (Electromagnetics), 8.03 (Waves), 8.422 (Quantum Optics)
- **Photonics Bootcamp (SiEPIC):** edX course on silicon photonics design (Chrostowski, UBC)
- **IBM Quantum Learning:** quantum computing fundamentals
- **Xanadu Quantum Codebook:** interactive quantum computing tutorials with Strawberry Fields/PennyLane
- **YouTube:** MIT OpenCourseWare channels; Alex Wozniak photonics lectures; Quantum Computing Now podcast
- **arXiv sections:** physics.optics, quant-ph, cs.ET (emerging technologies)
- **IEEE Photonics Society:** webinars and tutorials
- **SPIE digital library:** photonics conferences and proceedings

---

*End of Outline*

**Total estimated scope:**
- 10 Units
- 28 Chapters
- ~180 Sections
- ~500 Subsections
- ~350 Exercises (mathematical, conceptual, applied)
- ~75 Programming/Engineering Projects
- 6 Appendices
- ~50 Key Papers cited
- ~200 Key Researchers mentioned

**Estimated textbook length at full development:** 1,200-1,500 pages

---
