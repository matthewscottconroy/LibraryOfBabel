# 9.2.1 Direct-Detection Formats: OOK and PAM4

## On-Off Keying

OOK is the oldest and simplest optical modulation format. The transmitter switches the laser on for a "1" bit and off (or to low power) for a "0" bit. The receiver uses a photodetector to measure intensity; a threshold decision determines the bit.

The minimum energy for reliable OOK is set by the receiver sensitivity — the minimum number of photons per bit to achieve a target bit error rate (BER). For an ideal photodetector with no thermal noise (shot-noise limited):

$$\text{BER}_{\text{OOK}} = \frac{1}{2}e^{-\bar{n}_1/2}$$

where $\bar{n}_1$ is the mean photon count for a "1" bit (assuming "0" bits have zero photons). For BER = $10^{-9}$: $\bar{n}_1 = 2\ln(2 \times 10^9) \approx 43$ photons/bit. This is the **shot-noise limit** for OOK: at minimum, 43 photons/bit are needed for BER = $10^{-9}$.

In practice, thermal noise from the TIA dominates over shot noise at power levels above ~50 μW (as derived in Section 5.3.3), and the receiver sensitivity is typically 100–300 photons/bit for commercial OOK systems.

**OOK spectral efficiency**: 1 bit per symbol per polarization; typical symbol rates of 10–25 GBaud give 10–25 Gbps per channel. The spectral efficiency is 1 bit/s/Hz (Nyquist-limited).

**Advantages**: Simple transmitter (direct laser modulation or simple EAM), simple direct-detection receiver, tolerant of chromatic dispersion (envelope detection is used).

**Disadvantages**: Requires large OSNR penalty compared to coherent formats for the same BER; spectral efficiency limited to 1 bit/s/Hz; poor noise tolerance in cascaded EDFA systems.

OOK dominated optical networks until approximately 2010; it remains the standard for very short reach (<2 km, e.g., inside data centers) where simplicity and cost dominate.

## PAM4

PAM4 (4-level pulse amplitude modulation) extends OOK by using four amplitude levels (0, 1, 2, 3) rather than two, encoding 2 bits per symbol. For the same symbol rate $B_s$ GBaud, PAM4 carries twice the bit rate of OOK.

The four amplitude levels are (in optical power): $P_0 = 0$, $P_1 = P/3$, $P_2 = 2P/3$, $P_3 = P$ (approximately). The receiver applies a 3-threshold decision circuit to determine which of the four levels was transmitted.

The SNR penalty of PAM4 relative to OOK (at the same BER and symbol rate):

$$\text{SNR penalty} = 10\log_{10}\left(\frac{\text{SNR required for PAM4}}{\text{SNR required for OOK}}\right) \approx 4.6 \text{ dB}$$

This arises because PAM4 has a 3× smaller eye opening between levels compared to the full swing of OOK, requiring 3× higher SNR for the same BER — a penalty of $\approx 10\log_{10}(3^2) = 9.5$ dB in SNR. However, operating at half the symbol rate for the same bit rate (since 2 bits/symbol vs. 1 bit/symbol) reduces the noise bandwidth by 2×, recovering 3 dB. Net penalty: ~6.5 dB in SNR compared to OOK at the same bit rate.

**PAM4 in data centers**: PAM4 at 56 GBaud is the dominant format for 400G data center interconnects (100G per lane × 4 lanes, or 200G per lane × 2 lanes). IEEE 802.3bs (400GBASE-DR4, -FR4, -LR4) and OIF CEI-56G standards specify PAM4 at 50–56 GBaud with direct detection on single-mode or multimode fiber.

**State-of-art**: 800G per fiber pair using PAM4 at 112 GBaud (2 lanes × 400G) is in standardization as of 2024. This requires ~112 GHz bandwidth modulators and photodetectors — pushing the limits of current silicon photonics and Ge-on-Si detector technology.

---

## References

[1] Winzer, P.J. & Essiambre, R.J. (2006). "Advanced modulation formats for high-capacity optical transport networks." *Journal of Lightwave Technology*, 24(12), 4711–4728. [Comprehensive comparison of OOK, duobinary, and advanced modulation formats with SNR analysis.]

[2] IEEE Std 802.3bs-2017. *Amendment to IEEE Standard for Ethernet - Media Access Control Parameters, Physical Layers, and Management Parameters for 200 Gb/s and 400 Gb/s Operation.* [The standard that established PAM4 as the dominant 400G format.]
