# 4.4.2 Vertical-Cavity Surface-Emitting Lasers (VCSELs)

## Architecture: Turning the Laser Sideways

A conventional edge-emitting laser (DFB, FP) emits light from the cleaved edge of the chip, with the cavity oriented parallel to the substrate. A vertical-cavity surface-emitting laser (VCSEL) emits light perpendicular to the substrate, through a window in the top or bottom contact.

The VCSEL cavity is extremely short: the active region is one or a few quantum wells totaling $\sim$10 nm, with total cavity length $L \sim 1$–2 μm (about 1 wavelength). This short cavity has two consequences:

1. **Tiny mode volume**: The photon density per injected carrier is enormous → very low threshold current (sub-milliamp to microamp range)
2. **Huge longitudinal mode spacing**: $\Delta\nu_{FSR} = c/(2n_g L) \sim 50$–100 THz — far larger than the gain bandwidth of any semiconductor material. Only one longitudinal mode fits under the gain curve. **VCSELs are inherently single-longitudinal-mode without any grating**.

Since the optical cavity is so short (gain per pass is tiny), VCSELs use distributed Bragg reflector (DBR) mirrors with very high reflectivity: typically $R_1 = 99.5\%$ (back mirror) and $R_2 = 99\%$ (output coupler). These semiconductor DBR stacks consist of 20–30 pairs of alternating high-index and low-index quarter-wave layers (e.g., GaAs/AlAs), deposited epitaxially.

## Key VCSEL Parameters

| Parameter | Typical value | Notes |
|---|---|---|
| Threshold current | 0.5–2 mA | Much lower than edge-emitters |
| Operating wavelength | 850, 980, 1310, 1550 nm | Wavelength determined by cavity design |
| Wall-plug efficiency (max) | 40–60% | World record ~60% at 850 nm |
| Modulation bandwidth | 20–30 GHz | High-speed VCSELs at 850 nm |
| Output beam | Circular, low divergence | Easy fiber coupling |
| 2D array compatibility | Yes, natural | Wafer-scale fabrication |
| Single-mode output power | < 5 mW typically | Limited by multi-mode onset |

## Single-Mode vs. Multi-Mode VCSELs

Standard VCSELs with large aperture (> 5 μm diameter) emit in multiple transverse modes — the large aperture supports HG and LP modes beyond TEM$_{00}$. These multi-mode VCSELs are used for short-reach multimode fiber links (OM3/OM4 fiber, 300–400 m reach) at 850 nm, which dominate within-rack optical interconnects in data centers. Their bandwidth exceeds 25 Gbps per device; in 100 Gigabit Ethernet (100GBASE-SR4), four 850 nm VCSELs each at 25 Gbps provide 100 Gbps aggregate bandwidth.

Single-mode VCSELs are obtained either by oxide aperture reduction (< 4 μm diameter, forcing the higher-order transverse modes below cutoff) or by photonic crystal patterning of the top mirror. Single-mode output power is limited to ~5 mW by the small aperture.

## VCSELs in Neuromorphic Photonics

VCSELs have emerged as a key component in neuromorphic photonic systems, which exploit the nonlinear response of laser dynamics to implement spiking neuron behavior. Several proposals and demonstrations use:

1. **Excitable VCSEL neurons**: A VCSEL biased just below threshold can be excited by an input optical pulse above a threshold amplitude, firing a single coherent output spike before returning to the sub-threshold state — analogous to an integrate-and-fire neuron. The spike shape and recovery time are set by the relaxation oscillation frequency [1].

2. **Polarization switching**: VCSELs are sensitive to the polarization of injected light, and their polarization state can switch between two stable orientations. This bistable behavior can be used as an optical memory element.

3. **VCSEL arrays**: The natural 2D array format of VCSELs maps well onto the parallel signal processing needs of optical neural networks — an $N \times N$ VCSEL array could drive an $N \times N$ photonic matrix processor.

## Temperature Dependence

VCSELs at 850 nm (GaAs-based) are significantly more temperature-stable than DFB lasers at 1550 nm (InP-based), because the GaAs gain peak shifts less with temperature and the threshold current density variation with temperature is gentler. This gives 850 nm VCSELs a practical advantage for uncooled operation — a significant power savings in large arrays.

1550 nm VCSELs are more temperature-sensitive, requiring the same TEC stabilization as DFB lasers for WDM applications.

## References

[1] Hurtado, A., Schires, K., Henning, I.D., & Adams, M.J. (2012). "Investigation of vertical cavity surface emitting laser dynamics for neuromorphic photonic systems." *Applied Physics Letters*, 100(10), 103703.
