# Section 1: Imaging and Sensing

## The Challenge of Seeing Cognition

Cognition, even in its neural instantiation, is not directly observable. What we observe are the physical correlates of cognition — neural activity, molecular concentrations, electrical signals. The inference from observable physical states to cognitive function requires a theoretical framework that interprets the physical measurements as cognitive.

In non-neural systems, this challenge is intensified by the unfamiliarity of the substrate. We know, in a rough sense, what to look for in a nervous system because we have a theory of what neural computation does and how it is implemented. In a bacterial cell, a plant leaf, or a slime mold network, the relevant physical states — the ones that correspond to sensing, integration, memory, and decision-making — are largely unknown or contested. Developing imaging and sensing tools for basal cognition research therefore requires not just technical development but theoretical guidance about what to measure.

This section describes the major current approaches to observing the internal states of non-neural biological systems, with emphasis on the kinds of cognitive-relevant information each approach can provide.

## Fluorescent Reporters for Bioelectric State

The most important recent advance for observing bioelectric state in non-neural organisms is the development of genetically encoded voltage indicators (GEVIs) — fluorescent protein constructs whose fluorescence intensity or spectral properties change with membrane voltage. When expressed in a cell, a GEVI allows the membrane potential of that cell to be read out optically, using a fluorescence microscope, without physical contact.

GEVIs were initially developed for neuroscience applications — for imaging neural activity without the invasiveness of electrode recording. But their application to non-neural systems has been transformative. Michael Levin's group has used GEVIs to image the bioelectric state of developing frog embryos, showing that the spatial pattern of membrane voltage across the embryo precedes and predicts the pattern of tissue differentiation — that the bioelectric pattern is not merely a consequence of tissue identity but a causal contributor to it (Pai et al., 2012).

In bacteria, fluorescent reporters for membrane potential — including voltage-sensitive dyes and genetically encoded versions — have allowed direct observation of the action potential-like electrical spikes that propagate through bacterial biofilms (Prindle et al., 2015). These spikes are slow compared to neural action potentials — on the order of minutes rather than milliseconds — but they propagate across the entire biofilm, coordinating metabolic activity across the community. The observation of these signals, which would not have been visible with earlier techniques, was only possible with fluorescent membrane potential reporters.

The current limitations of GEVIs are substantial: they are generally slow (limited temporal resolution), noisy (limited signal-to-noise ratio at physiological voltages), and produce small signals that are difficult to distinguish from background fluorescence. Improving GEVI performance is an active engineering effort in neuroscience, and the improvements benefit non-neural applications equally.

## Voltage-Sensitive Dyes and Optogenetic Sensors

Before genetically encoded indicators, the primary tool for imaging membrane voltage across tissues was voltage-sensitive dyes (VSDs) — small organic molecules that insert into cell membranes and change their spectral properties with membrane voltage. VSDs can be applied to any tissue without genetic modification, making them immediately usable in organisms that are not genetically tractable.

VSDs have been used extensively in plant biology to image the propagation of electrical signals following mechanical or chemical stimulation (Fromm & Lautner, 2007). These signals propagate across plant tissues — from the point of stimulation through the vascular system and into leaves and stems — and are associated with the activation of defense responses, stomatal closure, and changes in photosynthesis. The spatial and temporal patterns of these propagating electrical signals are now mappable with VSD imaging at cellular resolution.

Optogenetic sensors — a newer class of reporters that use light to sense (as opposed to optogenetic actuators, which use light to control) — include fluorescent protein-based indicators for calcium (GCaMP family), cyclic AMP, ATP, and numerous other signaling molecules. The GCaMP family of calcium indicators has been particularly valuable: calcium is a nearly universal second messenger in signaling, and its dynamics often directly report information flow in signaling networks. GCaMP imaging of plant cells has revealed the dynamics of calcium waves following mechanical stimulation, herbivore attack, and pathogen invasion, with spatial and temporal detail that was previously inaccessible.

## Single-Cell RNA Sequencing

The revolution in single-cell RNA sequencing (scRNA-seq) — the ability to measure the complete transcript profile of individual cells simultaneously, across thousands of cells in a single experiment — has transformed many areas of biology and is beginning to be applied to basal cognition research.

The cognitive relevance of scRNA-seq is indirect but significant. The transcript profile of a cell is a readout of its gene expression state — which genes are actively transcribed at the moment of measurement. Gene expression state is closely related to cellular identity and signaling state, and changes in gene expression can report how a cell has "interpreted" the signals it has received. In the context of basal cognition, scRNA-seq can reveal which cells in a community are in which signaling state, how the distribution of states changes in response to environmental perturbations, and whether cells in different regions of a tissue or colony have different signaling states that reflect their position and history.

In biofilms, scRNA-seq (or its bacterial equivalent, which uses different technologies for individual-cell isolation) can reveal whether cells with different quorum sensing states or metabolic histories have systematically different gene expression profiles — and therefore whether the community has genuine cellular heterogeneity of the kind that would be required for distributed decision-making. In plant tissues, scRNA-seq reveals the cell-type landscape of different tissues and how it changes during development or in response to stress, contributing to the mechanistic understanding of how plant electrical signals translate into tissue-level behavioral responses.

## Super-Resolution Microscopy

Standard light microscopy is limited by the diffraction limit of light — approximately 200 nanometers in the lateral direction. Many of the molecular machinery relevant to basal cognition operates at scales below this limit: individual receptor proteins, signaling complexes, ion channels, and cytoskeletal elements are all smaller than the diffraction limit.

Super-resolution microscopy techniques — stimulated emission depletion (STED) microscopy, stochastic optical reconstruction microscopy (STORM), photoactivated localization microscopy (PALM), and their relatives — circumvent the diffraction limit by various methods to achieve lateral resolutions of 10–50 nanometers, within a factor of a few of electron microscopy, while retaining the ability to image fluorescently labeled specific proteins in living cells (Hell, 2009).

For basal cognition research, super-resolution microscopy enables direct visualization of the molecular machinery underlying cognitive behaviors: the clustering of receptors in signaling patches, the spatial organization of signaling complexes, the dynamics of membrane structures involved in electrical signal propagation. In bacteria, super-resolution microscopy has revealed the detailed spatial organization of chemoreceptor clusters — the molecular pattern recognition centers of the chemotaxis system — and how their organization changes with adaptation state (Briegel et al., 2012).

## The Integration of Imaging Approaches

The most powerful experimental approaches combine multiple imaging modalities: simultaneous imaging of voltage, calcium, and transcript levels; correlating subcellular imaging with whole-organism behavioral observation; integrating time-lapse imaging with subsequent molecular characterization of individual cells. These integrated approaches are technically demanding and produce complex datasets, but they provide the kind of multimodal view of biological systems that is needed to connect molecular mechanisms to cognitive behaviors.

The computational challenge of analyzing such complex, multidimensional datasets is itself an area of active development. Machine learning approaches — in particular, deep learning for image analysis and multimodal data integration — are increasingly being applied to the analysis of imaging data from biological systems, enabling the extraction of patterns and correlations that would be inaccessible by manual analysis. The development of appropriate analytical tools is as important as the development of imaging technologies themselves.

---

## References

Briegel, A., Wong, E. H., Hodges, H. L., Oikonomou, C. M., Piber, M., Ringgaard, S., ... & Jensen, G. J. (2012). New insight into bacterial chemoreceptor array structure and assembly from electron cryotomography. *Biochemistry*, 51(47), 9407–9417.

Fromm, J., & Lautner, S. (2007). Electrical signals and their physiological significance in plants. *Plant, Cell & Environment*, 30(3), 249–257.

Hell, S. W. (2009). Microscopy and its focal switch. *Nature Methods*, 6(1), 24–32.

Pai, V. P., Aw, S., Shomrat, T., Lemire, J. M., & Levin, M. (2012). Transmembrane voltage potential controls embryonic eye patterning in *Xenopus laevis*. *Development*, 139(2), 313–323.

Prindle, A., Liu, J., Asally, M., Ly, S., Garcia-Ojalvo, J., & Süel, G. M. (2015). Ion channels enable electrical communication in bacterial communities. *Nature*, 527(7576), 59–63.
