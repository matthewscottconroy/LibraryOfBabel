# 1.4.2 Plane Wave Solutions

## The Plane Wave Ansatz

A **plane wave** is the simplest solution to the wave equation — one in which the field is the same at all points on a plane perpendicular to the direction of propagation. If the wave propagates in the $z$-direction, a plane wave has the same field value at all $(x, y)$ for any fixed $z$ and $t$.

We try the ansatz (an educated guess about the form of the solution):

$$\mathbf{E}(\mathbf{r}, t) = \mathbf{E}_0 e^{i(\mathbf{k}\cdot\mathbf{r} - \omega t)}$$

where:
- $\mathbf{E}_0$ is the (complex) amplitude vector, constant in space and time
- $\mathbf{k}$ is the **wavevector** — a vector in the direction of propagation with magnitude $k = |\mathbf{k}|$
- $\omega$ is the angular frequency (rad/s), related to frequency $f$ by $\omega = 2\pi f$
- $\mathbf{r} = x\hat{\mathbf{x}} + y\hat{\mathbf{y}} + z\hat{\mathbf{z}}$ is the position vector

The physical field is the real part: $\mathbf{E}_{\text{phys}}(\mathbf{r}, t) = \text{Re}[\mathbf{E}_0 e^{i(\mathbf{k}\cdot\mathbf{r}-\omega t)}]$.

## Substituting into the Wave Equation

Compute the Laplacian of the ansatz:
$$\nabla^2 \left(\mathbf{E}_0 e^{i\mathbf{k}\cdot\mathbf{r}}\right) = \frac{\partial^2}{\partial x^2}\left(e^{i(k_x x + k_y y + k_z z)}\right)\mathbf{E}_0 + \cdots = -k^2 \mathbf{E}_0 e^{i\mathbf{k}\cdot\mathbf{r}}$$

where $k^2 = k_x^2 + k_y^2 + k_z^2 = |\mathbf{k}|^2$.

Compute the time derivative:
$$\frac{\partial^2}{\partial t^2}\left(e^{-i\omega t}\right) = -\omega^2 e^{-i\omega t}$$

Substituting into the wave equation $\nabla^2 \mathbf{E} = (n^2/c^2)\partial^2 \mathbf{E}/\partial t^2$:

$$-k^2 \mathbf{E}_0 e^{i(\mathbf{k}\cdot\mathbf{r}-\omega t)} = -\frac{n^2\omega^2}{c^2}\mathbf{E}_0 e^{i(\mathbf{k}\cdot\mathbf{r}-\omega t)}$$

This is satisfied if and only if:

$$k^2 = \frac{n^2\omega^2}{c^2} \quad \Leftrightarrow \quad k = \frac{n\omega}{c} = \frac{2\pi n}{\lambda_0}$$

This is the **dispersion relation** — the relationship between the wavevector magnitude $k$ and the angular frequency $\omega$.

## Physical Interpretation

The plane wave $\mathbf{E}_0 e^{i(\mathbf{k}\cdot\mathbf{r}-\omega t)}$ represents a pattern of oscillation that:
- Has amplitude $|\mathbf{E}_0|$ everywhere in the plane
- Has a **phase** $\phi = \mathbf{k}\cdot\mathbf{r} - \omega t$
- Has **surfaces of constant phase** (wavefronts) that are planes perpendicular to $\mathbf{k}$
- Moves in the direction of $\mathbf{k}$ at the **phase velocity**:

$$v_p = \frac{\omega}{k} = \frac{c}{n}$$

The wavelength (distance between successive wavefronts) is:
$$\lambda = \frac{2\pi}{k} = \frac{\lambda_0}{n}$$

## Transversality: E ⊥ B ⊥ k

Substituting the plane wave into $\nabla \cdot \mathbf{E} = 0$:

$$\nabla \cdot (\mathbf{E}_0 e^{i\mathbf{k}\cdot\mathbf{r}}) = i\mathbf{k}\cdot\mathbf{E}_0 e^{i\mathbf{k}\cdot\mathbf{r}} = 0$$

Therefore: $\mathbf{k} \cdot \mathbf{E}_0 = 0$.

**The electric field is perpendicular to the wavevector.** The wave is **transverse**.

Similarly, from $\nabla \cdot \mathbf{B} = 0$: $\mathbf{k} \cdot \mathbf{B}_0 = 0$. The magnetic field is also transverse.

From Faraday's law, $\nabla \times \mathbf{E} = -\partial\mathbf{B}/\partial t$, substituting the plane wave:

$$i\mathbf{k} \times \mathbf{E}_0 = i\omega \mathbf{B}_0 \quad \Rightarrow \quad \mathbf{B}_0 = \frac{\mathbf{k} \times \mathbf{E}_0}{\omega} = \frac{n}{c}\hat{\mathbf{k}} \times \mathbf{E}_0$$

The magnetic field is perpendicular to both $\mathbf{k}$ and $\mathbf{E}$. The three vectors $(\mathbf{k}, \mathbf{E}_0, \mathbf{B}_0)$ form a right-handed orthogonal triad.

The magnitude of $\mathbf{B}_0$:
$$B_0 = \frac{n}{c} E_0 = \frac{E_0}{v_p}$$

For light in vacuum: $B_0 = E_0/c$. The electric field is much stronger than the magnetic field in ordinary SI units: for $E_0 = 1$ V/m, $B_0 = 3.3 \times 10^{-9}$ T.

## The Wavevector and Phase Accumulation

The wavevector $\mathbf{k}$ is central to all of photonic computing. Its magnitude:

$$k = \frac{2\pi n}{\lambda_0}$$

is the spatial frequency of the electromagnetic wave — the number of radians of phase accumulated per meter of propagation.

**Phase accumulation** over a path length $L$:
$$\Delta\phi = kL = \frac{2\pi n L}{\lambda_0}$$

This is the optical path length (OPL) expressed in radians. It is the quantity that determines how light interferes at the output of a Mach-Zehnder interferometer (Chapter 2) and how phase is encoded and processed in photonic neural networks (Chapter 12).

**Worked example: phase in a silicon modulator.** A silicon MZI modulator has an arm length of $L = 1$ mm and an effective index $n = 3.47$ at 1550 nm. The phase accumulated is:

$$\Delta\phi = \frac{2\pi \times 3.47 \times 10^{-3}}{1.55 \times 10^{-6}} = \frac{2\pi \times 3.47}{1.55 \times 10^{-3}} \approx 14,067\ \text{rad}$$

A change in phase of $\pi$ radians (half a "turn") flips the MZI from its "on" state to its "off" state. A plasma-dispersion-induced change $\Delta n \approx 10^{-4}$ produces $\Delta\phi = 2\pi \times 10^{-4} \times 10^{-3} / (1.55 \times 10^{-6}) \approx 0.40$ rad — enough for significant modulation over this length scale.

## Group Velocity and Dispersion

For a medium with dispersion, the wavevector depends on frequency: $k = k(\omega)$. The phase velocity describes the motion of a monochromatic wave. But a real signal — a pulse — consists of many frequency components. The **group velocity** describes the velocity at which the envelope of a pulse travels:

$$v_g = \frac{d\omega}{dk}\bigg|_{\omega_0}$$

The group velocity is not the same as the phase velocity when $n$ depends on $\omega$. Specifically:

$$v_g = \frac{c}{n_g}$$

where $n_g = n - \lambda \frac{dn}{d\lambda}$ is the **group index**. For silicon at 1550 nm, $n \approx 3.47$ but $n_g \approx 4.2$ — the pulse travels more slowly than the phase fronts.

The rate at which the group velocity changes with frequency is the **group velocity dispersion** (GVD):

$$\beta_2 = \frac{d^2k}{d\omega^2}\bigg|_{\omega_0}$$

Dispersion broadens pulses in time during propagation — a crucial issue for fiber-optic communications and for pulsed photonic computing systems. We treat dispersion in detail in Chapter 6.
