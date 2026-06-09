# Chapter 10 Exercises: Biofilms — Collective Architecture

---

## Part I: Reflection and Discussion

**1. Biofilm vs. multicellular organism**
Biofilms exhibit cellular differentiation, division of labor, spatial organization, intercellular communication, and coordinated collective behaviors. These are also features of multicellular organisms. Is there a principled distinction between a mature *B. subtilis* biofilm and a simple multicellular organism? If so, what is it? If not, should we reconceptualize biofilms as a form of primitive multicellularity?

**2. The electrical wave and neural comparison**
The potassium wave in *B. subtilis* biofilms uses the same ionic currency and similar propagation principles as neural action potentials, but operates on a timescale of minutes rather than milliseconds and propagates through intercellular space rather than along a cell membrane. Is this analogy informative (revealing deep computational principles shared by biofilm and neural signaling) or misleading (concealing the enormous difference in speed, specificity, and computational sophistication between the two systems)? How would you evaluate claims that "biofilms are like brains"?

**3. Sacrificial cells and altruism**
The phenomenon of regulated cell death in biofilms — cells dying in ways that strengthen the biofilm matrix — has been described as "altruism." Evaluate this description. Is the behavior altruistic in the evolutionary sense (kin selection)? In the intentional sense (the cell "decides" to sacrifice itself)? In the functional sense (the death benefits the community)? Which sense of altruism is most appropriate, and does the choice matter for how we interpret the behavior?

**4. Resistance without resistance**
Biofilm tolerance to antibiotics — achieved through matrix barriers, metabolic dormancy, and metabolic heterogeneity — is mechanistically distinct from genetic antibiotic resistance. Does this distinction matter clinically? Evolutionarily? From the perspective of developing new therapies, is it more important that biofilm tolerance involves no genetic change, or that it is collective (an emergent property of community structure)?

**5. Collective memory in biofilms**
The chapter suggests that biofilms may exhibit community-level memory — persistent changes in collective state following prior experience. Describe the most rigorous experimental test you can design to demonstrate (or refute) the existence of collective memory in a biofilm. What would distinguish genuine community-level memory from individual cell memory that happens to persist in the community?

---

## Part II: Thought Experiments

**Thought Experiment 1: The Signal-Blocking Biofilm**
Imagine you have developed a drug that specifically blocks potassium channels in *B. subtilis* cells without killing them or affecting any other cellular function. You apply this drug to a mature *B. subtilis* biofilm. Predict: (a) what immediately happens to the potassium waves; (b) what happens to metabolic coordination between interior and peripheral cells over the next hour; (c) what happens to the metabolic state of interior cells over the next 24 hours; (d) what happens to the overall structure and viability of the biofilm over the next week. Compare your predictions to what would happen if you applied an antibiotic that kills dividing cells at the same biofilm. What does the comparison reveal about the relative importance of electrical coordination vs. active cell killing for biofilm integrity?

**Thought Experiment 2: The Biofilm at the Edge of Chaos**
Complexity science suggests that systems at the "edge of chaos" — between ordered and disordered behavior — exhibit maximal information processing capacity and adaptability. Consider a biofilm's electrical dynamics on a spectrum from fully ordered (no wave propagation, all cells at the same potential) to fully disordered (random, uncorrelated fluctuations). Where on this spectrum would you predict maximal biofilm adaptability to environmental challenge? Can you design an experiment that would measure where real biofilms fall on this spectrum and test whether more "edge-of-chaos" biofilms are indeed more adaptable?

**Thought Experiment 3: The Proto-Brain**
Propose a thought experiment in which you attempt to convert a biofilm into a simple information-processing device. You have the following tools: (1) genetic modifications to bacteria, (2) microfluidic chambers that let you deliver precise chemical or electrical stimuli to specific parts of the biofilm, (3) the ability to read electrical signals from the biofilm surface. Design a biofilm "circuit" that can implement a simple logic operation: it receives two inputs (a chemical at port A and a chemical at port B) and produces a detectable output (a change in light production or a metabolic shift) only if both inputs are present simultaneously (AND logic). What modifications to the bacteria would you need? What limitations would your biofilm logic gate have compared to an electronic logic gate? What does this thought experiment reveal about the computational potential and limitations of biofilm cognition?

---

## Part III: Laboratory Investigations

**Lab 1: Biofilm Formation and Quantification (wet lab)**
Quantify biofilm formation by bacterial strains under different conditions using the crystal violet assay.

*Procedure*: Inoculate overnight cultures of *E. coli* K-12, *P. aeruginosa* PAO1, or *S. aureus* in LB medium into 96-well polystyrene plates (200 µL per well, 1:100 dilution of overnight culture). Incubate at 37°C without agitation for 24 hours (static conditions). Remove planktonic cells by aspiration and washing with PBS (3×). Stain adherent biofilms with 0.1% crystal violet for 15 minutes. Wash again (3× PBS). Solubilize crystal violet with 30% acetic acid (200 µL per well) and measure OD590 in a plate reader.

*Analysis*: Compare biofilm formation under different conditions: LB vs. M63 minimal medium, 25°C vs. 37°C, polystyrene vs. glass wells, with and without glucose supplementation.

*Discussion*: Which conditions promote biofilm formation? Which inhibit it? How does this relate to what you know about the regulatory signals (c-di-GMP, QS) that control biofilm commitment? Design an experiment using biofilm-defective mutants (e.g., curli or fimbriae knockouts) to determine which matrix components are most important under your growth conditions.

**Lab 2: Antibiotic Tolerance of Biofilm vs. Planktonic Cells (wet lab)**
Measure the minimum biofilm eradication concentration (MBEC) and compare it to the minimum inhibitory concentration (MIC) for planktonic cells.

*Procedure*: Grow biofilms in the wells of a MBEC (also called Calgary Biofilm Device) peg lid plate system (or improvise with a polypropylene peg lid on a 96-well plate). After 24 hours of biofilm growth, transfer the peg lid to a fresh plate containing serial dilutions of antibiotic (ciprofloxacin, from 0.01 to 100 µg/mL). Expose for 24 hours. Transfer peg lid to recovery medium (antibiotic-free) and incubate for 24 hours. Read OD as a measure of viable cells.

*Analysis*: Calculate MIC from a parallel broth microdilution assay with planktonic cells. Calculate MBEC from the peg assay. What is the ratio MBEC/MIC? Does this vary across antibiotics with different mechanisms of action?

*Discussion*: If you were treating a biofilm infection with ciprofloxacin in a patient, and the MIC of the infecting strain is 0.1 µg/mL (susceptible by clinical breakpoints), what concentration would you need to achieve in the biofilm to eradicate it? Is this achievable with standard dosing? What does this suggest about why biofilm infections are so difficult to treat with standard antibiotic regimens?

**Lab 3: Modeling Biofilm Electrical Waves (computational)**
Implement a simple mathematical model of biofilm potassium wave propagation and explore the conditions for wave initiation and propagation.

*Model*: Implement a one-dimensional array of 100 bacterial "cells," each with a membrane potential V_i. Cell i is coupled to its neighbors by gap junction-like potassium exchange (current proportional to potential difference). Each cell has a potassium channel that opens when V_i exceeds a threshold V_thresh (positive feedback), releases potassium (depolarizing neighbors), and then inactivates with a time constant tau_inact. Implement as a system of differential equations.

*Procedure*: Stimulate cells 45-55 with a depolarizing current pulse (simulating metabolic stress in the biofilm interior). Observe wave propagation as a function of coupling strength, threshold, and inactivation time constant. Repeat with a "gap junction blocker" (set coupling constant to zero in cells 30-40) and observe whether the wave propagates past the gap.

*Analysis*: What coupling strength is required for wave propagation? Does the wave amplitude decrease with distance from the source? How does the inactivation time constant affect the wave frequency (how often waves can be generated)?

*Discussion*: Your model predicts specific relationships between ion channel properties and wave propagation dynamics. Identify one prediction that could be tested experimentally using bacteria with ion channel mutations. How would you measure the prediction, and what result would confirm or disconfirm it?

---

*For further study, see the Further Reading list for Chapter 10.*
