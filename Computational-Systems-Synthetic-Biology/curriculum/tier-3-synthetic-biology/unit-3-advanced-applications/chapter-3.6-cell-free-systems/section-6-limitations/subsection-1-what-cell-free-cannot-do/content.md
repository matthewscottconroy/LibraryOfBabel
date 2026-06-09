# What Cell-Free Systems Cannot Do

Any powerful tool is dangerous if you misunderstand its limits. Cell-free systems are genuinely transformative — they compress the design cycle, open new chemistry, and enable diagnostics that save lives. But they are not universal substitutes for cellular systems, and treating them as such will lead you astray. Understanding the fundamental limitations of cell-free systems — not just practical inconveniences but intrinsic conceptual barriers — is essential for designing experiments appropriately and interpreting results correctly. The limitations fall into several categories: processes that require cell growth or division, phenomena that depend on the full complexity of cellular organization, and practical constraints on reaction duration, scale, and cost.

## Limitations Requiring Cell Growth or Division

**Cell growth itself**: the most obvious limitation. Cell-free systems do not grow. Any experiment that requires observing cell proliferation — measuring growth rates under selective pressure, studying growth competition between genotypes, or following evolutionary trajectories over generations — cannot be done in cell-free. This eliminates fitness-based selection experiments (except when clever in vitro selection schemes are engineered), population dynamics, and ecology-like experiments.

**Antibiotic tolerance vs. resistance**: studying whether a mutation confers antibiotic resistance requires growing cells under antibiotic pressure. Cell-free cannot replicate this because there is no growth to be inhibited.

**Biofilm formation**: biofilm is an emergent property of cell populations adhering to surfaces and producing extracellular matrix. It requires cell-cell communication, division, and physical structure — none of which are present in cell-free reactions.

**Quorum sensing and population-level behaviors**: while individual quorum-sensing circuits can be reconstituted in cell-free (expressing LuxI to produce AHL, then adding AHL to activate LuxR-driven expression), the **population dynamics** of quorum sensing — how a population transitions from low-density to high-density signaling state across a threshold — require actual populations of cells at different densities. Cell-free reactions are single "pseudo-cells" of fixed volume.

## Limitations Due to Absence of Cellular Organization

**Membrane-associated processes**: many critical cellular functions occur at the plasma membrane or intracellular membrane systems. Signal transduction through receptor tyrosine kinases (RTKs), GPCRs, ion channels, and lipid-anchored signaling molecules requires a membrane with the correct lipid composition, asymmetry, and associated membrane proteins. While proteoliposomes can reconstitute single membrane proteins in cell-free-like formats, the complexity of signaling networks at real cell membranes cannot be replicated.

**Organelle-dependent processes**: eukaryotic cell biology depends on the endoplasmic reticulum (protein folding, disulfide bond formation, N-glycosylation), Golgi (protein glycosylation, sorting), lysosomes (degradation), and mitochondria (respiration). Cell-free extracts from eukaryotic cells retain some of these activities (ER microsomes in wheat germ extract support some co-translational translocation), but the full fidelity of the secretory pathway — including trafficking, sorting, and post-translational modification — is absent.

**Post-translational modifications**: 
- **Phosphorylation**: cell-free extracts contain kinases, but the specific spatial organization of kinase/phosphatase pairs at scaffolds and membranes is absent
- **Ubiquitination and proteasomal degradation**: ubiquitin pathway enzymes (E1/E2/E3 ligases) are present in crude extracts but at sub-cellular-level organization; substrate-specific degradation is poorly replicated
- **Glycosylation**: N-glycosylation occurs co-translationally in the ER and requires the dolichol-phosphate substrate anchored to ER membranes — not available in a soluble cell-free reaction without specialized ER-derived vesicles
- **Lipid modifications**: myristoylation, palmitoylation, GPI-anchor attachment — membrane-dependent modifications not accessible in standard cell-free

**Chromatin structure**: gene expression in eukaryotes is controlled by chromatin state — nucleosome positioning, histone modifications, DNA methylation. Cell-free transcription from naked plasmid DNA bypasses all of these. This means cell-free cannot be used to study epigenetic regulation, chromatin remodeling, or the effect of DNA methylation on transcription factor binding.

## Practical Limitations

**Reaction duration**: batch cell-free reactions exhaust their energy supply in 2–8 hours (depending on the energy system). Continuous exchange cell-free (CECF) extends this to 24–48 hours, but CECF adds complexity and cost. Processes that require days to weeks (e.g., protein complex assembly kinetics, long-term signaling dynamics) are not accessible in standard cell-free formats.

**Volume and scale**: cell-free reactions are practically limited to µL–mL scale in laboratory settings. Sutro Biopharma and a few industrial actors operate at 100-liter scale, but this requires specialized bioreactor equipment. Standard lab cell-free reactions cannot produce the grams of protein needed for structural biology (X-ray crystallography, cryo-EM), clinical trials, or industrial applications.

**Cost**: crude E. coli extract costs approximately $0.50–2 per 10 µL reaction (reagent costs for energy supplement + extract). The PURE system is 10–50× more expensive. For screening applications with hundreds of reactions, costs accumulate quickly. In contrast, growing 1 L of E. coli expressing a protein of interest costs ~$5–15 in media and plasmid reagents.

**Protein solubility and folding**: cell-free systems lack the full chaperone network present in cells. Highly aggregation-prone proteins or proteins requiring specific chaperones (ClpB, GroEL/ES system) for proper folding may produce soluble protein poorly in cell-free. While GroEL/ES can be added to PURE, the chaperone requirements of each protein are not always predictable.

**Membrane protein expression**: integral membrane proteins require a lipid bilayer for proper folding and stability. Cell-free systems produce soluble cytoplasmic proteins efficiently but struggle with membrane proteins unless detergent (e.g., DDM, LMNG) or nanodiscs (lipid bilayer discs supported by membrane scaffold protein) are added to the reaction. This works for some membrane proteins but not universally.

## What Cell-Free Cannot Tell You About In Vivo Behavior

Beyond physical limitations, there are epistemic limitations — things cell-free can measure but whose relevance to in vivo behavior is uncertain:

**Absolute expression levels**: protein concentrations in cell-free reactions are in the mg/mL (µM) range. In E. coli cells, most proteins are present at nM–µM concentrations. The absolute expression levels measured in cell-free do not predict absolute in vivo levels.

**Resource competition with endogenous genes**: in cells, expressing a new gene competes with thousands of endogenous genes for ribosomes, RNA polymerase, and ATP. In cell-free (especially PURE), there is no such competition. This means cell-free may give optimistic predictions for expression levels in cells where the new gene is competing with the full endogenous transcriptome.

**Regulatory network context**: in cells, a gene's expression is influenced by global regulators (sigma factors, small RNAs, ppGpp-mediated stringent response) that respond to growth phase, stress, and nutrient availability. Cell-free reactions capture none of this regulatory context.

**Evolutionary pressure and mutation**: cells can acquire mutations that circumvent engineered designs. This is irrelevant in cell-free, but critical to know for long-term in vivo deployment of synthetic circuits.

## When to Use Cell-Free vs. Cellular Systems

| Question | Cell-Free | Cellular System |
|---|---|---|
| Does this genetic circuit produce output? | Yes (rapid screening) | Required for deployment context |
| What is the rank order of these promoters? | Yes (correlates well in vivo) | Needed for absolute calibration |
| Will this circuit be stable over 100 generations? | No | Required |
| Can I incorporate an ncAA at position 47? | Yes | Difficult (RF1 competition) |
| Does this protein interact with its partner? | Yes | Needed for in vivo validation |
| What happens when growth is inhibited by antibiotic? | No | Required |
| What is the yield from fermentation? | No | Required |

## Why This Matters

Knowing what cell-free systems cannot do is as important as knowing what they can do — perhaps more so. The most common misuse of cell-free data is over-extrapolation: assuming that because a circuit works in cell-free (or does not), it will (or will not) work in cells, with the same kinetics, the same expression levels, and the same regulatory behavior. The appropriate role of cell-free in the DBTL cycle is as a **filter**: screening out designs that fail obvious tests (wrong logic truth table, no expression, no bistability) before committing to the experimental cost of in vivo characterization. It is not a replacement for in vivo validation. Practitioners who understand both the capabilities and the limitations of cell-free systems will use them to maximum effect — accelerating the design cycle without being misled by the differences between the test tube and the living cell.
