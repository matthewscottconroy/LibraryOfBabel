# Chapter 27 Exercises: Autopoiesis and Enactivism

---

## Part I: Reflection and Discussion Questions

**1. Organization vs. Structure**
Maturana and Varela distinguish between a system's *organization* (abstract relational pattern) and its *structure* (concrete physical implementation). They argue that the identity of an autopoietic system is its organizational identity. Does this distinction hold up under pressure? Consider: if a cell's organization were gradually replaced by a silicon-based implementation that preserved all the relational properties — each component producing the next in the same network topology — would the result be the *same* system? What does your intuition tell you, and does it track a principled theoretical commitment or a pre-theoretical bias about biological substrates?

**2. Operational Closure and the Epistemology of Science**
If autopoietic systems are operationally closed — if they do not receive information from the environment but only respond to perturbations according to their own organizational logic — what does this imply about scientists studying those systems? Scientists are also autopoietic systems. Do Maturana and Varela's arguments apply to scientific cognition as well? If so, what are the implications for the objectivity of scientific knowledge? (Maturana himself drew radical constructivist conclusions from this; evaluate whether those conclusions follow from the premises.)

**3. The Sense-Making Criterion**
Enactivism proposes that sense-making — the active constitution of significance relative to the organism's autopoietic organization — is the minimal form of cognition. Is this criterion too permissive, too restrictive, or approximately right? Consider the following cases and determine which, if any, satisfy the criterion: (a) a virus; (b) a prion; (c) an autocatalytic RNA molecule capable of self-replication; (d) a computer program that monitors its own processing load and reduces it when it exceeds a threshold. What does your analysis reveal about what the criterion is really tracking?

**4. Valence and the Origin of Normativity**
The enactivist account holds that valence — the positive/negative character of significance for the organism — is constituted by the organism's autopoietic organization. Good things are things that promote autopoiesis; bad things are things that threaten it. Critics argue that this reduces normativity to mere stability: a crystal that resists dissolution is also "good" at maintaining itself, but we wouldn't say dissolution is *bad for* the crystal in any genuine sense. How should the enactivist respond? Is there a principled way to distinguish the normative significance of autopoietic disruption from the merely causal fact of a system being disturbed?

**5. The Two Claims of Enactivism**
The chapter distinguishes the empirical claim (cognition is embodied and embedded) from the phenomenological claim (lived experience is constituted by organism-environment coupling). Which claim do you find more defensible, and why? Is there a version of enactivism that accepts the empirical claim while remaining neutral on the phenomenological claim? What would be lost and gained by adopting such a position?

---

## Part II: Thought Experiments

**1. The Isolated Cell**
Imagine a cell whose autopoietic organization is fully intact, but which is placed in an environment so perfectly uniform and homogeneous that no perturbation ever reaches it — no chemical gradients, no mechanical forces, no temperature variation, no light. The cell continues its internal metabolic cycling in perfect isolation. Is this cell cognizing? Is it sense-making? The enactivist account requires both the organism and the environment as partners in a coupling — what happens to sense-making when the coupling is severed? Does the cell have a cognitive domain with no domain to enact?

**2. The Synchronized Colony**
Consider a bacterial colony in which quorum sensing has been experimentally manipulated so that all cells are perfectly synchronized: every cell receives the same signals, undergoes the same state changes at the same time, and produces the same behavioral outputs simultaneously. In what sense is this colony a single autopoietic system, and in what sense is it many? Does the degree of coupling determine whether we should treat it as one system or many? If the colony suddenly began behaving in ways that no individual cell could achieve alone — generating spatial patterns that maintained the colony's overall organization at the cost of individual cells — would you say the colony itself had become an autopoietic system?

**3. The Structural Coupling Time Machine**
You are studying a bacterium adapted to an ancient pre-oxygen atmosphere. You place this bacterium in a modern oxygen-rich environment for one generation. Nothing catastrophic happens immediately — the bacterium survives and divides. Now consider: in what sense has the bacterium's structure been coupled to the oxygen environment? In what sense has it not been coupled, given that its organization is adapted to an oxygen-free world? Over how many generations of successful coupling would the bacterium become genuinely adapted to — genuinely coupled with — the oxygen environment? This thought experiment probes the temporal dimension of structural coupling and asks how much history is needed to ground the claim that a system "knows" its environment.

---

## Part III: Laboratory and Computational Investigations

**1. Perturbation vs. Instruction in Chemotaxis**
*Rationale*: The autopoietic framework claims that environments perturb but do not instruct autopoietic systems. Design an experiment to test whether the same chemical stimulus produces different behavioral responses in *E. coli* cells at different points in their metabolic cycle (e.g., exponential growth vs. stationary phase). If operationally closed cellular dynamics determine response, the same perturbation should produce different outputs depending on the cell's current state. Use a microfluidic device to expose synchronized bacterial populations to identical chemical gradients while tracking individual cell trajectories. Analyze whether variation in response correlates with metabolic state indicators (e.g., GFP-tagged metabolic reporters). What would confirm or challenge the operational closure thesis?

**2. Simulating Autopoietic Dynamics**
*Rationale*: Varela, Maturana, and Uribe (1974) published a formal model of autopoiesis in a two-dimensional grid world. Implement an agent-based model that attempts to capture the minimal features of autopoietic organization: (a) a boundary-producing process, (b) internal production processes that depend on the boundary, and (c) boundary maintenance that depends on internal processes. Run your simulation and ask: under what parameter conditions does the system maintain its organization indefinitely vs. collapse? How does structural coupling with a simulated environment (introducing and removing resource "molecules") affect system longevity? Compare your results with Varela et al.'s original model.

**3. Sense-Making and Valence in *Physarum polycephalum***
*Rationale*: The slime mold *Physarum polycephalum* provides a tractable experimental system for investigating enactivist predictions. Set up a maze assay with multiple food sources differing in quality (e.g., glucose concentration). According to the enactivist framework, *Physarum* should not simply follow any chemical gradient but should enact a meaningful landscape structured around its metabolic needs. Record network formation over 24 hours and analyze whether: (a) the organism's response to identical gradients differs depending on its current metabolic state (fed vs. starved); (b) the organism "anticipates" future resource availability in ways that cannot be explained by purely local gradient-following. Attempt to design conditions under which the sense-making account and a pure gradient-following account make different predictions.

---

## Bibliography for Chapter 27

Jonas, H. (1966). *The Phenomenon of Life: Toward a Philosophical Biology*. Harper & Row.

Maturana, H.R., & Varela, F.J. (1980). *Autopoiesis and Cognition: The Realization of the Living*. D. Reidel.

Maturana, H.R., & Varela, F.J. (1987). *The Tree of Knowledge: The Biological Roots of Human Understanding*. Shambhala.

Thompson, E. (2007). *Mind in Life: Biology, Phenomenology, and the Sciences of Mind*. Harvard University Press.

Varela, F.J., Maturana, H.R., & Uribe, R. (1974). Autopoiesis: The organization of living systems, its characterization and a model. *Biosystems*, 5(4), 187–196.

Varela, F.J., Thompson, E., & Rosch, E. (1991). *The Embodied Mind: Cognitive Science and Human Experience*. MIT Press.
