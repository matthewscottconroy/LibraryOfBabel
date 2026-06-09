# Section 28.2: The Free Energy Principle and All Life

## From Neural to Biological

Andy Clark's formulation of predictive processing remains within the brain. It is a theory about how nervous systems work — about the computational and representational principles that implement perception, action, and learning in neural circuits. This is already ambitious, but it is recognizably a theory about a particular kind of biological system.

Karl Friston has gone much further. Over a series of papers beginning around 2005 and most explicitly developed in "Life as we know it" (Friston, 2013), he has argued that the free energy principle — the mathematical framework underlying predictive processing — is not a theory about brains but a theory about *any self-organizing system that maintains its own identity against a fluctuating environment*. Cells, organs, organisms, colonies, and ecosystems all, Friston argues, minimize free energy. They could not exist as the kinds of systems they are unless they did.

This is a claim of enormous scope, and it demands careful examination. Let us begin with the mathematics.

## Free Energy: The Mathematics

The concept of **free energy** in Friston's framework is borrowed from statistical physics and information theory, where it was developed as a measure of the "available work" in a system — but Friston uses it in a specific information-theoretic sense. The relevant quantity is the *variational free energy*, which is an upper bound on the *surprise* (or negative log evidence) of sensory signals given the organism's generative model.

To understand this, we need three concepts:

**Surprise** (in the information-theoretic sense): the negative logarithm of the probability of an observation given a model, −log p(o | m). An observation that the model considers very likely generates little surprise; an observation considered unlikely generates much surprise. In Friston's framework, an organism's persistence — its remaining the kind of organism it is — requires that it not be surprised too often. An organism that is perpetually surprised by its environment is one whose sensory signals are very different from what its "model" of a viable environment would predict, which means it is probably dying.

**The generative model**: The probabilistic model that the system has (in a functional sense — not necessarily consciously or explicitly) of how sensory signals are generated. For a brain, this is the hierarchical set of predictions about the world. For a bacterium, the "model" is implicit in the structure of the chemotaxis signaling network: the network encodes, in its molecular structure, a prediction about what chemical concentrations a healthy, viable *E. coli* should encounter.

**Variational free energy**: A computable quantity that upper-bounds surprise. Minimizing free energy ensures that surprise stays low, but free energy is more tractable to compute than surprise directly. In neural terms, free energy is approximately equal to prediction error — so minimizing free energy is approximately equivalent to minimizing prediction error. The framework is thereby connected to the predictive processing account.

The key claim is: **any system that maintains a stable identity across time must, by mathematical necessity, be minimizing free energy**. If it were not — if its sensory (internal) states were perpetually surprising relative to any stable model — it would not maintain a stable identity; it would dissolve or undergo phase transition. Self-maintaining systems are free energy minimizers as a consequence of what it means to self-maintain (Friston, 2010).

## Markov Blankets and the Boundaries of Self

The concept of a **Markov blanket** is central to Friston's extension of the framework to all biological systems. The term comes from statistical graphical models, where a node's Markov blanket is the set of nodes that, once conditioned on, renders that node statistically independent of all other nodes in the network.

Friston applies this concept to the boundaries of self-organizing systems. A biological entity — a cell, an organism — has what amounts to a statistical boundary: a set of states that mediates all interaction between the system's internal states and the external environment. For a cell, this is approximately the cell membrane and its embedded proteins. For an organism, it includes the skin, sensory surfaces, and motor effectors.

The states of the system can then be partitioned into:

- **External states**: states of the environment, outside the blanket
- **Internal states**: states inside the blanket (cytoplasm, neural activity, etc.)
- **Active states**: states that influence external states (motor outputs, secretions)
- **Sensory states**: states influenced by external states (receptor activations, sensory signals)

The Markov blanket consists of the active and sensory states together. Internal states are only indirectly connected to external states through the blanket.

Now here is the crucial move. Given this structure, Friston argues that the internal states of any system with a Markov blanket can be interpreted as implementing a generative model of the external states — not necessarily explicitly, not necessarily consciously, but in the sense that the internal states track and respond to external states in ways that are statistically equivalent to inference. The system's self-maintenance can be read as an attempt to keep its internal states in configurations that are consistent with a stable external environment — which is precisely what minimizing free energy achieves.

This means that bacterial chemotaxis, immune response, and developmental patterning are all, formally speaking, inference processes: processes by which internal states are updated to track external states in ways that maintain the system's integrity. The bacterium's signaling network "infers" (in a functional, non-intentional sense) the chemical state of its environment; the immune system "infers" the presence of pathogens; the developing embryo "infers" the proper body plan.

### A Note on "Inference" Without a Mind

The use of the word "inference" here requires care. Friston and colleagues are using it in a technical, mathematical sense: internal states that update in response to sensory states in ways that minimize free energy are, formally, performing variational Bayesian inference. This does not imply that the bacterium consciously reasons, or that it has anything like the explicit probabilistic computations that characterize Bayesian inference in machine learning.

The formal equivalence is the claim. Whether this formal equivalence licenses us to say that bacteria *literally* infer is a philosophical question that the mathematics alone cannot settle. Some commentators (including Friston himself at times) slide between the formal claim and stronger intentional claims. Graduate students should be careful to track which level of claim is being made at any point.

## The FEP in Bacteria and Unicellular Organisms

What does the free energy principle actually predict or explain when applied to unicellular organisms? This is where the framework becomes interesting for basal cognition research and also where its limitations become most apparent.

**Chemotaxis as active inference**: The bacterium's chemotaxis system can be analyzed in FEP terms. The bacterium has internal states (signaling network states) that are responsive to external states (chemical concentrations). The signaling network generates "predictions" about appropriate chemical environments in the sense that its set-point concentrations define what a "good" environment looks like. When actual concentrations deviate from these set points, prediction errors (implemented as changes in CheY phosphorylation) arise, and the system acts (by changing flagellar rotation) to minimize those errors by moving toward environments that match the predictions. This is active inference: minimizing prediction error through action rather than through internal model updating.

This redescription is formally valid, but does it tell us anything we did not already know from standard biochemical accounts of chemotaxis? The answer is mixed. The FEP redescription connects bacterial chemotaxis to a broad mathematical framework that applies across many other systems, enabling cross-system comparisons. It also motivates specific questions — for example, about how the bacterium "predicts" across different timescales, and about how its active states (flagellar rotation) and sensory states (receptor binding) jointly minimize free energy — that might not arise from a purely biochemical perspective.

**Adaptive immune response**: The adaptive immune system provides a particularly interesting case for FEP analysis. The system generates diverse antibody configurations (prior distributions over pathogen identities), encounters pathogens (sensory input), and updates through clonal selection (posterior inference), ultimately arriving at specific, high-affinity antibodies (posterior distribution). The memory cells that persist after infection can be interpreted as "learned priors" — predictions about what pathogens are likely to be encountered — that reduce future surprise. This analysis connects immunological learning to the same framework that describes neural learning, which is conceptually unifying even if empirically unsurprising.

**Cell division as free energy minimization**: More speculatively, Friston and colleagues have proposed that cell division itself can be understood in FEP terms. A cell that grows larger faces increasing internal complexity and the threat of rising entropy within its boundary. Division can be interpreted as a resolution of this problem: by dividing, the system maintains two daughter cells that each have lower internal free energy than the parent was developing. This is a creative application of the framework, but its empirical content is limited: it is not clear what this analysis predicts that standard cell biology does not already explain.

## The Unfalsifiability Objection

The sharpest philosophical critique of the free energy principle is the **unfalsifiability objection**: that the FEP is so broadly stated that it cannot, even in principle, be falsified, and therefore does not qualify as a scientific theory in the sense required by Karl Popper's demarcation criterion.

The objection has several forms. The most direct is mathematical: if the FEP follows necessarily from the existence of systems with Markov blankets, then it is not an empirical claim but a mathematical theorem. Mathematical theorems cannot be falsified; they can only be proved or disproved within a formal system. An empirical theory must be *contingently* true — it must be possible that the world is organized differently than the theory predicts.

If the FEP says "any system that maintains its identity minimizes free energy," and if "maintaining its identity" is partly defined by minimizing free energy, then the claim is circular and unfalsifiable. What would it mean for a living system to exist without minimizing free energy? If no such thing is possible by definition, then the FEP tells us nothing about the world that we did not already know.

Friston's response to this objection has evolved over the years. His most consistent position is that the FEP is not an empirical hypothesis about how biological systems work but a mathematical framework — a set of formalisms — within which specific models can be built and tested. The specific models are falsifiable; the framework itself is not, but frameworks are not supposed to be. We do not falsify quantum mechanics; we build and test specific quantum mechanical models. Similarly, the FEP provides a mathematical language within which specific theories of biological self-organization can be formulated and tested.

This response is philosophically sophisticated and largely correct, but it creates a different problem: if the FEP is a framework rather than a theory, then applying it to a new domain (bacteria, plants, immune cells) does not in itself generate empirical predictions. It only generates predictions when combined with specific models of the mechanisms involved, and those predictions are predictions of the specific model, not of the framework. The framework itself may be empty.

### The Philosopher's Assessment

Several philosophers of science have argued that Friston's framework, while mathematically sophisticated, conflates formal and empirical claims in ways that obscure rather than illuminate (Colombo & Series, 2012; Klein, 2018). The claim that all living systems minimize free energy might be:

(a) A mathematical theorem following from definitions — true but uninformative
(b) An empirical hypothesis about a mechanism — informative but requiring specific predictions
(c) A conceptual reframing — useful for generating new models but not itself a scientific claim

Friston's writings sometimes seem to intend (b) while the arguments he provides only establish (a). Sorting out which level is being claimed at any given point requires considerable philosophical vigilance.

This does not mean the framework is useless. It means we should use it carefully, treating it as a mathematical language for building models rather than as a scientific theory that can be directly confirmed or disconfirmed. The models built within the framework can be — and have been — empirically tested. It is those specific models that generate genuine scientific knowledge.

## What the FEP Explains, What It Leaves Unexplained

In the spirit of honest theoretical assessment:

**What the FEP explains well**: The mathematical relationships among perception, action, and learning in systems with generative models. The conceptual unity of perceptual inference and motor control. The role of precision-weighting in attention and its relation to uncertainty. The connection between long-term learning (prior update) and short-term inference (posterior update). The formal structure of homeostasis as prediction-error minimization.

**What the FEP leaves underexplained**: The specific mechanisms by which different biological systems instantiate free energy minimization. The question of why living systems exist in the first place (the framework explains their persistence given existence, not their emergence from non-living matter). The relationship between free energy minimization and phenomenal consciousness — the framework is consistent with both the presence and the absence of subjective experience. The hard problem of consciousness.

**What the FEP potentially obfuscates**: The genuine differences between sophisticated neural inference and basic homeostatic regulation. By placing bacterial chemotaxis and human perceptual learning in the same mathematical framework, the FEP risks flattening genuinely important distinctions between kinds of systems. A formal equivalence is not a substantive identity.

For basal cognition researchers, the most productive use of the FEP is as a set of mathematical tools for modeling the dynamics of organism-environment interaction — tools that are rigorous, well-developed, and connected to a rich tradition in theoretical neuroscience. The philosophical claims about what the framework ultimately shows about the nature of cognition should be held more loosely.

---

## References

Colombo, M., & Series, P. (2012). Bayes in the brain — on Bayesian modelling in neuroscience. *British Journal for the Philosophy of Science*, 63(3), 697–723.

Friston, K. (2010). The free-energy principle: A unified brain theory? *Nature Reviews Neuroscience*, 11(2), 127–138.

Friston, K. (2013). Life as we know it. *Journal of the Royal Society Interface*, 10(86), 20130475.

Friston, K., Kilner, J., & Harrison, L. (2006). A free energy principle for the brain. *Journal of Physiology-Paris*, 100(1–3), 70–87.

Klein, C. (2018). What do predictive coders want? *Synthese*, 195(6), 2541–2557.
