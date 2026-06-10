# Chapter 3 Exercises: Light-Matter Interaction

---

## Mathematical Exercises

**Exercise 3.1 — The Lorentz Oscillator in Detail**

The equation of motion for an electron bound in a Lorentz oscillator is:

$$m\ddot{x} + m\gamma\dot{x} + m\omega_0^2 x = -eE_0 e^{-i\omega t}$$

(a) Show that the steady-state solution is:

$$x(t) = -\frac{e}{m} \frac{E_0}{\omega_0^2 - \omega^2 - i\gamma\omega} e^{-i\omega t}$$

by substituting a trial solution $x(t) = x_0 e^{-i\omega t}$ and solving for $x_0$.

(b) Derive the power absorbed by the oscillator per unit time, $\langle P \rangle = \langle -eE \cdot \dot{x} \rangle$, and show it is proportional to the imaginary part of the susceptibility, $\chi''(\omega)$.

(c) In the limit of small damping ($\gamma \ll \omega_0$), show that the absorption line has a Lorentzian profile:

$$\chi''(\omega) \approx \frac{\omega_p^2}{2\omega_0} \cdot \frac{\gamma/2}{(\omega - \omega_0)^2 + (\gamma/2)^2}$$

What is the FWHM of this profile? What is the Q factor of the oscillator?

(d) A quantum dot emitting at 900 nm has a linewidth of 1 meV. Compute the Q factor and the spontaneous emission lifetime (assuming $A = B \cdot u(\omega)$ with $u \to 0$ so spontaneous emission dominates). *Hint: $\hbar\gamma = \Delta E$.*

---

**Exercise 3.2 — Kramers-Kronig Verification**

The complex susceptibility of the single-resonance Lorentz oscillator is:

$$\chi(\omega) = \frac{\omega_p^2}{\omega_0^2 - \omega^2 - i\gamma\omega}$$

(a) Write out $\chi'(\omega)$ and $\chi''(\omega)$ explicitly.

(b) Verify the first Kramers-Kronig relation numerically for the case $\omega_0 = 2\pi \times 10^{15}$ rad/s, $\gamma = 0.01\omega_0$, $\omega_p = 0.1\omega_0$: compute the integral

$$\chi'(\omega) = \frac{2}{\pi} \text{P.V.} \int_0^\infty \frac{\omega' \chi''(\omega')}{\omega'^2 - \omega^2} d\omega'$$

numerically (e.g., using Python's `scipy.integrate.quad`) at several values of $\omega$, and check that it matches the analytical $\chi'(\omega)$.

(c) The f-sum rule states $\int_0^\infty \chi''(\omega) d\omega = \pi\omega_p^2/2$. Verify this analytically for the Lorentz oscillator.

(d) Why does the f-sum rule constrain the performance of a silicon electro-optic modulator? If you use the plasma dispersion effect to reduce the refractive index at $\omega_0$ (causing phase shift), what must happen to absorption elsewhere? Is this a problem for the C-band? Why or why not?

---

**Exercise 3.3 — Einstein Coefficients and Laser Threshold**

A four-level laser system has:
- Upper laser level lifetime: $\tau_{21} = 1/(A_{21}) = 1$ ns
- Stimulated emission cross-section: $\sigma = 4 \times 10^{-19}$ cm²
- Cavity length: $L = 10$ cm (mode confined to active medium)
- Cavity mirror reflectivities: $R_1 = 1.0$ (back mirror), $R_2 = 0.95$ (output coupler)
- Passive cavity round-trip loss: $\delta = 0.01$ (1% distributed loss)

(a) Derive the round-trip gain condition for lasing threshold. Show that threshold inversion density $\Delta N_{th}$ satisfies:

$$\sigma \Delta N_{th} L = \frac{1}{2}\left[\ln\frac{1}{R_1 R_2} + \delta\right]$$

(b) Compute $\Delta N_{th}$ in cm$^{-3}$.

(c) What pump power (in units of photon flux, $\phi_p$ in cm$^{-2}$s$^{-1}$) is needed to maintain this inversion? Assume the pump absorption cross-section is $\sigma_p = 1 \times 10^{-19}$ cm² and the ground-state population is $N_0 = 10^{19}$ cm$^{-3}$.

(d) Above threshold, show that the output power grows linearly with pump power with slope efficiency:

$$\eta_s = \frac{\omega_L}{\omega_p} \cdot \frac{\ln(1/R_2)}{\ln(1/R_1 R_2) + \delta}$$

What is the slope efficiency for this laser, assuming $\omega_L/\omega_p = 0.8$ (quantum defect)?

---

**Exercise 3.4 — Second-Harmonic Generation**

The coupled amplitude equations for SHG with phase mismatch $\Delta k = k(2\omega) - 2k(\omega)$ are:

$$\frac{dA_1}{dz} = i\kappa_1 A_1^* A_2 e^{-i\Delta k z}$$
$$\frac{dA_2}{dz} = i\kappa_2 A_1^2 e^{+i\Delta k z}$$

where $\kappa_1, \kappa_2 \propto \chi^{(2)}$.

(a) Under the undepleted pump approximation ($A_1 \approx$ const), solve for $A_2(z)$ starting from $A_2(0) = 0$. Show that:

$$|A_2(L)|^2 \propto L^2 \text{sinc}^2\left(\frac{\Delta k L}{2}\right)$$

(b) Plot the SHG efficiency vs. $\Delta k L$ for $|\Delta k L| \in [-4\pi, 4\pi]$. What is the coherence length $L_c = \pi/|\Delta k|$ if $\Delta k = 100$ m$^{-1}$?

(c) In a lithium niobate crystal with $d_{33} = 27$ pm/V at 1550 nm, the fundamental beam has intensity $I_\omega = 100$ MW/cm². Calculate the expected SHG efficiency per unit length for a phase-matched crystal ($\Delta k = 0$).

(d) Quasi-phase matching using periodic poling inverts the sign of $\chi^{(2)}$ every coherence length. Show that the effective nonlinear coefficient for first-order QPM is $d_{eff} = (2/\pi)d_{33}$. What poling period $\Lambda$ is needed for 1550 → 775 nm SHG in LiNbO₃ where $n(1550) = 2.138$ and $n(775) = 2.186$?

---

**Exercise 3.5 — Kerr Effect and Self-Phase Modulation**

A Gaussian pulse $A(0,t) = A_0 \exp(-t^2/T_0^2)$ propagates through a nonlinear fiber with $\gamma = 2$ W$^{-1}$km$^{-1}$, $\beta_2 = -20$ ps²/km (anomalous dispersion), and no loss.

(a) Neglecting dispersion, show that after propagating distance $z$, the phase accumulated is:

$$\phi_{NL}(t) = \gamma |A(0,t)|^2 z = \gamma P_0 e^{-2t^2/T_0^2} z$$

What is the instantaneous frequency deviation $\delta\omega(t) = -\partial\phi_{NL}/\partial t$? Describe the sign of the frequency chirp at the leading and trailing edges.

(b) The nonlinear length is $L_{NL} = 1/(\gamma P_0)$ and the dispersion length is $L_D = T_0^2/|\beta_2|$. For a 1 ps pulse with $P_0 = 50$ mW, compute $L_{NL}$ and $L_D$.

(c) Compute the soliton number $N = \sqrt{L_D/L_{NL}}$. Is this pulse close to a fundamental soliton? What power would be needed for an exact fundamental soliton at $T_0 = 1$ ps?

(d) For the fundamental soliton, verify by substitution that $A(z,t) = A_0 \text{sech}(t/T_0) e^{iz\gamma P_0/2}$ satisfies the NLSE:

$$i\frac{\partial A}{\partial z} = \frac{\beta_2}{2}\frac{\partial^2 A}{\partial t^2} - \gamma|A|^2 A$$

Show that this requires $A_0 = \sqrt{P_0}$ where $P_0 = |\beta_2|/(\gamma T_0^2)$.

---

**Exercise 3.6 — Rayleigh and Raman Scattering**

(a) Rayleigh scattering in silica fiber has an attenuation coefficient:

$$\alpha_R = \frac{A}{\lambda^4}$$

where $A \approx 0.78$ dB·km$^{-1}$·μm$^4$ for standard silica. Compute the Rayleigh-limited loss at 850 nm, 1310 nm, and 1550 nm. Which window is limited by other mechanisms at shorter wavelengths?

(b) A 1000 km submarine cable at 1550 nm uses EDFA amplifiers spaced every 80 km. Accounting only for fiber attenuation (Rayleigh + infrared absorption at 1550 nm gives total loss 0.2 dB/km), how many amplifiers are needed and what total gain do they provide?

(c) The Raman gain spectrum in silica fiber peaks near 13.2 THz shift with $g_R \approx 10^{-13}$ m/W. For a pump at 1450 nm:
   - What is the peak Raman gain wavelength?
   - For a 10 km fiber with $A_{eff} = 80$ μm² and pump power 1 W, what is the small-signal Raman gain in dB?

(d) Stimulated Raman scattering has a threshold estimated by $g_R P_{th} L_{eff}/A_{eff} \approx 16$. Compute the SRS threshold power for a 50 km SMF-28 fiber ($A_{eff} = 80$ μm², $g_R = 10^{-13}$ m/W, $\alpha = 0.046$ km$^{-1}$). This sets an upper bound on signal power in WDM systems.

---

**Exercise 3.7 — Brillouin Scattering and Sensing**

(a) The Brillouin frequency shift for backward SBS in silica fiber is:

$$\nu_B = \frac{2n v_a}{\lambda}$$

where $n = 1.445$, $v_a = 5960$ m/s (acoustic velocity), and $\lambda = 1550$ nm. Compute $\nu_B$.

(b) The SBS threshold for a single-mode fiber is approximately:

$$P_{th} \approx \frac{21 A_{eff}}{g_B L_{eff}}$$

where $g_B = 5 \times 10^{-11}$ m/W for silica. For a 25 km fiber with $A_{eff} = 80$ μm² and $\alpha = 0.046$ km$^{-1}$:
   - Compute $L_{eff} = (1-e^{-\alpha L})/\alpha$
   - Compute $P_{th}$

(c) BOTDA temperature sensing exploits the fact that $\nu_B$ shifts by approximately 1.1 MHz/°C. If your Brillouin frequency analyzer has a spectral resolution of 1 MHz and a spatial resolution of 1 m (determined by the pump pulse duration), what is the temperature resolution of the distributed sensor? What is the temperature resolution if spatial averaging over 100 m is acceptable?

(d) Why is SBS the dominant nonlinear process limiting launch power in single-channel coherent fiber transmission, rather than SPM or SRS? (*Hint: Consider the thresholds and the linewidth of the interaction.*)

---

## Conceptual Exercises

**Exercise 3.8 — The Meaning of Complex Refractive Index**

The complex refractive index is $\tilde{n} = n + i\kappa$, where $n$ is the phase index and $\kappa$ is the extinction coefficient.

(a) Show that a plane wave $E = E_0 e^{i(kz - \omega t)}$ with $k = \tilde{n}\omega/c$ gives:

$$E(z,t) = E_0 e^{-\kappa\omega z/c} e^{i(n\omega z/c - \omega t)}$$

What is the intensity attenuation coefficient $\alpha$ in terms of $\kappa$ and $\lambda$?

(b) Silicon at 1550 nm has $n = 3.48$ and $\kappa \approx 0$ (transparent). Silicon at 400 nm has $n \approx 5.6$ and $\kappa \approx 0.39$. Why is silicon used in photonic chips at telecom wavelengths but not at visible wavelengths?

(c) A material has $\kappa(\omega) = 0$ for all $\omega$ (perfectly transparent at all frequencies). What does this imply about $n(\omega)$ via the Kramers-Kronig relations? Is such a material physically possible?

(d) Explain in physical terms why normal dispersion ($dn/d\omega > 0$) dominates below the resonance frequency of a Lorentz oscillator, while anomalous dispersion ($dn/d\omega < 0$) occurs near and above the resonance. Why is anomalous dispersion associated with gain in a laser amplifier?

---

**Exercise 3.9 — Population Inversion: Why It's Hard**

(a) A two-level system at thermal equilibrium has population ratio $N_2/N_1 = e^{-\hbar\omega/k_BT}$. For a transition at $\lambda = 1550$ nm, what temperature would be needed to have $N_2/N_1 = 0.01$? $N_2/N_1 = 0.5$? What does this tell you about why a laser cannot be made from a two-level system in thermal equilibrium?

(b) In a three-level system, pumping populates level 3, which rapidly relaxes to level 2 (the upper laser level). The laser transition is 2 → 1, where level 1 is the ground state. Show that population inversion ($N_2 > N_1$) requires that more than half the atoms be pumped out of the ground state. How does this compare to a four-level system?

(c) An erbium-doped fiber amplifier (EDFA) is a three-level system at 1550 nm. What does this imply about the noise figure of an EDFA? Specifically: at threshold inversion (all atoms in upper state), what is the minimum noise figure? (*Hint: The noise figure is $F = 2n_{sp}$ where $n_{sp} = N_2/(N_2 - N_1)$ is the spontaneous emission factor.*)

(d) Silicon cannot lase because of its indirect bandgap. Explain what "indirect" means in terms of momentum conservation, and why phonon-assisted transitions make stimulated emission extremely inefficient in silicon compared to GaAs or InP.

---

**Exercise 3.10 — Nonlinear Optics: When Does It Matter?**

(a) The second-order susceptibility $\chi^{(2)}$ vanishes in centrosymmetric materials. Silicon has a diamond cubic structure with inversion symmetry. Does bulk silicon have $\chi^{(2)}$? Does the silicon-SiO₂ interface have $\chi^{(2)}$? What physical symmetry argument explains the difference?

(b) The nonlinear refractive index of silicon is $n_2 \approx 6 \times 10^{-18}$ m²/W, compared to silica fiber $n_2 \approx 2.6 \times 10^{-20}$ m²/W. However, silicon waveguides also suffer from two-photon absorption (TPA) with coefficient $\beta_{TPA} \approx 5 \times 10^{-12}$ m/W. Define the figure of merit $\text{FOM} = n_2/(\lambda \beta_{TPA})$ for nonlinear optics without TPA penalty. Compute the FOM for silicon at 1550 nm. Is it above or below unity (the threshold for useful nonlinear processing)?

(c) Silicon nitride (Si₃N₄) has $n_2 \approx 2.4 \times 10^{-19}$ m²/W and negligible TPA at 1550 nm. Compare the Si₃N₄ FOM to silicon. Why might Si₃N₄ be preferred for Kerr-effect-based photonic computing, even though its $n_2$ is smaller than silicon?

(d) In a microring resonator, the effective nonlinearity is enhanced by the power buildup factor $|E_{circ}/E_{in}|^2 = (1-R)/[(1-R)^2 + 4R\sin^2(\phi/2)]$ (FP-like). For a ring with finesse $\mathcal{F} = 100$, how much does the effective $\gamma$ increase? What constraint does this place on the Q factor if you want to use Kerr nonlinearity for photonic computing?

---

**Exercise 3.11 — Coherence and Photon Statistics**

(a) Laser light above threshold has Poissonian photon statistics: $p(n) = \bar{n}^n e^{-\bar{n}}/n!$. Show that the variance equals the mean: $\langle(\Delta n)^2\rangle = \bar{n}$.

(b) Thermal (chaotic) light has super-Poissonian statistics with $\langle(\Delta n)^2\rangle = \bar{n} + \bar{n}^2$. Why is this relevant to the noise performance of a photonic neural network that uses incoherent (LED or ASE) light sources vs. laser sources?

(c) The degree of first-order temporal coherence is:

$$g^{(1)}(\tau) = \frac{\langle E^*(t) E(t+\tau) \rangle}{\langle |E(t)|^2 \rangle}$$

For a Lorentzian lineshape laser with linewidth $\Delta\nu$, show that $|g^{(1)}(\tau)| = e^{-\pi\Delta\nu|\tau|}$, and identify the coherence time $\tau_c = 1/(\pi\Delta\nu)$.

(d) A photonic computing system uses a MZI with arm length difference $\Delta L = 2$ mm (path length mismatch). What minimum coherence length $L_c = c\tau_c$ does the laser source need for the interference visibility to exceed 0.99? What linewidth does this correspond to?

---

**Exercise 3.12 — Dispersion Engineering**

Group velocity dispersion in an optical waveguide has two contributions: material dispersion from the glass, and waveguide dispersion from the geometry.

(a) The material dispersion of silica is approximately:

$$D_{\text{mat}} \approx -\frac{\lambda}{c}\frac{d^2 n}{d\lambda^2}$$

Using the Sellmeier equation (three-term form for silica: $A_1 = 0.6961663$, $\lambda_1 = 0.0684043$ μm; $A_2 = 0.4079426$, $\lambda_2 = 0.1162414$ μm; $A_3 = 0.8974794$, $\lambda_3 = 9.896161$ μm), numerically compute $D_{\text{mat}}$ at 1310 nm and 1550 nm. (Use Python or a calculator.)

(b) A 450 × 220 nm silicon strip waveguide has a total group velocity dispersion of approximately $D = -1000$ ps/nm/km at 1310 nm and $D = +2000$ ps/nm/km at 1550 nm (anomalous), dominated by waveguide dispersion. If this waveguide is used as a fiber-optic pulse compressor, which wavelength gives anomalous dispersion (suitable for soliton propagation)?

(c) The zero-dispersion wavelength $\lambda_{ZD}$ of standard SMF-28 fiber is near 1310 nm. Why does WDM transmission in the C-band (1530–1565 nm, anomalous dispersion) use dispersion-shifted or dispersion-compensating fiber rather than operating near $\lambda_{ZD}$? (*Hint: Consider four-wave mixing.*)

(d) Microresonator frequency combs (Kerr combs) require anomalous GVD. A microring resonator made of Si₃N₄ with width 800 nm and height 600 nm has waveguide GVD that can be tuned to give anomalous dispersion at 1550 nm. Qualitatively explain why reducing the waveguide height tends to increase anomalous waveguide dispersion. (*Hint: Think about the effective index and its frequency dependence.*)

---

## Laboratory and Computational Exercises

**Lab 3.1 — Lorentz Oscillator Simulation**

Write a Python simulation of the complex refractive index for a material with multiple Lorentz resonances, representative of silica glass.

(a) Implement the Sellmeier equation for silica (three-term, from Malitson 1965) and plot $n(\lambda)$ from 200 nm to 2000 nm.

(b) Add a phenomenological damping to convert the Sellmeier model into a complex susceptibility. Plot $n(\lambda)$ and $\kappa(\lambda)$. At what wavelengths does silica absorb strongly?

(c) Numerically verify the Kramers-Kronig relation by computing the Hilbert transform of $\chi''(\omega)$ and checking that it gives $\chi'(\omega)$.

(d) Add the Drude contribution for a free-carrier density of $N = 10^{18}$ cm$^{-3}$ (typical for a doped silicon modulator). Plot the change in refractive index $\Delta n$ and extinction coefficient $\Delta\kappa$ vs. wavelength from 1200–1700 nm, and compare to the Soref-Bennett empirical coefficients.

---

**Lab 3.2 — Second-Harmonic Generation: Coupled Wave Integration**

Numerically integrate the coupled SHG equations beyond the undepleted pump approximation.

(a) For a 5 mm LiNbO₃ crystal, $d_{33} = 27$ pm/V, and a fundamental intensity of 1 GW/cm², integrate the coupled equations numerically using `scipy.integrate.odeint`. Plot $I_\omega(z)$ and $I_{2\omega}(z)$ for phase-matched ($\Delta k = 0$) and mismatched ($\Delta k = 1000$ m$^{-1}$) cases.

(b) Observe that in the phase-matched case, the fields exchange energy periodically (back-conversion). At what crystal length does maximum conversion occur?

(c) Implement quasi-phase matching by making $d_{eff}(z) = d_{33} \cdot \text{sign}[\cos(2\pi z/\Lambda)]$ where $\Lambda$ is the poling period. Show that QPM suppresses back-conversion. How does the QPM efficiency compare to perfect phase matching with $d_{33}$?

(d) The coupled equations conserve the Manley-Rowe relation: $I_\omega/({\hbar\omega}) + 2I_{2\omega}/(\hbar \cdot 2\omega) = $ const. Verify this numerically throughout your integration.

---

**Lab 3.3 — Nonlinear Schrödinger Equation: Split-Step Simulation**

Implement the split-step Fourier method to solve the NLSE for pulse propagation in a nonlinear fiber.

The NLSE in the retarded frame:

$$\frac{\partial A}{\partial z} = -\frac{i\beta_2}{2}\frac{\partial^2 A}{\partial t^2} + i\gamma|A|^2 A - \frac{\alpha}{2}A$$

(a) Use parameters for SMF-28 at 1550 nm: $\beta_2 = -21.7$ ps²/km, $\gamma = 1.3$ W$^{-1}$km$^{-1}$, $\alpha = 0.046$ km$^{-1}$ (0.2 dB/km). Start with a Gaussian pulse of $T_{FWHM} = 1$ ps, $P_0 = 100$ mW. Propagate for 10 km and plot the output pulse intensity and spectrum.

(b) For a soliton input $A(0,t) = \sqrt{P_{sol}}\,\text{sech}(t/T_0)$ where $P_{sol} = |\beta_2|/(\gamma T_0^2)$, verify that the pulse shape is preserved after propagation. What happens if $P_0 = 4 P_{sol}$ (N=2 soliton)? Describe the qualitative behavior.

(c) Add stimulated Raman scattering by including the Raman response function. The Raman term shifts the soliton center frequency: $d\omega_0/dz = -\delta_R T_R P_0/(T_0^2)$. Measure the Raman-induced frequency shift rate in your simulation and compare to the analytical formula.

(d) Model a microring resonator Kerr comb: use periodic boundary conditions (round trip propagation), include anomalous dispersion, a Kerr nonlinearity, and a driving pump term at a single frequency. Show that beyond a threshold pump power, new frequency components appear (primary comb), and with increased driving, a dense comb (dissipative Kerr soliton state) can form.

---

**Lab 3.4 — Raman Amplifier Design**

Design a distributed Raman amplifier for a 100-km fiber span.

(a) The Raman gain coefficient for silica at 1450 nm pump: $g_R = 1 \times 10^{-13}$ m/W. The 1550 nm signal propagates with loss $\alpha_s = 0.046$ km$^{-1}$; the pump propagates backward with loss $\alpha_p = 0.053$ km$^{-1}$.

Write differential equations for signal and pump:

$$\frac{dP_s}{dz} = g_R P_p(z) P_s - \alpha_s P_s$$
$$-\frac{dP_p}{dz} = -g_R \frac{\omega_p}{\omega_s} P_p P_s - \alpha_p P_p$$

(b) In the small-signal approximation (neglect pump depletion), solve for $P_s(z)$ and find the pump power needed for the signal to experience zero net loss over 100 km (unity gain).

(c) Including pump depletion, integrate the coupled equations numerically. Find the pump power for unity gain at 100 km.

(d) Compare distributed Raman amplification to lumped EDFA amplification: for a 100 km span, which gives better noise figure? (*Hint: The effective noise figure of a distributed amplifier is $F_{eff} = F_{sp} \cdot \exp(-\alpha L_{eff})$ where $F_{sp}$ is the spontaneous emission factor.*) Why do submarine cables often use Raman + EDFA hybrid amplification?

---

## Thought Experiments

**Exercise 3.13 — A Photon in Silicon**

Trace a single photon at 1550 nm from its entry into a silicon photonic chip via a grating coupler through a sequence of components.

At each stage, identify: (a) what physical interaction governs the behavior, (b) what approximations are being made (classical EM, semiclassical, quantum), and (c) what loss mechanisms are present.

Stages: grating coupler → silicon ridge waveguide → directional coupler → ring resonator (drop port) → germanium photodetector.

---

**Exercise 3.14 — Why Not X-Rays?**

Photonic computing uses 1550 nm infrared light rather than, say, X-rays (0.1 nm) or UV (250 nm). Using the physics developed in this chapter, explain in physical terms why shorter wavelengths are not advantageous:

(a) At 250 nm: What does the Kramers-Kronig relation predict about absorption in silicon at UV wavelengths? Is SiO₂ still transparent?

(b) At 0.1 nm: Can you make waveguides for X-rays? (*Hint: For X-rays, $n < 1$ in all materials — what does total internal reflection require?*)

(c) At 10 μm (CO₂ laser): What is the Rayleigh scattering loss? What about molecular absorption in silica? Why is the transparency window of silica bounded on the long wavelength side?

(d) Having eliminated UV and IR extremes, explain why the C-band (1530–1565 nm) specifically became the standard for both fiber optics and photonic computing, connecting to: silica transparency minimum, EDFA gain bandwidth, and silicon waveguide properties.
