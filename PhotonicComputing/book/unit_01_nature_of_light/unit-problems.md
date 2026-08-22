# Unit I Problem Set: The Nature of Light — Classical Electromagnetism

*These problems span Chapters 1–3 of Unit I, covering Maxwell's equations, electromagnetic waves, wave optics, and light-matter interaction. Problems are labelled: [Easy], [Medium], or [Hard]. Hints are provided for Hard problems.*

---

## Chapter 1: Maxwell's Equations and Electromagnetic Waves

**Problem 1.1** [Easy]
A plane wave travels in the $+z$ direction with electric field $\mathbf{E} = E_0\hat{x}\cos(kz - \omega t)$ where $E_0 = 10^4$ V/m, $\lambda = 1550$ nm.

(a) Find $k$ and $\omega$.
(b) Find the magnetic field $\mathbf{B}(z,t)$.
(c) Compute the Poynting vector $\mathbf{S} = (1/\mu_0)\mathbf{E}\times\mathbf{B}$.
(d) What is the intensity (time-averaged power per unit area)?
(e) How many photons per second pass through a $1\,\text{mm}^2$ cross-section?

**Problem 1.2** [Easy]
Starting from the differential form of Maxwell's equations in free space, derive the wave equation for the magnetic field $\mathbf{B}$. Do not just state the result — show each step, using the vector identity $\nabla\times(\nabla\times\mathbf{F}) = \nabla(\nabla\cdot\mathbf{F}) - \nabla^2\mathbf{F}$.

**Problem 1.3** [Medium]
A dielectric medium has complex permittivity $\varepsilon = \varepsilon'(1 + i\tan\delta)$ where $\tan\delta = 0.01$ is the loss tangent. A plane wave enters this medium.

(a) Write the wave equation in complex notation. Find the complex wavenumber $\tilde{k}$.
(b) Show that the field decays as $e^{-\alpha z}$ where $\alpha$ is the absorption coefficient. Find $\alpha$ in terms of $\varepsilon'$, $\tan\delta$, and $\omega$.
(c) For $\varepsilon' = 12$ (silicon), $\lambda_0 = 1550$ nm, and $\tan\delta = 5\times10^{-4}$ (near-infrared silicon loss): compute the power loss in dB/cm.

**Problem 1.4** [Medium]
Fresnel coefficients at a planar interface between media $n_1$ and $n_2$ (derived from boundary conditions):

$$r_s = \frac{n_1\cos\theta_i - n_2\cos\theta_t}{n_1\cos\theta_i + n_2\cos\theta_t}, \qquad r_p = \frac{n_2\cos\theta_i - n_1\cos\theta_t}{n_2\cos\theta_i + n_1\cos\theta_t}$$

(a) For normal incidence ($\theta_i = 0$): show $r_s = (n_1-n_2)/(n_1+n_2)$.
(b) Compute the reflectance $R = r_s^2$ at a Si/air interface ($n_\text{Si} = 3.47$) at normal incidence.
(c) At what angle (Brewster's angle) does $r_p = 0$? Express in terms of $n_1$, $n_2$.
(d) At a Si/SiO₂ interface ($n_\text{Si} = 3.47$, $n_{\text{SiO}_2} = 1.44$), find the critical angle for total internal reflection.

**Problem 1.5** [Medium]
Radiation pressure: A 1 mW CW laser beam (diameter 2 mm, $\lambda = 532$ nm) is focused onto a perfectly reflective mirror.

(a) What is the radiation pressure on the mirror?
(b) If the mirror has mass $m = 1$ mg, what acceleration does it experience?
(c) Optical tweezers: estimate the trapping force on a polystyrene bead (radius $a = 1$ μm, $n = 1.59$) in water ($n = 1.33$) illuminated by a 10 mW beam. Use the approximation $F \sim nP/c$ (gradient force order of magnitude).
(d) Why can optical tweezers stably trap a particle? What determines whether the force is attractive or repulsive?

**Problem 1.6** [Hard]
*Hint: Use the Lorentz model for atomic polarizability, $\tilde{\chi}(\omega) = \omega_p^2/(\omega_0^2 - \omega^2 - i\gamma\omega)$, and separate real and imaginary parts.*

The complex susceptibility of a dielectric near an atomic resonance $\omega_0$ can be written:

$$\chi(\omega) = \chi'(\omega) + i\chi''(\omega)$$

(a) Show that the refractive index $n \approx 1 + \chi'/2$ (for $|\chi|\ll 1$) and the absorption coefficient $\alpha = \omega\chi''/(nc)$.

(b) Using the Lorentz model, show that near resonance ($\omega \approx \omega_0$), the refractive index shows **anomalous dispersion** ($dn/d\omega < 0$) precisely in the region of maximum absorption. Sketch $\chi'(\omega)$ and $\chi''(\omega)$.

(c) The Kramers-Kronig relations connect $\chi'$ and $\chi''$:
$$\chi'(\omega) = \frac{2}{\pi} \text{P.V.}\int_0^\infty \frac{\omega'\chi''(\omega')}{\omega'^2-\omega^2}d\omega'$$
Verify this is satisfied by the Lorentz model (it is sufficient to verify the sign and general form; you do not need to evaluate the principal value integral).

---

## Chapter 2: Wave Optics

**Problem 2.1** [Easy]
Young's double-slit experiment: two slits separated by $d = 0.1$ mm are illuminated by a coherent plane wave at $\lambda = 550$ nm. The screen is at $L = 1$ m.

(a) Find the fringe spacing $\Delta y = \lambda L/d$.
(b) The intensity pattern is $I(y) = 4I_0\cos^2(\pi dy/\lambda L)$. Find the positions of the first five bright fringes.
(c) One slit is now covered by a thin glass plate of thickness $t = 1$ μm and index $n = 1.5$. By how much does the central fringe shift (in mm)?
(d) If the coherence length of the source is $L_c = 0.1$ mm, does the fringe pattern survive? Why or why not?

**Problem 2.2** [Medium]
Fabry-Pérot interferometer: a cavity of length $L$ with mirror reflectivities $R_1 = R_2 = R$.

(a) Derive the Airy function for the transmission:
$$T(\delta) = \frac{(1-R)^2}{1 + R^2 - 2R\cos\delta}$$
where $\delta = 4\pi n L/\lambda$ is the round-trip phase.

(b) Define the finesse $\mathcal{F} = \pi\sqrt{R}/(1-R)$. For $R = 0.99$: compute $\mathcal{F}$.

(c) The free spectral range is $\Delta\nu_\text{FSR} = c/(2nL)$. For $L = 10$ cm: find $\Delta\nu_\text{FSR}$.

(d) The FWHM linewidth is $\Delta\nu = \Delta\nu_\text{FSR}/\mathcal{F}$. Compute it.

(e) Why is the finesse limited in real Fabry-Pérot cavities? List three physical mechanisms.

**Problem 2.3** [Medium]
A Michelson interferometer is used to measure the coherence length of an LED with central wavelength 850 nm and spectral FWHM 30 nm.

(a) The visibility of fringes as a function of path difference $\Delta = 2\delta$ (arm length difference $\delta$) is the Fourier transform of the source spectrum. For a Gaussian spectrum, derive the visibility $V(\Delta) = \exp(-\Delta^2/L_c^2)$ and find the coherence length $L_c$.

(b) At what path difference does the visibility drop to $1/e$?

(c) How many fringes are visible within the coherence length?

**Problem 2.4** [Hard]
*Hint: Use the convolution theorem. The PSF is the squared modulus of the Fourier transform of the pupil function.*

The Point Spread Function (PSF) of a circular aperture of radius $R$ at wavelength $\lambda$, at focal distance $f$:

$$\text{PSF}(r) = \left|\frac{2J_1(2\pi R r/\lambda f)}{2\pi R r/\lambda f}\right|^2 \cdot (\pi R^2)^2$$

where $J_1$ is the Bessel function of the first kind.

(a) Find the radius of the Airy disk (first zero of $J_1(x)$, at $x = 3.832$) in terms of $\lambda$, $f$, $R$.

(b) What is the spatial frequency cutoff of the optical system? Express in lines/mm for $f/R = 5$ (i.e., $f$-number $= 5$), $\lambda = 550$ nm.

(c) The Strehl ratio is defined as $S = \text{PSF}(0)_\text{aberrated}/\text{PSF}(0)_\text{ideal}$. For a small phase aberration $\phi(\mathbf{r}) = \epsilon \cos(2\theta)$ (astigmatism), show that $S \approx 1 - \text{Var}(\phi)$ for small $\epsilon$.

---

## Chapter 3: Light-Matter Interaction

**Problem 3.1** [Easy]
A two-level atom with transition wavelength $\lambda = 780$ nm (rubidium D2 line), upper-state lifetime $\tau = 26$ ns.

(a) What is the Einstein $A$ coefficient $A_{21} = 1/\tau$?
(b) What is the natural linewidth $\Delta\nu_\text{nat} = A_{21}/(2\pi)$?
(c) At room temperature ($T = 300$ K), what is the Doppler linewidth $\Delta\nu_D = (2\nu_0/c)\sqrt{2k_BT\ln 2/m}$ for Rb (mass $m = 85$ amu)?
(d) Which linewidth dominates? Is this homogeneous or inhomogeneous broadening?

**Problem 3.2** [Medium]
Population inversion in a four-level laser system: pump rate $R_p$ (atoms/s) pumps the ground state $|0\rangle$ to level $|3\rangle$, which decays rapidly to $|2\rangle$ (upper laser level, lifetime $\tau_2 = 1$ ms). $|2\rangle$ lases to $|1\rangle$ (lower laser level), which decays rapidly to $|0\rangle$.

(a) Write rate equations for populations $N_2$ and $N_0 \approx N_T - N_2$ (where $N_T$ is total population density).

(b) In steady state without the laser field: find $N_2$ as a function of $R_p, \tau_2, N_T$.

(c) What pump rate $R_{p,\text{thresh}}$ gives $N_2 = N_T/2$ (transparency condition)?

(d) The gain coefficient is $g = \sigma_{21}(N_2 - N_1)\approx\sigma_{21}N_2$ for a good four-level system. For $\sigma_{21} = 3\times10^{-19}$ cm², $N_T = 10^{20}$ cm⁻³, find $g$ at $R_p = 2R_{p,\text{thresh}}$.

**Problem 3.3** [Medium]
Photodetector shot noise: a photodetector receives $P = 1$ μW of 1550 nm light and has quantum efficiency $\eta = 0.9$ and bandwidth $B = 10$ GHz.

(a) Compute the photocurrent $I_\text{ph} = \eta eP/(h\nu)$.
(b) Compute the shot noise current $I_\text{shot} = \sqrt{2eI_\text{ph}B}$.
(c) Compute the signal-to-noise ratio $\text{SNR} = I_\text{ph}^2/I_\text{shot}^2$.
(d) How much power is needed to achieve SNR $= 30$ dB?

**Problem 3.4** [Hard]
*Hint: Use the transfer matrix for a two-level system driven by a near-resonant field. The rotating wave approximation gives the Bloch equations.*

The optical Bloch equations for a two-level atom driven by a field of Rabi frequency $\Omega_R$ and detuning $\Delta = \omega - \omega_0$:

$$\dot{u} = -\Delta v, \qquad \dot{v} = \Delta u - \Omega_R w, \qquad \dot{w} = \Omega_R v$$

where $(u,v,w)$ is the Bloch vector.

(a) Show that $|u|^2 + |v|^2 + |w|^2$ is conserved. What does this represent physically?

(b) For exact resonance ($\Delta = 0$) and initial state $w(0) = -1$ (ground state): solve the Bloch equations. What is $w(t)$?

(c) What is the population inversion $P_e = (1+w)/2$ at time $t = \pi/(2\Omega_R)$ (a $\pi/2$ pulse)?

(d) A photon echo sequence uses $\pi/2$–wait $T$–$\pi$ pulses. Qualitatively describe what happens to the Bloch vector at each step, and why the echo forms at time $2T$.
