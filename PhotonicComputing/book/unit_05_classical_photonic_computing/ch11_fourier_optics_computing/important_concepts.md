# Chapter 11: Important Concepts

## Fourier Optics

**A thin lens performs a 2D Fourier transform.** For an input field at the front focal plane, the field at the back focal plane is the exact Fourier transform of the input, with spatial frequencies $f_x = x'/(\lambda f)$. The quadratic phase of the lens exactly cancels the quadratic phase in Fraunhofer diffraction.

**The 4f system implements coherent linear spatial filtering.** Two lenses in sequence: the first Fourier transforms the input, a filter mask at the Fourier plane multiplies the spectrum, and the second lens inverse-transforms. The output is the input convolved with the impulse response of the filter. Any linear translation-invariant operation on an image can be implemented by choosing the mask appropriately.

**The space-bandwidth product (SBP) bounds optical computation capacity.** $\text{SBP} = D^2/(\lambda f)$ sets the number of independent pixels a 4f system can process simultaneously. For $D = 25$ mm, $f = 100$ mm, $\lambda = 633$ nm: SBP $\approx 10^4 \times 10^4 = 10^8$ pixels. This is the fundamental capacity limit of an optical processing system.

**The VanderLugt filter computes cross-correlation optically.** A holographically recorded filter $H = \hat{f}^*$ at the Fourier plane produces the cross-correlation $(f \star g)$ at the output for any input $g$. The optical correlator was competitive with digital FFT-based correlation before the FFT algorithm matured and Moore's Law accelerated digital processing.

**Coherent systems work with complex fields; incoherent systems work with intensities.** Coherent processing can implement complex-valued filters (including matched filters with phase). Incoherent processing is restricted to non-negative impulse responses. The choice matters for photonic computing: MZI-mesh neural networks require coherent light.

---

## Microwave Photonics

**At frequencies above ~40 GHz, photonics outperforms electronics for RF signal processing.** Electronic ADC performance degrades with input frequency (Walden plot); photonic ADCs using mode-locked laser sampling achieve <1 fs jitter, enabling 8-bit ENOB at 40 GHz where electronic ADCs achieve only ~5–6 bits.

**The time-stretch ADC can capture signals at >100 GHz bandwidth.** Dispersive optical fiber stretches a modulated optical pulse in time by a factor $M$, effectively slowing the RF signal so a lower-rate electronic ADC can capture it. Demonstrated effective sampling rates >100 Gsps at 5–7 bit resolution.

**Photonic true time delay enables squint-free wideband phased-array beamforming.** Electronic phase shifters implement frequency-dependent delay (causing beam squint in wideband signals); photonic TTD using dispersive fiber implements frequency-independent delay. For > 10% fractional bandwidth phased arrays at mm-wave frequencies, photonic TTD is the only viable approach.

**Microwave photonics has the highest technology readiness level of any photonic computing domain.** Defense systems using photonic RF processing are deployed. The domain demonstrates that photonic computing is genuinely advantageous (not just theoretically promising) in a specific, well-defined physical regime.

---

## Optical Logic

**Optical logic cannot compete with CMOS for Boolean computation.** This is not an engineering limitation but a physical one: optical nonlinear elements (Kerr, SOA, PCM) require 100–10^9× more energy per switching event than CMOS transistors. The reason is that optical nonlinearities require sufficient photon fluence to perturb atomic/electronic states, while a CMOS transistor is controlled by a gate voltage that moves only a few hundred electrons.

**Two-photon absorption (TPA) prevents Kerr switching in silicon.** Silicon's TPA coefficient is large enough that at the power levels required for Kerr bistability, TPA absorbs the light before the Kerr shift can be realized. The FOM = $n_2/(\lambda \beta_{\text{TPA}})$ for silicon is $7.7\times10^{-7}$, six orders of magnitude below the required threshold of ~0.08.

**SOA cross-gain modulation demonstrates all-optical gates, but not competitively.** SOA-MZI gates have been demonstrated at 160 Gbps with ~10–100 fJ switching energy — three orders of magnitude worse than CMOS. Noise accumulation in cascaded SOA gates limits practical fan-out. No SOA logic system reached commercial deployment.

**PCM optical logic is non-volatile but energy-intensive.** Phase-change materials switch between absorbing and transparent states non-volatility (maintaining state without power). Write energy: ~10–50 nJ, five orders of magnitude above CMOS. PCM is useful for in-memory analog computation (the Feldmann tensor core) and one-time-programmable optical elements, not for high-throughput digital logic.

**The failure of optical logic is not the failure of optical computing.** The correct conclusion from 40 years of optical logic research is that light is not good at Boolean computation. It is good at linear algebra, Fourier transforms, and high-bandwidth analog signal processing — which is exactly where modern photonic computing research focuses.
