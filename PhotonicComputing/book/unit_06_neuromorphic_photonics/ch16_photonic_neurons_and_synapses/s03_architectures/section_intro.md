# Section 16.3: Photonic Spiking Neural Network Architectures

The previous two sections built the two elementary devices of a neuromorphic photonic system in isolation: the excitable photonic neuron (§16.1), which converts an integrated optical stimulus into an all-or-nothing picosecond spike, and the photonic synapse (§16.2), which imposes a programmable, non-volatile weight on an optical signal. A neuron and a synapse are not a network. This section addresses the wiring problem: how do we interconnect many photonic neurons through many weighted synapses on a single chip, so that each neuron's spike train reaches every other neuron, appropriately weighted?

The dominant answer is *broadcast-and-weight*, an architecture in which each neuron radiates its output on a distinct optical carrier wavelength $\lambda_i$, all wavelengths share a single broadcast waveguide, and each receiving neuron selects and weights the channels it needs with a bank of tunable microring resonators. Wavelength-division multiplexing (WDM) turns one physical bus into $N$ logically independent connections, and passive optical fan-out supplies the connectivity a spiking network demands.

We introduced broadcast-and-weight in Unit V (§12.4) as a way to compute a matrix–vector product $\mathbf{y}=W\mathbf{x}$. The physics of the weight bank is identical here, but the payload differs: there the inputs were static analog amplitudes; here they are *spike trains*, and the weighted sum drives an excitable element that itself produces spikes. The same silicon photonic substrate becomes a spike-processing network rather than a linear-algebra accelerator.

The two subsections that follow develop this idea. §16.3.1 details the WDM spiking network — wavelength assignment, microring weight banks, balanced photodetection for signed weights, and the limits on how many neurons one bus can carry. §16.3.2 extends it to spiking convolutional layers, where weight sharing and WDM parallelism map a convolution onto wavelength-parallel weighted sums.

---
## References

Tait, A.N., Nahmias, M.A., Shastri, B.J. & Prucnal, P.R. (2014). "Broadcast and weight: an integrated network for scalable photonic spike processing." *Journal of Lightwave Technology*, 32(21), 4029–4041.

Tait, A.N., de Lima, T.F., Zhou, E., Wu, A.X., Nahmias, M.A., Shastri, B.J. & Prucnal, P.R. (2017). "Neuromorphic photonic networks using silicon photonics weight banks." *Scientific Reports*, 7, 7430.
