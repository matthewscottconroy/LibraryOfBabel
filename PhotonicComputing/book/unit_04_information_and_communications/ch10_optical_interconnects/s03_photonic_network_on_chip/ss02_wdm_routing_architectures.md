# Subsection 10.3.2: WDM Routing Architectures

## Orientation

A photonic network-on-chip must solve a routing problem: given $N$ cores that need to exchange data, how do we configure the optical waveguides and switching elements so that core $i$ can send a message to core $j$ at maximum bandwidth with minimum energy? The answer depends on the traffic pattern, the number of wavelengths available, and the physical layout of the chip.

This subsection develops the three main architectural families for on-chip optical routing: broadcast-and-select, wavelength-routed, and token-based shared buses. Each has distinct bandwidth-energy-latency characteristics that make it suitable for different workloads.

---

## 10.3.2.1 Broadcast-and-Select

### Architecture

The simplest optical routing architecture is the *broadcast-and-select* (BAS) network. A single waveguide bus spans the chip. Every sender modulates its data onto a specific wavelength (assigned at design time). Every receiver is equipped with a filter (typically a ring resonator or microring) tuned to its designated wavelength.

When sender $A$ wants to transmit to receiver $B$, it modulates its data onto wavelength $\lambda_A$ (assigned to $A$). The modulated light propagates along the bus in both directions (or a directional bus in one direction), passes every ring-resonator filter, and is dropped by the ring tuned to $\lambda_A$ at the receiver designated for $A$.

The "broadcast" is the fact that every wavelength propagates to every receiver — the light does not care about addressing. The "select" is performed by the filters: only the designated receiver drops the wavelength.

**Transfer function**: A ring resonator drop filter has, in the undercoupled regime, a Lorentzian transmission:

$$T_{\text{drop}}(\omega) = \frac{(\kappa_e/2)^2}{(\omega - \omega_0)^2 + (\kappa_0/2 + \kappa_e/2)^2}$$

where $\omega_0$ is the resonance frequency, $\kappa_0$ is the intrinsic loss rate, and $\kappa_e$ is the coupling rate (the coupling between the waveguide and the ring). At critical coupling ($\kappa_e = \kappa_0$), 100% of the power at $\omega_0$ is dropped into the ring. At off-resonance wavelengths, the power passes through with negligible loss.

**Wavelength assignment**: For a network of $N$ communicating pairs, we need $N$ distinct wavelengths. With ring resonators at 1550 nm, typical FSR is 8 nm (for a 15 μm radius ring), and ring resonances can be placed every 0.4 nm (50 GHz DWDM grid). This gives $\sim 20$ channels per FSR, or $\sim 200$ channels across the C-band. For a chip with 64 cores, $N = 64$ wavelengths is feasible.

### Energy Analysis

The energy budget for a BAS network:

1. **Laser**: Shared comb source generates $N$ wavelengths. Power per wavelength $\approx P_{\text{total}}/N$. If $P_{\text{total}} = 100$ mW and $N = 64$, each wavelength gets 1.56 mW. At 100 Gbps modulation: $E_{\text{laser}} = P/B = 1.56\text{ mW}/100\text{ Gbps} = 15.6$ fJ/bit.

2. **Bus propagation loss**: A 1 cm bus waveguide at 2 dB/cm = 2 dB loss → 63% transmission. This costs a factor of 1.58 in effective laser power needed.

3. **Modulator**: Ring modulator energy ~5–20 fJ/bit (from Section 7.3.3).

4. **Drop filter + detector + TIA**: The ring drop filter has 0.5–1 dB insertion loss; the Ge PD + TIA require ~100–300 fJ/bit.

5. **Ring thermal stabilization**: Each ring requires ~1 mW to track thermal drift (from Section 7.3.3 analysis). For $N = 64$ rings at 100 Gbps:
   $$E_{\text{thermal}} = \frac{1 \text{ mW}}{100 \text{ Gbps}} = 10 \text{ fJ/bit per ring}$$
   With 64 modulator rings + 64 filter rings = 128 rings: $128 \times 10 = 1280$ fJ/bit.

This last item dominates. The thermal stabilization of $N$ ring resonators consumes more energy than all other optical components combined. This is the central engineering challenge of ring-resonator-based PNoC architectures.

**Total BAS energy**: $\sim 1280 + 300 + 20 + 15 \approx 1615$ fJ/bit at 100 Gbps with 64 channels.

This is *worse* than the electrical alternative, purely due to ring thermal control. The path to improvement requires either:
(a) Better ring thermal stabilization (< 100 μW per ring), or
(b) Different device physics that is thermally robust (e.g., Mach-Zehnder modulators with wider bandwidth and less thermal sensitivity, at the cost of larger footprint and higher drive power).

### Bandwidth Limitation

BAS has a fundamental bandwidth limitation: each wavelength $\lambda_i$ is assigned to one sender-receiver pair. The total network bandwidth is:

$$B_{\text{BAS}} = N_\lambda \times B_{\text{per-\lambda}}$$

For $N_\lambda = 64$ and $B_{\text{per-\lambda}} = 100$ Gbps: $B_{\text{BAS}} = 6.4$ Tbps. But this is the *aggregate* bandwidth — no single core-to-core link can use more than $B_{\text{per-\lambda}} = 100$ Gbps. If one core needs to send to another at 1 Tbps (e.g., adjacent tensor-parallel units during AI inference), BAS cannot accommodate it.

---

## 10.3.2.2 Wavelength-Routed Networks

### Architecture

A wavelength-routed network uses optical switches at junction nodes to direct specific wavelengths along specific paths. The canonical design is the *optical mesh* or *optical crossbar*, where each intersection between a horizontal and vertical waveguide contains a wavelength-selective switch.

**2D mesh routing**: Consider a $\sqrt{N} \times \sqrt{N}$ mesh of cores. At each intersection, an optical switching element (typically a ring resonator or MZI) can either:
(a) Pass the signal straight through without deflection, or
(b) Deflect the signal 90 degrees onto the perpendicular waveguide.

By programming the switches appropriately, any source-destination pair can be connected via a reconfigurable optical path.

**Wavelength-division routing**: Rather than using electronic switch programming (which requires setting and re-setting switch states), some architectures use *wavelength* to determine the route. Each wavelength is designed to follow a predetermined path through the mesh based on the passive structure. This approach — called *wavelength-division routing* — requires careful design of the ring resonator FSR and filter bandwidths but eliminates the need for active switch control during operation.

**The Torus architecture**: For all-to-all communication patterns (like all-reduce), a torus topology provides shorter average path length than a mesh. A $k \times k$ torus with wraparound connections has maximum path length $k$, vs. $2(k-1)$ for a mesh. For $N = 64$ cores in an 8×8 torus:
$$\text{Mean path length} = \frac{k}{2} = 4 \text{ hops vs. } 7 \text{ hops for mesh}$$
This matters for latency-sensitive workloads.

### Photonic Butterfly Network

The *butterfly network* is the topology of choice for collective communication operations like all-reduce. A $k$-ary $n$-fly butterfly has $N = k^n$ end-nodes and $n$ stages of $k$-way switches. For $N = 64 = 2^6$, a binary butterfly ($k=2$) has 6 stages of $N/2 = 32$ switches.

The appeal for photonic implementation: the butterfly's regular structure maps naturally onto a 2D silicon chip layout, and the all-reduce communication pattern of AI training directly implements the butterfly exchange pattern. Each stage of the butterfly corresponds to one all-reduce synchronization step.

**Bandwidth**: A photonic butterfly provides:
$$B_{\text{bisection}} = \frac{N}{2} \times B_{\text{per-link}} \times N_\lambda$$

For $N = 64$, $B_{\text{per-link}} = 100$ Gbps, $N_\lambda = 16$:
$$B_{\text{bisection}} = 32 \times 100 \times 16 = 51.2 \text{ Tbps}$$

### Optical Crossbar

The simplest $N \times N$ routing structure is the optical crossbar: $N$ input waveguides cross $N$ output waveguides, with a switch at each intersection. Any input can be connected to any output simultaneously, as long as there are no conflicts (two inputs trying to reach the same output simultaneously).

**Crossbar complexity**: $N^2$ switches for an $N \times N$ crossbar. For $N = 64$: 4,096 switches. Each ring-resonator switch has area $\sim 200$ μm² (ring + coupling gap + routing). Total area:
$$A_{\text{crossbar}} = 4096 \times 200 \text{ μm}^2 \approx 0.82 \text{ mm}^2$$

This is acceptable for a dedicated interconnect block. But the programming complexity (keeping 4,096 switches at their correct state) and the thermal control energy ($4096 \times 10 \text{ fJ/bit} \approx 40 \text{ pJ/bit}$) are prohibitive unless most switches are OFF (transparent) and only a few are ON (dropping).

The key insight for optical crossbar design: **in any non-conflicting routing, at most $N$ switches are active at any time** (one per source-destination pair). If the thermal control energy is only charged for active switches:
$$E_{\text{thermal, active}} = N \times 10 \text{ fJ/bit} = 640 \text{ fJ/bit for } N = 64$$

Still significant, but more manageable. This argues for thermally stable (but slower) switching technologies — MEMS switches (Section 7.4.2) with their zero static power are well-suited for the crossbar application.

---

## 10.3.2.3 Token-Based Shared Bus

### Architecture

The optical bus architecture provides a simpler alternative to the routing approaches above, at the cost of bandwidth. A single waveguide loops around the chip (a "photonic ring bus"), carrying data from source to destination along the ring. Sources modulate data onto the bus; destinations pick off data using drop filters tuned to their wavelength.

The challenge is arbitration: multiple sources cannot simultaneously use the same wavelength on the bus. A token-based protocol resolves this:

1. A "token" packet travels continuously around the ring.
2. A source that wants to transmit waits until the token passes.
3. It captures the token, transmits its packet, and releases the token.
4. The destination's drop filter receives the packet.

**Latency**: The mean latency for an $N$-node ring with token passing is:

$$\tau_{\text{mean}} = \frac{\tau_{\text{ring}}}{2} + \frac{N \tau_{\text{packet}}}{2 \cdot \text{utilization}}$$

For a 2 cm ring at $c/n = 2\times10^8$ m/s in Si: $\tau_{\text{ring}} = 0.1$ ns (electrical signals travel at $\sim 0.6c$ in copper, so this is comparable to an electrical 1-cm wire). Mean waiting time for N=8 cores at 50% utilization: $8 \times 0.01 \text{ ns} / (2 \times 0.5) = 0.08$ ns.

For cache coherence traffic with very short messages (<64 bytes), this latency is competitive with electrical meshes.

**Bandwidth**: The bus bandwidth is:
$$B_{\text{bus}} = \frac{B_{\text{waveguide}} \times \text{utilization}}{N}$$

For a 400 Gbps bus shared by 8 nodes at 50% utilization: $400 \times 0.5 / 8 = 25$ Gbps per node. This is insufficient for AI training all-reduce (which requires 224 Gbps per node), but may suffice for control-plane traffic.

---

## 10.3.2.4 Architecture Comparison

The three architectures occupy different positions in the bandwidth-energy-latency space:

| Architecture | Bandwidth | Energy/bit | Latency | Scalability |
|-------------|-----------|------------|---------|-------------|
| Broadcast-and-select | $N \times B_\lambda$ (limited by wavelengths) | ~1.5 pJ (ring thermal dominant) | Propagation only | Poor (ring thermal) |
| Wavelength-routed crossbar | Full bisection bandwidth | ~0.5 pJ with MEMS | Short path | Moderate ($N^2$ switches) |
| Photonic butterfly | $N/2 \times B_\lambda$ bisection | ~0.3 pJ | $\log_2 N$ hops | Good for all-reduce |
| Token bus | $B_\lambda / N$ per node | ~0.2 pJ | Ring latency + queue | Poor for high-traffic |

*Energies quoted for $N = 64$, 100 Gbps per channel, 16 WDM channels, 2025 state-of-the-art components.*

The conclusion from this comparison: there is no single PNoC architecture that dominates across all metrics. The choice depends on the workload:

- For AI training (all-reduce dominant): **photonic butterfly** or wavelength-routed network with torus topology
- For general-purpose cache coherence (irregular, low latency): **electrical NoC wins** at current component energy levels
- For high-bandwidth point-to-point links between fixed pairs (e.g., CPU-GPU in a heterogeneous chip): **broadcast-and-select** with fixed wavelength assignments, if ring thermal control is solved

The most honest assessment: PNoC is compelling for a specific use case — high-bandwidth, regular, predictable communication patterns between a moderate number of ports — and not yet competitive for general-purpose processor networking.

---

## References

[1] Shacham, A., et al. (2008). "Photonic NoC for DMA communications in chip multiprocessors." *Hot Interconnects 2007*. [Early photonic NoC architecture proposal.]

[2] Vantrease, D., et al. (2008). "Corona: System implications of emerging nanophotonic technology." *ACM ISCA 2008*. [The Corona paper; one of the first comprehensive photonic NoC architecture analyses with realistic energy models. Highly cited.]

[3] Kurian, G., et al. (2010). "ATAC: A 1000-core cache-coherent processor with online dynamic management and a scalable on-chip optical network." *ACM PACT 2010*. [Photonic torus network for 1000-core processor; demonstrates scalability limits of ring-resonator-based approaches.]

[4] Joshi, A., et al. (2009). "Silicon photonic interconnects for high-performance computing." *IEEE Micro*, 29(4), 66–76. [Comprehensive comparison of photonic NoC architectures.]

[5] Beamer, S., et al. (2010). "Re-architecting DRAM memory systems with monolithically integrated silicon photonics." *ACM ISCA 2010*. [Shows that the dominant application for PNoC may be processor-to-memory rather than core-to-core.]
