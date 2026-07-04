# 16.2.3 In-Memory Photonic Computing

## The von Neumann bottleneck

A conventional computer keeps its memory (DRAM) physically separate from its arithmetic units. For neural-network inference this separation is expensive: the workload is dominated not by the multiplications themselves but by *moving the weights* from memory to the processor. Every multiply-accumulate (MAC) requires a weight to be fetched across a bus, and this data movement dominates both the energy and the latency of the computation — the "von Neumann bottleneck," or "memory wall." Wright et al. (2013) argued that nanoscale phase-change memory offers a route *beyond* von Neumann: perform the computation inside the very devices that store the data, so that no weight is ever moved.

## Compute where the data lives

A PCM synapse *is* its weight — the number is a structural property of the cell, frozen into its crystalline fraction (§16.2.1). When probe light traverses the cell, it emerges multiplied by the stored transmission $T$. The multiply is not a separate operation preceded by a memory read; it **is** the propagation. There is no fetch, no bus, no word line: the weight sits passively in the light path and is applied at the speed of light, at zero standby power. Ríos et al. (2019) demonstrated exactly this — scalar multiplication performed *in memory* on a photonic platform, in which light passing a PCM cell is automatically and non-volatilely weighted. Because the weights never move and never need refreshing, the architecture is ideally matched to deployed, **fixed-weight inference**: program the network once, then run it at optical speed.

## From scalar multiply to multiply-accumulate

A neuron computes a weighted sum $y = \sum_i w_i x_i$. Photonics performs the *accumulate* almost for free. Encode each input $x_i$ on a distinct wavelength $\lambda_i$, pass each through its own PCM cell of transmission $T_i$, and combine all wavelengths onto one waveguide that terminates on a photodetector. Because different wavelengths do not interfere, the detector — which responds to total optical power — sums the weighted channels automatically, delivering a photocurrent proportional to $\sum_i T_i x_i$. This wavelength-multiplexed, detector-summed dot product uses the same physical mechanism as the broadcast-and-weight matrix multiplier developed for matrix-vector products in Unit V (§12.4); the distinction here is that the weights are *non-volatile PCM cells* rather than continuously tuned microrings. (A single PCM cell provides a non-negative weight, $0 \le T \le T_\text{max}$; signed weights require balanced or differential detection, exactly as in §12.4.)

## The photonic tensor core

Feldmann et al. (2021) scaled this idea into a fully parallel matrix engine: a $4 \times 4$ array of PCM cells holding a weight matrix, with the input vector supplied on many wavelengths drawn from a **Kerr frequency comb** (a chip-scale microresonator comb; cf. Unit III) and the outputs collected by on-chip **germanium photodetectors**. Broadcasting the WDM inputs across the PCM matrix computes a full matrix-vector product in a single pass, and by streaming data through the same matrix the core performs parallel convolutions. The reported throughput reaches into the trillions of MAC operations per second — the TOPS-scale regime — with the whole matrix held non-volatilely, so no weight reloading interrupts the compute. It is a concrete demonstration that an entire linear layer can be evaluated in the time it takes light to cross the chip.

## Worked Example: an optical MAC and its cost

*Compute the four-element dot product $y = \sum_{i=1}^{4} T_i x_i$ where the inputs are optical powers $x = [1.00,\ 0.50,\ 0.80,\ 0.20]$ mW on four wavelengths and the PCM weights are transmissions $T = [0.90,\ 0.30,\ 0.60,\ 0.75]$. The detector responsivity is $R = 0.9$ A/W.*

Each channel emerges with power $T_i x_i$:
$$[\,0.90,\ 0.15,\ 0.48,\ 0.15\,]\ \text{mW}.$$
The photodetector sums the (mutually incoherent) channels:
$$P_\text{out} = 0.90 + 0.15 + 0.48 + 0.15 = 1.68\ \text{mW}.$$
The output photocurrent — the analog dot product — is
$$I = R\,P_\text{out} = (0.9\ \text{A/W})(1.68\times10^{-3}\ \text{W}) = 1.51\ \text{mA}.$$
The entire MAC is produced in one shot by light propagating through the cells and landing on the detector.

*Latency and energy versus DRAM.* The weights are traversed at the speed of light. The time-of-flight through a $L = 5\ \mu\text{m}$ cell with group index $n_g \approx 4$ is
$$t = \frac{n_g L}{c} = \frac{4 \times 5\times10^{-6}\ \text{m}}{3\times10^{8}\ \text{m/s}} \approx 6.7\times10^{-14}\ \text{s} \approx 0.07\ \text{ps},$$
so the weight is applied essentially instantaneously. In a digital engine each of the four weights must instead be *fetched* from memory before any arithmetic begins. A DRAM access costs on the order of tens of picojoules and tens of nanoseconds (order-of-magnitude figures), so four fetches precede four multiplies and the adds. In the photonic cell the weight is never moved: the probe light performs the read and the multiply simultaneously, so the dominant cost of digital inference — data movement — is eliminated, and the weight-access latency drops from ~tens of ns to sub-ps, some four to five orders of magnitude. This collapse of the memory-access cost, not a faster multiplier, is the essence of in-memory photonic computing.

---

## References

- Ríos, C., Youngblood, N., Cheng, Z., Le Gallo, M., Pernice, W.H.P., Wright, C.D., Sebastian, A. & Bhaskaran, H. (2019). "In-memory computing on a photonic platform." *Science Advances*, 5(2), eaau5759.
- Wright, C.D., Hosseini, P. & Diosdado, J.A.V. (2013). "Beyond von-Neumann computing with nanoscale phase-change memory devices." *Advanced Functional Materials*, 23(18), 2248–2254.
- Feldmann, J. et al. (2021). "Parallel convolutional processing using an integrated photonic tensor core." *Nature*, 589(7840), 52–58.
