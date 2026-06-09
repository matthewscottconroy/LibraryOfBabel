# Genome Synthesis and Recoding

What would it mean to truly understand an organism's genome? Not to map it, or annotate it, or edit it gene by gene — but to design it from scratch and build it from raw chemical components, then watch it live? That is the question that drove Craig Venter's team to synthesize the genome of *Mycoplasma mycoides* in 2010. The result — the first cell running entirely on a synthetic genome — was not just a technical achievement. It was a philosophical landmark: proof that the information content of a genome, when transferred to a synthetic DNA molecule, is sufficient to direct all of cellular life. And it opened a line of research that has since led to the world's first minimal synthetic cell, an *E. coli* strain with a compressed genetic code, and a nearly complete synthetic yeast chromosome. Each of these projects asks a different version of the same profound question: what, exactly, is necessary for life?

Genome synthesis takes genome editing from modification of individual loci to the wholesale design and construction of entire chromosomes from synthetic DNA. It addresses questions that cannot be answered by editing existing sequences: What is the minimal gene set required for life? What happens if we reassign codons? Can we build entirely designed chromosomes that function in living cells?

## The First Synthetic Genomes

**JCVI-syn1.0 (2010)**: Gibson et al. and the Venter Institute synthesized the complete 1.08 Mb genome of *Mycoplasma mycoides* from chemically synthesized DNA fragments. The synthetic genome was transplanted into a *M. capricolum* recipient cell that had its own genome removed. The resulting cells replicated using only the synthetic genome — the first fully synthetic self-replicating organism.

**Technical approach**:
1. Genome sequence divided into 1,078 overlapping fragments of ~1 kb each
2. Fragments synthesized commercially, assembled in yeast by successive rounds of Gibson assembly and yeast homologous recombination
3. Assembled chromosome (1.08 Mb) transplanted into recipient cell by polyethylene glycol-mediated transformation
4. Cells screened by auxotrophic markers encoded in the synthetic genome

**JCVI-syn3.0 (2016)**: reduced JCVI-syn1.0 to the minimal gene set. By systematic deletion of genes, the minimal genome was reduced to **473 genes** occupying 531 kb. These 473 genes represent the core biosynthetic and information-processing machinery required for cellular life — though 149 of these genes have unknown functions, indicating significant gaps in our understanding of even the simplest life.

## Genome Recoding

Recoding replaces one or more codons throughout the entire genome, typically to free a codon for reassignment to non-natural amino acids (ncAAs) or to create **semantic containment** (the organism cannot survive without the recoded codon's unique function).

### *E. coli* Genomic Recoding (Recode-2)

The landmark 2013 paper by Lajoie et al. (Church lab) recoded all 321 TAG stop codons in *E. coli* to TAA stop codons:

**Why TAG**: of the three stop codons (TAA, TAG, TGA), TAG is least frequently used in *E. coli* coding sequences (~7%) and has the weakest read-through tendency. Replacing all TAGs with TAA has minimal effect on protein production.

**Approach**: MAGE (multiplex automated genome engineering) used to replace TAG codons in batches of 10–20 at a time, over 32 rounds of MAGE. Intermediate strains were combined by bacterial conjugation to accumulate all 321 replacements.

**Result**: the fully recoded *E. coli* strain (321 TAG→TAA replacements) is viable and grows at near-normal rates. The freed TAG codon is now available for reassignment: when an orthogonal aminoacyl-tRNA synthetase (aaRS)/tRNA pair recognizing TAG is introduced, the cell incorporates an ncAA at TAG positions in designed proteins.

**Significance**: because all endogenous stop codons are now TAA, the cell is genetically isolated — it cannot read proteins from contaminating organisms that use TAG as a stop codon. This is a form of biological containment and genetic isolation.

### Recode-2 (Fredens et al. 2019): Complete Genome Recoding

The Church lab's 2019 *Nature* paper (Fredens, Wang, de la Torre et al.) achieved a more ambitious recoding: replace all **18,214 serine TCG and TCA codons** plus **all 321 TAG stop codons** in *E. coli*, freeing two complete sense codons (Ser) and one stop codon. The result was a 4-Mb synthetic genome with 3 compressed codons, constructed in its entirety from a redesigned sequence.

**Construction method**: rather than MAGE-based accumulation, the redesigned genome was divided into 37 sections. Each section was synthesized chemically, assembled by Gibson assembly, and sequentially recombined into the host genome by replicon-excision-based genome replacement.

**Properties of the recoded strain (Syn61)**: viable, grows slightly slower than wild-type. Resistant to infection by natural phages (because phage proteins rely on TCG/TCA codons that are now absent from the translation machinery). This viral resistance demonstrates a functional application of genome recoding.

## Synthetic Yeast Genome: Sc2.0

The **Sc2.0 project** is an international consortium effort to synthesize and replace the entire *Saccharomyces cerevisiae* genome with a designed version:

**Design principles**:
- Remove repetitive elements (many LTR retrotransposons, tRNA genes relocated to synthetic chromosomes)
- Replace TAG stop codons with TAA
- Insert **loxPsym sites** every ~10 kb (for inducible genome rearrangement via Cre recombinase — "SCRaMbLE" system)
- Remove introns from most non-essential genes
- Retain all 6,000 essential and functional genes

**Status (as of recent years)**: ~7 of 16 yeast chromosomes have been synthesized and replaced in living yeast. The synthetic chromosomes function normally, demonstrating that designed eukaryotic chromosomes work in cells.

**SCRaMbLE (Synthetic Chromosome Rearrangement and Modification by LoxPsym-Mediated Evolution)**: inducible Cre recombinase randomly rearranges the synthetic chromosomes at loxPsym sites, generating a population of strains with different gene arrangements. Screens of these populations identify strains with improved phenotypes — genome-scale evolution in controlled experimental conditions.

## Semantic Containment by Recoding

A practical application of genome recoding is **semantic containment**: engineering organisms that cannot exchange functional genes with wild-type organisms because they use an incompatible genetic code.

**Implementation**:
1. Recode essential genes in the engineered organism to use a reassigned codon (e.g., all ACG-Thr replaced by synonymous codons; ACG freed for ncAA incorporation)
2. Essential proteins in the engineered organism require the ncAA at recoded positions
3. Without the ncAA (and the orthogonal machinery to incorporate it), the organism cannot produce functional essential proteins
4. Horizontal gene transfer of recoded genes into a natural organism produces non-functional proteins → effective gene flow containment

This approach, demonstrated in 2016 by Mandell et al., achieved organisms that require a synthetic amino acid for survival and are highly resistant to evolution of escape because multiple essential genes require the ncAA simultaneously.

## Why This Matters

Genome synthesis closes the loop between computational sequence design and physical genetic information: rather than modifying what evolution provided, synthetic genomics creates genomes designed from first principles. JCVI-syn3.0 revealed that even the simplest known life requires 149 genes of unknown function — a humbling reminder of how much we don't understand about biology. Genome recoding demonstrates that the genetic code itself is evolvable and can be repurposed for new functions, from ncAA incorporation to biocontainment. The Sc2.0 project, when completed, will provide the first demonstration that an entire eukaryotic genome can be designed and built by human engineers. These are not just technical achievements; they define the frontier of what it means to understand — and engineer — life at the molecular level.
