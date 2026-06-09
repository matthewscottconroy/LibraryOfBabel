# Annotation File Formats

There is a well-established pattern in computational biology: a powerful tool is developed, it produces output in some format that made sense to its authors, other tools adopt that format, and within a few years there is an ecosystem of dozens of tools all using slightly incompatible variants of the same format. Genomic annotation is no exception. You will encounter three primary formats — GFF3, GTF, and GenBank — each with its own coordinate system, its own way of encoding gene hierarchies, and its own pitfalls for the unwary.

Understanding these formats is not optional bookkeeping. Every analysis that uses gene coordinates — variant effect prediction, differential expression, pathway analysis, synteny analysis — depends on correctly formatted and correctly interpreted annotation files. The GFF3/GTF coordinate system (1-based, closed) differs from BED (0-based, half-open), and this difference causes systematic off-by-one errors when mixing formats. These are bugs that do not crash your pipeline; they silently produce wrong results. Learning the formats prevents you from making those errors and, equally importantly, lets you recognize them when you encounter them in published analyses.

Genome annotations are stored and exchanged in standardized file formats that encode feature hierarchies, coordinates, and biological attributes. Three formats dominate: GFF3 for universal annotation exchange, GTF for RNA-seq tool compatibility, and GenBank flat file format for rich, integrated sequence + annotation records.

## GFF3 for Hierarchical Annotation

GFF3 (described in detail in the file formats section) is the standard for genome annotation exchange. Its key feature for annotation is the `ID`/`Parent` attribute system that encodes the gene → mRNA → exon/CDS hierarchy.

**Complete example of a gene with two isoforms**:

```
##gff-version 3
##sequence-region chr1 1 248956422

chr1  AUGUSTUS  gene        10000  20500  .  +  .  ID=gene001;Name=BRCA1;biotype=protein_coding
chr1  AUGUSTUS  mRNA        10000  20500  .  +  .  ID=mRNA001;Parent=gene001;Name=BRCA1-201
chr1  AUGUSTUS  exon        10000  10200  .  +  .  Parent=mRNA001
chr1  AUGUSTUS  CDS         10050  10200  .  +  0  Parent=mRNA001
chr1  AUGUSTUS  exon        15000  15500  .  +  .  Parent=mRNA001
chr1  AUGUSTUS  CDS         15000  15500  .  +  1  Parent=mRNA001
chr1  AUGUSTUS  exon        20000  20500  .  +  .  Parent=mRNA001
chr1  AUGUSTUS  CDS         20000  20450  .  +  0  Parent=mRNA001
chr1  AUGUSTUS  stop_codon  20448  20450  .  +  0  Parent=mRNA001
chr1  AUGUSTUS  mRNA        10000  20500  .  +  .  ID=mRNA002;Parent=gene001;Name=BRCA1-202
chr1  AUGUSTUS  exon        10000  10200  .  +  .  Parent=mRNA002
chr1  AUGUSTUS  CDS         10050  10200  .  +  0  Parent=mRNA002
chr1  AUGUSTUS  exon        12000  12300  .  +  .  Parent=mRNA002  # additional exon in isoform 2
chr1  AUGUSTUS  CDS         12000  12300  .  +  2  Parent=mRNA002
chr1  AUGUSTUS  exon        20000  20500  .  +  .  Parent=mRNA002
chr1  AUGUSTUS  CDS         20000  20450  .  +  0  Parent=mRNA002
```

**CDS phase (column 8)**: the reading frame. 0 = the first base of the CDS feature is the first base of a codon; 1 = the second base; 2 = the third base.

The phase column is easy to overlook but critical for any analysis that extracts protein sequence from genomic coordinates. If you ignore the phase and assume every CDS feature starts at the first position of a codon, you will produce incorrect protein translations for every internal exon that starts mid-codon — which is most of them. The GFF3 specification requires correct phase values precisely because tools downstream depend on them to reconstruct the protein sequence from genome + annotation.

## GTF for RNA-seq Pipelines

GTF stores annotation as gene_id/transcript_id key-value pairs. The Ensembl GTF format is the standard input for STAR, featureCounts, and HTSeq:

```
chr1  ensembl_havana  gene        10000  20500  .  +  .  gene_id "ENSG00000139618"; gene_name "BRCA1"; gene_biotype "protein_coding";
chr1  ensembl_havana  transcript  10000  20500  .  +  .  gene_id "ENSG00000139618"; transcript_id "ENST00000357654"; transcript_name "BRCA1-201";
chr1  ensembl_havana  exon        10000  10200  .  +  .  gene_id "ENSG00000139618"; transcript_id "ENST00000357654"; exon_number "1";
chr1  ensembl_havana  CDS         10050  10200  .  +  0  gene_id "ENSG00000139618"; transcript_id "ENST00000357654";
chr1  ensembl_havana  start_codon 10050  10052  .  +  0  gene_id "ENSG00000139618"; transcript_id "ENST00000357654";
```

**Downloading Ensembl GTF**:
```bash
# Download human GTF from Ensembl
wget https://ftp.ensembl.org/pub/release-111/gtf/homo_sapiens/Homo_sapiens.GRCh38.111.gtf.gz

# Uncompress and index
gunzip Homo_sapiens.GRCh38.111.gtf.gz
```

GTF is simpler than GFF3 — it does not use a formal Parent hierarchy, relying instead on matching gene_id and transcript_id strings to associate features. This makes it somewhat easier to parse with simple text processing, but it loses the explicit tree structure that GFF3 encodes. For most RNA-seq workflows, you will use GTF because STAR, HISAT2, featureCounts, and HTSeq all expect it. For custom annotation work, particularly with complex isoform structures or novel feature types, GFF3 is the better choice.

## GenBank Flat File Format

GenBank format stores sequence + annotation in an integrated record. It is used for submission to NCBI and retrieval from GenBank:

```
LOCUS       NM_007294               7088 bp    mRNA    linear   PRI 15-JAN-2024
DEFINITION  Homo sapiens BRCA1 DNA repair associated (BRCA1), transcript
            variant 1, mRNA.
ACCESSION   NM_007294
FEATURES             Location/Qualifiers
     source          1..7088
                     /organism="Homo sapiens"
                     /mol_type="mRNA"
     gene            1..7088
                     /gene="BRCA1"
                     /db_xref="GeneID:672"
     CDS             join(141..222,403..559,4986..5193,...)
                     /gene="BRCA1"
                     /product="breast cancer type 1 susceptibility protein"
                     /protein_id="NP_009225.1"
                     /db_xref="GeneID:672"
ORIGIN
        1 ggagttgggg attaagaacc cagcagagtc agaatgtcag gctgttcaga aatcagcaat
       61 gcagccaact ttaaggagcc tgagcctgtg tcccctgtag gtcagtgagc tcagcagccc
```

Key features:
- `FEATURES` section: annotations using GenBank join() syntax for multi-exon features
- `ORIGIN` section: the sequence itself
- `CDS join(...)`: specifies exon positions as a comma-separated list

**Parsing GenBank with BioPython**:

```python
from Bio import SeqIO

# Parse GenBank record
record = SeqIO.read("BRCA1.gb", "genbank")
print(f"Accession: {record.id}")
print(f"Sequence length: {len(record.seq)}")

for feature in record.features:
    if feature.type == "CDS":
        print(f"CDS: {feature.location}")
        print(f"Product: {feature.qualifiers.get('product', ['Unknown'])[0]}")
        protein_seq = feature.translate(record.seq)
```

The join() notation in GenBank format is elegant in concept but can be surprising in practice. The CDS for BRCA1 spans 24 exons, and the join() list contains 24 coordinate pairs. When you see `join(141..222,403..559,...)` you are reading the entire exonic structure of the gene in a single line of text. BioPython handles the join() notation transparently — `feature.location` returns a CompoundLocation object that correctly concatenates the specified intervals and applies strand.

## Programmatic Access to Annotations

### gffutils: GFF3/GTF Database

```python
import gffutils

# Create SQLite database from GFF3
db = gffutils.create_db('annotation.gff3', dbfn='annotation.db',
                          force=True, keep_order=True,
                          merge_strategy='merge', sort_attribute_values=True)

# Query genes on chromosome 1
for gene in db.features_of_type('gene', seqid='chr1', start=1, end=1000000):
    print(f"{gene.id}: {gene.start}-{gene.end}, {gene.strand}")
    # Get children (mRNAs)
    for mrna in db.children(gene, featuretype='mRNA'):
        exons = list(db.children(mrna, featuretype='exon'))
        print(f"  Transcript {mrna.id}: {len(exons)} exons")

# Get sequence for a feature
import pyfaidx
genome = pyfaidx.Fasta('genome.fa')
for exon in db.children('mRNA001', featuretype='exon', order_by='start'):
    seq = genome[exon.seqid][exon.start-1:exon.end].seq
    print(f"Exon {exon.start}-{exon.end}: {seq[:20]}...")
```

### pyranges: Fast Interval Operations

```python
import pyranges as pr

# Load annotation as PyRanges object
genes = pr.read_gtf('annotation.gtf')

# Filter for protein-coding genes
coding_genes = genes[genes.Feature == 'gene']
coding_genes = coding_genes[coding_genes.gene_biotype == 'protein_coding']

# Intersect with variants
variants = pr.read_vcf('variants.vcf')
overlapping = coding_genes.join(variants)
```

Pyranges is worth highlighting for anyone doing interval operations at scale. Finding which variants overlap which exons, which peaks fall in promoter regions, which genes are within 10 kb of a structural variant breakpoint — all of these are interval problems. The naive approach (nested for loops over genomic coordinates) is catastrophically slow for genome-scale data. Pyranges uses sorted interval trees under the hood, making these operations fast enough to run interactively on whole-genome datasets.

## Why This Matters

Annotation file formats are the connective tissue of genomics. Every analysis that uses gene coordinates — variant effect prediction, differential expression, pathway analysis, synteny analysis — depends on correctly formatted and correctly interpreted annotation files. The GFF3/GTF coordinate system (1-based, closed) differs from BED (0-based, half-open), and this difference causes systematic off-by-one errors when mixing formats. GenBank format's CDS join() notation is the standard for sequence database submissions and must be understood for database depositions and retrievals. Mastering gffutils, pybedtools, and pyranges for programmatic annotation manipulation is essential for any genomics analysis requiring custom feature extraction.

The off-by-one problem deserves one final emphasis. GFF3 and GTF use 1-based, fully-closed coordinates: position 1 is the first base of the chromosome, and the interval [10000, 10200] includes both base 10000 and base 10200. BED format uses 0-based, half-open coordinates: the interval [9999, 10200) starts at position 9999 in 0-based coordinates and excludes position 10200. These conventions are incompatible, and converting between them requires adding or subtracting 1 from the start coordinate. This is trivially easy to get wrong, and the error is invisible — your analysis runs without error messages, your results are plausible, and the variants are mapped to the wrong exons. Check your coordinate conventions before every cross-format operation.
