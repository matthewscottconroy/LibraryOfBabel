# 7.1.1 Why Silicon: The Materials Case

## The Silicon Photonics Advantage

Silicon became the dominant platform for photonic integration for a set of reasons that are individually modest but collectively decisive:

**1. Transparency at telecom wavelengths**: Silicon ($E_g = 1.12$ eV) is transparent at 1310 nm and 1550 nm — above the two-photon absorption edge (~2200 nm) but below the indirect bandgap onset (~1100 nm). Waveguide propagation loss in silicon is limited by fabrication roughness (~1–3 dB/cm for standard SOI ridge waveguides) rather than material absorption.

**2. High refractive index**: $n_{Si} = 3.48$ at 1550 nm, compared to $n_{SiO_2} = 1.44$. This large index contrast enables:
- Tight mode confinement ($A_{eff} \approx 0.14$ μm²) → small device footprint
- Compact bends (radius ~2–5 μm) → dense circuit integration
- Strong electro-optic modulation efficiency via plasma dispersion

**3. Mature fabrication infrastructure**: Silicon-on-insulator (SOI) wafers and 193 nm deep-UV lithography were already in use for CMOS microelectronics. The entire photolithography, deposition, and etch process was available; only the device designs needed to be invented.

**4. CMOS co-integration**: Silicon photonics can be fabricated in standard CMOS fabs (with modifications), enabling monolithic integration of photonic devices with transistor circuits. The IBM, Intel, and GlobalFoundries silicon photonic processes co-integrate photonics with advanced electronics.

## Silicon's Disadvantages

Silicon cannot do everything. Its key limitations:

| Limitation | Physical cause | Engineering solution |
|---|---|---|
| No lasing | Indirect bandgap | III-V heterogeneous integration |
| Weak electro-optic effect | No Pockels effect (centrosymmetric) | Plasma dispersion; use LiNbO₃ instead |
| Two-photon absorption at high power | Below $E_g/2$ threshold | Keep power < 10 mW; use Si₃N₄ |
| High thermal sensitivity | Large $dn/dT$ | Active thermal tuning (TEC, resistors) |
| Poor IR detection (>1.1 μm) | Transparent beyond bandgap | Epitaxial Ge-on-Si detectors |

These limitations have driven the development of alternative platforms: Si₃N₄ (for high-power, low-loss applications), lithium niobate on insulator (LNOI, for high-speed electro-optic modulation), and heterogeneous III-V/Si (for on-chip lasing).

## Material Constants Used Throughout This Chapter

| Material | $n$ (1550 nm) | $n_g$ (1550 nm) | $dn/dT$ (K$^{-1}$) | Loss (typical) |
|---|---|---|---|---|
| Silicon | 3.478 | 4.24 | $1.87\times10^{-4}$ | 1–3 dB/cm (roughness) |
| SiO₂ | 1.444 | 1.47 | $9.7\times10^{-6}$ | < 0.1 dB/cm |
| Si₃N₄ | 1.997 | 2.05 | $2.4\times10^{-5}$ | 0.1–1 dB/m (ultra-low loss rings) |
| LiNbO₃ (ordinary) | 2.211 | 2.25 | ~$4\times10^{-5}$ | 0.1 dB/cm (LNOI waveguide) |
| Ge | 4.27 | 5.0 | $4.0\times10^{-4}$ | (absorbs at 1550 nm) |
| InP | 3.17 | 3.5 | $2.0\times10^{-4}$ | 1–5 dB/cm |
