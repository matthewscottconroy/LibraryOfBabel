# 2.5.2 — Spatial Coherence

## Two Points, One Time

Temporal coherence concerns the relationship between the field at one point at two different times. Spatial coherence concerns the relationship between the fields at two different points at the same time.

The *mutual coherence function* in its full spatiotemporal form is:

$$\Gamma(\mathbf{r}_1, \mathbf{r}_2, \tau) = \langle E^*(\mathbf{r}_1, t) E(\mathbf{r}_2, t + \tau) \rangle$$

For quasi-monochromatic light (narrow bandwidth compared to center frequency), we can separate the spatial and temporal dependences. The *mutual intensity* (equal-time correlation):

$$J(\mathbf{r}_1, \mathbf{r}_2) = \Gamma(\mathbf{r}_1, \mathbf{r}_2, 0) = \langle E^*(\mathbf{r}_1, t) E(\mathbf{r}_2, t) \rangle$$

The *complex degree of spatial coherence*:

$$\mu(\mathbf{r}_1, \mathbf{r}_2) = \frac{J(\mathbf{r}_1, \mathbf{r}_2)}{\sqrt{I(\mathbf{r}_1) I(\mathbf{r}_2)}}$$

By the Cauchy-Schwarz inequality, $|\mu(\mathbf{r}_1, \mathbf{r}_2)| \leq 1$. The fringe visibility in a Young's double-slit experiment with slits at $\mathbf{r}_1$ and $\mathbf{r}_2$ is exactly $V = |\mu(\mathbf{r}_1, \mathbf{r}_2)|$.

## Spatial Coherence Length

The *spatial coherence length* (or *lateral coherence length*) $l_c$ is the characteristic separation over which $|\mu|$ drops from 1 to approximately $1/e$. For separations $|\mathbf{r}_1 - \mathbf{r}_2| \ll l_c$, the field values are highly correlated; for separations $\gg l_c$, they are essentially uncorrelated.

The spatial coherence length is related to the angular size $\theta_s$ of the source (as seen from the observation plane) by:

$$l_c \approx \frac{\lambda}{\theta_s}$$

(in one dimension; the exact coefficient depends on the source geometry). This is the van Cittert-Zernike result, discussed more fully in Section 2.5.3.

**Examples**:
| Source | Angular size $\theta_s$ | Spatial coherence length $l_c$ at $\lambda = 550$ nm |
|--------|------------------------|------------------------------------------------------|
| Sun | 9.3 mrad | $\sim 60$ μm |
| Mercury arc lamp (filtered) | 2 mrad | $\sim 275$ μm |
| LED (10 mm chip, 1 m away) | 10 mrad | $\sim 55$ μm |
| Laser (collimated beam, divergence $\sim 1$ mrad) | 1 mrad | $\sim 550$ μm |
| Single-mode fiber output | essentially a point source | $\sim \infty$ (limited by aperture) |

Note: for a single-mode laser or single-mode fiber output, the light emerges from a diffraction-limited point source and has essentially unlimited spatial coherence at any distance — the entire output beam is spatially coherent.

## Coherent Mode Decomposition

Any partially coherent field can be decomposed into a sum of mutually incoherent, fully coherent *modes* (Wolf's coherent mode representation) [1]:

$$J(\mathbf{r}_1, \mathbf{r}_2) = \sum_n \lambda_n \phi_n^*(\mathbf{r}_1) \phi_n(\mathbf{r}_2)$$

where $\lambda_n \geq 0$ are eigenvalues and $\phi_n$ are orthonormal eigenfunctions (the coherent modes). This is the spectral decomposition of the mutual intensity operator.

**Fully coherent** light: only one term, $J(\mathbf{r}_1, \mathbf{r}_2) = E^*(\mathbf{r}_1)E(\mathbf{r}_2)$ — separable. $|\mu| = 1$ everywhere.

**Partially coherent** light: multiple terms, incoherent superposition of modes.

**Completely incoherent** light: $J(\mathbf{r}_1, \mathbf{r}_2) = I(\mathbf{r}_1)\delta(\mathbf{r}_1 - \mathbf{r}_2)$ — zero coherence between any two distinct points.

The coherent mode decomposition shows that the number of independent coherent modes in a field (the *number of modes* or *number of degrees of freedom*) is the number of significant eigenvalues $\lambda_n$. For a field of spatial extent $A$ with spatial coherence area $A_c = l_c^2$, the number of independent modes is roughly $N_\text{modes} \approx A/A_c$.

**For photonic computing**: A single-mode waveguide supports exactly one spatial mode — it carries a fully coherent field. An MZI connecting two single-mode waveguides operates entirely on coherent fields. Multiple single-mode waveguides, each carrying a coherent field, form an $N$-mode system where the $N$ fields are coherent within each waveguide but potentially incoherent between waveguides (if they come from different laser sources). For MZI-mesh computing, it is essential that all modes come from the same coherent source (or sources locked in phase), so that interference between any pair of modes is possible.

## Spatial Coherence at the Chip Input

Most photonic computing chips are illuminated by a single coherent laser whose output is split into $N$ input channels by a waveguide splitter network. Since all channels derive from the same source, they are mutually coherent — the relative phases are deterministic and controlled. This is precisely the condition for full spatial coherence across the input to the processing network.

If instead the $N$ channels came from $N$ independent laser sources, they would be mutually incoherent (in the statistical sense that their relative phases fluctuate randomly). Interference between channels would average to zero over time, and the MZI network would produce only time-averaged intensity additions, not coherent interference. The computation would be fundamentally different — and less powerful.

This is the physical basis of the distinction between coherent and incoherent optical neural networks:
- **Coherent**: single source, all modes mutually coherent, can exploit interference. Field amplitudes are the computational variables. Can implement complex-valued (unitary) matrix multiplications.
- **Incoherent**: modes incoherent with each other, cannot exploit interference. Intensities are the computational variables. Can only implement non-negative-real-valued matrix multiplications (more limited but potentially more robust to phase noise).

## Summary

- Mutual intensity $J(\mathbf{r}_1, \mathbf{r}_2) = \langle E^*(\mathbf{r}_1) E(\mathbf{r}_2) \rangle$; degree of spatial coherence $\mu = J/\sqrt{I_1 I_2}$.
- Fringe visibility in a double-slit experiment = $|\mu|$ for the two slit positions.
- Spatial coherence length $l_c \approx \lambda/\theta_s$ (inverse of the source angular size).
- Single-mode laser/fiber: fully spatially coherent (unlimited $l_c$).
- Coherent photonic computing requires all input channels to derive from the same coherent source.

---

*References*

[1] Wolf, E. (1982). New theory of partial coherence in the space-frequency domain. Part I: spectra and cross-spectra of steady-state sources. *Journal of the Optical Society of America*, 72(3), 343–351. [DOI: 10.1364/JOSA.72.000343] [Wolf's coherent mode decomposition.]

[2] Mandel, L. & Wolf, E. (1995). *Optical Coherence and Quantum Optics*. Cambridge University Press. [The comprehensive reference on coherence theory; Chapter 4–5 cover spatial coherence.]
