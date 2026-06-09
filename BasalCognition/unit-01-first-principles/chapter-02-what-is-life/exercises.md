# Chapter 2 Exercises: What Is Life?

---

## Part I: Reflection and Discussion

**1.** The thermodynamic argument in Section 2.3 holds that sensing is a physical necessity for any system maintaining far-from-equilibrium organization. But note that a modern gas-powered automobile also maintains a far-from-equilibrium state (the combustion reaction) by continuously coupling to an energy source, and it has sensors (oxygen sensors, temperature sensors, knock sensors) that modulate its behavior. Does a car satisfy the thermodynamic argument for cognition? If not, what is missing — and does your answer reveal a gap in the argument, or a feature of the definition that needs to be made more precise?

**2.** Section 2.1 noted that prions — misfolded proteins that catalyze the misfolding of other proteins — may satisfy some definitions of life, including the capacity for Darwinian evolution. Apply the autopoiesis definition to prions: Are they autopoietic? Do they sense, integrate, and act in ways relevant to their continued organization? What does this thought exercise reveal about the relationship between the different definitions of life?

**3.** Maturana and Varela claim that "living systems are cognitive systems, and living as a process is a process of cognition." Evan Thompson (2007) endorses a modified version of this claim. Godfrey-Smith (1994) argues it is too strong. Construct the best version of Godfrey-Smith's objection, and then construct the best response to it from Maturana and Varela's perspective. Which side do you find more convincing, and why?

**4.** The discovery that LUCA probably had ion channels and primitive chemosensory systems suggests that cognitive capacities are as old as life itself. But what does "as old as life itself" actually mean for the question of whether these are cognitive capacities? Is there a risk of circular reasoning here: defining life as cognitive, and then "discovering" that life has cognitive capacities from the beginning?

**5.** Nick Lane argues that the proton gradient across cellular membranes is the fundamental energy currency of life — and that this gradient is also the physical basis of bioelectric signaling. This suggests that energy metabolism and information processing are, at the cellular level, aspects of the same phenomenon. What are the implications of this claim for how we think about the evolution of nervous systems? Did animals "invent" electrical cognition, or did they inherit and elaborate a capacity that was already universal?

---

## Part II: Thought Experiments

**Thought Experiment 1: The Last Cell**

Imagine the very last living cell on Earth, in the final moments before it divides. Its daughter cells will be the final generation of life on the planet. The environment is gradually becoming uninhabitable. The cell senses the approaching conditions — it detects rising pH, falling nutrient concentrations, rising temperature. It adjusts its metabolism, produces heat-shock proteins, attempts chemotaxis toward a cooler region. Ultimately, it fails.

Now consider: at what point in this sequence did the cell stop being a cognitive agent? When it stopped moving? When its sensors could no longer detect gradients? When it could no longer produce the proteins required for its function? When it died?

If cognition is constitutive of life, then the answer should be: when it died. But is this satisfying? Is there a sense in which the cell was already a diminished cognitive agent before it died — as its cognitive capacities degraded one by one? What does your answer reveal about whether cognition is binary (a system either cognizes or it doesn't) or graded?

**Thought Experiment 2: The Protocell in the Primordial Soup**

Imagine a protocell at the origin of life: a lipid vesicle enclosing a self-replicating RNA molecule and a handful of metabolic reactions. It has no receptors in the modern sense. It has no cytoskeleton. It has no flagella. It has only its membrane, which is slightly permeable to some molecules and not others.

Does it cognize? Apply the SIA definition carefully:
- Does it *sense*? (Its membrane allows some molecules in and excludes others — this affects the chemistry inside)
- Does it *integrate*? (The internal chemistry responds to the composition of its interior)
- Does it *act*? (It grows or doesn't grow, it divides or doesn't divide, depending on the chemical environment)

If you conclude it does cognize, in some minimal sense, are you comfortable with this conclusion? If not, what additional element would you require before granting cognitive status — and can you justify that requirement without simply moving the goalpost to exclude entities you already decided weren't cognitive?

**Thought Experiment 3: Reversing the Definition**

Take the SIA definition — sensing, integrating, acting in service of continued organizational integrity — and apply it to the following non-biological systems:
- A computer running an adaptive algorithm (e.g., a machine learning model that adjusts its parameters in response to feedback)
- A market economy (which detects supply and demand, integrates information through price signals, and acts via the behavior of individual agents)
- A hurricane (which maintains its organized structure by feeding on the warm ocean, detecting temperature gradients, and developing a characteristic rotational structure)

For each: does the system cognize on the SIA definition? If yes: are you comfortable with this? If no: what feature of the definition excludes it, and does this feature represent a principled distinction or an ad hoc exclusion?

---

## Part III: Laboratory Investigations

**Lab 1: Membrane Permeability and Primitive Sensing**

Construct simple lipid vesicles from phospholipids (or the simpler fatty acids that primordial cells may have used — oleic acid vesicles are well-documented in the origin-of-life literature).

- Load the vesicles with a pH-sensitive fluorescent dye (such as BCECF or pyranine)
- Expose the vesicle solution to various pH gradients
- Observe and measure fluorescence changes inside the vesicles as a function of external pH

This models the simplest possible form of environmental sensing: passive detection of external conditions by an enclosed chemical system.

*Analysis questions*:
- Do the vesicles "sense" pH in any meaningful sense? On what definition?
- What would need to be added to the vesicles to make their response to pH adaptive (i.e., goal-directed in service of their continued organization)?
- Design a modification to the protocol that would test whether the vesicles' response to pH affects their structural integrity in a way that is relevant to their "survival"

*Materials*: Phosphatidylcholine or oleic acid, fluorescent pH dye (BCECF-AM), buffer solutions at various pH, fluorescence microscope

**Lab 2: Modeling Autopoiesis Computationally**

Using a computational modeling platform (NetLogo is free and accessible; Python with NumPy also works), implement a simple simulated autopoietic system:

- Model a 2D grid in which "membrane" components diffuse slowly and "substrate" components diffuse faster
- Implement a simple rule: membrane components that encounter substrate components are catalyzed to reproduce (producing more membrane components)
- Track the system's behavior: does it self-organize into a stable boundary? Does the boundary maintain itself against perturbation?

*Extensions*:
- Add a "metabolic" component inside the boundary that catalyzes the production of membrane components from substrate
- Introduce perturbations (remove a section of the boundary) and observe recovery
- Vary the rates of production and degradation to find the conditions under which autopoiesis is stable

*Analysis questions*:
- Under what parameter conditions does the simulated system exhibit organizational closure?
- What happens to the system when the rate of membrane component degradation exceeds the rate of production? Is this analogous to cell death?
- Does your simulated system satisfy the SIA definition of minimal cognition? What elements are present and which are absent?

**Lab 3: The Lac Operon as a Decision Device**

Using publicly available data and the interactive models in the Virtual Cell modeling platform (vcell.org) or BioNetGen, build a model of the E. coli lac operon regulatory system.

- Model the two-input regulation: glucose presence (catabolite repression via CRP) and lactose presence (LacI repressor inactivation)
- Simulate the system's response to four conditions: glucose only, lactose only, both glucose and lactose, neither

*Analysis questions*:
- Does the lac operon implement a logical AND gate? What are the inputs and output?
- Does the system's response constitute "integration" in the SIA sense? On what grounds?
- Now consider: the lac operon's response is not perfectly digital — there is a graded response at intermediate glucose and lactose concentrations. What does this graded response tell us about the "decision-making" capacity of the system?
- Design an experiment (real or simulated) that would test whether the lac operon constitutes a primitive predictive system — one that pre-emptively switches metabolic strategies in anticipation of environmental change, rather than simply responding reactively.
