# Chapter 5 Exercises: Bioelectricity — The Ancient Language of Life

---

## Part I: Reflection and Discussion

**1. The resting potential as a decision variable**
Every cell in your body maintains a resting membrane potential, continuously, using a significant fraction of its ATP. If this potential were merely a physical consequence of having a membrane, evolution might have found ways to eliminate it and save energy. What argument can you make that the membrane potential is itself a regulated variable — a parameter that cells actively set at specific values for functional reasons? What would it predict about cells in different functional states (dividing, quiescent, differentiating, dying)?

**2. Bioelectric vs. molecular information**
The chapter describes bioelectric patterns and molecular gradient patterns (morphogen concentrations) as two distinct channels of developmental information. Are they truly independent channels, or are they better understood as two aspects of a single integrated information system? Consider: what would it mean for two developmental signals to be "truly independent"? By that criterion, are bioelectric and molecular signals independent?

**3. The representation question**
Section 3 raises the philosophical question of whether bioelectric patterns in developing tissues are genuine representations of target morphologies, or merely physical correlates that happen to guide morphogenesis. What would convince you that the bioelectric pattern is a representation rather than just a pattern? Is the distinction philosophically meaningful or merely semantic?

**4. Cancer as defection from a collective**
Levin's framing of cancer as "bioelectric defection" — a cell withdrawing from the tissue-scale bioelectric community — resonates with multilevel selection theory, which sees cancer as a breakdown of the cooperative agreements that multicellular organisms require. How far does this analogy go? Does it make specific predictions about which tumor types should be most amenable to bioelectric intervention, and which should not? What evidence would test those predictions?

**5. Xenobots and the limits of the genome**
The Xenobot result suggests that cells can exhibit behaviors that are not encoded in their genome but emerge from their morphological and bioelectric context. If this is true, it means that the "same" genome can produce radically different behavioral repertoires depending on context. What implications does this have for how we understand genetic causation? Is it an argument for or against genetic determinism?

---

## Part II: Thought Experiments

**Thought Experiment 1: The Bioelectric Inverter**
Imagine you have access to a perfectly specific optogenetic tool that can set the membrane potential of any individual cell in a developing organism to any desired voltage, without affecting neighboring cells. You use this to create a planarian in which the normal bioelectric gradient along the anterior-posterior axis is perfectly inverted — cells that would normally be most hyperpolarized (at the head end) are made most depolarized, and vice versa. Predict what would happen to: (a) normal planarian regeneration, (b) the expression patterns of Wnt pathway genes, (c) the animal's behavior if it survives. Now consider: would the inverted bioelectric pattern constitute a "representation" of a different (inverted) body plan? Would restoring the normal bioelectric pattern in an adult that had developed with an inverted pattern reverse its morphology?

**Thought Experiment 2: The Galvanotactic Immune System**
Consider an organism whose entire immune system has been genetically engineered to navigate exclusively by galvanotaxis — it cannot respond to any chemical attractants, only to electric fields. Describe the benefits and vulnerabilities of this immune system relative to the normal chemotaxis-based system. Under what conditions would galvanotactic immune cells perform better than chemotactic ones? Under what conditions would they fail? What does this thought experiment reveal about the informational properties of electric fields versus chemical gradients as signaling media?

**Thought Experiment 3: The Sleeping Xenobot**
A Xenobot exhibits behaviors (maze navigation, self-replication) that were not anticipated from the genomes of its cells. Now imagine that the cells comprising a Xenobot undergo a period of "sleep" — all gap junction communication is transiently suppressed, all ion channels close, and all membrane potentials equalize to zero. After this period, the Xenobots re-awaken with normal membrane potentials restored. Do they remember what they were doing? Do they re-establish the same bioelectric patterns? Are they the same individuals? This thought experiment probes: what is the carrier of bioelectric memory, and what constitutes the "identity" of a bioelectric agent?

---

## Part III: Laboratory Investigations

**Lab 1: Measuring Wound Healing Electric Fields (wet lab)**
Wounds in epithelial tissues generate measurable electric fields that can be detected with simple equipment.

*Procedure*: Using a Petri dish coated with collagen or fibronectin, culture a confluent monolayer of an epithelial cell line (MDCK cells or similar). Use a sterile pipette tip to scratch a straight wound. Immediately add the voltage-sensitive dye DiBAC4(3) to the medium at the manufacturer's recommended concentration, and observe fluorescence under an epifluorescence microscope. Cells with more depolarized membranes will show stronger fluorescence.

*Analysis*: Image the wound region at 0, 30, 60, and 120 minutes after wounding. Measure fluorescence intensity as a function of distance from the wound edge. Calculate the rate of wound closure from phase-contrast images taken at the same time points. Test whether the region of altered membrane potential correlates spatially with the region of active cell migration.

*Discussion*: What would you predict about wound healing if you added a gap junction blocker (such as carbenoxolone) to the medium? Design an experiment to test this prediction.

**Lab 2: Modeling Bioelectric Pattern Formation (computational)**
Using a reaction-diffusion modeling framework (MATLAB, Python, or a web-based simulator like the Virtual Cell or the Morpheus platform), implement a simple model of bioelectric patterning in a one-dimensional tissue.

*Model*: Define a row of 100 cells, each with a membrane potential V that decays toward a resting value V_rest with a time constant tau, and that is influenced by gap junction coupling (V spreads to neighbors with coupling constant c) and by a local ion channel (which activates above a threshold V_thresh to further depolarize the cell).

*Procedure*: Start with all cells at V_rest. Apply a local perturbation (depolarize cells 45-55 to twice the normal value). Simulate the propagation of the bioelectric perturbation as a function of c (coupling strength) and tau (decay time constant).

*Analysis*: Under what conditions does the perturbation spread across the entire tissue? Under what conditions does it remain localized? Is there a critical coupling strength for "phase transition" behavior? What does this model teach you about the conditions under which bioelectric patterns can encode long-range spatial information?

**Lab 3: Galvanotaxis Assay (wet lab)**
Galvanotaxis — directed cell migration in an electric field — can be demonstrated in cell culture with simple equipment.

*Procedure*: Build a simple galvanotaxis chamber using two pieces of agar-saline gel bridges as electrodes, connected to a 1.5V battery through a resistor to achieve a field of approximately 1-3 V/cm across a standard Petri dish. Seed keratocytes (fish skin cells) or macrophages in the chamber at low density. Apply the field and image cell migration over 2-4 hours using time-lapse microscopy.

*Analysis*: Measure the angle of migration for each tracked cell relative to the field direction. Calculate the mean cosine of the migration angle (a measure of directional bias; 0 = random, 1 = perfect alignment with field). Compare migration speed and directionality in field vs. no-field conditions.

*Discussion*: How does the strength of galvanotaxis compare to published values for chemotaxis toward known attractants? Does this tell us anything about the relative "weight" cells give to bioelectric vs. chemical gradient cues?

---

*For further study, see the Further Reading list for Chapter 5.*
