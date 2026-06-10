# 7.5.2 Lithium Niobate on Insulator (LNOI) as a Complete Platform

## Beyond the Modulator

Section 7.3.4 introduced LNOI in the context of its modulator performance: the thin-film Pockels effect with $V_\pi L \approx 2.2$ V·cm, >100 GHz bandwidth, and zero chirp. But LNOI is more than a modulator platform. It is a complete photonic platform with a uniquely rich set of $\chi^{(2)}$ nonlinear phenomena, and understanding it as a platform — rather than just a device — opens up qualitatively new capabilities for photonic computing.

The key to LNOI's richness is that LiNbO₃ simultaneously possesses:
- **Large $r_{33} = 30.9$ pm/V**: Enables electro-optic modulation, as we have seen.
- **Large $d_{33} = 27.2$ pm/V**: The second-order nonlinear coefficient for second-harmonic generation and optical parametric amplification/oscillation.
- **Piezoelectricity**: The crystal deforms in response to an electric field (relevant for acousto-optic modulation and transduction to mechanical modes).
- **Wide transparency window**: 350 nm–5 μm, covering the visible, near-infrared, and part of the mid-infrared.
- **Low intrinsic optical loss**: Bulk LiNbO₃ has absorption < 0.01 dB/cm at 1550 nm; thin-film LNOI achieves < 0.1 dB/cm in fabricated waveguides.

No other material combines all of these properties. Silicon has excellent $\chi^{(3)}$ but no $\chi^{(2)}$. GaN and AlN have $\chi^{(2)}$ but much smaller than LiNbO₃. LNOI is the platform that maximizes $\chi^{(2)}$-mediated phenomena at chip scale.

## Waveguide Properties

LNOI waveguides are fabricated from single-crystal LiNbO₃ thin films (300–700 nm thick) produced by the ion-slicing method: helium ion implantation creates a damaged layer at a controlled depth; wafer bonding to SiO₂/Si; and thermal annealing causes the LiNbO₃ film to detach cleanly, leaving a single-crystal film on the substrate. This process is the same as the SOI process used for silicon photonics, adapted for LiNbO₃.

Ridge waveguides are formed by etching the LiNbO₃ film with argon ion beam milling (physical etching) or electron-beam-written dry etching. Sidewall roughness from etching is the dominant loss mechanism; state-of-art processes achieve:

$$\alpha_{\text{LNOI}} \approx 0.02\text{–}0.1 \text{ dB/cm}$$

compared to 1–3 dB/cm for silicon strip waveguides. The difference reflects both the lower refractive index contrast (and thus reduced sidewall scattering sensitivity) and advances in etch process development.

Waveguide parameters for 600-nm-thick × 1.4-μm-wide LNOI ridge waveguide (x-cut, TE mode):
- $n_{\text{eff}} \approx 1.85$
- $n_g \approx 2.2$
- $A_{\text{eff}} \approx 1$ μm²
- Bend radius (low loss): > 50 μm (larger than silicon due to lower contrast)
- Single-mode TE: yes (TM also guided but with different $n_{\text{eff}}$)

## Second-Harmonic Generation and Phase Matching

SHG in LiNbO₃ was one of the first observed nonlinear optical phenomena (Franken et al., 1961, as discussed in Chapter 3). In LNOI, the combination of tight mode confinement and periodic poling enables highly efficient on-chip SHG.

The coupled-wave equations for SHG (as derived in Chapter 3):

$$\frac{dA_2}{dz} = -i\kappa A_1^2 e^{i\Delta k z}$$

where $A_1$ and $A_2$ are the fundamental and second-harmonic amplitudes, $\kappa \propto d_{33}$, and $\Delta k = k_2 - 2k_1 = (2\omega/c)(n_{2\omega} - n_\omega)$ is the phase mismatch.

For efficient SHG, we need $\Delta k = 0$. In bulk LiNbO₃, the refractive index dispersion gives $n_{2\omega} > n_\omega$, so phase matching requires using different polarizations (type-I or type-II) at specific angles (critical phase matching) or temperatures (non-critical phase matching). In LNOI waveguides, an additional degree of freedom is available: **quasi-phase matching (QPM)** through periodic poling.

In QPM, the sign of $d_{33}$ is periodically reversed by applying an electric field pattern during crystal growth or post-processing. This creates a grating of period $\Lambda$ that supplies the missing momentum:

$$\Delta k_{\text{QPM}} = \Delta k - \frac{2\pi}{\Lambda} = 0 \implies \Lambda = \frac{2\pi}{\Delta k} = \frac{\lambda_1}{2(n_{2\omega} - n_\omega)}$$

For LiNbO₃ at $\lambda_1 = 1550$ nm (SHG to 775 nm), with $n_{1550} \approx 2.138$ and $n_{775} \approx 2.22$:

$$\Lambda = \frac{1550}{2(2.22 - 2.138)} \approx \frac{1550}{0.164} \approx 9.5 \text{ μm}$$

Periodically poled LiNbO₃ (PPLN) with this period has been used since the 1990s for efficient SHG. In PPLN LNOI waveguides, the conversion efficiency is dramatically enhanced by the tight mode confinement:

$$\eta_{\text{SHG}} = \frac{P_{2\omega}}{P_\omega^2 L^2} = \frac{8\pi^2 d_{\text{eff}}^2 L^2}{n_\omega^2 n_{2\omega} \varepsilon_0 c \lambda^2 A_{\text{eff}}}$$

State-of-art LNOI SHG efficiencies: >5000 %/(W·cm²) — compared to ~100 %/(W·cm²) for bulk PPLN waveguides [1]. This 50× enhancement comes directly from the $A_{\text{eff}}$ reduction ($\sim$1 μm² vs. $\sim$50 μm²).

## Optical Parametric Amplification

If SHG converts one photon at $\omega$ to two photons at $2\omega$ (or two photons at $\omega$ to one at $2\omega$), then optical parametric amplification (OPA) converts one "pump" photon at $\omega_p$ into two photons ("signal" at $\omega_s$ and "idler" at $\omega_i$) satisfying energy conservation $\omega_p = \omega_s + \omega_i$ and phase matching $k_p = k_s + k_i$.

OPA in PPLN LNOI waveguides has been demonstrated with gain > 30 dB using pump powers < 1 W on-chip [2]. This enables:

- **On-chip optical amplification**: Without the need for rare-earth doping or heterogeneous III-V integration, LNOI can amplify signals via OPA, though the bandwidth is determined by the phase-matching bandwidth.
- **Photon pair generation**: When operating below OPA threshold (spontaneous parametric down-conversion, SPDC), LNOI generates entangled photon pairs at $(\omega_s, \omega_i)$. This is the physical basis for quantum photonic devices (Chapters 17–22).
- **Wavelength conversion**: OPA can convert a signal at $\omega_s$ to $\omega_i$ (or vice versa), enabling wavelength-domain routing in photonic computing.

## Electro-Optic Frequency Comb

Beyond the Kerr DKS comb of Si₃N₄ (Section 7.5.1), LNOI enables a qualitatively different type of frequency comb: the **electro-optic (EO) comb**.

In an EO comb, a microwave signal at frequency $f_m$ drives a phase modulator. The phase modulation sidebands at $f_{\text{laser}} \pm n f_m$ generate equally spaced comb lines with spacing $f_m$. In a resonant cavity (ring resonator or Fabry-Perot), the modulation is resonantly enhanced, and many sidebands build up coherently.

The EO comb on LNOI has several advantages over DKS combs:
1. **Flexible line spacing**: The comb spacing is determined by the microwave drive frequency $f_m$, which can be tuned continuously. DKS comb spacing is set by the ring FSR (fixed by geometry).
2. **Phase coherence with the microwave reference**: The comb is phase-locked to the microwave drive, enabling ultra-precise frequency control for quantum and sensing applications.
3. **Lower pump power**: EO combs don't require the ~100 mW threshold of DKS combs; they work at any power level, though with fewer lines at lower power.

Zhang et al. (2019) demonstrated an LNOI EO comb with 900 lines spanning 80 nm at 1550 nm [3]. This establishes LNOI as a powerful comb source for WDM photonic computing applications.

## LNOI Foundry Access

As of 2024, several organizations provide LNOI photonic foundry services:

**HyperLight Corporation** (Cambridge, MA): MPW runs based on x-cut LNOI, offering electro-optic modulators, ring resonators, and passive waveguides. Device library includes high-bandwidth MZI modulators ($V_\pi L < 3$ V·cm, >100 GHz).

**EPFL CMi** (Lausanne, Switzerland): Academic MPW program using x-cut and z-cut LNOI with PPLN capability. Best-in-class SHG efficiency and low-loss waveguides.

**Ligentec** (Lausanne, Switzerland): Si₃N₄ foundry that has added LNOI layer capability for heterogeneous Si₃N₄/LNOI platforms.

The lack of mature LNOI foundry infrastructure remains a barrier to large-scale adoption: silicon photonics benefits from 20+ years of CMOS foundry optimization, while LNOI foundry processes have been commercially available for fewer than 5 years. The trajectory of improvement in LNOI yield, cost, and process maturity suggests this gap will narrow significantly over the next decade.

---

## References

[1] Lu, J., Surya, J.B., Liu, X., Bruch, A.W., Gong, Z., Xu, Y., & Tang, H.X. (2019). "Periodically poled thin-film lithium niobate microring resonators with a second-harmonic generation efficiency of 250,000%/W." *Optica*, 6(12), 1455–1460. [Record LNOI SHG efficiency via PPLN microring; demonstrates the advantage of tight confinement.]

[2] Javid, U.A., Ling, J., Staffa, J., Li, M., He, Y., & Lin, Q. (2021). "Ultrabroadband entangled photons on a nanophotonic chip." *Physical Review Letters*, 127(18), 183601. [Photon pair generation and OPA in PPLN LNOI waveguides; key parametric interaction reference.]

[3] Zhang, M., Buscaino, B., Wang, C., Shams-Ansari, A., Reimer, C., Zhu, R., Kahn, J.M., & Lončar, M. (2019). "Broadband electro-optic frequency comb generation in a lithium niobate microring resonator." *Nature*, 568(7752), 373–377. [LNOI EO comb with 900 lines at 1550 nm.]
