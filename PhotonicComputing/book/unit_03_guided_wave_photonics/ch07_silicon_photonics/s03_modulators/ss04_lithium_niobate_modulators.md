# 7.3.4 Lithium Niobate Modulators

## The Alternative That Doesn't Apologize for Silicon

There is a recurring pattern in the photonic computing literature: a researcher describes the limitations of silicon modulators — the modest $V_\pi L$, the chirp, the thermal sensitivity — and then writes "therefore, lithium niobate (LiNbO₃) is an attractive alternative." This framing understates the situation. Lithium niobate is not an alternative to silicon modulators that *compensates for silicon's weaknesses*. Lithium niobate has a fundamentally different physical mechanism for modulation — the Pockels effect — that is intrinsically superior for many applications. Understanding *why* it is superior, and what the engineering tradeoffs are, is the purpose of this subsection.

## The Pockels Effect

Lithium niobate (LiNbO₃) belongs to the trigonal crystal class $R3c$, which lacks inversion symmetry. As a result, it has a nonzero second-order susceptibility $\chi^{(2)}$, and in particular, a large linear electro-optic (Pockels) coefficient $r_{33}$.

The Pockels effect describes a change in the optical indicatrix (the ellipsoid defining refractive indices for different polarizations) that is linear in the applied electric field. For LiNbO₃ with the electric field applied along the optical axis ($z$-direction) and light polarized along $z$ (extraordinary polarization):

$$\Delta\left(\frac{1}{n^2}\right) = r_{33} E_z$$

$$\Delta n_e \approx -\frac{1}{2}n_e^3 r_{33} E_z$$

For LiNbO₃ at 1550 nm:
- $n_e = 2.138$ (extraordinary index)
- $n_o = 2.211$ (ordinary index)
- $r_{33} = 30.9$ pm/V [1]
- $r_{13} = 8.6$ pm/V (ordinary polarization)

For an electric field $E_z = V/d$ (voltage $V$ across electrode gap $d$):

$$|\Delta n_e| = \frac{1}{2}n_e^3 r_{33} \frac{V}{d} = \frac{1}{2}(2.138)^3 \times 30.9 \times 10^{-12} \times \frac{V}{d}$$

$$= 1.51 \times 10^{-10} \times \frac{V}{d} \text{ [V and d in SI units]}$$

For a conventional LiNbO₃ waveguide with electrode gap $d = 15$ μm and $V = 1$ V:

$$\Delta n_e = 1.51 \times 10^{-10} / (15 \times 10^{-6}) \approx 10^{-5} \text{ per volt}$$

This is comparable to silicon's plasma dispersion effect, but with a crucial difference: the Pockels effect is *instantaneous* (responds at the speed of the $\chi^{(2)}$ tensor, essentially the electronic response time, $\tau \sim 10^{-15}$ s), produces *no associated absorption change* (pure phase modulation, no chirp in the $\alpha$ parameter sense), and is *linear* in field strength with no saturation.

## The $V_\pi L$ Product for LiNbO₃

The phase shift in a Pockels modulator of length $L$ is:

$$\Delta\phi = \frac{2\pi}{\lambda}\Delta n_e L = \frac{2\pi}{\lambda} \times \frac{1}{2}n_e^3 r_{33} \frac{V}{d} \times L$$

Setting $\Delta\phi = \pi$:

$$V_\pi = \frac{\lambda d}{n_e^3 r_{33} L}$$

Therefore:

$$V_\pi L = \frac{\lambda d}{n_e^3 r_{33}}$$

For conventional bulk LiNbO₃ waveguides with $d = 15$ μm, $\lambda = 1550$ nm:

$$V_\pi L = \frac{1550 \times 10^{-9} \times 15 \times 10^{-6}}{(2.138)^3 \times 30.9 \times 10^{-12}} = \frac{2.325 \times 10^{-14}}{2.937 \times 10^{-10}} \approx 0.079 \text{ V·m} = 7.9 \text{ V·cm}$$

This is *worse* than silicon on paper! The reason is the large electrode gap $d$: conventional LiNbO₃ waveguides are large (diffusion-based, core size ~5–10 μm), so the electrodes must be far from the optical mode to avoid metal absorption, requiring large $d$.

However, if the electrode gap can be reduced — which requires moving the mode closer to the electrodes, which requires tighter mode confinement — the $V_\pi L$ product scales directly as $d$. This is the motivation for **thin-film lithium niobate on insulator (LNOI)**.

## Thin-Film Lithium Niobate on Insulator

The LNOI platform, developed primarily at Harvard University by Marko Lončar's group and commercialized by HyperLight and others, consists of:

- A thin film (300–600 nm) of LiNbO₃ on a silicon dioxide substrate
- Ridge waveguides formed by argon plasma etching or focused ion beam milling
- Electrodes separated by gaps of $d = 4$–6 μm (compared to 15–30 μm for conventional LiNbO₃)

The waveguide confinement is much tighter: mode sizes of ~1–2 μm², comparable to silicon photonics (vs. ~50 μm² for conventional LiNbO₃). This allows the electrodes to be brought much closer to the waveguide.

For LNOI with $d = 5$ μm:

$$V_\pi L = \frac{1550 \times 10^{-9} \times 5 \times 10^{-6}}{(2.138)^3 \times 30.9 \times 10^{-12}} \approx 0.026 \text{ V·m} = 2.6 \text{ V·cm}$$

The landmark demonstration by Wang et al. in 2018 [2] achieved $V_\pi L = 2.2$ V·cm with 40 GHz bandwidth. This is ~5–10× better than silicon depletion MZI modulators in $V_\pi L$.

The key 2018 LNOI results were:
- **$V_\pi L = 2.2$ V·cm** at 1550 nm (electro-optic, not thermal)
- **Bandwidth > 100 GHz** (demonstrated by frequency response measurement)
- **Insertion loss: ~2.7 dB** (primarily coupling to fiber)
- **Extinction ratio: 30 dB** (excellent, comparable to bench-top LiNbO₃ modulators)

Subsequent work has pushed LNOI modulator bandwidths to 210 GHz [3], a result that would have seemed impossible for an on-chip modulator a decade earlier.

## The LNOI Advantage: No Chirp

When a silicon modulator changes its carrier density, it changes both $n$ (real part) and $\alpha$ (imaginary part) of the optical index. This coupling between phase and amplitude modulation is characterized by the Henry chirp parameter:

$$\alpha_H = \frac{\partial n_r/\partial V}{\partial n_i/\partial V}$$

where $n_r$ and $n_i$ are the real and imaginary parts of the refractive index. For silicon depletion modulators, $\alpha_H \approx -1$ to $-3$ [4] — meaning every 1 rad of phase shift comes with a corresponding change in absorption. This chirp broadens the modulated signal spectrum (by approximately $\Delta\nu_{\text{chirp}} \approx (1+\alpha_H^2)/(2\pi\tau_p)$ for a pulse of duration $\tau_p$) and limits the reach of the signal in dispersive fibers.

For the Pockels effect in LiNbO₃, there is *no coupling* between phase and amplitude modulation. The $\chi^{(2)}$ tensor is purely real (below the absorption edge), so $\Delta n_r \neq 0$ but $\Delta n_i = 0$. This means:

$$\alpha_H^{\text{LiNbO}_3} = 0 \quad \text{(ideal)}$$

In practice, LNOI modulators achieve $|\alpha_H| < 0.05$ [2] — effectively zero chirp. For photonic computing at chip scale, chirp matters less than in long-haul communications (no km-scale dispersion). But in coherent photonic processors where phase accuracy is critical, zero-chirp modulation gives much cleaner complex amplitude control.

## Bandwidth: The Fundamental Limit

The bandwidth of a Pockels modulator is limited by three effects:

**1. RC bandwidth**: For a lumped modulator of capacitance $C$ and resistance $R$, $f_{-3\text{dB}} = 1/(2\pi RC)$. For LNOI, the relative permittivity at microwave frequencies is $\varepsilon_{r,\text{LN}} \approx 28$ (along the $z$-axis), leading to electrode capacitances similar to silicon.

**2. Velocity mismatch**: As for silicon, the microwave group velocity must match the optical group velocity. For LiNbO₃, $n_g^{\text{opt}} \approx 2.2$ and $\sqrt{\varepsilon_{r,\text{RF}}} \approx \sqrt{28} \approx 5.3$. This velocity mismatch is worse than silicon! However, with optimized electrode geometries (loaded CPW, slow-wave electrodes), the effective microwave index can be engineered to ~2.2, achieving velocity matching [2].

**3. Electrode loss**: Resistive loss in the RF electrodes attenuates the drive signal along the modulator length. For high-speed travel-wave modulators, this sets a practical length limit of ~1–2 cm.

For LNOI with velocity-matched CPW electrodes, the bandwidth is primarily limited by electrode loss and RF input impedance matching. Wang et al. 2018 achieved >100 GHz with a 5-mm-long device; subsequent work has demonstrated 210 GHz [3].

## Comparing LiNbO₃, LNOI, and Silicon

| Parameter | Si PN (MZI) | Si Ring | LiNbO₃ (bulk) | LNOI |
|-----------|-------------|---------|----------------|------|
| Mechanism | Plasma dispersion | Plasma (resonant) | Pockels effect | Pockels effect |
| $V_\pi L$ | 10–30 V·mm | ~0.1–1 V·mm* | 50–100 V·mm | 22–30 V·mm |
| Bandwidth | 25–60 GHz | 10–60 GHz | 40–70 GHz | >100 GHz |
| Chirp ($\alpha_H$) | −1 to −3 | −1 to −3 | ~0 | ~0 |
| Footprint | 2–5 mm | 10–50 μm | 2–5 cm | 3–10 mm |
| Insertion loss | 3–6 dB | 0.5–2 dB | 3–5 dB | 2–4 dB |
| Thermal sensitivity | Moderate | High | Low | Low |
| CMOS compatible | Yes | Yes | No | No |
| Platform maturity | High | High | Very high | Medium |

*Ring modulator $V_\pi$ is not directly comparable: it's a resonant device where the effective $V_\pi$ depends on the finesse and operating point.

## Hybrid Silicon-LNOI Integration

The most compelling development for photonic computing is the possibility of combining silicon photonics' dense passive routing and CMOS integration with LNOI's superior modulator performance.

Several integration approaches have been demonstrated:

**Heterogeneous bonding**: A thin LiNbO₃ film is bonded onto a silicon-on-insulator wafer. The optical mode is guided primarily in silicon but evanescently couples into the LiNbO₃ layer where electrodes are present. The effective $r_{33}$ is reduced by the mode overlap fraction with the LiNbO₃ layer:

$$V_\pi L_{\text{hybrid}} = \frac{V_\pi L_{\text{LNOI}}}{\Gamma_{\text{LN}}}$$

where $\Gamma_{\text{LN}}$ is the optical confinement fraction in LiNbO₃. For $\Gamma_{\text{LN}} \approx 0.3$, this gives $V_\pi L_{\text{hybrid}} \approx 3.3 \times 22 = 73$ V·mm — worse than pure LNOI but better than pure silicon, with the advantage of retaining the full silicon photonics routing infrastructure.

**Butt-coupling or edge-coupling**: Silicon waveguides carry the signal on-chip; at modulation regions, the mode is adiabatically transferred to an LNOI chip section, modulated, then transferred back. The coupling loss is typically 1–2 dB per interface. For a system with infrequent modulation sections, this can be acceptable.

**LiNbO₃ photonics foundry**: HyperLight and EPFL (with IMT Bucharest) have established LNOI MPW runs analogous to the silicon photonics MPW model. A designer can tape out an LNOI chip with passive waveguides, multimode interference couplers, and high-bandwidth electro-optic modulators. The platform does not offer the laser integration or photodetector integration that silicon photonics provides.

## Significance for Photonic Computing

The LNOI platform has changed the performance envelope for photonic computing hardware. Prior to 2018, the choice was: (a) silicon photonics with moderate-efficiency, chirped, thermally sensitive modulators on a foundry-compatible platform, or (b) conventional LiNbO₃ with excellent modulator quality but large footprint and no integration. LNOI offers a third path: the quality of the Pockels effect at the scale of silicon photonics.

For a photonic neural network accelerator, the modulator specification depends strongly on the system architecture:

- **Weight update rate**: If weights are updated at 1 kHz (slow learning), almost any modulator technology suffices. If updated at 1 GHz (fast reconfiguration for TDM processing), LNOI or high-speed silicon ring modulators are required.
- **Analog precision**: LNOI's zero chirp and linear Pockels response enable 8–10 bits of analog precision, vs. ~5–7 bits for silicon plasma dispersion modulators.
- **Energy per weight update**: LNOI capacitance is similar to silicon; energy per bit is comparable. Ring modulators win on energy.

The right choice depends on the application. What the LNOI platform has definitively demonstrated is that the modulator is no longer the fundamental bottleneck in silicon-adjacent photonic computing platforms. The remaining bottlenecks — laser integration, photodetector precision, thermal management, and the system-level challenges of calibrating hundreds or thousands of optical elements — are the active frontiers of the field.

---

## References

[1] Shoji, I. & Asahara, Y. (1992). "Accurate spectral analysis of second-harmonic generation in KTiOPO₄ and KNbO₃." *Physical Review B*. [For LiNbO₃ Pockels coefficients, see: Yariv, A. & Yeh, P. (2007). *Photonics*, 6th ed., Table 9.1.]

[2] Wang, C., Zhang, M., Chen, X., Bertrand, M., Shams-Ansari, A., Chandrasekhar, S., ... & Lončar, M. (2018). "Integrated lithium niobate electro-optic modulators operating at CMOS-compatible voltages." *Nature*, 562(7725), 101–104. [The foundational LNOI modulator paper: $V_\pi L = 2.2$ V·cm, >100 GHz, zero chirp.]

[3] Xu, M., He, M., Zhang, H., Jian, J., Pan, Y., Liu, X., ... & Cai, X. (2020). "High-performance coherent optical modulators based on thin-film lithium niobate platform." *Nature Communications*, 11(1), 3911. [LNOI modulators with 210 GHz bandwidth.]

[4] Koch, T.L. & Bowers, J.E. (1984). "Nature of wavelength chirping in directly modulated semiconductor lasers." *Electronics Letters*, 20(25–26), 1038–1040. [The chirp parameter for semiconductor-based (plasma dispersion) modulators is analyzed here in the laser context; the $\alpha_H$ for Si EO modulators is comparable.]
