# 8.1.3 Two-Dimensional Photonic Crystal Slabs

## From 3D Theory to 2D Reality

A true 3D photonic crystal — a material periodic in all three directions — has a complete photonic bandgap: a range of frequencies for which light cannot propagate in any direction. While such materials exist (opal, inverse opal, woodpile structures), they are difficult to fabricate with the precision required for optical frequencies and have not been adopted for integrated photonic circuits.

The practical geometry for integrated photonics is the **photonic crystal slab**: a 2D periodic structure (typically a triangular lattice of air holes) in a thin slab of high-refractive-index material (silicon). The slab confines light vertically by total internal reflection (like a conventional slab waveguide), while the 2D periodicity provides in-plane photonic crystal effects.

The photonic crystal slab is not a true 2D photonic crystal — the vertical confinement is imperfect, and modes above the light line of the cladding can couple to radiation. But for modes below the light line (modes that cannot couple to freely propagating radiation in the cladding), the slab behaves like an effective 2D photonic crystal, and the concept of a photonic bandgap is approximately maintained.

## Geometry and Parameters

A standard silicon photonic crystal slab consists of:
- Silicon slab: thickness $t = 220$ nm (the SOI device layer)
- Triangular lattice of air holes: radius $r$, lattice constant $a$
- Cladding: SiO₂ below, air or SiO₂ above

The key design parameters are:
- **Filling fraction**: $f = \pi r^2/(\sqrt{3}a^2/2)$ — fraction of slab area occupied by air holes
- **Hole radius**: $r/a$ typically 0.25–0.45 for a bandgap
- **Slab thickness**: $t/a$ typically 0.5–0.7 for maximum gap-guidance interaction

For a triangular lattice of air holes with $r/a = 0.3$ and $t/a = 0.6$ in silicon ($n = 3.478$):
- Complete bandgap (all in-plane propagation directions) for TE-like modes: approximately $\omega a/c \in [0.25, 0.32]$
- For $a = 434$ nm: bandgap from 1355 nm to 1736 nm — covering the full telecom C and L bands

The larger the index contrast ($\varepsilon_{\text{high}}/\varepsilon_{\text{low}}$), the wider the available bandgap. Silicon/air with $\varepsilon = 12$ gives much wider gaps than Si₃N₄/air with $\varepsilon = 4$.

## Photonic Crystal Waveguides

A **line defect** — a row of missing holes in the photonic crystal — creates a waveguide. The surrounding crystal acts as a mirror (reflecting modes at frequencies within the bandgap), confining light to propagate along the defect.

The mode in a photonic crystal waveguide (PCW) is fundamentally different from the mode in a strip waveguide:
- In a strip waveguide, the mode is confined by total internal reflection at a single interface.
- In a PCW, the mode is confined by the photonic bandgap of the surrounding crystal on both sides, over a bandwidth determined by the bandgap width.

For a W1 waveguide (one missing row in a triangular lattice), the guided mode has a dispersion relation $\omega(k)$ that can be strongly engineered by adjusting hole positions, sizes, and the lattice constant.

### Flat-Band Waveguides for Slow Light

Near the edge of the Brillouin zone ($k = \pi/a$), the waveguide dispersion can be made extremely flat — meaning the group velocity $v_g = d\omega/dk$ approaches zero. This "slow light" regime is discussed in Section 8.1.4.

### Photonic Crystal Resonators: L-cavities and H-cavities

Removing a finite number of holes creates a **photonic crystal cavity** rather than a waveguide. The simplest cavities are:

- **L3 cavity**: Three missing holes in a row. Quality factor $Q \approx 6000$ (unoptimized), $V_{\text{mode}} \approx 0.7(\lambda/n)^3$.
- **H1 cavity**: One missing hole with surrounding holes displaced. $Q$ up to $10^5$, $V_{\text{mode}} \approx 0.5(\lambda/n)^3$.
- **Heterostructure cavity**: Varying the lattice constant locally to create a potential well for photons. $Q > 10^6$ [1].

The extraordinary quality factor combined with mode volumes of order $(\lambda/n)^3$ gives the **Purcell factor**:

$$F_P = \frac{3}{4\pi^2}\left(\frac{\lambda}{n}\right)^3 \frac{Q}{V_{\text{mode}}}$$

For a heterostructure cavity with $Q = 10^6$ and $V_{\text{mode}} = (\lambda/n)^3$: $F_P \approx 24{,}000$. This means the spontaneous emission rate of an atom in the cavity is enhanced 24,000-fold compared to free space — the most extreme Purcell enhancement achievable in a solid-state system.

For photonic computing, this extreme Purcell enhancement is relevant in several ways:
1. **Ultra-low threshold nanolasers**: A laser with $F_P = 10^4$ needs a cavity to compensate the reduced emission rate; even a single emitter can lase at microWatt drive powers.
2. **Fast single-photon emitters**: Enhanced emission rate means shorter photon lifetime, enabling GHz-rate single-photon generation for quantum photonics.
3. **Nonlinear optics at single-photon level**: The extreme field concentration in a small-volume cavity makes the nonlinear interaction between photons observable at very low photon numbers.

## Fabrication

Silicon photonic crystal slabs are fabricated by:
1. Starting with a standard SOI wafer (220-nm Si on 2-μm SiO₂)
2. Spin-coating electron-beam resist (typically ZEP or PMMA)
3. E-beam lithography to define the hole pattern with ~5 nm precision
4. Dry etching (ICP RIE) to etch the holes through the 220-nm Si layer
5. Optional HF undercut to suspend the membrane (removing the SiO₂ below the slab, creating a free-standing silicon slab for maximum vertical refractive index contrast)

The suspended membrane geometry achieves maximum bandgap width (silicon/air vs. silicon/oxide contrast) and minimum out-of-plane radiation loss. The unsuspended geometry (holes etched only partway through, or oxide undercladding retained) has lower $Q$ but better mechanical stability.

Commercial foundries (IMEC, CEA-LETI) now offer photonic crystal features as part of their silicon photonics process flows, though with resolution limits (minimum feature size ~100 nm) that restrict design flexibility compared to academic e-beam processes.

## Light Guidance in Defect Modes

The key figure of merit for a photonic crystal waveguide (as opposed to a cavity) is the group velocity and loss. For a W1 waveguide in silicon at 1550 nm, with appropriate lattice parameters:

- Bandwidth: ~20–50 nm (within the bandgap)
- Group velocity: $c/30$ to $c/100$ (engineered for slow light)
- Propagation loss: 3–30 dB/cm (higher than strip waveguide due to increased coupling to radiation from slow modes)

The propagation loss in slow-light PCW scales as $v_g^{-2}$: as the group velocity decreases, photons spend more time near the rough sidewalls of the holes, increasing scattering loss. This is the fundamental tradeoff in slow-light waveguides, discussed quantitatively in the next subsection.

---

## References

[1] Asano, T., Song, B.-S., & Noda, S. (2006). "Analysis of the experimental Q factors (~1 million) of photonic crystal nanocavities." *Optics Express*, 14(5), 1996–2002. [Photonic crystal heterostructure cavity with $Q > 10^6$.]

[2] Krauss, T.F., De La Rue, R.M., & Band, S. (1996). "Two-dimensional photonic-bandgap structures operating at near-infrared wavelengths." *Nature*, 383(6602), 699–702. [Early demonstration of 2D photonic crystal slab in silicon; waveguide and bandgap measurement.]

[3] McNab, S.J., Moll, N., & Vlasov, Y.A. (2003). "Ultra-low loss photonic integrated circuit with membrane-type photonic crystal waveguides." *Optics Express*, 11(22), 2927–2939. [Propagation loss measurement in photonic crystal waveguides.]
