# Chapter 19: Exercises

## Part I: Reflection and Discussion Questions

**1. What counts as communication?**
We use the word "communication" for both the VOC signals between plants and the conversation between two humans. Is this the same concept applied in two different contexts, or are they genuinely different phenomena that happen to share a name? Construct a precise definition of "communication" that either includes or excludes plant VOC signaling, and defend your choice. Does the question of intentionality matter — should we require that a communicating entity "intend" to signal? If not, where does the concept of communication begin and end?

**2. The "wood wide web" problem**
The mycorrhizal network has been portrayed in popular media as a forest "internet" through which trees share resources and information. We have argued that this portrayal significantly exceeds what the evidence shows. At the same time, the evidence for carbon transfer and possible defense-priming signal transfer is real and important. What is the appropriate public communication about this research? How should scientists communicate findings that have a limited but genuine basis when they know the popular understanding will likely inflate those findings? Is this a failure of public communication, or an acceptable simplification?

**3. Specificity and meaning**
The VOC blend released by a plant attacked by caterpillars is different from the blend released by a plant attacked by aphids, and each blend attracts different predatory insects. Does the specificity of the blend — its different information content for different attackers — make it "meaningful"? What would it take for a VOC signal to be "meaningful" in a philosophically interesting sense? Compare to the specificity of pheromone signals in insects — are they meaningful?

**4. Competition and cooperation**
Both allelopathy (competitive chemical inhibition of neighbors) and VOC-mediated defense priming (which benefits neighbors by improving their defenses) are described in this chapter. Both are mediated by plant chemical releases. Both have fitness consequences for both the emitting plant and the receiving plant. What determines whether a plant chemical release is "cooperative" or "competitive"? Is this distinction meaningful at the mechanistic level, or only at the ecological level?

**5. Kin selection without brains**
Self/non-self discrimination in allelopathy parallels kin discrimination in Dictyostelium, in plant root competition, and in social insects. In social insects, kin discrimination involves neural processing, sensory systems, and behavioral decision-making. In plants and Dictyostelium, it involves chemical recognition without neurons. Does the mechanism matter for whether we consider these phenomena to be "kin selection" in the same biological sense? What is essential to kin selection, and what is substrate-dependent?

---

## Part II: Thought Experiments

**1. The Cheater Signal**
Suppose a plant species evolved the ability to release VOC blends that mimic the herbivory-distress signals of other species — "crying wolf" to prime neighbors' defenses before attacking them allelopathically. This would be a form of chemical deception: false signaling that increases the sender's fitness by reducing the receiver's competitive capacity. Is this scenario biologically plausible? What evolutionary conditions would favor it? What would prevent it from evolving? Does the concept of "deception" require intentionality, or can it be defined purely in functional terms?

**2. The Isolated Forest**
Imagine a forest in which all mycorrhizal fungi have been killed by a fungicide treatment. The trees are still alive and their roots still intact, but all fungal connections have been severed. Based on what you know about mycorrhizal network function, describe what changes you would expect to observe: (a) in individual tree physiology and nutrition; (b) in inter-tree carbon dynamics; (c) in defense responses to herbivore attack; (d) in understory plant diversity. Distinguish between changes that are well-predicted by established science and changes that depend on more contested claims.

**3. The Rational Allelopath**
Model allelopathy as an economic decision problem. A plant must "decide" (through evolved regulatory mechanisms) how much allelopathic compound to release. Each unit of compound costs metabolic resources and provides a competitive benefit by suppressing neighbors. The optimal amount depends on: the cost per unit, the competitive benefit per unit, the density and genetic similarity of neighbors, and the reliability of self/non-self discrimination. Write the optimization equation and solve for the optimal allelopathic investment as a function of these parameters. Under what conditions should allelopathy be abandoned in favor of resource competition?

---

## Part III: Laboratory Investigations

**1. VOC-Mediated Defense Priming**
*Goal*: Test whether volatiles from wounded plants can prime defense responses in unwounded neighbors.
*Materials*: Tomato or Arabidopsis plants, mechanical wounding equipment (scissors or roller for reproducible wounds), sealed chambers to allow volatile transfer, qRT-PCR for defense gene expression (PR1, JA biosynthesis genes).
*Procedure*: Place a wounded plant and an unwounded plant in a sealed chamber. Include a control with an unwounded plant in a chamber with another unwounded plant. After 24 hours, wound the unwounded plants in all chambers and measure defense gene expression at 2, 6, and 24 hours post-wounding.
*Analysis*: Do plants that were in chambers with wounded neighbors show faster or stronger defense gene induction than plants in control chambers? Is the difference statistically significant? Can you identify which VOC compounds in the chamber air might be responsible?

**2. Allelopathy Bioassay**
*Goal*: Test whether root exudates from one plant species inhibit the germination or growth of another.
*Materials*: Seeds of two plant species (e.g., sunflower and radish, or rye and lettuce), soil or agar for germination, collection vessel for root exudates.
*Procedure*: Collect root exudates from established plants of species A by washing roots in sterile water and concentrating the wash. Apply exudate solutions of varying concentrations to germination plates of species B. Record germination percentage and seedling root length at 3 and 7 days.
*Analysis*: Does species A's exudate inhibit species B's germination or growth? Is the effect dose-dependent? Is the effect general (affecting total biomass) or specific (affecting germination vs. root growth vs. shoot growth)?

**3. Mycorrhizal Network Carbon Transfer Simulation**
*Goal*: Simulate the dynamics of carbon transfer through a mycorrhizal network under different sink-source configurations.
*Materials*: Computer with Python.
*Procedure*: Model a network of N plants connected by fungal hyphae with defined conductivities. Each plant has a source strength (photosynthesis rate, proportional to its canopy area) and a sink strength (root respiration + growth demand). Model carbon flow along conductivity-weighted concentration gradients. Vary: (a) the canopy area of individual plants (shading effects); (b) the number of connections per plant; (c) the conductivity of individual hyphal connections.
*Analysis*: Under what conditions does net carbon transfer occur from source-strong to source-weak plants? How much does transfer improve the growth of shaded individuals? How sensitive is the result to the assumption that transfer is passive (driven by concentration gradients) vs. active (regulated by biological mechanisms)?
