# Subsection 11.2.3: Optical Beamforming Networks

## Orientation

A phased-array antenna forms a directed beam by introducing precise time delays between antenna elements. The electronic approach to generating these delays is elegant but limited: electronic phase shifters operate at the carrier frequency, so they implement *phase* delay (constant across frequency), not *time* delay (constant across time). For wideband radar and communications, this causes "squint" — the beam direction changes with frequency, making it impossible to form a wide-bandwidth beam in a constant direction. Photonic true time delay (TTD) beamforming solves this problem with a technique that has no electronic equivalent.

---

## 11.2.3.1 The Squint Problem

### Phase Delay vs. True Time Delay

For a linear phased array with $N$ elements spaced by $d$, the beam points in the direction $\theta$ when element $n$ is fed with delay $\tau_n = n d \sin\theta / c$. This gives a phase shift $\phi_n = 2\pi f \tau_n$, which depends on frequency $f$.

**Phase shift implementation**: A variable phase shifter applies $\phi_n$ directly. The phase shift is frequency-independent (the same phase regardless of signal frequency within the phase shifter bandwidth). This works perfectly for a single-frequency (CW) signal.

**True time delay implementation**: A variable delay line applies $\tau_n$ directly. The phase shift $\phi_n = 2\pi f \tau_n$ is *correctly proportional to frequency* — exactly what is needed for the beam to point at the same angle for all frequencies.

**Squint**: If we use a phase shifter set for frequency $f_0$ (phase $\phi_n = 2\pi f_0 \tau_n$) to steer toward angle $\theta_0$, then at frequency $f \neq f_0$:

$$\theta(f) = \arcsin\left(\frac{c \phi_n}{2\pi f n d}\right) = \arcsin\left(\frac{f_0}{f}\sin\theta_0\right)$$

For a wideband signal with $\Delta f / f_0 = 10\%$ at $\theta_0 = 45°$:
$$\Delta\theta \approx \frac{f_0}{f_0} \sin 45° \cdot \frac{f_0}{f} \cdot \frac{\Delta f}{f_0} \approx 7°$$

A 7° beam squint across the signal bandwidth causes severe degradation of array gain and beam resolution. For wideband radar or 5G mmWave communications (where 1–10 GHz bandwidth at 28–77 GHz carrier is common), squint is disabling.

---

## 11.2.3.2 Photonic True Time Delay

### The Dispersive Fiber Approach

Light at different wavelengths travels at different speeds in dispersive fiber (from Section 7.1 and the analysis of fiber dispersion). The group delay difference between wavelengths $\lambda_1$ and $\lambda_2$ over fiber length $L$ is:

$$\Delta\tau = D \cdot L \cdot |\lambda_1 - \lambda_2|$$

where $D$ is the fiber dispersion coefficient (ps/(nm·km)). For standard SMF-28, $D \approx 17$ ps/(nm·km) at 1550 nm.

**TTD beamforming network**: 
1. A tunable wavelength laser (or a multi-wavelength source) generates optical carriers at different wavelengths.
2. An electro-optic modulator imprints the RF signal onto all wavelengths simultaneously.
3. The modulated signals propagate through a length $L_n$ of dispersive fiber (or a chirped fiber Bragg grating) for each array element $n$.
4. At the detector for element $n$, the RF signal arrives with a delay:

$$\tau_n = D \cdot L_n \cdot \Delta\lambda_n$$

By choosing $\Delta\lambda_n$ appropriately (varying the laser wavelength), the delay $\tau_n$ is tuned — covering the entire range of beam steering angles.

**Tuning the delay by wavelength**: For a wavelength change $\delta\lambda$:
$$\delta\tau_n = D \cdot L_n \cdot \delta\lambda$$

For $D = 17$ ps/nm/km, $L = 100$ m = 0.1 km, $\delta\lambda = 10$ nm:
$$\delta\tau = 17 \times 0.1 \times 10 = 17 \text{ ps}$$

For an antenna array with 10 cm element spacing, the required delay range for full $\pm 90°$ scanning:
$$\tau_{\max} = \frac{(N-1) d}{c} = \frac{(N-1) \times 0.1}{3\times10^8}$$

For $N = 8$ elements: $\tau_{\max} = 2.3$ ns — achievable with 13.5 nm wavelength tuning over 10 m of fiber.

**Wideband operation**: Since $\tau_n$ is independent of RF frequency (it depends on the optical group delay, which is determined by $\lambda$ and $D$, not by the RF signal), the delay is true time delay — the beam points at the same angle for all RF frequencies within the signal bandwidth.

### Integrated Photonic Beamforming

The fiber-based TTD approach achieves multi-GHz bandwidth [1] but is bulky (meters of fiber per element) and sensitive to temperature (fiber dispersion changes with temperature: $dD/dT \approx 0.05$ ps/(nm·km·K) for SMF-28, requiring $\pm 0.1°$C stability for 10 ps delay accuracy).

**On-chip TTD using optical waveguides**: Silicon photonic or Si₃N₄ waveguide delay lines can implement TTD on a chip:
- Delay range: $L_{\text{max}} / v_g \approx 10 \text{ cm} / (2\times10^8 \text{ m/s}) = 500$ ps (for a 10 cm waveguide)
- Switching between delays: ring resonators or MZI switches select different path lengths
- Demonstrated: 8-element 5-bit delay network at 60 GHz, chip area < 2 cm² [2]

**LISC (Liquid Crystal-based Integrated Photonic Steering Circuit)**: Emerging technology using liquid crystal phase shifters on Si₃N₄ waveguides to implement TTD directly via group index tuning. Liquid crystal has large $dn/dT$: group index tuning of $\Delta n_g \sim 0.1$ over 1 cm gives $\Delta\tau = 1 \text{ cm} / (c/(n+\Delta n)) - 1 \text{ cm}/(c/n) \approx 330$ ps. This is sufficient for a 60 GHz 8-element array.

---

## 11.2.3.3 Current Status and Applications

### Defense Applications

Photonic beamforming networks are deployed in US and European defense systems for:
- **AESA (Active Electronically Steered Array) radar**: TTD enables wideband radar waveforms (1–10 GHz instantaneous bandwidth) while maintaining boresight accuracy.
- **Electronic warfare**: Instantaneous wideband direction-of-arrival estimation using photonic delay networks.
- **Communications**: Wideband satellite communication uplinks at Ka-band (26.5–40 GHz) where squint is problematic.

Specific systems are classified, but the technology is deployed. Public demonstrations include the DARPA PNIMBLE program (photonic integrated circuits for wideband EW) and the European PSALMS program.

### Emerging Commercial Applications

**5G mmWave**: 5G base stations at 28 GHz and 39 GHz use phased array antennas. As 5G evolves to wider bandwidths (400–800 MHz channels, eventually 1 GHz), squint becomes limiting. Photonic TTD is a candidate solution for compact, low-power TTD beamforming ICs.

**Automotive radar**: 77 GHz automotive radar with 4–5 GHz bandwidth faces squint in large arrays ($> 64$ elements). Photonic TTD on a CMOS-photonics chip is a long-term possibility.

**Satellite communication**: LEO satellites (Starlink, OneWeb) use Ka-band phased arrays for high-speed downlinks. Satellite-grade photonic TTD chips would reduce mass and power vs. electronic alternatives.

---

## References

[1] Zmuda, H., Soref, R.A., Payson, P., Johns, S., & Toughlian, E.N. (1994). "Photonic beamformer for phased array antennas using a fiber grating prism." *IEEE Photonics Technology Letters*, 6(9), 1130–1132. [Early photonic TTD beamforming demonstration using fiber gratings.]

[2] Zhuang, L., et al. (2011). "Novel ring resonator-based integrated photonic beamformer for broadband phased array receive antennas." *Journal of Lightwave Technology*, 28(1), 19–31. [On-chip photonic beamformer using ring resonators for TTD; the 8-element demonstration cited here.]

[3] Capmany, J., & Novak, D. (2007). "Microwave photonics combines two worlds." *Nature Photonics*, 1, 319–330. [The review paper that covers TTD beamforming as one of the key applications of microwave photonics.]

[4] Marpaung, D., Yao, J., & Capmany, J. (2019). "Integrated microwave photonics." *Nature Photonics*, 13, 80–90. [2019 review covering the on-chip microwave photonics developments; covers integrated beamforming networks.]
