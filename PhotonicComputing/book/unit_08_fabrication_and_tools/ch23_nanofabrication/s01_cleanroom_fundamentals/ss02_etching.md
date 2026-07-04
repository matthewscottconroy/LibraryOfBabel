# 23.1.2 Etching: Transferring the Pattern into Silicon

Lithography patterns resist; etching transfers that pattern into the functional material beneath. For photonics the etch is arguably the single most consequential process step: it defines the waveguide's physical cross-section, and its imperfections — sidewall angle, sidewall roughness, depth error — become effective-index errors and propagation loss.

## Wet Etching: Isotropic and Crystallographic

**Wet etches** dissolve material chemically in solution. Most are *isotropic*: they etch equally in all directions, undercutting the mask by roughly the etch depth. Hydrofluoric acid (HF, usually buffered as BOE) etches SiO₂ but barely touches silicon — invaluable for stripping oxide, opening windows in cladding, and for *release etches* that dissolve the BOX beneath a waveguide to create the suspended structures used in MEMS phase shifters (Chapter 7) and thermally isolated heaters. Isotropy makes wet etching useless for defining waveguides: a 220 nm deep wet etch would erase a 100 nm coupler gap entirely.

The exception is **anisotropic crystallographic etching**: KOH or TMAH etches silicon's ⟨100⟩ planes tens to hundreds of times faster than ⟨111⟩ planes, producing atomically smooth facets at the crystallographically determined angle of 54.74°. This is a MEMS staple — and photonics borrows it for V-grooves that passively align optical fibers to chips (Section 23.4.1).

## Dry Etching: RIE and ICP-RIE

Waveguides are defined by **reactive-ion etching (RIE)**: the wafer sits on an RF-driven electrode in a low-pressure plasma, which supplies two agents simultaneously — *chemically reactive radicals* (e.g., fluorine or chlorine species) that form volatile products with the substrate, and *energetic ions* accelerated across the plasma sheath by the electrode's DC self-bias, which strike the surface vertically. The chemistry provides selectivity and etch rate; the directional ion bombardment provides anisotropy, by damaging and clearing horizontal surfaces while sidewalls (which ions graze) etch slowly. The result: vertical sidewalls, typically 88–90° in a well-tuned silicon photonic etch.

**Inductively coupled plasma (ICP-)RIE** adds a second RF coil that sustains a dense plasma independently of the wafer bias, decoupling the two knobs that plain RIE ties together: plasma density (etch rate) and ion energy (anisotropy vs. damage). ICP-RIE is the standard for silicon photonics.

Common chemistries:

- **SF₆/C₄F₈** ("pseudo-Bosch" when mixed continuously): SF₆ supplies fluorine to etch Si; C₄F₈ deposits a fluoropolymer that passivates sidewalls. The time-multiplexed version (the true **Bosch process**, alternating etch and passivation cycles) achieves very deep etches but leaves periodic sidewall "scallops" tens of nm deep — acceptable for MEMS, ruinous for low-loss waveguides. Photonic etches use the mixed-mode variant for smooth walls.
- **HBr/Cl₂**: the CMOS gate-etch chemistry, used in 193 nm foundry photonic processes; slower, but exquisitely controllable and smooth.
- **CHF₃/CF₄ or C₄F₈-based** oxide etches for SiO₂ and Si₃N₄.

Key etch metrics: **anisotropy** (vertical/lateral rate ratio), **selectivity** (substrate/mask rate ratio — determines how thick the resist or hard mask must be), **uniformity** (center-to-edge rate variation across the wafer, typically a few percent), and **aspect-ratio-dependent etching** (narrow gaps etch slower — a 100 nm coupler gap may etch shallower than an open field, a bias the PDK's models must capture).

## Partial Etches and Endpoint Control

A silicon photonic process is not one etch but several. A typical foundry flow offers a **full etch** (220 nm, defining strip waveguides), one or two **partial etches** (e.g., 70 nm and 130 nm, defining grating couplers and rib waveguides whose thin slab allows electrical contact to modulator junctions), and deep etches for facets and trenches. Full etches can use the BOX as a natural **etch stop** — the chemistry is selective against SiO₂, so the etch self-terminates. Partial etches have no stop layer: they are *timed*, monitored by laser interferometry or optical emission spectroscopy, and their ±few-nm depth control is one of the largest sources of device variability. A grating coupler's center wavelength, for instance, shifts by roughly a nanometer per nanometer of etch-depth error.

## Sidewall Roughness: Where Waveguide Loss Comes From

Crystalline silicon is essentially transparent at 1550 nm; the 1–3 dB/cm loss of a real strip waveguide is dominated by **sidewall roughness scattering**. The roughness (RMS amplitude $\sigma \approx 1\text{–}3$ nm, correlation length ~50 nm) originates in resist line-edge roughness and in the stochastic chemistry of the etch, and it couples the guided mode to radiation modes. In the classic Payne–Lacey scaling, scattering loss grows as roughly

$$\alpha_{scat} \propto \sigma^2 \frac{(n_{core}^2 - n_{clad}^2)^2}{d^3} \cdot E_{sidewall}^2$$

— quadratic in the roughness amplitude, cubic in inverse core dimension $d$, and proportional to the field intensity at the sidewall. This explains the empirical hierarchy: high-confinement 450 nm strips (2–3 dB/cm at 193 nm lithography) lose more than wide multimode strips (<0.5 dB/cm, mode pulled away from the walls) and far more than weakly confining Si₃N₄ ($\sigma$ matters less when index contrast is small; <0.1 dB/cm routinely, <0.01 dB/cm with the Damascene process of Chapter 7).

**Worked example — why 1 dB matters.** A photonic accelerator routes light through 3 cm of on-chip waveguide. At 2 dB/cm the optical power falls by 6 dB — a factor of 4 — before detection; halving the roughness amplitude ($\sigma: 2\to1$ nm) cuts scattering loss ~4× to 0.5 dB/cm, recovering 4.5 dB of link budget, which is worth more than a bit of resolution at the output ADC (Unit IX). Loss reduction is fabrication work, not design work.

Post-etch smoothing can help where the process allows: **thermal oxidation** consumes a few nm of the roughest silicon and can be stripped or left as cladding; **hydrogen annealing** at ~1000 °C lets surface silicon migrate and reflow smooth. Both must happen before any metal or germanium is present (thermal budget, Section 23.1.3).

## Hard Masks and the Etch Stack

Thin DUV resists (~100–300 nm) cannot survive a 220 nm silicon etch with margin, so production processes transfer the resist pattern first into a **hard mask** (SiO₂ or Si₃N₄, occasionally metal), then etch silicon with high selectivity against the hard mask. Each transfer step adds its own CD bias (systematic widening/narrowing) and roughness evolution; the PDK's "as-fabricated" waveguide width the simulator should use is the drawn width plus the *net* bias of this whole stack — one more reason to trust the foundry's measured compact models over your own idealized geometry.
