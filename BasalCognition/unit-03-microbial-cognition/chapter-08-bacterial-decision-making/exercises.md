# Chapter 8 Exercises: Bacterial Decision-Making

---

## Part I: Reflection and Discussion

**1. The decision concept applied to bacteria**
The chapter argues that bacteria "make decisions" in a meaningful sense. A reductionist critic might respond: bacteria are just chemistry — complex chemistry, certainly, but not decision-making in any sense that deserves a cognitive label. Engage with this objection. What would it take for the reductionist to be right? Is there a principled distinction between "decision-making" and "complex chemistry," or is decision-making just a description of a particularly sophisticated type of complex chemistry?

**2. Individual vs. population decision-making**
Persister cell formation is described as a "population-level bet-hedging decision" but the mechanism is individual (stochastic fluctuations in individual cells). Is there such a thing as a population-level decision that is genuinely not reducible to the decisions of individual cells? Or is "population-level decision" just a convenient description of the statistical outcome of many individual stochastic events?

**3. The temporal memory constraint**
Bacterial chemotaxis memory operates on a timescale of 1-10 seconds; transcriptional memory on minutes to hours; CRISPR memory indefinitely. What determines these different timescales? Is there a constraint — physical or evolutionary — that prevents bacteria from having longer-timescale individual learning? What would a bacterium look like if it could individually learn on a one-month timescale?

**4. Graduated vs. threshold responses**
The SOS response activates different genes at different damage thresholds — a graduated response. The sporulation decision is essentially bistable — a threshold commitment. What determines whether a stress response should be graduated or bistable? Are there environments where the wrong type of response (graduated when threshold is needed, or vice versa) would be catastrophic? Can you identify real examples in bacterial biology?

**5. The pathogen's perspective**
Many bacterial pathogens use stress response systems — particularly the stringent response — to activate virulence genes. From the perspective of the cognitive framework, is pathogen virulence a form of adaptive decision-making? Does the fact that the decision harms another organism change its status as a cognitive act? Can we evaluate the "intelligence" of a decision independent of its moral status?

---

## Part II: Thought Experiments

**Thought Experiment 1: The Prescient Bacterium**
Imagine you have engineered an *E. coli* cell that has the following modification: its chemotaxis methylation system has been modified so that instead of tracking attractant concentration over the past 1 second, it tracks the past 1 hour. The cell now compares "now" to "one hour ago" rather than "one second ago." Predict what happens to this cell's chemotaxis behavior in: (a) a static gradient, (b) a rapidly fluctuating chemical environment (concentration changes every 10 seconds), (c) a slowly evolving environment (concentration changes every 2 hours). Does the modified cell do better or worse in each environment? What does this tell you about the "optimal" memory timescale for chemotaxis? Is there a universal optimal, or does it depend on environmental statistics?

**Thought Experiment 2: The Committed Bacterium**
Consider a bacterium that has committed to sporulation — the Spo0A-P positive feedback loop has passed the bistable threshold and the sporulation program has been initiated. Now imagine that nutrients suddenly become available — the environmental signal that originally triggered sporulation stress disappears. The committed bacterium continues toward sporulation anyway, because the bistable switch is self-sustaining. A closely related bacterial strain, however, has a modified sporulation switch that is not fully bistable — it can still be reversed by a large enough nutrient signal even after commitment. Under what environmental conditions is the irreversible (bistable) version more fit? Under what conditions is the reversible version more fit? What general principle does this suggest about when irreversible decisions are adaptive?

**Thought Experiment 3: Designing a Smarter Bacterium**
You have been challenged to design modifications to the *E. coli* decision-making system that would make the bacterium "smarter" — better adapted to a specific, known environment. The environment is: a laboratory chemostat with continuous culture, where glucose is the limiting nutrient, temperature is constant, and pH is controlled. Glucose concentration fluctuates with a period of exactly 2 hours (a sinusoidal oscillation between 0.1 mM and 1 mM). What modifications would you make to: (a) the chemotaxis system, (b) the carbon catabolite repression system, (c) the stringent response? Would your "smarter" bacterium do better in this specific environment? Would it do worse in other environments? What general principle about the cost of intelligence does this illustrate?

---

## Part III: Laboratory Investigations

**Lab 1: Quantifying Chemotaxis Performance (wet lab or simulation)**
Measure the efficiency of *E. coli* chemotaxis in a standardized soft agar assay, and determine how it scales with gradient steepness.

*Procedure*: Prepare minimal medium soft agar plates (0.3% agar, no attractant) and stab them with *E. coli* culture. After 12 hours at 37°C, measure the diameter of the chemotaxis ring (the expanding ring of bacteria migrating through the agar toward oxygen/nutrients at the colony periphery). Alternatively, use aspartate-containing wells to create a defined gradient.

*Analysis*: Measure ring diameter at 6, 12, 18, and 24 hours. Calculate the expansion velocity. Compare chemotaxis-proficient wild-type to a CheA knockout (non-chemotactic) to determine the contribution of directed chemotaxis to colony expansion.

*Discussion*: How does the soft agar assay approximate real-world chemotaxis? What aspects of natural chemotaxis does it capture, and what does it miss? How would you modify the assay to measure adaptation performance specifically?

**Lab 2: Modeling the SOS Response (computational)**
Implement a mathematical model of the SOS response and explore the computational properties of its threshold architecture.

*Model*: Implement a system of three variables: DNA damage D (input), LexA repressor concentration L, and SOS gene expression E. Write equations: dD/dt = -repair_rate * D + damage_input; dL/dt = synthesis_rate - degradation_rate * L - autocleavage_rate * RecA(D) * L; dE/dt = transcription_rate / (1 + (L/K)^n) - degradation_rate_E * E. Set RecA(D) = D^m / (K_RecA^m + D^m) (a Hill function for RecA activation by ssDNA).

*Procedure*: Simulate the response to step increases in damage_input at three levels (low, medium, high) and to a pulse of damage (brief high damage followed by return to baseline). Vary the Hill coefficient n for LexA repression (n = 1, 2, 4).

*Analysis*: How sharp is the threshold for SOS activation? Does the system show hysteresis (does the SOS response turn off at the same damage level at which it turned on)? How does changing the Hill coefficient affect the threshold sharpness?

*Discussion*: The SOS response is described as having "early" and "late" genes with different LexA binding affinities. Extend your model to include two output variables (E1 and E2 with different K values for LexA repression) and observe how the response is graduated across damage levels.

**Lab 3: Persister Cell Quantification (wet lab)**
Measure the frequency of persister cells in a bacterial population and determine how it changes under different growth conditions.

*Procedure*: Grow *E. coli* in LB medium to different growth phases: early log (OD600 = 0.1), mid-log (OD600 = 0.5), late log (OD600 = 1.0), and stationary phase (overnight culture). Add ampicillin (10x MIC) to each culture for 3 hours to kill non-persisters. Plate before and after ampicillin treatment on LB plates (with no antibiotic) to count surviving persisters.

*Analysis*: Calculate persister frequency (survivors/initial count) for each growth phase. Is persister frequency constant across growth phases, or does it change? What does this tell you about the regulation of persister formation?

*Discussion*: If you were a bacterium, at what growth phase would you maximize persister formation? Does the actual pattern you observe match your prediction? What does a mismatch (if any) tell you about the constraints on persister regulation?

---

*For further study, see the Further Reading list for Chapter 8.*
