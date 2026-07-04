# Subsection 13.4.2: Single-Node Photonic Reservoir

## Orientation

The single most consequential idea in photonic reservoir computing is that you do not need a network of nonlinear nodes at all — you need *one*, plus a delay. Appeltant et al. (2011) showed that a single nonlinear dynamical node with a delayed feedback loop, sampled in time, behaves as a network of many coupled nodes. This "virtual node" construction reduced a recurrent optical network to a modulator and a spool of fiber, and that minimal hardware went on to set most of the field's speed and accuracy records. This subsection develops the time-multiplexing trick, its optoelectronic and all-optical realizations, and the landmark results.

---

## 13.4.2.1 Time-Multiplexing: One Node Becomes Many

Consider a single nonlinear node whose output is fed back to its input through a delay line of length $\tau$. Partition the delay interval into $N$ equal slots of duration

$$\theta = \tau / N,$$

and treat the node's state at each slot as a distinct **virtual node**. Because the node has a finite response time, its value in one slot depends on its value in neighboring slots and, through the loop, on the value one full delay $\tau$ earlier — exactly the recurrent, neighbor-coupled connectivity a spatial reservoir would provide, now laid out along the time axis. A single physical device thus emulates an $N$-node network, its instantaneous connectivity fixed by the interplay of node dynamics and loop delay.

The virtual nodes must be driven differently or they would all see the same input and carry redundant information. This is the job of the **input mask**: a piecewise-constant, randomly valued function held for $\theta$ per slot that multiplies the input sample before injection, assigning each virtual node its own fixed input weight — the time-domain equivalent of the random $\mathbf{w}_{\text{in}}$ of Subsection 13.4.1. One input sample is held for a full delay period $\tau$, masked across the $N$ slots, and the resulting length-$N$ state vector, read at the slot spacing $\theta$, is the reservoir state passed to the linear readout.

## 13.4.2.2 Optoelectronic and All-Optical Realizations

Two hardware families implement the scheme:

- **Optoelectronic.** A Mach–Zehnder intensity modulator supplies the $\sin^2/\cos^2$ nonlinearity, a fiber spool supplies the delay $\tau$, and a photodiode closes the loop electronically. The nonlinearity lives in the modulator transfer function, the memory in the fiber. Larger et al. (2012) and Paquot et al. (2012) independently built this architecture and matched or beat digital ESNs on the standard benchmarks — the results that proved time-delay photonic reservoirs were competitive, not merely cute.
- **All-optical.** A semiconductor laser (or SOA) with optical feedback provides both nonlinearity and delay in the optical domain, with no electronic conversion in the loop. Duport et al. (2012) demonstrated an all-optical reservoir on this principle, and Brunner et al. (2013) used the transient states of a feedback laser for **parallel photonic information processing at gigabyte-per-second data rates**, performing several tasks simultaneously on one physical node.

## 13.4.2.3 Worked Example: Sizing a Time-Delay Reservoir

Take an optoelectronic loop with delay $\tau = 1\ \text{ns}$ and choose $N = 50$ virtual nodes. The node spacing is

$$\theta = \frac{\tau}{N} = \frac{1\ \text{ns}}{50} = 20\ \text{ps}.$$

One input symbol is processed per delay period, so the input (symbol) rate is

$$r_{\text{in}} = \frac{1}{\tau} = 1\ \text{GSymbol/s},$$

and the reservoir presents a state vector of dimension $N = 50$ to the readout each period. The demand this places on the hardware is set by $\theta$: the modulator must be driven, and the photodiode/ADC must resolve, states spaced $20\ \text{ps}$ apart, i.e. bandwidth $\sim 1/\theta = 50\ \text{GHz}$-class electronics. This is the architecture's central trade-off. At fixed $\tau$, doubling the node count to $N = 100$ halves the spacing to $\theta = 10\ \text{ps}$ and doubles the required bandwidth to $\sim 100\ \text{GHz}$; conversely, lengthening $\tau$ to add nodes at fixed $\theta$ lowers the input rate $1/\tau$ proportionally. Reservoir size, processing bandwidth, and input throughput are three faces of one constraint —

$$\text{bandwidth} \sim \frac{1}{\theta} = \frac{N}{\tau},$$

and the modulator/detector speed is what ultimately caps the product of node count and symbol rate.

## 13.4.2.4 The Speed Records

The time-delay architecture's economy of hardware freed its builders to push bandwidth, and the records followed. Larger et al. (2017, *Physical Review X*) drove a time-delay optoelectronic reservoir to spoken-word classification at **roughly one million words per second**, exploiting the GHz-scale symbol rates the single-node loop makes natural — a throughput no spatially-wired reservoir of comparable complexity has matched. Brunner et al.'s gigabyte-per-second parallel processing (2013) and the founding demonstrations of Larger, Paquot, and Duport (2012) together establish the pattern: because the reservoir is one device rather than a fabricated network, its performance rides directly on modulator and detector bandwidth, and those have improved relentlessly. The single-node time-delay reservoir remains the benchmark-setting architecture of the field precisely because it converts the entire cost of a recurrent network into a length of fiber and a fast transducer.

---

## References

[1] Appeltant, L., Soriano, M.C., Van der Sande, G., Danckaert, J., Massar, S., Dambre, J., Schrauwen, B., Mirasso, C.R., & Fischer, I. (2011). "Information processing using a single dynamical node as complex system." *Nature Communications*, 2, 468. [The founding paper: a single nonlinear node with delayed feedback emulates a virtual-node network — the basis of the entire subsection.]

[2] Larger, L., Soriano, M.C., Brunner, D., Appeltant, L., Gutiérrez, J.M., Pesquera, L., Mirasso, C.R., & Fischer, I. (2012). "Photonic information processing beyond Turing: an optoelectronic implementation of reservoir computing." *Optics Express*, 20(3), 3241–3249. [The optoelectronic Mach–Zehnder-and-fiber realization matching digital reservoirs on benchmark tasks.]

[3] Paquot, Y., Duport, F., Smerieri, A., Dambre, J., Schrauwen, B., Haelterman, M., & Massar, S. (2012). "Optoelectronic reservoir computing." *Scientific Reports*, 2, 287. [The independent, contemporaneous optoelectronic demonstration; jointly foundational with [2].]

[4] Brunner, D., Soriano, M.C., Mirasso, C.R., & Fischer, I. (2013). "Parallel photonic information processing at gigabyte per second data rates using transient states." *Nature Communications*, 4, 1364. [All-optical parallel processing at Gbyte/s on a single feedback laser — the high-throughput, all-optical branch.]

[5] Larger, L., Baylón-Fuentes, A., Martinenghi, R., Udaltsov, V.S., Chembo, Y.K., & Jacquot, M. (2017). "High-speed photonic reservoir computing using a time-delay-based architecture: Million words per second classification." *Physical Review X*, 7, 011015. [The speed record — spoken-word classification at ~$10^6$ words/s — that defines the architecture's ceiling.]
