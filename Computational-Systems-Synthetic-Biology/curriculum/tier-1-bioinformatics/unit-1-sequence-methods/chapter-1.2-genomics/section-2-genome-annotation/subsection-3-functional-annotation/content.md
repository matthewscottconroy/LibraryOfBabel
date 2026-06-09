# Functional Annotation

You have annotated the genes. The MAKER pipeline has run, the GFF3 file has been generated, and you can tell the world that your newly sequenced organism has 22,847 predicted protein-coding genes. This is real progress. But consider what you have actually produced: a list of genomic coordinates. You know where the genes are. You do not yet know what they do.

Functional annotation is the step that converts coordinates into biology. It is the process of asking, for each predicted gene product: what molecular function does this protein perform? What biological process does it participate in? What cellular compartment does it inhabit? The answers come primarily from sequence comparison — from the evolutionary fact that function is often conserved, so a protein with recognizable similarity to a protein of known function probably shares that function. But the caveats to this principle are significant, and the gap between "annotated" and "understood" is something you should never let yourself forget.

**Functional annotation** assigns biological meaning to predicted genes — determining what molecular function each gene product performs, which biological processes it participates in, and what cellular component it localizes to. It transforms a list of genome coordinates and protein sequences into a biologically interpretable picture of an organism's capabilities.

## Homology-Based Function Assignment

The most common approach: if a query protein is homologous to a protein of known function, infer that it shares that function.

### BLAST Against Curated Databases

```bash
# Search against SwissProt (manually curated, high quality)
blastp -query proteins.faa \
       -db swissprot \
       -evalue 1e-5 \
       -outfmt "6 qseqid sseqid pident length evalue bitscore stitle" \
       -num_threads 16 \
       -out blast_swissprot.txt

# Search against NCBI nr (comprehensive but noisy)
diamond blastp -q proteins.faa \
    --db nr \
    -e 1e-5 \
    --outfmt 6 qseqid sseqid pident length evalue bitscore stitle \
    -o diamond_nr.txt \
    --threads 16
```

**Caveats of homology-based annotation**:
- Function is inferred, not experimentally verified
- Annotation errors propagate: a misannotated protein in the database leads to incorrect annotations downstream
- Distant homologs (< 30% identity) may have evolved different functions
- "Conserved protein of unknown function" is a legitimate and honest annotation

The error propagation problem is more serious than it might appear. When a protein is misannotated in SwissProt as "putative methyltransferase," that annotation propagates to every new genome that uses it as a reference. The propagated annotation then serves as evidence for the next annotation, amplifying the original error across thousands of species. Estimates suggest that a significant fraction of the "putative function" annotations in public databases are ultimately traceable to a single incorrect original annotation — a cautionary reminder that database entries are hypotheses, not facts.

### InterProScan: Domain-Based Annotation

**InterPro** integrates protein family databases — Pfam, PANTHER, PRINTS, TIGRFAM, CDD, HAMAP, ProSite — into a unified annotation system. InterProScan searches a query protein against all these databases simultaneously:

```bash
interproscan.sh \
    -i proteins.faa \
    -f GFF3,TSV \
    -o interpro_results \
    -appl Pfam,PANTHER,TIGRFAM \
    --goterms \
    --pathways \
    --cpu 16
```

Output includes:
- Matched protein families and domains (with InterPro IDs like IPR000001)
- GO term annotations inferred from domain matches
- KEGG and Reactome pathway associations

Domain-based annotation has a crucial advantage over whole-protein BLAST: it works at the level of functional modules. A multi-domain protein may have a novel N-terminal domain but a well-characterized kinase domain in the C-terminus. Whole-protein similarity to the database will be low, potentially below the BLAST e-value threshold. But InterProScan will still identify the kinase domain and assign it the correct functional annotation. Protein domains are the units of functional evolution, and InterPro's domain-level analysis captures functional similarity that sequence-level comparison misses.

## Gene Ontology (GO) Annotation

The **Gene Ontology** provides a controlled, hierarchical vocabulary for describing gene function in three domains:

| Ontology | Describes | Example |
|---------|-----------|---------|
| Molecular Function | What the gene product does | "ATP binding", "serine-type endopeptidase activity" |
| Biological Process | The larger process it contributes to | "DNA repair", "immune response" |
| Cellular Component | Where in the cell it functions | "nucleus", "plasma membrane" |

GO terms form a directed acyclic graph (DAG): a child term (specific) implies all its parent terms (general). If a protein is annotated as "serine-type endopeptidase activity" (GO:0004252), it is also implicitly annotated as "peptidase activity" and "hydrolase activity".

**GO enrichment analysis** (after identifying a gene set of interest):

```python
from goatools.obo_parser import GODag
from goatools.go_enrichment import GOEnrichmentStudy

godag = GODag("go-basic.obo")
study_genes = set(["BRCA1", "TP53", "ATM", "CHEK2", "RAD51"])
background_genes = set(all_genome_genes)

goeaobj = GOEnrichmentStudy(
    background_genes,
    gene2go,  # dict: gene_id -> set of GO IDs
    godag,
    propagate_counts=True,
    alpha=0.05,
    methods=['fdr_bh']
)
results = goeaobj.run_study(study_genes)
```

```r
# In R with clusterProfiler
library(clusterProfiler)
library(org.Hs.eg.db)

go_results <- enrichGO(gene = gene_list,
                        OrgDb = org.Hs.eg.db,
                        keyType = "SYMBOL",
                        ont = "BP",  # Biological Process
                        pvalueCutoff = 0.05,
                        qvalueCutoff = 0.2)
dotplot(go_results)
```

GO enrichment analysis is one of the most commonly used tools in genomics and one of the most commonly misinterpreted. When you ask whether a set of differentially expressed genes is enriched for "DNA repair," you are asking a statistical question about whether that GO term appears in your gene set more often than expected by chance. The answer depends critically on the background set you use, the multiple testing correction you apply, and whether the GO annotations for your organism are complete. A GO enrichment result that says "DNA repair: p = 0.002" is only meaningful if your gene set is large enough, your background is appropriate, and the GO annotations are reliable. These caveats do not make the analysis useless — they make it something you interpret carefully rather than accept uncritically.

## KEGG Pathway Mapping

**KEGG (Kyoto Encyclopedia of Genes and Genomes)** provides metabolic pathway maps and functional modules. Mapping genes to KEGG identifies which biochemical pathways are encoded in the genome:

```bash
# Using KAAS (KEGG Automatic Annotation Server) — web-based
# Or local annotation with DIAMOND + KEGG database

# Map to KEGG PATHWAY using clusterProfiler (R)
```

```r
library(clusterProfiler)
kegg_results <- enrichKEGG(gene = entrez_ids,
                            organism = 'hsa',  # Homo sapiens
                            pvalueCutoff = 0.05)
barplot(kegg_results, showCategory=20)
```

## COG (Clusters of Orthologous Groups)

**COG** assigns genes to functional categories based on ortholog clusters:

| COG category | Function |
|---|---|
| J | Translation, ribosomal structure |
| K | Transcription |
| L | Replication, recombination and repair |
| O | Chaperones, protein folding |
| C | Energy production |
| E | Amino acid transport and metabolism |
| G | Carbohydrate transport and metabolism |
| S | Function unknown |

COG is particularly useful for microbial genomes and for identifying the functional content of metagenomes.

The "Function unknown" category — COG class S — is worth dwelling on. In a typical newly sequenced bacterial genome, 20–40% of genes fall into this category. This fraction has been remarkably resistant to reduction over decades of genomics research: as we annotate more genomes, we discover more novel genes, keeping the "unknown function" fraction stubbornly high. Category S is not a gap to be embarrassed about — it is a reminder that even in the best-studied organisms, a large fraction of the encoded biology remains mechanistically opaque.

## Repeat Annotation

A significant fraction of eukaryotic genomes consists of transposable elements and other repeats that are not protein-coding genes but require annotation:

```bash
# RepeatMasker: identify and mask repeats
RepeatMasker genome.fa \
    -species "Homo sapiens" \
    -pa 16 \
    -xsmall  # soft mask (lowercase)

# RepeatModeler: build a de novo repeat library for non-model organisms
RepeatModeler -database genome_db -pa 16
RepeatMasker -lib consensi.fa.classified genome.fa
```

Repeat annotation matters more than it might seem. Over 45% of the human genome consists of transposable element-derived sequences — Alu elements, LINE-1 repeats, endogenous retroviruses. If you do not mask these before running ab initio gene prediction, the predictor will try to find gene structure in repetitive sequence and produce thousands of spurious predictions. Masking is not optional preprocessing; it is an essential component of any annotation pipeline for eukaryotic genomes.

## Annotation Completeness Assessment

```bash
# BUSCO protein mode: how many universal single-copy orthologs are annotated?
busco -i proteome.faa -l vertebrata_odb10 -m protein -o busco_out -c 16

# Expected for vertebrate: > 95% complete BUSCOs
# C:98.5%[S:97.1%,D:1.4%],F:0.8%,M:0.7%,n:3354
```

## Why This Matters

Functional annotation is what converts a genome sequence from a list of DNA into biological knowledge. Without it, sequence data is a cipher — we know the letters but not what they mean. The quality of functional annotations determines the value of any genome as a research resource. Poor functional annotations (incorrectly propagated errors, vague "putative" assignments, missing genes) limit the ability to generate and test mechanistic hypotheses. For synthetic biology applications, accurate functional annotation is essential for identifying which pathways are present in a chassis organism, which genes to express or knock out, and what metabolic capabilities the system possesses.

The synthetic biology angle here is worth emphasizing. If you want to engineer an organism to produce a specific compound, you need to know which enzymes in that organism already catalyze relevant reactions, which pathway genes need to be introduced, and which competing pathways might divert your substrate. All of this depends on functional annotation. A poorly annotated chassis genome is a poorly characterized engineering substrate — you are building circuits in the dark. Every improvement in functional annotation methodology therefore has direct practical consequences for the feasibility of metabolic engineering projects.
