# Chapter 4 Exercises: The Cell as Problem-Solving Agent

---

## Part I: Reflection and Discussion

**1. The vocabulary of agency**
Throughout this chapter, we have used cognitive vocabulary — "sensing," "deciding," "remembering" — to describe cellular processes. A skeptic might argue that this is dangerously misleading anthropomorphism, while an enthusiast might argue that the mechanistic parallels justify the cognitive language fully. Stake out and defend your own position. What criteria should we use to determine when cognitive vocabulary is legitimately applied to non-neural systems?

**2. Bistability and identity**
A bistable switch creates a system with two distinct, mutually exclusive stable states. We have argued that hysteresis in such systems constitutes a form of memory. But consider: is a bistable switch's "memory" the same kind of thing as a synapse's memory in a neuron? What distinguishes them? Does the distinction matter for questions about whether the cell "knows" something about its past?

**3. Stochastic cell fate and individual vs. population-level cognition**
The bet-hedging hypothesis argues that stochastic fate decisions benefit the lineage even if they are not "optimal" for any individual cell. Does this force us to think about cellular cognition at the population level rather than the individual level? Is there such a thing as a "population-level decision" that is not reducible to the decisions of individual cells?

**4. The Bayesian cell**
The framing of cells as Bayesian integrators is theoretically productive but philosophically contested. What would it mean for a cell to have a "prior"? Can a prior that was set by evolution over millions of years rather than by learning within the organism's lifetime count as genuine prior knowledge? How does the evolutionary vs. individual timescale of prior-setting affect the legitimacy of the Bayesian framing?

**5. The sensor as filter**
Section 1 noted that different cells, despite sharing the same chemical environment, live in different "perceptual worlds" because they express different receptor complements. Does this mean that what a cell "knows" about its environment is fundamentally incomplete — filtered by its receptors? If so, does this incompleteness undermine or support the idea that cells are genuine information-processing agents?

---

## Part II: Thought Experiments

**Thought Experiment 1: The Minimal Cognizer**
Imagine you are designing a minimal "cognitive cell" from scratch. You want to create the simplest possible system that deserves to be called a problem-solver: it can detect a food gradient, move toward it, and adapt its sensitivity in response to experience. What is the minimum number of distinct molecular components you need? Start stripping components away from real *E. coli* chemotaxis — methyltransferases, methylesterases, CheY, the motor switch — and determine which are essential and which are redundant. At what point does removing a component turn the system from a "problem-solver" into a "mere mechanism"? Is there a principled answer, or is the distinction arbitrary?

**Thought Experiment 2: The Bistable Conspiracy**
Suppose a developing organism has a cell that sits at the bifurcation point of a bistable fate switch — perfectly balanced between two possible identities (say, neuron and glial cell). Environmental signals are noisy; either could happen. Now imagine you can intervene at three different levels: (a) you can add a small amount of a transcription factor that nudges the switch toward one fate; (b) you can increase the noise in the switch, making the outcome more random; (c) you can alter the topology of the switch so it is no longer bistable, forcing a graded, indeterminate state. Which intervention is most "cognitively" interesting? Which produces the most information about the cell's state? Is a cell in a perfectly indeterminate state between two fates more or less "informed" than one that has committed to either?

**Thought Experiment 3: The Tumbling Philosopher**
*E. coli* implements gradient navigation through a stochastic algorithm: it tumbles to randomly reorient, then runs, comparing current conditions to recent past. Consider a thought experiment in which we replace the stochastic tumble with a deterministic reorientation — the cell always turns exactly 90 degrees left during a tumble event. Now compare the navigational performance of the stochastic and deterministic variants in (a) a simple linear gradient, (b) a gradient with a local maximum in the center of an otherwise uniform field, and (c) an environment with two competing gradients in perpendicular directions. What does the relative performance of these strategies tell us about the value of randomness in cognition? Does this have implications for understanding why biological cognition so often incorporates stochastic elements?

---

## Part III: Laboratory Investigations

**Lab 1: Visualizing Chemotaxis in *E. coli* (wet lab or simulation)**
Using either living *E. coli* cells in a gradient chamber (soft agar plug method) or a computer simulation such as CellModeller or the Berg-Purcell simulation tools available online, observe chemotaxis in real time.

*Procedure*: In the soft agar method, stab a small amount of agar containing *E. coli* into the center of a minimal medium plate, and observe bacterial migration up to the edge of the agar plug over 6–12 hours. In simulation, set up a linear gradient of attractant and track individual cell trajectories.

*Analysis*: Calculate the average drift velocity up the gradient. Measure the run length distribution in the presence vs. absence of attractant. Estimate the "memory timescale" of the chemotaxis system from the autocorrelation of run length as a function of gradient magnitude.

*Discussion*: Compare your measured drift velocity to what would be expected from simple random diffusion without chemotaxis. How much does the biased random walk improve navigation efficiency?

**Lab 2: Modeling Signal Integration with Boolean Networks (computational)**
Using a freely available Boolean network simulation tool (e.g., BooleSim or the Cell Collective platform), construct a simple model of a eukaryotic signaling network with:
- Two inputs (growth factor A and growth factor B)
- An AND gate (both required for proliferation)
- An OR gate (either sufficient for survival)
- A negative feedback loop on input A

*Procedure*: Simulate the network under all four combinations of input states (A+B+, A+B-, A-B+, A-B-). Then introduce noise (random bit flipping at 5% probability) and rerun the simulations 100 times per input condition.

*Analysis*: Does the AND gate behavior degrade gracefully with noise, or is it brittle? How does the negative feedback loop change the dynamic behavior of the network? What is the "effective" truth table of the network under noisy conditions?

*Discussion*: Compare your Boolean model to the analog (graded) behavior of real kinase cascades. What information is lost by the Boolean abstraction? Under what conditions would Boolean models be sufficient for understanding cellular computation?

**Lab 3: Measuring Cooperativity in Receptor Systems (computational)**
Using a mathematical modeling tool (MATLAB, Python with SciPy, or even a spreadsheet), implement and analyze dose-response curves for receptor systems with different levels of cooperativity.

*Procedure*: Model a simple Hill equation response curve: y = x^n / (K^n + x^n), where n is the Hill coefficient (measure of cooperativity) and K is the EC50. Plot dose-response curves for n = 1, 2, 4, and 8 on both linear and log-log axes.

*Analysis*: For each value of n, calculate (a) the EC10 and EC90 (concentrations giving 10% and 90% of maximal response), (b) the ratio EC90/EC10 (the "response range"), and (c) the maximum slope of the dose-response curve (the "sensitivity peak").

*Discussion*: How does cooperativity affect the cell's ability to distinguish signal from noise? Is higher cooperativity always better? Consider a scenario where the cell needs to respond proportionally to stimulus magnitude (like a dimmer switch) versus a scenario where it needs a sharp threshold response (like an on/off switch). For each scenario, what Hill coefficient is optimal, and why?

---

*For further study, see the Further Reading list for Chapter 4.*
