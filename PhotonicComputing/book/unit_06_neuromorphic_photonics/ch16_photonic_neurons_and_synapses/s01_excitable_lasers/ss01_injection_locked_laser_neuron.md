# 16.1.1 The Injection-Locked / Integrate-and-Fire Laser Neuron

## From a laser near threshold to a spiking neuron

Take a semiconductor laser and bias it just below — or just at — its lasing threshold. The device now sits at a poised operating point: the round-trip gain almost, but not quite, compensates the cavity loss, so the intracavity photon field is nearly zero but strongly susceptible to perturbation. To turn this poised laser into a neuron we need two additional ingredients that biology supplies through ion channels: a *threshold* nonlinearity, and a *slow recovery* variable that enforces the refractory period. Two experimental strategies provide them.

The first is **optical injection**: light from a master laser is injected into the biased slave laser. Weak injected signals are integrated by the slave's carrier reservoir and decay; a signal that pushes the system across a locking or unlocking boundary triggers a single large optical pulse. The second, and the one we emphasize here, is an **embedded saturable-absorber (SA) section**: part of the laser cavity is reverse-biased so that it absorbs light at low intensity but bleaches (becomes transparent) once the circulating intensity is high enough. The absorber creates a genuine threshold — the field must build up enough to bleach the absorber before a pulse can escape — and its recovery time sets the refractory period.

## The Yamada picture: gain, absorber, intensity

The standard description of a laser with a saturable absorber is the Yamada model, a three-variable dynamical system for the gain $G$ (proportional to the carrier population in the pumped gain section), the absorber loss $Q$ (the un-bleached absorption of the SA section), and the circulating optical intensity $I$. Qualitatively, the cycle runs as follows. The pump slowly raises $G$ while the intensity $I$ stays near zero (sub-threshold: nothing happens, the field decays). When incoming perturbations push $G$ high enough that the net gain $G - Q$ becomes positive, the intensity $I$ grows explosively — a fast pulse. That surge of light saturates *both* $G$ (it depletes the carriers) and $Q$ (it bleaches the absorber), and once the gain is spent the pulse extinguishes. The system is then left with a depleted gain that must slowly recharge before another pulse is possible: this is the refractory period. Because $I$ evolves on the fast photon timescale while $G$ recovers on the slow carrier timescale, the pulse is stereotyped and all-or-nothing — the very definition of excitability, and structurally identical to FitzHugh–Nagumo (fast activator $I$, slow recovery $G$).

## The LIF mapping and its speed

Nahmias, Shastri, Tait & Prucnal (2013) showed that a laser of this kind maps *rigorously* onto the leaky integrate-and-fire model of §15.1. In their "leaky integrate-and-fire laser neuron," the gain-section carrier population plays the role of the LIF membrane voltage $V$: it *integrates* incoming optical energy, *leaks* back toward its bias level with the carrier lifetime (the analog of $\tau_m = RC$), and, when it crosses the absorber-defined threshold, emits a spike and resets. Sub-threshold inputs charge the carrier reservoir and decay away; a super-threshold input triggers exactly one optical spike followed by a refractory recovery. Crucially, the model predicted spiking at **GHz rates** — roughly eight orders of magnitude faster than the millisecond dynamics of a biological neuron — because the laser's carrier and photon lifetimes are nanoseconds and picoseconds rather than milliseconds.

The concept was demonstrated experimentally by Shastri et al. (2016), who used a **graphene** saturable absorber integrated with a semiconductor laser to realize an excitable spiking source and showed genuine spike-processing behavior — temporal integration, thresholding, and refractoriness — at the predicted ultrafast timescales. Together these two works, reviewed in Prucnal et al. (2016), established the excitable SA laser as the canonical photonic LIF neuron.

## Refractory structure

As in biology, two refractory regimes appear. During the **absolute refractory period**, immediately after a spike, the gain is so depleted that no input, however strong, can trigger a second pulse. During the subsequent **relative refractory period**, the gain has partly recovered, so a spike can be evoked but only by a larger-than-usual input. Both are governed by the slow carrier recovery, and both are inherited automatically from the rate-equation dynamics rather than being engineered in by hand.

## Worked Example: spike width and maximum spike rate

Consider a semiconductor excitable laser neuron with a photon (cavity) lifetime $\tau_p \approx 2\ \text{ps}$ and a carrier lifetime $\tau_s \approx 1\ \text{ns}$. Estimate the optical spike width and the maximum sustainable firing rate, and compare with a biological neuron.

**Spike width.** The pulse is built and extinguished by the fast field dynamics, but its natural width is set by the *relaxation-oscillation* timescale of a class-B laser, which is the geometric mean of the two lifetimes:

$$\tau_{\text{spike}} \sim \sqrt{\tau_p\,\tau_s} = \sqrt{(2\times10^{-12}\ \text{s})(1\times10^{-9}\ \text{s})} = \sqrt{2\times10^{-21}\ \text{s}^2} \approx 4.5\times10^{-11}\ \text{s} \approx 45\ \text{ps}.$$

So the emitted optical spike is a few tens of picoseconds wide — consistent with the tens-of-ps to sub-ns pulses seen in excitable-laser experiments.

**Maximum firing rate.** After a spike, the neuron cannot fire again until the gain reservoir recharges, which takes on the order of the carrier lifetime $\tau_s$. The refractory-limited maximum rate is therefore

$$f_{\max} \sim \frac{1}{\tau_s} = \frac{1}{1\times10^{-9}\ \text{s}} = 1\times10^{9}\ \text{s}^{-1} = 1\ \text{GHz}.$$

**Biological comparison.** A cortical neuron has a spike width and absolute refractory period of order $1\ \text{ms}$, capping its rate near $1/(1\ \text{ms}) \approx 1\ \text{kHz}$. The laser neuron thus fires about $10^{6}$ times faster, and its individual spike is roughly $45\ \text{ps} / 1\ \text{ms} \approx 5\times10^{-8}$ as long — the "eight orders of magnitude" speed advantage quoted by Nahmias et al. (2013). The price is that the entire computation must be clocked to match: nothing about the photonic neuron is slow, so the surrounding architecture (Chapter 16.3) must feed and read it at commensurate GHz rates.

## References

- Nahmias, M.A., Shastri, B.J., Tait, A.N. & Prucnal, P.R. (2013). "A leaky integrate-and-fire laser neuron for ultrafast cognitive computing." *IEEE J. Sel. Top. Quantum Electron.*, 19(5), 1800212.
- Shastri, B.J., Nahmias, M.A., Tait, A.N., Rodriguez, A.W., Wu, B. & Prucnal, P.R. (2016). "Spike processing with a graphene excitable laser." *Scientific Reports*, 6, 19126.
- Prucnal, P.R., Shastri, B.J., Ferreira de Lima, T., Nahmias, M.A. & Tait, A.N. (2016). "Recent progress in semiconductor excitable lasers for photonic spike processing." *Advances in Optics and Photonics*, 8(2), 228–299.
