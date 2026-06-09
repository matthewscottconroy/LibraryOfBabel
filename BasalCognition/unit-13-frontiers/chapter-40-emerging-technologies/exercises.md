# Chapter 40: Exercises

## Part I: Reflection and Discussion

1. **The limits of imaging.** Fluorescent reporters and optogenetic tools give us unprecedented ability to observe and control the internal states of biological systems. But all these tools involve perturbation: expressing a fluorescent protein changes the proteome; illuminating cells with light produces heat and can generate reactive oxygen species. How serious are these perturbations for basal cognition research, and how can experiments be designed to assess and minimize their effects? What kinds of cognitive behaviors are most and least susceptible to perturbation by imaging tools?

2. **Models and mechanisms.** The Tero et al. (2010) model of *Physarum* network optimization accurately predicts network topology but does not specify the molecular mechanism. What would it take to connect this computational model to the biophysical mechanism? What measurements would be necessary? What would you expect the mechanistic account to add to the computational one — would it change any of the model's predictions, or just explain how the computation is implemented?

3. **Directed evolution of cognition.** The section proposes using directed evolution to evolve cognitive capacities in simplified bacterial systems. What selection pressure would you use to evolve anticipatory behavior — behavior that prepares for an event before it occurs? How would you design the selection protocol so that the bacteria that survive and reproduce are those that anticipate the event, rather than those that simply respond to it when it occurs? What control experiments would be necessary?

4. **The reconstruction strategy's limits.** A synthetic circuit that achieves chemotaxis-like behavior using different molecular components than natural *E. coli* chemotaxis demonstrates that the function can be implemented in multiple ways. What does this imply about the relationship between the function and the mechanism in natural systems? Does it support or undermine the argument that understanding the natural mechanism is important for understanding basal cognition?

5. **Machine learning and explanation.** Deep learning models can predict the behavior of biological systems from high-dimensional input data, but the predictions are often difficult to interpret mechanistically. Is a predictive model without mechanistic interpretation scientifically valuable in basal cognition research? What additional steps would be needed to move from prediction to explanation? Can deep learning models generate mechanistic hypotheses, or only correlational predictions?

---

## Part II: Thought Experiments

1. **The complete sensor.** Imagine a technology that could measure, simultaneously and without perturbation, every molecular species in every cell of a living organism, at single-cell spatial resolution and millisecond temporal resolution. How would the availability of this technology change basal cognition research? What questions would it answer? What questions would it leave open? And what does your answer reveal about the relationship between measurement and understanding in biological science?

2. **The minimal cognitive organism.** You are given the task of designing, from scratch, the smallest possible organism that exhibits genuine cognitive behavior — behavior that meets at least three of the five criteria discussed in Chapter 39 (flexibility, memory, anticipation, goal-directedness, non-linear integration). You can use any biological parts from the synthetic biology toolkit. What is the minimum number of genes you need? What are the essential components? What emergent cognitive behavior would you expect your organism to exhibit? What would surprise you?

3. **Optogenetics and free will.** A thought experiment: suppose you could control, with light, the activity of every neuron in a human brain with single-cell, millisecond precision. Could you, in principle, control the behavior of that person completely? Would the person's behavior under such control count as voluntary? Does this thought experiment reveal something about free will — or only about the relationship between neural activity and behavior? Now apply the same thought experiment to a bacterium: if you could control every ion channel in a bacterium with light, would you be controlling the bacterium's "decisions," or eliminating them?

---

## Part III: Laboratory and Computational Investigations

1. **Implement a reaction-diffusion model.** Using Python (with NumPy and Matplotlib, or the pyDiDi or reaction-diffusion-py packages), implement a 2D Turing reaction-diffusion model with an activator-inhibitor system. Start from a near-uniform initial condition with small random perturbations. Simulate the system until a stable pattern forms, and plot the result. Vary the ratio of activator to inhibitor diffusion constants and observe how the pattern changes. Identify the parameter range in which spots, stripes, and labyrinthine patterns form. Discuss what this parameter sensitivity reveals about how biological organisms might tune Turing dynamics to produce specific body patterns.

2. **Agent-based model of quorum sensing.** Using an agent-based modeling platform (NetLogo is free and well-documented), implement a simplified quorum sensing model: bacteria produce and detect an autoinducer molecule; when the local autoinducer concentration exceeds a threshold, bacteria switch from individual to collective behavior (e.g., forming a biofilm). Simulate the model with varying initial conditions, bacterial densities, and autoinducer diffusion rates. Identify the conditions under which collective behavior emerges synchronously across the population, and those under which it emerges heterogeneously. Discuss what these simulations reveal about the information-processing properties of quorum sensing.

3. **Optogenetics literature review: Non-neural applications.** Conduct a systematic search of papers applying optogenetic tools to non-neural biological systems (plants, bacteria, yeast, or developing embryos) published in the past five years. For each paper, identify: (a) which optogenetic tool was used; (b) which biological process was controlled; (c) what cognitive-relevant insight, if any, the control experiment provided. Compile your findings into a structured review and assess: what are the most significant cognitive insights that have emerged from optogenetics in non-neural systems? What are the most important remaining questions that better optogenetic tools could address?
