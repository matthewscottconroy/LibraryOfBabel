# The Mach-Zehnder Interferometer: From Phase to Intensity

> *The Mach-Zehnder interferometer (MZI) converts a phase shift — invisible to a photodetector — into an intensity change that any detector can measure. This conversion is the foundation of every high-speed optical modulator on the planet. From the 400 Gbit/s coherent transceivers that carry most of the world's internet traffic, to the LiNbO₃ modulators in photonic neural networks, to the silicon photonic switches in data center optical interconnects: all are MZIs. Understanding the MZI from first principles means understanding the entire language of electro-optic modulation.*

---

## 1. The MZI as a Phase-to-Intensity Converter

### 1.1 Conceptual Picture

An MZI splits an input beam into two arms, allows each arm to accumulate phase independently, then recombines them. If both arms accumulate the same phase, the beams interfere constructively at one output and destructively at the other (depending on how the combiner is configured). If the phases differ, the interference is partial, and the power is distributed between outputs accordingly.

The MZI is therefore a tunable beamsplitter: by controlling the differential phase $\Delta\Phi = \Phi_1 - \Phi_2$, we control the splitting ratio continuously from 0:100 to 100:0. For a modulator, the electrical signal controls $\Delta\Phi$, and the optical output encodes the signal.

### 1.2 Transfer Matrix Framework

Integrated photonic circuits are most cleanly analyzed using the transfer matrix (T-matrix) method. Each element — waveguide section, beamsplitter, phase shifter — is represented by a $2\times2$ unitary matrix acting on the vector of (complex) field amplitudes at the two ports.

**Notation:** Let $\mathbf{a} = (a_1, a_2)^T$ be the field amplitude vector at some cross-section, where $|a_j|^2 = P_j$ is the power in port $j$.

**50/50 Beamsplitter (directional coupler):**

$$M_\text{BS} = \frac{1}{\sqrt{2}}\begin{pmatrix}1 & i \\ i & 1\end{pmatrix}$$

This is the symmetric beamsplitter: input in port 1 gives equal power in both outputs, with a $\pi/2$ phase shift in the cross-coupled output. It is unitary: $M_\text{BS}^\dagger M_\text{BS} = I$.

**Phase shifter:**

$$M_\Phi = \begin{pmatrix}e^{i\Phi_1} & 0 \\ 0 & e^{i\Phi_2}\end{pmatrix}$$

This applies independent phase shifts to the two arms.

---

## 2. Full MZI Analysis

### 2.1 Transfer Matrix of the Complete MZI

An MZI consists of: input beamsplitter $M_1$, phase section $M_\Phi$, output beamsplitter $M_2$. The total transfer matrix is:

$$M_\text{MZI} = M_2 \cdot M_\Phi \cdot M_1$$

For two ideal 50/50 beamsplitters:

$$M_\text{MZI} = \frac{1}{\sqrt{2}}\begin{pmatrix}1 & i \\ i & 1\end{pmatrix} \begin{pmatrix}e^{i\Phi_1} & 0 \\ 0 & e^{i\Phi_2}\end{pmatrix} \frac{1}{\sqrt{2}}\begin{pmatrix}1 & i \\ i & 1\end{pmatrix}$$

Let $\bar{\Phi} = (\Phi_1 + \Phi_2)/2$ and $\Delta\Phi = \Phi_1 - \Phi_2$. Computing the matrix product:

$$M_\text{MZI} = e^{i\bar{\Phi}}\begin{pmatrix}i\cos(\Delta\Phi/2) & -\sin(\Delta\Phi/2) \\ -\sin(\Delta\Phi/2) & -i\cos(\Delta\Phi/2)\end{pmatrix}$$

Wait — let us be careful. Computing explicitly:

$$M_\Phi \cdot M_1 = \frac{1}{\sqrt{2}}\begin{pmatrix}e^{i\Phi_1} & ie^{i\Phi_1} \\ ie^{i\Phi_2} & e^{i\Phi_2}\end{pmatrix}$$

Then $M_2 \cdot (M_\Phi \cdot M_1)$:

$$M_\text{MZI} = \frac{1}{2}\begin{pmatrix}e^{i\Phi_1} + i^2 e^{i\Phi_2} & ie^{i\Phi_1} + ie^{i\Phi_2} \\ ie^{i\Phi_1} + ie^{i\Phi_2} & i^2 e^{i\Phi_1} + e^{i\Phi_2}\end{pmatrix} = \frac{e^{i\bar\Phi}}{2}\begin{pmatrix}e^{i\Delta\Phi/2} - e^{-i\Delta\Phi/2} & i(e^{i\Delta\Phi/2}+e^{-i\Delta\Phi/2}) \\ i(e^{i\Delta\Phi/2}+e^{-i\Delta\Phi/2}) & -(e^{i\Delta\Phi/2}-e^{-i\Delta\Phi/2})\end{pmatrix}$$

$$\boxed{M_\text{MZI} = e^{i\bar{\Phi}}\begin{pmatrix}i\sin(\Delta\Phi/2) & i\cos(\Delta\Phi/2) \\ i\cos(\Delta\Phi/2) & -i\sin(\Delta\Phi/2)\end{pmatrix}}$$

### 2.2 The Phase-to-Intensity Transfer Function

With all input power in port 1 ($\mathbf{a}_\text{in} = (1,0)^T$), the output amplitudes are:

$$a_{\text{out},1} = i e^{i\bar{\Phi}}\sin(\Delta\Phi/2), \qquad a_{\text{out},2} = i e^{i\bar{\Phi}}\cos(\Delta\Phi/2)$$

The output powers:

$$\boxed{P_\text{out,1} = |a_{\text{out},1}|^2 = P_\text{in}\sin^2(\Delta\Phi/2)}$$
$$\boxed{P_\text{out,2} = |a_{\text{out},2}|^2 = P_\text{in}\cos^2(\Delta\Phi/2)}$$

Power conservation: $P_1 + P_2 = P_\text{in}$ (since $\sin^2 + \cos^2 = 1$). The fundamental MZI transfer function: $I_\text{out} = I_\text{in}\cos^2(\Delta\Phi/2)$.

**Three operating points:**

| $\Delta\Phi$ | $P_1$ | $P_2$ | Name |
|---|---|---|---|
| $0$ | $0$ | $P_\text{in}$ | Bar state (null output at port 1) |
| $\pi/2$ | $P_\text{in}/2$ | $P_\text{in}/2$ | 3 dB point (quadrature) |
| $\pi$ | $P_\text{in}$ | $0$ | Cross state (full output at port 1) |

The $\pi/2$ **quadrature point** is where $dP/d(\Delta\Phi)$ is maximum — the most sensitive operating point for analog modulation. The $0$ and $\pi$ points are used for on-off keying (digital modulation).

### 2.3 Extinction Ratio

In a real MZI, the beamsplitters are not perfectly 50/50, and fabrication imperfections cause imbalance. The **extinction ratio** is:

$$\text{ER} = 10\log_{10}\!\left(\frac{P_\text{max}}{P_\text{min}}\right) \text{ dB}$$

For an ideal MZI, $P_\text{min} = 0$ and ER $= \infty$. In practice:
- Silicon photonic MZIs (passive, room temperature): ER $\approx 20$–30 dB
- With thermal tuning of the coupler: ER $> 40$ dB achievable
- LiNbO₃ MZIs: ER $> 30$ dB typical

The extinction ratio is limited by the imbalance $\epsilon$ in the splitting ratio: if one coupler splits $50\pm\epsilon$ instead of $50:50$, the residual field that doesn't cancel in the destructive port gives ER $\approx -20\log_{10}(2\epsilon)$ dB.

---

## 3. The Electro-Optic Effect

To make the MZI a modulator, we must convert an electrical voltage into a phase shift. The primary mechanism is the **Pockels effect** (linear electro-optic effect).

### 3.1 Origin: The Nonlinear Polarization

In a medium with a second-order optical nonlinearity, the polarization induced by an electric field $\mathbf{E}$ contains a term:

$$P_i^{(2)} = \varepsilon_0 \sum_{jk} \chi_{ijk}^{(2)} E_j E_k$$

where $\chi^{(2)}$ is the second-order susceptibility tensor. The Pockels effect arises when one of the fields is DC (or low-frequency), say $E_k^\text{RF}$, and the other is the optical field $E_j^\text{opt}$. Then:

$$P_i^{(2)} \propto \chi_{ijk}^{(2)} E_j^\text{opt} E_k^\text{RF}$$

This modifies the linear susceptibility: the effective permittivity depends on the applied RF field. Writing the modified refractive index:

$$n(E) = n_0 - \frac{1}{2} n_0^3 r E$$

where $r$ is the electro-optic coefficient (element of the reduced $r_{ij}$ tensor). This is the Pockels effect.

### 3.2 The Pockels Effect Formula

For lithium niobate (LiNbO₃) with the optical field along the extraordinary axis ($z$-axis) and the RF field applied along $z$:

$$\Delta n_e = -\frac{1}{2} n_e^3 r_{33} E_z$$

**Material parameters:**
- LiNbO₃: $n_e = 2.138$ at 1550 nm, $r_{33} = 30.8$ pm/V — the largest Pockels coefficient in common materials
- BaTiO₃: $r_{33} \approx 1300$ pm/V (but more difficult to integrate)
- InP (III-V semiconductors): $r_{41} \approx 1.4$ pm/V (much smaller, but electrically pumped)
- Silicon: centrosymmetric crystal, no Pockels effect (requires strain engineering or heterogeneous integration)

For an electric field applied across a gap $d$ by voltage $V$: $E = V/d$.

### 3.3 Phase Shift and the $V_\pi L$ Product

The phase accumulated by the optical field over an interaction length $L$ in an arm with applied voltage $V$:

$$\Phi(V) = \frac{2\pi}{\lambda} n(V) L = \frac{2\pi}{\lambda}\left(n_0 - \frac{n_e^3 r_{33}}{2}\frac{V}{d}\right)L$$

The differential phase shift relative to zero voltage:

$$\Delta\Phi(V) = -\frac{\pi}{\lambda}\frac{n_e^3 r_{33} L}{d} V$$

**Definition of $V_\pi$:** The voltage required to shift the phase by $\pi$ (which switches the MZI from bar to cross state):

$$\boxed{V_\pi = \frac{\lambda d}{n_e^3 r_{33} L}}$$

The **$V_\pi \cdot L$ product** is a figure of merit for the modulator that is independent of length:

$$V_\pi L = \frac{\lambda d}{n_e^3 r_{33}}$$

Lower $V_\pi L$ means more efficient modulation (less voltage for the same phase shift, or same phase shift over shorter interaction length).

**Typical values:**
- LiNbO₃ x-cut MZI: $V_\pi L \approx 3$–5 V·cm (electrode gap $d \approx 10$ μm)
- LiNbO₃ z-cut with $r_{33}$: $V_\pi L \approx 1.5$–2 V·cm (electrode can be placed directly on waveguide)
- Thin-film LiNbO₃ (TFLN) on silicon: $V_\pi L \approx 1$ V·cm (much tighter mode confinement)
- Silicon-organic hybrid (SOH): $V_\pi L < 0.5$ V·cm (organic EO material fills the waveguide gap)
- Plasma dispersion (silicon PN junction): not a Pockels effect — see below

### 3.4 Modulation Depth and Linearity

The output intensity as a function of voltage (single-drive configuration):

$$P_\text{out}(V) = \frac{P_\text{in}}{2}\left[1 + \cos\left(\pi\frac{V}{V_\pi} + \phi_0\right)\right]$$

where $\phi_0$ is the static phase bias. Biased at quadrature ($\phi_0 = \pi/2$):

$$P_\text{out}(V) \approx \frac{P_\text{in}}{2}\left[1 - \frac{\pi}{V_\pi}V + O\left(\frac{V^3}{V_\pi^3}\right)\right] \quad \text{for } V \ll V_\pi$$

The response is approximately linear for small signals, with deviations (harmonic distortion) that scale as $(V/V_\pi)^3$. This is important for analog/RF photonics applications requiring high spurious-free dynamic range (SFDR).

---

## 4. Bandwidth Limitations

### 4.1 The RC Time Constant

A lumped-element modulator has bandwidth limited by the RC time constant of the electrode-waveguide structure. The electrode has capacitance $C$ and the drive circuit has impedance $R_S$ (typically 50 Ω). The 3 dB bandwidth:

$$f_{3\text{dB}} = \frac{1}{2\pi R_S C}$$

For LiNbO₃: capacitance per unit length $\sim 0.5$ pF/mm. For $L = 1$ cm: $C = 5$ pF, giving $f_{3\text{dB}} = 1/(2\pi \times 50 \times 5\times10^{-12}) \approx 640$ MHz.

This is adequate for some applications but far below the GHz–THz bandwidths needed for telecommunications.

### 4.2 Traveling-Wave Electrode Design

The solution to RC bandwidth limitations: make the RF electrode itself a transmission line. The RF signal travels along the electrode as a microwave wave, synchronized with the optical wave in the waveguide. This is the **traveling-wave (TW) electrode** design.

In a TW-MZI, the bandwidth is now limited by:

1. **Velocity mismatch:** If the RF phase velocity $v_\text{RF}$ differs from the optical group velocity $v_g = c/n_g$, the two waves fall out of phase as they propagate. The interaction length becomes limited to $L_\pi = v_\text{RF} v_g / [f |v_\text{RF} - v_g|]$.

2. **Microwave losses:** The coplanar waveguide (CPW) electrode has finite conductivity; the RF wave attenuates as $e^{-\alpha_\text{RF} \ell}$.

The bandwidth of a TW-MZI considering both effects:

$$f_{3\text{dB}} = \frac{1.4 v_g}{\pi L \Delta n_\text{micro}} \quad \text{(velocity mismatch dominated)}$$

where $\Delta n_\text{micro} = c/v_\text{RF} - n_g$ is the mismatch in group and microwave indices.

**LiNbO₃ challenge:** Microwave index $n_\text{micro} \approx 4.2$, optical group index $n_g \approx 2.2$. Large mismatch. Solutions:
- Use x-cut geometry and optimize electrode thickness
- Add microwave slow-wave structures to reduce $v_\text{RF}$
- Use thin-film LiNbO₃ on silicon (lower microwave index; $n_\text{micro} \approx 2.4$, much closer to $n_g$)

Modern TFLN modulators achieve 100 GHz bandwidth with $V_\pi = 2$ V.

**Silicon photonics:** Uses plasma dispersion (free-carrier effects) rather than Pockels effect. Bandwidth is ultimately set by the carrier lifetime and PN junction RC constant. Leading silicon modulators: $>100$ Gb/s NRZ with segmented push-pull designs.

---

## 5. Push-Pull Operation and Chirp

### 5.1 Dual-Drive Push-Pull

In a single-drive MZI, voltage is applied to one arm only. The phase imbalance is $\Delta\Phi = \pi V / V_\pi$, but the common-mode phase $\bar{\Phi} = (\Phi_1 + \Phi_2)/2$ also changes, adding a phase modulation to the amplitude modulation. This is **chirp** — a time-varying frequency shift of the carrier.

In a **push-pull** (dual-drive) configuration, equal and opposite voltages $\pm V/2$ are applied to the two arms:

$$\Phi_1 = +\frac{\pi V}{2V_\pi}, \qquad \Phi_2 = -\frac{\pi V}{2V_\pi}$$

Then:
- $\Delta\Phi = \pi V/V_\pi$ (same modulation depth as single-drive)
- $\bar{\Phi} = 0$ (no common-mode phase change)

**Chirp parameter $\alpha$ (Henry parameter):**

$$\alpha = \frac{d\phi/dt}{(-1/2) dP/P dt}$$

where $\phi$ is the instantaneous phase and $P$ is the optical power. For push-pull operation, $\alpha = 0$ — the modulator is **chirp-free**.

For single-drive: $\alpha \neq 0$. The sign and magnitude depend on the bias point.

**Why chirp matters:** Chirped pulses disperse in a fiber, opening the eye diagram and limiting distance. In 100G and 400G coherent systems, $\alpha = 0$ modulators are essential. Even in short-reach (<2 km) applications, chirp can introduce power penalties.

### 5.2 IQ Modulator

Two MZIs in quadrature (relative phase of $\pi/2$ between parent and child MZIs) form an **IQ modulator** capable of modulating both quadratures of the optical field:

$$E_\text{out} = A(t)e^{i\phi(t)} = I(t) + iQ(t)$$

This enables QAM (quadrature amplitude modulation) formats (16-QAM, 64-QAM) that pack multiple bits per symbol — essential for 400G and 800G transmission.

---

## 6. Silicon Photonic MZIs vs. LiNbO₃ MZIs

The two dominant platforms have complementary strengths:

| Property | Silicon (PN junction) | LiNbO₃ (bulk) | Thin-film LiNbO₃ |
|---|---|---|---|
| **Mechanism** | Plasma dispersion | Pockels ($r_{33}$) | Pockels ($r_{33}$) |
| **$V_\pi L$** | 0.5–1 V·cm | 5–10 V·cm | 1–2 V·cm |
| **Bandwidth** | 50–100 GHz | 40–100 GHz | 50–100+ GHz |
| **Insertion loss** | 3–8 dB | 2–4 dB | 1–3 dB |
| **Footprint** | ~0.1 mm² | ~10 cm² | ~1 mm² |
| **CMOS integration** | Native | Hybrid only | Heterogeneous |
| **Chirp** | Design-dependent | Push-pull: zero | Push-pull: zero |
| **Temperature dependence** | $dn/dT = 1.8\times10^{-4}$ K⁻¹ | Low | Low |

**Silicon: advantages** — dense integration, foundry manufacturing, CMOS-compatible electronics. **Silicon: limitations** — plasma dispersion is not a true Pockels effect; it causes both phase change (desired) and absorption change (creates chirp); requires careful bias control.

**LiNbO₃: advantages** — pure phase modulation (Pockels effect), low chirp, mature technology. **LiNbO₃: limitations** — large footprint (cm scale), cannot be monolithically integrated with silicon electronics.

**Thin-film LiNbO₃** (TFLN, 300–600 nm thick LiNbO₃ bonded to SiO₂) combines the advantages: Pockels effect + tight optical confinement ($n_\text{eff} \approx 2.2$ vs. $1.7$ in bulk) + small footprint. TFLN is the leading platform for ultra-high-bandwidth modulators as of 2024.

---

## 7. The MZI as a Matrix Element in Optical Neural Networks

In the context of photonic computing (Chapter 6), MZIs serve as tunable beam splitters that implement $2\times 2$ unitary transformations:

$$U_\text{MZI}(\theta,\phi) = \begin{pmatrix}e^{i\phi}\cos\theta & -\sin\theta \\ e^{i\phi}\sin\theta & \cos\theta\end{pmatrix}$$

where $\theta = \Delta\Phi/2$ controls the splitting ratio and $\phi$ is an additional phase. By cascading $N(N-1)/2$ such MZIs, any $N\times N$ unitary matrix can be implemented (Reck decomposition, 1994). This is the foundation of optical matrix-vector multipliers and photonic neural network accelerators.

For such applications, the key figure of merit is not bandwidth but **reconfigurability speed** (for dynamic networks) and **phase error** (for accuracy). Silicon photonic MZIs with thermo-optic phase tuning ($\sim 10$ mW for $\pi$ shift, $\sim 100$ μs response time) are standard; electro-optic phase tuning enables nanosecond reconfigurability.

---

## 8. Worked Example: Link Budget for a 100G MZI Modulator

**Given:**
- Laser power at MZI input: $P_0 = 13$ dBm (20 mW)
- MZI insertion loss: 5 dB (0.35×)
- Fiber coupling loss: 2 dB per facet, two facets = 4 dB
- Fiber span: 80 km, fiber loss 0.2 dB/km = 16 dB
- Receiver sensitivity at 100 Gb/s: −18 dBm

**Total link budget:**
$$P_\text{received} = 13 - 5 - 4 - 16 = -12 \text{ dBm}$$

**Margin:** $-12 - (-18) = 6$ dB. This is a comfortable margin, but note that we have not accounted for penalty from extinction ratio, chromatic dispersion, and nonlinear effects, which typically subtract another 3–4 dB.

**$V_\pi$ scaling:** At 100 Gb/s NRZ with modulation bandwidth 70 GHz, the MZI requires $V_\pi < 1$ V to swing from 0 to $V_\pi$ within one bit period with available CMOS driver voltage. This is exactly the target for TFLN and silicon-organic hybrid platforms.

---

## 9. Exercises

**9.1** (Easy) An MZI is biased at quadrature with $V_\pi = 5$ V. A voltage $V = 0.5$ V is applied. What is the output power as a fraction of the input? Compute the small-signal approximation and compare.

**9.2** (Easy) Using the transfer matrix method, compute the output of an MZI where the first beamsplitter has coupling $\theta_1 = \pi/3$ (not balanced) and the second has $\theta_2 = \pi/4$. Input is in port 1 only.

**9.3** (Medium) A push-pull MZI applies $+V/2$ to arm 1 and $-V/2$ to arm 2 of a LiNbO₃ modulator ($r_{33} = 30.8$ pm/V, $n_e = 2.138$, $d = 5$ μm). Find the interaction length $L$ needed for $V_\pi = 3.5$ V.

**9.4** (Medium) A silicon PN junction modulator has $dn/dN = -8.8\times10^{-22}$ m³ (free electron contribution at 1550 nm). The junction is 450 nm wide. Under reverse bias, depletion width increases from 100 nm to 200 nm, so the overlap of the optical mode with the depleted region changes by $\Gamma = 0.3$. Estimate the resulting phase shift $\Delta\Phi$ for a 1 mm long device.

**9.5** (Medium) Derive the frequency response of a traveling-wave MZI considering only velocity mismatch (neglect microwave attenuation). Show that the EO response is:

$$H(f) = \frac{\sin(\pi f L \Delta n/c)}{\pi f L \Delta n/c}$$

and find the 3 dB bandwidth.

**9.6** (Hard) An IQ modulator consists of two MZIs with a relative optical phase of $\pi/2$ between them (parent MZI). Drive signals are $V_I(t) = V_\pi/2 \cdot \cos(\Omega t)$ and $V_Q(t) = V_\pi/2 \cdot \sin(\Omega t)$. Show that the output optical field traces a circle in the IQ plane (QPSK modulation), and compute the symbol rate.

**9.7** (Hard) Thermal drift: the thermo-optic coefficient of silicon is $dn/dT = 1.84\times10^{-4}$ K⁻¹. A 1 cm MZI arm in silicon experiences a temperature fluctuation of 0.1 K. What is the resulting phase drift $\delta\Phi$? How often must the bias be corrected if the modulator must remain within 5° of quadrature?

---

## 10. Further Reading

- **Textbooks:** Saleh & Teich, *Fundamentals of Photonics*, Ch. 18–20; Yariv, *Quantum Electronics*, Ch. 9; Hunsperger, *Integrated Optics*, Ch. 10
- **Review Articles:** Wooten et al., "A review of lithium niobate modulators for fiber-optic communications systems," *IEEE J. Sel. Topics Quantum Electron.* 6, 69 (2000); Wang et al., "Integrated lithium niobate electro-optic modulators operating at CMOS-compatible voltages," *Nature* 562, 101 (2018)
- **Silicon Photonics:** Reed et al., "Silicon optical modulators," *Nature Photon.* 4, 518 (2010)
- **Photonic Neural Networks:** Shen et al., "Deep learning with coherent nanophotonic circuits," *Nature Photon.* 11, 441 (2017)
