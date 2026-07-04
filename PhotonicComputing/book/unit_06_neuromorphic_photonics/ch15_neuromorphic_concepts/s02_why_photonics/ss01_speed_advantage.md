# 15.2.1 The Speed Advantage

## A hierarchy of spike timescales

The single most conspicuous difference between a brain and a photonic neural network is *how fast a spike happens*. A biological action potential is about 1 ms wide and is followed by a refractory period of similar duration, capping firing rates at a few hundred hertz. Electronic neuromorphic processors are far quicker: analog subthreshold circuits and digital spiking cores such as Loihi and TrueNorth operate with spike or time-step granularity on the order of microseconds down to nanoseconds. Photonic neurons are quicker still. An excitable semiconductor laser can emit an optical pulse a few picoseconds to tens of picoseconds wide and sustain spiking at gigahertz rates. Comparing the ~1 ms biological timescale with a ~10 ps optical one gives a factor of about $10^{8}$ — the "roughly eight orders of magnitude faster than biology" estimated for the leaky integrate-and-fire laser neuron by Nahmias et al. (2013). Measured instead as sustained firing rate — gigahertz versus a few hundred hertz — the advantage is on the order of $10^{6}$–$10^{7}$.

This headroom ultimately traces to the optical carrier. At $\lambda = 1550$ nm the carrier frequency is

$$f_c = \frac{c}{\lambda} = \frac{2.998\times 10^{8}\ \text{m/s}}{1550\times 10^{-9}\ \text{m}} \approx 1.93\times 10^{14}\ \text{Hz} = 193\ \text{THz},$$

so an optical channel offers an essentially unlimited-seeming frequency reservoir. The practical spike rate is not set by this carrier but by the modulation bandwidth of the devices and their carrier and photon dynamics — typically tens of gigahertz per channel — which is still vastly faster than any electrochemical membrane.

## Why latency scales with depth × spike time

For a feedforward spiking network, throughput and *inference latency* are distinct quantities. Latency is the time from presenting an input to reading a decision, and in a spiking network it is dominated by the need to propagate activity through the layers: each layer must integrate its inputs and emit a spike before the next can respond. To first order,

$$L \approx N_{layers}\times \tau_{spike} + \textstyle\sum \tau_{prop}, \tag{1}$$

where $\tau_{spike}$ is the per-layer integrate-and-fire time and $\tau_{prop}$ the interconnect propagation delay. On a photonic chip the propagation term is small — light crosses a millimetre of waveguide in about 10 ps — so the depth-times-spike-time product dominates, and shrinking $\tau_{spike}$ shrinks latency almost proportionally.

## Worked Example: end-to-end latency of a 10-layer network

Consider a spiking network of $N_{layers} = 10$ stages, and estimate its inference latency from (1) using the per-layer spike time characteristic of each technology (taking the propagation term as negligible on chip).

- **Biological**, $\tau_{spike}\approx 1$ ms:  $L \approx 10\times 1\ \text{ms} = 10\ \text{ms}$.
- **Electronic neuromorphic**, $\tau_{spike}\approx 1\ \mu\text{s}$:  $L \approx 10\times 1\ \mu\text{s} = 10\ \mu\text{s}$.
- **Photonic**, $\tau_{spike}\approx 10$ ps:  $L \approx 10\times 10\ \text{ps} = 100\ \text{ps}$.

The photonic network reaches a decision in about 100 ps, versus ~10 µs for the electronic one and ~10 ms for the biological substrate:

$$\frac{L_{bio}}{L_{phot}} = \frac{10\ \text{ms}}{100\ \text{ps}} = 10^{8}, \qquad \frac{L_{elec}}{L_{phot}} = \frac{10\ \mu\text{s}}{100\ \text{ps}} = 10^{5}.$$

The ~$10^{8}$ ratio over biology matches the order-of-magnitude estimate of Nahmias et al. (2013), and a five-order-of-magnitude edge remains over microsecond-class electronic neuromorphic hardware. For context, the ~150 ms the primate visual system needs for object recognition would, at photonic spike times over the same ~10 stages, correspond to sub-nanosecond inference — the qualitative promise that motivates neuromorphic photonics (Ferreira de Lima et al. 2017; Shastri et al. 2021).

## Throughput versus latency

Latency (equation 1) measures one input's trip through the network; **throughput** measures how many inputs can be processed per second, and the two decouple. A feedforward photonic network can be *pipelined*: while layer two processes the first input, layer one already accepts the second, so the sustainable input rate is limited not by the total depth but by the per-stage modulation bandwidth — tens of gigahertz per channel. WDM compounds this, carrying many independent data streams on distinct wavelengths through the same physical hardware at once. A photonic layer can therefore sustain input rates orders of magnitude above an electronic one even before the per-spike latency advantage above is counted, which is why photonic accelerators are so often quoted in operations per second rather than in latency alone.

## Caveats

Raw spike speed is necessary but not sufficient. The end-to-end latency of a *deployed* system also includes getting data into and out of the optical domain (electrical-to-optical modulation and detection) and any optical-electronic-optical conversions inside the network; these can dominate if the architecture is not carefully all-optical or if the input/output path is slow. The speed advantage is real and large, but it is realized only when the surrounding energy and interconnect problems — the subjects of the next two subsections — are solved together with it (Prucnal & Shastri 2017).

## References

Nahmias, M.A., Shastri, B.J., Tait, A.N. & Prucnal, P.R. (2013). "A leaky integrate-and-fire laser neuron for ultrafast cognitive computing." *IEEE J. Sel. Top. Quantum Electron.*, 19(5), 1800212.

Ferreira de Lima, T., Shastri, B.J., Tait, A.N., Nahmias, M.A. & Prucnal, P.R. (2017). "Progress in neuromorphic photonics." *Nanophotonics*, 6(3), 577–599.

Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D. & Prucnal, P.R. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15(2), 102–114.

Prucnal, P.R. & Shastri, B.J. (2017). *Neuromorphic Photonics*. CRC Press.
