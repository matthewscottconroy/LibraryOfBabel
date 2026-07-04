# Subsection 12.4.1: Microring Weight Banks

## Orientation

The microring resonator was introduced in Chapter 7 as a filter and a modulator. Here it acquires a third identity: an analog multiplier. A ring parked *near* — not on — a WDM channel's wavelength transmits a continuously adjustable fraction of that channel's power, and that fraction is a synaptic weight. A bank of $N$ rings along one waveguide, each addressing its own wavelength, applies $N$ weights in parallel; the photodetector at the end sums them. This subsection develops the device physics of the weight bank quantitatively: the transmission function, the signed-weight trick, the crosstalk-versus-channel-count tradeoff, and the control precision demonstrated in hardware.

---

## 12.4.1.1 The Add-Drop Ring as a Tunable Weight

Consider an add-drop microring: a ring waveguide evanescently coupled to two bus waveguides (through and drop). Near a resonance $\lambda_0$, the power transmissions to the two ports are approximately Lorentzian in the detuning $\delta\lambda = \lambda - \lambda_0$:

$$T_{\text{drop}}(\delta\lambda) = \frac{T_{\text{peak}}}{1 + \left(2\delta\lambda/\Delta\lambda_{\text{FWHM}}\right)^2}, \qquad T_{\text{thru}}(\delta\lambda) \approx 1 - T_{\text{drop}}(\delta\lambda) - A$$

where $\Delta\lambda_{\text{FWHM}} = \lambda_0/Q$ is the loaded linewidth and $A$ accounts for intrinsic ring loss (small for $Q_{\text{intrinsic}} \gg Q_{\text{loaded}}$). For a ring with loaded $Q = 10^4$ at $\lambda_0 = 1550$ nm: $\Delta\lambda_{\text{FWHM}} = 0.155$ nm ($\approx 19$ GHz).

A WDM channel at $\lambda_i$ carrying signal power $P_i(t) \propto x_i(t)$ passes the ring tuned near $\lambda_i$. By adjusting the ring's resonance position — thermo-optically, via an integrated heater — the fraction of $P_i$ sent to the drop port is set anywhere between $\approx 0$ (far detuned) and $T_{\text{peak}} \approx 1$ (on resonance). The detuning is the knob; the transmission is the weight.

**Signed weights from a balanced photodetector.** Transmission is non-negative, but the *pair* of complementary outputs is the loophole. Route the drop port to the positive input and the through port to the negative input of a balanced photodetector pair. The differential photocurrent from channel $i$ is

$$i_i \propto \left[T_{\text{drop}}(\delta\lambda_i) - T_{\text{thru}}(\delta\lambda_i)\right] P_i = w_i P_i, \qquad w_i \in [-1, +1]$$

One ring, one continuous parameter, a full signed weight — this complementary-output arrangement (Tait et al., Princeton) is the incoherent counterpart of the differential encoding tricks that appear throughout analog computing. Summation is automatic: all $N$ wavelengths land on the same detector pair, and Kirchhoff's law adds their photocurrents:

$$y = \sum_{i=1}^{N} w_i \, x_i$$

The multiply is a filter transmission; the accumulate is a photocurrent sum. No phase coherence between channels is required — indeed the channels *must* be mutually incoherent (distinct wavelengths) so that their intensities, not their fields, add.

---

## 12.4.1.2 Crosstalk and Channel Count

Each ring's Lorentzian tail overlaps its neighbors' channels. For channel spacing $\Delta\lambda_{\text{ch}}$ and ring linewidth $\Delta\lambda_{\text{FWHM}}$, the parasitic weight applied by ring $i$ to its neighbor's channel is

$$w_{\text{xtalk}} \approx \frac{1}{1 + \left(2\Delta\lambda_{\text{ch}}/\Delta\lambda_{\text{FWHM}}\right)^2}$$

**Worked example.** $Q = 10^4$ ($\Delta\lambda_{\text{FWHM}} = 0.155$ nm), 100-GHz WDM grid ($\Delta\lambda_{\text{ch}} = 0.8$ nm): $w_{\text{xtalk}} \approx 1/(1 + (1.6/0.155)^2) = 0.93\%$ — about 7 bits of channel isolation. Halving the grid to 50 GHz quadruples the crosstalk to $\approx 3.6\%$ ($\sim$5 bits). Crosstalk, not footprint, sets the packing density.

The total channel count is bounded by the ring's free spectral range, $\text{FSR} = \lambda^2 / (n_g L_{\text{ring}})$: all $N$ channels must fit within one FSR, since a ring resonates periodically. For a silicon ring of radius 8 μm ($L = 50$ μm, $n_g \approx 4.2$): FSR $= 11.4$ nm, accommodating $N \approx 14$ channels at 100 GHz spacing or $N \approx 28$ at 50 GHz (with the crosstalk penalty above). Pushing $N$ toward 100 requires smaller rings (FSR $\propto 1/R$, at the cost of bend loss), higher $Q$ (at the cost of optical bandwidth — the ring must still pass the signal modulation sidebands, requiring $\Delta f_{\text{FWHM}} \gtrsim 2 f_{\text{mod}}$), or spectral interleaving of multiple weight banks. The practical sweet spot demonstrated to date is $N \sim 4$–$32$.

---

## 12.4.1.3 Tuning: Power, Speed, and Precision

**Thermo-optic tuning.** Integrated heaters shift silicon ring resonances by roughly 0.1–0.25 nm/mW for standard designs; a full-FSR shift therefore costs tens of milliwatts, and holding an arbitrary weight costs a few milliwatts continuously per ring. Thermal isolation trenches and undercut etching improve efficiency by an order of magnitude (to $\sim$1–4 mW/FSR) at the expense of tuning speed. Time constants are $\sim$1–10 μs — weights update at 100 kHz-class rates, fine for inference with occasional reconfiguration, hopeless for per-sample weight streaming.

**Depletion tuning.** A reverse-biased PN junction in the ring shifts the resonance electro-refractively in $<$1 ns with essentially zero static power, but over a range 10–100$\times$ smaller than thermal tuning and with bias-dependent loss. Practical systems combine both: thermal for coarse weight setting, depletion for fast dither and trim.

**Control precision.** The weight is exquisitely sensitive to detuning exactly where its slope is largest, and silicon rings drift with temperature at $\sim$10 GHz/K. Open-loop weight setting is therefore good to only a few bits. Closed-loop control — monitoring each ring via photoconductance of its own doped heater or via small dither tones, then feeding back — is what makes weight banks viable. Tait et al. (2016) demonstrated simultaneous multi-channel control of a 4-ring bank at effective weight precisions of roughly 4–5 bits; subsequent dithering-based feedback control by the same Princeton group (Zhang et al. 2022) pushed a single microring synapse beyond 9 bits of weight accuracy — comparable to the INT8 precision standard of digital inference and, notably, *better* than the demonstrated precision of coherent MZI meshes of useful size.

---

## 12.4.1.4 The Weight Bank as a Matrix Row

One bank of $N$ rings plus one balanced detector computes one inner product — one row of a matrix-vector product. An $M \times N$ matrix requires $M$ banks tapping the same WDM bus (each bank receives a copy of all $N$ channels via a splitter), for a total of $MN$ rings and $M$ detectors: the ring count reproduces the element count of the matrix, just as the MZI count reproduces the parameter count of the unitary group in the coherent architecture. At 10 μm-class ring diameters, a $16 \times 16$ weight matrix occupies well under 1 mm$^2$ of silicon — an order of magnitude denser than the equivalent MZI mesh, one of the quiet advantages of the incoherent approach.

What a single layer of weight banks cannot do is anything requiring optical phase: complex-valued matrices, unitary transforms, or coherent accumulation across layers. The comparison is drawn systematically in Subsection 12.4.3.

---

## References

[1] Tait, A.N., Nahmias, M.A., Shastri, B.J., & Prucnal, P.R. (2014). "Broadcast and weight: An integrated network for scalable photonic spike processing." *Journal of Lightwave Technology*, 32(21), 4029–4041. [Defines the weight bank and the network protocol built on it.]

[2] Tait, A.N., et al. (2016). "Multi-channel control for microring weight banks." *Optics Express*, 24(8), 8895–8906. [Simultaneous closed-loop control of multiple rings; the first quantified weight-precision figures.]

[3] Zhang, W., et al. (2022). "Silicon microring synapses enable photonic deep learning beyond 9-bit precision." *Optica*, 9(5), 579–584. [Dithering-based feedback control achieving >9-bit weight accuracy — the precision record for photonic weights.]

[4] Tait, A.N., et al. (2017). "Neuromorphic photonic networks using silicon photonic weight banks." *Scientific Reports*, 7, 7430. [System-level demonstration: weight banks wired into a recurrent network solving a differential equation.]
