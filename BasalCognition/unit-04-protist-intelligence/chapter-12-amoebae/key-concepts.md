# Chapter 12: Key Concepts

## Glossary

**Pseudopod**
A temporary cytoplasmic extension used for locomotion and phagocytosis. Pseudopods are formed by the directed polymerization of actin filaments beneath the cell membrane, pushing it outward. The controlled formation and retraction of pseudopods in specific locations — regulated by the LEGI mechanism — is the physical basis of amoeboid directed movement. The term literally means "false foot," a historical artifact that obscures the biological sophistication of the structure.

*Why it matters*: The pseudopod is the elementary unit of amoeboid behavior. Understanding how pseudopod formation is spatially regulated is equivalent to understanding how the cell navigates — how it translates information about its environment into action.

---

**Chemotaxis**
Directed movement along a chemical concentration gradient. In amoebae, chemotaxis typically involves moving toward attractants (positive chemotaxis) or away from repellents (negative chemotaxis). The directionality of chemotaxis is not computed from explicit gradient detection but emerges from the asymmetric activation of the cytoskeletal machinery driven by the LEGI mechanism.

*Why it matters*: Chemotaxis is the most tractable example of cellular navigation — a behavioral output that can be quantitatively measured and mechanistically dissected. Understanding chemotaxis provides the template for understanding more complex cellular behaviors.

---

**Local Excitation / Global Inhibition (LEGI)**
A signaling architecture in which a local excitatory signal (proportional to local receptor occupancy) is combined with a global inhibitory signal (proportional to the average receptor occupancy across the entire cell). The result is a sharp spatial activity peak at the site of maximum receptor occupancy, even when the gradient is shallow and the absolute differences in receptor occupancy are small. LEGI is a spatial filtering algorithm implemented in biochemical form.

*Why it matters*: The LEGI model provides a mechanistic explanation for how cells can detect shallow chemical gradients despite substantial molecular noise. It also illustrates a more general principle: that cognitive-like functions (in this case, directional sensing) can be implemented in biochemical networks without requiring anything resembling a nervous system.

---

**Symmetry Breaking**
A process by which a system that is initially symmetric — with no preferred direction or state — spontaneously transitions to an asymmetric state. In amoeboid motility, symmetry breaking describes how a cell that is initially isotropic (no front or back) develops a defined polarity (a front and back) through the competitive dynamics of internal signaling. Symmetry breaking is driven by noise amplification: a small random fluctuation is amplified by positive feedback until it dominates, suppressing all competitors.

*Why it matters*: Symmetry breaking shows that directionality can emerge from a non-directed system — it does not require a pre-specified "front." This insight changes how we think about cellular decision-making: choices can emerge from random fluctuations biased by environmental signals, rather than from explicit computation.

---

**cAMP (Cyclic Adenosine Monophosphate)**
A ubiquitous second messenger molecule synthesized from ATP by adenylyl cyclase and degraded by phosphodiesterase. In Dictyostelium, cAMP serves a dual role: intracellularly, as a relay in response to receptor activation; and extracellularly, as the diffusible aggregation signal. Cells both respond to extracellular cAMP (by polarizing and moving toward the source) and relay it (by producing and secreting their own cAMP pulse), allowing wave propagation through large cell populations.

*Why it matters*: The cAMP signaling system of Dictyostelium is one of the best-characterized examples of self-organizing collective signaling in biology. Understanding it provides insight into how individual cells can coordinate behavior at the population scale without centralized control.

---

**Excitable Medium**
A physical or chemical system characterized by three properties: a stable resting state, the ability to produce a large transient response when stimulated past a threshold, and a refractory period after excitation during which the system cannot be re-excited. Neurons, cardiac muscle, and the Dictyostelium cAMP signaling network are all examples of excitable media. Excitable media support traveling waves that carry directional information.

*Why it matters*: Framing Dictyostelium aggregation in terms of excitable media dynamics connects it to a broad class of phenomena in physics, chemistry, and biology, and provides the mathematical tools for quantitative analysis and prediction.

---

**Altruism (Biological)**
In evolutionary biology, altruism refers to a behavior that reduces the actor's own reproductive fitness while increasing the fitness of others. Stalk cell formation in Dictyostelium is a textbook example: stalk cells die without reproducing, enabling spore cells to be elevated for dispersal. Biological altruism, unlike psychological altruism, requires no motivation or intent; it is defined purely in terms of fitness consequences.

*Why it matters*: The existence of biological altruism in single-celled organisms challenges naive views of evolution as simply maximizing individual reproductive success. Understanding how altruism can evolve — through kin selection and other mechanisms — is one of evolutionary biology's central intellectual achievements, and Dictyostelium makes these dynamics experimentally accessible.

---

**Kin Discrimination**
The ability to distinguish genetically related from genetically unrelated individuals and to behave differently toward each. In Dictyostelium, kin discrimination is mediated by cell-surface proteins (TgrB1/TgrC1) that mediate self/non-self recognition: cells preferentially aggregate with genetically similar cells, limiting the fitness benefits of altruism to relatives. This is, in functional terms, a form of cellular identity detection — a sensing system that reads genetic information from the environment.

*Why it matters*: Kin discrimination in Dictyostelium demonstrates that molecular sensing can extend beyond the physical and chemical properties of the environment to include social information — specifically, the genetic relatedness of neighboring cells. This is arguably one of the earliest evolutionary roots of social cognition.

---

**Phenotypic Switching**
A transition between distinct, stable phenotypic states in response to environmental signals, without changes in the underlying DNA sequence. In Entamoeba histolytica, the switch between non-invasive (colonizing) and invasive (tissue-penetrating) phenotypes represents a dramatic change in gene expression, morphology, and behavior triggered by environmental inputs including host immune signals. Phenotypic switching is distinct from gradual adaptive changes; it is a discrete, often rapid transition between qualitatively different states.

*Why it matters*: Phenotypic switching demonstrates that behavioral repertoire can be expanded without genetic change — that a single genome can encode multiple distinct behavioral programs, expressed conditionally depending on environmental context. This is, in a meaningful sense, a cellular analog of context-dependent behavior.
