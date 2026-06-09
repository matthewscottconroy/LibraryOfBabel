# Section 2: Vascular Signaling — The Plant's Internal Communication System

## Introduction

When an herbivore bites into a leaf, information about that event must travel. Not just to the damaged tissue, which can defend itself locally, but to undamaged leaves, to roots, to tissues that have not yet been touched but may be next. The plant's response to damage is systemic: distant organs prepare themselves based on information that originates at the wound site and travels, somehow, through the plant body. How?

The answer involves a cast of molecular signals and the vascular system that carries them — a communication infrastructure that, while nothing like a nervous system in its hardware, achieves something recognizable as long-distance information transfer. Understanding this system is essential for understanding plant communication, because most of what plants "know" about themselves they learn through vascular signals. The wound response is the clearest example, but the same infrastructure mediates responses to pathogens, to drought, to changes in light, and to the plant's own developmental state.

---

## 2.1 Systemin: The First Identified Long-Distance Wound Signal

The story of plant vascular wound signaling began in earnest with Clarence Ryan's laboratory at Washington State University, working with tomato plants in the 1970s and 1980s. Ryan was investigating how tomato plants induce proteinase inhibitor proteins in response to herbivore attack — proteins that interfere with the digestive enzymes of insects and make the plant less nutritious. He found that when one leaf was damaged, proteinase inhibitor proteins accumulated not just in the damaged leaf but throughout the plant — in undamaged leaves, stems, even roots. A local wound was producing a systemic response.

Something was traveling through the plant to signal the undamaged tissues. Ryan's group spent years trying to identify that something. In 1991, they succeeded: they isolated a small peptide, 18 amino acids long, that could induce proteinase inhibitor production when applied to plant tissue at very low concentrations. They named it systemin — from the word "systemic" — and it was the first plant peptide hormone identified as a long-distance wound signal (Pearce et al., 1991).

Systemin is produced by proteolytic cleavage of a larger precursor protein, prosystemin, in response to wounding. It enters the phloem — the plant's nutrient transport tissue — and travels through the vascular system to distant tissues. In those tissues, it binds to a receptor and activates the jasmonate signaling pathway, ultimately inducing the expression of defensive proteins including proteinase inhibitors, lectins, and other compounds that deter herbivores.

The systemin story is remarkable for what it reveals about the minimum requirements for long-distance biological signaling: a small, diffusible molecule with specific receptor binding; a vascular transport system; and a conserved signal transduction cascade at the receiving end. The same elements — signal, carrier, receptor, response pathway — that appear in neural signaling are present here, implemented in completely different molecular hardware.

Systemin appears to be specific to the Solanaceae (tomatoes, potatoes, and related species); the universal wound signal hormone in other plant families is jasmonic acid and its derivatives (jasmonates), which serve some of the same functions as systemin by directly activating JA signaling pathways.

---

## 2.2 Jasmonates and Salicylates: The Two Languages of Systemic Defense

Plants conduct two distinct systemic defense conversations, using different molecular languages for different threats: jasmonates for herbivores and wounding, salicylates for pathogens.

**The jasmonate pathway** responds to mechanical damage and herbivore attack. When cells are ruptured, membrane lipids are released and converted, by a sequence of enzymatic reactions, first to linolenic acid and then to jasmonic acid (JA). JA is itself active as a signal, but it is also methylated to produce volatile methyl jasmonate (MeJA) — the compound that can leave the plant entirely and travel through the air to warn neighbors, as we discussed in Section 1. Within the plant, JA travels through the phloem and activates the jasmonate signaling cascade in distant tissues: repressor proteins called JAZ are degraded, allowing transcription factors to activate defensive genes. The result is systemic upregulation of proteinase inhibitors, secondary metabolites, and other defense compounds in tissues that have not been directly attacked.

**The salicylate pathway** responds primarily to pathogen attack. Infection by bacteria, fungi, or viruses triggers the accumulation of salicylic acid (SA) at the infection site, which then travels systemically through the phloem and activates a state called systemic acquired resistance (SAR). SAR involves the upregulation of pathogenesis-related (PR) proteins — chitinases, glucanases, and other antimicrobial proteins — throughout the plant, making undamaged tissues resistant to subsequent pathogen attack (Ryals et al., 1996). SAR can persist for days to weeks, providing a plant-wide immune memory that outlasts the specific threat that induced it.

The jasmonate and salicylate pathways do not operate independently: they interact, sometimes synergistically, sometimes antagonistically. Pathogens that suppress the salicylate pathway can reactivate susceptibility to bacteria. Some herbivores exploit this cross-talk, introducing salicylate-inducing compounds in their saliva to suppress the jasmonate-based defense that would otherwise make the plant a poorer food source. This manipulation suggests that the information content of the defense signaling system is legible to the plant's antagonists as well as to the plant itself — a form of evolutionary arms race in molecular communication.

---

## 2.3 Electrical Signals and Calcium Waves

The vascular transport of hormones is not the only mode of long-distance communication within plants. Plants also generate and propagate electrical signals — rapid changes in membrane potential that travel through the plant body. These were first described in sensitive plants like *Mimosa pudica* (where they accompany the rapid leaf-folding response to touch) and carnivorous plants like Venus flytraps and sundews. But they are not limited to these dramatic cases.

The electrical signals in plants are not identical to animal action potentials — they involve different ion channels and propagate through different cell types — but they share functional features: a rapid, propagating change in membrane potential that carries information about a local stimulus to distant tissues faster than diffusion-based chemical signals could.

Mousavi et al. (2013) provided some of the clearest molecular evidence for electrically propagating signals in a non-specialist plant. Using genetically encoded calcium sensors in *Arabidopsis thaliana*, they demonstrated that wounding one leaf triggers a wave of elevated calcium concentration that propagates through the plant's vascular system to distant leaves within minutes — much faster than hormone diffusion could account for. The calcium wave travels through the phloem and xylem at speeds consistent with a propagated signal rather than simple diffusion, and its arrival in distant leaves is correlated with the activation of jasmonate-dependent defense genes.

The identification of the calcium channel responsible for this wave came from the same group: GLR3.3 and GLR3.6, glutamate receptor-like channels in the phloem, are required for systemic wound signaling. When these channels are mutated, the calcium wave fails to propagate and systemic defense induction is impaired. The plant's equivalent of a neural signal — a propagating calcium wave carried by glutamate receptor-like channels — turns out to use molecular machinery that is recognizably related to the glutamate receptors of animal nervous systems, even though the implementation is quite different.

This finding has generated debate. Taiz et al. (2019) argue that describing these calcium waves as "plant action potentials" or drawing direct analogies to neural signaling is misleading, because the timescales, propagation mechanisms, and computational roles are different enough to make the analogy confusing rather than illuminating. They are correct that the analogy should not be taken too literally. But the functional similarity — a rapidly propagating, long-distance signal that carries information about a local event and elicits a coordinated response in distant tissues — is real and worth acknowledging as long as the differences are kept in view.

---

## 2.4 Systemic Acquired Resistance as Distributed Memory

Systemic acquired resistance is, from a cognitive perspective, one of the most interesting aspects of plant vascular signaling because it has the hallmarks of memory. A local pathogen attack induces a systemic state — elevated PR protein expression, activated SA signaling, altered cell wall composition — that persists for days to weeks and makes the entire plant more resistant to subsequent attack. The plant has been changed by its experience in ways that alter its future responses.

This is not identical to learning in animals: the change is chemical and physiological rather than synaptic and neural, and it is expressed throughout the plant rather than in specific tissue. But it is functionally analogous to immunological memory — and immunological memory is itself a form of learned response, in the sense that prior exposure changes the system's future response to the same stimulus.

The molecular mediator of SAR-based memory is not yet fully characterized. Salicylic acid itself travels through the phloem and contributes to systemic signaling, but it appears to be insufficient alone. A lipid-based signal, methyl salicylate, has been proposed as the "mobile signal" that travels in the phloem and activates SAR in distant tissues. The pathway is likely redundant — multiple signals contributing to the systemic state rather than a single messenger carrying all the information.

What matters for our purposes is the functional description: the plant's vascular system acts as a distributed information network that carries signals about local threat states to distant tissues, activates coordinated protective responses across the entire plant, and establishes a persistent state of enhanced readiness that constitutes a form of somatic memory. The plant without a nervous system achieves, through hormonal and electrical vascular signaling, something that deserves to be called systemic information integration and adaptive response.

---

## References

Mousavi, S. A. R., Chauvin, A., Pascaud, F., Kellenberger, S., & Farmer, E. E. (2013). Glutamate receptor-like genes mediate leaf-to-leaf wound signalling. *Nature*, 500(7463), 422–426.

Pearce, G., Strydom, D., Johnson, S., & Ryan, C. A. (1991). A polypeptide from tomato leaves induces wound-inducible proteinase inhibitor proteins. *Science*, 253(5022), 895–897.

Ryals, J. A., Neuenschwander, U. H., Willits, M. G., Molina, A., Steiner, H. Y., & Hunt, M. D. (1996). Systemic acquired resistance. *Plant Cell*, 8(10), 1809–1819.

Taiz, L., Alkon, D., Draguhn, A., Murphy, A., Blatt, M., Hawes, C., Thiel, G., & Robinson, D. G. (2019). Plants neither possess nor require consciousness. *Trends in Plant Science*, 24(8), 677–687.
