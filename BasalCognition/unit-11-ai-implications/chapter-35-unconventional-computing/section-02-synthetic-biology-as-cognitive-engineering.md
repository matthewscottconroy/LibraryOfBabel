# Section 2: Synthetic Biology as Cognitive Engineering

## The Repressilator and the Birth of a Field

On January 20, 2000, *Nature* published two papers that, together, founded synthetic biology as a discipline. The first, by Michael Elowitz and Stanislas Leibler, described the repressilator — a synthetic gene circuit that generated sustained oscillations in the level of three proteins in *Escherichia coli* (Elowitz & Leibler, 2000). The second, by Timothy Gardner, Charles Cantor, and James Collins, described a genetic toggle switch — a bistable memory element implemented in gene regulation (Gardner, Cantor, & Collins, 2000). Both papers demonstrated the same thing: that gene regulatory networks could be designed from first principles, with predictable dynamic behavior, using a small number of well-characterized parts.

The repressilator consisted of three genes connected in a repression cycle: gene A represses gene B, gene B represses gene C, gene C represses gene A. Each gene encoded both a repressor protein and a reporter (green fluorescent protein), allowing the oscillations to be observed directly in living cells under a microscope. The mathematical analysis predicted that such a circuit would oscillate, given appropriate parameter values. The experiment confirmed it: cells glowed, dimmed, and glowed again, with a period of roughly 160 minutes, in a population where no natural oscillation existed. The repressilator was computing time — implementing a clock — using nothing but the transcription and translation machinery of the cell.

What Elowitz and Leibler had built was not a discovery about natural biology. It was an engineering achievement: a proof of concept that gene regulatory networks could be rationally designed to perform specific computational functions. The repressilator is the synthetic biology equivalent of Adleman's DNA computer — not, in itself, a practical technology, but a demonstration that the substrate is amenable to rational programming.

## Genetic Logic Gates

The logic gate vocabulary of digital electronics — AND, OR, NOT, NAND — can be implemented in gene regulatory networks. The simplest case is an AND gate: a gene whose expression requires the simultaneous binding of two activating transcription factors implements the logical AND of their presence. NOT is implemented by repressor proteins: a gene repressed by protein A is active when A is absent (NOT A). More complex gates require more elaborate circuit architectures, but the principle is consistent: gene expression states (protein present/absent) can encode Boolean values, and regulatory relationships (activation, repression) implement logical operations.

Synthetic biologists have systematically built and characterized collections of these genetic "parts" — standardized, characterized, reusable genetic elements that implement specific functions and can be combined like electronic components (Endy, 2005). The BioBricks standard, developed at MIT, formalized this parts-based approach, creating a registry of biological parts that researchers worldwide can access, characterize, and combine. The iGEM (International Genetically Engineered Machine) competition, also founded at MIT, has since grown into a global institution in which student teams build novel synthetic biological systems from standardized parts.

The cognitive framing of this work is not imposed from outside. Gardner and Collins explicitly described the toggle switch as a "cellular memory element" — a bistable system that maintains one of two stable states and can be switched between them by appropriate inputs. This is not a metaphor for computation; it is computation. The bistable gene circuit implements the same logic as a flip-flop in digital electronics: it is a one-bit memory that retains its state until commanded to change.

## Programming Cellular Behavior

From logic gates and memory elements, synthetic biologists have built increasingly sophisticated programs of cellular behavior. Consider a few examples:

**Cellular computation.** Tabor and colleagues (2009) built a synthetic circuit in bacteria that implemented edge detection — detecting the boundary between light and dark regions on an agar plate and generating a precisely localized pattern of pigment. The cells were programmed to sense light, communicate through quorum sensing, and integrate these signals to produce a spatially precise output. This is an image-processing algorithm implemented in living cells.

**Temporal programs.** Synthetic oscillators — building on the repressilator design — have been constructed with tunable periods and coupled to other genetic circuits to generate complex temporal programs: cells that cycle between different behavioral states, that count oscillation cycles, that time their developmental transitions precisely. These systems implement something functionally analogous to circadian clocks, though with engineered rather than evolved components.

**Decision circuits.** Multi-input, multi-output synthetic circuits can implement complex decision functions: cells that produce different responses depending on which combination of inputs they detect, that integrate signals over time, that implement switch-like transitions between different phenotypic states. In principle, any computable function can be implemented in gene regulatory networks — the substrate is Turing complete. In practice, the noise inherent in stochastic gene expression limits the precision and reliability of genetic computation, which is one of the central engineering challenges of the field.

**Distributed circuits.** Because individual cells in a population can communicate via quorum sensing (Chapter 9), it is possible to distribute a computation across a population, with different cells implementing different modules and communicating their intermediate results chemically. This enables computation at spatial scales larger than any individual cell, and robustness through population averaging. Basu and colleagues (2005) demonstrated a synthetic band-detection circuit in which a population of bacteria collectively detected and responded to the center of a concentration gradient — a computation requiring both short-range activation and long-range inhibition, familiar from developmental biology.

## Xenobots: The First Designed Living Machines

In 2020, a collaboration between Josh Bongard's computational group at the University of Vermont and Michael Levin's biology group at Tufts published a paper describing the creation of "xenobots" — novel biological machines assembled from frog (*Xenopus laevis*) cells according to body plans designed by an evolutionary algorithm (Kriegman et al., 2020).

The design process was entirely computational: the algorithm explored the space of possible arrangements of skin and heart muscle cells (using a physical simulator) and selected configurations that could propel themselves through water. The winning designs were then assembled from actual frog cells — not programmed genetically, but physically arranged by microsurgery. The assembled xenobots moved through water using the spontaneous beating of the heart muscle cells in their bodies, with trajectories shaped by their physical configuration.

This is remarkable for several reasons. The xenobots were not programmed — there was no genetic engineering involved in the basic demonstration. They were shaped. The computation was done by the evolutionary algorithm in silico; the result was a design that was implemented in biological matter. The matter did the rest: muscle cells contracted, creating directed movement; skin cells provided structure. The biological behavior emerged from the physical configuration, not from any explicit program.

Subsequent work has shown that xenobots can move in coordinated groups, can push small objects, and — in an unexpected finding — can self-replicate: when placed in a dish with loose frog cells, xenobots can physically aggregate those cells into new xenobots (Kriegman et al., 2021). This is kinematic self-replication — replication by movement and contact, not by biological cell division — and it is a form of behavior that no one designed. It emerged from the properties of the materials.

The xenobot work raises direct questions about what counts as cognitive behavior in designed biological systems. The self-replication was unexpected, emergent, and clearly adaptive in the sense that it propagates the xenobot configuration. Whether it constitutes cognition in any meaningful sense — whether there is any information processing, any sensing, integration, or decision-making underlying the behavior — is a question the system itself cannot answer. But it is exactly the right question to ask.

## The Ethics of Engineering Cognition

Synthetic biology is now moving fast enough that its ethical dimensions require active engagement, not eventual consideration. Several issues deserve careful attention:

**Containment.** Synthetic organisms designed for environmental release — for bioremediation, pest control, or agriculture — raise questions about ecological impact that existing regulatory frameworks address only partially. The cognitive sophistication of the organisms involved complicates the assessment: an organism that can adapt its behavior to novel environments is, by definition, harder to predict than one with fixed behavior.

**Moral status.** As synthetic biological systems become more neurologically sophisticated — as organoids become more brain-like, as xenobots acquire more behavioral complexity — the question of their moral status becomes less abstract. We cannot currently determine whether any of these systems have subjective experience. But the precautionary principle suggests that this uncertainty should be reflected in our treatment of them. The philosophical frameworks developed in Chapter 36 will be relevant here.

**Dual use.** Genetic circuits can be designed to detect pathogens, deliver drugs, or diagnose disease — all beneficial applications. They can also, in principle, be designed to produce toxins, to modify the behavior of target organisms, or to spread novel genetic elements through natural populations. The same synthetic biology tools that enable medical breakthroughs could enable bioweapons. This dual-use potential requires governance frameworks that are both technically sophisticated and ethically grounded.

**Ownership and access.** The BioBricks standard and the iGEM competition have promoted an open-source approach to biological parts. But commercial pressures — and the increasing overlap between synthetic biology and pharmaceutical and agricultural industries — are generating pressure toward proprietary control of biological information. The ethics of patenting life, especially cognitive life, is unresolved.

None of these questions has a simple answer, and we will not pretend that this section has provided one. What we can say is that synthetic biology is, at its core, the engineering of cellular cognition — the design and construction of systems that sense, integrate information, and respond adaptively. That this project has profound ethical implications is not a reason to halt it. It is a reason to pursue it with greater philosophical care.

---

## References

Basu, S., Gerchman, Y., Collins, C. H., Arnold, F. H., & Weiss, R. (2005). A synthetic multicellular system for programmed pattern formation. *Nature*, 434(7037), 1130–1134.

Elowitz, M. B., & Leibler, S. (2000). A synthetic oscillatory network of transcriptional regulators. *Nature*, 403(6767), 335–338.

Endy, D. (2005). Foundations for engineering biology. *Nature*, 438(7067), 449–453.

Gardner, T. S., Cantor, C. R., & Collins, J. J. (2000). Construction of a genetic toggle switch in *Escherichia coli*. *Nature*, 403(6767), 339–342.

Kriegman, S., Blackiston, D., Levin, M., & Bongard, J. (2020). A scalable pipeline for designing reconfigurable organisms. *Proceedings of the National Academy of Sciences*, 117(4), 1853–1859.

Kriegman, S., Blackiston, D., Levin, M., & Bongard, J. (2021). Kinematic self-replication in reconfigurable organisms. *Proceedings of the National Academy of Sciences*, 118(49), e2112672118.

Tabor, J. J., Salis, H. M., Simpson, Z. B., Chevalier, A. A., Levskaya, A., Marcotte, E. M., ... & Ellington, A. D. (2009). A synthetic genetic edge detection program. *Cell*, 137(7), 1272–1281.
