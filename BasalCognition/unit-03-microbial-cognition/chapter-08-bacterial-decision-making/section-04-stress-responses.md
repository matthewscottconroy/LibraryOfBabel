# Section 4: Stress Responses and Survival Decisions

Bacteria face a world of unpredictable and potentially lethal chemical and physical stresses: UV radiation, reactive oxygen species, DNA-alkylating agents, antibiotics, osmotic shock, heat, cold, and starvation. The strategies they have evolved to detect, respond to, and survive these stresses are some of the most sophisticated information-processing programs in cellular biology. They are also, when examined in the cognitive framework of this book, clear examples of decision-making: bacteria detecting a threat, integrating information about its severity, and executing a coherent, multi-component behavioral response that is calibrated to the threat.

---

## The SOS Response: DNA Damage Decision

The SOS response is the bacterial emergency response to DNA damage — a comprehensive program of damage repair, mutagenesis, and cell cycle arrest that is activated when the bacterium's genome is damaged beyond a threshold level. The decision to activate the SOS response is, in many respects, the paradigm of bacterial cellular decision-making.

The key sensor of the SOS response is the protein RecA. When DNA is damaged, single-stranded DNA (ssDNA) is exposed — either directly by the damage or by the exonuclease activity of repair enzymes trying to process the damage. RecA binds cooperatively to ssDNA, forming a nucleoprotein filament that is the active, signaling form of RecA. The RecA filament stimulates the autoproteolysis of LexA — the master repressor of the SOS regulon. With LexA cleaved and inactive, the promoters of over 40 SOS genes are derepressed, allowing their expression.

The SOS regulon includes genes for DNA repair (recA itself, uvrABC, polB, dinB), genes that arrest cell division (sulA), and genes encoding error-prone DNA polymerases (polB/Pol II, dinB/Pol IV, umuCD/Pol V) that can copy past damaged DNA templates ("translesion synthesis") at the cost of introducing mutations. The decision to activate error-prone polymerases — to accept increased mutation in exchange for DNA replication through damage — is one of the most consequential decisions a bacterium makes. It is a calculated bet: better to replicate (even mutagenically) than not to replicate at all.

The SOS response has a temporal structure that reflects decision logic. The early SOS genes (those with the weakest LexA binding sites) are derepressed first, implementing the least costly responses (repair). The late genes (those with the strongest LexA binding sites) are derepressed only with more DNA damage, implementing more costly responses (mutagenic polymerases). The system thus implements a graduated response, calibrated to damage severity, rather than a binary on/off switch.

---

## The Stringent Response: Starvation Decision

When bacteria face amino acid starvation — a condition that threatens to stall translation and halt growth — they activate the stringent response: a dramatic, global reprogramming of cellular priorities driven by the second messenger (p)ppGpp (guanosine tetra- and pentaphosphate).

The trigger is simple and elegant. Ribosomes normally require aminoacyl-tRNA — tRNA charged with its cognate amino acid — to proceed with translation. When an amino acid is scarce, uncharged (aminoacyl-free) tRNA accumulates and enters the ribosomal A site. This normally-forbidden occupancy activates the ribosome-associated synthetase RelA, which synthesizes (p)ppGpp from GTP and ATP. High (p)ppGpp concentrations then act globally on RNA polymerase, redirecting it away from growth-related genes (rRNA, ribosomal proteins, flagellar genes) and toward survival-related genes (amino acid biosynthesis, stress response, virulence in pathogens).

The stringent response is a cellular decision to abandon growth and invest in survival. It is triggered by a specific, quantifiable signal (uncharged tRNA in the ribosomal A site), and it produces a calibrated, global response that is appropriate to the detected condition. The decision is not binary — (p)ppGpp levels rise gradually with the severity of starvation, and the regulatory effects on different promoters have different sensitivities to (p)ppGpp, creating a graded, prioritized response.

The stringent response also demonstrates the integration of multiple signals. SpoT, a bifunctional enzyme that both synthesizes and degrades (p)ppGpp, integrates information about fatty acid starvation, carbon starvation, and other stresses to set the overall (p)ppGpp level. The cell integrates multiple starvation signals into a single global regulatory output — a demonstration of the kind of signal integration that we have argued is central to cellular cognition.

---

## Persister Cells: The Decision Not to Grow

Among the most fascinating examples of bacterial decision-making is the phenomenon of persister cells — a small fraction of any bacterial population that spontaneously enters a metabolically dormant state that renders them highly tolerant to antibiotics. Persisters are not genetically resistant to antibiotics; they are phenotypically tolerant, because most antibiotics require active metabolism to kill. A cell that is not metabolizing, not dividing, not transcribing, is largely immune to drugs that kill by disrupting these processes.

Persister formation is a stochastic process: individual cells switch into the persister state at a low, constitutive rate, regardless of the presence or absence of antibiotic. The switches appear to be driven by stochastic fluctuations in the levels of toxin-antitoxin (TA) module proteins. Bacterial genomes typically encode multiple TA modules, each consisting of a toxin (which inhibits growth when active) and an antitoxin (which neutralizes the toxin by binding it directly or by activating toxin-degrading proteases). When the antitoxin is stochastically degraded and the toxin transiently exceeds the antitoxin, growth is arrested, creating a persister cell (Keren et al., 2004).

From a cognitive perspective, persister formation is a form of bet-hedging decision: the population "decides" (at the population level, through a stochastic molecular mechanism) to maintain a small fraction of cells in a survival mode, accepting that these cells will not contribute to current growth in exchange for their potential to restart the population if antibiotic treatment eliminates the growing majority. The decision mechanism — stochastic fluctuations in TA module balance — is intrinsically random, ensuring that no environmental signal is needed to trigger persister formation. The population is always betting a small fraction of itself on survival.

This has profound clinical implications. Persister cells are the likely source of antibiotic treatment failures in many chronic bacterial infections (including *Pseudomonas aeruginosa* in cystic fibrosis, *Mycobacterium tuberculosis* in tuberculosis, and *Staphylococcus aureus* in device-associated infections). Understanding persister cell formation as a cognitive decision — a bet-hedging strategy — reframes the therapeutic challenge: the problem is not just to kill sensitive bacteria but to deprive the population of its survival option (Balaban et al., 2004).

---

## Sporulation: The Ultimate Survival Decision

*Bacillus subtilis* and related species have evolved the most extreme bacterial survival strategy: sporulation. When nutrient deprivation is severe and prolonged, *B. subtilis* can initiate a complex developmental program that transforms the vegetative bacterium into an endospore — a dormant, highly resistant cell with a protein coat, DNA-protective proteins, and metabolic inactivity that can survive heat, desiccation, UV radiation, and many chemical stresses for thousands of years.

Sporulation is the most complex developmental decision a bacterium makes. It is irreversible once committed (the cell will never return to vegetative growth; instead, the spore must germinate as a new vegetative cell), it is metabolically costly, and it requires the coordination of over 100 genes in a temporally ordered program that takes approximately 8 hours to complete. The cell commits to this program only when starvation is severe and prolonged — not at the first sign of nutrient limitation, which might be transient.

The decision circuit for sporulation — the Spo0A phosphorelay — is a multicomponent signal integrator that weighs inputs from multiple kinases (representing different stress signals), a phosphatase network that modulates phospho-Spo0A levels, and the level of Spo0A protein itself. The system implements bistability through a positive feedback loop: Spo0A-P activates the transcription of its own gene, but also of genes that inhibit the phosphatases that degrade Spo0A-P, reinforcing its own accumulation (Dubnau & Losick, 2006). The result is a switch-like, bistable commitment to sporulation that is irreversible once Spo0A-P exceeds a threshold — a molecular implementation of an irrevocable decision.

---

## Stress Responses as a Cognitive Portfolio

Stepping back, the suite of bacterial stress responses — SOS response to DNA damage, stringent response to amino acid starvation, persister formation, sporulation — constitutes a cognitive portfolio: a set of distinct behavioral strategies, each triggered by specific information about the current environmental state, each calibrated to the severity and nature of the detected threat.

What is remarkable about this portfolio is not just that individual responses are sophisticated but that they are coordinated. The stringent response and the SOS response interact: (p)ppGpp influences DNA repair gene expression; DNA damage can trigger (p)ppGpp synthesis. The stringent response and persister formation interact: high (p)ppGpp promotes toxin-antitoxin module activation, increasing persister frequency. The sporulation decision integrates multiple stress signals, not just amino acid starvation. The bacteria are not running separate, unconnected stress response programs; they are running an integrated information-processing system that detects the type, severity, and duration of stress and selects the most appropriate behavioral response.

This integrated, information-driven stress response system is bacterial cognition at scale. It would be intellectually dishonest to describe it as anything less than decision-making.

---

## References

Balaban, N. Q., Merrin, J., Chait, R., Kowalik, L., & Leibler, S. (2004). Bacterial persistence as a phenotypic switch. *Science*, *305*(5690), 1622–1625.

Berg, H. C. (2004). *E. coli in Motion*. Springer.

Dubnau, D., & Losick, R. (2006). Bistability in bacteria. *Molecular Microbiology*, *61*(3), 564–572.

Keren, I., Shah, D., Spoering, A., Kaldalu, N., & Lewis, K. (2004). Specialized persister cells and the mechanism of multidrug tolerance in *Escherichia coli*. *Journal of Bacteriology*, *186*(24), 8172–8180.

Sourjik, V., & Wingreen, N. S. (2012). Responding to chemical gradients: bacterial chemotaxis. *Current Opinion in Cell Biology*, *24*(2), 262–268.
