# 2.1.2 — Snell's Law: A Wave Derivation

## Why Derive Snell's Law Again?

Snell's law — $n_1 \sin\theta_1 = n_2 \sin\theta_2$ — is usually presented as an empirical fact, or derived from Fermat's principle, or just stated and used. But the wave derivation is worth doing carefully, because it reveals the underlying physics: Snell's law is a *phase matching* condition, and phase matching is one of the most important concepts in photonic computing (it appears again in waveguide coupling, grating couplers, nonlinear frequency conversion, and photonic crystal design).

## Setup: A Plane Wave at an Interface

Consider a monochromatic plane wave in medium 1 (refractive index $n_1$) incident on a flat interface at $z = 0$ with medium 2 (refractive index $n_2$). The incident wave propagates at angle $\theta_i$ to the interface normal:

$$\mathbf{E}_i = \mathbf{E}_{0i} \, e^{i(\mathbf{k}_i \cdot \mathbf{r} - \omega t)}$$

where $\mathbf{k}_i = k_i(\sin\theta_i \, \hat{x} + \cos\theta_i \, \hat{z})$ and $k_i = n_1 \omega/c$.

At the interface, we expect a reflected wave:

$$\mathbf{E}_r = \mathbf{E}_{0r} \, e^{i(\mathbf{k}_r \cdot \mathbf{r} - \omega t)}$$

and a transmitted wave:

$$\mathbf{E}_t = \mathbf{E}_{0t} \, e^{i(\mathbf{k}_t \cdot \mathbf{r} - \omega t)}$$

## The Phase Matching Condition

The boundary conditions from Maxwell's equations (Section 1.4.4) require that the tangential electric and magnetic fields be continuous across the interface. These conditions must hold at every point $(x, y)$ on the interface and at all times $t$. This means the *phase factors* of all three waves must be equal at $z = 0$:

$$e^{i(\mathbf{k}_i \cdot \mathbf{r})} \big|_{z=0} = e^{i(\mathbf{k}_r \cdot \mathbf{r})} \big|_{z=0} = e^{i(\mathbf{k}_t \cdot \mathbf{r})} \big|_{z=0}$$

for all $(x, y)$. This requires the $x$ and $y$ components of all three wavevectors to be equal:

$$k_{ix} = k_{rx} = k_{tx}$$

$$k_{iy} = k_{ry} = k_{ty}$$

This is the phase matching condition. It is a direct consequence of the boundary conditions, which are themselves a consequence of Maxwell's equations.

## Deriving the Laws of Reflection and Snell's Law

Take the wave in the $x$-$z$ plane ($k_{iy} = 0$, so all $y$-components vanish). Then:

$$k_{ix} = k_{rx} \implies k_i \sin\theta_i = k_r \sin\theta_r$$

Since both $\mathbf{k}_i$ and $\mathbf{k}_r$ are in medium 1, $k_i = k_r = n_1\omega/c$. Therefore $\sin\theta_i = \sin\theta_r$, giving:

$$\theta_r = \theta_i \qquad \text{(Law of Reflection)}$$

For the transmitted wave:

$$k_{ix} = k_{tx} \implies k_i \sin\theta_i = k_t \sin\theta_t$$

Now $k_i = n_1\omega/c$ and $k_t = n_2\omega/c$, giving:

$$n_1 \sin\theta_i = n_2 \sin\theta_t \qquad \text{(Snell's Law)}$$

The derivation is complete. Snell's law is the requirement that phase be matched along the interface — it is not a separate empirical law but a consequence of boundary conditions, which are consequences of Maxwell's equations.

## Physical Interpretation: Phase Matching

What does "phase matching" mean physically? Consider the crests of the incident wave as they arrive at the interface. These crests arrive at different points along the interface at different times, because the wave is traveling at an angle. The spacing between crest arrival points along the interface is $\lambda_1/\sin\theta_i$, where $\lambda_1 = \lambda_0/n_1$ is the wavelength in medium 1.

The transmitted wave must have crests emerging from the interface with the same spacing. But the wavelength in medium 2 is $\lambda_2 = \lambda_0/n_2$. For the crests to have spacing $\lambda_1/\sin\theta_i$, the transmitted wave must propagate at angle $\theta_t$ where $\lambda_2/\sin\theta_t = \lambda_1/\sin\theta_i$, which gives $\sin\theta_t/\lambda_2 = \sin\theta_i/\lambda_1$, which is Snell's law.

Alternatively: the component of the wavevector parallel to the interface, $k_\parallel = (n\omega/c)\sin\theta$, must be conserved across the interface. This is the statement that the interface, being infinite and uniform in the $x$-direction, cannot change the $x$-component of the wave's momentum.

## Total Internal Reflection

When $n_1 > n_2$ (going from a denser to a rarer medium), there exists a critical angle:

$$\theta_c = \arcsin\left(\frac{n_2}{n_1}\right)$$

For $\theta_i > \theta_c$, Snell's law would require $\sin\theta_t > 1$, which has no real solution. Instead, the wave is totally internally reflected — no energy is transmitted into medium 2. The transmitted wave becomes an *evanescent wave*: it decays exponentially into medium 2 but carries no net energy flux in the $z$-direction.

For silicon ($n_1 = 3.48$) on silica ($n_2 = 1.44$):

$$\theta_c = \arcsin(1.44/3.48) = \arcsin(0.414) \approx 24.5°$$

Any ray in silicon hitting the Si/SiO₂ interface at more than 24.5° from the normal undergoes TIR. In a silicon-on-insulator waveguide, this is the primary confinement mechanism: light in the silicon core bounces off the silica cladding and stays confined. Because the critical angle is small (the index contrast is large), the acceptance angle for the waveguide is large — a silicon waveguide accepts and guides rays up to $\sim 65°$ from the waveguide axis.

## The Evanescent Field

The evanescent wave in TIR is not merely a mathematical artifact. It carries real energy along the interface (the $z$-component of the Poynting vector averages to zero; the $x$-component does not). This evanescent field is important for:

1. **Waveguide coupling**: Two waveguides placed close enough together will exchange energy through their overlapping evanescent fields. This is the basis of directional couplers (Section 2.2.4) and the MZI splitters and combiners.

2. **Grating couplers**: Periodic perturbations to the evanescent field can launch or couple light into/out of a waveguide. Grating couplers are how photonic chips are connected to optical fibers.

3. **Frustrated TIR**: If a second medium is brought within a wavelength of the TIR interface, the evanescent wave can "tunnel" through the gap. This is the optical analog of quantum tunneling and is used in some fiber-to-chip coupling schemes.

The evanescent field decays as $e^{-\gamma z}$ where:

$$\gamma = \frac{\omega}{c}\sqrt{n_1^2\sin^2\theta_i - n_2^2} = \sqrt{k_\parallel^2 - k_2^2}$$

The penetration depth $1/\gamma$ is typically on the order of $\lambda/4$ to $\lambda$ — for 1550 nm, this is hundreds of nanometers. Silicon photonic directional couplers use gaps of 100–400 nm to control coupling strength.

## The Fresnel Equations

Phase matching determines the *angles* of the reflected and transmitted waves. The *amplitudes* require solving the boundary conditions for the fields explicitly. The results — the Fresnel equations — give the reflection and transmission coefficients for $s$-polarization (electric field perpendicular to the plane of incidence) and $p$-polarization (electric field in the plane of incidence):

**s-polarization (TE)**:

$$r_s = \frac{n_1\cos\theta_i - n_2\cos\theta_t}{n_1\cos\theta_i + n_2\cos\theta_t}, \qquad t_s = \frac{2n_1\cos\theta_i}{n_1\cos\theta_i + n_2\cos\theta_t}$$

**p-polarization (TM)**:

$$r_p = \frac{n_2\cos\theta_i - n_1\cos\theta_t}{n_2\cos\theta_i + n_1\cos\theta_t}, \qquad t_p = \frac{2n_1\cos\theta_i}{n_2\cos\theta_i + n_1\cos\theta_t}$$

Note: $r$ and $t$ are amplitude ratios (of $E$-fields), not intensity ratios. The intensity reflectance and transmittance are $R = |r|^2$ and $T = (n_2\cos\theta_t)/(n_1\cos\theta_i)|t|^2$, with $R + T = 1$ for lossless media.

**Brewster's angle**: For $p$-polarization, $r_p = 0$ when $n_2\cos\theta_i = n_1\cos\theta_t$, which combined with Snell's law gives $\tan\theta_B = n_2/n_1$. At Brewster's angle, the reflected light is purely $s$-polarized. This is used in laser cavities (Brewster windows) to suppress polarization-dependent losses.

## Summary

- Snell's law is a phase-matching condition, not an independent law: it follows from requiring the boundary conditions (Maxwell's equations) to be satisfied at all points and times.
- Phase matching is the requirement that the wavevector component parallel to the interface be conserved.
- Total internal reflection occurs when $\theta_i > \theta_c = \arcsin(n_2/n_1)$; for Si/SiO₂, $\theta_c \approx 24.5°$.
- The evanescent field in TIR is the basis of waveguide coupling, grating coupling, and frustrated TIR.
- The Fresnel equations give the amplitude of reflected and transmitted fields; they polarization-dependent.

---

*References*

[1] Griffiths, D.J. (2017). *Introduction to Electrodynamics*, 4th ed. Section 9.3. Cambridge University Press. [Derives the Fresnel equations and Snell's law from Maxwell's boundary conditions.]

[2] Born, M. & Wolf, E. (1999). *Principles of Optics*, 7th ed. Chapter 1. Cambridge University Press. [Rigorous derivation including oblique incidence and anisotropic media.]
