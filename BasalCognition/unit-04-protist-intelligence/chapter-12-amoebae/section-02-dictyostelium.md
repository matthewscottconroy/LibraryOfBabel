# Section 2: Dictyostelium — Social Life Among the Amoebae

## Introduction

Dictyostelium discoideum spends most of its life as an unremarkable soil amoeba: solitary, predatory, dividing by binary fission, hunting bacteria through the leaf litter and organic debris of temperate forests. It is good at being a cell. But when food runs out — when the last bacterium has been consumed and the chemical signals of abundance have faded — something extraordinary happens.

Individual cells begin to release pulses of cyclic AMP, the same second messenger molecule that relays signals inside your own cells when adrenaline binds to its receptor. Other cells detect this cAMP, amplify it, and relay it outward in expanding waves. Across a lawn of half a million independent amoebae, these waves propagate like ripples in a pond, and the cells respond to each passing wave by moving a small increment toward its source. Within hours, tens of thousands of cells have aggregated into a coherent mass, and that mass has begun to behave — to move, to differentiate, to build structures — in ways that no individual cell could accomplish alone.

This lifecycle is one of the most intensively studied examples of self-organization in biology. It is also one of the most philosophically provocative: here is a creature that spends part of its life as a genuine individual and part of its life as a participant in something that looks, uncomfortably, like a multicellular organism. Understanding Dictyostelium means grappling with the question of what individuality is — and what cognition means when the boundary between individual and collective is itself negotiable.

---

## 2.1 The Lifecycle: From Cell to Slug to Fruiting Body

The Dictyostelium lifecycle is conventionally divided into three phases: the vegetative phase (solitary predation), the aggregation phase, and the multicellular phase (which includes the migrating slug and the final fruiting body).

During vegetative growth, cells behave as described in Section 1 — chemotaxing toward bacteria, engulfing them by phagocytosis, dividing when resources permit. This phase can last indefinitely as long as food is available.

Starvation triggers aggregation. Within the first few hours of food deprivation, cells begin to express adenylyl cyclase, the enzyme that synthesizes cAMP from ATP. A few cells — perhaps those that have been starving longest, or perhaps those in which stochastic fluctuations have triggered the cascade — begin releasing pulses of cAMP. Neighboring cells that detect this cAMP through the cAR1 receptor respond in two ways: they move chemotactically toward the source, and they produce their own cAMP pulse, amplifying and relaying the signal outward (Devreotes, 1994).

The result is a self-organizing signaling system of remarkable elegance. Concentric or spiral waves of cAMP propagate outward from aggregation centers at speeds of approximately 300 micrometers per minute. Cells respond to the rising phase of each wave by polarizing and moving toward the center, then adapt during the falling phase, ready for the next wave. After each wave passes, there is a refractory period during which cells cannot respond — this prevents back-propagation of the signal and ensures that waves travel only outward, maintaining directional information (Goldbeter, 1996).

This is not a trivial coordination problem. The cells are not following a pre-existing gradient — they are collectively generating one. The center of aggregation is not specified by any single cell; it emerges from competition between multiple incipient centers, with larger centers typically winning by drawing cells away from smaller ones. The spatial structure of the aggregate — the pattern of waves, the identity of the aggregation center — is a property of the collective, not of any individual.

---

## 2.2 cAMP Waves and the Information Architecture of Aggregation

The cAMP signaling system that coordinates Dictyostelium aggregation has been analyzed in mathematical detail that few biological signaling systems can match. The core dynamics are those of an excitable medium: a system that is stable at rest, capable of large transient excursions when perturbed past a threshold, and that returns to rest with a refractory period before it can be excited again.

This class of dynamics — shared by neurons, cardiac muscle cells, and reaction-diffusion chemical systems — produces wave propagation in a characteristic way. The wave front is the leading edge of excitation. Behind the wave front, cells are in the refractory state and cannot be re-excited. Only cells ahead of the wave front are in the excitable state and can receive and relay the signal. This guarantees that waves travel in one direction: away from the center, so that cells can use the gradient of wave arrival times to navigate inward.

The mathematical models developed to describe Dictyostelium aggregation (Goldbeter, 1996; Martiel & Goldbeter, 1987) reveal several important features:

The system exhibits **oscillatory instability**: under starvation conditions, the signaling network spontaneously generates periodic cAMP pulses. This is not noise — it is a genuine dynamical instability driven by the mutual activation between cAMP synthesis and cAMP-stimulated cAMP synthesis.

The system shows **frequency encoding**: as food deprivation deepens, the frequency of cAMP oscillations increases. Cells closer to starving oscillate faster, and cells detect not just the cAMP signal but its temporal pattern, allowing them to assess the state of their neighbors.

The system has **robust patterning**: even starting from random initial conditions with slight variations in cell density and signaling competence, the wave patterns that emerge are highly reproducible. The aggregation centers form, the waves organize, and the cells converge — reliably, repeatedly, across a wide range of environmental conditions.

---

## 2.3 Cell Sorting and Spatial Self-Organization

Once cells have aggregated into a mass, a new problem arises: not all cells are identical. Some will become the stalk cells that build the structure on which spores are elevated for dispersal. Others will become the spores themselves. This differentiation must be spatially organized — stalk cells need to be in the stalk, spore cells need to be in the spore head — and the amazing thing is that the cells figure this out through a process of sorting.

When the aggregate forms the migrating slug — an elongated structure roughly 1–2 mm long that moves toward light and heat — the cells at the front are pre-stalk cells and the cells at the back are pre-spore cells (Bonner, 2009). If you artificially mix them, scrambling the slug, they sort back into the correct spatial arrangement within a few hours. The sorting is driven by differential adhesion (cells of the same type adhere more strongly to each other than to cells of the other type) and by differential chemotactic responses (the two cell types respond differently to the same cAMP gradient).

What determines which fate a cell adopts? This is partly determined by the cell's history — cells in different phases of the cell cycle when starvation strikes tend to sort into different fates — and partly by signaling within the aggregate. There is evidence for a feedback system in which cells that have committed to the pre-stalk fate release signals that discourage neighboring cells from doing the same, maintaining a stable ratio of approximately 20% stalk to 80% spore cells (Strassmann & Queller, 2011).

This ratio-maintenance is biologically significant: a slug with the wrong ratio cannot form a functional fruiting body. The regulatory system that maintains the ratio in the face of variation in aggregate size and composition is an example of cellular-level computation — the collective is solving an optimization problem that serves the interests of spore dispersal.

---

## 2.4 Altruism, Cheating, and the Social Contract of Slime Molds

Here is the biological problem that makes Dictyostelium one of evolutionary biology's favorite case studies: stalk cells die. They sacrifice themselves — contributing structure, elevating the spore mass, but leaving no genetic descendants — so that spore cells can disperse. In evolutionary terms, this is altruism in the strict sense: a behavior that benefits recipients at a cost to the actor's fitness.

How can altruism like this evolve and be maintained? In most natural Dictyostelium aggregates, the cells that aggregate together are genetically identical (or nearly so) — they are the clonal descendants of a single founding cell (Strassmann et al., 2000). When all cells in the aggregate share the same genes, the stalk cells are, in a sense, not sacrificing their genes — they are sacrificing themselves to propagate copies of their genes in their clonal relatives. This is kin selection, Hamilton's rule, evolutionary altruism at the cellular level.

But the real world is messier. Studies by Joan Strassmann and David Queller and their colleagues have shown that in natural soil samples, multiple Dictyostelium genotypes often co-aggregate (Strassmann et al., 2000). When genetically distinct strains share an aggregate, the stage is set for cheating: a cell lineage that consistently becomes spore rather than stalk, benefiting from the altruism of others while contributing nothing to the shared infrastructure.

Laboratory experiments have confirmed that cheater strains can be isolated and evolved. These strains, when mixed with cooperators, preferentially sort into the spore mass, leaving cooperators to form the stalk (Dao et al., 2000). In head-to-head competition, cheaters outcompete cooperators within a mixed aggregate. But a population of all cheaters — unable to form functional stalks, forced to compete as pure vegetative cells without the benefits of multicellular development — does poorly compared to cooperator populations. This is the evolutionary prisoner's dilemma instantiated in a single-celled organism.

More remarkably, cooperator strains have evolved mechanisms to resist cheating. Some strains can recognize genetic strangers and segregate from them during aggregation — a form of kin discrimination at the cellular level (Strassmann & Queller, 2011). When a cooperator strain and a cheater strain are plated together, the cooperators may aggregate with each other, effectively excluding the cheaters from the collective benefit. The molecular basis of this discrimination involves cell-surface recognition proteins, including members of the TgrB1/TgrC1 family (Hirose et al., 2011).

What we have here, in a soil amoeba, is a recognizable social problem — the tragedy of the commons — and a recognizable social solution — kin discrimination and cheater suppression. These are the same structural problems that social theorists describe in human societies, in social insect colonies, and in the evolution of cooperation generally. The Dictyostelium system makes them visible at the cellular level, with the tools of genetics and molecular biology available for their analysis.

---

## 2.5 What Dictyostelium Teaches Us

The Dictyostelium system challenges us on several levels simultaneously.

At the level of individual cell cognition, it demonstrates gradient sensing and directed movement of the kind described in Section 1, refined and elaborated in the context of a coordinating collective signal. Cells must filter the directional information in a self-generated, propagating cAMP wave — a considerably more complex task than responding to a static gradient.

At the level of collective cognition, it demonstrates something stranger and harder to categorize. The aggregate behaves in ways that serve collective goals — efficient spore dispersal, maintenance of appropriate cell ratios, spatial organization — but there is no controlling cell. The "intelligence" of the slug is distributed across thousands of individuals, each following local rules, and the collective behavior emerges from their interactions.

At the level of evolutionary cognition, it demonstrates that the tension between individual and collective interests is ancient, and that molecular mechanisms for negotiating that tension — kin recognition, cheater suppression — can evolve in organisms without nervous systems.

John Bonner, who spent his career studying Dictyostelium and authored the definitive synthesis of its biology (Bonner, 2009), was careful to avoid overclaiming. He did not argue that slime molds were intelligent in any rich sense. But he was equally careful to insist on what the data showed: organisms with no individual neurons, coordinating thousands of individuals through signaling cascades, solving collective action problems, and building structured multicellular forms. Whatever vocabulary we ultimately prefer, the phenomena are real and they are remarkable.

---

## References

Bonner, J. T. (2009). *The Social Amoebae: The Biology of Cellular Slime Molds*. Princeton University Press.

Dao, D. N., Kessin, R. H., & Bhatt, H. G. (2000). Developmental cheating and the evolutionary biology of Dictyostelium and Myxococcus. *Microbiology*, 146(7), 1505–1512.

Devreotes, P. N. (1994). G protein-linked signaling pathways control the developmental program of Dictyostelium. *Neuron*, 12(2), 235–241.

Goldbeter, A. (1996). *Biochemical Oscillations and Cellular Rhythms: The Molecular Bases of Periodic and Chaotic Behaviour*. Cambridge University Press.

Hirose, S., Benabentos, R., Ho, H. I., Kuspa, A., & Shaulsky, G. (2011). Self-recognition in social amoebae is mediated by allelic pairs of tiger genes. *Science*, 333(6041), 467–470.

Martiel, J. L., & Goldbeter, A. (1987). A model based on receptor desensitization for cyclic AMP signaling in Dictyostelium cells. *Biophysical Journal*, 52(5), 807–828.

Strassmann, J. E., Zhu, Y., & Queller, D. C. (2000). Altruism and social cheating in the social amoeba Dictyostelium discoideum. *Nature*, 408(6815), 965–967.

Strassmann, J. E., & Queller, D. C. (2011). Evolution of cooperation and control of cheating in a social microbe. *Proceedings of the National Academy of Sciences*, 108(Suppl 2), 10855–10862.
