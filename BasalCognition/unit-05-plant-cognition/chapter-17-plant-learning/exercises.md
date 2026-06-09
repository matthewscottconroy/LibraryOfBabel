# Chapter 17: Exercises

## Part I: Reflection and Discussion Questions

**1. Distinguishing learning from adaptation**
The most important conceptual challenge in plant learning research is distinguishing genuine learning (lasting behavioral modification based on experience that satisfies operational criteria from the animal learning literature) from sensory adaptation (transient decreases in receptor sensitivity with sustained stimulation) and fatigue (temporary decreases in effector capacity due to resource depletion). Design a flowchart or decision tree for a researcher studying a putative plant learning phenomenon: what sequence of experiments would you require to be confident that you are observing genuine learning rather than one of the alternatives?

**2. Vernalization as memory**
Vernalization satisfies all functional criteria for memory (encoding, storage, retrieval, specificity, adaptive function). Yet it operates through chromatin modification rather than synaptic plasticity. Does the different mechanism matter for whether we call vernalization "memory"? What are the scientifically important properties that memory should have, and which of them does vernalization share or not share with neural memory?

**3. The replication problem**
The Gagliano et al. (2014) habituation paper and the Jiang et al. (2018) replication produced different results, particularly regarding the persistence of the reduced response after rest. How should the scientific community respond to this situation? What is the appropriate weight to give each study? What additional experiments are needed? Who should conduct them? Does the choice of who conducts replication studies matter for how we interpret their results?

**4. Epistemic standards for extraordinary claims**
The claim that plants perform associative learning is extraordinary because it attributes to brainless organisms a capacity previously associated only with animals with nervous systems. What epistemic standard should we apply to extraordinary claims? Is "extraordinary claims require extraordinary evidence" a principled standard, or is it simply a way of protecting prior assumptions from revision? How do you decide what counts as "extraordinary"?

**5. The cost of priming**
Systemic acquired resistance and stress priming represent an adaptive memory system: the plant pays a small ongoing cost (maintaining primed chromatin states) to reduce the cost of future pathogen encounters (faster, stronger immune responses). Analyze this as an economic optimization problem. Under what ecological conditions would priming be adaptive? When would it not be? Does the analogy between plant immune priming and animal immunological memory hold up quantitatively?

---

## Part II: Thought Experiments

**1. The Perfect Plant Learning Experiment**
Design what you consider to be the gold-standard experiment for demonstrating associative learning in plants. Specify: the plant species, the conditioned and unconditioned stimuli, the training protocol, all the controls you would include, the sample size and statistical power, the criteria for success, and how you would distinguish genuine associative learning from alternative explanations. Then consider: what would it mean for our understanding of plant biology if this experiment succeeded? What would it mean if it failed?

**2. The Memoryless Plant**
Imagine a mutant Arabidopsis plant that is unable to form any epigenetic modifications — its chromatin state is fixed and cannot be altered by environmental experience. How would this plant behave differently from wild type? Consider: vernalization, stress priming, the defense response to repeated pathogen attack, and daily growth rhythms (if any depend on chromatin state changes). What does this thought experiment reveal about the role of epigenetic plasticity in plant adaptation?

**3. The Heritable Experience**
Suppose evidence emerges that Mimosa plants that have habituated to a drop stimulus produce seeds that germinate into plants that are more quickly habituated by the same stimulus — and that this difference is specifically due to epigenetic modifications transmitted through the seeds. If this were true, what would be its evolutionary implications? Would it change how you evaluate the Gagliano habituation paper? Would it vindicate or further complicate the "plant learning" claim?

---

## Part III: Laboratory Investigations

**1. Mimosa Habituation Study (Pre-registered)**
*Goal*: Conduct a well-controlled investigation of Mimosa leaflet folding in response to repeated stimulation.
*Materials*: Mimosa pudica plants (grown from seed or purchased), drop apparatus (a rail system for reproducible drops), ruler, camera, video analysis software.
*Procedure*: (a) Register your hypotheses and analysis plan before collecting data. (b) Grow 30+ plants in identical conditions. (c) Randomly assign 15 to the "dropped" condition (60 drops, one per 5 seconds) and 15 to control (no drops). (d) Record leaflet closure angle after each drop for the experimental group. (e) After 24 hours, test all plants with 5 drops. (f) After 1 week, test again.
*Analysis*: Does closure decrease over the drop series? Does it recover after 24 hours? After 1 week? Is there a statistically significant difference between experimental and control plants at the 1-week test?

**2. Stress Priming in Arabidopsis**
*Goal*: Observe systemic acquired resistance by comparing the defense response of naive and previously infected plants.
*Materials*: Arabidopsis thaliana plants, a mild pathovirus (e.g., a commercially available attenuated pathogen or a bacterial suspension), quantitative RT-PCR equipment for measuring PR gene expression.
*Procedure*: (a) Infect 3 leaves of 10 plants with the pathogen, leave 10 plants uninfected. (b) After 72 hours, measure PR1 and PR2 gene expression in systemic (uninfected) leaves of both groups using qRT-PCR. (c) One week later, infect both groups systemically and measure defense gene expression at 24 and 48 hours post-infection.
*Analysis*: Do previously infected plants show higher basal PR gene expression in systemic leaves? Do they show faster or stronger induction of PR genes after systemic infection?

**3. Modeling Epigenetic Memory**
*Goal*: Understand the dynamics of chromatin-state-based memory using a mathematical model.
*Materials*: Computer with Python or MATLAB.
*Procedure*: Model the FLC chromatin state as a variable C that ranges from 0 (fully active) to 1 (fully silenced by H3K27me3). Implement: (a) during cold exposure, C increases at a rate proportional to (1-C) × cold_signal, representing progressive PRC2-mediated silencing; (b) during warm periods, C decreases at a slow rate (representing slow reactivation); (c) at each cell division, C is passed to daughter cells without loss. Simulate the vernalization of a plant that experiences 0, 4, 8, and 12 weeks of cold before returning to warm conditions.
*Analysis*: How does the final FLC silencing level depend on the duration of cold exposure? At what cold duration is the silencing effectively irreversible? How does the cell division term affect the model's behavior?
