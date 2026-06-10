# 8.1.2 Bloch's Theorem and the Photonic Band Structure

## The Master Equation

In Section 8.1.1, we treated the 1D photonic crystal using the transfer matrix. This approach gives exact results but provides limited physical intuition about what the allowed modes look like. The deeper approach — and the one that generalizes to 2D and 3D crystals — is based on Bloch's theorem.

We begin with Maxwell's equations in a medium with spatially varying dielectric constant $\varepsilon(\mathbf{r})$ and no magnetic response ($\mu = 1$). In the absence of free charges and currents:

$$\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t}, \quad \nabla \times \mathbf{H} = \frac{\partial \mathbf{D}}{\partial t}$$

with $\mathbf{D} = \varepsilon_0\varepsilon(\mathbf{r})\mathbf{E}$ and $\mathbf{B} = \mu_0\mathbf{H}$. For monochromatic fields $e^{-i\omega t}$:

$$\nabla \times \mathbf{E} = i\omega\mu_0\mathbf{H}$$
$$\nabla \times \mathbf{H} = -i\omega\varepsilon_0\varepsilon(\mathbf{r})\mathbf{E}$$

Taking the curl of the first equation and substituting the second:

$$\nabla \times \left(\nabla \times \mathbf{E}\right) = i\omega\mu_0\nabla \times \mathbf{H} = i\omega\mu_0(-i\omega\varepsilon_0\varepsilon(\mathbf{r})\mathbf{E}) = \omega^2\mu_0\varepsilon_0\varepsilon(\mathbf{r})\mathbf{E}$$

Using $\nabla \times (\nabla \times \mathbf{E}) = \nabla(\nabla \cdot \mathbf{E}) - \nabla^2\mathbf{E}$, and $\nabla \cdot (\varepsilon(\mathbf{r})\mathbf{E}) = 0$ (Gauss's law in this form; note: $\varepsilon(\mathbf{r})\nabla \cdot \mathbf{E} + \mathbf{E}\cdot\nabla\varepsilon = 0$ so $\nabla \cdot \mathbf{E} = -\mathbf{E}\cdot\nabla\ln\varepsilon$, which is zero if $\varepsilon$ is uniform), this becomes complicated for inhomogeneous media.

More natural is the **master equation for H**, obtained by eliminating **E**. Substituting $\mathbf{E} = (i/\omega\varepsilon_0\varepsilon)\nabla\times\mathbf{H}$ into the curl equation for $\mathbf{E}$:

$$\nabla \times \left(\frac{1}{\varepsilon(\mathbf{r})}\nabla \times \mathbf{H}\right) = \left(\frac{\omega}{c}\right)^2 \mathbf{H}$$

This is the **master equation of photonic crystals**, often written as:

$$\hat{\Theta}\mathbf{H} = \left(\frac{\omega}{c}\right)^2 \mathbf{H}$$

where $\hat{\Theta} = \nabla \times \left(\frac{1}{\varepsilon(\mathbf{r})}\nabla \times\right)$ is a Hermitian operator (it can be shown that $\hat{\Theta}$ is Hermitian and positive semi-definite when $\varepsilon > 0$) [1].

The Hermiticity is crucial: it guarantees that all eigenvalues $(\omega/c)^2$ are real and non-negative, and that eigenmodes are orthogonal. This is the photonic analog of the Schrödinger equation $\hat{H}|\psi\rangle = E|\psi\rangle$, with the correspondence:
- $\hat{\Theta} \leftrightarrow \hat{H}$
- $(\omega/c)^2 \leftrightarrow E$
- $\mathbf{H}(\mathbf{r}) \leftrightarrow \psi(\mathbf{r})$

## Bloch's Theorem Applied to Photons

For a periodic dielectric $\varepsilon(\mathbf{r}) = \varepsilon(\mathbf{r} + \mathbf{R})$ for all lattice vectors $\mathbf{R} = n_1\mathbf{a}_1 + n_2\mathbf{a}_2 + n_3\mathbf{a}_3$, the operator $\hat{\Theta}$ commutes with the discrete translation operators $\hat{T}_\mathbf{R}$ defined by $(\hat{T}_\mathbf{R}\mathbf{H})(\mathbf{r}) = \mathbf{H}(\mathbf{r}+\mathbf{R})$.

Bloch's theorem (applied to the photonic system): the eigenmodes of $\hat{\Theta}$ in a periodic medium can be chosen to have the form:

$$\mathbf{H}_{n\mathbf{k}}(\mathbf{r}) = e^{i\mathbf{k}\cdot\mathbf{r}}\mathbf{u}_{n\mathbf{k}}(\mathbf{r})$$

where $\mathbf{u}_{n\mathbf{k}}(\mathbf{r}) = \mathbf{u}_{n\mathbf{k}}(\mathbf{r}+\mathbf{R})$ is periodic with the lattice periodicity, $\mathbf{k}$ is the Bloch wavevector (confined to the first Brillouin zone), and $n$ is the band index.

The eigenvalues form discrete bands $\omega_n(\mathbf{k})$ — the **photonic band structure**. By symmetry:
- $\omega_n(\mathbf{k}) = \omega_n(-\mathbf{k})$ (time-reversal symmetry)
- $\omega_n(\mathbf{k}) = \omega_n(\mathbf{k}+\mathbf{G})$ for any reciprocal lattice vector $\mathbf{G}$ (lattice periodicity)

## The Reciprocal Lattice and First Brillouin Zone

For a 2D triangular lattice (the most common for photonic crystals) with lattice constant $a$:
- Primitive vectors: $\mathbf{a}_1 = a(1, 0)$, $\mathbf{a}_2 = a(1/2, \sqrt{3}/2)$
- Reciprocal lattice vectors: $\mathbf{b}_1 = (2\pi/a)(1, -1/\sqrt{3})$, $\mathbf{b}_2 = (2\pi/a)(0, 2/\sqrt{3})$
- First Brillouin zone: hexagonal, with high-symmetry points $\Gamma$ (center), $M$ (edge midpoint), $K$ (corner)

The photonic band structure is plotted along the high-symmetry path $\Gamma \to M \to K \to \Gamma$, analogous to the electronic band structure of graphene on the same lattice.

## Counting Modes: Density of States

The density of states (DOS) $g(\omega)d\omega$ counts the number of modes per unit cell in the frequency range $[\omega, \omega + d\omega]$:

$$g(\omega) = \sum_n \int_{\text{BZ}} \delta(\omega - \omega_n(\mathbf{k}))\frac{d\mathbf{k}}{(2\pi)^d/V_{\text{BZ}}}$$

At the bandgap, $g(\omega) = 0$ — no modes, no propagation. At band edges (where $|\nabla_\mathbf{k}\omega| \to 0$), the DOS diverges as a van Hove singularity. These singularities are associated with slow-light behavior (Section 8.1.4).

## The Scale Invariance of Maxwell's Equations

One of the most useful properties of the master equation is its **scale invariance**. If $\mathbf{H}(\mathbf{r})$ is a mode with frequency $\omega$ in a crystal with lattice constant $a$, then $\mathbf{H}(\mathbf{r}/s)$ is a mode with frequency $s\omega$ in a geometrically scaled crystal with lattice constant $sa$. This follows directly from the absence of any fundamental length scale in Maxwell's equations (unlike Schrödinger's equation, which has $\hbar$ and $m$).

Consequence: photonic crystal band structures can be plotted in dimensionless units $\omega a/c$ and $ka/2\pi$, and scaled to any wavelength by rescaling the lattice constant. A crystal designed for 1550 nm can be operated at 780 nm simply by halving all dimensions.

This scale invariance also means that **no material parameters matter except $\varepsilon(\mathbf{r})$** — the band structure depends only on the geometry (ratio of dimensions to lattice constant) and the dielectric contrast $\varepsilon_{\text{high}}/\varepsilon_{\text{low}}$.

## Numerical Methods for Band Structures

Computing photonic band structures requires solving the eigenvalue problem $\hat{\Theta}\mathbf{H} = (\omega/c)^2\mathbf{H}$ numerically. The standard approach is the **plane wave expansion method**:

1. Expand $\varepsilon^{-1}(\mathbf{r})$ in Fourier series: $\varepsilon^{-1}(\mathbf{r}) = \sum_\mathbf{G}\varepsilon^{-1}_\mathbf{G} e^{i\mathbf{G}\cdot\mathbf{r}}$

2. Expand the Bloch mode: $\mathbf{H}_{n\mathbf{k}}(\mathbf{r}) = \sum_\mathbf{G}\mathbf{h}_{n,\mathbf{k}+\mathbf{G}} e^{i(\mathbf{k}+\mathbf{G})\cdot\mathbf{r}}$

3. Substitute into the master equation to get a matrix eigenvalue problem with matrix elements proportional to $(\mathbf{k}+\mathbf{G}) \times [(\mathbf{k}+\mathbf{G}') \times \mathbf{h}_{n,\mathbf{k}+\mathbf{G}'}] \times \varepsilon^{-1}_{\mathbf{G}-\mathbf{G}'}$

4. Solve numerically. The MIT Photonic Bands (MPB) software package implements this efficiently and is freely available [2].

For a 2D triangular lattice of air holes (radius $r = 0.3a$) in silicon ($\varepsilon = 12$), the band structure shows a complete photonic bandgap for TE-like modes (electric field in the plane) between the first and second bands, centered at $\omega a/c \approx 0.28$. For $a = 434$ nm, this corresponds to a gap centered at 1550 nm.

---

## References

[1] Joannopoulos, J.D., Johnson, S.G., Winn, J.N., & Meade, R.D. (2008). *Photonic Crystals: Molding the Flow of Light*, 2nd ed. Princeton University Press. [The master equation and its Hermiticity are derived in Chapter 2; Bloch's theorem is in Chapter 3.]

[2] Johnson, S.G. & Joannopoulos, J.D. (2001). "Block-iterative frequency-domain methods for Maxwell's equations in a planewave basis." *Optics Express*, 8(3), 173–190. [The MPB (MIT Photonic Bands) paper; open-source at mpb.readthedocs.io.]
