# Section 7: What Nervous Systems Add — The Marginal Cognitive Contribution of Neural Hardware

## Introduction

We have now surveyed the simplest organisms with nervous systems — *C. elegans* with its 302-neuron connectome, *Hydra* with its radially distributed nerve net, planaria with their cephalic ganglia and mutable memories. In each case, we have found cognitive capacities that are real and impressive: chemotaxis, habituation, coordinated whole-body behavior, and something that functions like memory. And in each case, we have noted that the underlying principles — gradient-sensing, experience-dependent modification, distributed information processing — are continuous with what we have documented in non-neural organisms throughout this book.

This raises a question that this book has been building toward: what do nervous systems actually add? Given that bacteria chemotax, that plants integrate multiple environmental signals, that slime molds solve maze problems, that colonial organisms make collective decisions — what is the marginal cognitive contribution of a nervous system?

The question is not rhetorical. It has an answer, and the answer is illuminating precisely because it reveals what neural hardware is for.

---

## 7.1 Speed

The most obvious thing that nervous systems add is speed. Chemical signaling — diffusion of hormones, exudates, second messengers — operates on timescales of seconds to hours. Electrical signaling along myelinated axons in vertebrates operates at speeds approaching 100 meters per second. Action potentials in *C. elegans* unmyelinated neurons are slower, but still dramatically faster than the diffusion-based signals they replaced.

Why does speed matter? Because some adaptive problems require responses on timescales shorter than chemical diffusion can support. A bacterium fleeing a toxin gradient can afford to reorient over seconds; an insect evading a predator strike cannot. A plant integrating soil chemistry over minutes to days is well-served by slow chemical signaling; an animal integrating visual, auditory, and tactile information to make a decision in fractions of a second is not.

The speed of neural signaling thus defines the ecological domain in which neural cognition confers advantage: the domain of rapid, temporally demanding interactions with the environment. Predator evasion, prey pursuit, mate detection in a competitive context — these are the problems that neural hardware solves efficiently. They are not the only adaptive problems that biology faces, but they were clearly important enough to drive the evolution of nervous systems in multiple independent lineages.

It is worth noting that plants are not completely without electrical signaling: the calcium waves and action potential-like propagating signals described in Chapter 19 (Mousavi et al., 2013) can travel at speeds of a few centimeters per minute — far slower than neural conduction, but much faster than hormone diffusion. This is not an argument that plants have nervous systems; it is an argument that the evolutionary pressure toward faster signaling is not unique to animals, and that the neural solution is the most extreme instance of a more general trend.

---

## 7.2 Distance and Specificity

Nervous systems allow information to travel long distances with high specificity. In an animal, a sensory signal detected at the skin of the foot can be transmitted, with spatial specificity, to a motor neuron in the spinal cord and from there to the appropriate muscle group — all within milliseconds and without altering anything along the pathway. The information is addressed: it travels to a specific target, not diffusely through the tissue.

This addressability is the functional significance of the synapse. A synapse is a point-to-point connection: the axon terminal of one neuron connects to the dendrite or cell body of a specific target neuron. This targeted connectivity allows nervous systems to route information with precision — not "some stress signal is present somewhere in this body" but "nociceptive input from the right index finger is being processed in this specific cortical column."

Non-neural organisms achieve some degree of spatial specificity through targeted exocytosis and receptor localization: hormones are released at specific cellular locations and bind to receptors with high affinity. But the degree of specificity achievable through synaptic connectivity — thousands of specifically addressed connections per neuron — is many orders of magnitude greater. This difference in specificity is the enabling condition for the complex, modular information processing that characterizes advanced neural cognition.

---

## 7.3 Representation

Perhaps the most cognitively significant contribution of nervous systems is the capacity for internal representation — the encoding of information about the external world in patterns of neural activity that persist and can be manipulated independently of current sensory input.

What does this mean? In bacterial chemotaxis, the organism responds to the current concentration of attractant: the behavior is directly coupled to the present sensory state. There is a simple memory of the recent past (the methylation state of chemoreceptors encodes a running average of recent stimulation), but there is no representation of where the food source is, what it looks like, or what route to take to reach it. The bacterium is always in the present.

A nervous system with sufficient complexity can decouple behavior from the present sensory state. *C. elegans* shows a rudimentary version of this: its behavior in a given sensory environment depends on its recent history (whether it has found food recently, whether it has encountered specific odorants before). In *Drosophila*, the mushroom body stores associations between odors and their outcomes — a representation of past experience that alters future behavior in response to the same odor. In mammals, the hippocampus constructs and stores maps of spatial environments — representations of "where things are" that can be used to plan routes without current sensory access to the destination.

Representation enables planning, anticipation, and the use of past experience to guide behavior in novel situations. These capacities are not binary — they exist on a continuum — and even simple nervous systems exhibit rudimentary versions of them. But the capacity for rich, flexible, context-sensitive representation of the world is one of the most important cognitive advances that large, complex neural networks provide.

---

## 7.4 Continuity, Not Discontinuity

The crucial point, and the one this book has been building toward, is that none of these contributions — speed, distance, specificity, representation — are genuinely novel inventions. Each is an enhancement of a capacity that already existed in non-neural form.

**Speed**: Bacteria already use electrical signaling — membrane potential changes — to propagate information rapidly across their bodies. Plants use calcium waves and action potentials. Neural signaling is a faster, more efficient, more precise implementation of a general biological strategy.

**Distance and specificity**: Plants achieve long-distance information transfer through vascular signaling; fungi achieve it through hyphal networks; bacteria achieve it through quorum sensing. The synapse is a more precise and higher-fidelity implementation of targeted chemical signaling.

**Representation**: The methylation state of bacterial chemoreceptors is a form of representation — an internal state that encodes information about recent chemical history and influences future behavior. Epigenetic modifications in plant cells are a form of representation — an encoding of past stress history that alters future gene expression. The difference between these and neural representation is one of resolution, flexibility, and the timescales over which states can be maintained and manipulated.

The history of life is not a history of cognitive discontinuities — sudden jumps from no-cognition to cognition, or from simple cognition to complex cognition. It is a history of gradual elaboration, enhancement, and integration of cognitive capacities that were present, in primitive form, from the earliest living systems. Nervous systems are the most recent and most elaborate cognitive hardware that evolution has produced. But they were built on a foundation laid by four billion years of cellular cognition, and the principles that govern them — sensing, integrating, acting; representing, remembering, anticipating — are the same principles that govern the simplest living systems we have studied.

The brain did not arrive to create cognition. It arrived to elaborate it.

---

## References

Mousavi, S. A. R., Chauvin, A., Pascaud, F., Kellenberger, S., & Farmer, E. E. (2013). Glutamate receptor-like genes mediate leaf-to-leaf wound signalling. *Nature*, 500(7463), 422–426.

White, J. G., Southgate, E., Thomson, J. N., & Brenner, S. (1986). The structure of the nervous system of the nematode *Caenorhabditis elegans*. *Philosophical Transactions of the Royal Society B*, 314(1165), 1–340.
