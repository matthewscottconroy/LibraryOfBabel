# Subsection 14.2.2: Spatial Light Modulators (SLMs)

## Orientation

A spatial light modulator replaces the frozen plastic relief of Section 14.2.1 with an array of pixels whose phase is set electrically and can be rewritten at video rates. This one change — reprogrammable weights — converts a static optical classifier into a general-purpose "diffractive processing unit" that can be retrained, time-multiplexed into a deep network, and adapted in place to its own imperfections. This subsection covers the devices, the calibration they demand, and the reconfigurability they buy.

---

## 14.2.2.1 The Devices: LC-SLMs and DMDs

Two technologies dominate, both operating in the visible and near-infrared (typical lines 532, 633, and 1064 nm). A **liquid-crystal SLM (LC-SLM)** is a reflective silicon backplane (LCoS) whose per-pixel voltage rotates liquid-crystal molecules, changing the local birefringent path and thus imparting a controllable phase. A representative panel offers $\sim 1920\times1080$ pixels on an $\sim 8\ \mu$m pitch, a full $0$–$2\pi$ phase stroke quantized to 8 bits, and a refresh rate of $\sim 60$ Hz. A **digital micromirror device (DMD)** instead tilts aluminium micromirrors between two states at tens of kilohertz; it modulates amplitude (binary) rather than phase, and is used where speed matters more than smooth phase control, often with phase synthesized holographically.

The decisive number is the refresh rate. The LC-SLM's $\sim 60$ Hz makes the mask $\phi^l(x,y)$ a variable rather than a constant: the diffractive weights can be changed every $\sim 16.7$ ms.

## 14.2.2.2 The Reconfigurable Diffractive Processing Unit

This reprogrammability is not a convenience but an architecture. Zhou et al. (2021) built their reconfigurable "diffractive processing unit" (DPU) around exactly this capability: an input field encoded on a modulator, a phase SLM applying a trained diffractive layer, and a camera reading the resulting intensity, whose $|E|^2$ detection supplies the network's nonlinearity (Section 13.2). Because the SLM can be rewritten, a single physical DPU is time-multiplexed to emulate successive layers of a deep network, and the same hardware runs different trained models on demand. With this platform they demonstrated recognition of handwritten digits (MNIST), fashion products (Fashion-MNIST), and human-action video, scaling to millions of neurons, and — crucially — an in-situ adaptive training step that tunes the network on the real hardware to absorb its own alignment and response errors. The reconfigurable SLM is thus what makes hardware-in-the-loop training (Section 13.3.4) possible for diffractive networks: the same knobs used for inference are used for learning.

## 14.2.2.3 Worked Example: An LC-SLM Diffractive Layer at 532 nm

Consider one LC-SLM layer, $1920\times1080$ pixels at $\Delta x = 8\ \mu$m pitch, illuminated at $\lambda = 532$ nm. The pixel is large relative to the wavelength,

$$\frac{\Delta x}{\lambda} = \frac{8\ \mu\text{m}}{0.532\ \mu\text{m}} \approx 15,$$

so each pixel spans about $15\lambda$ — the opposite regime from the sub-wavelength terahertz pixel of Section 14.2.1, where the pitch was a *fraction* of $\lambda$. The active aperture is $1920\times 8\ \mu\text{m} \approx 15.4$ mm by $1080\times 8\ \mu\text{m} \approx 8.6$ mm. The refresh rate fixes the weight-update cadence at $1/60\ \text{Hz} \approx 16.7$ ms — glacial next to the microsecond weight settling of an integrated MZI mesh (Chapter 13), a reminder that reconfigurability here is for retraining, not for high-throughput inference.

The large pixel carries a cost in connectivity. A pixel of width $\Delta x$ radiates into a diffraction cone whose half-angle (to the first null) is

$$\theta \approx \sin^{-1}\!\frac{\lambda}{\Delta x} = \sin^{-1}(0.067) \approx 3.8^\circ.$$

For light from one pixel to fan out across the full aperture and reach every neuron of the next layer — the all-to-all coupling the model presumes — the layers must be separated by roughly $(\text{aperture}/2)/\tan\theta \approx 7.7\ \text{mm}/0.067 \approx 11$ cm. Coarse pixels therefore force layer spacings of order 10 cm; the sub-wavelength platforms of Section 14.2.3, whose pixels radiate into nearly the full hemisphere, recover wide-angle coupling in a far smaller volume.

## 14.2.2.4 Calibration and Phase Design

An SLM is not the ideal element the model assumes, and closing the gap is a calibration exercise. The phase response versus addressed grey level is nonlinear and wavelength-dependent, requiring a measured look-up table (a gamma curve) so that a requested $\phi$ produces the intended retardance. Neighbouring pixels are not independent: fringing fields and the finite elastic response of the liquid crystal blur the boundary between them (inter-pixel crosstalk), and LC backplanes exhibit temporal phase flicker synchronous with the addressing scheme. The masks themselves are computed offline — by the differentiable-diffraction training of Section 14.1, the modern descendant of the iterative Gerchberg–Saxton phase-retrieval algorithms long used to synthesize computer-generated holograms — and then uploaded. Left uncalibrated, these effects degrade the trained model; measured and folded into training, they are largely absorbed, which is precisely why in-situ adaptation on a reconfigurable device is so valuable.

---

## References

[1] Zhou, T., Lin, X., Wu, J., Chen, Y., Xie, H., Li, Y., Fan, J., Wu, H., Fang, L., & Dai, Q. (2021). "Large-scale neuromorphic optoelectronic computing with a reconfigurable diffractive processing unit." *Nature Photonics*, 15, 367–373. [The SLM-based reconfigurable DPU: MNIST, Fashion-MNIST, and human-action recognition with in-situ adaptation — the central reference of this subsection.]

[2] Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008. [The framework the SLM makes reconfigurable, and the fixed-plate baseline against which the SLM's programmability is measured.]

[3] Mengu, D., Luo, Y., Rivenson, Y., & Ozcan, A. (2019). "Analysis of diffractive optical neural networks and their integration with electronic neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 3700114. [Sensitivity of diffractive networks to phase quantization and modelling error, the analysis that calibration on an SLM must satisfy.]
