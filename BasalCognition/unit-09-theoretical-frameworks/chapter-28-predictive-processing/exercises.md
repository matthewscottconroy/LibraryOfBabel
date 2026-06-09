# Chapter 28 Exercises: Predictive Processing and the Free Energy Principle

---

## Part I: Reflection and Discussion Questions

**1. Helmholtz and the Boundary of Inference**
Helmholtz proposed that perception is unconscious inference. This suggests that there is a meaningful distinction between the inferential process and the sensory data it operates on. But if the nervous system is operationally closed (as Maturana and Varela claimed in Chapter 27), can there be genuine "data" that arrives from the world to drive inference? Work through the tension between the predictive processing account (which requires that prediction errors carry genuine environmental information) and the autopoietic account (which holds that environments perturb but do not instruct). Are these frameworks compatible? If not, which do you find more compelling and why?

**2. Active Inference and Free Will**
The active inference framework reinterprets voluntary action as prediction-error minimization: I act by generating predictions of the sensory consequences of movement and allowing those predictions to be fulfilled. Critics have argued that this eliminates the notion of genuine agency — that an "agent" that merely minimizes prediction errors is not an agent in any philosophically meaningful sense, but an automaton. Evaluate this objection. Does the active inference account of action preserve what is important about agency, or does it explain it away? Consider whether your answer differs for human agents, mammalian agents, and bacterial agents.

**3. The Markov Blanket and Personal Identity**
Friston's use of Markov blankets to demarcate the boundaries of self has been criticized on the grounds that Markov blankets are not fixed biological structures but statistical artifacts that depend on the analysis chosen. A cell, an organ, and an organism all have Markov blankets at different levels of description. Does the existence of nested Markov blankets mean that there are nested "selves" — that your liver has a self as well as you? What does this imply for theories of personal identity? Is the concept doing real philosophical work here, or is Friston using a mathematical convenience to generate philosophical conclusions it cannot support?

**4. The Falsifiability Debate**
The section presents Friston's defense: the FEP is a mathematical framework, not an empirical hypothesis, so it cannot be falsified, but that's fine because frameworks aren't supposed to be. Evaluate this defense. Is there a meaningful distinction between a "framework" and a "theory" in science? If so, what is it? Can you think of other cases where a framework/theory distinction has been invoked — quantum mechanics, evolutionary theory, thermodynamics — and does Friston's position fit the same pattern? What would it take to give up the FEP as a framework, if empirical hypotheses built within it systematically failed?

**5. Precision, Attention, and Psychiatry**
The predictive processing framework proposes that psychiatric disorders may reflect dysregulation of precision-weighting: in anxiety, excessive precision is given to threatening stimuli; in psychosis, internal predictions may be given excessive precision relative to sensory evidence, producing hallucinations. Evaluate this account as an explanation of psychiatric phenomena. What does it explain that traditional accounts do not? What does it fail to explain? How would you design an experiment to test a specific precision-weighting hypothesis about a specific psychiatric symptom?

---

## Part II: Thought Experiments

**1. The Perfect Predictor**
Imagine an organism that has, through millions of years of structural coupling with its environment, become so precisely adapted that its generative model of the environment generates zero prediction error at every moment. Sensory input perfectly matches predictions; no errors propagate upward. Would this organism be maximally alive and maximally cognitive, or would it be a kind of zombie — an automaton executing predetermined responses without any genuine engagement with the world? Does the predictive processing framework imply that surprise is necessary for genuine cognition? If an organism never makes errors, does it learn anything?

**2. The Inside-Out Bacterium**
Suppose a geneticist engineers a bacterium whose chemotaxis system is inverted: it runs toward repellents and tumbles more near attractants. The bacterium's internal model now predicts that beneficial chemicals are those that trigger its "run" response — which are actually repellents. Does this bacterium have a mistaken model of its environment, or does it simply have a different Umwelt? From the FEP perspective, is this bacterium minimizing free energy relative to its (inverted) model? Does it matter for the FEP whether the model is "correct"? What does this thought experiment reveal about the relationship between the FEP and evolutionary fitness?

**3. The Philosophical Zombie Colony**
A philosophical zombie, in the classic thought experiment, is a system that is functionally identical to a conscious being but has no subjective experience. Suppose a colony of bacteria is formally equivalent — at the level of FEP analysis — to a brain implementing the same active inference computations. The colony minimizes free energy, updates its internal states in response to sensory states, and acts to fulfill its predictions. Does this colony have any form of experience? Does the FEP analysis give us any reason to attribute or deny experience to such a system? This thought experiment probes the relationship between the computational-functional claims of the FEP and claims about consciousness.

---

## Part III: Laboratory and Computational Investigations

**1. Precision Weighting in Bacterial Chemotaxis**
*Rationale*: The predictive processing framework proposes that the weight given to error signals (precision) is dynamically regulated. In neural systems, this corresponds to attention. In *E. coli* chemotaxis, the adaptation system (particularly the methylation of chemoreceptors by CheB and CheR) implements something analogous: it adjusts the sensitivity of the receptor response based on the current chemical background. Design an experiment to test whether *E. coli* receptor adaptation implements precision regulation consistent with the FEP framework. Specifically: does the adaptation system reduce sensitivity (lower precision) when the chemical environment is highly variable (noisy), and increase sensitivity (higher precision) when it is stable? Use microfluidics to impose different patterns of chemical fluctuation and measure receptor methylation states and behavioral response.

**2. A Minimal Free Energy Agent**
*Rationale*: Implement an agent-based simulation of a minimal organism operating according to the free energy principle. The agent should have: (a) internal states representing its "generative model" of a preferred environment, (b) sensory states updated by the environment, (c) active states that move the agent through the environment, and (d) a free energy minimization objective. Compare the behavior of this agent with a simpler gradient-following agent in environments of varying complexity (uniform gradient, noisy gradient, gradient with obstacles, dynamic gradient). In which conditions does the FEP agent outperform the gradient-follower? In which conditions do they perform identically? Does the FEP agent exhibit anything analogous to epistemic foraging?

**3. FEP Analysis of Slime Mold Decision-Making**
*Rationale*: Apply FEP concepts to the foraging behavior of *Physarum polycephalum*. The slime mold constructs networks that efficiently connect food sources, and its network structure changes as food sources are added or removed. Construct a formal model of *Physarum* foraging as active inference: define the organism's "generative model" as a prior over food-source distributions, the sensory states as local chemical concentrations at network nodes, and the active states as changes in tube diameter (which redirect flow). Fit this model to time-series data of network reorganization and compare its predictive accuracy to a purely mechanical model of flux optimization. Does the FEP model predict reorganization dynamics better than the mechanical model? What parameters would need to be adjusted to improve fit?

---

## Bibliography for Chapter 28

Clark, A. (2016). *Surfing Uncertainty: Prediction, Action, and the Embodied Mind*. Oxford University Press.

Colombo, M., & Series, P. (2012). Bayes in the brain — on Bayesian modelling in neuroscience. *British Journal for the Philosophy of Science*, 63(3), 697–723.

Friston, K. (2010). The free-energy principle: A unified brain theory? *Nature Reviews Neuroscience*, 11(2), 127–138.

Friston, K. (2013). Life as we know it. *Journal of the Royal Society Interface*, 10(86), 20130475.

Hohwy, J. (2013). *The Predictive Mind*. Oxford University Press.

Knill, D.C., & Pouget, A. (2004). The Bayesian brain: The role of uncertainty in neural coding and computation. *Trends in Neurosciences*, 27(12), 712–719.

Rao, R.P.N., & Ballard, D.H. (1999). Predictive coding in the visual cortex: A functional interpretation of some extra-classical receptive-field effects. *Nature Neuroscience*, 2(1), 79–87.
