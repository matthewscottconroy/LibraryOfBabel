# Post-Translational Modification Proteomics

Consider the humble p53 protein. The gene encodes a single 393-amino acid sequence, identical in every cell of your body. Yet p53 can be a dormant tumor suppressor that sits quietly in unstressed cells, an activated transcription factor that drives apoptosis after DNA damage, a transcriptionally inert protein sequestered in the cytoplasm, or a dominant-negative mutant that promotes rather than suppresses cancer — depending on its post-translational status. More than 50 distinct PTM sites have been mapped on p53, including phosphorylation, acetylation, ubiquitination, sumoylation, methylation, and neddylation. The combination of modifications present at any given moment determines which of these functional states p53 occupies.

This is the world that PTM proteomics reveals. Proteins are not static entities: they are dynamically modified after translation by covalent chemical changes that alter their activity, localization, stability, and interactions. **Post-translational modifications (PTMs)** expand the functional diversity of the proteome far beyond what is encoded in the genome. Mass spectrometry is uniquely suited to characterize PTMs because each modification adds or removes a defined mass detectable at peptide and residue level.

## Phosphoproteomics

**Phosphorylation** — addition of a phosphate group to serine (pSer, +79.966 Da), threonine (pThr, +79.966 Da), or tyrosine (pTyr, +79.966 Da) — is the most studied PTM, regulating virtually every signaling pathway. The challenge: phosphopeptides are substoichiometric (often 1–10% of a given protein is phosphorylated under basal conditions) and suppress poorly in the presence of abundant non-phosphopeptides.

**Enrichment methods**:
- **TiO₂** (titanium dioxide): Phosphopeptides bind selectively to the Lewis acid Ti⁴⁺ surface at low pH. Eluted with base. High capacity, slight bias toward multiply phosphorylated peptides.
- **IMAC** (Immobilized Metal Affinity Chromatography): Fe³⁺- or Zr⁴⁺-loaded resins bind phosphopeptides. High specificity for monophosphopeptides.
- **SIMAC** (Sequential IMAC and TiO₂): Two-step enrichment for comprehensive coverage.

After enrichment, phosphopeptides are analyzed by LC-MS/MS. ETD fragmentation is preferred for long or highly phosphorylated peptides because it preserves labile phosphate groups better than CID/HCD.

**Site localization**: Identifying exactly which residue carries the phosphate requires observing phosphorylated b or y ions. **PhosphoRS** and **PTM-Shepherd** assign a probability score to each candidate phosphorylation site based on the evidence from observed fragment ions. Only sites with PhosphoRS probability ≥ 0.75 are reported with confidence.

The importance of site localization cannot be overstated. It is not enough to know that a peptide is phosphorylated — you need to know which serine, threonine, or tyrosine carries the phosphate, because different sites on the same protein can have completely opposite functional effects. On the MAP kinase ERK2, phosphorylation of T185 and Y187 activates the kinase, while phosphorylation of T188 inhibits it. An experiment that reports "ERK2 phosphorylation increased" without specifying the site is, at best, incomplete information.

## Ubiquitinomics: GlyGly Remnant Proteomics

**Ubiquitination** (addition of ubiquitin, a 76 amino acid protein, to lysine residues) is a signaling modification that marks proteins for proteasomal degradation, alters subcellular localization, or modulates activity. After trypsin digestion of ubiquitinated proteins, the trypsin cleaves within ubiquitin, leaving a characteristic **diglycine (GlyGly) remnant** (+114.043 Da) on the modified lysine. This GlyGly tag is detectable by MS and shifts the mass of the modified lysine by 114 Da.

**K-GG proteomics**: Anti-GlyGly antibodies (Cell Signaling Technology #5562) immunoprecipitate GlyGly-modified peptides from cell lysates, enabling proteome-wide ubiquitination site mapping. Thousands of ubiquitination sites can be profiled in a single experiment, revealing substrate changes upon proteasome inhibition or deubiquitinase perturbation.

The GlyGly remnant approach was a beautiful accident of enzyme specificity: trypsin cleaves ubiquitin's C-terminal sequence (-LRGG) after the R, leaving the two glycine residues covalently attached to the target lysine. This two-amino-acid stub is short enough to not prevent LC-MS analysis, but distinctive enough in mass to be unmistakably identified. Proteome-scale ubiquitinome profiling using anti-GlyGly antibodies has revealed that ubiquitination is far more widespread than previously appreciated — tens of thousands of sites on thousands of proteins — and has become a central tool for studying the ubiquitin-proteasome system in disease, particularly cancer and neurodegeneration.

## Acetylomics

**Lysine acetylation** (+42.011 Da) is catalyzed by acetyltransferases (HATs, KATs) and reversed by deacetylases (HDACs, sirtuins). It regulates histone function (histone acetylation = active chromatin), metabolic enzymes, and mitochondrial proteins. Pan-acetyllysine antibodies enable proteome-wide acetylation profiling analogously to GlyGly proteomics.

**N-terminal acetylation** (on the α-amine of the first residue) is carried out co-translationally by NatA/NatB/NatC complexes and affects protein stability and interactions. NTA is detected by observing the +42 Da modification on the protein N-terminus in MS data.

## Glycoproteomics

**Glycosylation** is the most structurally complex PTM: chains of sugars (glycans) are attached to proteins at asparagine (N-glycosylation) or serine/threonine (O-glycosylation). Glycans are not encoded in the genome and show immense heterogeneity — a single N-glycosylation site can carry hundreds of different glycan structures.

Analysis requires specialized workflows:
- **N-glycan release with PNGaseF** (converts N-Asn to D-Asp, +0.984 Da mass shift) followed by glycan MS or glycopeptide analysis with intact glycan.
- **O-glycan analysis**: No efficient enzymatic release; requires chemical methods (β-elimination) or direct glycopeptide MS.
- **HexNAc-based data analysis**: Tools like Byonic and GlycoWorkbench handle the large glycan search space.

Glycoproteomics is perhaps the most technically demanding area of PTM proteomics, precisely because glycans are so heterogeneous. A therapeutic antibody like rituximab has a single N-glycosylation site on its Fc region, but that site can carry dozens of different glycan compositions, each affecting the antibody's effector function differently. Characterizing this glycan heterogeneity — and how it changes during cell culture production — is a critical quality attribute in biopharmaceutical manufacturing. Glycoproteomics is therefore simultaneously one of the hardest problems in academic research and one of the most commercially important.

## Global Phosphoproteomics for Kinase Substrate Mapping

Quantitative phosphoproteomics under kinase perturbation (inhibitor treatment, kinase knockout, or overexpression) reveals kinase substrates: phosphorylation sites that change specifically when a kinase is modulated. **Kinase-substrate enrichment analysis (KSEA)** and **PhosphoSitePlus** annotation databases connect changing phosphosites to their upstream kinases, enabling inference of kinase activity changes from substrate phosphorylation patterns. This approach has identified novel substrates for oncogenic kinases (EGFR, BRAF, CDK4) with therapeutic implications.

The logic of KSEA is elegant: rather than trying to measure kinase activity directly (which is technically demanding and requires immunoprecipitation), you measure the phosphorylation state of hundreds of known substrates of that kinase and infer activity from the collective behavior. If the substrates of EGFR are all hyperphosphorylated in your tumor samples, EGFR is probably active — even if EGFR protein levels are unchanged. This enables kinase activity profiling from standard phosphoproteomic data without any additional experiments, effectively transforming a phosphoproteomics dataset into a kinome activity map.

## Why This Matters

PTM proteomics reveals the regulatory layer of the proteome that cannot be inferred from genomics or standard expression proteomics — phosphorylation patterns encode active signaling states, ubiquitination marks proteins for destruction, and glycosylation controls cell-surface interactions — making PTM analysis essential for understanding cell signaling, drug response, and disease mechanisms. When a cancer drug fails, it is rarely because the target protein has disappeared from the cell; it is far more often because the signaling network has rewired itself through changes in PTMs, enabling bypass of the block. PTM proteomics is the tool that makes those rewiring events visible — and that transforms a static protein expression catalog into a dynamic portrait of cellular decision-making.
