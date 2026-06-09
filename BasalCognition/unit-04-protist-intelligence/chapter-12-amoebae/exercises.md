# Chapter 12: Exercises

## Part I: Reflection and Discussion Questions

**1. The vocabulary of cognition**
Throughout this chapter, we have used words like "decision," "sensing," "strategy," and "reading" to describe amoeboid behavior. Choose one of these terms and construct the best possible argument that its application to amoebae is scientifically appropriate, not merely metaphorical. Then construct the best possible argument against that application. Which argument do you find more persuasive, and why?

**2. Symmetry breaking as stochastic choice**
We described chemotaxis in Dictyostelium as a process in which the external gradient biases a stochastic symmetry-breaking event rather than specifying a direction top-down. What are the functional advantages of this design compared to a hypothetical "pure computation" model in which the cell computes a gradient vector and executes movement precisely toward it? Are there disadvantages?

**3. The tragedy of the commons in the petri dish**
Strassmann and Queller's work on Dictyostelium cheater strains demonstrates evolutionary prisoner's dilemma dynamics in a microorganism. In what sense does this represent a "cognitive" problem — a problem requiring information processing — for the cells involved? Is it the same kind of problem for the cells as it is for humans in analogous social situations? What are the similarities and where does the analogy break down?

**4. Phenotypic switching and the self**
Entamoeba histolytica can switch between non-invasive and invasive phenotypes in response to environmental signals. If we define an organism's "behavioral repertoire" as a key criterion for assessing its cognitive complexity, how should we account for phenotypic switching — the expansion of the repertoire through gene expression changes — in that assessment? Does a cell with two possible phenotypes have a "larger" behavioral repertoire than one with one, in a meaningful sense?

**5. Kin discrimination without brains**
The Dictyostelium TgrB1/TgrC1 system allows cells to discriminate genetically related from genetically unrelated strains during aggregation. Contrast this with kin recognition in social insects (bees, ants), which involves sensory processing, memory formation, and behavioral decisions made by animals with complex nervous systems. What is similar about the functional outcome? What is fundamentally different about the mechanisms? Does the mechanism matter for how we evaluate the cognitive complexity of the behavior?

---

## Part II: Thought Experiments

**1. The Minimum Cognizer**
Imagine you are designing the simplest possible organism that would qualify as a "cognizer" — an entity that genuinely processes information about its environment and adjusts its behavior accordingly. Based on what you have learned about amoeboid sensing and signaling, what would be the minimum molecular machinery required? Could your minimum cognizer be simpler than Entamoeba? At what point, moving from simpler to more complex, does the system cross the threshold from "mere chemistry" to "cognition" — or is there no such threshold?

**2. The Invisible Gradient**
In a thought experiment proposed by philosophers of mind, you replace every neuron in a human brain with a functionally identical synthetic substitute — one by one, until the entire brain is artificial. The person continues to behave normally throughout. Does this thought experiment apply, with appropriate modifications, to an amoeba? Suppose you replaced every protein in the cAMP signaling pathway with synthetic analogs that function identically. At what point (if any) does the cell stop "sensing" and become a "mere chemical reactor"? What does your intuition tell you, and what does it tell you about your intuitions?

**3. The Colonial Selves of Dictyostelium**
During the Dictyostelium multicellular stage, the aggregate behaves in ways that appear to serve collective rather than individual interests. Individual pre-stalk cells "sacrifice" themselves for the collective. Consider the following question: at what point in the lifecycle is Dictyostelium one organism, and at what point is it many? Choose the most defensible boundary and defend it rigorously. Then consider: does your answer change anything about how we should evaluate the "intelligence" of what Dictyostelium does?

---

## Part III: Laboratory Investigations

**1. Chemotaxis Assay with Unicellular Organisms**
*Goal*: Observe directed movement toward a chemical attractant.
*Materials*: Dictyostelium discoideum cells (available from the Dicty Stock Center), microscope slides, coverslips, chemoattractant (cAMP or folate solution), soft agarose, phase-contrast or brightfield microscope.
*Procedure*: Prepare a thin layer of agarose on a slide. Place a small drop of concentrated cAMP solution at one end of the field. Introduce starved Dictyostelium cells (6–8 hours of starvation) at the far end. Observe and photograph or video the cell population at 10-minute intervals for 2 hours. Measure the displacement of the cell population's centroid toward or away from the cAMP source.
*Analysis*: Is movement toward the attractant statistically significant? Do all cells move equally, or is there variation? What do departures from directed movement tell you about the noise in the system?

**2. Aggregation Wave Visualization**
*Goal*: Observe and characterize the cAMP wave dynamics of Dictyostelium aggregation.
*Materials*: Dictyostelium cells, non-nutrient agar plates, darkfield or phase-contrast microscope with a camera capable of time-lapse.
*Procedure*: Plate starved Dictyostelium at high density on non-nutrient agar. Set up time-lapse imaging at 1-frame-per-minute over 8–12 hours, beginning within 2 hours of plating. Image the entire plate with low-magnification darkfield illumination, which will reveal the aggregation waves as dark/light concentric or spiral patterns.
*Analysis*: Measure wave period, wave speed, and the number of aggregation centers. Does the number of centers decrease over time as larger centers absorb smaller ones? How does wave speed compare to values reported in the literature (~300 µm/min)?

**3. Simulating LEGI Dynamics**
*Goal*: Understand local excitation / global inhibition as a computational strategy.
*Materials*: Computer with Python and NumPy/Matplotlib, or any simulation environment.
*Procedure*: Implement a simple 1D simulation of the LEGI model. Represent a cell as an array of 100 receptor positions. Each position has a receptor occupancy value drawn from a Gaussian distribution with mean equal to the concentration of attractant at that position (linear gradient across the cell) and standard deviation proportional to the square root of the mean (Poisson noise). Apply local excitation (each position's activity is boosted by its own receptor occupancy) and global inhibition (each position's activity is suppressed by the mean receptor occupancy across all positions). Plot the final activity profile as a function of position.
*Analysis*: How does the sharpness of the activity peak at the high-concentration end depend on the slope of the gradient? At what gradient steepness does the system fail to localize the peak reliably? What does this tell you about the limits of gradient sensing?
