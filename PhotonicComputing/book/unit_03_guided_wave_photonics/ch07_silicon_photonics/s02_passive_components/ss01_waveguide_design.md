# 7.2.1 Waveguide Design: Loss, Bends, and Tapers

## Propagation Loss

Silicon waveguide propagation loss has three contributions at 1550 nm:

1. **Sidewall roughness scattering**: The dominant loss mechanism. Lithographic edge roughness (root-mean-square ~3–5 nm for deep-UV lithography) scatters light out of the guided mode. For a 450 × 220 nm strip waveguide: ~1–3 dB/cm. For a 500 nm × 220 nm rib waveguide with 90 nm slab: ~0.3–1 dB/cm.

2. **Two-photon absorption (TPA)**: $\beta_{TPA} \approx 5 \times 10^{-12}$ m/W. For $P = 10$ mW in a 450 nm strip ($A_{eff} = 0.14$ μm²): intensity $I = P/A_{eff} = 71$ MW/cm². TPA loss $\alpha_{TPA} = \beta_{TPA} I = 0.36$ cm$^{-1}$ = 1.5 dB/cm. This limits silicon photonic circuits to <10 mW per waveguide for TPA not to dominate over roughness loss.

3. **Free-carrier absorption**: TPA generates electron-hole pairs that absorb light. In undoped waveguides, the carrier density due to TPA is $\Delta N = \beta_{TPA} I^2 \tau/(2\hbar\omega)$ where $\tau$ is carrier lifetime (~1 ns in undoped Si). For 10 mW: $\Delta N \approx 10^{14}$ cm$^{-3}$ → $\Delta\alpha_{FC} \approx 8.5 \times 10^{-18} \Delta N \approx 8.5 \times 10^{-4}$ cm$^{-1}$ — negligible. But for 100 mW: $\Delta N \approx 10^{16}$ cm$^{-3}$ → $\Delta\alpha_{FC} \approx 0.085$ cm$^{-1}$ = 0.37 dB/cm — significant.

**Design rule for loss**: Keep power < 5–10 mW per waveguide to stay roughness-limited. Above this, TPA and free-carrier absorption add significant excess loss.

## Bends

Waveguide bends introduce radiation loss when the mode "leaks" around the outside of the bend. The bend loss decreases exponentially with bend radius; for 450 × 220 nm Si strip waveguides:

- $R = 10$ μm: ~0.01 dB/90°
- $R = 5$ μm: ~0.05 dB/90°
- $R = 2$ μm: ~0.5 dB/90°
- $R = 1$ μm: ~5 dB/90°

The minimum acceptable bend radius for low loss in standard Si strip waveguides is approximately 5 μm — enabling very tight routing and compact photonic circuits. Compare to standard SMF-28 fiber with minimum bend radius ~30 mm (30,000 μm) — a factor of 6000 difference in bend radius, reflecting the factor of $6000 \times$ higher index contrast.

## Tapers

Waveguide tapers are used to:
1. **Convert between waveguide widths**: e.g., transition from 220 nm (near-cutoff, expanded mode) to 450 nm (well-confined mode) for coupling efficiency
2. **Reduce reflection**: Abrupt width changes cause reflections; tapers adiabatically transfer mode shape
3. **Fiber coupling** (inverse taper): Taper to <100 nm width to expand mode for SMF coupling

**Adiabatic taper**: A taper is adiabatic (no mode conversion) when the rate of width change is small compared to the beat length between modes: $d\omega/dz \ll \pi/L_b$ where $\omega(z)$ is the local width and $L_b = \pi/(|\beta_1 - \beta_2|)$ is the local beat length between modes 1 and 2. For a taper from 450 nm to 1 μm over length $L$: the beat length between TE₀ and TE₁ at 700 nm width is ~100 μm. An adiabatic taper requires $L \gg 100$ μm — a few hundred μm is standard.

**Linear vs. parabolic tapers**: Parabolic (exponential width profile) tapers minimize the taper length for a given maximum conversion loss. For a 450 → 150 nm inverse taper over 200 μm, typical insertion loss is < 0.5 dB with fabrication-grade lithography.
