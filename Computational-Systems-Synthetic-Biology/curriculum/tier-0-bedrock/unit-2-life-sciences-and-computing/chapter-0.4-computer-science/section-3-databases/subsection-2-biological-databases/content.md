# Biological Databases

Before any algorithm runs, before any statistical model is fit, before any figure is drawn, there is a step that every bioinformatics analysis shares: querying a database. When you BLAST a sequence, you are querying GenBank. When you look up a variant's population frequency, you are querying gnomAD. When you annotate differentially expressed genes with pathway information, you are querying KEGG. When you ask whether a protein has a known structure, you are querying the PDB — or, increasingly, the AlphaFold database, which contains predicted structures for essentially every known protein.

Biological databases are the accumulated knowledge of molecular biology — millions of sequences, thousands of structures, hundreds of thousands of functional annotations. Every bioinformatics analysis begins by querying at least one. Knowing what each database contains, how to access it programmatically, and what its limitations are is prerequisite knowledge. This is not a directory listing — it is a guide to which databases to use for what, and how to access them at the command line and via API.

## Sequence Databases

### NCBI GenBank / RefSeq

**GenBank** is the comprehensive public database of all known nucleotide sequences, deposited by researchers worldwide. **RefSeq** is a curated, non-redundant set of reference sequences (one representative per gene per species).

- GenBank Accessions: nucleotide sequences (NM_, NR_), proteins (NP_), whole genome shotgun (WGS, JAAAA-ZZZZZ), whole genomes (NC_, NZ_)
- Access via **Entrez E-utilities** (NCBI's REST API):

```bash
# Fetch a RefSeq nucleotide sequence in FASTA format
curl "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=nucleotide&id=NM_007294&rettype=fasta&retmode=text"

# Search for all E. coli K-12 genome records
curl "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=genome&term=Escherichia+coli+K-12&retmode=json"
```

In Python via Biopython:
```python
from Bio import Entrez, SeqIO
Entrez.email = "your@email.com"

# Fetch BRCA1 protein sequence
handle = Entrez.efetch(db="protein", id="NP_009225", rettype="fasta", retmode="text")
record = SeqIO.read(handle, "fasta")
```

### UniProt / Swiss-Prot

**UniProt** is the reference protein sequence and functional annotation database. The distinction between its two halves is worth understanding: Swiss-Prot entries are manually reviewed — a human expert has read the literature, evaluated the evidence, and written the functional annotation. TrEMBL entries are computationally predicted and may be unreliable:

- **Swiss-Prot**: manually curated, ~570,000 entries, high-quality annotations
- **TrEMBL**: computationally annotated (many millions of entries, lower confidence)
- **UniRef**: clustered at 100%, 90%, or 50% identity for reduced redundancy

Access:
```bash
# REST API: get BRCA1 human protein in JSON
curl "https://rest.uniprot.org/uniprotkb/P38398.json"

# Download all human reviewed proteins in FASTA
curl "https://rest.uniprot.org/uniprotkb/stream?query=organism_id:9606+AND+reviewed:true&format=fasta" > human_swissprot.faa
```

### Ensembl and BioMart

**Ensembl** provides vertebrate genome sequences, gene annotations, comparative genomics, and variation data. The annotations are gene models built by the Ensembl annotation pipeline and GENCODE (for human and mouse).

**BioMart** is the batch query interface — retrieve large sets of data with complex filters:

```r
library(biomaRt)
mart <- useMart("ensembl", dataset = "hsapiens_gene_ensembl")

# Get all protein-coding genes on chr17 with their coordinates
genes_chr17 <- getBM(
    attributes = c("hgnc_symbol", "ensembl_gene_id",
                   "chromosome_name", "start_position", "end_position",
                   "gene_biotype"),
    filters = list(chromosome_name = "17", biotype = "protein_coding"),
    mart = mart
)
```

## Structure Databases

### PDB (Protein Data Bank)

The PDB houses all experimentally determined macromolecular structures: X-ray crystallography, NMR, cryo-EM, and neutron diffraction. Each entry has a 4-character ID (e.g., `1BNA` for the canonical B-DNA structure, `1HHO` for human hemoglobin).

Access:
```bash
# Download a PDB structure in mmCIF format (current standard)
curl "https://files.rcsb.org/download/6VXX.cif" > spike_protein.cif

# Or legacy PDB format
curl "https://files.rcsb.org/download/6VXX.pdb" > spike_protein.pdb
```

Key PDB statistics (2024): ~215,000 structures; ~90% from X-ray; ~7% from cryo-EM (rising rapidly); ~55,000 unique UniProt accessions represented.

### AlphaFold Database

DeepMind and EMBL-EBI released predicted structures for >200 million proteins (essentially all known protein sequences with pLDDT confidence scores). This has transformed structural biology — previously, only ~55,000 unique proteins had experimental structures; now any protein of interest likely has a predicted structure.

```bash
# Download AlphaFold prediction for human BRCA1
curl "https://alphafold.ebi.ac.uk/files/AF-P38398-F1-model_v4.pdb" > BRCA1_alphafold.pdb
```

The **pLDDT** (predicted local distance difference test) score per residue indicates model confidence: >90 = very high, 70–90 = confident, 50–70 = low confidence, <50 = disordered/unreliable. Critically, low pLDDT regions are often not failed predictions — they represent intrinsically disordered regions that genuinely lack stable structure. The pLDDT score is interpretable biological information, not merely a quality flag.

## Functional Annotation Databases

### Gene Ontology (GO)

The Gene Ontology provides a controlled vocabulary for gene function organized into three hierarchical ontologies:
- **Molecular Function (MF)**: the biochemical activity (e.g., ATP binding, kinase activity)
- **Biological Process (BP)**: the biological objective (e.g., DNA repair, cell cycle)
- **Cellular Component (CC)**: where in the cell (e.g., nucleus, plasma membrane)

GO terms are organized as a DAG (directed acyclic graph). Annotations are provided at different levels of evidence (EXP = experimental; IEA = inferred from electronic annotation; lowest confidence).

**GO enrichment analysis**: Given a set of differentially expressed genes, is any GO term statistically overrepresented? Uses hypergeometric or Fisher's exact test:

```r
library(clusterProfiler)
library(org.Hs.eg.db)

# Convert gene symbols to Entrez IDs
gene_list <- c("BRCA1", "TP53", "PTEN", "ATM")
gene_ids <- bitr(gene_list, fromType = "SYMBOL", toType = "ENTREZID", OrgDb = org.Hs.eg.db)

# GO enrichment
ego <- enrichGO(gene = gene_ids$ENTREZID,
                OrgDb = org.Hs.eg.db,
                ont = "BP",
                pAdjustMethod = "BH",
                pvalueCutoff = 0.05)
dotplot(ego)
```

### KEGG: Metabolic Pathways

**KEGG** (Kyoto Encyclopedia of Genes and Genomes) maps genes to metabolic pathways (KEGG PATHWAY), molecular interactions (KEGG BRITE), and chemical compounds (KEGG COMPOUND/REACTION). The pathway maps are manually curated and represent the gold standard for metabolic knowledge.

```bash
# KEGG REST API: get glycolysis pathway for E. coli
curl "https://rest.kegg.jp/get/eco00010" | head -50

# Get all genes in the E. coli glycolysis pathway
curl "https://rest.kegg.jp/link/eco/path:eco00010"
```

For pathway visualization and enrichment in R:
```r
library(clusterProfiler)
kk <- enrichKEGG(gene = gene_ids$ENTREZID, organism = "hsa", pvalueCutoff = 0.05)
```

## Variant Databases

| Database | Content | Access |
|---|---|---|
| **dbSNP** | All known SNPs and small indels | NCBI E-utilities, FTP |
| **ClinVar** | Clinically interpreted variants | NCBI FTP, API |
| **gnomAD** | Population allele frequencies (~730K exomes + genomes) | gnomad.broadinstitute.org, BigQuery |
| **COSMIC** | Somatic mutations in cancer | COSMIC FTP (registration required) |
| **OMIM** | Mendelian disease gene-phenotype relationships | API (registration required) |

```bash
# Query gnomAD for a specific variant via GraphQL API
curl -X POST "https://gnomad.broadinstitute.org/api" \
  -H "Content-Type: application/json" \
  -d '{"query": "{ variant(dataset: gnomad_r4, variantId: \"17-43092919-A-G\") { exome { ac an af } } }"}'
```

## Why This Matters for Computational Biology

Biological databases are the substrate of all bioinformatics. Every BLAST search, every GO enrichment, every pathway analysis, every variant annotation queries at least one of these databases. Knowing their APIs means you can automate data retrieval at scale — downloading 10,000 protein sequences from UniProt, fetching all clinical variants for a gene list from ClinVar, or building a custom annotation database that combines information from multiple sources. BioMart and KEGG REST are used in virtually every genomics workflow. gnomAD population frequencies are the key reference for clinical variant interpretation (is this variant too common to be pathogenic?). Understanding database architectures — what is curated vs. computationally predicted, what the update cadence is, when to use RefSeq vs. Ensembl, why Swiss-Prot and TrEMBL have different reliability — prevents major errors in downstream analysis.
