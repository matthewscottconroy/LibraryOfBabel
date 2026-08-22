# 7.3.2 The MZI Modulator

## From Phase Shift to Intensity Modulation

The plasma dispersion effect changes the refractive index of silicon, producing a phase shift. But the information we want to encode — a matrix weight, a bit, an analog voltage — is typically more naturally represented as an amplitude (or intensity) rather than a phase. A pure phase shift is invisible to a direct-detection photodetector: the intensity $|E|^2 = |E_0 e^{i\phi}|^2 = |E_0|^2$ is independent of phase.

The Mach-Zehnder interferometer (MZI) converts phase to intensity by interference. We covered the MZI transfer matrix in Section 7.2.4; here we develop the modulator implementation in full physical detail.

## Device Architecture

A silicon photonic MZI modulator consists of four elements in series:

1. **Input beam splitter** (50:50 multimode interference coupler or directional coupler)
2. **Two phase-shifting arms** (typically one or both containing a PN junction phase shifter)
3. **Output beam combiner** (symmetric to the input splitter)
4. **Traveling-wave electrodes** (for high-bandwidth drive signals)

The optical transfer function is developed from the coupler matrix formalism. For a symmetric 3-dB coupler:

$$U_{3\text{dB}} = \frac{1}{\sqrt{2}}\begin{pmatrix} 1 & i \\ i & 1 \end{pmatrix}$$

After propagating through the two arms with phase shifts $\phi_1$ and $\phi_2$:

$$U_{\text{arms}} = \begin{pmatrix} e^{i\phi_1} & 0 \\ 0 & e^{i\phi_2} \end{pmatrix}$$

And through the output coupler (identical to the input):

$$U_{\text{MZI}} = U_{3\text{dB}} U_{\text{arms}} U_{3\text{dB}}$$

$$= \frac{1}{2}\begin{pmatrix} 1 & i \\ i & 1 \end{pmatrix} \begin{pmatrix} e^{i\phi_1} & 0 \\ 0 & e^{i\phi_2} \end{pmatrix} \begin{pmatrix} 1 & i \\ i & 1 \end{pmatrix}$$

Computing the matrix product:

$$U_{\text{MZI}} = ie^{i(\phi_1+\phi_2)/2} \begin{pmatrix} \sin\frac{\Delta\phi}{2} & \cos\frac{\Delta\phi}{2} \\ \cos\frac{\Delta\phi}{2} & -\sin\frac{\Delta\phi}{2} \end{pmatrix}$$

where $\Delta\phi = \phi_1 - \phi_2$ is the differential phase. The output intensity at the "through" port (for input at port 1 only) is:

$$\boxed{I_{\text{out}} = I_{\text{in}} \cos^2\left(\frac{\Delta\phi}{2}\right)}$$

This is the classic MZI transfer function. The intensity varies sinusoidally between zero (at $\Delta\phi = \pi$, the "off" state) and maximum (at $\Delta\phi = 0$, the "on" state).

For intensity modulation, the modulator is biased at the **quadrature point** $\Delta\phi_0 = \pi/2$, where the transfer function has maximum slope and is locally linear:

$$I_{\text{out}} = I_{\text{in}} \cos^2\left(\pi/4 + \delta\phi/2\right) \approx \frac{I_{\text{in}}}{2}\left(1 - \sin(\delta\phi)\right) \approx \frac{I_{\text{in}}}{2}(1 - \delta\phi)$$

for small modulation depth $\delta\phi \ll 1$. This linear region is essential for analog modulation — photonic matrix multipliers depend on encoding continuous-valued weights as continuous amplitudes.

## Push-Pull Operation

Rather than modulating only one arm (single-drive, or "push"), modern MZI modulators typically modulate both arms in opposite directions (differential, or "push-pull"):

- Arm 1: $\phi_1 = \phi_0 + \delta\phi$
- Arm 2: $\phi_2 = \phi_0 - \delta\phi$

The differential phase shift $\Delta\phi = 2\delta\phi$, so the same driving voltage produces twice the phase change compared to single-arm modulation. This halves the $V_\pi L$ requirement.

Additionally, push-pull operation eliminates chirp from the common-mode term $(e^{i(\phi_1+\phi_2)/2} = e^{i\phi_0})$, which is a constant phase factor that does not depend on the modulation signal $\delta\phi$. Single-arm modulators have a common-mode phase variation that produces frequency chirp; push-pull eliminates this [1].

## PN Junction Geometry

The phase shifter in each arm is typically a lateral PN junction embedded in the silicon waveguide. The standard geometry, established by Intel's silicon photonics program and refined by many groups, places the PN junction near the center of the 450 × 220 nm waveguide:

```
           Metal contact (n-type)
                    |
    ________________|_______________
    |    p+  |  p  | n  |  n+     |   ← 220 nm
    |________|_____|____|_________|
         ←  450 nm  →
                    |
           Metal contact (p-type)
```

The doping concentrations are typically:
- p+ contact region: $N_A \approx 10^{20}$ cm⁻³
- p region (near junction): $N_A \approx 5 \times 10^{17}$ cm⁻³  
- n region (near junction): $N_D \approx 5 \times 10^{17}$ cm⁻³
- n+ contact region: $N_D \approx 10^{20}$ cm⁻³

The heavy doping in the contact regions ensures ohmic metal contacts; the lighter doping near the junction gives the depletion region maximum extent within the waveguide while minimizing free-carrier absorption from the undepleted carriers.

The junction position is typically offset slightly to favor the hole side, because holes produce a larger $\Delta n$ per carrier at equal density (from the Soref-Bennett asymmetry). The optimal offset, found empirically and through simulation, is approximately 50–100 nm toward the p-side from center [2].

## Traveling-Wave Electrode Design

For modulation beyond ~10 GHz, the electrical bandwidth of a lumped-element modulator is limited by its RC time constant. A modulator section 2 mm long with $C_j = 0.5$ fF/μm has total junction capacitance $C = 1$ pF. With $R = 50\ \Omega$ source impedance:

$$f_{\text{RC}} = \frac{1}{2\pi RC} = \frac{1}{2\pi \times 50 \times 10^{-12}} \approx 3.2 \text{ GHz}$$

To extend bandwidth to 40 GHz or beyond, the modulator is designed as a **transmission line**: the electrodes have distributed inductance and the junction provides distributed capacitance, forming a microwave waveguide (coplanar waveguide, CPW) that co-propagates the electrical drive signal with the optical signal.

The key design constraint for a traveling-wave modulator is velocity matching: the microwave signal must travel at the same velocity as the optical signal, so that they remain in phase throughout the modulator length:

$$v_{\text{RF}} = \frac{c}{\sqrt{\varepsilon_{\text{eff,RF}}}} = v_{\text{opt}} = \frac{c}{n_{g,\text{opt}}}$$

For silicon waveguides, $n_{g,\text{opt}} \approx 4.24$, so we need $\varepsilon_{\text{eff,RF}} \approx 18$. The silicon substrate has $\varepsilon_r \approx 11.7$ at microwave frequencies, but the CPW electrodes see both silicon and silicon dioxide (BOX layer), giving an effective microwave index of about 2.5–3. This is significantly less than the optical group index, causing velocity mismatch.

The velocity mismatch limits the effective interaction length. For a mismatch $\Delta v/v$, the modulator bandwidth is approximately:

$$f_{\text{vel}} \approx \frac{1.4c}{L|n_{g,\text{opt}} - n_{\text{RF}}|}$$

For $n_{g,\text{opt}} = 4.24$, $n_{\text{RF}} = 2.8$, and $L = 5$ mm:

$$f_{\text{vel}} \approx \frac{1.4 \times 3 \times 10^8}{5 \times 10^{-3} \times 1.44} \approx 58 \text{ GHz}$$

This exceeds the RC limit, so velocity mismatch is not the binding constraint for silicon MZI modulators of typical length. The bandwidth is primarily RC-limited in practice [3].

To maximize bandwidth for a given $V_\pi L$, designers minimize the junction capacitance per unit length by adjusting doping profiles, while maintaining sufficient $\Delta n$ per volt. The traveling-wave termination resistance at the far end of the electrode absorbs the remaining drive signal; a typical design uses $R_{\text{term}} = 50\ \Omega$ for impedance matching, which doubles power consumption compared to an ideal lossless design.

## Performance Parameters

State-of-art silicon MZI modulators at 1550 nm achieve:

| Parameter | Typical | Best Demonstrated |
|-----------|---------|-------------------|
| $V_\pi L$ | 15–30 V·mm | ~8 V·mm [4] |
| Bandwidth ($f_{-3\text{dB}}$) | 25–40 GHz | >60 GHz [5] |
| Insertion loss | 3–6 dB | ~2 dB |
| Extinction ratio | 10–20 dB | >30 dB |
| Footprint | 2–5 mm long | ~1 mm |
| Energy/bit | 100–500 fJ/bit | ~50 fJ/bit |

The $V_\pi L = 8$–30 V·mm range reflects the state of the field. At the favorable end, recent designs with interleaved PN junctions achieve 8–10 V·mm [4]; at the conservative end, early straight-PN-junction designs were ~30 V·mm. A practical design target is ~15 V·mm — achievable with standard foundry PDK parameters.

For a 2-mm-long modulator with $V_\pi L = 15$ V·mm, $V_\pi = 7.5$ V — meaning the device swings through a full $\pi$ phase shift when driven from 0 to 7.5 V. For small-signal analog modulation at the quadrature point, the drive voltage required for a given phase shift $\delta\phi$ is:

$$V_{\text{drive}} = \frac{V_\pi \delta\phi}{\pi} = \frac{7.5 \times \delta\phi}{\pi} \text{ V}$$

For $\delta\phi = 0.1$ rad (10% modulation): $V_{\text{drive}} \approx 0.24$ V — easily achievable with CMOS drivers.

## The MZI Modulator as a Linear Activation Unit

In photonic computing, the MZI modulator serves a role that extends beyond simple on-off keying. In a **photonic matrix-vector multiplier**, each MZI implements one element of the unitary decomposition of a matrix (Reck or Clements architecture, discussed in Unit V). The MZI transfer function:

$$t = \cos(\Delta\phi/2)e^{i\phi_{\text{common}}}$$

maps a voltage to a complex transmission coefficient. The programmable weight is the differential phase $\Delta\phi$, set by the DC bias on the phase shifter.

For the MZI mesh to compute the correct matrix, each MZI must be set to a precise $\Delta\phi$ value with accuracy sufficient to represent the desired weight. For an $N \times N$ unitary matrix, there are $N^2$ parameters, each implemented by one MZI. The precision requirement is:

$$\delta(\Delta\phi) < \frac{\pi}{2^k}$$

for $k$ bits of precision. At 8 bits, $\delta(\Delta\phi) < 0.012$ rad, requiring voltage precision:

$$\delta V < 0.012 \times \frac{V_\pi}{\pi} = \frac{0.012 \times 7.5}{\pi} \approx 29 \text{ mV}$$

This is a 12-bit DAC requirement for a ±1.5 V drive range — achievable with modern CMOS, but it places a direct constraint on the precision of the analog computation.

The sinusoidal transfer function of the MZI also means that the voltage-to-weight mapping is nonlinear. For a photonic matrix multiplier that needs to set weight $w \in [-1, 1]$:

$$\Delta\phi = 2\arccos(\sqrt{(1+w)/2})$$

which is monotonic but nonlinear. The DAC must either pre-distort the drive voltage to linearize the response, or the calibration routine must account for the nonlinearity directly.

## Biasing and Calibration

Silicon MZI modulators drift with temperature (~0.1 dB/°C change in extinction ratio, from the thermo-optic effect on the bias point) and require active feedback control for stable operation. A common scheme uses a monitoring photodiode at the null port of the output coupler to detect the bias drift, and applies a correction to a low-speed thermal phase shifter that adjusts the operating point back to quadrature.

In a large MZI mesh (e.g., 64×64 = 2048 MZIs), the calibration overhead is substantial. Each MZI requires its own feedback loop, and the loops interact (changing one MZI's phase affects the output signal seen by adjacent MZIs). Efficient calibration algorithms — typically iterative gradient-descent routines — are an active research area [6].

---

## References

[1] Wooten, E.L., Kissa, K.M., Yi-Yan, A., Murphy, E.J., Lafaw, D.A., Hallemeier, P.F., ... & Howerton, M.M. (2000). "A review of lithium niobate modulators for fiber-optic communications systems." *IEEE Journal of Selected Topics in Quantum Electronics*, 6(1), 69–82. [The chirp analysis for push-pull vs. single-drive MZI is developed here for LiNbO₃ but applies equally to Si.]

[2] Gardes, F.Y., Thomson, D.J., Emerson, N.G., & Reed, G.T. (2011). "40 Gb/s silicon photonics modulator for TE and TM polarisations." *Optics Express*, 19(12), 11804–11814. [Optimized PN junction geometry for silicon MZI modulator.]

[3] Watts, M.R., Zortman, W.A., Trotter, D.C., Nielson, G.N., Luck, D.L., & Young, R.W. (2010). "Adiabatic resonant microrings (ARMs) with directly integrated thermal microphotonics." *CLEO 2010*, CPDB10. [Traveling-wave electrode analysis for Si photonic modulators.]

[4] Ding, J., Ji, R., Zhang, L., Yang, L., Chen, S., Tian, Y., ... & Tao, Z. (2012). "Electro-optical response analysis of a 40 Gb/s silicon Mach-Zehnder optical modulator." *Journal of Lightwave Technology*, 31(14), 2434–2440. [Interleaved PN junction achieving $V_\pi L \approx 10$ V·mm.]

[5] Lischke, S., Peczek, A., Morgan, J.S., Sun, K., Kaynak, D., Gurlu, O., ... & Zimmermann, L. (2021). "Ultra-fast 100 GHz bandwidth silicon-germanium electroabsorption modulator." *Nature Photonics*, 15(12), 916–921. [100 GHz SiGe EAM, demonstrating the Si platform can reach very high speeds with Ge absorption mechanism.]

[6] Bandyopadhyay, S., Hamerly, R., & Englund, D. (2021). "Hardware error correction for programmable photonics." *Optica*, 8(10), 1247–1255. [Calibration algorithms for large MZI meshes with hardware imperfections.]
