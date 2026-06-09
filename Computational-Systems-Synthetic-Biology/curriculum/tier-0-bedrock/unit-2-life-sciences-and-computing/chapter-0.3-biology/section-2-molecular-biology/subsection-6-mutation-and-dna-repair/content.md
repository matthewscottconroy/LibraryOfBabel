# Mutation and DNA Repair

Every day, each of your cells suffers approximately 10,000 instances of DNA damage. Depurination alone — the spontaneous hydrolysis of the glycosidic bond that holds guanine or adenine to the sugar-phosphate backbone — happens at this rate from nothing more than the thermal motion of water molecules. Add in deamination of cytosine, oxidative lesions from reactive oxygen species, and the inevitable mistakes of DNA polymerase, and you have a genome under continuous assault. Yet the actual mutation rate in human somatic cells is less than one new mutation per cell division. How? Through an interlocking network of damage-sensing, excision, and re-synthesis pathways that collectively constitute the DNA repair systems of the cell. Understanding them quantitatively links molecular biology to evolution, cancer biology, and genome engineering.

The genome is not a static archive. DNA is continuously damaged by endogenous metabolic processes, exogenous environmental agents, and the imperfect fidelity of replication itself. Without repair, these lesions would rapidly accumulate, causing mutations that compromise gene function or trigger cancer.

## Types of Mutations

**Point mutations** involve a single base pair:
- **Transitions**: purine ↔ purine (A↔G) or pyrimidine ↔ pyrimidine (C↔T); more common because the geometry of the mismatch is less distorting
- **Transversions**: purine ↔ pyrimidine (A/G ↔ C/T); less common
- Synonymous (silent), missense, or nonsense (stop-creating) depending on codon context

**Small indels** (insertions/deletions): cause frameshifts if not in multiples of 3. The most pathogenic class for protein-coding genes.

**Chromosomal rearrangements**: inversions (a segment reversed), translocations (segments exchanged between chromosomes), amplifications, deletions. These alter gene dosage and can create fusion oncogenes (e.g., BCR-ABL in chronic myeloid leukemia).

## Sources of DNA Damage

**Endogenous sources** are the most quantitatively significant:
- **Depurination**: spontaneous hydrolysis of the N-glycosidic bond, releasing a purine base. Rate: ~10,000 events/cell/day. Leaves an **AP site** (abasic site) that is non-informative for replication.
- **Deamination**: cytosine → uracil (rate ~100–500/cell/day); 5-methylcytosine → thymine (C→T transitions at CpG sites are the most common single-nucleotide variant in the human germline). Adenine → hypoxanthine (decoded as G).
- **Reactive oxygen species (ROS)**: 8-oxoguanine (8-oxoG) is a major oxidative lesion; it mispairs with A during replication, causing G:C → T:A transversions.
- **Replication errors**: residual mismatches after proofreading; ~1 per 10⁷ bp.

**Exogenous sources:**
- **UV light**: forms **cyclobutane pyrimidine dimers (CPDs)** and **6-4 photoproducts (6-4 PPs)** at adjacent pyrimidines (TT, TC, CT, CC). These distort the helix and block replication.
- **Ionizing radiation**: causes double-strand breaks (DSBs) via radical chemistry. ~40 DSBs/cell/Gy.
- **Alkylating agents**: methylate bases (e.g., O6-methylguanine from MNNG), distorting base pairing and causing G:C → A:T transitions.
- **Intercalating agents**: acridines insert between base pairs, causing frameshift mutations during replication.

## Repair Pathway Overview

Different lesions are repaired by distinct pathways. Understanding which pathway applies to which lesion type is essential for interpreting mutational signatures.

### Base Excision Repair (BER)

Targets: small, non-helix-distorting lesions (8-oxoG, AP sites, deaminated bases).

Mechanism (short-patch BER):
1. **DNA glycosylase** recognizes the damaged base and excises it, leaving an AP site (monofunctional) or a nick (bifunctional)
2. **APE1 (AP endonuclease)** cleaves the phosphodiester backbone 5' to the AP site, leaving a 3'-OH and a 5'-dRP
3. **Pol β** removes the 5'-dRP by its lyase activity and inserts 1–4 correct nucleotides
4. **Ligase III/XRCC1** seals the nick

### Nucleotide Excision Repair (NER)

Targets: bulky, helix-distorting lesions (UV-CPDs, 6-4PPs, large adducts). This pathway is why sun exposure causes C→T mutations at dipyrimidines.

Two sub-pathways:
- **Global genome NER (GG-NER)**: scans the entire genome; XPC/RAD23B recognizes distortion
- **Transcription-coupled NER (TC-NER)**: repairs the transcribed strand faster; stalled RNAP recruits CSB/CSA

Core NER mechanism:
1. Damage recognition → local unwinding by **XPD/XPB** helicases (components of TFIIH)
2. **XPA** verifies the lesion; **RPA** stabilizes ssDNA
3. **XPG** (3' cut) and **XPF-ERCC1** (5' cut) make incisions ~22–30 nt apart, excising an oligonucleotide
4. DNA Pol δ/ε fills the gap; ligase seals

Defects in NER cause **Xeroderma pigmentosum** (XP), characterized by extreme UV sensitivity and >1000-fold increased skin cancer risk.

### Mismatch Repair (MMR)

Targets: replication-induced mismatches and small indel loops.

In *E. coli*: **MutS** recognizes the mismatch → **MutL** recruits **MutH**, which cleaves the unmethylated (newly synthesized) strand at GATC sites → **UvrD** helicase unwinds → exonuclease degrades toward and past the mismatch → Pol III re-synthesizes → Ligase seals.

In humans, **MSH2/MSH6** recognize mismatches; **MLH1/PMS2** coordinate repair. Germline mutations in these genes cause **Lynch syndrome** (hereditary nonpolyposis colorectal cancer, HNPCC). MMR-deficient tumors accumulate thousands of insertion/deletion mutations at microsatellites (**microsatellite instability, MSI-H**) and are hypersensitive to immunotherapy.

### Double-Strand Break Repair

DSBs are the most dangerous lesion because both strands are severed. Two main pathways with different fidelity:

**Non-Homologous End Joining (NHEJ)** — dominant in G1 and G0:
1. **Ku70/Ku80** heterodimer binds and protects DSB ends
2. **DNA-PKcs** kinase is recruited; phosphorylates H2AX (γH2AX, a repair focus marker)
3. Ends are processed (often imperfectly) by Artemis and Pol μ/λ
4. **XRCC4/Ligase IV/XLF** ligate the ends
- NHEJ is fast but error-prone; small insertions/deletions at the junction are common

**Homologous Recombination (HR)** — restricted to S/G2 when sister chromatid is available:
1. **MRN complex (MRE11-RAD50-NBS1)** senses DSBs; nucleolytic processing creates 3'-ssDNA overhangs
2. **RPA** coats ssDNA; **BRCA2** loads **RAD51** recombinase
3. **RAD51 nucleoprotein filament** invades the homologous sister chromatid (strand invasion), forming a D-loop
4. DNA synthesis uses the sister chromatid as template; Holliday junction resolution or synthesis-dependent strand annealing (SDSA) completes repair
- HR is high-fidelity; it is the basis of CRISPR-mediated HDR (homology-directed repair) for precise genome editing

## Mutational Signatures

Different mutational processes leave characteristic patterns detectable in tumor genomes. **SBS (single base substitution) signatures** in the COSMIC database assign trinucleotide context (the base 5' and 3' of the mutated base) to 96 possible mutation types, then decompose tumor mutation spectra into signatures. For example:
- **Signature 1** (age-related): C→T at CpG, reflecting spontaneous deamination of 5mC
- **Signature 4** (tobacco smoking): predominantly C→A, reflecting polycyclic aromatic hydrocarbon adducts repaired by NER
- **Signature 3** (BRCA1/2 deficiency, HR defect): elevated deletions and specific SBS pattern

The key insight of mutational signatures is that you can read the history of a tumor's mutagenic exposures from its genome, in the same way that geologists read a rock formation. The tumor's mutation spectrum is a record of every environmental insult and every DNA repair failure that occurred during its development.

## Why This Matters for Computational Biology

Mutation rates parameterize every evolutionary model: the probability of fixation of a beneficial mutation depends on $\mu$, $N_e$, and $s$. Cancer genome analysis uses mutation signature decomposition to infer etiology and defective repair pathways from somatic mutation catalogs. CRISPR editing outcomes are mechanistically determined by NHEJ vs. HDR: high NHEJ activity (G1 cells) favors indels; HDR-competent cells with a repair template produce precise edits. Understanding DSB repair informs strategies to improve HDR efficiency (e.g., inhibiting NHEJ with DNA-PK inhibitors, or using RAD51-stimulating compounds). The mutation spectrum also encodes information about selective pressures: nonsynonymous to synonymous rate ratios ($d_N/d_S$) reveal whether genes are under purifying or positive selection across species.
