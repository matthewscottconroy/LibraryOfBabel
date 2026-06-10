# 9.3.1 DWDM Channel Plans and the ITU Grid

## The ITU Frequency Grid

Dense WDM (DWDM) systems use a standardized frequency grid established by the ITU-T (International Telecommunication Union) to ensure interoperability between equipment from different vendors. The ITU-T G.694.1 standard [1] defines the following channel spacings:

- **12.5 GHz**: Ultra-dense, used in emerging high-capacity systems
- **25 GHz**: Becoming common with 100 GBaud transponders
- **50 GHz**: Standard for most current long-haul DWDM
- **100 GHz**: Legacy systems; now considered coarse DWDM (CWDM) scale

The grid is anchored at 193.1 THz (1552.52 nm), which lies in the center of the C-band (conventional band, 1530–1565 nm, corresponding to 191.6–196.0 THz).

Channel count for the C-band:
- At 50 GHz spacing: 96 channels (1530–1565 nm)
- At 25 GHz spacing: 192 channels
- At 12.5 GHz spacing: 384 channels

The L-band (long-wavelength band, 1565–1625 nm, ~6 THz) provides additional capacity. C+L systems with 96+96 = 192 channels at 50 GHz have been commercially deployed, providing ~96 × 400 Gbps = 38.4 Tbps per fiber pair.

## Band Architecture

The optical amplification bands determine the usable spectral window:

| Band | Wavelength range | Bandwidth | Amplifier |
|------|-----------------|-----------|-----------|
| O-band | 1260–1360 nm | 100 nm | SOA, BDFA |
| E-band | 1360–1460 nm | 100 nm | Difficult |
| S-band | 1460–1530 nm | 70 nm | TDFA |
| C-band | 1530–1565 nm | 35 nm | EDFA |
| L-band | 1565–1625 nm | 60 nm | EDFA |
| U-band | 1625–1675 nm | 50 nm | Research |

The C-band dominates because EDFA gain is highest (25–30 dB with ~5 dB noise figure) and most reliable there. Thulium-doped fiber amplifiers (TDFAs) cover the S-band; wideband EDFAs and Raman amplifiers can extend coverage. Research into amplification of the O, E, and S bands is driven by the desire to exploit the full fiber bandwidth.

## The Flex-Grid

The rigid ITU grid (fixed spacing) is being replaced by a **flex-grid** architecture where channel bandwidths can be allocated in multiples of 12.5 GHz slots. This allows bandwidth to be matched to the channel's baud rate (e.g., a 100 GBaud channel needs ~150 GHz of guard-band-included bandwidth, which doesn't fit neatly in a 100 GHz slot but fits in 3 × 50 GHz flex-slots).

ITU-T G.694.1 (2020 revision) standardizes the flex-grid. Optical network controllers implementing software-defined networking (SDN) can dynamically allocate flex-grid slots based on real-time traffic demand, a capability called **elastic optical networking** [2].

---

## References

[1] ITU-T Recommendation G.694.1 (2020). *Spectral grids for WDM applications: DWDM frequency grid*. [The standard defining the DWDM channel plans.]

[2] Jinno, M., Takara, H., Kozicki, B., Tsukishima, Y., Sone, Y., & Matsuoka, S. (2009). "Spectrum-efficient and scalable elastic optical path network: Architecture, benefits, and enabling technologies." *IEEE Communications Magazine*, 47(11), 66–73. [The elastic optical networking concept.]
