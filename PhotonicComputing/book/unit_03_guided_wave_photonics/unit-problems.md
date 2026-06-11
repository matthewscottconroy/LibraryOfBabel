# Unit III Problem Set: Guided-Wave Photonics

*Problems covering waveguide theory, optical fiber, and silicon photonics. Chapters 6–8.*

---

## Chapter 6: Waveguides and Fiber

**Problem 6.1** [Easy]
A symmetric slab waveguide has core index $n_1 = 3.47$ (silicon), cladding index $n_2 = 1.44$ (SiO₂), core thickness $d = 220$ nm, wavelength $\lambda = 1550$ nm.

(a) The V-number is $V = (\pi d/\lambda)\sqrt{n_1^2 - n_2^2}$. Compute $V$.

(b) The single-mode condition for a symmetric slab is $V < \pi/2$. Is this waveguide single-mode?

(c) What is the maximum core thickness for single-mode guidance at 1550 nm?

(d) The numerical aperture is $\text{NA} = \sqrt{n_1^2 - n_2^2}$. Compute it. What is the acceptance half-angle?

**Problem 6.2** [Easy]
Optical fiber: a step-index single-mode fiber (SMF-28) has $n_\text{core} = 1.4681$, $n_\text{clad} = 1.4628$, core radius $a = 4.1$ μm at $\lambda = 1310$ nm.

(a) Compute the V-number. The single-mode condition for fiber is $V < 2.405$.

(b) The effective refractive index lies between $n_\text{clad}$ and $n_\text{core}$. Using the weakly guiding approximation, estimate $n_\text{eff}$.

(c) Chromatic dispersion $D = -(λ/c)(d^2n_\text{eff}/d\lambda^2)$ is $17$ ps/(nm·km) at 1550 nm for SMF-28. How much does a 10 Gb/s pulse (bit duration 100 ps) spread after 10 km? After 80 km?

(d) At what transmission distance does the pulse spread equal the bit period (ISI limit)?

**Problem 6.3** [Medium]
TE mode eigenvalue equation for a symmetric slab waveguide: $\kappa\tan(\kappa d/2) = \gamma$ (even modes), where $\kappa = \sqrt{n_1^2 k_0^2 - \beta^2}$ and $\gamma = \sqrt{\beta^2 - n_2^2 k_0^2}$.

(a) Define $U = \kappa d/2$ and $W = \gamma d/2$. Show that $U^2 + W^2 = (kd/2)^2(n_1^2-n_2^2) = (V/2)^2$.

(b) The eigenvalue equation in $(U,W)$ coordinates: $U\tan U = W$ with $U^2 + W^2 = (V/2)^2$. For $V = 1.5$, graphically (or numerically) find the solution for the fundamental (even) TE mode.

(c) From the solution, compute $\beta$ and the effective index $n_\text{eff} = \beta/k_0$.

(d) What happens to $n_\text{eff}$ as $V\to 0$ (thin waveguide limit)? As $V\to\infty$ (thick waveguide)?

**Problem 6.4** [Medium]
Directional coupler in silicon: two 450 nm × 220 nm silicon strip waveguides separated by a gap $g = 200$ nm, at $\lambda = 1550$ nm. Numerical simulation gives $\kappa = 1.2$ rad/μm (coupling coefficient) and $\Delta\beta = 0$ (identical waveguides).

(a) Find the coupling length $L_c = \pi/(2\kappa)$ for complete power transfer.

(b) Design a 50/50 (3 dB) coupler. What is the required length?

(c) If the fabrication gives $g = 200 \pm 20$ nm, how much does $\kappa$ vary? Assuming $\kappa \propto e^{-\gamma_c g}$ with $\gamma_c = 5$ μm⁻¹, compute $\Delta\kappa$ and the resulting variation in splitting ratio.

(d) A wavelength-division-multiplexed (WDM) system uses this coupler. The coupling coefficient changes by $d\kappa/d\lambda = -0.02$ (rad/μm)/nm. Over the C-band (1530–1565 nm), what is the worst-case splitting ratio variation?

**Problem 6.5** [Medium]
Microring resonator: ring radius $R = 5$ μm, $n_\text{eff} = 2.5$, waveguide-ring power coupling coefficient $\kappa^2 = 0.1$, propagation loss $\alpha = 3$ dB/cm.

(a) Find the resonant wavelengths $\lambda_m = 2\pi R n_\text{eff}/m$ (free spectral range).

(b) The round-trip amplitude transmission: $a = e^{-\alpha\cdot 2\pi R}$. Compute $a$.

(c) At the critical coupling condition $t = a$ ($t = \sqrt{1-\kappa^2}$), the resonator transmission drops to zero. Is this ring critically coupled?

(d) The loaded $Q$ factor at resonance: $Q = \omega_0 \tau_\text{phot}/2$ where $\tau_\text{phot} = t_\text{RT}/(-\ln(t\cdot a))$ is the photon lifetime per round trip time $t_\text{RT} = 2\pi R n_g/c$. For $n_g = 3.5$: compute $Q$.

**Problem 6.6** [Hard]
*Hint: Start with the transfer matrix of the full MZI and expand the coupling coefficient as $\kappa(\omega) = \kappa_0 + (\partial\kappa/\partial\omega)\Delta\omega$.*

Wavelength-flattened directional coupler: a standard directional coupler has a sinusoidal splitting ratio $\eta(\lambda) = \sin^2(\kappa(\lambda)L)$, which is strongly wavelength-dependent. Design a "wavelength-flattened coupler" using two coupler sections with different coupling gaps.

(a) For two cascaded sections with coupling coefficients $\kappa_1, \kappa_2$ and lengths $L_1, L_2$: write the transfer matrix.

(b) The condition for wavelength-flat 50/50 splitting over a 40 nm band centered at 1550 nm requires $d\eta/d\lambda = 0$ at 1550 nm and $d^2\eta/d\lambda^2 = 0$. Write these conditions in terms of $\kappa_1, L_1, \kappa_2, L_2$.

(c) If $\kappa_1 = 0.8$ rad/μm and $\kappa_2 = 1.4$ rad/μm (different gaps), solve for $L_1$ and $L_2$ to first order.

---

## Chapter 7: Silicon Photonics

**Problem 7.1** [Easy]
Silicon photonic MZI modulator: two arms each 1 cm long. One arm has a PN junction with doping $N_d = N_a = 10^{17}$ cm⁻³. At 0 V bias, depletion width $W = 65$ nm. The optical mode overlap with the depletion region is $\Gamma = 0.3$.

The free-carrier-induced index change: $\Delta n = -8.8\times10^{-22}N_e - 8.5\times10^{-18}N_h^{0.8}$ (Soref & Bennett, SI units).

(a) Under $-3$ V reverse bias, depletion width increases to 130 nm (doubled). The change in carrier density $\Delta N_e \approx N_d\Delta W/W_0$. Compute $|\Delta n|$.

(b) Compute the phase shift $\Delta\Phi = (2\pi/\lambda)\Gamma|\Delta n| L$.

(c) Compute $V_\pi$.

(d) The modulator also has absorption change $\Delta\alpha = 8.5\times10^{-18}\Delta N_h^{0.8}$ cm⁻¹. Estimate the insertion loss change (in dB) at $-3$ V.

**Problem 7.2** [Medium]
Thermo-optic phase shifter: silicon ($dn/dT = 1.84\times10^{-4}$ K⁻¹). A resistive heater above the waveguide dissipates power $P_h$ and heats the waveguide by $\Delta T = \theta_\text{th} P_h$ where $\theta_\text{th} = 50$ K/mW (typical thermal resistance).

(a) Find the heater power needed for a $\pi$ phase shift in a 500 μm long arm at $\lambda = 1550$ nm.

(b) Thermal time constant $\tau_\text{th} = R_\text{th}C_\text{th}$ (thermal resistance × heat capacity). For $\tau_\text{th} = 100$ μs: what is the modulation bandwidth?

(c) Compare the energy per switching operation ($E = P \times \tau$) for thermo-optic vs. electro-optic (silicon PN junction, $V_\pi = 8$ V, $C = 0.5$ pF).

**Problem 7.3** [Hard]
*Hint: Use the group delay $\tau_g(\omega) = -d\phi/d\omega$ where $\phi$ is the round-trip phase of the ring. The dispersion of $\tau_g$ gives GVD.*

Dispersion engineering with coupled ring resonators: A bus waveguide coupled to two ring resonators (radii $R$, coupling $\kappa_j$) can engineer the group velocity dispersion (GVD) of the effective waveguide.

(a) Write the transmission function $T(\omega)$ of the double-ring system using temporal CMT. Identify the two resonant poles.

(b) The group delay $\tau_g(\omega) = -d\arg(T)/d\omega$. Near a resonance, $\tau_g$ has a Lorentzian shape. Find the peak group delay and bandwidth.

(c) Show that with appropriate coupling parameters, the GVD $\beta_2 = d^2\beta/d\omega^2$ can be made anomalous (negative) near the resonance, even in a normally dispersive waveguide. This is the basis for dispersion-engineered photonic crystal fibers and ring-resonator dispersion compensation.

---

## Chapter 8: Photonic Crystals and Metamaterials

**Problem 8.1** [Easy]
A 1D photonic crystal is a periodic stack with alternating layers of $n_H = 3.5$ and $n_L = 1.5$, each quarter-wavelength thick ($d_H = \lambda_0/(4n_H)$, $d_L = \lambda_0/(4n_L)$) for $\lambda_0 = 1550$ nm.

(a) Compute $d_H$ and $d_L$.

(b) The photonic band gap center is at $\lambda_0$. The fractional band gap width is $\Delta\omega/\omega_0 \approx (4/\pi)\arcsin[(n_H-n_L)/(n_H+n_L)]$. Compute $\Delta\lambda$.

(c) After $N = 20$ periods, the reflectance at the band center is $R = [1 - (n_L/n_H)^{2N}n_s/n_0]^2/[1 + (n_L/n_H)^{2N}n_s/n_0]^2$. For $n_s = n_0 = 1$: compute $R$.

**Problem 8.2** [Medium]
Negative refraction and superlensing: a slab of thickness $d$ with $n = -1$ (left-handed metamaterial) focuses rays from a point source. Unlike a conventional lens, it preserves evanescent waves.

(a) Using Snell's law with $n_2 = -1$ and $n_1 = 1$: for an incident ray at angle $\theta_i$, find $\theta_t$. Sketch the ray diagram for a point source and a $n = -1$ slab.

(b) The perfect lens recovers evanescent waves with wave vector $k_x > k_0$. These waves decay as $e^{-\kappa z}$ in free space but grow as $e^{+\kappa z}$ in the negative-index medium. Show that the transfer function for the slab is $T = e^{-\kappa d}\cdot e^{+\kappa d}\cdot e^{-\kappa d} = e^{-\kappa d}$ — the same as free-space propagation of distance $d$. What does this mean for image resolution?

(c) Why are practical negative-index metamaterials lossy, and how does loss limit the achievable resolution?
