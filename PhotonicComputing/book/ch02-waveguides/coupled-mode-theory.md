# Coupled-Mode Theory for Waveguide Coupling

> *Two waveguides brought into proximity exchange light like two coupled pendulums exchange energy. The mathematics underlying this exchange — coupled-mode theory — is one of the most powerful analytical tools in photonics. It reduces Maxwell's equations, which are partial differential equations in space and time, to a pair of ordinary differential equations in a single propagation coordinate. From this reduction flows an entire design language for filters, switches, add-drop multiplexers, and optical logic gates.*

---

## 1. Why Coupled-Mode Theory?

Maxwell's equations are exact, but they are hard. For a single uniform waveguide the problem is tractable: we find the modes by solving a Helmholtz equation in the transverse plane and obtain discrete eigenvalues (propagation constants) and eigenfunctions (mode profiles). The moment we perturb the waveguide — by bringing a second waveguide nearby, by introducing a periodic corrugation, by changing the refractive index locally — the clean eigenmode picture breaks down.

Coupled-mode theory (CMT) restores analytical tractability by treating the perturbed system as a perturbation of the unperturbed system. The key approximation: assume the fields of the perturbed structure can be expanded in the modes of the unperturbed structure. Energy then flows slowly and continuously among modes, with coupling coefficients determined by overlap integrals. The result is a set of amplitude equations, first-order ODEs, that are exactly solvable in many geometries.

CMT comes in two flavors:

1. **Temporal CMT** — for resonant systems (ring resonators, photonic crystal cavities). The dynamical variable is the cavity mode amplitude $a(t)$, and coupling is to external waveguide channels and to loss. This is the framework of Haus and colleagues.

2. **Spatial CMT** — for propagating systems (directional couplers, fiber couplers). The dynamical variable is the mode amplitude $a(z)$ as a function of propagation distance. This is the framework we derive in detail here.

Both frameworks are manifestations of the same underlying physics; they are connected by the substitution $\partial_t \leftrightarrow -i\beta\partial_z$ in the slowly-varying-envelope approximation.

---

## 2. Maxwell's Equations in the Guided-Wave Approximation

### 2.1 Setup and Notation

Consider a dielectric waveguide system. The permittivity is $\varepsilon(\mathbf{r}) = \varepsilon_0 n^2(\mathbf{r})$. Maxwell's equations in the absence of free sources are:

$$\nabla \times \mathbf{E} = -\mu_0 \frac{\partial \mathbf{H}}{\partial t}, \qquad \nabla \times \mathbf{H} = \varepsilon(\mathbf{r})\frac{\partial \mathbf{E}}{\partial t}$$

For fields at a single angular frequency $\omega$ (time dependence $e^{-i\omega t}$):

$$\nabla \times \mathbf{E} = i\omega\mu_0 \mathbf{H}, \qquad \nabla \times \mathbf{H} = -i\omega\varepsilon(\mathbf{r})\mathbf{E}$$

Eliminating $\mathbf{H}$:

$$\nabla \times \left[\frac{1}{\varepsilon(\mathbf{r})}\nabla \times \mathbf{H}\right] = \frac{\omega^2}{c^2}\mathbf{H}$$

or equivalently for the electric field:

$$\nabla \times \nabla \times \mathbf{E} = \omega^2\mu_0\varepsilon(\mathbf{r})\mathbf{E}$$

### 2.2 Mode Expansion for a Single Waveguide

A waveguide with permittivity profile $\varepsilon_0(\mathbf{r}_\perp)$ (independent of $z$) supports guided modes. Mode $m$ has the form:

$$\mathbf{E}_m(\mathbf{r},t) = \mathbf{e}_m(\mathbf{r}_\perp) e^{i(\beta_m z - \omega t)} + \text{c.c.}$$
$$\mathbf{H}_m(\mathbf{r},t) = \mathbf{h}_m(\mathbf{r}_\perp) e^{i(\beta_m z - \omega t)} + \text{c.c.}$$

where $\mathbf{r}_\perp = (x,y)$ is the transverse coordinate, $\beta_m$ is the propagation constant (eigenvalue), and $\mathbf{e}_m, \mathbf{h}_m$ are the transverse mode profiles (eigenfunctions).

The modes satisfy an orthogonality relation derived from the Lorentz reciprocity theorem. For modes $m$ and $n$ of the same waveguide:

$$\int_{-\infty}^{\infty} (\mathbf{e}_m \times \mathbf{h}_n^* + \mathbf{e}_n^* \times \mathbf{h}_m) \cdot \hat{z}\, d\mathbf{r}_\perp = 4 P_m \delta_{mn}$$

where $P_m$ is the power carried by mode $m$. We normalize modes so that $P_m = 1/2$ (each mode carries unit total power when the amplitude is 1).

**Normalization convention:** Define normalized mode functions $\hat{\mathbf{e}}_m, \hat{\mathbf{h}}_m$ such that:

$$\frac{1}{2}\text{Re}\int (\hat{\mathbf{e}}_m \times \hat{\mathbf{h}}_m^*) \cdot \hat{z}\, d\mathbf{r}_\perp = 1$$

A mode carrying power $P$ has amplitude $a_m = \sqrt{P}$, so the total field amplitude is $a_m \hat{\mathbf{e}}_m$.

---

## 3. The Perturbed System and Mode Coupling

### 3.1 The Perturbation

Now consider a system with permittivity $\varepsilon(\mathbf{r}) = \varepsilon_0(\mathbf{r}_\perp) + \Delta\varepsilon(\mathbf{r})$, where $\Delta\varepsilon$ is a perturbation. In a directional coupler, $\Delta\varepsilon$ represents the second waveguide: the permittivity of waveguide 2 is a perturbation when viewed from waveguide 1.

Maxwell's equations in the perturbed system:

$$\nabla \times \nabla \times \mathbf{E} = \omega^2\mu_0[\varepsilon_0(\mathbf{r}_\perp) + \Delta\varepsilon(\mathbf{r})]\mathbf{E}$$

### 3.2 Field Expansion

Write the total electric field as a superposition of the unperturbed modes with slowly-varying amplitudes:

$$\mathbf{E}(\mathbf{r},t) = \sum_m a_m(z) \hat{\mathbf{e}}_m(\mathbf{r}_\perp) e^{i(\beta_m z - \omega t)} + \text{c.c.}$$

The amplitudes $a_m(z)$ vary with $z$ because the perturbation redistributes power among modes. The slowly-varying-envelope approximation (SVEA) assumes:

$$\left|\frac{d^2 a_m}{dz^2}\right| \ll \beta_m \left|\frac{da_m}{dz}\right|$$

i.e., the amplitude changes little over a wavelength.

### 3.3 Substitution into Maxwell's Equations

Substituting the mode expansion into Maxwell's equations and using the fact that each unperturbed mode satisfies $\nabla \times \nabla \times \hat{\mathbf{e}}_m = \omega^2\mu_0\varepsilon_0\hat{\mathbf{e}}_m$, one obtains (after algebra involving the SVEA to drop second derivatives of amplitudes):

$$\sum_m \left(2i\beta_m \frac{da_m}{dz}\right)\hat{\mathbf{e}}_m e^{i\beta_m z} = -\omega^2\mu_0 \Delta\varepsilon \sum_m a_m \hat{\mathbf{e}}_m e^{i\beta_m z}$$

Multiply both sides by $\hat{\mathbf{e}}_n^*$ and integrate over the transverse cross-section, using the orthogonality relation:

$$\frac{da_n}{dz} = \frac{i\omega^2\mu_0}{2\beta_n} \sum_m a_m e^{i(\beta_m - \beta_n)z} \int \Delta\varepsilon(\mathbf{r}_\perp) \hat{\mathbf{e}}_n^* \cdot \hat{\mathbf{e}}_m \, d\mathbf{r}_\perp$$

### 3.4 The Coupling Coefficient

Define the **cross-coupling coefficient** between modes $n$ and $m$:

$$\kappa_{nm} = \frac{\omega}{4} \int \Delta\varepsilon(\mathbf{r}_\perp) \hat{\mathbf{e}}_n^* \cdot \hat{\mathbf{e}}_m \, d\mathbf{r}_\perp$$

where we have absorbed a factor to obtain clean units ($\kappa$ has dimensions of inverse length).

The evolution equation becomes:

$$\frac{da_n}{dz} = i \sum_m \kappa_{nm} a_m e^{i(\beta_m - \beta_n)z}$$

This is the fundamental coupled-mode equation.

---

## 4. Two-Mode Coupled-Mode Theory

### 4.1 The Standard Form

For a system with two modes $a$ (forward in waveguide 1) and $b$ (forward in waveguide 2), retaining only the coupling between them and defining phase-matched amplitudes $A = a e^{-i\beta_a z}$, $B = b e^{-i\beta_b z}$:

$$\frac{dA}{dz} = i\kappa_{ab} B \, e^{i(\beta_b - \beta_a)z} \cdot e^{-i(\beta_b - \beta_a)z}$$

After absorbing the phase factors into the definition of slowly-varying amplitudes $\tilde{A}(z) = a_1(z)$, $\tilde{B}(z) = a_2(z) e^{i\Delta\beta z}$ where $\Delta\beta = \beta_1 - \beta_2$:

$$\boxed{\frac{d\tilde{A}}{dz} = i\kappa \tilde{B} e^{-i\Delta\beta z}}$$
$$\boxed{\frac{d\tilde{B}}{dz} = i\kappa^* \tilde{A} e^{+i\Delta\beta z}}$$

where $\kappa \equiv \kappa_{12}$ is the coupling coefficient (units: rad/m), and we have used $\kappa_{21} = \kappa_{12}^*$ which follows from the reality of $\Delta\varepsilon$.

Alternatively, working in a rotating frame where $A(z) = \tilde{A}e^{+i\Delta\beta z/2}$ and $B(z) = \tilde{B}e^{-i\Delta\beta z/2}$:

$$\frac{dA}{dz} = i\frac{\Delta\beta}{2} A + i\kappa B$$
$$\frac{dB}{dz} = i\kappa^* A - i\frac{\Delta\beta}{2} B$$

This is the canonical form of spatial CMT. In matrix notation:

$$\frac{d}{dz}\begin{pmatrix}A\\B\end{pmatrix} = i\begin{pmatrix}\delta & \kappa \\ \kappa^* & -\delta\end{pmatrix}\begin{pmatrix}A\\B\end{pmatrix}$$

where $\delta = \Delta\beta/2$ is the half-detuning.

**Connection to temporal CMT:** Replace $z$ with $t$ and $\delta$ with $(\omega_a - \omega_b)/2$. The temporal CMT equations for two coupled resonators are:

$$\dot{a} = -i\omega_a a + i\kappa b, \qquad \dot{b} = i\kappa^* a - i\omega_b b$$

The spatial and temporal forms are mathematically identical; only the physical interpretation changes.

### 4.2 Energy Conservation

For lossless coupling, the total power $|A|^2 + |B|^2$ is conserved. Verify:

$$\frac{d}{dz}(|A|^2 + |B|^2) = A^*\frac{dA}{dz} + A\frac{dA^*}{dz} + B^*\frac{dB}{dz} + B\frac{dB^*}{dz}$$

Substituting the CMT equations and using $\kappa^* = \kappa$ (real coupling for this geometry):

$$= i\delta|A|^2 + i\kappa A^*B - i\delta|A|^2 - i\kappa^* AB^* + i\kappa^* AB^* - i\delta|B|^2 - i\delta|A|^2 + i\kappa A^*B - \ldots$$

After careful bookkeeping: all terms cancel, and $\frac{d}{dz}(|A|^2 + |B|^2) = 0$. Power is conserved.

---

## 5. Exact Solution: The Directional Coupler

### 5.1 Phase-Matched Case ($\Delta\beta = 0$)

The simplest and most important case: two identical waveguides, $\beta_1 = \beta_2 = \beta$, so $\delta = 0$. The CMT system reduces to:

$$\frac{dA}{dz} = i\kappa B, \qquad \frac{dB}{dz} = i\kappa A$$

This is a system of linear ODEs with constant coefficients. The general solution:

$$A(z) = A(0)\cos(\kappa z) + iB(0)\sin(\kappa z)$$
$$B(z) = iA(0)\sin(\kappa z) + B(0)\cos(\kappa z)$$

With initial condition $A(0) = 1$, $B(0) = 0$ (all power in waveguide 1):

$$A(z) = \cos(\kappa z), \qquad B(z) = i\sin(\kappa z)$$

**Power evolution:**

$$|A(z)|^2 = \cos^2(\kappa z), \qquad |B(z)|^2 = \sin^2(\kappa z)$$

The power oscillates sinusoidally between the two waveguides with spatial period $L_c = \pi/(2\kappa)$, the **coupling length** or **transfer length**.

At $z = L_c = \pi/(2\kappa)$: all power has transferred to waveguide 2. At $z = 2L_c$: power is back in waveguide 1. The system is a periodic pump-dump oscillator in space.

**Splitting ratio:** A coupler of length $L$ has power splitting ratio $\eta = \sin^2(\kappa L)$. For a 50/50 (3 dB) coupler: $\kappa L = \pi/4$, i.e., $L = \pi/(4\kappa)$.

### 5.2 General Solution for Arbitrary $\Delta\beta$

For $\delta \neq 0$, the CMT matrix has eigenvalues $\pm\Omega$ where:

$$\Omega = \sqrt{\kappa^2 + \delta^2}$$

The general solution (with $A(0) = 1$, $B(0) = 0$):

$$A(z) = \left[\cos(\Omega z) - i\frac{\delta}{\Omega}\sin(\Omega z)\right]e^{i\delta z}$$

$$B(z) = i\frac{\kappa}{\Omega}\sin(\Omega z) e^{-i\delta z}$$

The power transfer fraction is:

$$\eta = |B(z)|^2 = \frac{\kappa^2}{\Omega^2}\sin^2(\Omega z) = \frac{\kappa^2}{\kappa^2 + \delta^2}\sin^2\!\left(\sqrt{\kappa^2 + \delta^2}\, z\right)$$

**Key result:** Phase mismatch reduces the maximum power transfer fraction from 1 to $\kappa^2/(\kappa^2 + \delta^2)$. For $|\delta| \gg |\kappa|$, the power transfer is negligible — hence the requirement for phase matching in efficient couplers.

### 5.3 Transfer Matrix Representation

The directional coupler is described by a $2\times 2$ transfer matrix $M$:

$$\begin{pmatrix}A(L)\\B(L)\end{pmatrix} = M \begin{pmatrix}A(0)\\B(0)\end{pmatrix}$$

For the phase-matched case:

$$M = \begin{pmatrix}\cos(\kappa L) & i\sin(\kappa L) \\ i\sin(\kappa L) & \cos(\kappa L)\end{pmatrix}$$

Note: $\det M = 1$ and $M^\dagger M = I$; this is a unitary matrix, consistent with power conservation. This unitary structure is the foundation of optical matrix-vector multiplication (see Chapter 6).

The scattering matrix (S-matrix) relates output to input amplitudes including phase shifts from propagation:

$$S = e^{i\beta L}\begin{pmatrix}\cos(\kappa L) & i\sin(\kappa L) \\ i\sin(\kappa L) & \cos(\kappa L)\end{pmatrix}$$

---

## 6. Coupling Coefficients: Physical Intuition and Computation

### 6.1 Overlap Integral

The coupling coefficient is:

$$\kappa = \frac{\omega\varepsilon_0}{4}\int_{\text{guide 2}} (n_2^2 - n_\text{clad}^2) \hat{\mathbf{e}}_1^* \cdot \hat{\mathbf{e}}_2 \, d\mathbf{r}_\perp$$

The integral is taken over the cross-section of waveguide 2, where $\Delta\varepsilon = \varepsilon_0(n_2^2 - n_\text{clad}^2) \neq 0$. The overlap of mode 1's field with the index contrast of waveguide 2 determines how strongly mode 1 is scattered into mode 2.

**Physical picture:** The tail of mode 1 extends into the region occupied by waveguide 2. The index of waveguide 2 acts as a source current that radiates into mode 2. The coupling coefficient measures how much of mode 1's field overlaps with waveguide 2.

### 6.2 Exponential Decay of $\kappa$ with Gap

Mode fields in the cladding decay evanescently:

$$\hat{e}_1(x) \sim A e^{-\gamma_1 x}, \quad x > d/2$$

where $\gamma_1 = \sqrt{\beta_1^2 - k_0^2 n_\text{clad}^2}$ is the cladding decay constant. If the gap between waveguides is $g$:

$$\kappa \propto \int_{d/2}^{d/2+g} e^{-2\gamma_1 (x - d/2)} dx \propto e^{-\gamma_1 g}$$

**Result:** The coupling coefficient decays exponentially with gap spacing. Halving the gap can increase $\kappa$ by an order of magnitude.

Typical values (silicon photonics, 1550 nm, 450 nm wide waveguides):
- Gap 200 nm: $\kappa \approx 1000$ rad/mm, $L_c \approx 1.6$ mm
- Gap 400 nm: $\kappa \approx 100$ rad/mm, $L_c \approx 16$ mm
- Gap 100 nm: $\kappa \approx 5000$ rad/mm, $L_c \approx 0.3$ mm (but fabrication is difficult)

### 6.3 Wavelength Dependence

The coupling coefficient inherits frequency dependence from both the mode profile (which changes with wavelength) and the evanescent decay constant. Near the phase-matching condition:

$$\frac{d\kappa}{d\omega} \approx \frac{\kappa}{\omega}\left(1 + \frac{\gamma_1}{\kappa}\frac{d\gamma_1}{d\omega}\cdot\omega\right)$$

This frequency dependence creates wavelength-selective coupling — the basis for wavelength filters.

---

## 7. Resonant Coupling: Coupled Resonators

### 7.1 Temporal CMT for Two Resonators

Two optical resonators (e.g., microring resonators) with resonant frequencies $\omega_1$, $\omega_2$ and coupling rate $\mu$ obey temporal CMT:

$$\frac{da_1}{dt} = -i\omega_1 a_1 - \frac{\gamma_1}{2}a_1 + i\mu a_2 + \sqrt{\gamma_{e1}} s_+$$
$$\frac{da_2}{dt} = -i\omega_2 a_2 - \frac{\gamma_2}{2}a_2 + i\mu^* a_1$$

where $\gamma_1, \gamma_2$ are total decay rates (intrinsic loss + external coupling), $\gamma_{e1}$ is the coupling rate of resonator 1 to the input waveguide, and $s_+$ is the input field amplitude (power normalized: $|s_+|^2$ = input power).

### 7.2 Normal Modes of Coupled Resonators

In the absence of loss and driving, the eigenfrequencies are:

$$\omega_\pm = \frac{\omega_1 + \omega_2}{2} \pm \sqrt{\left(\frac{\omega_1 - \omega_2}{2}\right)^2 + |\mu|^2}$$

For identical resonators ($\omega_1 = \omega_2 = \omega_0$):

$$\omega_\pm = \omega_0 \pm |\mu|$$

The coupling splits the degenerate resonance into two normal modes separated by $2|\mu|$. This is the **avoided crossing** or **normal mode splitting** — a hallmark of strong coupling between resonators.

The corresponding mode profiles (superposition states):

$$|+\rangle = \frac{1}{\sqrt{2}}(|1\rangle + |2\rangle), \qquad |-\rangle = \frac{1}{\sqrt{2}}(|1\rangle - |2\rangle)$$

The symmetric mode $|+\rangle$ has energy concentrated constructively in both resonators; the antisymmetric mode $|-\rangle$ has a node between them.

### 7.3 Transfer Function

The steady-state response to a monochromatic input $s_+ e^{-i\omega t}$: set $a_j(t) = \tilde{a}_j e^{-i\omega t}$ and solve the algebraic system. The transmission through the coupled resonator system:

$$T(\omega) = \left|\frac{s_-}{s_+}\right|^2 = \left|1 - \frac{i\gamma_{e1}}{i(\omega - \omega_1) + \gamma_1/2 + \frac{|\mu|^2}{i(\omega-\omega_2) + \gamma_2/2}}\right|^2$$

This produces a characteristic Fano lineshape when the two resonators have different quality factors, and electromagnetically induced transparency (EIT)-like features when two closely spaced resonances interfere.

---

## 8. Applications

### 8.1 Wavelength Filters: The Add-Drop Multiplexer

**Architecture:** A bus waveguide evanescently coupled to a ring resonator, which is in turn coupled to a drop port waveguide.

The ring resonator has resonances at $\omega_m = mc/(n_\text{eff} \cdot 2\pi R)$ where $m$ is an integer and $R$ is the ring radius. Only wavelengths on resonance are coupled from the bus to the ring and thence to the drop port. Off-resonance wavelengths pass straight through.

**Transfer function at the drop port** (temporal CMT, matched coupling rates $\gamma_e$):

$$T_\text{drop}(\omega) = \frac{\gamma_e^2}{(\omega - \omega_m)^2 + \gamma_e^2}$$

This is a Lorentzian with FWHM linewidth $\Delta\omega = \gamma_e$. The quality factor $Q = \omega_m/\gamma_e$.

**Wavelength selectivity:** A ring with $Q = 10^5$ at 1550 nm has linewidth $\Delta\lambda = \lambda/Q = 0.015$ nm. The free spectral range (FSR) is $\Delta\lambda_\text{FSR} = \lambda^2/(n_g \cdot 2\pi R)$. For $R = 5$ μm, FSR $\approx 15$ nm. The ratio FSR/FWHM is the **finesse** $\mathcal{F} = Q \cdot \Delta\lambda_\text{FSR}/\lambda$.

### 8.2 Directional Coupler in Silicon Photonics

A 50/50 coupler in silicon photonics (450 nm × 220 nm waveguides, gap $g = 200$ nm, $\lambda = 1550$ nm):
- $\kappa \approx 1000$ rad/mm, so $\kappa L = \pi/4$ at $L \approx 0.8$ mm
- This coupler is the fundamental building block of Mach-Zehnder interferometers (Chapter 3)

### 8.3 Photonic Topological Insulators via Coupling

Arrays of coupled resonators with engineered coupling phases can realize photonic analogues of topological insulators. The coupling matrix:

$$\mathbf{H} = \begin{pmatrix}0 & \kappa_1 & 0 & \kappa_2 & \cdots \\ \kappa_1 & 0 & \kappa_2 & 0 & \cdots \\ \vdots & & \ddots & & \end{pmatrix}$$

with alternating coupling strengths $\kappa_1 \neq \kappa_2$ realizes the SSH (Su-Schrieffer-Heeger) model. When $\kappa_1 < \kappa_2$, topological edge modes exist at zero frequency — localized at the boundary of the array, robust to disorder.

---

## 9. Beyond Two Modes: Multi-Mode Coupling and Mode Conversion

### 9.1 N-Mode Generalization

For $N$ modes, the CMT system is:

$$\frac{d\mathbf{a}}{dz} = i K \mathbf{a}, \qquad K_{mn} = \kappa_{mn} e^{i(\beta_n - \beta_m)z}$$

where $K$ is the coupling matrix. The solution is:

$$\mathbf{a}(z) = e^{iKz}\mathbf{a}(0)$$

For the coupling matrix to be Hermitian ($K = K^\dagger$, required for power conservation), one needs $\kappa_{mn} = \kappa_{nm}^*$.

### 9.2 Adiabatic Mode Evolution

If the coupling changes slowly along $z$, the system can evolve adiabatically — following the instantaneous eigenmodes of $K(z)$ rather than oscillating between them. The adiabaticity condition:

$$\left|\frac{d\kappa/dz}{\kappa}\right| \ll |\omega_+ - \omega_-| = 2\Omega$$

Adiabatic transitions are used in **adiabatic couplers** (slow taper couplers that achieve near-unity coupling efficiency) and in **adiabatic mode converters** that convert TE to TM without cross-coupling to other modes.

---

## 10. Worked Example: Design of a 3 dB Coupler

**Problem:** Design a 50/50 directional coupler in silicon strip waveguides (450 nm × 220 nm, $n = 3.47$) at $\lambda = 1550$ nm. The gap is $g = 300$ nm. Find the required coupler length.

**Step 1: Compute the coupling coefficient.**

The coupling coefficient is obtained from numerical simulation (e.g., eigenmode expansion or FDTD) or from the analytical formula. For the given geometry, simulation gives $\kappa \approx 350$ rad/mm.

**Step 2: Find the required length.**

For 50/50 splitting: $|B(L)|^2 = \sin^2(\kappa L) = 0.5$, so $\kappa L = \pi/4$.

$$L = \frac{\pi}{4\kappa} = \frac{\pi}{4 \times 350 \text{ rad/mm}} = 2.24 \text{ mm}$$

**Step 3: Check the phase mismatch.**

For identical waveguides, $\Delta\beta = 0$, so $\delta = 0$. Maximum power transfer is 1. Good.

**Step 4: Estimate the bandwidth.**

The 3 dB bandwidth is set by $|B(L)|^2 = 0.5 \pm 0.05$ (i.e., $45\%$–$55\%$ transfer). From the general formula:

$$\eta = \frac{\kappa^2}{\kappa^2 + \delta^2}\sin^2(\sqrt{\kappa^2+\delta^2}\, L)$$

Numerical solution gives the 1 dB bandwidth $\Delta\lambda \approx 30$ nm centered at 1550 nm — sufficient to cover the entire C-band.

---

## 11. Exercises

**11.1** (Easy) Starting from the canonical CMT equations $dA/dz = i\kappa B$, $dB/dz = i\kappa A$, verify by direct differentiation that $A(z) = \cos(\kappa z)$ and $B(z) = i\sin(\kappa z)$ satisfy the system with $A(0)=1$, $B(0)=0$.

**11.2** (Easy) A directional coupler with $\kappa = 500$ rad/mm is used as a 3 dB splitter. What is the required coupling length? If the coupling length is doubled, what is the splitting ratio?

**11.3** (Medium) Derive the full solution $A(z), B(z)$ for the mismatched case $\Delta\beta \neq 0$ using the matrix exponential of the CMT matrix. Show that the maximum power transfer is $\kappa^2/(\kappa^2+\delta^2)$.

**11.4** (Medium) The coupling coefficient scales as $\kappa \propto e^{-\gamma g}$ where $\gamma = 2$ μm⁻¹ for a typical silicon waveguide and $g$ is the gap. If increasing the gap from 200 nm to 300 nm halves $\kappa$, what is the implied $\gamma$? What gap gives $\kappa = 100$ rad/mm if $\kappa(200 \text{ nm}) = 1000$ rad/mm?

**11.5** (Medium) Two microring resonators, each with $\omega_0 = 2\pi c/\lambda_0$ ($\lambda_0 = 1550$ nm), $Q = 10^5$, are mutually coupled with rate $\mu = 10^{10}$ rad/s. (a) What is the normal mode splitting $\Delta\lambda = \lambda_0^2 \Delta\omega / (2\pi c)$? (b) Under what condition is this splitting resolved (greater than the linewidth)?

**11.6** (Hard) Derive the transfer matrix for a general mismatched coupler with parameters $\kappa$, $\delta$, length $L$. Show it is unitary. Find the eigenvalues and eigenvectors. What does each eigenvector represent physically?

**11.7** (Hard) In an N-site SSH chain with couplings alternating as $\kappa_1, \kappa_2, \kappa_1, \kappa_2, \ldots$ (N even), show that for $\kappa_2 > \kappa_1$, there exist zero-energy eigenmodes localized at the edges. Explicitly construct these modes for $N = 4$.

---

## 12. Further Reading

- **Textbooks:** Haus & Huang, "Coupled-mode theory" *Proc. IEEE* 79, 1505 (1991); Yariv & Yeh, *Photonics: Optical Electronics in Modern Communications* (Ch. 6); Saleh & Teich, *Fundamentals of Photonics* (Ch. 8)
- **Original Papers:** H.A. Haus et al., "Coupled-mode theory of optical waveguides," *J. Lightwave Technol.* 5, 16 (1987); W.P. Huang, "Coupled-mode theory for optical waveguides: an overview," *J. Opt. Soc. Am. A* 11, 963 (1994)
- **Computation:** For computing $\kappa$ numerically, see Lumerical MODE; for analytical approximations in weakly guiding fibers, see Snyder & Love, *Optical Waveguide Theory*
