# 7.1.3 Foundry Ecosystem and PDK Design Flow

## The Multi-Project Wafer Model

Silicon photonics has adopted the multi-project wafer (MPW) model from the semiconductor industry: multiple customers share a single wafer run, each occupying a small "tile" of the wafer. A typical MPW run produces chips of 3 × 3 mm to 20 × 20 mm, with quantities of 5–50 chips per design. Cost per chip in MPW: $500–$5000 depending on foundry and chip area.

This model has democratized photonic chip design: university research groups, startups, and small companies can access advanced fabrication (45 nm CMOS + photonics) without a $50M wafer run. The major photonic foundry MPW services:

| Foundry | Platform | Node | Notable for |
|---|---|---|---|
| IMEC (Belgium) | iSiPP50G | 200 mm, 180 nm | Broadest standard PDK, 50 Gbps PDs |
| IMEC | iSiPP25G | 200 mm, 130 nm | Standard research/prototyping |
| LETI (France) | PIC200 | 200 mm, 193 nm | Heterogeneous III-V |
| GlobalFoundries | Fotonix (45CLO) | 300 mm, 45 nm CMOS | Most advanced CMOS+photonics |
| AMF (Singapore) | 200 mm | 180 nm | Low-cost prototyping |
| Tower Jazz | PH18 | 200 mm, 180 nm | High-power, InP option |
| AIM Photonics | APF | 200 mm, 193 nm | US-based; DOD access |

## Process Design Kit (PDK)

A **process design kit (PDK)** is the collection of design rules, device models, and parameterized cell (pcell) layouts provided by a foundry for circuit designers. A silicon photonic PDK typically includes:

- **Design rule checks (DRC)**: Minimum feature sizes, spacing requirements, antenna rules
- **Passive components**: Waveguide pcells (straight, bend, taper), grating coupler, MMI, ring resonator templates
- **Active components**: p-n junction modulator, p-i-n phase shifter, Ge photodetector, heater
- **SPICE/VHDL-AMS models**: Electrical + optical simulation models for each device
- **Layout vs. schematic (LVS) rules**: Verify that the drawn layout matches the schematic

The design flow for a silicon photonic computing chip:
1. **System design**: Partition the computation into optical operations (matrix multiply, routing, amplification)
2. **Circuit schematic**: Specify MZIs, ring resonators, couplers from PDK cells
3. **Circuit simulation**: Verify transfer function using optical circuit simulator (Lumerical INTERCONNECT, Ansys Photonics)
4. **Layout design**: Place and route PDK cells, check DRC
5. **Component simulation**: FEM/FDTD simulation of critical components (couplers, tapers)
6. **Tape-out**: Submit GDSII layout to foundry
7. **Chip testing**: Probe station, fiber array, high-speed electronics

**Critical point for photonic computing**: Unlike digital CMOS where a gate either works or doesn't, analog photonic components have *continuous* performance that degrades with fabrication variations. A ring resonator designed for 1550.0 nm resonance may fabricate at 1550.3 nm due to ±5 nm width variation. All photonic computing architectures must include calibration and tuning strategies that compensate for this fabrication uncertainty.
