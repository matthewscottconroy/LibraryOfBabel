# VCF Format

**VCF (Variant Call Format)** is the standard for storing and exchanging genetic variant data. Every SNP, indel, structural variant, and copy number variant discovered by sequencing is recorded in VCF. The format encodes the type, position, quality, and genotype of variants, enabling downstream analyses from population genetics to clinical interpretation.

Consider the scale of the problem VCF was designed to handle. The 1000 Genomes Project characterized variants across 2,504 individuals representing 26 populations worldwide, identifying ~84 million SNPs and ~3.6 million indels. Every one of those variants is a row in a VCF file. The gnomAD database of population-level variation contains hundreds of millions of variants across hundreds of thousands of individuals. Clinical sequencing labs use VCF to communicate variant discoveries to oncologists and geneticists. Understanding VCF is not just a technical skill — it is essential for connecting sequencing experiments to the population genetics, evolutionary, and clinical contexts that give them meaning.

## File Structure

A VCF file has two sections: **meta-information lines** (beginning with `##`) and **data lines**.

### Meta-information Lines

```
##fileformat=VCFv4.3
##FILTER=<ID=PASS,Description="All filters passed">
##FILTER=<ID=LowQual,Description="Low quality">
##INFO=<ID=AF,Number=A,Type=Float,Description="Allele Frequency">
##INFO=<ID=DP,Number=1,Type=Integer,Description="Total Depth">
##FORMAT=<ID=GT,Number=1,Type=String,Description="Genotype">
##FORMAT=<ID=AD,Number=R,Type=Integer,Description="Allelic depths">
##FORMAT=<ID=GQ,Number=1,Type=Integer,Description="Genotype Quality">
##contig=<ID=chr1,length=248956422>
##reference=GRCh38
```

### Data Lines

Eight mandatory tab-separated columns, followed by optional FORMAT and sample columns:

| Col | Name | Example | Description |
|-----|------|---------|-------------|
| 1 | CHROM | `chr1` | Chromosome |
| 2 | POS | `925952` | 1-based position of variant |
| 3 | ID | `rs699` | Variant ID (dbSNP rsID or `.`) |
| 4 | REF | `A` | Reference allele at POS |
| 5 | ALT | `G` | Alternate allele(s); comma-separated for multiple |
| 6 | QUAL | `3200.25` | Phred quality of variant call |
| 7 | FILTER | `PASS` | `PASS` or semicolon-separated filter names |
| 8 | INFO | `AF=0.42;DP=150` | Semicolon-separated key-value annotations |
| 9 | FORMAT | `GT:AD:GQ` | Colon-separated format keys for genotype columns |
| 10+ | Samples | `0/1:25,30:99` | Per-sample genotype data |

### Example VCF Lines

```
#CHROM  POS     ID       REF  ALT  QUAL    FILTER  INFO              FORMAT  Sample1       Sample2
chr1    925952  rs699    A    G    3200.25 PASS    AF=0.42;DP=150    GT:AD:GQ  0/1:35,40:99  0/0:80,0:99
chr1    1234567 .        ACGT A    150.30  PASS    DP=45             GT:AD:GQ  1/1:0,45:99   0/1:22,20:85
chrX    5000000 rs12345  G    A,T  800.12  PASS    AF=0.15,0.02;DP=80 GT:AD:GQ 0/2:0,5,70:75 0/1:40,35,0:90
```

## Genotype Encoding (GT field)

The GT (genotype) field encodes which alleles are present at a locus in a specific sample:

- Alleles numbered: `0` = REF, `1` = first ALT, `2` = second ALT, etc.
- `/` separator = unphased (alleles not assigned to chromosomes)
- `|` separator = phased (alleles assigned to specific chromosomes)

| GT | Meaning |
|----|---------|
| `0/0` | Homozygous reference |
| `0/1` | Heterozygous (REF and first ALT) |
| `1/1` | Homozygous alternate |
| `0/2` | Heterozygous (REF and second ALT) |
| `1/2` | Heterozygous (first ALT and second ALT) |
| `0\|1` | Phased: haplotype 1 = REF, haplotype 2 = ALT |
| `./.` | Missing genotype |

The phasing distinction is biologically significant. An unphased `0/1` genotype tells you that one chromosome carries the REF allele and one carries the ALT allele, but not which is which. A phased `0|1` genotype additionally specifies which haplotype each allele belongs to — essential for compound heterozygosity analysis (when two different mutations on two different copies of a gene together cause disease), haplotype-based association studies, and long-range phasing across chromosomes.

## Key INFO Fields

GATK and other callers populate the INFO field with many annotations:

| Tag | Description |
|-----|-------------|
| `AF` | Allele frequency in the cohort |
| `DP` | Total sequencing depth at this position |
| `QD` | Variant quality divided by depth (GATK quality metric) |
| `FS` | Fisher's Strand Bias (low = good; high = strand-biased artifact) |
| `MQ` | RMS mapping quality of covering reads |
| `SOR` | Strand Odds Ratio (alternative strand bias metric) |
| `InbreedingCoeff` | Inbreeding coefficient across samples |

## VCF Manipulation with bcftools

```bash
# View VCF (with header)
bcftools view variants.vcf.gz | head -50

# Index a compressed VCF
bcftools index variants.vcf.gz

# Filter: PASS variants, MAF > 1%, depth > 10
bcftools filter -i 'FILTER="PASS" && AF>0.01 && DP>10' variants.vcf.gz \
    -o filtered.vcf.gz -O z

# Extract only SNPs
bcftools view -v snps variants.vcf.gz -o snps.vcf.gz -O z

# Subset samples
bcftools view -s Sample1,Sample2 variants.vcf.gz -o subset.vcf.gz -O z

# Statistics
bcftools stats variants.vcf.gz | grep "^SN"

# Annotate with VEP (Ensembl Variant Effect Predictor)
vep --input_file variants.vcf --vcf --output_file annotated.vcf \
    --species homo_sapiens --assembly GRCh38 --cache

# Merge multiple VCFs
bcftools merge sample1.vcf.gz sample2.vcf.gz sample3.vcf.gz -o merged.vcf.gz -O z
```

## Multi-Allelic Sites and Normalization

Multi-allelic sites (multiple ALT alleles at one position) occur frequently. Before analysis, VCF files should be **normalized** and multi-allelic sites split:

```bash
# Left-align and normalize indels
bcftools norm -f reference.fa variants.vcf.gz -o normalized.vcf.gz -O z

# Split multi-allelic sites (one ALT per line)
bcftools norm -m -any normalized.vcf.gz -o split.vcf.gz -O z
```

**Left-normalization**: indels can be represented equivalently at different positions due to repetitive sequence. The convention is to left-align — shift the indel as far left as possible:

```
REF: ATTTTTG
ALT: ATTTTG  (delete one T)
```

Could be called at position 2, 3, 4, or 5 (all Ts are equivalent). Left-normalization places it at position 2.

## Functional Annotation

Raw VCFs contain positional information but no functional interpretation. Annotation tools add:

- **Gene consequence**: synonymous, missense, frameshift, stop-gained, splice-site
- **Protein change**: e.g., p.Arg175His (amino acid change in protein)
- **Population frequency**: gnomAD allele frequency
- **Clinical significance**: ClinVar classification (benign, pathogenic, VUS)

```bash
# VEP annotation
vep --input_file variants.vcf --vcf --symbol --numbers --canonical \
    --sift b --polyphen b \
    --output_file annotated.vcf \
    --cache --offline

# ANNOVAR
table_annovar.pl variants.vcf humandb/ \
    -buildver hg38 \
    -out annotated \
    -protocol refGene,gnomad211_genome,clinvar_20230122 \
    -operation g,f,f -vcfinput
```

## Why This Matters

VCF is the universal language of human genetics, population genomics, and clinical genomics. All GWAS summary statistics, all gnomAD variants, all ClinVar submissions are stored and exchanged in VCF or formats derived from it. Understanding VCF structure — particularly genotype encoding and the INFO/FORMAT fields — is essential for correctly filtering variants, interpreting variant effect predictions, and computing population genetics statistics. A failure to normalize or correctly parse multi-allelic sites can lead to incorrect allele frequency estimates or missed functional variants.
