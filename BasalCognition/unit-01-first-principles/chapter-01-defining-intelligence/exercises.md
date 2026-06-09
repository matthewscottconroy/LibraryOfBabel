# Chapter 1 Exercises: The Problem of Defining Intelligence

---

## Part I: Reflection and Discussion

*These questions are designed for seminar discussion, written response, or both. They do not have single correct answers — they are invitations to reason carefully.*

**1.** The section on psychometric intelligence describes Spearman's *g* factor as "reliably predictive of a wide range of real-world outcomes." At the same time, we noted that *g* is inherently anthropocentric — it is a measure of human cognitive abilities. Discuss the following tension: Can a measure that is useful for predicting outcomes within a species be meaningfully extended to cross-species comparisons? What would need to change about the concept of *g* to make it applicable to a bacterium? Is such an extension desirable, or does it simply change the subject?

**2.** The folk-psychological notion of intelligence involves "taking into account" information about the world. We noted that a hawk adjusting its flight path to track a rabbit seems to involve the hawk "taking into account" the rabbit's trajectory, while a boulder rolling downhill does not. But is this distinction as sharp as it seems? Describe two cases: one where the "taking into account" criterion clearly applies, and one where it is genuinely ambiguous. What features of the ambiguous case make it hard to decide?

**3.** Section 1.2 distinguishes between functional claims ("the bacterium senses aspartate") and phenomenological claims ("the bacterium experiences desire for aspartate"). Consider the following: Is there a level of complexity, richness, or integration of information processing at which functional cognition *implies* phenomenal experience? If so, where is that threshold, and what justifies drawing it there? If not, why not — what is the relationship between functional cognition and experience?

**4.** The biogenic approach defines cognition as "the set of processes by which a living system registers and responds to information relevant to its continued function and organization." Consider a self-replicating RNA molecule in a primordial soup. Does it satisfy this definition? If it does, does this seem like the right result? If it doesn't, what additional condition would you add, and how would you justify it biologically?

**5.** Wiener's cybernetics showed that goal-directed behavior can be fully described in mechanistic, non-teleological terms. But some philosophers (notably Elliott Sober and others in the philosophy of biology) argue that functional explanations in biology are *ineliminable* — that you cannot fully explain why the heart beats by describing its mechanical operation; you also need to say what it is *for*. Do you agree? And if functional explanation is ineliminable in biology, does this support the basal cognition project or complicate it?

---

## Part II: Thought Experiments

*These thought experiments are designed to probe the definitions and intuitions developed in this chapter. They are provocative by design — the goal is productive discomfort, not settled answers.*

**Thought Experiment 1: The Silicon Bacterium**

Imagine that we construct a nanoscale silicon device that is chemically and functionally identical to an E. coli bacterium: it detects chemical gradients using the same molecular logic, integrates signals using the same network architecture, and actuates flagellar-equivalent motors to navigate. It even has a methylation-equivalent adaptation system that implements short-term memory. But it is made entirely of silicon and manufactured chemicals, not biological molecules.

- Does the silicon bacterium cognize, on the SIA definition offered in this chapter?
- If it does, does it matter that it was designed rather than evolved? Why or why not?
- Now suppose we gradually replace biological components of a real E. coli with silicon equivalents, one at a time. At what point, if any, does the hybrid system stop cognizing? What changes with each replacement?

This thought experiment tests the substrate-neutrality claim of the functional definition. Consider carefully what the implications of your answer are.

**Thought Experiment 2: The Distributed Decider**

Consider a forest of Douglas firs connected by a mycorrhizal network (the "wood wide web" discussed in Chapter 19). Carbon, nutrients, and signaling molecules flow through this network between trees. There is evidence (contested, as you will see) that trees under stress can receive resources through the network from less-stressed neighbors.

Now consider the following question: Is the mycorrhizal network-plus-trees system a *single cognitive agent*? Does it sense (via the individual trees' sensory systems), integrate (across the network), and act (via the coordinated responses of multiple trees) in ways that serve the continued organization of the network as a whole?

- Apply the SIA definition strictly. What do you conclude?
- What is the relevant unit of analysis — the individual tree, the mycorrhizal fungus, or the network as a whole?
- Does the answer to this question have implications for how we think about the relationship between individual cognition and collective cognition in, say, the human brain?

**Thought Experiment 3: Cognition by Subtraction**

You are presented with an animal whose cognitive capacities you are gradually reducing by surgical and pharmacological intervention. You remove long-term memory first. Then short-term memory. Then the capacity for flexible behavioral selection (leaving only fixed-action patterns). Then the capacity for multi-modal sensory integration (leaving only single-channel responses to individual stimuli). Then the adaptation mechanism (leaving only immediate reflex responses with no habituation).

At what point in this sequence does the animal stop cognizing? Is there a sharp threshold, or does cognition diminish gradually? How does your answer relate to the SIA definition?

Now consider: is there a point in the subtraction sequence at which the resulting system becomes *more similar* to a bacterium? If so, what does this suggest about the relationship between bacterial cognition and reduced animal cognition?

---

## Part III: Laboratory Investigations

*These are empirical exercises. Some can be done with basic wet lab equipment; some require computer simulations; all require careful experimental design.*

**Lab 1: Comparing Goal-Directedness Across Systems**

Design a simple experiment to compare the goal-directedness of three systems: (a) a thermostat, (b) an E. coli culture in a chemotaxis assay, and (c) a Paramecium culture in a salt-gradient dish.

For each system:
- Identify the reference state(s)
- Identify the sensing mechanism(s)
- Identify the integration step (if any)
- Identify the effector mechanism(s)
- Identify what happens when the system is disturbed from its reference state
- Rate the system on the 0–6 scale from Section 1.3.5

Then introduce a novel disturbance that the system has not "encountered" before (e.g., a different chemical gradient for the bacterium; a temperature gradient combined with a salt gradient for the thermostat and Paramecium). Record and compare the responses.

*Materials*: Thermostats (commercial), E. coli strain RP437 (chemotaxis competent), Paramecium caudatum cultures, capillary tubes, microscope, basic chemicals for gradient creation.

*Analysis questions*:
- Does the presence of a novel disturbance reveal differences in the systems' "cognitive" capacities?
- How does the speed of response relate to the cognitive status of the system?
- How would you design a follow-up experiment to test for information integration (rather than just sensing and acting)?

**Lab 2: Mapping the Information Integration Architecture of a Simple Signal Network**

Using publicly available data from the E. coli regulatory network (EcoCyc database: https://ecocyc.org), map the chemotaxis signaling network and analyze its logical structure.

- Identify all inputs (attractants and repellents) and outputs (motor behavior)
- Identify the integration nodes: where does the network combine information from multiple inputs?
- Determine whether the network can implement logical operations (AND, OR, NOT) on its inputs. Give concrete examples.
- Identify any feedback loops: which of these implement memory?

*Analysis questions*:
- Does the chemotaxis network satisfy the integration criterion of the SIA definition? On what evidence?
- How does the network's information integration architecture compare to a simple neural reflex arc?
- Design a minimal modification to the network that would increase its integrative capacity. What would this require biologically?

**Lab 3: Testing the Biogenic Definition Against a Non-Biological System**

Obtain or construct a simple self-regulating physical system — a feedback-controlled water level, a thermostat circuit, or a simple electronic feedback oscillator.

Apply the SIA definition and Barandiaran & Moreno's minimally cognitive organization criteria to the system, step by step:
- Does it sense relevant environmental features?
- Does it integrate information (with a modulating layer, not just a direct reflex)?
- Does it act to maintain its organization?
- Is its "organization" something it itself constitutes (autopoietic) or something assigned by an external designer?

Then modify the system to add a second feedback loop (e.g., a temperature sensor that modulates the set point of a pressure controller). Repeat the analysis.

*Analysis questions*:
- At what point, if any, does the system tip from "non-cognitive" to "minimally cognitive" on the biogenic definition?
- What does the exercise reveal about the relationship between complexity and cognition?
- Does the distinction between a system's "self-constituted" goals and "externally assigned" goals hold up under scrutiny? How would you test whether a biological system's goals are really self-constituted?

---

## Chapter 1 Key Terms Review

Before proceeding to Chapter 2, make sure you can define and distinguish:
- Intelligence (psychometric sense vs. functional sense)
- Cognition (representationalist vs. biogenic definition)
- Mind (phenomenological sense)
- Goal-directedness (cybernetic sense)
- Homeostasis
- Sensing-integrating-acting triad
- Minimally cognitive organization
- Functional claim vs. phenomenological claim
- Intrinsic vs. derived intentionality
- Anthropomorphism vs. eliminativist overcaution
