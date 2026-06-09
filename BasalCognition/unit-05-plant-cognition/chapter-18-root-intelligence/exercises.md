# Chapter 18: Exercises

## Part I: Reflection and Discussion Questions

**1. Darwin's hypothesis revisited**
Darwin's "root brain" hypothesis has been both celebrated and criticized over the 140+ years since it was proposed. Evaluate the hypothesis in light of modern molecular plant biology. What aspects of Darwin's observation have been confirmed? What aspects have been shown to be inaccurate or misleading? Is the "brain" metaphor scientifically productive — does it generate testable predictions — or is it merely a colorful way of describing what the root apex does?

**2. Local rules and global outcomes**
The root system's exploration strategy — concentrating growth where resources are found, through local sensing and local growth modification — produces globally adaptive outcomes without any centralized representation of the soil environment. Compare this to the exploration strategies of animals (which typically involve representations of the environment and planning ahead). What are the advantages and disadvantages of the plant strategy? Under what ecological conditions would each strategy be preferred?

**3. Self/non-self discrimination**
Root self/non-self discrimination allows plants to modulate their competitive investment based on the identity of their neighbor. This phenomenon has parallels in kin recognition systems in other organisms (Dictyostelium, social insects). What is the evolutionary logic common to all these systems? What is the minimum molecular machinery required for self/non-self discrimination in roots, and how does this compare to the mechanisms in other organisms?

**4. The extended phenotype of the root**
The holobiont concept treats the root-rhizobiome system as an integrated unit. How does this perspective change how we think about root intelligence? If the plant recruits specific microbial partners to extend its physiological capabilities, does the "intelligence" of the plant-microbiome system exceed that of the plant alone? What additional capabilities does the rhizobiome add?

**5. Obstacle avoidance without a map**
We described root obstacle avoidance as an emergent property of continued tropistic responses after physical deflection — the root doesn't "know" there is an obstacle and route around it; it simply continues responding to gravity and nutrient gradients after being mechanically redirected. Is this a satisfying explanation, or does it leave something unexplained? What would it look like if the root did "know" there was an obstacle? What experiments could test whether root obstacle avoidance requires representation of the obstacle?

---

## Part II: Thought Experiments

**1. The Modular Root**
Imagine an experiment in which the root cap of a plant is surgically removed and replaced with the root cap from a different plant of the same species but with a different genotype. The host plant's root begins growing with the transplanted root cap. Now consider: which plant's chemical signals does the root tip sense as "self"? Will the root now recruit the microbiome appropriate to the donor plant or the host plant? Will the root respond competitively to its own root siblings (because the cap signals "non-self") or cooperatively (because the cap signals "self")? What does this experiment tell you about where root self-recognition signals originate?

**2. The Informed Root**
Design a hypothetical genetic modification to give a plant's root system capabilities it doesn't currently have: specifically, the ability to form a rudimentary spatial map of the soil volume it has explored. What genes would need to be added or modified? What signaling channels would need to be created? Would a plant with this capability have a competitive advantage over wild-type plants? Under what conditions?

**3. The Soil as Mind**
Consider the following proposition: the soil microbiome surrounding a plant root is, in an important sense, an extension of the plant's cognitive system — it senses soil conditions, responds to them, and feeds back signals that alter the plant's physiology and behavior. Evaluate this proposition. What aspects of cognition does the rhizobiome have? What aspects does it lack? Does your answer depend on whether you define cognition functionally or mechanistically?

---

## Part III: Laboratory Investigations

**1. Root Gravitropism and Nutrient Response**
*Goal*: Compare gravitropic and chemotropic responses in root growth direction.
*Materials*: Arabidopsis or tomato seeds, agar plates with defined nutrient concentrations, ruler, camera.
*Procedure*: (a) Grow seedlings on vertical agar plates for 5 days. (b) Rotate plates 90 degrees and observe root curvature over 24 hours (gravitropism). (c) On separate plates, establish a phosphate gradient by placing high-phosphate agar on one side of a low-phosphate plate. Observe root growth direction over 3–5 days.
*Analysis*: How long does gravitropic reorientation take? Does the presence of a lateral phosphate gradient alter the gravitropic set-point angle? Can you calculate a GSA from your observations?

**2. Root Exudate Collection and Microbiome Effects**
*Goal*: Observe the effect of root exudates on soil microbial growth.
*Materials*: Seedlings, sterile nutrient solution, agar plates with basic microbial media, soil samples.
*Procedure*: Collect root exudates by growing seedlings in sterile hydroponic solution for 1 week, then concentrating the solution. Spot the concentrated exudates onto microbial growth plates seeded with soil dilutions. Photograph at 24, 48, and 72 hours.
*Analysis*: Do areas treated with root exudates show different microbial growth than controls? Are there areas of inhibition (suggesting antimicrobial compounds in the exudate) or areas of enhanced growth?

**3. Modeling Root System Architecture**
*Goal*: Understand how local growth rules produce global root system geometry.
*Materials*: Computer with Python.
*Procedure*: Implement a simple agent-based model of root system development. Each "root tip" agent moves in a direction determined by: (a) its gravitropic set-point angle (biased toward downward); (b) a random walk component (representing local soil heterogeneity); (c) a nutrient-following component (if nutrient concentration at the current position is higher than the running average, continue in the current direction; if lower, increase turning probability). Generate a random 2D nutrient distribution.
*Analysis*: Does the simulated root system concentrate more tips in high-nutrient regions? How does the balance between gravitropism, random exploration, and chemotropism affect the efficiency of nutrient foraging? What happens if you vary the nutrient patch size?
