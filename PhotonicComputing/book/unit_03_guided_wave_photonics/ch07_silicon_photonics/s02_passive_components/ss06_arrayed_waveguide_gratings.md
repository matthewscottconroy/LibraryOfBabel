# 7.2.6 Arrayed Waveguide Gratings

## Operation Principle

An arrayed waveguide grating (AWG) is a waveguide-based wavelength (de)multiplexer that routes different wavelengths to different spatial output ports. It is the integrated photonic equivalent of a diffraction grating in free space.

**Structure**:
1. **Input waveguide** → **Input slab coupler (free propagation region, FPR)**: Light from the input waveguide fans out in the FPR, illuminating all waveguides of the array
2. **Arrayed waveguides**: An array of $N$ channel waveguides, each one longer than the last by exactly $\Delta L$ (path length increment). Different wavelengths accumulate different phases in the array.
3. **Output FPR**: Light from the array interferes in the output slab, focusing to different positions depending on wavelength
4. **Output waveguides**: Placed at positions matching the focal spots of each wavelength

**Phase condition**: For wavelength $\lambda_m$ to focus on the $m$-th output waveguide, the path length difference $\Delta L$ must satisfy:

$$n_c \Delta L = m_0 \lambda_0 \quad (\text{central wavelength conditions})$$

$$\Delta\lambda_{ch} = \frac{\lambda_0 n_c \Delta L}{n_g \Delta L N_{ch}} = \frac{\lambda_0}{m_0 N_{ch}}$$

where $m_0$ is the diffraction order and $N_{ch}$ is the number of channels.

## AWG Performance

| Parameter | Typical value | Notes |
|---|---|---|
| Channel spacing | 100 GHz (0.8 nm) or 200 GHz (1.6 nm) | ITU-T grid |
| Channel count | 8–64 | Limited by chip area |
| Insertion loss | 2–5 dB | Including routing and FPR loss |
| Crosstalk | < −20 dB | Adjacent channel |
| Passband width | ~0.4 × channel spacing (3 dB) | Gaussian passband |
| Chip area | ~1–5 mm² | Dominated by FPR radii |
| Temperature sensitivity | ~0.011 nm/°C | Needs TEC for WDM |

## AWGs in Photonic Computing

AWGs serve as the wavelength router in WDM photonic matrix processors:

1. **Input demultiplexer**: An incoming WDM signal carrying 32 wavelength channels is demultiplexed by an AWG, routing each channel ($\lambda_1, \lambda_2, \ldots, \lambda_{32}$) to its own waveguide row of the matrix.

2. **Output multiplexer**: After each row is weighted and combined, the outputs at different wavelengths are recombined by a second AWG for further routing or readout.

The AWG's insertion loss (2–5 dB) directly adds to the photonic computing link budget. For a 32-channel system with input AWG + output AWG: ~4–10 dB overhead before any device losses. This is why some architectures prefer ring resonator-based wavelength routing (lower insertion loss per channel, ~0.5–1 dB) over AWGs for chip-scale photonic computing.
