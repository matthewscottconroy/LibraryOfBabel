# 15.2.3 Energy Considerations

## The event-driven energy model

Energy is the metric on which neuromorphic computing ultimately competes, and the spiking paradigm changes how energy is spent. In a synchronous digital accelerator, essentially every multiply–accumulate (MAC) in a weight matrix is evaluated on every cycle whether or not it matters. In an **event-driven** spiking system, a neuron and its synapses draw signal energy only when a spike actually occurs. To first order the inference energy is therefore

$$E_{inf} \approx S \times E_{spike}, \tag{1}$$

where $S$ is the total number of spikes generated during the inference and $E_{spike}$ is the average energy cost of producing one spike and propagating it through its synapses. Because activity in a well-designed spiking network is *sparse* — only a small fraction of neurons fire for any given input — $S$ can be far smaller than the dense MAC count of the equivalent artificial network, and this sparsity is the source of the energy advantage.

Two quantities thus matter: the spike count $S$, set by the algorithm and the coding scheme (Section 15.1.2 showed temporal codes minimize it), and the energy per spike $E_{spike}$, set by the devices. Present photonic neuron demonstrations sit in the **femtojoule-to-picojoule per spike** range depending on platform, and a widely quoted target for the field is **below 1 fJ per spike** (Shastri et al. 2021; Ferreira de Lima et al. 2017).

## Where the energy goes

Several device-level costs make up $E_{spike}$:

- **Weighting (synapse).** With non-volatile phase-change (PCM) synapses, the weight is stored as the crystalline state of the material and the multiply happens *in place* as light traverses the cell. Holding a weight costs **zero static power** — a decisive advantage over electronic weights that must be refreshed or held in powered memory. Programming a PCM cell, however, is comparatively expensive: a melt-quench RESET pulse carries energy on the order of nanojoules. That **write energy is amortized** — for deployed, fixed-weight inference one writes each weight once and reads it optically many times, so it does not enter the per-inference budget (1).
- **Optical–electronic–optical (O-E-O) conversion.** Architectures that detect light, process in electronics, and re-modulate at each neuron pay a conversion cost of order **sub-picojoule to picojoule** per event; if conversions are frequent they can dominate $E_{spike}$, which is the standing argument for keeping the datapath all-optical.
- **The spike itself.** Generating and routing the optical pulse — laser bias, modulation, and splitting loss (Section 15.2.2) — accounts for the remainder.

For reference, a **biological** synaptic transmission event dissipates on the order of a picojoule (indeed the whole ~20 W brain, spread over its enormous event rate, works out to roughly this scale per event), and a **digital MAC** in a modern accelerator costs of order ~1 pJ for the arithmetic alone, with off-chip memory access often 10–100× higher.

## Worked Example: inference energy versus a digital accelerator

Consider a spiking network of $N = 10^{6}$ neurons in which, for a given input, each neuron emits on average $a = 10$ spikes. The total spike count is

$$S = N\times a = 10^{6}\times 10 = 10^{7}\ \text{spikes}.$$

Applying (1):

- At today's **$E_{spike} = 1$ pJ**:  $E_{inf} = 10^{7}\times 1\ \text{pJ} = 10^{-5}\ \text{J} = 10\ \mu\text{J}.$
- At the **$E_{spike} = 1$ fJ** target:  $E_{inf} = 10^{7}\times 1\ \text{fJ} = 10^{-8}\ \text{J} = 10\ \text{nJ}.$

Now estimate a digital baseline. Suppose the equivalent artificial network evaluates $W = 10^{9}$ MACs per inference at $e_{MAC}\approx 1\ \text{pJ}$ (arithmetic only):

$$E_{digital} = W\times e_{MAC} = 10^{9}\times 1\ \text{pJ} = 10^{-3}\ \text{J} = 1\ \text{mJ},$$

and data movement would push the real figure higher. The photonic spiking system is then about

$$\frac{E_{digital}}{E_{inf}} = \frac{1\ \text{mJ}}{10\ \mu\text{J}} \approx 10^{2}\ \ \text{(at 1 pJ/spike)}, \qquad \frac{1\ \text{mJ}}{10\ \text{nJ}} \approx 10^{5}\ \ \text{(at 1 fJ/spike)}$$

lower in energy — a hundredfold improvement with present devices and up to five orders of magnitude if the sub-femtojoule goal is reached. The comparison also exposes the two levers: the digital cost scales with the *dense* operation count $W$, whereas the spiking cost scales with the *sparse, event-driven* spike count $S$ — so the advantage grows precisely when activity is sparse and $E_{spike}$ is small.

## The bottom line

Speed (Section 15.2.1) and connectivity (Section 15.2.2) are only worth pursuing if the energy per spike is low enough that equation (1) beats a digital accelerator across a real workload. This is why so much of neuromorphic-photonics device research — excitable lasers, PCM synapses, and all-optical datapaths in Chapter 16 — is measured in joules per spike, and why driving $E_{spike}$ below a femtojoule is the field's central engineering target (Nahmias et al. 2013; Shastri et al. 2021).

## References

Nahmias, M.A., Shastri, B.J., Tait, A.N. & Prucnal, P.R. (2013). "A leaky integrate-and-fire laser neuron for ultrafast cognitive computing." *IEEE J. Sel. Top. Quantum Electron.*, 19(5), 1800212.

Ferreira de Lima, T., Shastri, B.J., Tait, A.N., Nahmias, M.A. & Prucnal, P.R. (2017). "Progress in neuromorphic photonics." *Nanophotonics*, 6(3), 577–599.

Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D. & Prucnal, P.R. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15(2), 102–114.
