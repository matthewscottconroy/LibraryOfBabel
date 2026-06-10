# Section 10.2: Data Center Networks

## What This Section Is About

The previous section established the physics of the optical interconnect problem: why electrical wires fail at high bandwidth and distance, why optics is the natural solution, and what the energy and bandwidth targets must be. This section zooms out from the individual link to the network that the links compose — the data center.

A modern hyperscale data center is among the largest engineered systems in human history. A single facility may house hundreds of thousands of servers consuming hundreds of megawatts of power, connected by petabits of aggregate switching bandwidth. The architecture of this network determines how efficiently computation can be distributed across those servers — and for artificial intelligence training, the network has become a first-order constraint on what computations are even tractable.

The relevance to photonic computing goes beyond the obvious fact that fiber is the medium of the interconnect. The *topology* of the network determines how many optical connections are needed and what their reach requirements are. The *switching architecture* determines whether the optical signals must be converted back to electrical at each hop or can pass through as light. The *traffic patterns* of AI workloads — specifically, the all-to-all communication patterns of large-scale model training — create bandwidth demands that are rewriting the economics of data center design and creating opportunities for optical technologies that would otherwise be impractical.

This section covers two subsections:

**Subsection 10.2.1: Leaf-Spine Topology** develops the mathematics of fat-tree network design, the bandwidth requirements of modern AI training clusters, and why the all-reduce communication pattern that dominates distributed learning creates specific challenges for conventional electronic switching.

**Subsection 10.2.2: Optical Circuit Switching** explores the physics and engineering of optical switching fabrics — MEMS mirror arrays, liquid crystal on silicon, wavelength-selective switches — and the architectural question of whether the network can be reconfigured fast enough to adapt to AI training traffic patterns in real time.

The central tension of this section is between the optical interconnect's strength (high bandwidth, low energy per bit) and its weakness (inability to buffer photons). Electronic packet switches derive much of their value from their ability to buffer packets and adapt to bursty traffic; optical circuit switches cannot buffer at all, and their value depends entirely on whether traffic is predictable enough that you can configure the circuit before the data arrives. AI training traffic, as we will see, is remarkably well-suited to this model.

---

*References for the section introduction are given within the subsections.*
