# 6.2.4 Chromatic Dispersion and Dispersion Management

## Sources of Chromatic Dispersion

Chromatic dispersion — the wavelength dependence of the group velocity — has two contributions in a single-mode fiber:

1. **Material dispersion** $D_M$: the wavelength dependence of the refractive index of the silica glass, described by the Sellmeier equation. $D_M = 0$ at ~1270 nm for silica; negative (anomalous) above this wavelength.

2. **Waveguide dispersion** $D_W$: the wavelength dependence of the fraction of mode power in core vs. cladding, which changes the effective index even at fixed material indices. $D_W$ is negative (anomalous) for step-index fiber; its magnitude increases with tighter confinement.

$$D_{total} = D_M + D_W \quad \text{[ps/(nm·km)]}$$

For SMF-28: $D_M \approx 22$ ps/(nm·km) at 1550 nm, $D_W \approx -5$ ps/(nm·km), giving $D_{total} \approx 17$ ps/(nm·km) (anomalous — a positive $D$ by convention means that longer wavelengths travel faster, which is the anomalous dispersion regime).

## Dispersion Impact on WDM Transmission

Chromatic dispersion limits the transmission of WDM channels in two ways:

1. **Pulse broadening**: A pulse of spectral width $\delta\lambda$ broadens by $\Delta t = D \cdot L \cdot \delta\lambda$ after propagating distance $L$. For a 100 Gbps OOK signal with $\delta\lambda \approx 0.8$ nm: $\Delta t = 17 \times L \times 0.8$ ps = 13.6L ps/km. For unpenalized transmission, $\Delta t < T_{bit}/2 = 5$ ps → $L < 0.37$ km. This is why 100G NRZ links over SMF-28 require dispersion compensation.

2. **Four-wave mixing crosstalk**: At the zero-dispersion wavelength, FWM efficiency peaks because all WDM channels experience nearly the same group velocity (near-zero $\Delta k$). This is why WDM systems avoid operating near $\lambda_{ZD}$, despite the seemingly favorable pulse broadening.

## Zero-Dispersion Wavelength and Dispersion-Shifted Fiber

The zero-dispersion wavelength $\lambda_{ZD}$ of SMF-28 is near 1310 nm. This made 1310 nm the original telecom window. When 1550 nm was adopted for its lower loss, the high dispersion at 1550 nm required new fiber designs.

**Dispersion-shifted fiber (DSF)**: By modifying the core profile (triangular or segmented), the waveguide dispersion $D_W$ is made more negative, shifting $\lambda_{ZD}$ from 1310 nm to 1550 nm. But single-channel DSF at 1550 nm has $D \approx 0$, making FWM a problem for WDM. This led to:

**Non-zero dispersion-shifted fiber (NZDSF, ITU-T G.655)**: Designed with $D = \pm2$–4 ps/(nm·km) at 1550 nm — enough dispersion to suppress FWM (channels dephase over < 1 km) while small enough that dispersion compensation is manageable. NZDSF is the standard for long-haul submarine and terrestrial DWDM systems.

## Dispersion Compensation

For installed SMF-28 (most of the world's fiber plant), dispersion is compensated by:

1. **Dispersion-compensating fiber (DCF)**: A specialty fiber with $D \approx -100$ ps/(nm·km) (highly normal dispersion from tight confinement). A 10 km span of DCF compensates 100 km of SMF-28. DCF is lossy (~0.5 dB/km), so EDFA amplification is needed after compensation.

2. **Chirped fiber Bragg gratings**: Reflect different wavelengths at different positions along the grating, introducing a time delay proportional to wavelength. Compact (< 1 m), low loss, but limited bandwidth.

3. **Electronic dispersion compensation (EDC)**: DSP in the coherent receiver corrects for dispersion digitally. This has become the dominant technique for long-haul systems since 2010, enabling single-wavelength symbol rates of 64–128 GBaud without physical dispersion management.

## Relevance to Photonic Computing

For photonic computing systems using off-chip optical interconnects (e.g., a photonic accelerator communicating over optical fiber to a memory bank), the relevant dispersion is over distances of meters to kilometers:

- **Rack-to-rack (< 100 m)**: Negligible dispersion penalty, even at 400 Gbps
- **Building-to-building (1–10 km)**: Dispersion compensation or coherent detection needed for > 100 Gbps per channel
- **Data center to data center (> 10 km)**: Full coherent optical networking with EDC

For on-chip waveguides (distances < 1 cm), propagation dispersion is negligible. On-chip dispersion engineering matters only for nonlinear optics (phase matching, soliton generation) — not for signal propagation.
