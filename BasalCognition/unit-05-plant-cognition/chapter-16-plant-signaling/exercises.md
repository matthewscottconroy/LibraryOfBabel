# Chapter 16: Exercises

## Part I: Reflection and Discussion Questions

**1. The vocabulary question**
The controversy between Baluška/Mancuso and Taiz et al. is partly about vocabulary: should we use neural terms ("action potentials," "neurotransmitters," "plant neurobiology") for plant phenomena? Construct the strongest possible argument for using these terms, and the strongest possible argument against. Crucially: does the resolution of this debate depend on any empirical fact that has not yet been established, or is it purely a matter of how we choose to use words? If the latter, what are the consequences for scientific practice of choosing one vocabulary over the other?

**2. Systemin and the wound response**
The systemic wound response — proteinase inhibitor accumulation throughout the plant in response to local herbivory — coordinates a whole-plant defensive response without any nervous system. What are the similarities between this response and the immune response in vertebrates? What are the differences? Does the wound response tell us anything about what a "nervous system" is for, or what makes nervous systems different from other whole-body coordination systems?

**3. Tropisms as behavior**
We described tropisms as "sensorimotor integration" — combining sensory information with growth responses to reposition the plant. Is this an appropriate description? What does the tropism comparison reveal about what we take "behavior" to mean? Could you design a non-biological system (a robot, for example) that behaved by growing rather than by muscle contraction, and would you describe it as exhibiting sensorimotor integration?

**4. The speed argument**
Critics of plant cognition sometimes argue that the slow speed of plant electrical signals compared to neural signals disqualifies them from cognitive function. Evaluate this argument. What is the appropriate reference for "slow"? Does the relevant comparison depend on the ecological timescale of the selective pressure the signals are responding to? Can you construct a case where slower signaling is actually adaptive?

**5. Self-organization and auxin transport**
We described auxin transport as a self-organizing system that produces complex developmental patterns (phyllotaxis, leaf vein patterning) from simple local rules. Does this self-organizing pattern generation constitute a form of cognition? If not, what would it take to make it cognitive? Compare to Physarum network formation (Chapter 13) — what are the similarities and differences?

---

## Part II: Thought Experiments

**1. The Amputated Plant**
Imagine a tomato plant from which the apical shoot has been removed. The plant continues to grow lateral shoots. Now consider: where is the "information" that coordinates the plant's growth response to apical removal stored? Is it in the distribution of hormones? In the gene expression states of cells? In the architecture of the vascular network? Design an experiment to distinguish between these possibilities. What does your answer tell you about where "cognition" is located in plants?

**2. The Supersensing Plant**
Imagine a genetically modified plant with phototropin receptors 1000 times more sensitive than wild type. This plant can respond to light gradients so shallow that they carry essentially no directional information about the light source (the gradient is dominated by thermal noise). What would happen to this plant's phototropism? What does this thought experiment tell you about the relationship between sensor sensitivity, signal-to-noise, and adaptive behavior? Is there an analog of this problem in neural sensory systems?

**3. The Memory-Erased Plant**
Many plant responses "remember" past conditions through epigenetic mechanisms — methylation patterns that persist through cell division and influence future gene expression. Suppose you could erase all plant epigenetic memory (reset all methylation patterns to naive defaults) without otherwise affecting the plant. What would happen to its stress responses? Its seasonal timing? Its defense responses to previously encountered pathogens? What does this thought experiment tell you about the relationship between memory and adaptive behavior in plants?

---

## Part III: Laboratory Investigations

**1. Phototropism Measurement**
*Goal*: Quantify phototropism in seedlings and assess the Cholodny-Went prediction.
*Materials*: Arabidopsis thaliana or tomato seeds, black cardboard boxes with a single hole for directional light, ruler, camera for time-lapse.
*Procedure*: Grow seedlings vertically in darkness for 48–72 hours after germination (etiolated seedlings are maximally phototropic). Transfer to boxes with directional light from one side. Photograph seedlings every 30 minutes for 6 hours. Measure the angle of the shoot from vertical as a function of time.
*Analysis*: How long before curvature is detectable? What is the final angle? Does the curvature advance from the tip downward (consistent with signal originating in the tip)? If you cover the tip with aluminum foil, does the plant still curve?

**2. Gravitropism in Roots**
*Goal*: Observe and quantify gravitropic reorientation after displacement from vertical.
*Materials*: Arabidopsis seedlings grown on agar plates, rotating platform (or simply rotating the plate), camera or microscope for imaging.
*Procedure*: Grow seedlings for 4–5 days on vertical agar plates. Rotate the plate 90 degrees (so roots now grow horizontally). Photograph at 30-minute intervals for 8 hours. Measure the angle of root curvature from horizontal as a function of time.
*Analysis*: How quickly do roots begin curving downward? Is the rate of curvature constant, or does it slow as the root approaches vertical? What does the kinetics tell you about the dynamics of the auxin redistribution mechanism?

**3. Electrical Signaling Simulation**
*Goal*: Model the propagation of an electrical signal through a plant-like network and compare to neural signal propagation.
*Materials*: Computer with Python.
*Procedure*: Implement a simple cable equation model for electrical propagation through a plant vascular element (phloem sieve tube). Use literature values for membrane capacitance, membrane resistance, and cytoplasmic resistance in phloem. Simulate the propagation of an action potential from one end of a 10 cm segment. Compare: (a) plant phloem parameters, (b) mammalian unmyelinated C fiber parameters, (c) mammalian myelinated A fiber parameters.
*Analysis*: How does propagation speed and signal decay depend on the cable parameters? How far does the signal travel before decaying to half its original amplitude? What structural changes to the plant conducting system would be needed to achieve neural-speed propagation?
