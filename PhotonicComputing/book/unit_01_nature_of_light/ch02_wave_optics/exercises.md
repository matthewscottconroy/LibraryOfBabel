# Exercises — Chapter 2: Wave Optics

## Mathematical Exercises

**2.1** (Interference) Two coherent plane waves of intensity $I_0$ each interfere with a phase difference $\Delta\phi$.

(a) Write the expression for the total intensity $I(\Delta\phi)$.

(b) Compute the fringe visibility for the case where one beam has twice the intensity of the other ($I_2 = 2I_1$).

(c) Show that the time-averaged intensity when $\Delta\phi$ varies uniformly in $[0, 2\pi)$ equals $I_1 + I_2$ — confirming energy conservation.

---

**2.2** (Fabry-Pérot) A symmetric Fabry-Pérot etalon has mirror reflectance $R = 0.95$ and spacing $L = 1$ mm in air.

(a) Compute the finesse $F$.

(b) Compute the free spectral range $\nu_\text{FSR}$ and $\lambda_\text{FSR}$ near $\lambda = 1550$ nm.

(c) Compute the FWHM of a single resonance peak, in both frequency (Hz) and wavelength (pm).

(d) If the etalon is now filled with silicon ($n = 3.48$, $n_g = 4.2$), how does each of the above change?

---

**2.3** (MZI Transfer Function) An MZI has a 50:50 beam splitter with the transfer matrix $\frac{1}{\sqrt{2}}\begin{pmatrix} 1 & i \\ i & 1 \end{pmatrix}$ at each coupler. A phase shift $\Delta\phi$ is applied to the upper arm.

(a) Compute the full MZI transfer matrix $U(\Delta\phi)$.

(b) Show that $|U_{11}|^2 + |U_{21}|^2 = 1$ (the transformation is unitary).

(c) Compute the output intensities $I_1$ and $I_2$ when input is $E_\text{in} = (1, 0)^T$ (all power in port 1), as functions of $\Delta\phi$.

(d) What value of $\Delta\phi$ routes all power to port 2? What value routes all power to port 1?

(e) What is the output when both ports are equally illuminated: $E_\text{in} = (1/\sqrt{2}, 1/\sqrt{2})^T$?

---

**2.4** (Fraunhofer Diffraction) A slit of width $a = 50$ μm is illuminated with $\lambda = 633$ nm light.

(a) Compute the angular positions of the first three minima.

(b) Compute the ratio of intensities at the first and second secondary maxima to the central maximum. (Note: secondary maxima occur approximately at $\sin\theta = (m+1/2)\lambda/a$ for integer $m$.)

(c) Verify that the angular width of the central peak ($2\lambda/a$) equals twice the angle to the first minimum.

---

**2.5** (Jones Calculus) An optical system consists of three elements in sequence:
1. A linear polarizer at $0°$ (horizontal)
2. A quarter-wave plate with fast axis at $45°$
3. A linear polarizer at $90°$ (vertical)

(a) Write the Jones matrix for each element.

(b) Compute the product Jones matrix for the complete system.

(c) What is the output intensity for an input of horizontal polarization? Vertical polarization? $+45°$ polarization?

(d) Explain physically why horizontal polarization produces no output through this system.

---

**2.6** (Gaussian Beam) A Gaussian beam at $\lambda = 1550$ nm has waist $w_0 = 3$ μm.

(a) Compute the Rayleigh range $z_R$.

(b) Compute the beam radius $w(z)$ at $z = 0$, $z = z_R$, $z = 5z_R$, and $z = 50z_R$.

(c) Compute the far-field half-angle divergence $\theta$.

(d) A thin lens of focal length $f = 10$ mm is placed at the beam waist. Compute the new waist $w_0'$ and its location relative to the lens.

(e) What focal length is needed to focus this beam to a $1$ μm spot, if the input beam (at the lens) has $w_\text{in} = 1$ mm?

---

**2.7** (Coherence Length) A DFB laser has a Lorentzian lineshape with $\Delta\nu = 100$ kHz at $\lambda = 1550$ nm.

(a) Compute the coherence time $\tau_c$ and coherence length $L_c$.

(b) A Michelson interferometer scans path length differences from 0 to $L_c$. At what path difference does the fringe visibility drop to $1/e$ of its maximum?

(c) A photonic chip has MZI arms differing in length by $\Delta L = 500$ μm (in silicon, $n_g = 4.2$). The effective path difference is $n_g \Delta L$. What fringe visibility does this produce?

---

**2.8** (Ring Resonator Design) Design a silicon ring resonator (in silicon, $n_g = 4.2$) with:
- Free spectral range $\nu_\text{FSR} = 100$ GHz
- Q factor $Q = 10^4$

(a) What ring circumference $C$ is needed for $\nu_\text{FSR} = 100$ GHz?

(b) What resonance linewidth $\Delta\nu$ (in GHz) does $Q = 10^4$ imply at 1550 nm?

(c) What finesse $F$ does this correspond to?

(d) If the bus waveguide coupling coefficient is $\kappa^2 = 0.01$ (power fraction coupled per pass), what round-trip amplitude loss $r$ is needed to achieve critical coupling ($\kappa^2 = 1 - r^2$)?

---

**2.9** (4f System and Spatial Filtering) A 4f system has lenses of focal length $f = 50$ mm. An input field with spatial frequency content up to $f_{x,\text{max}} = 200$ mm⁻¹ is processed.

(a) What is the maximum spatial frequency retained? (All frequencies are retained unless a filter is applied.)

(b) A circular aperture of radius $r_0 = 1$ mm is placed in the Fourier plane. What is the maximum transmitted spatial frequency? What is the corresponding minimum feature size in the output?

(c) The same aperture acts as a spatial frequency filter. Qualitatively, does this implement a low-pass or high-pass filter? What feature size is "blurred" vs. "passed"?

(d) If the input is a perfect point source (delta function), what is the output after the aperture filter? (Hint: the Fourier transform of a delta function is a constant; the inverse Fourier transform of a truncated constant is a sinc/Airy function.)

---

**2.10** (Mode Overlap and Coupling) A standard single-mode fiber at 1550 nm has mode field diameter (MFD) $= 10.4$ μm (assume Gaussian profile with $w_\text{fiber} = 5.2$ μm). A silicon inverse taper expands the waveguide mode to MFD $= 3$ μm ($w_\text{Si} = 1.5$ μm).

(a) Compute the mode overlap integral and coupling efficiency.

(b) What is the coupling loss in dB?

(c) What is the optimum coupling lens focal length if the fiber output divergence is $\theta_\text{fiber} = \lambda/(\pi w_\text{fiber}) = 95$ mrad, and the fiber mode is to be focused to match the inverse taper output mode?

(d) How much would a lateral alignment error of $\delta x = 1$ μm reduce the coupling efficiency?

---

## Conceptual Exercises

**2.11** (Geometric vs. Wave Optics) A silicon waveguide of width 450 nm is operating at 1550 nm. Is geometric optics (ray optics) valid for describing the optical mode in this waveguide? Justify your answer quantitatively using the eikonal condition.

---

**2.12** (Interference and Energy Conservation) Two coherent laser beams of equal power 1 mW each interfere. At some points the intensity is 4 mW/m² (constructive interference) and at others it is 0 (destructive interference). No energy is created or destroyed — where does the energy go? Explain using the spatial distribution of the interference pattern.

---

**2.13** (Coherence and Computation) You are designing a photonic neural network and must choose between two laser sources: (A) a DFB laser with $\Delta\nu = 1$ MHz and (B) an LED with $\Delta\nu = 5$ THz.

(a) Compute the coherence lengths of both sources.

(b) The photonic chip has MZI path length differences up to 5 mm (in silicon, $n_g = 4.2$). Which source can support coherent interference at all MZIs? 

(c) Source B is cheaper and more compact. Can you design a photonic computing architecture that uses source B? What fundamental computational capabilities would be lost?

---

**2.14** (The $\pi/2$ Phase Shift) In a lossless, symmetric beam splitter, the through-coupling amplitude is real ($t$) and the cross-coupling amplitude is $r = i|r|$ (imaginary). 

(a) Use the requirement that the beam splitter matrix is unitary to derive the relationship $|t|^2 + |r|^2 = 1$.

(b) Use unitarity to also show that $t^* r + r^* t = 0$, which implies $r = i|r|$ (imaginary) if $t$ is real.

(c) Why is the $\pi/2$ phase shift between through and cross-coupled beams physically necessary for energy conservation? Describe a thought experiment that shows energy would not be conserved if both $t$ and $r$ were real.

---

**2.15** (Polarization and Silicon) You launch light with 45° linear polarization into a 1 cm long silicon strip waveguide (with TE and TM effective indices $n_\text{TE} = 2.40$, $n_\text{TM} = 1.78$ at 1550 nm).

(a) Write the input Jones vector.

(b) After 1 cm of propagation, what is the output polarization state? Compute the phase accumulated by TE and TM components.

(c) What polarization state is the output? Is it linear, circular, or elliptical?

(d) Does this polarization-mixed output interfere constructively or destructively with a reference TE-polarized beam?

---

**2.16** (Diffraction and Waveguide Arrays) A phased array of 8 equally spaced optical antennas (waveguide array outputs) has antenna spacing $d = 5$ μm. The wavelength is 1550 nm.

(a) Compute the angular positions of the main diffraction orders (for uniform phase across all antennas).

(b) If a linear phase ramp $\Delta\phi = \pi/4$ per element is applied (each antenna has $\pi/4$ more phase than the previous), by how much does the main beam steer?

(c) What is the far-field angular resolution (FWHM of main lobe) for 8 elements?

(d) What physical limitation prevents steering the beam beyond $\pm 90°$?

---

## Laboratory Exercises

**Lab 2.1: MZI Simulation in Python**

Build a numerical simulation of a Mach-Zehnder interferometer.

(a) Implement the beam splitter matrix $\frac{1}{\sqrt{2}}\begin{pmatrix} 1 & i \\ i & 1 \end{pmatrix}$ and a phase shift matrix $\begin{pmatrix} e^{i\phi_1} & 0 \\ 0 & e^{i\phi_2} \end{pmatrix}$.

(b) Compute and plot the MZI transmission $T(\Delta\phi) = |E_\text{out1}|^2$ for $\Delta\phi \in [0, 4\pi]$.

(c) Simulate a 4×4 MZI mesh (Clements architecture) with random phase settings. Show that the $4 \times 4$ transfer matrix is unitary (verify $U^\dagger U = I$ numerically).

(d) Find the phase settings to implement the $4 \times 4$ discrete Fourier transform matrix $F_{mn} = e^{2\pi i mn/4}/2$.

**Skills practiced**: Linear algebra, complex exponentials, unitary matrix verification.

---

**Lab 2.2: Fabry-Pérot Ring Resonator Transfer Function**

Simulate the spectral response of a ring resonator.

(a) Implement the ring resonator transfer function using the formula from Section 2.2.3. Plot the through-port transmission $T(\nu)$ for $R = 0.9$ and $R = 0.99$ over a range of 3 FSRs.

(b) Fit a Lorentzian to one resonance peak and extract the FWHM. Compare to the theoretical value.

(c) Compute the group delay $\tau_g = -d\phi/d\omega$ (phase of the transmission function) near resonance. Show that the group delay diverges at resonance — indicating slow light enhancement.

(d) Add a frequency-dependent loss (e.g., $r \to r e^{-\alpha L/2}$ where $\alpha$ increases with detuning from resonance, simulating two-photon absorption in silicon). How does this affect the resonance shape?

**Skills practiced**: Transfer function calculation, Lorentzian fitting, phase analysis.

---

**Lab 2.3: Fraunhofer Diffraction and Fourier Transform**

(a) Implement the 1D Fraunhofer diffraction integral numerically using the FFT: $E(\theta) \propto \text{FFT}[E(x')]$. 

(b) Verify: uniform aperture of width $a$ → sinc function. Gaussian aperture of width $w$ → Gaussian. Two-slit aperture → sinc × interference fringes.

(c) Design an aperture whose diffraction pattern (intensity) matches a given target pattern (e.g., a flat-top spot, or a spot with a sidelobe at a specific angle). Use a genetic algorithm or gradient descent on the aperture transmission to find the optimal design.

(d) Implement a 2D FFT-based simulation of the 4f system. Place a spatial filter in the Fourier plane and observe the effect on the output image. Try: low-pass circular aperture, high-pass annulus, and a custom phase mask (e.g., a spiral phase plate for OAM beam generation).

**Skills practiced**: FFT as a physical tool, numerical optimization, 2D image processing.

---

**Lab 2.4: Gaussian Beam Propagation Simulation**

(a) Implement Gaussian beam propagation using the $q$-parameter ABCD law. For a beam with $w_0 = 10$ μm, $\lambda = 1550$ nm, compute and plot $w(z)$ and $R(z)$ over $z \in [-5z_R, +50z_R]$.

(b) Place a thin lens of focal length $f = 5z_R$ at $z = 0$. Compute the new beam parameters $w_0'$ and location of the new waist. Verify against the analytical formula.

(c) Design a two-lens beam expander that takes the beam from $w_0 = 5$ μm (at a chip facet) to $w_0' = 5.2$ μm (to match single-mode fiber MFD of 10.4 μm). The total length of the beam expander should be less than 20 mm.

(d) Compute the coupling efficiency as a function of lateral offset (scan from 0 to $3w_\text{mode}$) for coupling to a Gaussian fiber mode of $w_\text{mode} = 5.2$ μm. Plot $\eta(\delta x)$. What is the $3$ dB alignment tolerance?

**Skills practiced**: ABCD matrix computation, lens system design, coupling efficiency optimization.

---

## Thought Experiments

**2.17** (What if Photons Were Bosons... but Classical?)  
Classical electromagnetic waves are already bosons in the sense that they obey superposition (constructive interference). Quantum mechanics adds particle statistics: bosons bunch (Hong-Ou-Mandel effect). 

Consider a 50:50 beam splitter with two input photons entering one port each (classical coherent states). Classically, intensity is split equally: 50% to each output. Quantum mechanically (two single photons), the photons always exit the same port (both to port 1 or both to port 2, with equal probability). How would this quantum behavior (HOM effect) limit or enable a photonic computing architecture? What computation would be *impossible* without exploiting HOM interference, but *possible* with it?

---

**2.18** (Coherence and Analog Precision)  
A photonic neural network uses an MZI mesh with 1024 phase shifters, each requiring precision to $1/256$ of $\pi$ radians (8-bit precision). The chip operates at 1 ns per vector-matrix multiply.

(a) Estimate the thermal phase drift in 1 ns, assuming the silicon chip is at room temperature with thermal fluctuations $\delta T = 1$ mK (from thermal noise), and the thermo-optic coefficient $dn/dT = 2 \times 10^{-4}$/K.

(b) Compare to the required precision. Is thermal noise a fundamental problem for 8-bit photonic computing?

(c) Now consider a laser with phase noise: the Schawlow-Townes linewidth $\Delta\nu = 1$ kHz. In a 1 ns integration window, what is the rms phase noise accumulated by the laser?

(d) If we use the full 1 ms coherence time of this laser, how many matrix-vector multiplies (at 1 ns each) can be performed before the accumulated phase drift causes a 1-radian phase error? Is this practically useful?

---

*These exercises span numerical computation, physical derivation, conceptual reasoning, and system design — the four modes of physical thinking needed for photonic computing work.*
