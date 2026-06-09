# Cathedral III: A Multi-Omics Integration Analysis

---

## The Question

How does a cell integrate transcriptional, translational, and metabolic responses to a biological perturbation? What regulatory mechanisms underlie this coordination?

---

## Prerequisites

- [Tier 1.3](../curriculum/tier-1-bioinformatics/1.3-transcriptomics.md): RNA-seq analysis
- [Tier 1.4](../curriculum/tier-1-bioinformatics/1.4-proteomics-metabolomics.md): Proteomics and metabolomics
- [Tier 2.2](../curriculum/tier-2-systems-biology/2.2-metabolic-modeling.md): Metabolic modeling
- [Tier 2.3](../curriculum/tier-2-systems-biology/2.3-gene-regulatory-networks.md): GRN analysis
- [Tier 4.4](../curriculum/tier-4-computational-tools/4.4-network-analysis.md): Network analysis

---

## The Project

This cathedral can be done entirely computationally using publicly available datasets.

### Phase 1: Question and Dataset Selection

1. Choose a biological question with multi-omics data:
   - How does *E. coli* rewire metabolism during carbon source shift (glucose → acetate)?
   - How does a pathogen reprogram host cell transcriptome and metabolome during infection?
   - How do tumor cells differ from normal cells in transcriptome + metabolome?
   - How does antibiotic treatment induce resistance at the transcriptomic and metabolomic level?

2. Find datasets on public repositories:
   - GEO (Gene Expression Omnibus): RNA-seq data
   - PRIDE: proteomics data
   - MetaboLights: metabolomics data
   - Look for: same biological system, ideally same lab, same time points

3. Document: experimental conditions, time points, organism, tissue/cell type

### Phase 2: Individual Omics Processing

4. RNA-seq:
   - Download raw FASTQ from GEO (use SRA Toolkit)
   - Run: fastp → STAR → featureCounts → DESeq2
   - Output: differential gene expression table with LFC and padj

5. Proteomics:
   - Download: MaxQuant output (proteinGroups.txt) or raw MS files
   - If MaxQuant output provided: load into Perseus or Python; normalize LFQ intensities
   - If raw: run MaxQuant with standard parameters
   - Output: log2-fold change per protein across conditions

6. Metabolomics (if available):
   - Download: processed peak table or use XCMS to process raw mzML
   - Normalize by total signal; log-transform
   - HMDB/KEGG annotation for identified features

### Phase 3: Multi-Omics Correlation Analysis

7. Transcript-protein correlation:
   - For each detected gene: plot mRNA fold change vs. protein fold change
   - Pearson/Spearman correlation coefficient
   - Identify outliers: high mRNA change, low protein change (post-transcriptional regulation?)
   - Gene ontology enrichment for poorly correlated genes vs. well-correlated

8. Protein-metabolite correlation:
   - For metabolic enzymes: correlate enzyme abundance to metabolite abundance
   - High enzyme, low substrate → enzyme not rate-limiting?
   - Low enzyme, high substrate → enzyme is bottleneck

9. KEGG pathway enrichment across layers:
   - Which pathways are enriched in DE genes?
   - Are the same pathways enriched at the protein level?
   - Do corresponding metabolites change in those pathways?

### Phase 4: Network-Based Integration

10. Build a network for each omics layer:
    - GRN: from RNA-seq using GRNBoost2 or SCENIC
    - PPI: from STRING (physical interactions only)
    - Metabolic: from GEM restricted to detected enzymes

11. Find conserved functional modules:
    - Overlap protein network communities with differentially expressed genes
    - Identify modules active at both transcript and protein level

12. Key regulator identification:
    - Master regulators: transcription factors with many differentially expressed targets
    - Metabolic regulators: enzymes connecting multiple altered metabolic pathways
    - VIPER (Virtual Inference of Protein activity by Enriched Regulon analysis): infer TF activity from expression of targets

13. COSMOS or CARNIVAL analysis (optional advanced):
    - Network-based causal integration
    - Find shortest paths connecting known upstream perturbation to downstream metabolite changes through signaling and gene expression layers

### Phase 5: Biological Interpretation

14. Synthesize findings into a coherent biological narrative:
    - What is the primary regulatory strategy the cell uses?
    - Is the response transcription-driven or post-transcriptional?
    - What metabolic pathways are being upregulated, and does this make biological sense given the perturbation?

15. Generate 3-5 testable hypotheses:
    - "Transcription factor X is the master regulator; its deletion should abrogate the response"
    - "Enzyme Y is rate-limiting for metabolite Z accumulation; overexpression should alleviate the bottleneck"

16. Literature validation: do your findings agree with published mechanisms?

---

## Expected Output

- Complete analysis pipeline (Snakemake or documented Jupyter notebooks) in GitHub
- Integrated figure: multi-omics heatmap, network visualization, pathway enrichment
- Table of key findings at each omics layer
- Biological narrative synthesizing findings
- 3-5 testable hypotheses for future experiments

---

## Key Tools

- RNA-seq: STAR, DESeq2 (R)
- Proteomics: Perseus (GUI) or custom Python
- Multi-omics integration: MOFA+ (Python/R), mixOmics (R)
- Network: STRINGdb (R), NetworkX (Python), SCENIC (Python)
- Pathway: clusterProfiler (R), GSEA, KEGG Mapper
- Visualization: ComplexHeatmap (R), Cytoscape
