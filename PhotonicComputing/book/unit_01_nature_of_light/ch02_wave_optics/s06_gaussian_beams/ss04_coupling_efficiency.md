# 2.6.4 — Coupling Efficiency

## The Mode Overlap Integral

When a light beam of field profile $E_\text{beam}(x,y)$ is launched into a waveguide or fiber with mode profile $E_\text{mode}(x,y)$, the fraction of beam power that is coupled into the guided mode is determined by the *mode overlap integral*:

$$\eta = \frac{\left|\iint E_\text{beam}(x,y) \, E_\text{mode}^*(x,y) \, dx \, dy\right|^2}{\iint |E_\text{beam}|^2 \, dx \, dy \cdot \iint |E_\text{mode}|^2 \, dx \, dy}$$

This is the normalized squared inner product of the beam and mode profiles — mathematically the cosine-squared of the angle between the two field vectors in the infinite-dimensional function space $L^2(\mathbb{R}^2)$.

The coupling efficiency $\eta \in [0, 1]$: $\eta = 1$ for perfect mode matching (beam = mode); $\eta < 1$ for any mismatch in size, shape, or position.

**Interpretation**: The coupling efficiency formula is a consequence of the orthogonality of guided modes in a waveguide. Each guided mode is an eigenfunction of the transverse wave equation with a distinct propagation constant $\beta$. The power in each mode is determined by the projection of the incoming field onto that mode. The single-mode coupling efficiency is the squared projection amplitude onto the fundamental mode.

## Gaussian Beam to Gaussian Mode (Fiber Coupling)

For a Gaussian beam with waist $w_\text{beam}$ and a waveguide/fiber with a Gaussian mode profile of radius $w_\text{mode}$ (both centered on-axis, no phase mismatch):

$$\iint E_\text{beam} E_\text{mode}^* \, dA = \int_0^\infty e^{-r^2/w_\text{beam}^2} e^{-r^2/w_\text{mode}^2} 2\pi r \, dr = \pi \frac{w_\text{beam}^2 w_\text{mode}^2}{w_\text{beam}^2 + w_\text{mode}^2}$$

After normalization:

$$\eta = \frac{4w_\text{beam}^2 w_\text{mode}^2}{(w_\text{beam}^2 + w_\text{mode}^2)^2} = \left(\frac{2w_\text{beam}w_\text{mode}}{w_\text{beam}^2 + w_\text{mode}^2}\right)^2$$

This is maximized when $w_\text{beam} = w_\text{mode}$: $\eta_\text{max} = 1$. The AM-GM inequality guarantees this: $(w_\text{beam}^2 + w_\text{mode}^2)/2 \geq w_\text{beam}w_\text{mode}$, with equality iff $w_\text{beam} = w_\text{mode}$.

**Missize penalty**: If $w_\text{beam}/w_\text{mode} = 2$ (beam twice as large as mode): $\eta = 4(2)(1)/(4+1)^2 = 8/25 = 0.64$, or $-1.9$ dB coupling loss. If ratio = 3: $\eta = 4(3)/(9+1)^2 = 12/100 = 0.12$, or $-9.3$ dB loss. **Size matching is critical.**

For a standard single-mode fiber (mode field diameter MFD = $2w_\text{mode} = 10.4$ μm at 1550 nm) and a Gaussian beam focused to $w_\text{beam} = 5.2$ μm ($1/e^2$ radius): perfect matching, $\eta = 1$. Any larger or smaller focused spot reduces coupling.

## Fiber-to-Silicon Chip Coupling

The silicon waveguide mode has a $1/e^2$ half-width of approximately $w_x \approx 0.2$ μm horizontal (confined by the waveguide width 0.45 μm) and $w_y \approx 0.1$ μm vertical (confined by the waveguide height 0.22 μm). This is 25–50 times smaller than the fiber mode ($w_\text{mode} = 5.2$ μm).

Direct coupling from fiber to silicon waveguide has coupling efficiency:

$$\eta_\text{direct} \approx \left(\frac{2 \times 5.2 \times 0.2}{5.2^2 + 0.2^2}\right)^2 \approx \left(\frac{2.08}{27.1}\right)^2 \approx 0.006 \approx -22 \text{ dB}$$

This is catastrophically bad — over 99% of the power is lost. Mode converters are essential.

### Inverse Taper Coupler

The most common approach: the silicon waveguide tapers down from the standard 450 nm width to a tip of $\sim 80$–200 nm over a length of $\sim 300$ μm. The taper is clad in a material with a larger mode (SiO₂, polymer, or SU-8).

At the narrow tip, the silicon wire's effective index $n_\text{eff}$ approaches the cladding index (light is delocalized from the silicon into the surrounding material), expanding the mode field diameter to match the cladding mode ($\sim 3$–5 μm for polymer cladding). A lensed fiber then couples efficiently to this expanded mode.

State-of-the-art inverse taper couplers achieve coupling efficiency $> 70$% (loss $< 1.5$ dB) per facet [1].

### Grating Coupler

An alternative: etch a diffraction grating into the silicon waveguide surface to couple light vertically from a standard optical fiber placed above the chip. The grating period $\Lambda$ is designed so the Bragg condition $n_\text{eff} \lambda/\Lambda = \sin\theta_c$ is satisfied at the desired coupling angle $\theta_c$ (typically $8$–$12°$ from vertical for 1550 nm silicon gratings).

Advantages: alignment is done in-plane (xy) rather than end-facet (z), allowing wafer-scale automated testing. Disadvantages: inherently narrowband (coupling efficiency drops off away from the design wavelength), typically limited to one polarization, and require alignment in three dimensions.

State-of-the-art grating couplers: efficiency $> 85$% with optimized apodized gratings and back reflectors [2].

## General Mode Overlap Considerations

When the beam is not perfectly centered (lateral offset $\delta x$) or tilted (angular offset $\delta\theta$) relative to the waveguide mode:

**Lateral offset**: The overlap decreases as $e^{-2\delta x^2/(w_\text{beam}^2 + w_\text{mode}^2)}$. For $w_\text{mode} = 5$ μm, a 1 μm offset reduces efficiency by $e^{-2(1)^2/50} \approx 0.96$ — only 4% loss. Alignment tolerance is on the order of the mode field radius.

**Angular offset**: The coupling efficiency for angular misalignment $\delta\theta$ decreases as $e^{-(\delta\theta/\theta_\text{div})^2/2}$, where $\theta_\text{div} = \lambda/(\pi w_\text{mode})$ is the beam divergence. For 5 μm mode: $\theta_\text{div} \approx 0.1$ rad, so $1°$ angular offset reduces efficiency by $< 1$%.

**Elliptical vs. circular modes**: Silicon waveguide modes are highly elliptical ($w_x \gg w_y$ for large aspect ratio waveguides). A circularly symmetric fiber mode overlaps with the elliptical waveguide mode with reduced efficiency. Spot-size converters that match both axes improve coupling.

## Practical Coupling for Photonic Computing Systems

A photonic computing chip operating at 1550 nm with 16 input/output channels requires 32 efficient fiber-to-chip couplings. At 1 dB loss per coupler:
- 32 couplers: 32 dB total input/output loss
- Only 1/1585 of the input laser power reaches the computation; only 1/1585 of the output power reaches detectors.

This is a severe penalty. State-of-the-art grating couplers at 0.5 dB (88% efficiency):
- 32 couplers: 16 dB total = 97.5% power loss to coupling alone.

This is why coupling efficiency is one of the most intensely optimized aspects of photonic computing chip design. It directly determines the total system power budget and the required laser power.

## Summary

- Coupling efficiency: mode overlap integral $\eta = |\iint E_\text{beam} E_\text{mode}^* dA|^2 / (\|E_\text{beam}\|^2 \|E_\text{mode}\|^2)$.
- Gaussian-to-Gaussian: $\eta = [2w_1 w_2/(w_1^2+w_2^2)]^2$, maximized at $w_1 = w_2$.
- Fiber (MFD 10 μm) to silicon waveguide (width 0.45 μm): direct coupling $\approx -22$ dB.
- Inverse tapers and grating couplers reduce this to $-0.5$ to $-1.5$ dB.
- Coupling efficiency directly constrains photonic computing power budgets.

---

*References*

[1] Almeida, V.R., Panepucci, R.R., & Lipson, M. (2003). Nanotaper for compact mode conversion. *Optics Letters*, 28(15), 1302–1304. [DOI: 10.1364/OL.28.001302] [Introduction of the inverse taper for fiber-to-chip coupling.]

[2] Marchetti, R. et al. (2019). Coupling strategies for silicon photonics integrated chips. *Photonics Research*, 7(2), 201–239. [DOI: 10.1364/PRJ.7.000201]
