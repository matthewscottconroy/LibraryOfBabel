# Chapter 7 Exercises: Homeostasis, Allostasis, and Goal-Directedness

---

## Part I: Reflection and Discussion

**1. Set points and norms**
The concept of a "set point" implies that a system has a preferred value for a regulated variable — not just any stable value, but a specific target. But who or what decides what the set point is? In a thermostat, the set point is externally programmed. In a cell, the set point for pH or calcium seems to be determined by the molecular properties of the enzymes involved. Is a cell's set point "chosen" in any meaningful sense? By what criteria should we evaluate whether a set point is appropriate?

**2. Allostasis and the brain as predictor**
Sterling's allostatic framework requires that the brain maintains a predictive model of the body's anticipated demands. Is this the same claim as the free energy principle's requirement that all biological systems have generative models of their environments? If so, is the FEP just allostasis stated more formally? If not, what distinguishes the two frameworks?

**3. The tautology objection to the FEP**
Engage seriously with the objection that the free energy principle is tautological: any system that persists must, by definition, be in states compatible with its persistence — and this is what the FEP says. Is this tautology a problem? Can a true proposition about all living systems fail to make interesting predictions about any specific living system? How would you design an experiment that could, in principle, falsify the FEP as applied to a bacterium?

**4. Valence without experience**
The chapter argues that valence — the property of states being positive or negative — is a functional property of homeostatic regulatory systems, present in bacteria as "proto-valence." A critic might object that valence requires subjective experience: without something it is like to be the organism, there is nothing for the valence to be "for." How would you defend the functional concept of valence against this objection? Does the objection establish that bacterial proto-valence is a different kind of thing from animal valence, or merely a simpler version of the same thing?

**5. Goal-directedness without goals**
The chapter argues that homeostatic systems exhibit "functional goal-directedness" without requiring an explicit representation of a desired state. Is this coherent? Can a system be goal-directed without having goals? Contrast this with the position that goal-directedness is only meaningful when applied to systems that have genuine representations of desired outcomes. Which position do you find more defensible, and why?

---

## Part II: Thought Experiments

**Thought Experiment 1: The Homeless Cell**
Imagine a liver cell that is somehow extracted from its normal tissue context and placed in a perfectly controlled medium: all nutrients maintained at ideal concentrations, temperature maintained at 37°C, pH maintained at 7.2, oxygen at physiological partial pressures. The cell's homeostatic machinery now has nothing to do — all its set points are being maintained externally, with zero deviation at all times. What would happen to the cell? Would it continue normal behavior? Would its homeostatic machinery atrophy from disuse? Would it "know" that its homeostatic needs are being met by an external agent rather than by its own efforts? This thought experiment probes: what is the relationship between homeostatic effort and cellular identity?

**Thought Experiment 2: Conflicting Set Points**
Consider a bacterium whose set point for internal potassium concentration is 200 mM but whose set point for membrane potential is -80 mV. Increasing internal potassium tends to depolarize the membrane (moving toward less negative potential). The bacterium thus faces two homeostatic demands that, in its current ionic environment, pull in opposite directions: maintaining potassium requires action that disrupts membrane potential, and maintaining membrane potential requires action that depletes potassium. Describe the dynamics of this system in terms of the interplay between the two homeostatic loops. Is there a stable resolution? What determines which set point "wins"? Now generalize: what does this tell us about how biological systems resolve conflicts between homeostatic demands, and is the resolution itself a form of decision-making?

**Thought Experiment 3: The Allostatic Bacterium**
You discover a bacterium that, unlike normal bacteria, exhibits clear allostatic rather than purely homeostatic regulation of its glucose transport. Specifically, when you add a chemical signal X to the medium (which, in the bacterium's natural environment, reliably precedes a period of glucose scarcity), the bacterium immediately reduces its internal set point for glucose consumption by 50% — a preemptive reduction before any glucose scarcity has actually occurred. Describe the molecular architecture that could implement this allostatic adjustment. What would have to be true about the regulatory connections between the X-sensing pathway and the glucose metabolism regulatory network? Under what evolutionary conditions would this allostatic adjustment have been selected for? What would be the cost if signal X is an unreliable predictor of scarcity?

---

## Part III: Laboratory Investigations

**Lab 1: Cellular pH Homeostasis Under Metabolic Stress (wet lab)**
Investigate how yeast cells (or bacteria) maintain internal pH under metabolic stress conditions.

*Procedure*: Use the pH-sensitive fluorescent protein pHluorin (or the dye BCECF-AM in mammalian cells) to measure intracellular pH in real time. Grow cells to mid-log phase. Measure baseline intracellular pH by fluorescence ratiometry. Add sodium acetate at concentrations of 0, 10, 50, and 100 mM (an acidifying stress) and measure intracellular pH over 30 minutes. Then add carbonyl cyanide m-chlorophenylhydrazone (CCCP, an uncoupler of the proton gradient) and measure the response.

*Analysis*: How rapidly does internal pH recover after the acidification challenge? How large a perturbation can the cells compensate for within 5 minutes? Does CCCP treatment (which depletes the proton motive force) impair pH recovery? What does this tell you about the energy requirements of cellular pH homeostasis?

*Discussion*: The recovery of intracellular pH after acidification is itself a homeostatic response. Can you identify which molecular mechanisms are likely responsible, based on the pharmacological responses you observe?

**Lab 2: Modeling Homeostasis and Allostasis (computational)**
Compare homeostatic and allostatic control strategies using a simple mathematical model.

*Model*: Implement a one-variable homeostatic controller: dx/dt = -k(x - x_0) + noise, where x is the regulated variable, x_0 is the set point, k is the feedback gain, and noise is Gaussian white noise with standard deviation sigma. Now implement an allostatic version: the set point x_0 itself changes according to a predictive signal P: dx_0/dt = alpha * (P - x_0), where P is a predicted future demand (you can model this as a sinusoidal oscillation representing, say, anticipation of a daily cycle of demand).

*Procedure*: Simulate both versions with k = 1, sigma = 0.1, alpha = 0.5, and P = 1.0 + 0.5*sin(2*pi*t/24). Run for 72 simulated hours. For the allostatic version, introduce a lag of 1 hour in the predictive signal (so x_0 tracks P but with a 1-hour delay).

*Analysis*: Compare the variance of x around its target under homeostatic vs. allostatic control. Does allostatic control reduce variance? Does it matter whether the prediction is accurate or lagged? What happens to the system's behavior if P suddenly changes (simulate an unexpected environmental shift at t=36)?

*Discussion*: What does this model teach you about when allostatic control is superior to homeostatic control? Under what conditions is the predictive investment of allostasis not worth the cost?

**Lab 3: Free Energy Principle in Bacterial Chemotaxis (analytical/conceptual)**
Apply the FEP framework to the *E. coli* chemotaxis system and assess whether it generates novel predictions.

*Procedure*: Using the published biophysical parameters of the *E. coli* chemotaxis system (Berg, 2004), construct a description of the system in FEP terms: identify the Markov blanket (the cell membrane and its receptor complex), the internal states (receptor methylation level, CheY-P concentration), the active states (flagellar motor switching), and the sensory states (receptor occupancy). Formulate the "generative model" of the bacterium: what does it "expect" its sensory states to be? What would constitute a "surprise" in FEP terms?

*Analysis*: Compute the variational free energy of the system in three conditions: (1) the bacterium is in a spatially uniform attractant field (no gradient); (2) the bacterium is swimming up a gradient (favorable direction); (3) the bacterium is swimming down a gradient (unfavorable direction). Which condition has the lowest free energy according to the FEP?

*Discussion*: Does the FEP account of *E. coli* chemotaxis predict any behaviors that are not already predicted by the standard mechanistic description? If not, what is the added value of the FEP framework? If yes, describe the novel prediction and sketch an experiment to test it.

---

*For further study, see the Further Reading list for Chapter 7.*
