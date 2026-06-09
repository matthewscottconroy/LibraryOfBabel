# CRISPR-Assisted Recombineering

Lambda Red recombineering on its own has a frustrating problem: most cells in your population don't recombine. After electroporating an oligo, you get roughly one recombinant per thousand cells — which means screening ninety-nine failures to find one success. That's manageable for a few experiments, but when metabolic engineering requires dozens of sequential modifications, it becomes a bottleneck. CRISPR changes the arithmetic entirely. If Cas9 cuts the genomic target and that cut is lethal, then the only cells that survive are the ones that repaired the break — and if you've provided a recombineering template as the repair substrate, those survivors are the cells you want. You've turned a 0.1% efficiency problem into a near-complete selection. The logic is elegant: rather than finding the rare recombinant, you kill everything else.

CRISPR-assisted recombineering combines the programmable DNA cutting of Cas9 with the precise sequence modification capabilities of lambda Red recombineering. The combination overcomes a fundamental limitation of each approach individually: recombineering is efficient but unselected (must screen hundreds of colonies), while Cas9 creates DSBs that are lethal unless repaired. Together, they create a powerful system where Cas9 lethality selects for cells that have successfully repaired the DSB by recombineering.

## The Logic of CRISPR Counter-Selection for Recombineering

The key insight is that a Cas9 DSB at the target site is lethal to bacterial cells unless they repair it. If a recombineering repair template is provided, cells that successfully recombine the template are no longer cut by Cas9 (because the recombination disrupts the guide RNA target site or PAM). These cells survive; unrecombined cells are cut again and again until they die.

**Net effect**: CRISPR acts as a **counter-selection** for recombineering, enriching the recovered population for cells that have incorporated the repair template. This converts recombineering efficiency from 0.01–0.1% to near 100% of recovered colonies.

## Protocol

### Setup

A strain expressing lambda Red proteins (Gam, Beta, Exo) from an inducible promoter, plus a Cas9 expression system (or RNP delivery).

### sgRNA and Repair Template Design

**sgRNA**: target the genomic site to be modified. The cut site should overlap with the modification to be introduced, or be within 20 bp.

**Repair template design**: design the oligo or PCR product so that:
1. The repaired sequence contains the desired modification
2. The repaired sequence **disrupts the PAM or spacer** recognized by the guide RNA (to prevent re-cutting after repair)

If the desired modification does not naturally disrupt the PAM, add a silent mutation in the PAM or seed region of the guide target on the repair template.

### Procedure

```
1. Induce Red expression (grow + arabinose for pKD46-based systems)
2. Prepare cells by washing, transfer to ice
3. Mix with:
   - sgRNA (or Cas9-sgRNA RNP)
   - Repair template (ssDNA oligo or PCR product)
4. Electroporate
5. Recover in rich medium (1–3 hours)
6. Plate on selective plates (antibiotic for Cas9 plasmid + plating medium)
7. Cas9 cuts unrepaired cells → they die; repaired cells survive
8. Screen survivors (colony PCR or sequencing)
```

**Expected result**: >80% of surviving colonies carry the desired modification.

## CRISPR-Assisted vs. Standard Recombineering: Efficiency Comparison

| System | Recombinant Frequency | Screening Required |
|--------|---------------------|-------------------|
| Lambda Red alone (ssDNA oligo, no selection) | 0.01–0.1% | 100–1000 colonies |
| Lambda Red alone (dsDNA, antibiotic selection) | 100% of survivors | Minimal |
| CRISPR + Lambda Red (counter-selection) | 50–100% of survivors | 5–10 colonies |

CRISPR-assisted recombineering achieves near-perfect enrichment without requiring an antibiotic resistance gene in the modification itself, making it ideal for seamless modifications (single-nucleotide changes, small insertions, deletions without selectable markers).

## Extensions: CRISPR-Assisted Genome-Scale Engineering

**MAGE + CRISPR counter-selection** (CAGE — CRISPR Accelerated Genome Engineering): applies multiple rounds of MAGE while using CRISPR to positively select for cells that have accumulated the desired mutations. Because Cas9 kills cells that have not recombined, each MAGE round is enriched for successful modifications.

Wang et al. (2016, Church lab) used CAGE to recode essential genes with non-natural codons at dramatically higher efficiency than MAGE alone, achieving near-complete recoding in shorter experimental timelines.

**Multiplex CRISPR-assisted editing in yeast**: in *Saccharomyces cerevisiae*, which has naturally efficient homologous recombination, CRISPR DSBs at multiple sites simultaneously increase the efficiency of multi-locus recombineering dramatically. Cas9 with multiple sgRNAs + multiple repair templates can modify 5+ genomic loci in a single transformation.

## Applications in Metabolic Engineering

CRISPR-assisted recombineering is the preferred method for metabolic pathway engineering in *E. coli* and other bacteria because it enables:

**Seamless gene knockouts**: delete entire genes without leaving antibiotic resistance scars that would complicate subsequent modifications.

**Promoter replacements**: replace native promoters with synthetic variants of defined strength (from Anderson promoter library) to tune expression levels. Each replacement is a small modification (< 100 bp) perfectly suited to oligo recombineering.

**RBS optimization**: systematically vary the Shine-Dalgarno sequence of a target gene to tune translation efficiency. Testing a library of 20 RBS variants across a single gene requires 20 electroporations — tractable within a week.

**Simultaneous pathway optimization**: in combination with MAGE, optimize multiple enzymes in a biosynthetic pathway in a single experiment.

## CRISPR Recombineering in Other Organisms

The principle extends beyond *E. coli*:

**Mycobacteria**: lambda Red orthologs (Che9c phage system) + CRISPR used for *Mycobacterium tuberculosis* engineering — previously one of the most difficult organisms to manipulate genetically.

**Pseudomonas and Acinetobacter**: native recombination systems + CRISPR counter-selection enable modification of these industrially relevant and clinically important bacteria.

**Yeast**: yeast have highly efficient endogenous recombination that does not require lambda Red. CRISPR DSBs in yeast efficiently stimulate HDR from supplied linear DNA with 50-nt homology arms, achieving >90% recombinant frequency at the CRISPR cut site.

## Why This Matters

CRISPR-assisted recombineering solved a persistent problem in bacterial genetics: that precise modifications without selectable markers required screening thousands of colonies to find rare recombinants. By coupling Cas9 lethality with recombineering as an escape route, the system makes any precisely defined modification recoverable from a small number of colonies. For metabolic engineering — where dozens of successive modifications are often required to optimize a production strain — reducing the screening burden at each step from hundreds of colonies to five or ten is a practical revolution. The combination of Lambda Red's short-homology recombination mechanism with Cas9's programmable cutting is a clear example of how tools from different biological contexts (phage-bacterial warfare; adaptive immune memory) can be combined to solve engineering challenges that neither could address alone.
