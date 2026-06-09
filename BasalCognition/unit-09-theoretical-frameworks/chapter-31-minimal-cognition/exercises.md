# Chapter 31 Exercises: Minimal Cognition Theories

---

## Part I: Reflection and Discussion Questions

**1. Drawing the Line**
Lyon's biogenic argument claims that any principled line between cognition and non-cognition that excludes bacteria must also exclude many animals we routinely attribute cognition to. Evaluate this argument carefully. Is it a valid argument or does it commit a fallacy of false equivalence? Consider: even if the behavioral criteria for animal cognition apply to bacteria, there might be other criteria — perhaps phenomenological, perhaps neural — that bacteria fail and the relevant animals satisfy. Can you construct a criterion for cognition that (a) applies to all the animals we confidently take to be cognitive, (b) excludes bacteria, and (c) is not simply a restatement of "has a nervous system"?

**2. The MCO Conditions**
Barandiaran and Moreno propose three conditions for minimal cognition: self-determination, adaptive self-regulation, and sensorimotor coupling. Consider a simple artificial system: a Roomba vacuum cleaner equipped with bump sensors, dirt sensors, and a learning algorithm that modifies its search pattern based on where it finds dirt. Does the Roomba satisfy all three MCO conditions? If so, is the MCO program too permissive? If not, which condition does it fail, and does the failure reveal something important about what the condition is really tracking?

**3. Derived vs. Intrinsic Intentionality**
Searle argues that artifacts have only derived intentionality — they are about things only because human minds have assigned them meaning. But consider: what is the status of the intentionality of human neural states? Is my belief about Paris intrinsically about Paris, or is its intentional directedness derived from the evolutionary and developmental history that shaped my nervous system? If the latter, is there a principled distinction between derived and intrinsic intentionality, or is all intentionality derived in some sense? Work out the implications of your answer for the intentionality of bacterial signaling.

**4. Valence and the Is-Ought Problem**
The claim that valence — the positive/negative character of organism-world interactions — is grounded in autopoietic organization risks committing a version of the naturalistic fallacy: deriving an "ought" (what is good for the organism) from an "is" (what maintains autopoiesis). Just because the continuation of autopoiesis is a causal condition for the organism's existence does not seem to entail that it is *good* for the organism in any normative sense. Evaluate this objection. Is there a way to ground genuine normativity — real goodness and badness — in biological facts without committing the naturalistic fallacy? Or should we accept that the valence we attribute to minimal organisms is only a functional (descriptive) concept, not a genuinely normative one?

**5. Agency Without Consciousness**
The chapter argues that we can attribute genuine agency to bacteria without attributing phenomenal consciousness, because agency (goal-directed self-maintenance) and consciousness (phenomenal experience) can in principle come apart. But is this distinction sustainable? Consider the following argument: genuine agency requires that the system really *cares* about achieving its goals — not in a metaphorical sense, but in a sense that requires there to be something at stake for it. And something can only be at stake for a system if the system has experiences — if there is something it is like to be that system. Does genuine agency require phenomenal consciousness? If so, does functional intentionality collapse into the claim that bacteria merely simulate agency without having it?

---

## Part II: Thought Experiments

**1. The Cognition Gradient**
Consider the following sequence of systems, ordered from simpler to more complex: (a) a negative feedback loop maintaining temperature; (b) an autocatalytic chemical reaction maintaining its own rate; (c) a simple autopoietic chemical system (like a lipid vesicle that synthesizes its own components); (d) a minimal artificial cell with receptor molecules that change its behavior in response to chemical signals; (e) *E. coli*; (f) *Paramecium caudatum*; (g) a honeybee. Where on this gradient does cognition begin? For each transition in the sequence, identify what new property is acquired that might constitute the addition of cognition. Does your analysis suggest that cognition is a threshold property (present or absent), a graded property (present in degrees), or a cluster concept (composed of several properties that can be present in different combinations)?

**2. The Minimal Cognitive System**
Design the simplest possible artificial system that you would be willing to call cognitive, using the theoretical criteria discussed in this chapter. The system must: (a) have some form of boundary that separates it from its environment; (b) maintain that boundary through its own activity; (c) sense some features of its environment; (d) act on its environment in ways that tend to maintain its boundary. Specify the physical components of the system as concretely as possible. How does your minimal cognitive system compare to *E. coli*? What does your system lack that *E. coli* has, and does that additional capacity matter for cognition?

**3. The Evolutionary Origin of Meaning**
Suppose we could run the tape of life back to the very first autopoietic system — the first self-producing, self-maintaining molecular system. In what sense, if any, did things *mean* something to this system? Was glucose *good* for it in any sense beyond "glucose increased the probability of its persistence"? Did the persistence of the system *matter* to the system, or only to us observing it? Work through the difference between these interpretations carefully. Does meaning require subjectivity — something it is like to be the system? If so, do we have any reason to think the first autopoietic systems were subjective? If meaning does not require subjectivity, what *does* it require?

---

## Part III: Laboratory and Computational Investigations

**1. Testing the MCO Framework in *E. coli***
*Rationale*: Design an experiment to test each of the three MCO conditions in *E. coli* chemotaxis. (1) Self-determination: Create strains with genetically altered chemotaxis signaling networks that decouple receptor activity from flagellar rotation. Compare the behavior of these strains to wild type in an identical chemical environment. If the wild type's behavior is self-determined by its signaling network rather than specified by the environment, the altered strains should produce different behaviors in the same environment, even if their sensory inputs are identical. (2) Adaptive self-regulation: Measure chemotaxis performance across a wide range of background chemical concentrations. Wild type *E. coli* adapts to background concentration and maintains high sensitivity across orders of magnitude — this is the adaptive self-regulation condition. Strains with mutations in the adaptation system (CheB or CheR mutants) should show reduced performance at non-optimal concentrations. (3) Sensorimotor coupling: Use optogenetic control of flagellar motor rotation to decouple sensing from motor output. If the sensorimotor coupling is specifically integrated in wild type, randomly substituting motor outputs should produce deficits even when sensing remains intact.

**2. Functional Intentionality and Predictive Accuracy**
*Rationale*: The functional intentionality framework claims that attributing cognitive vocabulary to a system improves predictive accuracy. Test this claim computationally. Implement three models of *E. coli* chemotaxis behavior in a complex chemical landscape: (a) a purely mechanistic model that describes the molecular interactions of the chemotaxis signaling cascade without any cognitive vocabulary; (b) a functional intentionality model that attributes goals and beliefs to the bacterium and uses those attributions to predict behavior; and (c) a hybrid model that uses cognitive vocabulary at the behavioral level but mechanistic descriptions at the molecular level. Compare the predictive accuracy of all three models against experimental data on *E. coli* trajectories in complex microfluidic environments. Does the cognitive vocabulary improve predictive accuracy? If so, at what level of description is it most useful?

**3. Valence and Learning in *Caenorhabditis elegans***
*Rationale*: *C. elegans* is the simplest organism with a fully mapped nervous system (302 neurons) and exhibits learning and memory. Use established chemotaxis assays to measure the valence structure of *C. elegans*' chemical environment: which odors are approach-worthy and which are avoid-worthy, and how does training (pairing a neutral odor with starvation or feeding) change the valence of that odor? Compare valence learning in *C. elegans* to valence-like responses in *E. coli* (the analogy of sensory adaptation): is there a principled difference in the nature of the valence response, or only a difference in degree? Specifically, can you identify a feature of *C. elegans* valence learning (associative, hierarchical, context-sensitive) that is absent in bacterial adaptation and that might justify treating the two systems differently on the MCO or biogenic criteria?

---

## Bibliography for Chapter 31

Barandiaran, X.E., & Moreno, A. (2008). On what makes certain dynamical systems cognitive: A minimally cognitive organization (MCO) program. *Adaptive Behavior*, 16(5), 293–309.

Bickhard, M.H. (2009). The biological foundations of cognitive science. *New Ideas in Psychology*, 27(1), 75–84.

Brentano, F. (1995). *Psychology from an Empirical Standpoint*. Routledge. (Original work published 1874)

Dennett, D.C. (1987). *The Intentional Stance*. MIT Press.

Hutto, D.D., & Myin, E. (2013). *Radicalizing Enactivism: Basic Minds without Content*. MIT Press.

Lyon, P. (2006). The biogenic approach to cognition. *Cognitive Processing*, 7(1), 11–29.

Mitchell, A., Romano, G.H., Groisman, B., Yona, A., Dekel, E., Kupiec, M., ... & Pilpel, Y. (2009). Adaptive prediction of environmental changes by microorganisms. *Nature*, 460(7252), 220–224.

Searle, J.R. (1992). *The Rediscovery of the Mind*. MIT Press.

Thompson, E. (2007). *Mind in Life: Biology, Phenomenology, and the Sciences of Mind*. Harvard University Press.
