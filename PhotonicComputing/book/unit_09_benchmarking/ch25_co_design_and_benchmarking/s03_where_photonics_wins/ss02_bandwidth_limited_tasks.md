# 25.3.2 Bandwidth-Limited and Optics-Native Tasks

## Photonics' Native Currency Is Bandwidth

Electronics buys its computation with switched charge; photonics buys its advantages with bandwidth. A single silicon waveguide carries terahertz of usable optical band — dozens to hundreds of WDM channels, each modulated at tens of GHz — through one physical structure with no crosstalk-inducing $RC$ parasitics between channels. Where Section 25.3.1's case rested on *amortizing* the conversion tax, this subsection's case rests on workloads where the tax either multiplies across parallel wavelength channels (WDM parallelism) or is never levied at all (signals already optical) or cannot be paid by electronics at any price (signals too fast to digitize).

## WDM Parallelism: One Structure, K Processors

Wavelength multiplexing converts a photonic processor's *spectrum* into compute parallelism. In broadcast-and-weight networks, each of $K$ comb lines carries an independent signal through shared microring weight banks [1]; in comb-driven convolution engines, $K$ lines and a dispersive delay turn a single modulator-detector chain into a $K$-tap parallel convolver — the mechanism behind the 11-TOPS microcomb accelerator of Section 25.2.3 [2] and the parallel kernels of the phase-change tensor core [3]. The economics are distinctive: the marginal hardware for the $K$-th processor is one more comb line — no new mesh, no new area — so throughput scales with source spectrum while the conversion tax scales only with the ports actually digitized.

The audit-trained reader will anticipate the caveats. Comb sources pay a generation efficiency (pump power into usable per-line power is often percent-level for microcombs); each wavelength channel needs its own calibration against ring resonance drift; and channel counts are bounded by free spectral range, crosstalk, and the flatness of the comb. WDM parallelism is real and demonstrated; it is a multiplier on a well-designed system, not a substitute for one.

## Optics-Native Signals: Skipping the Tax Entirely

Every budget in this chapter charged the input DAC/modulator and output ADC to the photonic system — correctly, for digital workloads. But a large class of signals is *born optical*: camera fields, LIDAR returns, fiber-borne communications, microscopy, astronomical light. For these, the input conversion line item is zero, and photonic preprocessing operates on the signal in its native domain:

- **Computing before detecting.** A lens performs a Fourier transform in transit (Chapter 11); diffractive and metasurface frontends (Chapters 8, 14) apply fixed linear feature extractors at literally zero marginal energy per frame. The design principle is *information compression before conversion*: if optics reduces a megapixel field to 100 class scores or 10 matched-filter outputs, the ADC that follows is small, slow, and cheap. The benchmark metric shifts accordingly — not TOPS/W, but detector count and ADC rate avoided at equal task accuracy.
- **The general rule.** Optical preprocessing wins when (input bandwidth × dimensionality) ≫ (output information rate). Imaging classifiers, wavefront sensors, and mode sorters fit; tasks needing the full field digitized anyway do not — if you must store the raw data, the ADC returns and with it the whole Section 25.1 ledger.

## Faster Than the ADC: RF and Sensing Front-Ends

Section 25.1.1 established a hard ceiling: aperture jitter limits *any* electronic converter to roughly 7 effective bits at 10 GHz input and less beyond — wideband, high-dynamic-range digitization is not expensive so much as *unavailable*. Photonics attacks this bound from two directions [4, 5]:

1. **Process optically, digitize narrowband.** Integrated microwave photonics implements filtering, beamforming, channelization, and correlation directly on the modulated optical carrier [5]. A 40-GHz-wide surveillance problem that would demand an impossible 80 GS/s high-ENOB converter becomes, after an optical channelizer, an array of slow, cheap, high-resolution ADCs each seeing only megahertz of pre-computed output. The photonic stage is judged not in TOPS/W but in application units — spur-free dynamic range, probability of intercept, beam count — against an electronic alternative that often simply does not exist.
2. **Slow the signal for the ADC.** Photonic time-stretch preprocessing dilates a wideband transient in time before electronic digitization, the principle behind time-stretch ADCs and single-shot instruments that capture tens of GHz with electronics rated for far less [4]. Here photonics is not competing with the converter; it is *rescuing* it — the purest case of co-design in the chapter.

FMCW LIDAR and radar pipelines combine both patterns: optical mixing performs the correlation (dechirping) in the analog domain, so the digitizer sees only kHz–MHz beat notes rather than the THz-scale optical or GHz-scale RF signal. The computation happened where the bandwidth was; the conversion happened where it was cheap.

## The Already-Won Case: Interconnect and Delocalized Compute

The largest bandwidth-limited "task" photonics performs in AI systems today is not arithmetic at all — it is **moving the data**. Beyond roughly a meter (and increasingly, beyond a centimeter), optics beats electrical signaling on energy and bandwidth-density, which is why optical links, co-packaged optics, and the optical circuit switches inside TPU v4 pods [6] are production infrastructure rather than research claims. Chapter 10 developed the physics; the benchmarking lesson here is that photonics' first verified wins in computing systems came exactly where its conversion tax was already part of the job description (a link needs E-O and O-E regardless), so the marginal cost of doing something smarter with the light — switching it, routing it, even computing on it in flight — is small.

That observation motivates the most inventive entry in this space: **delocalized photonic inference**, in which a server streams neural-network weights over fiber to edge clients whose entire computational hardware is a modulator and a detector — the fiber's bandwidth substitutes for the client's compute and memory [7]. It is a genuinely new point in the design space (compute where the data is, weights in flight), and its honest accounting — server laser power, per-client energy, fiber as shared resource — is exactly the kind of system-boundary exercise this chapter trains for.

## Summary of the Regime

Photonics wins bandwidth-limited tasks when at least one of three conditions holds: the parallelism rides on wavelengths rather than replicated hardware; the signal is already optical and can be *compressed* before conversion; or the signal exceeds what electronic converters can capture at required fidelity. In all three, the correct benchmark is application-level capability against the true electronic alternative — including the case, common in this regime, where that alternative is "no."

---

## References

[1] Tait, A.N., Ferreira de Lima, T., Zhou, E., Wu, A.X., Nahmias, M.A., Shastri, B.J., & Prucnal, P.R. (2017). "Neuromorphic photonic networks using silicon photonic weight banks." *Scientific Reports*, 7, 7430. [Broadcast-and-weight WDM architecture: wavelength channels as parallel neurons.]

[2] Xu, X., Tan, M., Corcoran, B., et al. (2021). "11 TOPS photonic convolutional accelerator for optical neural networks." *Nature*, 589, 44–51. [Microcomb time-wavelength interleaved convolution; WDM parallelism as throughput multiplier.]

[3] Feldmann, J., Youngblood, N., Karpov, M., et al. (2021). "Parallel convolutional processing using an integrated photonic tensor core." *Nature*, 589, 52–58. [Comb-driven parallel kernels over phase-change weight arrays.]

[4] Coppinger, F., Bhushan, A.S., & Jalali, B. (1999). "Photonic time stretch and its application to analog-to-digital conversion." *IEEE Transactions on Microwave Theory and Techniques*, 47(7), 1309–1314. [The time-stretch principle: photonic preprocessing to overcome electronic ADC limits.]

[5] Marpaung, D., Yao, J., & Capmany, J. (2019). "Integrated microwave photonics." *Nature Photonics*, 13(2), 80–90. [Filtering, beamforming, and channelization of RF signals in the optical domain.]

[6] Jouppi, N.P., et al. (2023). "TPU v4: an optically reconfigurable supercomputer for machine learning with hardware support for embeddings." *ISCA 2023*. [Optical circuit switching deployed in production AI infrastructure.]

[7] Sludds, A., Bandyopadhyay, S., Chen, Z., et al. (2022). "Delocalized photonic deep learning on the internet's edge." *Science*, 378, 270–276. [Weights streamed optically to receiver-only edge clients; fiber bandwidth traded for local compute.]

[8] Miller, D.A.B. (2017). "Attojoule optoelectronics for low-energy information processing and communications." *Journal of Lightwave Technology*, 35(3), 346–396. [The energy-versus-distance analysis underlying optics' interconnect victories.]
