# Section 3: Implications for Computing and Artificial Intelligence

## Unconventional Computing Paradigms

The history of computing is, in large measure, a history of increasing abstraction away from physical reality. Early computing machines — mechanical calculators, analog computers, early transistor-based digital machines — performed computation through physical processes that had a direct, visible relationship to the operations being performed. Over decades, this physical reality was progressively abstracted away: silicon logic gates operate at timescales and spatial scales invisible to human perception, and the connection between the physical substrate and the computation it implements is something that only specialists can follow.

Unconventional computing is, in a sense, a return to physical computation — to the idea that the physics of a material substrate can be harnessed to perform computational operations that would be difficult or expensive in a conventional digital framework. Reaction-diffusion systems, mechanical metamaterials, fluidic logic circuits, DNA computing, quantum computing — all of these are "unconventional" in that they exploit specific physical properties of their substrates to perform computation that is natural to those substrates, rather than implementing Turing-machine operations in silicon.

Fungal computing, as Adamatzky and others envision it, falls in this category. The idea is not to replace silicon with mycelium for general-purpose computation, but to identify the classes of computational problem for which the physical properties of the mycelium make it a natural or advantageous substrate.

## What Fungi Might Offer

Several properties of the mycelial substrate are potentially advantageous for specific computing applications.

**Self-organization and adaptive routing**: The mycelium's ability to build, optimize, and adaptively reroute its network in response to changing conditions is a computational capability that is genuinely difficult to implement in conventional hardware. Routing algorithms in digital networks are implemented in software and require significant computational resources. The mycelium implements a class of routing algorithm "for free" in its physical growth dynamics. For applications where adaptive routing in response to unpredictable environmental changes is important — sensor networks in complex environments, for example — a biological substrate that implements routing physically might have real advantages.

**Distributed sensing and integration**: A mycelial network is simultaneously a sensor (responsive to chemical, mechanical, electrical, and light stimuli at every point), a transport medium, and a computing substrate. In a conventional computing system, sensing, communication, and computation are implemented by separate hardware components that must be connected and coordinated. The mycelium integrates all three in a single physical substrate. For applications where distributed sensing and local computation are needed — environmental monitoring, perhaps, or soft robotics — this integration may be advantageous.

**Self-repair and growth**: Electronic circuits do not repair themselves. If a component fails, the circuit fails, unless redundancy has been designed in — which is expensive. The mycelium, by contrast, can grow around damaged regions, reroute through alternative pathways, and even regenerate severed connections. This self-repair capability is directly relevant to applications where the computing substrate must operate in environments where damage is likely and repair by humans is impractical.

**Chemical computation**: The mycelium operates in a chemical world. It senses chemical gradients, produces chemical signals, and responds to chemical inputs with exquisite specificity. For applications that require interfacing with chemical environments — biosensing, bioremediation, chemical synthesis — a computing substrate that natively operates in the chemical domain may offer advantages that silicon, which requires extensive transduction to interface with chemistry, does not.

## Wetware: Biology as Computing Substrate

The broader category that contains fungal computing is sometimes called "wetware" — computing systems that use biological materials as their physical substrate. Wetware approaches range from relatively well-established technologies (biosensors, lab-on-a-chip devices, DNA computing) to highly speculative visions (neural organoids as computing devices, neuromorphic interfaces with biological neurons).

The appeal of wetware is not primarily that biology computes better than silicon in any general sense — it does not. The appeal is that biology is already adapted to operate in biological environments, to respond to biological signals, and to perform certain classes of operation (adaptive routing, chemical sensing, self-assembly, self-repair) that are expensive or impossible to implement in conventional hardware.

Fungal wetware is interesting because fungi are, relative to neurons, robust and tractable. Growing fungal cultures is relatively inexpensive, does not require the specialized conditions needed for animal cell culture, and can be scaled. The electrical properties of mycelium can be measured with relatively simple equipment. And fungi are not sentient (as far as we know) in ways that would raise ethical concerns about their use as computing substrates.

This last point deserves a moment's attention. As our understanding of biological intelligence deepens, the ethical question of which organisms can suffer, and what moral consideration they deserve, becomes more pressing. If the mycelium turns out to be cognitively richer than we currently believe, that enrichment of our understanding has implications for how we should treat fungi — including in computing applications. This is not an argument against fungal computing research; it is an argument for pursuing that research with appropriate ethical attention.

## What Fungal Computing Teaches AI Research

Even if fungal computing never becomes practically useful for building computing devices, it is relevant to artificial intelligence research in a different way: as a biological model of distributed, adaptive, embodied intelligence.

Much of the research in artificial intelligence over the past decade has been driven by large-scale neural network models — systems that bear a conceptual relationship to biological brains but are implemented on conventional hardware with traditional programming approaches. These systems are powerful but brittle in ways that brains are not: they fail on inputs that differ slightly from their training distribution, they do not adapt continuously to changing environments, and they cannot repair themselves.

Biological systems — including mycelium — are, in contrast, adaptive, robust, and self-repairing. They integrate sensing and computation. They operate with massive parallelism and local control, without any central processor. Understanding how these properties are implemented in biological systems might inspire AI architectures that are more adaptive and robust than current approaches.

The mycelium, in particular, offers a model of how a network can solve difficult optimization problems (network design, resource allocation, adaptive routing) through purely local rules without any global plan. The mathematical analysis of these problems — of which Physarum's network optimization is the most studied example — has directly inspired new algorithms for distributed optimization in computing and telecommunications (Tero et al., 2010). Whether the same insight can be extracted from true fungal mycelium, and whether it can be implemented in computing systems more efficiently than current approaches, remains an open question.

## The Philosophical Dimension

There is a philosophical dimension to fungal computing that is easy to miss if we focus only on the practical applications. The fungal mycelium, if it computes, is a system in which computation, sensing, and embodiment are inseparable. The mycelium does not first sense its environment, then process the sensory data, then produce a behavioral output. All of these happen simultaneously, in the same physical substrate, through the same physical processes. The network is the sensor, the processor, and the effector.

This stands in sharp contrast to the standard cognitive architecture implicit in much AI research and cognitive science: the architecture of a system that receives inputs, processes them, and produces outputs — a system in which sensing, computation, and action are distinct stages in a sequential process.

The embodied cognition tradition in cognitive science, associated with thinkers like Francisco Varela, Evan Thompson, and Eleanor Rosch (Varela et al., 1991), has argued that this standard architecture misses something essential about how biological cognition works. Real cognitive systems, on this view, do not represent the world and then act on those representations; they enact their world through continuous sensorimotor coupling with their environment. The mycelium is an extreme example of this enactive model: its "cognition" (to whatever degree it has any) is entirely constituted by its ongoing physical coupling with its environment.

Whether or not one accepts the full enactivist position, the mycelium makes it vivid in a way that no artificial system currently does. For AI researchers interested in moving beyond the input-process-output paradigm, the mycelium is a provocative existence proof: this is what a system that integrates sensing, computation, and action into a single physical substrate looks like. It has been doing it for hundreds of millions of years. We are only beginning to understand how.

---

## References

Tero, A., Takagi, S., Saigusa, T., Ito, K., Bebber, D. P., Fricker, M. D., Yumiki, K., Kobayashi, R., & Nakagaki, T. (2010). Rules for biologically inspired adaptive network design. *Science*, 327(5964), 439–442.

Varela, F. J., Thompson, E., & Rosch, E. (1991). *The Embodied Mind: Cognitive Science and Human Experience*. MIT Press.
