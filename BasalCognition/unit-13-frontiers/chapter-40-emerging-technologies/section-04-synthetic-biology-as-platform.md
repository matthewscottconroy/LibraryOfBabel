# Section 4: Synthetic Biology as Platform

## The Reconstruction Strategy

One powerful approach to understanding complex biological systems is reconstruction: rather than trying to infer the principles of the system by observing its natural behavior, build the system from scratch — or build simplified versions that capture the essential features — and determine what properties emerge from what components. This is the reconstruction strategy, applied in different forms across many areas of science.

In basal cognition research, the reconstruction strategy asks: can we build a minimal cognitive system from well-characterized biological parts? If we assemble a system with the properties we believe to be necessary for cognition — sensing, integration, memory, goal-directed action — will it exhibit cognitive behavior? What is the minimal set of components required?

This is not a rhetorical or philosophical question; it is an empirical one that synthetic biology is beginning to make tractable.

## Building Minimal Cognitive Systems from Genetic Parts

The synthetic biology toolkit described in Chapter 35 — standardized genetic parts, characterized circuits, BioBricks and their successors — provides the raw material for building minimal cognitive systems. Starting from individual well-characterized components (sensors, logic gates, memory elements, actuators), synthetic biologists can assemble circuits that implement specific cognitive functions, test whether those functions emerge from the assembled system, and compare the performance of the synthetic circuit to the natural cognitive system it is intended to model.

This approach has several advantages:

**Mechanistic clarity.** A synthetic system is built from known components with known properties. When it exhibits a cognitive behavior, the mechanism is transparent: we know which component is doing what. This is in contrast to natural systems, where the mechanism must be inferred from observations.

**Parameter control.** The performance parameters of a synthetic system — the binding constants, kinetic rates, expression levels — can be varied systematically. This allows the mapping of which parameter values are necessary for cognitive function, and how performance degrades as parameters deviate from optimal values. Natural systems can be perturbed (with mutations, drugs, or environmental changes), but the perturbations are indirect and their effects on specific parameters are often uncertain.

**Modularity.** A synthetic system built from modular parts can be systematically expanded: new sensors can be added to extend the range of stimuli to which the system responds; additional memory elements can be added to extend the history over which the system integrates information; output modules can be swapped to change the behavioral response. This modularity allows systematic exploration of what each component contributes to the overall cognitive function.

**Baseline comparison.** A synthetic system that implements the minimal requirements for a specific cognitive function, and is tested in the same organism as the natural cognitive system, provides a controlled baseline against which the natural system can be compared. Differences between synthetic and natural performance reveal what additional properties — beyond the minimal requirements — contribute to the natural system's performance.

## Reconstruction Approaches to Bacterial Cognition

The most developed area of synthetic cognitive biology is the reconstruction of bacterial cognition — specifically, the reconstruction of chemotaxis-like behavior from synthetic genetic circuits.

The *E. coli* chemotaxis system has been the primary model for this work, both because it is the best-understood natural bacterial cognitive system and because the synthetic biology toolkit for *E. coli* is the most developed. Synthetic versions of chemotaxis circuits — circuits that implement sensing, adaptation, and directed motility — have been built using synthetic receptor proteins that respond to non-natural ligands, engineered phosphorylation cascades, and synthetic flagellar expression control systems (Dueber et al., 2004, and subsequent work).

These synthetic chemotaxis circuits are less efficient than the natural *E. coli* chemotaxis system — they adapt more slowly, are less sensitive, and achieve a smaller range of motion control. This performance difference reveals what properties of the natural system are necessary for its performance: the precise matching of receptor methylation kinetics to swimming speed, the amplification gain of the phosphorylation cascade, the statistical properties of the flagellar switching dynamics. Each of these properties is absent from the simplest synthetic reconstruction, and adding each one improves performance in specific, measurable ways.

## Directed Evolution of Cognitive Capacities

Another reconstruction strategy that has attracted interest is directed evolution — using in vitro or in vivo selection to evolve novel cognitive capacities in simplified biological systems.

In directed evolution, a population of variants (differing in DNA sequence and therefore in the properties of the proteins they encode) is subjected to a selection pressure that favors variants with a desired property, and the selected variants are amplified and subjected to further rounds of mutation and selection. Over many rounds, the population evolves toward the desired property.

Applied to cognitive function, directed evolution could ask: what minimal circuit architecture, subjected to selection for a specific cognitive behavior, evolves that behavior? For instance, what is the minimal promoter-gene-regulator architecture that evolves anticipatory behavior when selected against a periodically changing environment? What minimal quorum sensing circuit architecture evolves collective decision-making when populations of bacteria with different variants are placed in competition?

Directed evolution of cognitive capacities is at the frontier of synthetic biology — most experiments to date have targeted specific molecular properties (enzyme activity, protein binding affinity) rather than system-level cognitive properties. But the conceptual extension is straightforward, and early demonstrations that circuit-level properties can be evolved in the laboratory suggest that the approach is feasible.

## What Synthetic Reconstruction Cannot Tell Us

The reconstruction strategy has limitations that should be acknowledged. Most importantly, a synthetic system that exhibits a cognitive behavior in a laboratory context may not exhibit that behavior in the natural ecological context of the organism — the natural system is optimized for performance in a specific environment, and the synthetic system may be missing properties that are necessary in that environment but not in the controlled laboratory setting.

More fundamentally, a synthetic system tells us about the sufficient conditions for a cognitive behavior — what components, assembled in what way, are sufficient to produce it. It does not tell us whether those components are the ones the natural system uses — whether the natural system evolved the same solution to the same problem. The natural system may use a different mechanism that achieves the same function, and the synthetic reconstruction may be discovering one possible implementation among many.

This limitation is real but not debilitating. The reconstruction strategy does not replace the study of natural systems; it complements them. By identifying what is sufficient for a cognitive behavior, it constrains the space of possible natural mechanisms and generates testable hypotheses about which of those mechanisms the natural system employs. The dialogue between reconstruction and natural system study is one of the most productive approaches available in basal cognition research.

---

## References

Dueber, J. E., Yeh, B. J., Bhattacharyya, R. P., & Lim, W. A. (2004). Rewiring cell signaling: The logic and plasticity of eukaryotic protein circuitry. *Current Opinion in Structural Biology*, 14(6), 690–699.

Elowitz, M. B., & Leibler, S. (2000). A synthetic oscillatory network of transcriptional regulators. *Nature*, 403(6767), 335–338.

Gardner, T. S., Cantor, C. R., & Collins, J. J. (2000). Construction of a genetic toggle switch in *Escherichia coli*. *Nature*, 403(6767), 339–342.
