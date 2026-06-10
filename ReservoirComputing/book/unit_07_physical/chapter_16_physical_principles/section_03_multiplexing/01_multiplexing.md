# Time-Multiplexing and Virtual Nodes

## The Core Trick

The time-multiplexing scheme is the key enabling innovation of single-node physical reservoir computing [Appeltant et al. 2011]. Its essence can be stated concisely: one physical degree of freedom, driven by a delayed feedback loop with input masking, behaves like an $N$-node reservoir network. The trick exploits the separation of timescales between the physical response time $\tau$ (nanoseconds for optical systems), the virtual node interval $\theta$ (sub-nanosecond to microsecond), and the input symbol period $\tau_R = N\theta$ (microseconds to milliseconds).

This separation means that within a single input symbol period, the physical system passes through $N$ distinct "states" — one per virtual node — each influenced by both the current masked input and the state $\tau_R$ ago (same node, previous input step) via the delay feedback. The physical layer is the analog of the reservoir weight matrix in a software ESN; the delay feedback is the analog of the recurrent connections.

## The Appeltant 2011 Setup

Appeltant et al. [2011] implemented this scheme using a single opto-electronic nonlinear node with an electronic delay feedback loop. The key parameters were:

| Parameter | Value |
|-----------|-------|
| Delay length $\tau_R$ | 77.6 ns |
| Number of virtual nodes $N$ | 400 |
| Node interval $\theta = \tau_R/N$ | 0.194 ns |
| Input symbol rate | $1/\tau_R \approx 12.9$ MHz |
| Physical response time $\tau$ | $\sim 240$ ps |

The physical nonlinearity was provided by a Mach–Zehnder modulator (MZM) with transfer function $f(x) = \cos^2(x)$. The delay loop was implemented electronically, allowing precise control of the delay length.

With 400 virtual nodes and a delay of 77.6 ns, the system processes each input symbol in 77.6 ns and provides 400 sample points (virtual node states) for the readout. At 12.9 MHz input rate, this corresponds to processing $12.9 \times 10^6$ symbols per second — orders of magnitude faster than a simulated ESN on a conventional CPU.

## Virtual Node Definition

The $i$-th virtual node state for input $t$ is the physical state $x(t')$ sampled at time $t' = t \cdot \tau_R + i \cdot \theta$:

$$x_i^{(t)} \equiv x(t \cdot \tau_R + i \cdot \theta), \quad i = 1, \ldots, N.$$

The governing equation is the delay differential equation:

$$\tau \dot{x}(t') + x(t') = f\!\left(\eta x(t' - \tau_R) + \varepsilon m_i u_t\right), \quad t' \in [t\tau_R + (i-1)\theta, t\tau_R + i\theta),$$

where $m_i$ is the mask value for node $i$. Note that within the $i$-th sub-interval, the mask value is constant and determined by node index $i$ and input symbol index $t$ [Appeltant et al. 2011].

## Equivalence to Ring Reservoir

The delay-feedback virtual node network is equivalent to a ring-structured ESN. To see this, approximate the delay differential equation by the discrete map (valid when $\tau \ll \theta$):

$$x_i^{(t)} \approx f\!\left(\eta x_{i-1}^{(t)} + \varepsilon m_i u_t\right),$$

where $x_0^{(t)} \equiv x_N^{(t-1)}$ (the wrap-around connection from the last node of the previous period to the first node of the current period via the delay line). This is a ring reservoir with nearest-neighbor coupling, input scaling $\varepsilon \mathbf{m}$, and feedback gain $\eta$ [Larger et al. 2012].

The ring structure means that virtual node $i$ is connected only to virtual node $i-1$ (with coupling $\eta$) and to the input (with coupling $\varepsilon m_i$). This is a special case of the general random reservoir: the connectivity is highly structured (ring), whereas a simulated ESN has random all-to-all connectivity. The ring structure provides weaker mixing than a full random reservoir, but it is sufficient for most tasks and is the natural structure imposed by the single-delay implementation.

## Speed Advantage

The primary motivation for time-multiplexed physical reservoirs is speed. All $N$ virtual nodes are processed within one physical response period $\tau_R$. For an optical system with $\tau_R = 77.6$ ns and $N = 400$, the effective computation is 400 node updates in 77.6 ns — a throughput of $400 / 77.6 \text{ ns} \approx 5 \times 10^9$ node updates per second. A simulated ESN with $N = 400$ on a CPU processes approximately $10^6$–$10^7$ node updates per second for serial computation. The physical system is $10^2$–$10^3$ times faster.

This speed advantage makes physical reservoir computing competitive for real-time applications such as radar signal processing, optical communications channel equalization, and high-frequency financial prediction [Appeltant et al. 2011].

## Mask Design and Node Diversity

The mask $\mathbf{m} = [m_1, \ldots, m_N]$ is the primary design parameter for ensuring diversity among virtual nodes. Two important design principles: (1) the mask must break the symmetry of the ring topology — without a mask, all nodes would receive the same input and produce identical states; (2) the mask values should be chosen to cover the input space efficiently.

Random binary masks ($m_i \in \{-1, +1\}$ i.i.d.) are the standard choice and provide good average performance across tasks. Optimized masks (selected by cross-validation or genetic algorithms) can improve performance on specific tasks but require additional design effort [Larger et al. 2012].

---

## References

- Appeltant, L., Soriano, M. C., Van der Sande, G., Danckaert, J., Massar, S., Dambre, J., ... & Fischer, I. (2011). Information processing using a single dynamical node as complex artificial neural network. *Nature Communications*, 2(1), 468.
- Larger, L., Soriano, M. C., Brunner, D., Appeltant, L., Gutiérrez, J. M., Pesquera, L., ... & Fischer, I. (2012). Photonic information processing beyond Turing: An optoelectronic implementation of reservoir computing. *Optics Express*, 20(3), 3241–3249.
