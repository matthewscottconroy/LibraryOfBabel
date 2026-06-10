# The Frame Problem

The frame problem, originally identified by John McCarthy and Patrick Hayes in 1969, is one of the central difficulties facing the classical computational approach to cognition. It began as a technical problem in AI and grew into a philosophical puzzle about the nature of cognition itself — about what it would take for a system to represent and reason about a changing world.

**The Original Formulation**

In classical AI planning systems, a robot or agent is represented as having a set of beliefs about the world. When the agent performs an action, the planning system must update its representation of the world to reflect the consequences of that action. The problem arises because almost everything does not change when an action is performed.

If a robot moves a block from one location to another, the positions of all other blocks remain unchanged. The color of the room remains unchanged. The robot's own name remains unchanged. In principle, the planning system must verify that each of these things has not changed after each action — and since there are indefinitely many facts that might be relevant, this verification is computationally intractable.

McCarthy and Hayes proposed "frame axioms" to solve this: for each action, axioms explicitly state what does not change. But this solution creates a new problem: there are potentially infinitely many frame axioms, and the system must represent and reason over all of them. The formal solution seems to make the problem worse by making its scale explicit.

**The Philosophical Frame Problem**

The technical frame problem prompted a deeper philosophical question: how do human cognizers manage to update their beliefs efficiently when the world changes? We do not laboriously verify that each fact about the world remains unchanged after each action. We do this effortlessly and rapidly, in a way that seems to require knowing in advance what kinds of things can be affected by what kinds of actions.

Dreyfus argued that this is possible for human beings because we are not representing the world as a collection of discrete facts in the first place. Human engagement with the world is primordially practical and holistic — we move through a world that is already organized by our interests, habits, and bodily capacities, not a neutral array of propositions. The frame problem arises only if you start with the assumption that world-knowledge is propositionally explicit; but that assumption may be wrong.

This connects the frame problem to embodied and phenomenological approaches to cognition. Merleau-Ponty's account of the body schema and motor intentionality suggests that skilled agents navigate the world without propositional representation of the kind that gives rise to frame problem worries. The expert chess player does not represent all possible board positions; she perceives configurations of pieces and responds to salient patterns with skilled habitual responses.

**Relevance and Inference**

The frame problem is one instance of a more general problem about relevance: how does a cognitive system determine which pieces of stored information are relevant to a given problem? In central cognition, as Fodor noted, any piece of information might in principle be relevant to any inference. Human beings seem to navigate this effortlessly, accessing relevant information quickly while ignoring the vast majority of what they know.

This is computationally mysterious. Classical symbolic systems have no principled way to navigate the vast space of potentially relevant information without either missing important connections or becoming overwhelmed by irrelevant ones. Human beings seem to have an intuitive sense of relevance — what Dreyfus called "being-in-the-world" in Heideggerian terms — that is not reproduced by formal systems.

Connectionist systems partially address this through distributed representations and learned associations: relevant information tends to be activated together, through the statistical structure of learned associations, without explicit search. But whether this constitutes a genuine solution to the frame problem or merely a practical workaround remains contested.

**The Frame Problem and Common-Sense Reasoning**

The frame problem is closely related to the challenge of common-sense reasoning — reasoning that humans find effortless but AI systems find extraordinarily difficult. Doug Lenat's CYC project was a decades-long attempt to encode human common-sense knowledge in a formal system large enough to support general intelligent reasoning. The difficulty of the project illustrates the frame problem: common-sense reasoning depends on knowing an enormous amount about what remains stable in a changing world, and this knowledge resists explicit formalization.

Recent large language models have achieved impressive performance on common-sense reasoning benchmarks, and it is an open question whether they have "solved" the frame problem in a practical sense or whether their performance masks systematic failures of the kind the frame problem predicts.

**The Frame Problem as a Window on Cognition**

Whether or not it has been solved, the frame problem has been philosophically productive. It revealed that the apparently simple capacity to reason about a changing world presupposes a sophisticated background of knowledge and inference that classical AI radically underestimated. It focused attention on the role of context, relevance, and implicit knowledge in cognition — themes that have become central to cognitive science, and that connect computational approaches to phenomenological and embodied alternatives.
