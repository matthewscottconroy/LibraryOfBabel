# Section 2: Phytohormone Networks — The Chemical Language of Plants

## Introduction

Plants communicate with themselves. The long-distance signals described in Section 1 — electrical action potentials, systemin, jasmonate — are part of a broader network of chemical communication that coordinates growth, development, defense, and responses to the environment across the plant body. The primary language of this communication is hormones: small molecules produced in one tissue and transported to others, where they alter gene expression and cellular behavior.

Animal hormones — insulin, cortisol, estrogen — are familiar. Plant hormones are less so, but they are equally important to their organisms' physiology, and the signaling networks they participate in are, if anything, more complex than those of typical animal hormone systems. This complexity reflects the plant's situation: unable to move, facing threats and opportunities that change over developmental timescales, needing to coordinate responses across a whole-body architecture without a nervous system to integrate them rapidly.

---

## 2.1 The Major Phytohormones

**Auxin (indole-3-acetic acid, IAA)** is the first plant hormone discovered and remains the most studied. It is produced primarily in young growing tissues (shoot apices, young leaves) and transported downward through the plant by a unique directional transport mechanism involving PIN-FORMED (PIN) protein carriers on the cell membrane. Auxin promotes cell elongation at low concentrations but can inhibit elongation at high concentrations — giving it a concentration-dependent biphasic response that underlies several important growth patterns.

Auxin transport is directional and self-organizing: the PIN proteins that determine the direction of auxin flow are themselves localized to cell membranes by auxin levels, creating a feedback loop in which auxin flow patterns are self-reinforcing (Wisniewska et al., 2006). This creates a situation where auxin distribution patterns — the channels and gradients that guide development — are emergent properties of the self-organizing transport network rather than being specified top-down by any cellular organizer. This is Turing-style self-organization applied to plant hormone transport, and it underlies many of the most spectacular patterns of plant development: phyllotaxis (the spiral arrangement of leaves), vein patterning in leaves, and the branching geometry of root systems.

**Cytokinin** is produced primarily in root tips and promotes cell division. It moves through the xylem from roots to shoots, where it antagonizes auxin and promotes branching. The ratio of auxin to cytokinin in a tissue is a key determinant of development: high auxin favors root formation, high cytokinin favors shoot formation — a gradient that is used practically in plant tissue culture to control the morphogenesis of plant callus.

**Ethylene** is a gas — a volatile plant hormone that diffuses through air as well as tissues. It is produced in response to mechanical stress, wounding, flooding, pathogen attack, and (crucially for agriculture) fruit ripening. Ethylene promotes fruit ripening, leaf abscission, and senescence. Remarkably, it can coordinate responses across individuals: a damaged fruit releasing ethylene can trigger ripening in neighboring fruits on the same plant or even on neighboring plants. This volatile signaling represents a different channel of communication — not vascular but atmospheric — that connects plant physiology to its local environment.

**Jasmonate (jasmonic acid and jasmonoyl-isoleucine)** is the primary hormone of the wound defense response, as described in Section 1. It is produced from membrane lipids at sites of damage or stress and activates a transcription factor module (COI1/JAZ) that upregulates defense gene expression. JA signaling also promotes reproduction (it is required for pollen development and fertility), closes stomata, and inhibits growth — reflecting a classic allocation tradeoff between growth and defense.

**Salicylate (salicylic acid)** is the primary hormone of pathogen defense. It accumulates in response to pathogen attack (particularly biotrophic pathogens — those that feed from living tissue) and activates a different defense program from JA, centered on pathogenesis-related (PR) proteins and the hypersensitive response. JA and SA signaling pathways are largely mutually antagonistic: conditions that activate JA tend to suppress SA and vice versa. This antagonism is one of the major regulatory logics of plant immunity: resources are allocated to one defense strategy or the other depending on the nature of the threat.

**Abscisic acid (ABA)** is the primary hormone of stress responses — particularly water stress (drought) and cold. ABA accumulates during drought and causes stomata to close (reducing water loss), triggers the expression of cold and drought tolerance genes, and promotes dormancy. It is a master regulator of the plant's response to adverse environmental conditions.

---

## 2.2 Cross-Talk: The Hormone Network as Signal Integrator

No phytohormone acts in isolation. Each hormone pathway interacts — positively or negatively — with multiple others, and the actual cellular response at any location in the plant reflects the integrated output of all these interactions. This cross-talk is not noise; it is part of the plant's information processing system.

Some examples of cross-talk that are well-established:

**JA-SA antagonism**: As noted above, the jasmonate and salicylate pathways mutually suppress each other. This has important functional consequences: plants that activate SA-based immunity become more susceptible to insect herbivores (which are controlled by JA-based defenses) and vice versa. Pathogens can exploit this antagonism: some biotrophic pathogens actively induce SA signaling in their plant hosts, suppressing JA-based defenses that would otherwise limit herbivore damage and thereby potentially manipulating the plant's defense allocation (Spoel & Dong, 2012).

**Auxin-ethylene interactions**: Auxin and ethylene interact at multiple levels. Auxin promotes ethylene biosynthesis in most tissues; ethylene inhibits auxin transport. This creates a feedback loop that helps regulate root growth in response to soil impedance (mechanical resistance): a root encountering an obstacle experiences increased auxin and therefore more ethylene production, which slows elongation of the blocked root while promoting lateral root emergence to circumvent the obstacle.

**ABA-growth hormone interactions**: ABA suppresses the growth-promoting effects of gibberellins and cytokinins. This creates a hormonal switch between growth mode (low ABA, active growth) and stress response mode (high ABA, growth inhibition, stress gene expression). The speed of this switch is fast — ABA can accumulate substantially within minutes of water deficit onset.

**Brassinosteroids and other hormones**: Brassinosteroids (a class of steroid hormones in plants) interact with auxin, gibberellins, and other hormones to regulate cell elongation and development. The signaling networks are complex enough that systems biology approaches — mathematical modeling of interacting signaling pathways — are increasingly necessary to understand them.

The plant hormone network can be understood as a high-dimensional state space: at any moment, the concentrations of the major hormones in a given tissue define a point in this space, and the cellular response (gene expression, growth, defense status) is a function of that point. The plant's regulatory network navigates this state space in response to environmental inputs, tracking a trajectory that (ideally) maximizes fitness given the current conditions.

This is not a metaphor. It is a framework that biologists actually use to model plant hormone signaling, and it is a framework that has productive connections to concepts from systems biology, control theory, and even cognitive science.

---

## 2.3 Auxin Transport as a Self-Organizing System

The self-organizing properties of auxin transport deserve extended discussion, because they illustrate a principle of general importance: that the distributions of morphogenetic signals in plant development are not specified by a genetic blueprint but emerge from the self-organizing dynamics of the transport network.

The key observation is that auxin transport direction is determined by the distribution of PIN carrier proteins, and PIN distribution is itself regulated by auxin. Specifically, PIN proteins tend to accumulate on the cell membrane face through which auxin has been flowing — creating a feedback in which the direction of auxin flow tends to be maintained and reinforced (Wisniewska et al., 2006). This creates a canalization effect: initially diffuse auxin flow gradually organizes into discrete channels (which become the veins of developing leaves), and these channels become self-reinforcing as PIN proteins concentrate along them.

The mechanism is a biological implementation of the Turing reaction-diffusion principle, producing spatial patterns from purely local interactions without any global template. The spiral phyllotactic patterns of plant leaves and flowers — the same patterns that appear in sunflower seeds and pinecone scales, mathematically related to the Fibonacci sequence — arise from this same self-organizing auxin transport dynamics, as the developing apex establishes a zone of high auxin concentration that moves around the apex tip, triggering leaf initiation at each position it occupies (Jonsson et al., 2006).

This is computation in the sense that is most relevant to understanding basal cognition: a physical system that, through the dynamics of its local interactions, generates global patterned outcomes that would require complex calculation to specify explicitly. The plant is not "computing" the Fibonacci sequence; it is implementing the physical dynamics from which Fibonacci patterning emerges. But the functional result — a precise, reproducible, adaptive spatial pattern — is something we would recognize as a cognitively complex achievement if it were produced by a brain.

---

## References

Jonsson, H., Heisler, M. G., Shapiro, B. E., Meyerowitz, E. M., & Mjolsness, E. (2006). An auxin-driven polarized transport model for phyllotaxis. *Proceedings of the National Academy of Sciences*, 103(5), 1633–1638.

Spoel, S. H., & Dong, X. (2012). How do plants achieve immunity? Defence without specialized immune cells. *Nature Reviews Immunology*, 12(2), 89–100.

Wisniewska, J., Xu, J., Seifertová, D., Brewer, P. B., Ruzicka, K., Blilou, I., ... & Friml, J. (2006). Polar PIN localization directs auxin flow in plants. *Science*, 312(5775), 883.
