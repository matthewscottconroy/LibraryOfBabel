# Metabolite Annotation

There is a joke in metabolomics that the easy part is collecting the data, and the hard part is figuring out what you measured. After a well-executed untargeted LC-MS experiment, you might have 20,000 features — unique combinations of m/z and retention time that were reproducibly detected across your samples. Each feature represents a molecule that exists in your biological system. But the molecule has not introduced itself. You only know its approximate mass and when it eluted from your LC column. Connecting that mass and retention time to a chemical identity — a structural formula, a name, a place in metabolic pathways — is the annotation problem, and it is arguably the central intellectual challenge of untargeted metabolomics.

Detecting tens of thousands of mass spectrometry features is only the beginning — the challenge that defines untargeted metabolomics is **annotation**: assigning a chemical identity to each detected feature. This is substantially harder than peptide identification in proteomics, because the chemical space of metabolites is far larger and less constrained than the space of tryptic peptides from a known genome.

## The Annotation Confidence Ladder (MSI Levels)

The Metabolomics Standards Initiative (MSI) defined a hierarchy of annotation confidence levels that is now the standard for reporting metabolomics results:

**Level 1 — Identified**: The compound is confirmed by comparison to an authentic reference standard analyzed on the same instrument under identical conditions. Both the accurate mass AND the MS/MS spectrum AND the chromatographic retention time match the standard. This is the gold standard and is required for quantitative targeted studies.

**Level 2 — Putatively Annotated**: No reference standard is available, but the compound is annotated based on spectral similarity to a library entry (MS/MS spectrum match ≥ 0.7 cosine similarity to HMDB, MassBank, or GNPS) and accurate mass match. Retention time is not confirmed. Most untargeted metabolomics annotations fall in this category.

**Level 3 — Putatively Characterized**: The compound class can be identified (e.g., "an unsaturated phosphatidylcholine with 36 carbons"), but the exact structure and isomer cannot be determined. Based on class-specific fragmentation rules.

**Level 4 — Unknown**: The feature is reproducibly detected and shows biological variation, but no structural information can be assigned. It may be a known compound not in the reference database, or a genuinely novel metabolite.

The MSI levels are not just a classification scheme — they are a statement about what you can and cannot conclude from your data. A Level 2 annotation means you have a hypothesis about what the feature is, but you have not proven it. Treating Level 2 annotations as Level 1 identifications is one of the most common errors in metabolomics, and it has led to published claims about novel biomarkers that could not be reproduced when the original annotation turned out to be incorrect. Be honest in your reporting: if you did not buy the authentic standard and run it on your instrument, you have a candidate annotation, not an identification.

## Exact Mass Matching

The first step in annotation is matching the measured accurate mass to the theoretical mass of candidate compounds in a database (HMDB, LipidMaps, PubChem, KEGG).

Exact mass matching within **5 ppm** is the standard threshold for high-resolution instruments (Orbitrap, Q-TOF). At m/z = 500, 5 ppm corresponds to ±0.0025 Da — a very narrow window that substantially reduces the candidate list.

However, at any given mass there may still be multiple molecular formulae that match within 5 ppm. **Adduct prediction** is also required: the same molecule can form multiple adduct ions depending on the ionization mode and mobile phase:
- Positive mode: [M+H]⁺ (+1.0073 Da), [M+Na]⁺ (+22.9898 Da), [M+NH₄]⁺ (+18.0344 Da), [M+K]⁺ (+38.9637 Da)
- Negative mode: [M-H]⁻ (−1.0073 Da), [M+Cl]⁻ (+34.9694 Da), [M+COOH]⁻ (+44.9977 Da)

Software tools (**CAMERA** in R, **mzMine**'s adduct detection module) group features by adduct mass differences to identify different ionic forms of the same molecule.

The adduct problem is a genuine source of inflation in metabolomics feature counts. When you see 20,000 features, it does not mean 20,000 distinct molecules — one molecule can appear as [M+H]⁺, [M+Na]⁺, [M+K]⁺, and [2M+H]⁺, producing four features from a single compound. Without careful adduct detection and grouping, you will overestimate metabolome diversity and count the same biological change multiple times in your statistical analysis.

## MS/MS Library Matching

MS/MS fragmentation spectra provide structural information that dramatically reduces the candidate list. Three major spectral databases are available:

**HMDB** (Human Metabolome Database): Contains MS/MS spectra for ~5,000 metabolites at multiple collision energies. Specifically curated for human biomedical metabolomics.

**MassBank**: Community-curated database with >70,000 spectra, including environmental and food metabolites beyond HMDB's scope.

**GNPS** (Global Natural Products Social Molecular Networking): Contains >1 million spectra contributed by thousands of groups; particularly strong for natural products, lipids, and microbial metabolites.

Spectral similarity is quantified by the **cosine similarity score**:

$$\cos(\theta) = \frac{\sum_{i} I_i^A \cdot I_i^B}{\sqrt{\sum_i (I_i^A)^2} \cdot \sqrt{\sum_i (I_i^B)^2}}$$

where $I_i^A$ and $I_i^B$ are matched fragment ion intensities in spectrum A (query) and B (reference). A cosine score ≥ 0.7 is typically considered a good spectral match.

## Molecular Networking with GNPS

**Molecular networking** (Wang et al., 2016) organizes MS/MS spectra by spectral similarity into a network where nodes are spectra and edges connect spectra with cosine similarity ≥ 0.7. Structurally related metabolites form clusters ("spectral families") — for example, all acylcarnitines with different chain lengths form one cluster.

This network visualization serves multiple purposes: it groups unknowns with related known compounds (enabling class-level annotation), identifies novel variants of known compound classes, and enables dereplication of known compounds from complex mixtures. GNPS molecular networking is widely used in natural products discovery and microbiome metabolomics.

The power of molecular networking is that it turns the annotation problem from "identify every unknown independently" into "identify a few knowns in each cluster, and propagate the annotation to the unknowns nearby." If you know that feature A is an acylcarnitine with a C16 chain, and feature B has a spectrum that differs from feature A only by 14.016 Da (a CH₂ group), then feature B is almost certainly a C17 acylcarnitine — a structural inference that mass accuracy alone could not provide.

## In Silico Fragmentation Tools

For features without library matches, in silico tools predict MS/MS spectra from candidate structures:

**MetFrag**: Takes a candidate molecular formula, retrieves all structures from databases, and scores each structure based on how well its predicted in silico MS/MS matches the observed spectrum. Rankings are combined with database sources and other metadata.

**SIRIUS + CSI:FingerID**: A two-step approach. SIRIUS first determines the molecular formula from the accurate precursor mass and the isotope pattern. CSI:FingerID then uses a machine learning model trained on tens of thousands of MS/MS spectra to predict the **molecular fingerprint** (a vector of structural features) and matches it to structures in a database. CANOPUS extends this to compound class prediction without requiring a database match.

These in silico tools have dramatically improved untargeted metabolomics annotation rates, though Level 1 confirmation (with a reference standard) remains required for definitive identification.

The SIRIUS/CSI:FingerID approach is a good example of how machine learning is transforming metabolomics annotation. The key insight is that MS/MS fragmentation patterns encode structural features in a predictable way — certain substructures produce characteristic fragment masses — and a deep learning model trained on millions of annotated spectra can learn to "read" these structural fingerprints from raw spectra, without needing an exact match in a reference library. CANOPUS, which extends this to compound class prediction, can at least tell you whether an unknown feature is a fatty acid, a flavonoid, or a bile acid, even if the exact structure cannot be determined — enabling biological interpretation at the pathway level even for unannotated features.

## Why This Matters

Metabolite annotation is the rate-limiting step in untargeted metabolomics — the gap between detecting a feature and knowing its chemical identity determines whether a biological discovery can be made or whether the experiment produces only a list of unidentifiable signals; advances in spectral databases, molecular networking, and machine learning annotation tools are steadily closing this gap. The annotation problem is also a community-wide challenge: every research group that acquires reference spectra for a novel compound and deposits them in GNPS or MassBank makes annotation easier for everyone else. The metabolomics community is collectively building a spectral atlas of the metabolome, one compound at a time, and the rate of discovery accelerates with every new deposit.
