# Genome-Scale Engineering: Metabolic Engineering Applications

In 2006, Jay Keasling's group at Berkeley engineered *E. coli* and yeast to produce artemisinic acid — a precursor of the antimalarial drug artemisinin — from simple sugars. The project required reconstructing a 10-enzyme biosynthetic pathway, redirecting carbon flux away from competing pathways, and fine-tuning expression of each enzyme to prevent toxic intermediate accumulation. It took years and involved dozens of genetic modifications. Today, a comparable engineering project could be accomplished in weeks, using CRISPR to make each modification precisely, quickly, and without scars. That acceleration is not just a convenience — it changes what is scientifically feasible. Questions about the relationship between genome configuration and metabolic phenotype that previously required a career can now be answered in a semester.

Metabolic engineering in the CRISPR era goes beyond modifying individual genes to systematically rewiring entire cellular metabolism. CRISPR enables sequential knockouts, safe harbor insertions, and combinatorial modifications that together constitute genome-scale metabolic engineering — building on the flux balance analysis frameworks from systems biology to make rational, genome-wide design decisions.

## The Sequential Knockout Strategy

Metabolic engineering typically requires eliminating multiple competing pathways to maximize flux toward the target product. Each elimination is a separate genome editing event, and the number of required knockouts for complex products often reaches 10–20 genes.

**CRISPR simplifies sequential knockouts**: each gene is targeted with a specific sgRNA; NHEJ-mediated indels disrupt the reading frame; no selection cassette is needed (or cassettes can be removed between rounds). In yeast, 6–10 gene knockouts that previously required months of work can be achieved in 2–3 weeks.

### Safe Harbor Sites

When overexpressing pathway enzymes, the genomic insertion site matters:
- **Insertion into the coding region of an essential gene** is lethal
- **Insertion into a gene with a phenotype** may cause unwanted effects
- **Safe harbor sites** are genomic loci that tolerate insertions without affecting fitness

Validated safe harbors:
- *S. cerevisiae*: HO (mating type switching, dispensable in laboratory), CAN1 (can be deleted without growth defect), and specific intergenic regions on chromosomes V and XI
- Human cells: **AAVS1** (intron of PPP1R12C on chr19; widely used), **H11 locus** (chr22), **Rosa26 ortholog** in various cell types
- *E. coli*: between lacZ and bioB; attTn7 site (phage attachment site, well-characterized)

### Example: Lycopene Production Strain in *E. coli*

Constructing an *E. coli* strain overproducing lycopene (carotenoid pigment) requires:

**Knockouts** (competing pathway elimination):
- ΔcrtB (eliminate endogenous geranylgeranyl pyrophosphate consumption)
- Δpta-ackA (reduce acetate overflow, redirect acetyl-CoA)
- ΔfadR (improve fatty acid precursor flux)

**Overexpressions** (via chromosomal integration at safe harbor):
- dxs (1-deoxy-D-xylulose-5-phosphate synthase, rate-limiting in MEP pathway)
- idi (IPP isomerase)
- ispA (geranylgeranyl pyrophosphate synthase)
- crtEBI (lycopene biosynthesis genes from Pantoea agglomerans)

All modifications achievable by CRISPR in 2–3 weeks; final strain produces >2 g/L lycopene.

## The OptKnock / FBA-Guided Design Paradigm

Choosing which genes to knock out is not always obvious, especially for complex metabolic networks. **Flux Balance Analysis (FBA)** and the OptKnock algorithm identify knockout combinations that couple cell growth to product formation:

```python
# Pseudocode for OptKnock-guided knockout selection
from cobra.flux_analysis import OptKnock

model = load_gem("iJO1366")  # E. coli GEM, 1366 reactions
model.objective = "biomass"

opt = OptKnock(model)
opt.setup(
    target_reaction="EX_lycopene",  # exchange reaction for product
    num_knockouts=3,
    must_grow=True
)
solutions = opt.run()
# Returns: list of 3-gene knockout combinations that maximize lycopene yield at growth optimum
```

OptKnock forces the optimization to find combinations where the only way for the cell to grow well is to also produce the desired compound — so evolution drives the strain toward higher production rather than away from it.

## CRISPR-Enabled Multiplex Modification

A key advantage of CRISPR over traditional recombineering is the ability to target multiple sites simultaneously by delivering multiple sgRNAs. In yeast, 6 simultaneous deletions have been achieved in a single transformation. In mammalian cells, 3–5 simultaneous knockouts are routine.

This is important for metabolic engineering because:
- Precursor pathways often require multiple simultaneous changes
- Testing all combinations of 5 knockouts one at a time would require 5 sequential experiments; simultaneously achieves all in one

**Multiplex sgRNA delivery options**:
- Multiple sgRNAs from a single plasmid (polycistronic, processed by ribozymes or Cas12a)
- Multiple PCR-amplified sgRNA cassettes co-transformed
- Arrayed delivery (one sgRNA per well for systematic studies)

## CRISPR in Industrially Relevant Organisms

Not all industrial organisms are as well-characterized as *E. coli* and *S. cerevisiae*. CRISPR has dramatically expanded the range of organisms amenable to genetic modification:

**Corynebacterium glutamicum**: major industrial amino acid producer; CRISPR tools now enable multiplex knockout of competing pathways for lysine and valine overproduction.

**Aspergillus niger**: used for citric acid and enzyme production; CRISPR replaces traditional protoplast transformation with higher efficiency and precision.

**Clostridium** species: anaerobic producers of butanol and other chemicals; CRISPR in anaerobes was technically challenging and is now achievable with modified delivery conditions.

**Microalgae** (Chlamydomonas, Nannochloropsis): photosynthetic producers of lipids and high-value metabolites; CRISPR enables precise lipid pathway engineering.

## Chromosomal Copy Number and Expression Tuning

Beyond knockouts and insertions, CRISPR can modulate expression of existing chromosomal genes:

**CRISPRi for dosage optimization**: dCas9-KRAB or dCas9-MQ1 can repress native genes to intermediate levels — not full knockout, but reduced expression. This allows tuning enzyme levels along a pathway without irreversible chromosomal modification.

**CRISPRa for overexpression without plasmid**: dCas9-VP64 activates native gene expression 2–50-fold without plasmid maintenance. Relevant when plasmid metabolic burden is a concern at scale.

**Gene multiplication**: integrate additional copies of rate-limiting enzymes at safe harbor sites to increase total enzyme activity without high-copy plasmid maintenance.

## Why This Matters

Genome-scale metabolic engineering using CRISPR represents a qualitative shift in what can be accomplished in rational strain design. Pre-CRISPR, 10 sequential genetic modifications required 10 rounds of transformation, selection, and marker excision — 3–6 months of work. Post-CRISPR, the same 10 modifications can be achieved in 3–4 weeks, with each modification precisely specified and confirmed by sequencing. The connection to FBA and genome-scale metabolic models means that computational predictions of optimal knockout combinations are directly testable without the experimental burden that previously made exhaustive combinatorial testing impossible. This convergence of genome-scale computation (metabolic modeling) and genome-scale physical intervention (CRISPR) is the core of modern metabolic engineering.
