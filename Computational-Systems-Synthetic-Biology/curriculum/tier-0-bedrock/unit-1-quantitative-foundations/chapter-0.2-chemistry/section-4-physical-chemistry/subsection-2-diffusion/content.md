# Diffusion

There is a classic calculation in developmental biology: how long does it take for a morphogen to spread from one side of a *Drosophila* wing disc to the other? The disc is about 200 micrometers across. The morphogen Dpp (a BMP family member) has a diffusion coefficient of roughly 0.1 µm²/s and degrades with a half-life of about 90 minutes. Plug those numbers into the diffusion-degradation equation, and you get a concentration gradient with a decay length of about 23 micrometers — spanning about a quarter of the disc, exactly as observed. The calculation takes five minutes. It explains decades of experiments.

This is what physics brings to biology: the ability to reason quantitatively about spatial phenomena without simulating every individual molecule. Diffusion is the random thermal motion of molecules through a medium, driven by concentration gradients. It is the dominant transport mechanism for molecules at the cellular scale, and understanding diffusion quantitatively is essential for modeling morphogen gradients, signal transduction kinetics, the speed of gene regulation, and the design of microfluidic devices.

## Fick's Laws of Diffusion

**Fick's First Law** relates the diffusion flux (amount of substance crossing a unit area per unit time) to the concentration gradient:

$$J = -D \frac{\partial c}{\partial x}$$

The negative sign means flux opposes the gradient — material flows from high to low concentration. $D$ is the **diffusion coefficient** with units m$^2$/s (or $\mu$m$^2$/s in biology).

**Fick's Second Law** (continuity equation combined with Fick's First Law) gives the time evolution of concentration:

$$\frac{\partial c}{\partial t} = D \nabla^2 c = D \left(\frac{\partial^2 c}{\partial x^2} + \frac{\partial^2 c}{\partial y^2} + \frac{\partial^2 c}{\partial z^2}\right)$$

This is the diffusion equation. Solutions depend on initial and boundary conditions (see Section 1.4 on PDEs).

**The key length-time scaling:** For diffusion, the characteristic length over which a concentration perturbation spreads in time $t$ is:

$$l \sim \sqrt{D \cdot t}$$

This is the single most important relationship in diffusion biology. For a protein with $D \approx 10\ \mu\text{m}^2/\text{s}$:
- $t = 1$ s: $l \approx 3.2\ \mu\text{m}$ (nucleus diameter)
- $t = 100$ s: $l \approx 32\ \mu\text{m}$ (cell diameter)
- $t = 1\ \text{hr}$: $l \approx 190\ \mu\text{m}$ (tissue scale)

A signaling protein produced at the nucleus must diffuse $\sim 10\ \mu\text{m}$ to reach the plasma membrane, taking seconds. A morphogen must spread across an embryo ($\sim 500\ \mu\text{m}$), taking hours — consistent with developmental timescales.

## The Einstein Relation

The diffusion coefficient is related to molecular properties through the **Einstein-Stokes relation**:

$$D = \frac{k_B T}{6 \pi \eta r}$$

where $k_B$ is Boltzmann's constant, $T$ is temperature, $\eta$ is dynamic viscosity of the medium ($\eta_{\text{water, 37°C}} \approx 0.69 \times 10^{-3}$ Pa·s), and $r$ is the hydrodynamic radius.

**Implications:**
- Larger molecules diffuse more slowly: $D \propto 1/r \propto 1/M^{1/3}$ (for globular proteins, $M$ is molecular weight)
- Diffusion increases with temperature (directly through $T$ and indirectly through decreasing viscosity)
- In the crowded cellular cytoplasm, effective diffusion coefficients are 5–10 times slower than in dilute solution

**Typical diffusion coefficients:**
| Molecule | $D$ ($\mu$m$^2$/s) | Notes |
|---|---|---|
| Small metabolite (glucose) | ~600 | In water |
| GFP (~27 kDa) | ~90 | In water; ~25 in cytoplasm |
| Protein (~100 kDa) | ~50 | In water; ~10 in cytoplasm |
| Ribosome (~3 MDa) | ~2 | In cytoplasm |
| mRNA | ~0.03 | In cytoplasm (anomalous diffusion) |
| DNA (chromosomal) | ~0.001 | Essentially immobile on short timescales |

## Reaction-Diffusion and Morphogen Gradients

A morphogen is a signaling molecule produced at a localized source that diffuses and degrades, establishing a concentration gradient across a tissue. At steady state (production balanced by degradation and diffusion), the 1D steady-state profile satisfies:

$$D \frac{\partial^2 c}{\partial x^2} - \delta c + p(x) = 0$$

For a localized source at $x = 0$ with concentration $c_0$ and no-flux at $x \to \infty$:

$$c(x) = c_0 e^{-x/\lambda}, \quad \lambda = \sqrt{D/\delta}$$

where $\lambda$ is the **decay length** (spatial range of the gradient). $\lambda$ depends on the ratio of diffusion to degradation: fast diffusion or slow degradation gives a long-range gradient; slow diffusion or fast degradation gives a short-range gradient.

**Dpp (BMP4) in *Drosophila* wing disc:** $D \approx 0.1\ \mu\text{m}^2/\text{s}$, $t_{1/2} \approx 90$ min → $\lambda \approx \sqrt{0.1 \times 90 \times 60} \approx 23\ \mu\text{m}$. The wing disc is ~200 µm wide — the gradient spans the entire structure, consistent with measurement.

**How cells read gradients:** Cells must detect a spatial difference in morphogen concentration. The relative difference in receptor occupancy between the "front" and "back" of a cell body (diameter $\sim 10\ \mu\text{m}$) in a gradient with $\lambda = 20\ \mu\text{m}$:

$$\Delta c / c \approx 10/20 = 50\%$$

This is a detectable signal; computing this sensitivity ratio for different gradient profiles and cell sizes is a standard exercise in developmental biology.

## Anomalous Diffusion

In biological cells, diffusion is often **anomalous** — the mean-square displacement scales as $\langle r^2(t) \rangle \propto t^\alpha$ where $\alpha \neq 1$.

- **Normal diffusion:** $\alpha = 1$ (Fickian)
- **Subdiffusion ($\alpha < 1$):** Common for mRNA, chromatin loci, membrane proteins. Caused by molecular crowding, transient binding, and confinement in compartments.
- **Superdiffusion ($\alpha > 1$):** Active transport by motor proteins along cytoskeletal tracks.

Understanding anomalous diffusion is important for interpreting single-particle tracking experiments and for accurately modeling reaction kinetics in cells where substrate and enzyme are not uniformly distributed.

## Why This Matters for Computational Biology

Diffusion is the physical basis of spatial organization in biology. Every reaction-diffusion model of morphogenesis, gradient reading, or calcium signaling depends on accurate diffusion coefficients. Fluorescence recovery after photobleaching (FRAP) measures effective diffusion coefficients in cells — these measurements feed into models. The diffusion length $\lambda = \sqrt{D/\delta}$ appears in every spatially extended model of gene circuits (e.g., quorum sensing: how far does an AHL signal travel before degrading?). In microfluidics, which is increasingly used to create controlled environments for synthetic biological systems, diffusion dictates gradient shapes and mixing timescales. Knowing how molecules move in space is inseparable from understanding how biological systems work.

```python
import numpy as np
import matplotlib.pyplot as plt

# Morphogen gradient model: steady-state exponential decay
def morphogen_gradient(x, c0, D, delta):
    """Steady-state morphogen gradient with diffusion D and degradation delta."""
    lam = np.sqrt(D / delta)  # decay length
    return c0 * np.exp(-x / lam)

# Parameters for Dpp-like morphogen
D = 0.1       # um^2/s
delta = 0.1 / (90 * 60)  # 1/s, half-life = 90 min -> delta = ln2/t_half
c0 = 1.0      # normalized source concentration

lam = np.sqrt(D / delta)
print(f"Decay length lambda = {lam:.1f} µm")

x = np.linspace(0, 200, 500)  # tissue extends 200 µm
c = morphogen_gradient(x, c0, D, delta)

fig, axes = plt.subplots(1, 2, figsize=(12, 4))

# Linear plot
axes[0].plot(x, c)
axes[0].axvline(lam, linestyle='--', color='red', label=f'λ = {lam:.0f} µm')
axes[0].set_xlabel('Distance from source (µm)')
axes[0].set_ylabel('Morphogen concentration (c/c₀)')
axes[0].set_title('Morphogen Gradient (linear scale)')
axes[0].legend()

# Log plot (exponential appears linear)
axes[1].semilogy(x, c)
axes[1].axvline(lam, linestyle='--', color='red', label=f'λ = {lam:.0f} µm')
axes[1].set_xlabel('Distance from source (µm)')
axes[1].set_ylabel('log(c/c₀)')
axes[1].set_title('Morphogen Gradient (log scale)')
axes[1].legend()

plt.tight_layout()

# Diffusion timescale: how long to diffuse distance L?
print("\nDiffusion timescales (t ~ L^2 / 2D):")
D_protein = 10  # um^2/s in cytoplasm
for L_um in [1, 5, 10, 50]:
    t_s = L_um**2 / (2 * D_protein)
    print(f"  L = {L_um} µm: t ≈ {t_s:.1f} s")
```
