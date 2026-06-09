# Section 2: Priming, Sensitization, and Immune Memory

Memory in the neurological sense involves the persistence of a changed response to a previously encountered stimulus. Sensitization is the increase in response magnitude following prior exposure; habituation is the decrease. Both represent stored information about past experience that modifies current behavior. Neither requires neurons. This section examines how these basic forms of experiential modification appear in immune cells, microbial systems, and molecular machines like the CRISPR-Cas system.

---

## Immune Memory: Learning at the Population Level

The vertebrate adaptive immune system is the most thoroughly characterized example of cellular learning in biology. When a naive lymphocyte — a B cell or T cell that has not previously encountered antigen — is activated by its specific target, it does not merely mount a response: it transforms into a population that includes long-lived memory cells that can persist for decades in the organism. On subsequent encounter with the same antigen, these memory cells respond far more rapidly, with greater magnitude, and produce higher-affinity antibodies than the primary response.

This is, by any reasonable definition, learning. The immune system has encountered a pattern (an antigen), stored information about that encounter (in the form of long-lived memory cells with appropriate antigen receptors), and modified its subsequent behavior in response to that stored information (the memory response). The learning is specific — memory cells respond much more strongly to the original antigen than to a related but distinct one. It is durable — immune memory can persist for the lifetime of the organism. And it is adaptive — the memory response is better suited to eliminating the pathogen than the primary response was.

The molecular mechanisms of immune memory involve both cellular and molecular changes. Memory T cells express distinct surface markers (CD44high, CD122high) and have different transcriptional and epigenetic profiles from naive T cells — including specific histone modifications at the promoters of rapid-response genes that keep those genes "poised" for rapid activation. When a memory T cell encounters its antigen again, the open chromatin at these loci allows rapid transcription and fast effector function. The epigenetic marks of memory T cells are thus the molecular substrate of immunological memory — they encode, in a durable and heritable chromatin format, the information that this antigen has been encountered before and should be rapidly responded to.

---

## Trained Immunity: Memory in Innate Immune Cells

For decades, immunological memory was thought to be exclusively a property of adaptive immune cells (lymphocytes) — cells with specific antigen receptors that can be selected and clonally expanded. Innate immune cells — macrophages, neutrophils, natural killer cells — were thought to lack memory, responding identically to any given stimulus regardless of prior exposure.

This view has been substantially revised by the discovery of "trained immunity" — a form of memory in innate immune cells that was systematically characterized by Mihai Netea and colleagues (Netea et al., 2011). Following exposure to certain pathogens or pathogen-derived molecules (particularly beta-glucan, a component of fungal cell walls), macrophages show enhanced responses to subsequent stimulation with a variety of different pathogens — a non-specific enhancement of inflammatory capacity. This trained state persists for weeks or months and can even be transmitted to monocyte precursors in the bone marrow.

The molecular basis of trained immunity is epigenetic: exposure to beta-glucan or other training stimuli induces specific histone methylation and acetylation changes at the promoters of inflammatory genes, increasing their accessibility and transcriptional readiness. The trained macrophage has, in effect, "experienced" a pathogen encounter and stored that experience in its chromatin in a way that prepares it for faster, stronger responses to future challenges.

Trained immunity is philosophically important for this chapter because it shows that memory-like phenomena can arise in cells without antigen-specific receptors or clonal expansion — the features that distinguish adaptive immunity. The mechanism is simpler and more general: experience changes chromatin state, and chromatin state persists across divisions. This is essentially the same mechanism as epigenetic memory in development, applied to the problem of immune experience.

---

## CRISPR-Cas: Molecular Memory of Infection

The CRISPR-Cas system represents perhaps the most literal form of cellular memory in biology: a molecular database in which bacteria record the genetic sequences of bacteriophage viruses that have previously infected them, using those stored sequences to recognize and destroy the same viruses on future encounters.

CRISPR (Clustered Regularly Interspaced Short Palindromic Repeats) loci are found in the genomes of roughly half of all bacterial species. They consist of short, repeating DNA sequences interspersed with unique "spacer" sequences — snippets of foreign DNA derived from viruses or plasmids that have previously entered the cell. These spacers are the memory: each one records a past infection event. During a new infection, the spacers are transcribed into guide RNAs that direct the Cas nucleases to cleave matching DNA sequences in the invading phage genome.

The acquisition of new spacers — the "immunization" step — requires the action of Cas1 and Cas2 proteins, which capture short DNA fragments from the invading pathogen and insert them into the CRISPR array. This process is specific: the spacer sequence is derived from the invader's DNA, not from the bacterium's own genome (a protospacer adjacent motif, or PAM, provides a molecular signal that distinguishes foreign from self). The result is that each infection either kills the cell or immunizes it against future infection by the same strain.

CRISPR-Cas is remarkable as a memory system because it stores information in the most durable possible format — the DNA sequence itself. Unlike epigenetic marks, which can be erased by reprogramming, or RNA-based memory, which is unstable, CRISPR spacers are as stable as the genome. They are faithfully replicated and transmitted to daughter cells, providing multigenerational immunity. In environments where phage pressure is chronic, the CRISPR locus can expand to dozens or hundreds of spacers — a literal memory bank of infectious encounters.

---

## Stress Memory in Microbes

Beyond CRISPR, microbes exhibit various forms of stress memory — changes in cellular state following a stress exposure that persist for some time and alter the cell's response to subsequent stresses.

**Heat shock memory**: Bacteria exposed to a sublethal heat shock become transiently more resistant to a subsequent lethal heat shock — a classic "pre-conditioning" or "priming" effect. This thermotolerance depends on the induction of heat shock proteins (chaperones like GroEL, DnaK) during the first exposure; these proteins are still present in elevated amounts when the second heat shock arrives and provide protection. The memory is short-lived — it decays as heat shock protein levels return to baseline — but represents a genuine form of experience-dependent behavior modification.

**Osmotic stress memory**: *E. coli* cells that have experienced osmotic stress exhibit altered gene expression patterns that persist for several generations after the stress is removed, even when the osmolarity returns to baseline. These persistent expression changes involve modifications to RNA polymerase sigma factor ratios and to the relative expression of metabolic enzymes, and they alter the cell's physiology in ways that could be adaptive in future osmotic stress encounters. This is a form of epigenetic stress memory implemented at the transcriptional rather than chromatin level — a faster but less durable form of information storage.

**Oxidative stress priming**: Exposure of bacteria to sublethal oxidative stress (hydrogen peroxide) activates the OxyR regulon, inducing a set of antioxidant enzymes. This activation can persist for some time after the peroxide is removed, and it prepares cells for subsequent oxidative challenge. In some contexts, the history of oxidative stress exposure appears to influence the spontaneous mutation rate of the bacteria — because oxidative damage to DNA and ROS can be mutagenic — creating a lasting genetic record of the stress encounter. This is a striking example of where the boundary between epigenetic and genetic memory becomes blurred.

---

## Priming vs. Sensitization vs. Habituation

The terminology of learning science distinguishes several types of experience-dependent behavioral modification:

- **Priming**: Prior exposure to a stimulus alters the processing of a subsequent related stimulus, typically by reducing the threshold for response or increasing response magnitude. Trained immunity and heat shock pre-conditioning are examples.
- **Sensitization**: Repeated exposure to a stimulus increases the magnitude of the response — the opposite of habituation. In immunology, repeated antigen exposure leads to class switching and affinity maturation — the immune response gets better, not weaker, with repetition. This is sensitization at the population level.
- **Habituation**: Repeated exposure to a stimulus decreases the magnitude of the response, as the system "learns" that the stimulus predicts nothing important. The *Stentor* results (Section 3) are the best non-neural example.

What these three phenomena share is that they are all forms of experience-dependent modification of future behavior — all forms of cellular learning in the functional sense. What distinguishes them is the direction and duration of the modification, and whether it is specific to the experienced stimulus or generalized. Mapping these distinctions onto cellular systems without nervous systems — as we are doing in this chapter — requires care, because the cellular systems often blur the distinctions that were originally drawn in neural contexts. But the underlying conceptual framework remains useful: experience modifies state, and modified state alters behavior.

---

## References

Heard, E., & Martienssen, R. A. (2014). Transgenerational epigenetic inheritance: myths and mechanisms. *Cell*, *157*(1), 95–109.

Netea, M. G., Quintin, J., & van der Meer, J. W. M. (2011). Trained immunity: a memory for innate host defense. *Cell Host & Microbe*, *9*(5), 355–361.

Rechavi, O., Minevich, G., & Hobert, O. (2011). Transgenerational inheritance of an acquired small RNA-based antiviral response in *C. elegans*. *Cell*, *147*(6), 1248–1256.

Wiedenheft, B., Sternberg, S. H., & Doudna, J. A. (2012). RNA-guided genetic silencing systems in bacteria and archaea. *Nature*, *482*(7385), 331–338.
