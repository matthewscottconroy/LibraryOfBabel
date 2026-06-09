# Hi-C and Chromatin Architecture

Here is something that does not follow directly from a linear reading of the genome: the Sonic hedgehog (SHH) enhancer that drives expression of SHH in limb buds — determining whether you will have five fingers rather than fused digits — is located approximately 1 megabase upstream of the SHH gene in the linear genome. By any intuitive notion of proximity, this enhancer should not be able to influence its target. And yet it does, reliably, in every vertebrate limb.

The resolution of this apparent paradox lies in three dimensions. The genome is not read as a linear tape. It folds in the nucleus into structures — loops, domains, compartments — that bring distant regulatory elements into physical proximity with their target genes, while insulating them from genes that should not be influenced. The SHH limb enhancer loops to contact the SHH promoter across a megabase of intervening sequence. This loop is maintained by the protein CTCF, which binds to sites flanking the domain and prevents regulatory signals from leaking to neighboring domains.

Before Hi-C, this architecture was inferred from genetics and from low-throughput 3C (Chromosome Conformation Capture) experiments, one locus at a time. Hi-C extended the principle to the entire genome simultaneously — capturing all chromatin contacts in a single experiment and revealing the global three-dimensional organization of every chromosome.

The three-dimensional organization of chromatin in the nucleus profoundly influences gene regulation. **Hi-C** is the genomic technique that captures genome-wide chromatin contacts, revealing how chromosomes fold in 3D space — from A/B compartments at the megabase scale to topologically associating domains (TADs) at the 100 kb scale to individual enhancer-promoter loops at the kilobase scale.

## Biological Context: Why 3D Matters

A linear representation of the genome (the reference sequence) obscures the fact that regulatory elements and their target genes may be millions of base pairs apart in linear sequence but in close physical proximity in 3D nuclear space. Examples:
- The Sonic hedgehog (SHH) enhancer is ~1 Mb from the SHH promoter in linear distance but loops to contact it in limb cells
- Super-enhancers cluster in phase-separated condensates to drive high expression
- Topological insulation by CTCF/cohesin prevents enhancers from acting on non-target genes

## Hi-C Protocol

Hi-C captures chromatin contacts through proximity ligation:

1. **Crosslink**: formaldehyde fixes chromatin contacts in situ
2. **Restriction digest**: HindIII or DpnII cuts chromatin into fragments
3. **Fill and mark**: fill restriction overhangs with biotinylated nucleotides
4. **Ligation**: dilute conditions favor ligation between crosslinked (proximal) fragments
5. **Reverse crosslinks and pull-down**: streptavidin beads capture biotinylated ligation junctions
6. **Sequence**: paired-end sequencing; each read pair = one chromatin contact

**Output**: a symmetric matrix of contact frequencies between all pairs of genomic loci (Hi-C contact matrix).

## Hi-C Contact Matrix Structure

The Hi-C matrix is typically visualized as a heatmap where color intensity represents contact frequency. Several hierarchical features are visible:

### A/B Compartments

At the megabase scale, chromosomes partition into two compartments:
- **A compartment** (active): open chromatin, gene-rich, early-replicating, H3K27ac+
- **B compartment** (inactive): compact chromatin, gene-poor, late-replicating, H3K27me3+

Principal component analysis (PC1) of the Hi-C matrix separates A (positive PC1) from B (negative PC1):

```python
import cooler
import cooltools

# Load Hi-C data in cooler format
clr = cooler.Cooler('sample.mcool::resolutions/1000000')

# Compute A/B compartments
gc_cov = cooltools.expected.coverage(clr, ...)
# E1 (first eigenvector) corresponds to A/B compartments
```

Compartment identity is highly cell-type-specific: a gene can switch from A to B (and be silenced) during differentiation.

The A/B compartmentalization is one of the most striking features of mammalian chromosome organization. The active (A) compartment is enriched for all the marks of active chromatin: H3K27ac, H3K4me3, early DNA replication, high gene density. The inactive (B) compartment is enriched for H3K9me3, late replication, gene-poor, and associated with the nuclear lamina. These compartments are not static: during cellular reprogramming (converting a somatic cell to an induced pluripotent stem cell), genome-wide compartment switching occurs as pluripotency genes move from B to A and tissue-specific genes move from A to B. The Hi-C map records these transitions with exquisite precision.

### Topologically Associating Domains (TADs)

At 100 kb–1 Mb scale, the genome is organized into **TADs**: triangular blocks of high intra-domain contact frequency separated by boundaries of low contact frequency.

```
     Domain 1       Domain 2       Domain 3
  ├──────────────┤├──────────────┤├──────────────┤
  High internal contacts   Low inter-domain contacts
```

TAD boundaries are enriched for:
- **CTCF binding sites**: CTCF + cohesin organizes loop extrusion
- Housekeeping genes
- Active transcription

TADs are largely conserved across cell types and species. Disruption of TAD boundaries by structural variants causes ectopic enhancer-promoter contacts and human disease (e.g., limb malformations from disruption of the EPHA4 TAD boundary).

The EPHA4 example is worth dwelling on. Patients with limb malformations carrying inversions or duplications in the EPHA4 locus were initially puzzling — the inversions did not disrupt any coding genes. The answer came from Hi-C: the rearrangements disrupted the TAD boundary flanking EPHA4, causing limb enhancers that normally act on EPHA4 to now contact WNT6 and IHH in the neighboring TAD, ectopically activating these genes in limb cells. The structural variant was not a coding mutation — it was a topological mutation, a disruption of the insulation that normally prevents enhancers from crossing domain boundaries. This opened an entirely new category of disease mechanism: regulatory rewiring through TAD disruption.

### Chromatin Loops

At 10–100 kb scale, individual enhancer-promoter loops and CTCF-mediated insulator loops are detectable in high-resolution Hi-C:

```bash
# MUSTACHE: loop calling from Hi-C
mustache -f sample.mcool -r 10000 -o loops.tsv -p 8

# Output: pairs of loop anchors with p-value
# Each loop = one enhancer-promoter or CTCF-CTCF interaction
```

**Loop extrusion model**: cohesin extrudes chromatin into loops; CTCF at convergently oriented binding sites acts as a roadblock, defining loop anchors. This mechanism explains:
- Why CTCF sites must be in convergent orientation to form stable loops
- Why cohesin depletion collapses loops while maintaining compartments
- Why CTCF depletion disrupts TAD boundaries

The loop extrusion model is one of the most elegant mechanistic proposals in modern cell biology. Cohesin — a ring-shaped protein complex that topologically embraces two DNA strands — is thought to extrude chromatin loops by sliding along DNA in an ATP-dependent manner. When it encounters a CTCF site on the DNA, it stalls (but only if the CTCF sites are in convergent orientation). The result is a stable, CTCF-anchored loop. Experimental depletion of cohesin (using the auxin-inducible degron system) immediately collapses all loops and TADs while leaving A/B compartments intact — a beautiful dissociation that demonstrated that loops and compartments are maintained by different mechanisms.

## Hi-C Analysis Pipeline

```bash
# Step 1: Align read pairs separately
bwa mem -5SP genome.fa reads_R1.fastq.gz reads_R2.fastq.gz \
    | samtools view -bS > aligned.bam

# Step 2: Process with pairtools (identifies valid ligation pairs)
pairtools parse -c chromsizes.txt --drop-sam aligned.bam \
    | pairtools sort --nproc 16 \
    | pairtools dedup \
    | pairtools select '(pair_type=="UU") or (pair_type=="RU") or (pair_type=="UR")' \
    | pairtools split --output-pairs valid_pairs.pairs.gz \
                      --output-sam /dev/null

# Step 3: Generate contact matrix (cooler format)
cooler cload pairs -c1 2 -p1 3 -c2 4 -p2 5 \
    chromsizes.txt:10000 valid_pairs.pairs.gz sample_10kb.cool

# Step 4: Multi-resolution (mcool)
cooler zoomify --nproc 8 sample_10kb.cool

# Step 5: Normalize (ICE/balanced normalization)
cooler balance sample_10kb.cool

# Step 6: TAD calling (TopDom, insulation score)
python -m cooltools insulation sample.mcool -r 50000 \
    -w 500000 -o insulation_scores.tsv

# Step 7: Loop calling (Mustache or HICCUPS)
mustache -f sample.mcool -r 10000 -o loops.tsv -p 8
```

## Visualization with Juicebox and pyGenomeTracks

```python
import cooler
import matplotlib.pyplot as plt
import numpy as np

# Load and visualize a genomic region
clr = cooler.Cooler('sample.mcool::resolutions/25000')
region = ('chr17', 35_000_000, 45_000_000)
matrix = clr.matrix(balance=True).fetch(region)

fig, ax = plt.subplots(figsize=(10, 10))
im = ax.imshow(np.log2(matrix + 1), cmap='RdYlBu_r', 
               aspect='auto', origin='upper',
               vmin=0, vmax=4)
plt.colorbar(im, ax=ax, label='log2(contact frequency)')
ax.set_title(f'Hi-C contact matrix: {region[0]}:{region[1]/1e6:.1f}-{region[2]/1e6:.1f} Mb')
```

## Micro-C and High-Resolution Methods

**Micro-C** uses MNase instead of restriction enzymes to cut chromatin at single-nucleosome resolution, enabling detection of nucleosome-scale contacts and very fine loop structures. It requires deeper sequencing than Hi-C but provides unprecedented resolution.

## Why This Matters

Chromatin architecture is the structural basis of gene regulation. Understanding which enhancers contact which promoters — and how this changes across cell types and disease states — is essential for interpreting non-coding GWAS variants (80% of which fall in non-coding regulatory regions). TAD boundaries insulate regulatory domains; their disruption by structural variants causes cancer (e.g., oncogene activation by hijacking neighboring enhancers) and developmental disorders. Hi-C analysis has become standard in developmental biology, cancer genomics, and regulatory genomics. The insight that the linear genome folds into a functional 3D architecture that is dynamically regulated is one of the most profound contributions of the genomics era.

Zoom out to the fundamental insight: the regulatory genome is not a one-dimensional text. It is a three-dimensional object, and the regulatory information is encoded in the spatial relationships between elements as much as in their sequences. A GWAS variant in a non-coding region 500 kb from the nearest gene might be in a perfect position to disrupt an enhancer-promoter loop that brings those two elements into contact. Without the Hi-C contact map for the relevant cell type, you cannot know. With it, you can identify the likely regulatory target of the variant, generate a mechanistic hypothesis, and test it experimentally. The map of three-dimensional genome organization is, in this sense, the missing context that makes non-coding variation interpretable.
