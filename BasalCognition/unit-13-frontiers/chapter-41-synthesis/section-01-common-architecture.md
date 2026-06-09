# Section 1: Common Architecture

## The Invariant Structure

When we survey the cognitive behaviors documented throughout this book — bacterial chemotaxis, slime mold maze-solving, plant electrical signaling, quorum sensing in biofilms, mycorrhizal resource distribution, collective decision-making in ant colonies — a structural pattern emerges that is consistent across all of them. Every cognitive behavior, at whatever level of biological organization, involves three functional components: sensing (detecting the state of the environment), integration (combining detected information with internal state to produce a representation of the current situation), and action (generating an adaptive behavioral response on the basis of the integrated representation).

This sensing-integration-action architecture is not merely a convenient description; it is a structural necessity. A system that senses but cannot integrate cannot adapt its responses to history or context. A system that integrates but cannot act has no effect on its situation. A system that acts without sensing is not responding to its environment but merely producing outputs. The cognitive value of the whole requires all three components.

What varies across biological systems is not the presence of these three components but their implementation — the molecular and physical mechanisms used to sense, integrate, and act — and their sophistication — the range of stimuli sensed, the depth and complexity of integration, the variety and flexibility of actions available.

## Sensing: Universal Specificity

Every biological system that has been examined in this book possesses sensory apparatus: molecular receptors in bacteria that bind specific chemical ligands; photoreceptors in unicellular algae and plants that detect light wavelength and direction; mechanosensitive channels in diverse cell types that detect physical force; electrical sensors in plant and fungal tissues that detect voltage gradients; olfactory and gustatory receptors in animal nervous systems that sample the chemical environment.

What is universal about sensing is not the specific molecules involved — these vary enormously across the tree of life — but the functional property of selectivity. Every sensing apparatus discriminates: it responds to some aspects of the environment and is indifferent to others. This discrimination embodies "prior knowledge" about what matters — about which environmental features are relevant to the organism's survival and reproduction. The selective sensitivity of a receptor is the evolutionary encoding of what the organism has, over its evolutionary history, found worth knowing about.

This selectivity is itself a form of cognition: the organism has already "decided," at the evolutionary level, what to pay attention to. Individual sensing events are the actualization of that evolutionary decision — the moment when an abstract evolutionary judgment about relevance is applied to the concrete current situation.

## Integration: The Core Computation

Integration is where the genuinely interesting cognitive work happens. A system that simply passes sensory signals through to motor outputs, without any transformation, is reactive but not cognitive. Integration means that incoming sensory information is combined with information about the organism's current state, its recent history, and its goals (or their functional equivalents) to produce a representation of the situation that is richer than any single input alone could provide.

In bacteria, integration occurs through the signaling cascade that connects receptor states to motor output: the methylation state of chemoreceptors encodes a "memory" of recent chemical concentrations; the phosphorylation state of CheY represents the integrated current signal; the flagellar motor response depends on both of these. The integration is implemented by the kinetics of the phosphorylation and methylation reactions, which operate on different timescales and effectively compute the difference between the current signal and the signal expected on the basis of recent history (Bray, 2009).

In slime molds, integration occurs through the dynamics of the cytoplasmic network: flows in the network carry chemical signals about resource quality from different parts of the network to a common medium (the cytoplasm), where their effects on tube diameter reinforcement are summed. The network's geometry at any moment represents the integrated history of resource encounters across its spatial extent (Nakagaki, Yamada, & Tóth, 2000).

In nervous systems, integration occurs through the connectivity of neural circuits: neurons receive inputs from many sources, combine them through the nonlinear summation of excitatory and inhibitory post-synaptic potentials, and generate outputs when the integrated input exceeds a threshold. The connectivity of the circuit determines how different inputs are weighted and combined, implementing the circuit's "opinion" about which inputs are most relevant to the decision being made.

Despite the enormous differences in mechanism, the computational principle is the same: incoming signals are combined with internal state to produce an output that is more informative than either alone. Integration is the computationally essential step in cognition, and it is universally present in cognitive systems.

## Memory Across Scales

Memory — the persistence of information about past states in ways that influence future behavior — is a universal feature of biological cognition, operating across an enormous range of timescales and through an enormous range of mechanisms.

At the molecular scale, memory is implemented by covalent modifications: the methylation of chemoreceptors in bacteria (persisting for seconds to minutes), the phosphorylation of kinases in eukaryotic signal transduction (persisting for minutes to hours), the acetylation of histones in epigenetic regulation (persisting for hours to days), and the methylation of DNA bases in epigenetic inheritance (persisting for the lifetime of a cell and potentially inherited across generations). Each of these molecular modifications is a stored bit of information about past states, available to influence future responses.

At the cellular scale, memory is implemented by stable states: the bistability of gene regulatory networks (Chapter 35) allows cells to commit to one of two stable expression states and maintain that commitment even after the signal that caused the transition has disappeared. Cancer cells, immune memory cells, and differentiated cell types all maintain their identity through bistable regulatory states that were established by transient signals.

At the organismal scale, memory in neural systems is implemented by synaptic weight changes — the long-term potentiation and depression of synaptic connections that encode experience in the connectivity of the brain. At the colony scale, memory is implemented by stigmergic marks — pheromone trails in ant colonies, cytoplasmic streaming patterns in fungal networks — that persist in the environment and influence future agent behavior.

What is universal is not the mechanism but the function: information about the past is retained in some physical form and used to modulate future responses. The diversity of mechanisms reflects the diversity of timescales and substrates across which cognitive systems operate; the universality of the function reflects the universal value of learning from experience.

## Anticipation as Universal Feature

Anticipation — the use of current information to prepare for future states — is present in all cognitive systems examined in this book. The slime mold reduces its locomotive speed in anticipation of a regular temperature drop (Saigusa et al., 2008). The bacterium computes the temporal derivative of chemical concentration, effectively asking whether the environment will be better or worse in the near future rather than just what it is now. The plant opens its stomata in the morning before the light intensifies, in response to the circadian clock that anticipates the daily light cycle. The honeybee scout communicates the location of a food source that the foragers are not currently near, enabling anticipatory flight planning.

Anticipation is a more cognitively demanding property than reactive response: it requires that the organism maintain an internal model of how the world is likely to change — a model that can be used to generate appropriate behavior before the anticipated change arrives. The models used by different organisms vary in their sophistication and their timescale: the slime mold's anticipation of periodic stimuli operates on the scale of hours; the bacterium's temporal differentiation of chemical gradients operates on the scale of seconds; the circadian clock's anticipation of the light-dark cycle operates on the scale of a day.

What is common to all of these is the functional relationship: current internal state represents future external state, and motor output is modulated accordingly. This is, in miniature, what prediction is. Basal organisms are not planning in the way that a chess player plans; they are not modeling counterfactual scenarios or reasoning about consequences. But they are doing something that is structurally homologous to prediction: they are using present information to prepare for a future that has not yet arrived.

## Valence: The Evolutionary Origin of Good and Bad

Every adaptive system must be able to distinguish states that are beneficial from states that are harmful — it must have some way of representing the difference between good and bad, between what to seek and what to avoid. This functional property is what philosophers call valence: the positive or negative character of a state, its quality of being something to seek or avoid.

In organisms with nervous systems, valence is associated with phenomenal affect: pain is not just a signal that something is wrong, but an experience that is aversive in a phenomenally specific way. Pleasure is not just a signal that something is beneficial, but an experience that is desirable in a phenomenally specific way. This phenomenal character is what motivates behavior directly, without the need for calculation.

In non-neural organisms, the functional analogue of valence is approach-avoidance: the differentiation of the motor output into movement toward or away from a stimulus. Bacteria move toward glucose and away from copper sulfate. Slime molds grow toward yeast and retract from light. Plants open their stomata to carbon dioxide and close them against pathogens. This is functional valence without (presumably) phenomenal valence — the distinction between beneficial and harmful is encoded in motor behavior, not in subjective experience.

The evolutionary origin of valence — of the capacity to distinguish beneficial from harmful states and to bias behavior accordingly — may be traced to the very origins of life. Any self-replicating system faces an energetic imperative: certain environmental conditions favor replication, others prevent it. The capacity to distinguish these conditions and to move toward the favorable ones was selected from the beginning. Every organism alive today is the descendant of an unbroken line of ancestors that successfully made this distinction. Valence, in its functional sense, is as ancient as life itself.

Whether phenomenal valence — the felt quality of good and bad — is equally ancient, or whether it emerged with neural systems, or somewhere between, is one of the great open questions that basal cognition research forces us to take seriously.

---

## References

Bray, D. (2009). *Wetware: A Computer in Every Living Cell*. Yale University Press.

Nakagaki, T., Yamada, H., & Tóth, Á. (2000). Maze-solving by an amoeboid organism. *Nature*, 407(6803), 470.

Saigusa, T., Tero, A., Nakagaki, T., & Kuramoto, Y. (2008). Amoebae anticipate periodic events. *Physical Review Letters*, 100(1), 018101.
