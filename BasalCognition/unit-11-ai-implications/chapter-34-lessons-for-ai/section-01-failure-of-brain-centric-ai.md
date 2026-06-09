# Section 1: The Failure of Brain-Centric AI

## The Cortical Assumption

The history of artificial intelligence is, in large part, a history of trying to build a cortex. The perceptron, proposed by Frank Rosenblatt in the 1950s, was explicitly inspired by the neuron. Multi-layer neural networks — the architectures that now power everything from image recognition to language modeling — are called neural networks because they were conceived as abstractions of neuronal computation. The key insight driving the deep learning revolution of the last two decades was that stacking many such layers, with billions of parameters, and training on vast datasets, produced systems capable of superhuman performance on a remarkable range of pattern recognition tasks (LeCun, Bengio, & Hinton, 2015).

These achievements are real. AlexNet's performance on ImageNet in 2012 was a genuine shock to the field. GPT-class language models can generate fluent, contextually appropriate text that passes many superficial tests for human-like understanding. AlphaFold solved a fifty-year-old challenge in structural biology (Jumper et al., 2021). If we restrict our attention to these benchmarks, the story of AI sounds triumphant.

But there is a deeper problem, one that surfaces when we ask what these systems are actually doing, and what they cannot do. The problem is not a technical limitation that will dissolve with more data and more compute. It is an architectural commitment — a choice, mostly tacit, to build intelligence as pattern matching over static corpora rather than as real-time sensorimotor engagement with a physical world. That commitment purchases certain capabilities at the cost of others, and the capabilities it forfeits are precisely the ones that characterize life.

## What Deep Learning Does Well — and What It Cannot Do

Let us be precise about the distinction. Deep learning systems excel at tasks that can be reduced to finding statistical regularities in large, curated datasets. Given enough labeled images, a convolutional neural network can classify objects more accurately than a human. Given enough text, a transformer can model the statistical structure of language with extraordinary fidelity. Given enough protein sequences and structures, a learned model can predict structure from sequence. These are impressive feats of compression and generalization.

What these systems cannot do — or do only in very limited, brittle ways — is the following:

**Real-time adaptation without retraining.** A large language model is trained once (or periodically fine-tuned) on a fixed dataset, then deployed. During deployment, it does not learn. If the world changes in ways not reflected in the training distribution, the model degrades. By contrast, even a bacterium adapts its behavior on timescales of seconds to minutes, adjusting its receptor sensitivity, gene expression, and motility in response to changing chemical gradients (Bray, 2009). This is not "training" in the machine learning sense — it is continuous online adaptation, achieved through molecular mechanisms that update the system's response properties without any external supervisor.

**Genuine sensorimotor coupling.** An LLM has no body. It does not act in the world and perceive the consequences of its actions. It processes tokens that represent descriptions of the world, but it has no causal loop connecting perception to action to perception. This is not a trivial limitation. There is substantial theoretical and empirical evidence that genuine intelligence — the kind that generalizes robustly, that can handle novel situations, that can learn from a handful of examples — requires this sensorimotor grounding (Pfeifer & Scheier, 1999; Dreyfus, 1972). A language model that has read ten thousand descriptions of swimming has learned something about the statistical co-occurrence of words. A fish that has never read anything about swimming is nonetheless an expert.

**Energy efficiency.** The human brain — the most elaborate neural system on Earth — consumes approximately twenty watts. GPT-4-class models require data centers with megawatt-scale power supplies for training, and non-trivial power for inference. A bacterial cell performs its complete cognitive repertoire — sensing, integration, decision-making, action — at a cost measured in a small fraction of an ATP per second. The slime mold *Physarum polycephalum* solves maze problems and approximates optimal transport networks at the cost of roughly ten microwatts per cubic centimeter of tissue (Nakagaki, Yamada, & Tóth, 2000). The energy gap between biological and silicon cognition is not a minor engineering problem. It reflects a fundamental difference in computational architecture.

**Robustness without explicit fault tolerance.** Biological systems operate in a world that is noisy, partially observable, and continuously changing. They have evolved mechanisms for maintaining adaptive behavior despite damaged sensors, missing limbs, and environmental insults that would halt any carefully engineered system. An ant colony continues to function when individual workers are removed. A mycelial network reroutes cytoplasmic flow around damaged sections. A bacterium adjusts its swimming behavior as individual flagellar motors fail. This robustness is a property of distributed, redundant, continuously adaptive systems — the opposite of the architectural monolith.

## The Embodiment Gap

Hubert Dreyfus argued, beginning in the 1960s and with increasing technical precision over the following decades, that symbolic AI would never capture human-level practical intelligence because practical intelligence is not primarily propositional — it is embodied skill, grounded in a history of sensorimotor engagement with the world (Dreyfus, 1972). His argument was ridiculed at the time and is now, in a qualified sense, vindicated.

The point is not that symbolic representations are useless — clearly they are not. The point is that the kind of generalized, flexible intelligence characteristic of living systems is grounded in a loop: the organism acts, perceives the result, updates its action tendencies, acts again. This loop is not optional; it is constitutive. Intelligence, on this view, is not a property of a brain or a network in isolation but of a brain-body-environment system in continuous interaction (Varela, Thompson, & Rosch, 1991).

Deep learning architectures have begun to address this through reinforcement learning and sim-to-real transfer — training agents in simulated environments where they can act and observe consequences. This is a step in the right direction, and it has produced impressive results in game environments and some physical robotics applications. But the gap between simulated experience and the full richness of physical embodiment remains enormous, and the brittleness of sim-to-real transfer in complex physical environments is a well-documented problem (Peng et al., 2018).

## What Slime Molds and Bacteria Teach Us

Here is what is philosophically striking about the organisms this book has examined. *Physarum polycephalum* solves maze problems without training, without a dataset, and without a supervisor. It does so by exploring the physical space with its body, using cytoplasmic flow dynamics as an analog computer that implements gradient descent on a physical objective function (Nakagaki, Yamada, & Tóth, 2000). The computation is in the physics of the organism-environment interaction, not in a separately stored model.

*Escherichia coli* performs chemotaxis — gradient following in three-dimensional chemical space — using a network of six chemoreceptors that implement a remarkable version of temporal differentiation. The bacterium effectively computes the derivative of the chemical signal over time, asking not "how good is the current environment?" but "is it getting better or worse?" This is real-time derivative computation, implemented by the methylation dynamics of receptor adaptation, using no more molecular machinery than fits comfortably in two micrometers of cytoplasm (Bray, 2009; Berg, 2004).

Ant colonies solve the traveling salesman problem — an NP-hard optimization problem — by distributed exploration and stigmergic reinforcement, achieving near-optimal solutions with no individual ant capable of representing the full problem. The solution is in the colony's collective behavior, not in any individual agent (Dorigo, Maniezzo, & Colorni, 1996).

What these systems share is not neurons or large weight matrices. They share a different computational philosophy: use the body as the computer, use the environment as external memory, use time and physics to do the work that silicon systems offload to stored parameters. Researchers working in the tradition of morphological computation have been articulating this philosophy for decades (Pfeifer, Lungarella, & Iida, 2007), but it has not yet penetrated mainstream AI practice.

## The Representation Problem

A more technically precise version of this critique targets the role of representation. Standard AI systems — including deep learning systems — are representationalist: they build internal models of the world that are then used to generate behavior. The quality of the representation determines the quality of the behavior. The assumption is that getting the representation right is the central problem of intelligence.

Rodney Brooks's famous paper, "Intelligence Without Representation" (1991), challenged this assumption directly. Brooks argued that the world is its own best model, and that the right approach to intelligent behavior is direct sensorimotor coupling rather than internal representation. His early robots — Genghis, Allen, Herbert — demonstrated that surprisingly sophisticated behavior could emerge from simple reactive rules with no world model at all.

The critique of representationalism has been developed further by enactivist cognitive scientists (Varela, Thompson, & Rosch, 1991) and by researchers in ecological psychology following J.J. Gibson's theory of affordances (Gibson, 1979). The empirical question — how much internal representation is necessary for what kinds of intelligent behavior — remains open and actively contested. But the last three decades of work in embodied robotics, collective intelligence, and basal cognition strongly suggest that we have overestimated the centrality of rich internal representation and underestimated the cognitive work done by bodies, environments, and time.

## Toward a More Biologically Informed AI

This chapter is not arguing that deep learning should be abandoned or that current AI systems are without value. It is arguing something more specific: that the exclusive focus on brain-inspired, cortex-like architectures — on neural networks, on learned representations, on centralized processing — has caused the field to neglect a vast design space that four billion years of evolution has explored in remarkable depth.

The organisms studied throughout this book constitute an existence proof that intelligence can be implemented in radically different substrates, through radically different mechanisms. Basal cognition is not primitive or deficient cognition; it is differently organized cognition, evolved for robustness, energy efficiency, real-time adaptation, and distributed operation. Understanding its mechanisms, not just its existence, is a scientific project that has barely begun — and one whose implications for AI may turn out to be as significant as the deep learning revolution itself.

The following sections explore three directions in which biological intelligence is already informing AI: the logic of swarm and stigmergic computation, the philosophy and practice of embodied robotics, and the emerging hardware of neuromorphic computing.

---

## References

Berg, H. C. (2004). *E. coli in Motion*. Springer.

Bray, D. (2009). *Wetware: A Computer in Every Living Cell*. Yale University Press.

Brooks, R. A. (1991). Intelligence without representation. *Artificial Intelligence*, 47(1–3), 139–159.

Dorigo, M., Maniezzo, V., & Colorni, A. (1996). Ant system: Optimization by a colony of cooperating agents. *IEEE Transactions on Systems, Man, and Cybernetics, Part B*, 26(1), 29–41.

Dreyfus, H. L. (1972). *What Computers Can't Do: A Critique of Artificial Reason*. Harper & Row.

Gibson, J. J. (1979). *The Ecological Approach to Visual Perception*. Houghton Mifflin.

Jumper, J., Evans, R., Pritzel, A., Green, T., Figurnov, M., Ronneberger, O., ... & Hassabis, D. (2021). Highly accurate protein structure prediction with AlphaFold. *Nature*, 596(7873), 583–589.

LeCun, Y., Bengio, Y., & Hinton, G. (2015). Deep learning. *Nature*, 521(7553), 436–444.

Nakagaki, T., Yamada, H., & Tóth, Á. (2000). Maze-solving by an amoeboid organism. *Nature*, 407(6803), 470.

Peng, X. B., Andrychowicz, M., Zaremba, W., & Abbeel, P. (2018). Sim-to-real transfer of robotic control with dynamics randomization. In *2018 IEEE International Conference on Robotics and Automation (ICRA)* (pp. 3803–3810). IEEE.

Pfeifer, R., Lungarella, M., & Iida, F. (2007). Self-organization, embodiment, and biologically inspired robotics. *Science*, 318(5853), 1088–1093.

Pfeifer, R., & Scheier, C. (1999). *Understanding Intelligence*. MIT Press.

Varela, F. J., Thompson, E., & Rosch, E. (1991). *The Embodied Mind: Cognitive Science and Human Experience*. MIT Press.
