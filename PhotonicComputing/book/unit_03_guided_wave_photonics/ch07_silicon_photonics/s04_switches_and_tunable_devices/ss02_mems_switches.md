# 7.4.2 MEMS Optical Switches

## Mechanics Meets Photonics

Microelectromechanical systems (MEMS) are miniaturized mechanical structures — beams, levers, cantilevers, diaphragms — fabricated on silicon chips using processes adapted from semiconductor manufacturing. MEMS accelerometers are in every smartphone; MEMS microphones are in every laptop; MEMS gyroscopes guide aircraft and automobiles. The integration of MEMS with silicon photonics is natural: both are silicon-based, both use photolithography and etching, and the spatial precision required for optical alignment (~10–100 nm) is within the reach of MEMS.

The fundamental advantage of a MEMS optical switch is its near-zero static power consumption. An electrostatic MEMS device holds its position with a voltage across a capacitive gap — essentially zero current, essentially zero power (only the leakage current of the dielectric matters). Compare this to a thermo-optic phase shifter, which requires continuous power dissipation proportional to the phase shift it maintains. For a large photonic computing matrix with hundreds of weights that change infrequently, the MEMS approach can reduce static power consumption by orders of magnitude.

## Electrostatic Actuation

The dominant actuation mechanism in silicon MEMS optical switches is electrostatic attraction. Two parallel conducting plates (one fixed, one movable) of area $A$ separated by gap $g$ experience an attractive force:

$$F = \frac{\varepsilon_0 A V^2}{2g^2}$$

When one plate is attached to a spring with spring constant $k$, the equilibrium gap $g_0$ satisfies:

$$k(g_{\text{rest}} - g_0) = \frac{\varepsilon_0 A V^2}{2g_0^2}$$

This equation has a critical instability: when the applied voltage exceeds the **pull-in voltage**:

$$V_{\text{pull-in}} = \sqrt{\frac{8k g_{\text{rest}}^3}{27\varepsilon_0 A}}$$

the movable plate snaps to contact the fixed plate. This pull-in instability is the basis of digital MEMS switches (two-state: open or closed). For analog phase control, the device must operate below pull-in voltage, where the displacement varies as:

$$\Delta g \approx -\frac{\varepsilon_0 A V^2}{2kg_{\text{rest}}^2}$$

producing a phase shift through either evanescent coupling change or waveguide gap change (described below).

For a typical silicon MEMS actuator with $A = 100$ μm², $g_{\text{rest}} = 200$ nm, $k = 0.1$ N/m:

$$V_{\text{pull-in}} = \sqrt{\frac{8 \times 0.1 \times (200\times10^{-9})^3}{27 \times 8.85\times10^{-12} \times 100\times10^{-12}}} \approx 4.4 \text{ V}$$

This is CMOS-compatible.

## Optical Switching Mechanisms

MEMS actuators couple to optical waveguides through several mechanisms:

### Evanescent Coupling Switch

A movable silicon waveguide segment is positioned above (or beside) a fixed waveguide. When the gap between them decreases to the evanescent coupling range (~100–300 nm), optical power couples between the waveguides. The coupling coefficient $\kappa$ depends exponentially on the gap:

$$\kappa(g) = \kappa_0 e^{-\gamma g}$$

where $\gamma \approx 1/d_{\text{evanescent}} \approx 10$ μm⁻¹ for silicon waveguides. A small gap change $\delta g$ produces a large change in coupling coefficient:

$$\delta\kappa = -\gamma\kappa_0 e^{-\gamma g}\delta g$$

For a coupler length $L_c$, the power coupling ratio changes as:

$$\delta\eta = \sin(2\kappa L_c)\kappa L_c \cdot \frac{\delta\kappa}{\kappa}$$

This exponential sensitivity means MEMS evanescent switches can achieve complete switching (from 0% to 100% coupling) with gap changes of just 100–200 nm — achievable with moderate voltages [1].

### Waveguide Gap Switch

A MEMS-actuated air gap is inserted directly into the waveguide. When the gap is closed (gap = 0 or near 0), light transmits. When the gap opens to tens of nanometers, the waveguide discontinuity reflects the light, creating an essentially reflective switch. The contrast ratio depends on the coupling efficiency at the gap:

$$T_{\text{gap}} = \frac{4n_1^2n_2^2}{(n_1^2+n_2^2)^2}\left|\cos(\delta\phi) + \frac{n_1n_2 - n_2n_1}{n_1n_2+n_2n_1}i\sin(\delta\phi)\right|^2$$

Wait — for a simple air gap of width $d$ in a waveguide with effective index $n$, the Fabry-Perot reflection gives a transmission:

$$T = \left|\frac{t_1 t_2 e^{i\phi_{\text{gap}}}}{1 - r_1 r_2 e^{2i\phi_{\text{gap}}}}\right|^2$$

where $t_{1,2}$ and $r_{1,2}$ are the Fresnel transmission and reflection coefficients at the waveguide-air interfaces, and $\phi_{\text{gap}} = 2\pi n_{\text{air}} d/\lambda$.

For small gaps ($d \ll \lambda$) and a silicon waveguide with $n_{\text{eff}} \approx 2.4$: the Fresnel reflection at each Si-air interface is $r = (n-1)/(n+1) = 1.4/3.4 \approx 0.41$. The transmission through both interfaces (ignoring round-trip terms for small $d$):

$$T \approx (1-r^2)^2 \approx (1-0.17)^2 \approx 0.69$$

This means even a zero-width air gap transmits only ~69% of the power — not a good switch. The gap must be engineered with anti-reflection features (mode expanders, tapers) to avoid this Fresnel loss.

### MEMS-Actuated Phase Shifter

The most relevant MEMS device for photonic computing is not a digital switch but an analog phase shifter. A MEMS-actuated waveguide gap changes the effective index of a waveguide mode by changing the boundary conditions. Consider a "dual-rail" waveguide pair where two silicon waveguides are separated by a gap $g$: when $g \to \infty$, the modes are uncoupled; when $g$ is small, the supermodes have different effective indices $n_{\text{even}}$ and $n_{\text{odd}}$.

The key result from coupled-mode theory for the differential effective index:

$$\Delta n_{\text{eff}}(g) = n_{\text{even}} - n_{\text{odd}} = \sqrt{\Delta n_0^2 + (n_2 \kappa/\omega)^2}$$

Changing $g$ with a MEMS actuator changes $\Delta n_{\text{eff}}$, and hence the phase accumulated in a balanced MZI. For displacement ranges of 50–200 nm, the achievable $\Delta n_{\text{eff}}$ change is typically $10^{-3}$–$10^{-2}$, sufficient for a $\pi$ phase shift over $L = 100$–500 μm.

## Demonstrated Silicon Photonic MEMS Devices

The field of silicon photonic MEMS is relatively young, emerging around 2015 as MEMS and silicon photonics foundry processes matured sufficiently for co-fabrication. Key results:

**Quack et al. 2021** [2]: A 4 × 4 MEMS-actuated MZI switch matrix on silicon. Switching voltage 6–12 V, crosstalk < −30 dB, insertion loss 2–4 dB per switch. Zero static power demonstrated.

**Edinger et al. 2021** [3]: MEMS phase shifter using evanescent coupling, achieving $\pi$ phase shift with 2.25 V, $P_\pi < 1$ μW static power, and 500 kHz mechanical resonance frequency. Compared to thermo-optic: 40,000× reduction in static power for equivalent phase shift.

**Akihama & Hane 2010** [4]: Early MEMS-waveguide switch demonstrating principle of evanescent coupling control.

The state-of-art MEMS phase shifter performance:
- Actuation voltage: 2–10 V
- Static power: < 10 μW (limited by leakage, not mechanical)
- Phase range: 0 to > $2\pi$
- Bandwidth: DC to ~500 kHz (set by mechanical resonance)
- Insertion loss: 0.5–2 dB
- Footprint: 50–200 μm × 5–20 μm

## Reliability and Long-Term Stability

MEMS devices are susceptible to stiction (surfaces adhering after contact), fatigue (accumulated mechanical damage from repeated cycling), and particulate contamination. For optical MEMS, these concerns are mitigated by:

1. **Non-contact operation**: MEMS phase shifters for photonic computing operate below pull-in, never touching. This eliminates stiction.
2. **Low cycle count**: A photonic computing weight bank updated at 1 kHz accumulates $3 \times 10^{10}$ cycles per year — a large number, but silicon flexures have demonstrated > $10^{12}$ cycle lifetimes under clean-room conditions [5].
3. **Hermetic packaging**: MEMS devices are typically packaged in sealed cavities with controlled atmospheres (dry nitrogen or vacuum), eliminating particulate and humidity effects.

Commercial photonic MEMS products (e.g., variable optical attenuators in DWDM systems) have demonstrated 20+ year operational lifetimes under field conditions, providing confidence that the technology is reliable enough for deployment.

## MEMS Switches in Photonic Computing Architectures

For photonic matrix-vector multiplication with slowly updated weights (update rate < 100 kHz), MEMS phase shifters are the most energy-efficient option for the matrix element. A MEMS mesh of $N^2$ elements with $N = 64$:

| Metric | Thermo-optic | MEMS |
|--------|-------------|------|
| Static power per element | 10–20 mW | <0.01 mW |
| Total static power ($N=64$) | 40 W | 40 mW |
| Reconfiguration speed | 1–100 μs | 1–100 μs |
| Phase precision | High (DAC-limited) | Medium (position noise) |
| Fabrication complexity | Low | High |
| Commercial availability | Yes | Limited |

The MEMS approach also offers the possibility of non-volatile locking: by operating in the pull-in regime, the MEMS switch snaps to a fixed position that is held mechanically rather than electrostatically. This provides true zero-power static operation, though the digital nature of pull-in limits it to binary weight values unless multiple MEMS elements are combined.

---

## References

[1] Seok, T.J., Kwon, K., Henriksson, J., Luo, J., & Wu, M.C. (2019). "Wafer-scale silicon photonic switches beyond die size limit." *Optica*, 6(4), 490–494. [32×32 MEMS optical switch fabric on a single die.]

[2] Quack, N., Takabayashi, A.Y., Sattari, H., Edinger, P., Khan, U., Errando-Herranz, C., ... & Bhave, S.A. (2023). "Integrated silicon photonic MEMS." *Microsystems & Nanoengineering*, 9(1), 27. [Comprehensive review of silicon photonic MEMS technology, including 4×4 switch matrix.]

[3] Edinger, P., Takabayashi, A.Y., Errando-Herranz, C., Khan, U., Sattari, H., Quack, N., ... & Bhave, S.A. (2021). "Silicon photonic microelectromechanical phase shifters for scalable programmable photonics." *Optics Letters*, 46(22), 5671–5674. [State-of-art MEMS phase shifter: 2.25 V, <1 μW static power, 500 kHz bandwidth.]

[4] Akihama, Y. & Hane, K. (2012). "Single and multiple optical switches that use freestanding silicon nanowire waveguide couplers." *Light: Science & Applications*, 1(6), e16. [Early demonstration of evanescent-coupling MEMS switch in silicon photonics.]

[5] Petersen, K.E. (1982). "Silicon as a mechanical material." *Proceedings of the IEEE*, 70(5), 420–457. [The foundational paper on silicon MEMS material properties; fatigue lifetime analysis.]
