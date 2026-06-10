# 5.2.5 Germanium-on-Silicon Photodetectors

## The Silicon Photonics Integration Challenge

Silicon is transparent at 1310 nm and 1550 nm — it cannot detect the light it guides. The standard solution is to grow or deposit germanium on the silicon platform, exploiting Ge's absorption at both telecom wavelengths.

The challenge: germanium has a 4.2% lattice mismatch to silicon. Growing Ge directly on Si produces a high density of threading dislocations (crystal defects) that act as traps, increasing dark current and degrading quantum efficiency. This is not a minor engineering problem; it is a fundamental materials challenge that required decades to solve to the level required for photonic integrated circuit fabrication.

## Selective Area Epitaxial Growth

The solution used in most silicon photonic foundries is **selective area epitaxy (SAE)**: Ge is grown only in pre-patterned windows in an oxide layer, with a Si seed layer below. Key steps:

1. Grow a thin (~30 nm) low-temperature Ge buffer layer directly on Si (this accommodates most of the lattice mismatch through misfit dislocations)
2. Grow a thick (~300–500 nm) Ge body layer at higher temperature for better crystallinity
3. Anneal at 850°C to reduce threading dislocation density by dislocation glide
4. Ion implantation and silicidation to form low-resistance ohmic contacts

After this process, threading dislocation densities of $<10^7$ cm$^{-2}$ are achievable — low enough for useful device performance, though still much higher than bulk Ge on Ge substrates ($<10^3$ cm$^{-2}$).

## Waveguide-Integrated Ge-on-Si Photodetector

The standard Ge-on-Si detector in a silicon photonic platform is evanescently coupled to the Si waveguide. Light propagating in the Si waveguide couples into an adjacent Ge region through the evanescent field. The coupling length and Ge dimensions are chosen so that essentially all the optical power is absorbed within the Ge body before the end of the device.

**Absorption coefficient**: Ge at 1550 nm has $\alpha \approx 2500$ cm$^{-1}$ (pure tensile-strained Ge-on-Si may have $\alpha$ up to $5000$ cm$^{-1}$). For a 10 μm long Ge absorber, the single-pass absorption is $1 - e^{-\alpha L} = 1 - e^{-0.25} \approx 22\%$. For a 50 μm waveguide, absorption ~72%. Longer devices have higher QE but larger capacitance.

## State-of-the-Art Performance

| Parameter | State of art | Foundry PDK (typical) |
|---|---|---|
| Responsivity | 1.0–1.2 A/W at 1550 nm | 0.8–1.0 A/W |
| Bandwidth | > 60 GHz (research) | 40–50 GHz |
| Dark current | 10–100 nA | 50–200 nA |
| Operating voltage | −1 to −4 V | −2 V |
| Dimensions (waveguide PD) | 1×10–50 μm | 1×20 μm typical |
| NEP | < 20 pW/√Hz | |

High-speed demonstrations have reached 100 GHz bandwidth for Ge-on-Si detectors in traveling-wave geometries [1]. Foundry-available devices in GlobalFoundries 45SPCLO and IMEC iSiPP50G achieve 40–50 GHz bandwidth, sufficient for 50–100 Gbps optical receivers.

## Wavelength Range

Ge-on-Si detectors work well at 1310 nm ($\alpha \approx 5000$ cm$^{-1}$, slightly more absorptive) and at 1550 nm. At wavelengths beyond ~1600 nm, Ge absorption drops sharply toward the bandgap edge. Extended-wavelength operation (to 2 μm and beyond) requires GeSn alloys or other materials — an active research area for mid-infrared photonic sensing.

## Photonic Computing Integration

Ge-on-Si detectors are the standard output element of silicon photonic computing chips:
- Each row of a photonic matrix processor is summed by a Ge detector
- For a $64 \times 64$ MZI mesh, 64 Ge detectors read out 64 output values
- Each detector contributes shot noise, thermal noise, and dark current to the output

The dark current of a single Ge detector (~50–200 nA) produces shot noise $\langle i^2_{dark}\rangle = 2 e I_d B \approx 2 \times 1.6\times10^{-19} \times 100\times10^{-9} \times 10^{10} = 3.2 \times 10^{-19}$ A²/Hz for 10 GHz bandwidth — small compared to signal shot noise at signal powers > 1 μW, but non-negligible for low-power operation.

## Reference

[1] Chen, H., et al. (2016). "100-Gbps RZ data reception in 67-GHz Si-Ge waveguide photodetector." *Optics Express*, 24(2), 946–951.
