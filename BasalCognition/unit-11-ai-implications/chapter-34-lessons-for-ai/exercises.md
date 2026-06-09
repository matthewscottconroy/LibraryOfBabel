# Chapter 34: Exercises

## Part I: Reflection and Discussion

1. **The embodiment gap.** Brooks argued that "the world is its own best model." What does this claim mean precisely, and what are its limits? Are there classes of problems for which rich internal representation is genuinely necessary, even from an enactivist perspective? Give concrete examples from both AI systems and biological organisms.

2. **Energy as a cognitive constraint.** The human brain consumes roughly twenty watts; a comparable GPU cluster consumes megawatts. Does energy efficiency matter for intelligence, or is it purely an engineering concern? Could a sufficiently energy-inefficient system still qualify as intelligent by functional criteria? Discuss with reference to both the biological systems examined in this book and current AI architectures.

3. **Stigmergy and credit assignment.** In ant colony optimization, the "learning signal" is pheromone reinforcement — a distributed environmental record of past good solutions. In deep learning, the learning signal is an error gradient propagated backward through the network. Compare these two approaches to the problem of credit assignment: how does a system figure out which of its past actions contributed to a good outcome? Which approach is more biologically plausible, and does that matter?

4. **Subsumption architecture revisited.** Brooks's subsumption architecture was proposed in 1991 and never fully displaced classical planning-based AI. In fact, many successful autonomous systems — self-driving cars, for instance — use hybrid approaches with rich internal maps alongside reactive behaviors. Does this suggest that Brooks was wrong, that he was right, or that the dichotomy he set up was incomplete? How does the concept of a "cognitive hierarchy" from basal cognition research bear on this question?

5. **Neuromorphic computing and biological fidelity.** Current neuromorphic chips implement spiking neurons with local learning rules. But the biological synapse is far more complex than any current silicon implementation — it involves dozens of protein species, multiple forms of short- and long-term plasticity, metabolic coupling, and modulation by neurotransmitters and glial cells. How much biological detail is enough? Is there a principled way to determine which biological features are computationally necessary and which are implementation accidents?

---

## Part II: Thought Experiments

1. **The minimal intelligent agent.** Imagine you are designing an artificial agent that must perform chemotaxis — following a chemical gradient in a noisy environment — using the minimum possible computational resources. What is the minimal architecture? How does your answer compare to what *E. coli* actually does? Now extend: what is the minimum architecture required for associative learning? For maze-solving? At what point does the architecture begin to look like a brain?

2. **Stigmergy in human institutions.** Identify a human institution or practice that functions stigmergically — that is, in which coordination is achieved through the environment rather than through direct communication or central planning. (Markets, city street layouts, and open-source software are candidates, but find your own example.) Analyze the stigmergic dynamics: what serves as the "pheromone"? What is the positive feedback mechanism? What is the evaporation equivalent? What does this analysis reveal about the relationship between collective intelligence in biological and human systems?

3. **The neuromorphic endgame.** Suppose neuromorphic computing eventually produces a chip with the same number of neurons and synapses as the human brain, implementing biologically realistic spiking dynamics and local learning rules, running at biological timescales, consuming twenty watts. Would this chip be conscious? Would it deserve moral consideration? Would it be a person? Work through the philosophical implications carefully, drawing on the frameworks discussed in earlier chapters of this book.

---

## Part III: Laboratory and Computational Investigations

1. **Implement a simple ACO algorithm.** Using a programming language of your choice, implement a basic Ant System algorithm for the Traveling Salesman Problem on a set of 30 randomly generated cities. Vary the pheromone evaporation rate (α) and the relative influence of pheromone versus distance (β) and characterize how solution quality and convergence speed depend on these parameters. Compare your best solution to the nearest-neighbor greedy algorithm. Write a brief analysis of what the parameter sensitivity reveals about the design principles of stigmergic computation.

2. **Subsumption architecture simulation.** Using a robot simulation environment such as Webots or Gazebo (both free), implement a simple three-layer subsumption architecture for a wheeled robot: Layer 0 avoids collisions, Layer 1 wanders randomly when no obstacle is present, Layer 2 moves toward a detected light source. Demonstrate that the robot successfully navigates to the light source in a cluttered environment. Now add a fourth layer of your design. Document what behaviors you observe at each layer of the hierarchy, and how the layers interact.

3. **Energy efficiency comparison.** Using published benchmarks for a neuromorphic chip (Intel Loihi 2 or IBM TrueNorth) and a GPU (NVIDIA A100 or equivalent), compute the energy-per-inference for a comparable classification task (e.g., MNIST handwritten digit recognition or a comparable spike-based benchmark). Express the comparison in terms of joules per correct classification. Then estimate, from the literature, the energy cost of an equivalent classification task performed by the appropriate neural circuit in a mammalian brain. What does this three-way comparison reveal about the current state and future trajectory of neuromorphic computing?
