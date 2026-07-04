# 16.1.3 Excitable Microring and Semiconductor Ring Laser Neurons

The excitable lasers of the previous subsections are compact but still discrete. For a wafer-scale spiking network we want neurons that can be lithographically defined and cascaded on a silicon or silicon-nitride photonic circuit, alongside the microring weight banks of §16.3. Ring-cavity lasers and passive microrings provide exactly this, and they exhibit two distinct routes to excitability.

## Mode competition in semiconductor ring lasers

A semiconductor ring laser supports two counter-propagating modes — clockwise (CW) and counter-clockwise (CCW) — that share the same active medium and therefore compete for the same carriers. Because of this competition the laser typically operates in one direction at a time, and the two directional states are bistable. Coomans et al. (2011) showed that near the boundary between these states the ring laser is **excitable**: resting in (say) the CW state, a sub-threshold perturbation decays, whereas a super-threshold perturbation triggers a single stereotyped excursion — the field briefly swings toward the CCW state and back — producing an all-or-nothing optical spike before the laser returns to rest. The CW/CCW competition supplies the threshold and the winner-take-all nonlinearity, while carrier recovery supplies the refractory period. The mechanism is attractive because a semiconductor ring laser is an in-plane cavity that is directly **integrable on-chip** and, in principle, silicon-compatible, and because the same device can be coupled to neighbors to build networks of optical spiking neurons.

## Cascadable excitability in microrings

A second route needs no directional bistability at all. Van Vaerenbergh et al. (2012) showed that a microring (in an active or even a passive-plus-gain configuration) becomes excitable through the interplay of **free-carrier** and **thermal** nonlinearities. Light circulating on resonance generates free carriers and heat; the free-carrier dispersion shifts the resonance one way on a fast (nanosecond) timescale while thermal effects shift it back on a slower timescale. This fast/slow competition around a resonance produces the same FitzHugh–Nagumo excitable structure: a stable off state, a threshold, an all-or-nothing pulse, and a recovery. Critically, the authors showed the excitability is **cascadable** — the optical output of one excitable microring can trigger the next — which is a prerequisite for building multilayer networks rather than isolated neurons. The spike energies in these microring devices are small, on the order of **10–100 fJ per spike**, thanks to the tight optical confinement of a wavelength-scale resonator.

## A genuine relative refractory period

How do we *prove* that one of these devices is behaving like a neuron and not merely pulsing? One of the sharpest tests is the **relative refractory period**: after firing, a neuron should be able to fire again, but only in response to a stronger-than-usual stimulus, with the required stimulus decreasing as the device recovers. Selmi et al. (2014) demonstrated exactly this in a **micropillar laser with an integrated saturable absorber**. By sending pairs of trigger pulses with variable delay, they showed that the threshold for evoking a second spike was elevated immediately after the first spike and relaxed back toward its baseline as the delay increased — a quantitative relative refractory period at nanosecond timescales. This is strong evidence that the excitable-laser abstraction is not a loose analogy but captures a defining dynamical property of real neurons.

## Worked Example: spike energy and refractory-limited rate of a microring neuron

Estimate the optical spike energy of an integrated microring neuron from its photon content, cross-check it against the reported 10–100 fJ range, and find the maximum firing rate set by the recovery time.

**Photon energy scale.** At the telecom wavelength $\lambda = 1550\ \text{nm}$, each photon carries

$$E_{\text{ph}} = \frac{hc}{\lambda} = \frac{(6.626\times10^{-34}\ \text{J·s})(3.0\times10^{8}\ \text{m/s})}{1.55\times10^{-6}\ \text{m}} \approx 1.28\times10^{-19}\ \text{J} \approx 0.80\ \text{eV}.$$

**Spike energy from power and duration.** Model the excitable pulse as a peak output power $P \approx 100\ \mu\text{W}$ lasting for a recovery-limited duration $\tau \approx 500\ \text{ps}$ (set by the free-carrier dynamics). Then

$$E_{\text{spike}} = P\,\tau \approx (1\times10^{-4}\ \text{W})(5\times10^{-10}\ \text{s}) = 5\times10^{-14}\ \text{J} = 50\ \text{fJ}.$$

This lands squarely inside the **10–100 fJ/spike** range reported for microring excitability, a reassuring consistency check.

**Photon-number cross-check.** A $50\ \text{fJ}$ spike contains

$$N_{\text{ph}} = \frac{E_{\text{spike}}}{E_{\text{ph}}} = \frac{5\times10^{-14}\ \text{J}}{1.28\times10^{-19}\ \text{J}} \approx 3.9\times10^{5}\ \text{photons}.$$

A few hundred thousand photons is a firmly classical, easily detected pulse, yet it is roughly $10^{4}$–$10^{5}$ times less energy than a typical VCSEL spike from §16.1.2 — the microring's tight mode confinement is what buys the fJ-scale efficiency.

**Refractory-limited maximum rate.** If the dominant recovery is the free-carrier lifetime, $\tau_{\text{rec}}\approx\tau\approx 500\ \text{ps}$, the neuron cannot re-fire faster than

$$f_{\max} \sim \frac{1}{\tau_{\text{rec}}} = \frac{1}{5\times10^{-10}\ \text{s}} = 2\times10^{9}\ \text{s}^{-1} = 2\ \text{GHz}.$$

A caveat worth stating: microrings that rely on thermal nonlinearity carry a much slower thermal recovery tail (microseconds), which can throttle the sustained rate well below the free-carrier estimate. This is a genuine engineering tension — the same thermal shift that helps create excitability also imposes a slow relaxation — and it is one reason free-carrier-dominated and saturable-absorber designs are often preferred when high sustained spike rates are the goal.

## References

- Coomans, W., Gelens, L., Beri, S., Danckaert, J. & Van der Sande, G. (2011). "Solitary and coupled semiconductor ring lasers as optical spiking neurons." *Physical Review E*, 84(3), 036209.
- Van Vaerenbergh, T. et al. (2012). "Cascadable excitability in microrings." *Optics Express*, 20(18), 20292–20308.
- Selmi, F., Braive, R., Beaudoin, G., Sagnes, I., Kuszelewicz, R. & Barbay, S. (2014). "Relative refractory period in an excitable semiconductor laser." *Physical Review Letters*, 112(18), 183902.
