# 15.2.2 Fan-Out and Broadcast with WDM

## The connectivity problem

A neuron is defined as much by its wiring as by its dynamics: a cortical cell may drive thousands of targets and receive from thousands of sources. Reproducing this dense, high-**fan-out** connectivity is where conventional electronics struggles and where photonics has a structural advantage.

**Fan-out** in optics is passive. A single optical output can be divided among many destinations by a splitter — a Y-branch tree or a multimode-interference coupler — with no active driver and, crucially, no bandwidth penalty from the load: an ideal $1{:}N$ splitter simply divides the power $N$ ways. The unavoidable cost is the **splitting loss**,

$$L_{split} = 10\log_{10} N \ \ \text{dB}, \tag{1}$$

3 dB for each doubling of the fan-out, plus a small excess loss per stage.

## WDM: many synapses on one waveguide

The second ingredient is **wavelength-division multiplexing (WDM)**. Because independent optical signals at different wavelengths propagate through the same linear waveguide without interacting, a single physical bus can carry $N$ separate channels, one per wavelength $\lambda_i$. In a neuromorphic setting each neuron is assigned its own emission wavelength; the shared bus then **broadcasts** every neuron's output to every receiver simultaneously. At each receiving node, a bank of tuned add–drop microring resonators — one ring resonant with each $\lambda_i$ — taps off a controllable fraction of each channel, and that drop fraction sets the corresponding **synaptic weight**. Balanced photodetection of the drop versus through ports produces a *signed* ($\pm$) weighted sum, which drives the node's nonlinear spiking element.

This is the **broadcast-and-weight** architecture. Its matrix–vector-product form was developed in Unit V (§12.4); here the emphasis is its use as a *spiking* interconnect that wires excitable-laser neurons into a network, taken up in detail in Chapter 16 (§16.3). Tait et al. (2014) proposed the protocol for photonic spike processing, and Tait et al. (2017) demonstrated a 49-node silicon-photonic weight-bank network — the point to carry forward is simply that WDM turns one waveguide into $N$ independent, individually weighted synaptic channels.

How many wavelengths actually fit is set by device physics rather than by principle. The microrings of a weight bank repeat their resonances every **free spectral range (FSR)**, so all $N$ channels must be packed within one FSR while keeping enough spacing that inter-channel crosstalk and thermal tuning drift stay manageable; representative demonstrations use tens of channels. Widening the FSR by shrinking the rings buys spectral room but reduces the achievable tuning range, so channel count, crosstalk, and thermal control trade off against one another. This — not any bandwidth ceiling — is what bounds how many synapses a single broadcast bus can host.

## Contrast with electronic fan-out

In electronics, driving one output to $N$ inputs means charging the combined load capacitance of $N$ gates and the wires between them. Each added branch increases the capacitance, so the driver must be larger and the RC charging time grows; long interconnects require repeater buffers, each costing energy, area, and delay. Fan-out therefore trades directly against speed and power, and total wire count grows with connectivity. Optical broadcast sidesteps both limits: the splitter imposes a static power penalty (equation 1) but no dynamic RC slowdown, and WDM multiplies channel count on a *single* waveguide instead of adding wires. The scaling burden moves from bandwidth to **optical power budget**.

## Worked Example: power budget of a broadcast bus

Let a neuron's output at power $P_{in}$ fan out to $N = 32$ receiving synapses over a WDM broadcast bus, and check that each receiver gets enough light.

Take $P_{in} = 1\ \text{mW} = 0\ \text{dBm}$. The ideal splitting loss for $N = 32$ is

$$L_{split} = 10\log_{10} 32 = 15.1\ \text{dB}.$$

The signal also passes a splitter tree of $\log_2 32 = 5$ stages, the microring weight bank, and routing waveguides; lump these excess and insertion losses as $\approx 5\ \text{dB}$. The power reaching each receiver is then

$$P_{rx} = 0\ \text{dBm} - 15.1\ \text{dB} - 5\ \text{dB} \approx -20\ \text{dBm} = 10\ \mu\text{W}.$$

If the photodetector and receiver need at least about $-20$ dBm for adequate signal-to-noise at the target bandwidth, the budget just closes at $N = 32$. Doubling the fan-out to $N = 64$ adds 3 dB of splitting loss, dropping each channel to $\approx -23$ dBm ($5\ \mu\text{W}$) and forcing either more input power or a more sensitive receiver.

Turned around, the maximum fan-out for a given budget is

$$N_{max} = 10^{(P_{in} - P_{sens} - L_{excess})/10}.$$

With a stronger source $P_{in} = +10\ \text{dBm}$ (10 mW), receiver sensitivity $P_{sens} = -20\ \text{dBm}$, and $L_{excess} = 6\ \text{dB}$, the available splitting budget is $10-(-20)-6 = 24\ \text{dB}$, giving

$$N_{max} = 10^{24/10} \approx 2.5\times 10^{2},$$

on the order of 250-way fan-out. Laser power and receiver sensitivity — not bandwidth — thus set the connectivity ceiling of a broadcast-and-weight network, exactly the opposite of the RC-limited electronic case.

## References

Tait, A.N., Nahmias, M.A., Shastri, B.J. & Prucnal, P.R. (2014). "Broadcast and weight: an integrated network for scalable photonic spike processing." *Journal of Lightwave Technology*, 32(21), 4029–4041.

Tait, A.N., de Lima, T.F., Zhou, E., Wu, A.X., Nahmias, M.A., Shastri, B.J. & Prucnal, P.R. (2017). "Neuromorphic photonic networks using silicon photonics weight banks." *Scientific Reports*, 7, 7430.

Prucnal, P.R. & Shastri, B.J. (2017). *Neuromorphic Photonics*. CRC Press.
