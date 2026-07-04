# Subsection 13.4.3: Integrated Photonic Reservoir

## Orientation

The time-delay reservoir of Subsection 13.4.2 lays its network out along the time axis; the integrated reservoir lays it out in *space*, as a physical web of waveguides on a chip. The move trades the fiber spool and fast transducer for a compact, wafer-scale, potentially passive device — and in its purest form dispenses with the internal nonlinearity entirely, letting the readout photodetector supply the only $|E|^2$ the network needs. This subsection covers the passive silicon reservoir that made the case, its nonlinear alternatives, the large free-space scattering reservoirs that reach $10^4$ nodes, and the loss-versus-scale trade-off that governs them all.

---

## 13.4.3.1 The Passive Silicon Reservoir

Vandoorne et al. (2014) demonstrated reservoir computing on a silicon photonics chip using a network with **no active element inside the reservoir at all**: a mesh of waveguides connected by passive splitters and combiners in a "swirl" topology, through which the input light propagates, splits, recombines, and interferes. The reservoir dynamics are entirely linear and passive — the nodes are combination points, the connections are waveguides with their propagation phases and delays — and the *only* nonlinearity in the system is the $|E|^2$ of the readout photodetectors reading each node. With just **16 nodes** the chip performed Boolean logic operations (including the temporally nontrivial XOR) and spoken-digit / bit-header recognition, at accuracies competitive with software reservoirs.

The architectural payoff is threefold. The reservoir consumes **no power** — it is passive glass-on-silicon, dissipating only propagation loss, so there is no per-node pump budget of the kind that burdens SOA- or laser-based reservoirs (Subsection 13.2.3). It is **CMOS-compatible**, fabricated in a standard silicon photonics flow. And light propagates through it at **THz-class optical bandwidth**, so the reservoir's intrinsic speed is set by device delays, not by any electronic time constant. The nonlinearity being deferred to detection is not a compromise but a feature: it is exactly the free quadratic nonlinearity that Subsection 13.2.3 identified as the field's most reliable, placed where it costs nothing.

## 13.4.3.2 Worked Example: The Loss Budget of a Passive Swirl

Passivity has a price, and it is loss. In a silicon-wire waveguide the group index is $n_g \approx 4.2$, so light travels at $v = c/n_g \approx 7.1\times 10^{7}\ \text{m/s}$. With inter-node waveguide segments of order $L \approx 1\ \text{mm}$, the per-hop delay is

$$\tau_{\text{hop}} = \frac{L}{v} = \frac{1\times 10^{-3}}{7.1\times 10^{7}} \approx 14\ \text{ps}.$$

A 16-node swirl with a network diameter of $\sim\!5$ hops therefore spreads path delays over $\Delta\tau \approx 5 \times 14\ \text{ps} \approx 70\ \text{ps}$. That spread *is* the reservoir's fading memory — the interference of many differently-delayed copies of the input — and it sets the usable input rate at $\sim\!1/\Delta\tau \approx 14\ \text{GSymbol/s}$, comfortably in the regime where the reservoir integrates temporal context without smearing consecutive symbols.

The scaling limit is the loss budget. Each node (a splitter/combiner plus its waveguide segment) costs on the order of $0.5$–$1\ \text{dB}$ — dominated by splitter excess loss, with $\sim\!2\ \text{dB/cm}$ propagation adding $\sim\!0.2\ \text{dB}$ per millimeter. A worst-case path through $\sim\!8$ nodes of the 16-node device thus loses $\sim\!4$–$8\ \text{dB}$, easily afforded from a milliwatt input against microwatt-scale detection. But because the reservoir is passive there is **no gain to replenish it**, so the tolerable path length — and hence the node count — is capped by the fixed budget between input power and detector sensitivity. Scaling to $N \sim 100$ nodes implies paths of $\sim\!20$ nodes and $\gtrsim\!15$–$20\ \text{dB}$ of loss to the farthest readout, which is the wall that limits how large a *passive* integrated reservoir can grow.

## 13.4.3.3 Nonlinear and Large-Scale Alternatives

Where passivity's loss wall bites, active reservoirs restore gain at the cost of power. Networks of **coupled semiconductor lasers** and of **coupled microring resonators** place a nonlinearity (and, in the laser case, gain) at every node, supporting richer dynamics and larger effective networks than a passive swirl, at the expense of pump power and fabrication uniformity across the array.

A different escape from the on-chip node limit is to leave the chip entirely. Rafayelyan et al. (2020, *Physical Review X*) built a **free-space optical reservoir** in which a spatial light modulator encodes the reservoir state across its pixels, a diffractive/multiple-scattering step mixes them, and a camera reads the intensity — the $|E|^2$ nonlinearity again — with the readout fed back to close the loop. Because the node count is the pixel count, this architecture reaches $\sim\!10^4$ nodes, and the authors used it to predict the evolution of a spatiotemporally chaotic system (Kuramoto–Sivashinsky dynamics) — a scale of reservoir unreachable on any integrated chip, bought with the bulk and slower frame-rate of free-space SLM/camera hardware.

## 13.4.3.4 The Trade-off Space

The integrated reservoir occupies a design triangle whose corners pull against one another. **Node count** wants large networks, but on a passive chip it is capped by cumulative **on-chip loss**; adding gain lifts the cap but reimposes the per-node power budget the passive design was meant to escape. **Readout access** demands a detector tap at every node whose state matters, and taps themselves cost loss and area — so the *readable* dimension can be smaller than the physical one. And any coherent reservoir depends on **optical coherence** being maintained across the path-delay spread, constraining laser linewidth and thermal stability. Free-space scattering reservoirs relax the on-chip loss and readout-tap constraints (a camera reads all $10^4$ nodes at once) but sacrifice integration and speed. There is, as yet, no single point that is best on every axis; the right reservoir is the one whose corner of this triangle matches the task's demands for size, bandwidth, and power — which is, fittingly, the same "let the physics be itself" logic that motivated the whole section.

---

## References

[1] Vandoorne, K., Mechet, P., Van Vaerenbergh, T., Fiers, M., Morthier, G., Verstraeten, D., Schrauwen, B., Dambre, J., & Bienstman, P. (2014). "Experimental demonstration of reservoir computing on a silicon photonics chip." *Nature Communications*, 5, 3541. [The passive 16-node silicon swirl with detector-only nonlinearity — the central demonstration of this subsection.]

[2] Rafayelyan, M., Dong, J., Tan, Y., Krzakala, F., & Gigan, S. (2020). "Large-scale optical reservoir computing for spatiotemporal chaotic systems prediction." *Physical Review X*, 10, 041037. [The free-space SLM/scattering reservoir reaching $\sim\!10^4$ nodes for spatiotemporal chaos prediction — the large-scale alternative.]

[3] Brunner, D., Soriano, M.C., Mirasso, C.R., & Fischer, I. (2013). "Parallel photonic information processing at gigabyte per second data rates using transient states." *Nature Communications*, 4, 1364. [The high-speed active-reservoir counterpoint to passive integration, referenced for the gain-versus-loss trade-off.]

[4] Van der Sande, G., Brunner, D., & Soriano, M.C. (2017). "Advances in photonic reservoir computing." *Nanophotonics*, 6(3), 561–576. [The survey placing integrated, coupled-laser, microring, and free-space reservoirs in one comparative frame — the map of the trade-off space of Subsection 13.4.3.4.]
