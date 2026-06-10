# 1.4.4 Complex Notation, Phasors, and Physical Fields

## Why Complex Exponentials?

Working with complex exponentials $e^{i(\mathbf{k}\cdot\mathbf{r}-\omega t)}$ is mathematically far more convenient than working with sines and cosines. Addition, differentiation, and multiplication all become algebraic operations. But electromagnetic fields are real physical quantities — measurable with voltmeters and antennas. So we must be precise about the relationship between the complex mathematical representation and the physical fields.

The convention adopted throughout this book (consistent with most optics and photonics literature) is:

$$\mathbf{E}_{\text{physical}}(\mathbf{r}, t) = \text{Re}\left[\mathbf{E}_0 e^{i(\mathbf{k}\cdot\mathbf{r}-\omega t)}\right]$$

where $\mathbf{E}_0$ may be complex (to allow for an arbitrary initial phase). Since Maxwell's equations are linear and have real coefficients, the real and imaginary parts of a complex solution each independently satisfy the equations. So if $\mathbf{E}_{\text{complex}}$ satisfies Maxwell's equations, so does $\text{Re}[\mathbf{E}_{\text{complex}}]$, which is the physical field.

## The Phasor Representation

For calculations involving a single frequency $\omega$, it is convenient to suppress the time dependence entirely and work with the **complex amplitude** (phasor):

$$\tilde{\mathbf{E}}(\mathbf{r}) = \mathbf{E}_0 e^{i\mathbf{k}\cdot\mathbf{r}}$$

so that $\mathbf{E}(\mathbf{r}, t) = \text{Re}[\tilde{\mathbf{E}}(\mathbf{r}) e^{-i\omega t}]$.

Under this convention, time differentiation $\partial/\partial t$ is replaced by multiplication by $-i\omega$:

$$\frac{\partial}{\partial t} \to -i\omega$$

Maxwell's equations become the time-harmonic (frequency-domain) form:

$$\nabla \cdot \tilde{\mathbf{E}} = \rho/\varepsilon_0$$
$$\nabla \cdot \tilde{\mathbf{B}} = 0$$
$$\nabla \times \tilde{\mathbf{E}} = i\omega\tilde{\mathbf{B}}$$
$$\nabla \times \tilde{\mathbf{B}} = \mu_0\tilde{\mathbf{J}} - i\omega\mu_0\varepsilon_0\tilde{\mathbf{E}}$$

These are *algebraic* in $\omega$ — for each frequency, we solve a system of differential equations in space, without any time derivatives. This is the form most often encountered in numerical electromagnetic simulations (FDFD, FEM).

## The Helmholtz Equation

In a uniform medium with no sources, the time-harmonic wave equation becomes:

$$\nabla^2 \tilde{\mathbf{E}} + k^2 \tilde{\mathbf{E}} = 0 \quad \text{where} \quad k = n\omega/c$$

This is the **Helmholtz equation**. It is an eigenvalue problem: given the boundary conditions (e.g., the geometry of a waveguide), find the field distributions $\tilde{\mathbf{E}}$ (eigenfunctions) and the corresponding propagation constants $k$ (eigenvalues). The allowed field distributions are the **modes** of the structure.

This is the equation that governs light propagation in every photonic device: optical fibers, silicon waveguides, photonic crystal slabs, microring resonators. The modes of a structure completely characterize what field distributions can exist inside it.

## Caution: Intensity and the Factor of 1/2

A critical caution when using complex notation: the time-averaged intensity (power per unit area) is not simply $|\tilde{\mathbf{E}}|^2$ but:

$$I = \langle \mathbf{S} \rangle_t = \frac{1}{2}\text{Re}[\tilde{\mathbf{E}} \times \tilde{\mathbf{H}}^*]$$

The factor of $1/2$ appears because $\langle\cos^2(\omega t)\rangle_t = 1/2$.

This is a source of constant errors. When computing power in a photonic circuit — the power splitting ratio of a directional coupler, the transmission of a ring resonator, the output intensity of an MZI — always use the time-averaged Poynting vector, which includes the factor of $1/2$.

## Complex Refractive Index

In an absorbing medium, the refractive index is complex:
$$\tilde{n} = n + i\kappa$$

where $n$ is the (real) refractive index and $\kappa \geq 0$ is the **extinction coefficient**. The plane wave in such a medium is:

$$\tilde{\mathbf{E}} = \mathbf{E}_0 e^{i\tilde{k}z} = \mathbf{E}_0 e^{i(n+i\kappa)\omega z/c} = \mathbf{E}_0 e^{in\omega z/c} e^{-\kappa\omega z/c}$$

The first factor $e^{in\omega z/c}$ represents the oscillation (phase). The second factor $e^{-\kappa\omega z/c}$ represents exponential decay of the amplitude. The intensity (proportional to amplitude squared) decays as:

$$I(z) = I_0 e^{-\alpha z}, \quad \alpha = \frac{2\kappa\omega}{c} = \frac{4\pi\kappa}{\lambda_0}$$

This is the **Beer-Lambert law**, with absorption coefficient $\alpha$ [m⁻¹].

For silicon at 1550 nm: $\kappa \approx 0$ (silicon is transparent at this wavelength). For silicon at 400 nm (visible light): $\kappa \approx 0.07$, and silicon strongly absorbs light — this is why silicon photonic chips are designed for near-infrared operation, not visible.

The complex refractive index will reappear throughout this book: in the analysis of absorption losses in waveguides, in the gain of lasers (where $\kappa < 0$), in the plasma dispersion effect in silicon modulators, and in the nonlinear optical processes of Chapter 3.
