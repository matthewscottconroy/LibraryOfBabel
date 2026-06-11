# Appendix: Mathematical Methods for Photonics

> *This appendix collects the mathematical toolkit used throughout the book. It is not a substitute for a course in electromagnetism, complex analysis, or numerical methods — but it is a self-contained reference for the key formulas, derivations, and methods that recur in photonic device analysis. Each section gives the essential results with enough derivation to make them intelligible, plus references for deeper study.*

---

## A.1 Maxwell's Equations in SI Units

### A.1.1 The Full Tensor Form

Maxwell's equations in matter, in SI units:

$$\nabla \cdot \mathbf{D} = \rho_f$$
$$\nabla \cdot \mathbf{B} = 0$$
$$\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t}$$
$$\nabla \times \mathbf{H} = \mathbf{J}_f + \frac{\partial \mathbf{D}}{\partial t}$$

**Constitutive relations** for linear, isotropic, non-magnetic media:

$$\mathbf{D} = \varepsilon_0\varepsilon_r \mathbf{E} = \varepsilon \mathbf{E}, \qquad \mathbf{B} = \mu_0\mu_r \mathbf{H} \approx \mu_0 \mathbf{H} \text{ (non-magnetic)}$$

For anisotropic media (e.g., birefringent crystals, liquid crystals), the permittivity becomes a rank-2 tensor $\boldsymbol{\varepsilon}$:

$$D_i = \sum_j \varepsilon_{ij} E_j$$

The permittivity tensor is symmetric for lossless media ($\varepsilon_{ij} = \varepsilon_{ji}$) and Hermitian for absorbing media ($\varepsilon_{ij} = \varepsilon_{ji}^*$). In the crystal's principal axes, $\boldsymbol{\varepsilon}$ is diagonal: $\varepsilon = \text{diag}(\varepsilon_x, \varepsilon_y, \varepsilon_z)$; the three principal refractive indices are $n_j = \sqrt{\varepsilon_j/\varepsilon_0}$.

**The electromagnetic energy density:**

$$u = \frac{1}{2}(\mathbf{E}\cdot\mathbf{D} + \mathbf{H}\cdot\mathbf{B}) = \frac{1}{2}\varepsilon_0 n^2 |\mathbf{E}|^2 + \frac{1}{2\mu_0}|\mathbf{B}|^2$$

**Poynting's theorem:**

$$\frac{\partial u}{\partial t} + \nabla\cdot\mathbf{S} = -\mathbf{J}\cdot\mathbf{E}$$

where $\mathbf{S} = \mathbf{E}\times\mathbf{H}$ is the Poynting vector (energy flux density, W/m²).

### A.1.2 Units and Constants

| Quantity | Symbol | SI Unit | Value |
|---|---|---|---|
| Speed of light | $c$ | m/s | $2.998\times10^8$ |
| Permittivity of free space | $\varepsilon_0$ | F/m | $8.854\times10^{-12}$ |
| Permeability of free space | $\mu_0$ | H/m | $4\pi\times10^{-7}$ |
| Planck's constant | $h$ | J·s | $6.626\times10^{-34}$ |
| Reduced Planck's constant | $\hbar$ | J·s | $1.055\times10^{-34}$ |
| Boltzmann constant | $k_B$ | J/K | $1.381\times10^{-23}$ |
| Electron charge | $e$ | C | $1.602\times10^{-19}$ |

**Photon energy and momentum:**

$$E_\gamma = \hbar\omega = hc/\lambda, \qquad p_\gamma = \hbar k = h/\lambda$$

At $\lambda = 1550$ nm: $E_\gamma = 0.8$ eV $= 1.28\times10^{-19}$ J.

---

## A.2 Wave Equation Derivation

### A.2.1 From Maxwell to Helmholtz

Starting from Maxwell's equations in a source-free, linear, isotropic medium:

$$\nabla\times\mathbf{E} = -\mu\frac{\partial\mathbf{H}}{\partial t}, \qquad \nabla\times\mathbf{H} = \varepsilon\frac{\partial\mathbf{E}}{\partial t}$$

Take the curl of the first equation:

$$\nabla\times(\nabla\times\mathbf{E}) = -\mu\frac{\partial}{\partial t}(\nabla\times\mathbf{H}) = -\mu\varepsilon\frac{\partial^2\mathbf{E}}{\partial t^2}$$

Apply the vector identity $\nabla\times(\nabla\times\mathbf{F}) = \nabla(\nabla\cdot\mathbf{F}) - \nabla^2\mathbf{F}$, and use $\nabla\cdot\mathbf{E} = 0$ (source-free, homogeneous medium):

$$\nabla^2\mathbf{E} = \mu\varepsilon\frac{\partial^2\mathbf{E}}{\partial t^2} = \frac{n^2}{c^2}\frac{\partial^2\mathbf{E}}{\partial t^2}$$

This is the **vector wave equation**. Each component satisfies a scalar wave equation.

For time-harmonic fields $\mathbf{E}(\mathbf{r},t) = \mathbf{E}(\mathbf{r})e^{-i\omega t}$, the wave equation becomes the **Helmholtz equation**:

$$\nabla^2\mathbf{E} + k^2\mathbf{E} = 0, \qquad k = \frac{n\omega}{c} = \frac{2\pi n}{\lambda}$$

### A.2.2 Wave Equation in Inhomogeneous Media

When $\varepsilon = \varepsilon(\mathbf{r})$ varies spatially:

$$\nabla\times(\nabla\times\mathbf{E}) = k_0^2\varepsilon_r(\mathbf{r})\mathbf{E}$$

Using $\nabla\times(\nabla\times\mathbf{E}) = \nabla(\nabla\cdot\mathbf{E}) - \nabla^2\mathbf{E}$ and $\nabla\cdot(\varepsilon_r\mathbf{E}) = 0 \Rightarrow \nabla\cdot\mathbf{E} = -(\nabla\ln\varepsilon_r)\cdot\mathbf{E}$:

$$\nabla^2\mathbf{E} + k_0^2\varepsilon_r\mathbf{E} + \nabla(\mathbf{E}\cdot\nabla\ln\varepsilon_r) = 0$$

The last term couples different polarization components and is responsible for TE-TM conversion at interfaces. For waveguides with small index contrast (weakly guiding approximation), this term can be neglected, recovering a scalar Helmholtz equation for each field component.

---

## A.3 Transfer Matrix Method for Layered Structures

### A.3.1 Setting Up the Problem

Consider a 1D layered structure: $N$ layers, each with thickness $d_j$ and refractive index $n_j$, sandwiched between a semi-infinite input medium ($n_0$) and substrate ($n_s$). A plane wave at angle $\theta$ to the normal.

For TE polarization ($E_y$ field), define the transfer matrix for a single layer:

$$M_j = \begin{pmatrix}\cos\phi_j & -\frac{i}{\eta_j}\sin\phi_j \\ -i\eta_j\sin\phi_j & \cos\phi_j\end{pmatrix}$$

where $\phi_j = k_0 n_j d_j \cos\theta_j$ is the phase accumulated in layer $j$, and the admittance $\eta_j = n_j\cos\theta_j/Z_0$ for TE (or $n_j/(Z_0\cos\theta_j)$ for TM), with $Z_0 = \sqrt{\mu_0/\varepsilon_0} = 377\,\Omega$.

### A.3.2 Cascading Layers

The total transfer matrix of the stack:

$$M_\text{total} = M_1 M_2 \cdots M_N = \begin{pmatrix}m_{11} & m_{12} \\ m_{21} & m_{22}\end{pmatrix}$$

The reflection and transmission amplitudes:

$$r = \frac{m_{11}\eta_0 + m_{12}\eta_0\eta_s - m_{21} - m_{22}\eta_s}{m_{11}\eta_0 + m_{12}\eta_0\eta_s + m_{21} + m_{22}\eta_s}$$

$$t = \frac{2\eta_0}{m_{11}\eta_0 + m_{12}\eta_0\eta_s + m_{21} + m_{22}\eta_s}$$

Power reflectance and transmittance: $R = |r|^2$, $T = (\eta_s/\eta_0)|t|^2$.

### A.3.3 Anti-Reflection Coating Design

A single quarter-wave layer ($d = \lambda/4n_j$, $\phi = \pi/2$) of index $n_j = \sqrt{n_0 n_s}$ eliminates reflection ($R = 0$) at the design wavelength. The transfer matrix:

$$M_\text{QW} = \begin{pmatrix}0 & -i/\eta_j \\ -i\eta_j & 0\end{pmatrix}$$

Substituting: $r = (\eta_0 n_s - \eta_j^2)/({\eta_0 n_s + \eta_j^2})$. With $\eta_j = n_j = \sqrt{n_0 n_s}$: $r = 0$.

### A.3.4 Distributed Bragg Reflector (DBR)

A DBR consists of alternating quarter-wave layers of two materials with indices $n_H > n_L$. The reflectance after $N$ pairs:

$$R = \left[\frac{1 - (n_L/n_H)^{2N} n_s/n_0}{1 + (n_L/n_H)^{2N} n_s/n_0}\right]^2$$

For $(n_L/n_H)^{2N} \to 0$ (large $N$ or large index contrast): $R\to 1$. Practical DBRs achieve $R > 99.9\%$ with $N = 20$–30 pairs.

---

## A.4 Finite-Difference Time-Domain (FDTD) Method

### A.4.1 The Yee Grid

FDTD (Yee, 1966) discretizes Maxwell's equations directly in time and space. The fields are staggered on a Cartesian grid with spatial step $\Delta x, \Delta y, \Delta z$ and time step $\Delta t$:

- $E_x$ is located at $(i+1/2, j, k)$ time $(n)$
- $H_x$ is located at $(i, j+1/2, k+1/2)$ time $(n+1/2)$

And so on for the other components. The curl equations become finite differences:

$$\frac{H_x^{n+1/2}(i,j+\tfrac{1}{2},k+\tfrac{1}{2}) - H_x^{n-1/2}(i,j+\tfrac{1}{2},k+\tfrac{1}{2})}{\Delta t} = \frac{1}{\mu}\left[\frac{E_y^n(i,j+\tfrac{1}{2},k+1)-E_y^n(i,j+\tfrac{1}{2},k)}{\Delta z} - \frac{E_z^n(i,j+1,k+\tfrac{1}{2})-E_z^n(i,j,k+\tfrac{1}{2})}{\Delta y}\right]$$

### A.4.2 Stability: The Courant Condition

FDTD is conditionally stable. The Courant-Friedrichs-Lewy (CFL) condition requires:

$$\Delta t \leq \frac{1}{c\sqrt{\frac{1}{(\Delta x)^2}+\frac{1}{(\Delta y)^2}+\frac{1}{(\Delta z)^2}}}$$

In 3D with cubic cells ($\Delta x = \Delta y = \Delta z = \Delta$): $\Delta t \leq \Delta/(c\sqrt{3})$.

### A.4.3 Absorbing Boundaries: Perfectly Matched Layer (PML)

To simulate open (infinite) boundaries, the PML (Berenger, 1994) surrounds the computational domain. The PML is a fictitious absorbing medium constructed so that a plane wave of any angle or frequency passes through the PML-domain interface without reflection, then is absorbed.

The PML complex coordinate stretching: replace $\partial_x \to \frac{1}{s_x}\partial_x$ where $s_x = 1 + \sigma_x(x)/i\omega\varepsilon_0$.

### A.4.4 Computing Device Properties with FDTD

To find the transmission spectrum of a device:
1. Launch a broadband pulse (Gaussian modulated by $e^{-i\omega_0 t}$)
2. Record the field at an output monitor
3. Fourier transform: $\tilde{E}(\omega) = \int E(t) e^{i\omega t} dt$
4. Compute $T(\omega) = |\tilde{E}_\text{out}(\omega)/\tilde{E}_\text{in}(\omega)|^2$

---

## A.5 S-Parameters and Scattering Matrix Formalism

### A.5.1 Definition

The scattering matrix $S$ relates the outgoing wave amplitudes to the incoming wave amplitudes at the ports of a device:

$$\mathbf{b} = S \mathbf{a}$$

where $a_j = $ (incoming amplitude at port $j$), $b_j = $ (outgoing amplitude at port $j$), normalized so that $|a_j|^2$ = power incident on port $j$.

For a 2-port device:

$$\begin{pmatrix}b_1\\b_2\end{pmatrix} = \begin{pmatrix}S_{11} & S_{12} \\ S_{21} & S_{22}\end{pmatrix}\begin{pmatrix}a_1\\a_2\end{pmatrix}$$

$S_{11}$ = reflection at port 1, $S_{21}$ = transmission from port 1 to port 2, etc.

### A.5.2 Properties of the S-Matrix

- **Reciprocity:** For a reciprocal device (no magnetic materials, no active elements): $S = S^T$.
- **Losslessness:** For a lossless device: $S^\dagger S = I$ (unitary), so $|S_{11}|^2 + |S_{21}|^2 = 1$ (power conservation at port 1).
- **Symmetry with loss:** For a lossy device: $S^\dagger S \leq I$ (all eigenvalues of $S^\dagger S \leq 1$).

### A.5.3 S-Matrix of Common Photonic Elements

**50/50 beamsplitter:**

$$S = \frac{1}{\sqrt{2}}\begin{pmatrix}0 & 1 & i & 0\\ 1 & 0 & 0 & i\\ i & 0 & 0 & 1\\ 0 & i & 1 & 0\end{pmatrix}$$

(4-port device with two inputs and two outputs; rows/columns ordered as ports 1,2,3,4)

**Ring resonator (single bus):** For coupling coefficient $\kappa$ and round-trip amplitude $a = \sqrt{1-\alpha}$ (loss $\alpha$ per round trip) and phase $\phi = 2\pi n_\text{eff} L/\lambda$:

$$S_{21}(\omega) = \frac{a e^{i\phi} - \kappa^2/(1-\kappa^2)}{1 - a e^{i\phi}\kappa^2/(1-\kappa^2)^{1/2}} \cdot \frac{-\sqrt{1-\kappa^2}}{1}$$

The exact transfer function for the all-pass ring resonator (Yariv, 2000):

$$T = \left|\frac{a - te^{i\phi}}{1 - ate^{i\phi}}\right|^2$$

where $t = \sqrt{1-\kappa^2}$ is the self-coupling coefficient. At resonance ($\phi = 2\pi m$): critical coupling ($t = a$) gives $T = 0$.

---

## A.6 Fourier Optics: Angular Spectrum Representation

### A.6.1 The Angular Spectrum

Any monochromatic optical field at the plane $z = 0$ can be written as a superposition of plane waves propagating at different angles:

$$E(x, y, 0) = \iint \tilde{E}(k_x, k_y) e^{i(k_x x + k_y y)} \frac{dk_x dk_y}{(2\pi)^2}$$

where the angular spectrum $\tilde{E}(k_x, k_y)$ is the 2D Fourier transform of the field:

$$\tilde{E}(k_x, k_y) = \iint E(x,y,0) e^{-i(k_x x + k_y y)} dx\, dy$$

### A.6.2 Propagation in the Angular Spectrum

In free space with $k = n\omega/c$, the $z$-component of the wavevector is $k_z = \sqrt{k^2 - k_x^2 - k_y^2}$. The field at plane $z$ is:

$$E(x, y, z) = \iint \tilde{E}(k_x, k_y) e^{i(k_x x + k_y y + k_z z)} \frac{dk_x dk_y}{(2\pi)^2}$$

This is exact (within the paraxial approximation $k_{x,y} \ll k$, one expands $k_z \approx k - (k_x^2 + k_y^2)/(2k)$ to obtain the paraxial wave equation).

### A.6.3 Fraunhofer Diffraction

In the far field ($z \gg k(x^2+y^2)/2$), the Fraunhofer approximation gives:

$$E(x, y, z) \approx \frac{e^{ikz}}{i\lambda z} \iint E(x',y',0) \exp\!\left(-\frac{2\pi i}{\lambda z}(xx' + yy')\right) dx' dy'$$

The far-field pattern is the Fourier transform of the aperture field. This is why a circular aperture of radius $a$ produces an Airy disk pattern with first zero at $\theta = 1.22\lambda/(2a)$.

### A.6.4 The 4-f Imaging System

A 4-f system (two lenses separated by $2f$, with the object and image each at distance $f$ outside) performs an exact Fourier transform and its inverse. The field in the Fourier plane (between the two lenses) is:

$$E_F(x, y) \propto \tilde{E}\!\left(\frac{2\pi x}{\lambda f}, \frac{2\pi y}{\lambda f}, 0\right)$$

Inserting a spatial filter (mask, spatial light modulator) in the Fourier plane implements a linear spatial filter $H(k_x, k_y)$. This is the basis of optical correlators and Fourier-domain signal processing.

---

## A.7 Key Integrals and Special Functions

### A.7.1 Gaussian Integrals

The fundamental Gaussian integral:

$$\int_{-\infty}^{\infty} e^{-ax^2} dx = \sqrt{\frac{\pi}{a}}, \qquad \text{Re}(a) > 0$$

Generalizations:

$$\int_{-\infty}^{\infty} e^{-ax^2 + bx} dx = \sqrt{\frac{\pi}{a}}\, e^{b^2/4a}$$

$$\int_{-\infty}^{\infty} x^{2n} e^{-ax^2} dx = \frac{(2n-1)!!}{2^n a^n}\sqrt{\frac{\pi}{a}}$$

In 2D:

$$\iint e^{-(ax^2 + bxy + cy^2)} dx\, dy = \frac{2\pi}{\sqrt{4ac - b^2}}$$

### A.7.2 The Gaussian Beam

The Gaussian beam is the solution to the paraxial wave equation with beam waist $w_0$ at $z=0$:

$$E(r,z) = E_0 \frac{w_0}{w(z)} \exp\!\left(-\frac{r^2}{w(z)^2}\right) \exp\!\left(-i\left[kz - \arctan\!\frac{z}{z_R} + \frac{kr^2}{2R(z)}\right]\right)$$

where:

$$w(z) = w_0\sqrt{1 + (z/z_R)^2}, \qquad R(z) = z\left[1+(z_R/z)^2\right], \qquad z_R = \frac{\pi w_0^2}{\lambda}$$

$z_R$ is the Rayleigh range (the distance over which the beam area doubles), $w(z)$ is the beam radius, and $R(z)$ is the wavefront radius of curvature.

### A.7.3 Bessel Functions

Bessel functions of the first kind $J_m(x)$ satisfy:

$$x^2 J_m'' + x J_m' + (x^2 - m^2)J_m = 0$$

and arise in circular waveguides (optical fiber). The modified Bessel function $K_m(x)$ (decaying exponential for large $x$) describes the evanescent field in the cladding.

For an optical fiber with core radius $a$, core index $n_1$, cladding index $n_2$, the guided modes satisfy the eigenvalue equation:

$$\left[\frac{J_m'(ua)}{ua\, J_m(ua)} + \frac{K_m'(wa)}{wa\, K_m(wa)}\right]\left[\frac{n_1^2 J_m'(ua)}{ua\, J_m(ua)} + \frac{n_2^2 K_m'(wa)}{wa\, K_m(wa)}\right] = \frac{m^2\beta^2}{k_0^2}\left[\frac{1}{(ua)^2} + \frac{1}{(wa)^2}\right]^2$$

where $u^2 = n_1^2 k_0^2 - \beta^2$ and $w^2 = \beta^2 - n_2^2 k_0^2$.

### A.7.4 The Fourier Transform and Its Properties

$$\mathcal{F}[f(t)](\omega) = \tilde{f}(\omega) = \int_{-\infty}^{\infty} f(t) e^{i\omega t} dt$$

$$\mathcal{F}^{-1}[\tilde{f}(\omega)](t) = f(t) = \frac{1}{2\pi}\int_{-\infty}^{\infty} \tilde{f}(\omega) e^{-i\omega t} d\omega$$

Key properties (with $\mathcal{F}[f] = \tilde{f}$):

| Property | Time domain | Frequency domain |
|---|---|---|
| Linearity | $af + bg$ | $a\tilde{f} + b\tilde{g}$ |
| Shift | $f(t-t_0)$ | $e^{i\omega t_0}\tilde{f}(\omega)$ |
| Modulation | $e^{i\omega_0 t}f(t)$ | $\tilde{f}(\omega - \omega_0)$ |
| Derivative | $f'(t)$ | $-i\omega\tilde{f}(\omega)$ |
| Convolution | $(f*g)(t)$ | $\tilde{f}(\omega)\tilde{g}(\omega)$ |
| Parseval | $\int|f|^2 dt$ | $\frac{1}{2\pi}\int|\tilde{f}|^2 d\omega$ |

Transform pairs of photonic relevance:

$$\text{rect}(t/T) \longleftrightarrow T\,\text{sinc}(\omega T/2\pi)$$
$$e^{-t^2/2\tau^2} \longleftrightarrow \sqrt{2\pi}\tau\, e^{-\omega^2\tau^2/2}$$
$$\delta(t-t_0) \longleftrightarrow e^{i\omega t_0}$$

**Time-bandwidth product:** For any pulse, $\Delta t \cdot \Delta\omega \geq 1/2$ (Gaussian achieves equality). This sets the fundamental limit on how short a pulse can be for a given spectral bandwidth.

---

## A.8 Vector Calculus Reference

### A.8.1 Identities

**Curl of a curl:**
$$\nabla\times(\nabla\times\mathbf{F}) = \nabla(\nabla\cdot\mathbf{F}) - \nabla^2\mathbf{F}$$

**Divergence of a curl:**
$$\nabla\cdot(\nabla\times\mathbf{F}) = 0$$

**Curl of a gradient:**
$$\nabla\times(\nabla f) = 0$$

**Product rules:**
$$\nabla\cdot(f\mathbf{F}) = \mathbf{F}\cdot\nabla f + f\nabla\cdot\mathbf{F}$$
$$\nabla\times(f\mathbf{F}) = f\nabla\times\mathbf{F} + (\nabla f)\times\mathbf{F}$$
$$\nabla\cdot(\mathbf{A}\times\mathbf{B}) = \mathbf{B}\cdot(\nabla\times\mathbf{A}) - \mathbf{A}\cdot(\nabla\times\mathbf{B})$$

### A.8.2 Coordinate Expressions

**Cylindrical coordinates** $(r,\phi,z)$:

$$\nabla f = \frac{\partial f}{\partial r}\hat{r} + \frac{1}{r}\frac{\partial f}{\partial\phi}\hat{\phi} + \frac{\partial f}{\partial z}\hat{z}$$

$$\nabla^2 f = \frac{1}{r}\frac{\partial}{\partial r}\left(r\frac{\partial f}{\partial r}\right) + \frac{1}{r^2}\frac{\partial^2 f}{\partial\phi^2} + \frac{\partial^2 f}{\partial z^2}$$

$$\nabla\times\mathbf{F} = \left(\frac{1}{r}\frac{\partial F_z}{\partial\phi} - \frac{\partial F_\phi}{\partial z}\right)\hat{r} + \left(\frac{\partial F_r}{\partial z} - \frac{\partial F_z}{\partial r}\right)\hat{\phi} + \frac{1}{r}\left(\frac{\partial(rF_\phi)}{\partial r} - \frac{\partial F_r}{\partial\phi}\right)\hat{z}$$

---

## A.9 Numerical Methods Summary

### A.9.1 Method Comparison

| Method | Geometry | Domains | Accuracy | Speed | Best for |
|---|---|---|---|---|---|
| FDTD | Arbitrary | Time-domain | 2nd order | Medium | Broadband, transient |
| FEM (COMSOL) | Arbitrary | Freq-domain | High | Slow | Resonators, eigenmode |
| EME (eigenmode expansion) | Waveguide-like | Freq-domain | High | Fast | Propagation, S-params |
| RCWA | Periodic | Freq-domain | High | Fast | Gratings, photonic crystals |
| BPM (beam propagation) | Slowly varying | Freq-domain | Low | Very fast | Long waveguide sections |

### A.9.2 Convergence and Mesh Resolution

For FDTD: $\Delta x \leq \lambda/(10 n_\text{max})$ in the medium with highest index. The simulation time must extend until the field has decayed to $< -60$ dB of peak to avoid Fourier artifacts.

For FEM: refinement near curved surfaces and material interfaces. Adaptive mesh refinement converges to the exact solution.

---

## A.10 Physical Constants Summary

$$c = 2.998\times10^8 \text{ m/s}, \quad \varepsilon_0 = 8.854\times10^{-12}\text{ F/m}, \quad \mu_0 = 1.257\times10^{-6}\text{ H/m}$$

$$Z_0 = \sqrt{\mu_0/\varepsilon_0} = 376.7\,\Omega \quad \text{(impedance of free space)}$$

$$\hbar = 1.055\times10^{-34}\text{ J·s}, \quad k_B = 1.381\times10^{-23}\text{ J/K}, \quad e = 1.602\times10^{-19}\text{ C}$$

**Photonic computing material parameters at 1550 nm:**

| Material | $n$ | $dn/dT$ (K⁻¹) | Loss (dB/cm) | Notes |
|---|---|---|---|---|
| Si | 3.47 | $1.84\times10^{-4}$ | 0.5–3 | CMOS, no Pockels |
| SiO₂ | 1.44 | $1.0\times10^{-5}$ | $<0.001$ | Cladding, fiber |
| SiN | 2.00 | $2.5\times10^{-5}$ | 0.01–0.1 | Low loss, no TPA |
| LiNbO₃ (bulk) | 2.14 (n_e) | $4\times10^{-6}$ | 0.1 | Pockels $r_{33}=30.8$ pm/V |
| GaAs | 3.37 | $2.0\times10^{-4}$ | $<1$ | EO, active gain |
| InP | 3.17 | $2.3\times10^{-4}$ | $<1$ | Active, telecom |
