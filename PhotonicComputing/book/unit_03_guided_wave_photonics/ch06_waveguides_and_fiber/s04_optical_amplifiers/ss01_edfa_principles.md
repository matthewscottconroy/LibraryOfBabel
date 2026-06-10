# 6.4.1 EDFA Principles: Gain, Bandwidth, and Saturation

## Erbium as a Gain Medium

Erbium-doped fiber amplifiers use the $^4$I$_{13/2} \to ^4$I$_{15/2}$ transition of Er³⁺ ions in silica glass. This transition:
- Wavelength: 1530–1565 nm (C-band) with a secondary L-band 1565–1625 nm
- Upper level lifetime: $\tau_2 \approx 10$ ms (long: good for energy storage, bad for dynamic gain)
- Three-level system: lower laser level is the ground state

**Pumping**: The most common pump wavelength is 980 nm ($^4$I$_{15/2} \to ^4$I$_{11/2}$, then fast non-radiative decay to $^4$I$_{13/2}$). The 980 nm pump is efficient and gives better noise figure than 1480 nm pumping. Typical pump power: 30–200 mW.

## Gain Spectrum and Bandwidth

The EDFA gain spectrum $g(\lambda)$ arises from the Stark-split manifolds of the $^4$I$_{13/2}$ level in the silica host. The gain spectrum is approximately flat from 1530–1560 nm with 15–20 nm bandwidth (3 dB), but with a sharp peak at ~1530 nm and a broader plateau at 1545–1560 nm.

**Gain flattening**: WDM systems need uniform gain across all channels. Standard EDFA gain spectrum is not flat; gain-flattening filters (GFF) — thin-film or long-period grating filters — are used to equalize it to < 0.5 dB variation over the C-band.

## Gain and Saturation

The small-signal gain is:

$$G_0 = e^{(\Gamma g(N_2 - N_1) - \alpha_i)L}$$

where $\Gamma$ is the mode-gain overlap, $g$ is the emission cross-section, and $L$ is the fiber length. Typical small-signal gain: 20–40 dB for a 4–10 m long EDF.

At high input power (or high total signal power in WDM), the gain saturates: stimulated emission depletes the inversion faster than the pump can replenish it. The saturation output power $P_{sat}$ is typically 10–20 dBm. In a WDM system with 80 channels, each at 0 dBm, total signal power is +19 dBm — the EDFA is strongly saturated, and per-channel gain is much less than small-signal gain.

**Gain dynamics**: The 10 ms upper level lifetime means that EDFA gain responds slowly to changes in input power (channel add/drop transients). Rapid gain changes (in < 10 ms) cause transient gain excursions that affect the gain of other channels — a known problem in dynamically reconfigurable optical networks and photonic computing systems with variable loads.
