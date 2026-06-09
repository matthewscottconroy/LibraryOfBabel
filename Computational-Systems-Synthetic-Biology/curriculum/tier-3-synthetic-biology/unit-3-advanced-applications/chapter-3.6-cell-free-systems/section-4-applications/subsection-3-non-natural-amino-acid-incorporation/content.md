# Non-Natural Amino Acid Incorporation in Cell-Free Systems

The genetic code has 64 codons and encodes 20 amino acids — a mapping that has been conserved across nearly all life for three and a half billion years. But that code is not a physical law. It is a biochemical convention, and cell-free systems give you the tools to break it deliberately. One of the most powerful unique capabilities of cell-free systems — one that is essentially impossible to replicate in living cells — is the incorporation of **non-natural amino acids (ncAAs)** into proteins at defined positions. Cell-free systems enable this because every component of the translation machinery can be individually added, replaced, or modified. The result is site-specific incorporation of any chemical group — functional handles for bioconjugation, spectroscopic probes, photocrosslinking agents, or entirely new chemistries — into proteins with near-perfect positional control.

## The Amber Suppression Strategy

The standard approach for ncAA incorporation exploits the **amber stop codon (UAG)**:

1. The gene of interest is mutated to introduce a UAG codon at the desired incorporation site.
2. An **orthogonal aaRS/tRNA pair** is added to the translation reaction — an aminoacyl-tRNA synthetase (evolved to charge a non-natural amino acid) paired with a cognate suppressor tRNA that decodes UAG.
3. When the ribosome reaches the UAG codon, the orthogonal tRNA — carrying the ncAA — competes with Release Factor 1 (RF1, which would normally terminate translation at UAG).
4. If the orthogonal tRNA wins the competition, translation continues and the ncAA is incorporated at that position.

The orthogonal requirement is critical: the suppressor tRNA must not be charged by any natural *E. coli* aaRS, and the orthogonal aaRS must not charge any natural *E. coli* tRNA. Without orthogonality, the ncAA would be incorporated at unintended positions, or the orthogonal tRNA would be charged with a natural amino acid.

## Commonly Used Orthogonal Pairs

**Methanocaldococcus jannaschii TyrRS/tRNA$^{Tyr}_{CUA}$** (Schultz lab): the founding orthogonal pair, evolved to charge p-azidophenylalanine (AzF), p-acetylphenylalanine, and many other aromatic ncAAs. The *M. jannaschii* pair functions in *E. coli* cell-free because the archaeal tRNA is not recognized by any bacterial aaRS.

**Pyrrolysyl-tRNA synthetase (PylRS)/tRNA$^{Pyl}_{CUA}$** (Srinivasan, Chin labs): derived from *Methanosarcina* species. Especially versatile — accepts a wide range of lysine analogues including:
- Propargyl-L-lysine (PrK) — alkyne for click chemistry
- Nε-acetyl-L-lysine — for studying protein acetylation
- Norbornene-lysine — for tetrazine ligation
- BCNK — for inverse electron demand Diels-Alder cycloaddition

PylRS has been evolved for dozens of ncAAs and is now the most commonly used pair for bioconjugation applications.

## Why Cell-Free Is Superior for ncAA Incorporation

**In vivo ncAA incorporation** faces three key limitations:

1. **RF1 competition**: RF1 terminates translation at UAG codons. In cells, this reduces ncAA incorporation efficiency (typical 20–40% suppression efficiency). Removing RF1 from cells is lethal without complementation.

2. **ncAA uptake**: the ncAA must cross the cell membrane. Many charged or large ncAAs are not efficiently imported.

3. **ncAA stability**: metabolically active cells may degrade or modify the ncAA before it is incorporated.

**In cell-free systems**, all three problems are eliminated:

1. **RF1 can be removed from the PURE system**: simply omit RF1 from the reconstituted reaction. In a genomically recoded strain (where all UAG codons have been eliminated from the genome — as in *E. coli* C321.ΔA*, the Lajoie 2013 strain), extracts lack functional RF1 activity at the amber codon, and suppression efficiency approaches 95–99%.

2. **No membrane barrier**: add any ncAA directly to the reaction at any concentration.

3. **No metabolism**: cell-free reactions do not metabolize the ncAA.

**Suppression efficiency comparison**:

| Platform | RF1 status | ncAA suppression efficiency |
|---|---|---|
| Standard in vivo | Full RF1 competition | 20–40% |
| RF1-knockout + genome recoding (in vivo) | RF1 absent | 60–80% |
| PURE system (RF1 omitted) | RF1 absent | 80–99% |
| C321.ΔA* extract (cell-free from recoded strain) | RF1 absent | 70–95% |

## Multi-Site Incorporation

Because cell-free systems can achieve near-complete suppression of amber codons, it becomes feasible to incorporate multiple ncAA molecules per protein at different positions.

**Example**: a protein with UAG codons at positions 15, 48, and 112 will incorporate ncAA at all three positions if the suppression efficiency per codon is high enough. At 90% suppression per codon:
$$P(\text{full-length with 3 ncAAs}) = 0.90^3 \approx 0.73$$

At 50% (in vivo with RF1): $0.50^3 = 0.125$. The yield difference is enormous.

Multi-site ncAA incorporation has been used to make **bispecific antibody conjugates** with two different ncAAs at specific sites, allowing attachment of two distinct drug payloads or fluorescent labels in defined stoichiometry.

## Applications of ncAA Incorporation

**Bioconjugation for antibody-drug conjugates (ADCs)**:
Site-specific conjugation at an ncAA avoids the heterogeneity of traditional NHS-ester or maleimide conjugation (which reacts with all lysines or cysteines). Companies including Sutro Biopharma and Ambrx have used cell-free ncAA incorporation for ADC production at industrial scale.

Example reaction:
```
Protein-AzF (azide at position 124) + DBCO-drug → Cu-free click → Protein-drug conjugate
```
Stoichiometry: exactly 1 drug per protein (if one UAG introduced), or 2 per protein (two UAGs), etc.

**Fluorescent labeling for single-molecule spectroscopy**:
Tetrazine-ncAA + TCO-dye (trans-cyclooctene-fluorophore) → site-specific fluorescent label with ms ligation kinetics. Enables dual-labeling for FRET (two positions labeled with different colors) to measure conformational changes.

**Photocrosslinking for protein interaction mapping**:
p-Benzoylphenylalanine (BPA) incorporated at candidate binding interfaces; UV irradiation crosslinks BPA to any protein within 3.1 Å. Identifies protein-protein contact sites by mass spectrometry.

**Novel backbone chemistry**:
Moving beyond side chain modification: ribosomal synthesis of proteins with modified backbone (β-amino acids, α-hydroxy acids) has been demonstrated in the PURE system by replacing aaRS and/or modifying peptide bond geometry. This pushes cell-free toward truly non-biological polymer synthesis — **peptidomimetics** with enhanced proteolytic stability.

## The Genetic Code Expansion Workflow in Cell-Free

```python
# Conceptual workflow for ncAA incorporation in PURE system

# 1. Express and purify the orthogonal pair
orthogonal_aaRS = purify_protein("MjTyrRS_evolved")  # evolved for AzF
orthogonal_tRNA = transcribe_in_vitro("tRNA_CUA_Mj")  # suppressor tRNA

# 2. Aminoacylate the tRNA in vitro (pre-charging)
charged_tRNA = orthogonal_aaRS.charge(orthogonal_tRNA, AzF, ATP)

# 3. Set up PURE reaction with RF1 omitted
pure_reaction = PURE(
    ribosomes=1.2e-6,   # M
    IFs=[IF1, IF2, IF3],
    EFs=[EFTu, EFTs, EFG],
    RFs=[RF2],          # RF1 omitted
    aaRS=all_20_plus_MjTyrRS,
    tRNAs=total_tRNA_plus_tRNA_CUA,
    DNA=gene_with_UAG_at_position_124,
    energy=CP_CK_system
)

# 4. Run reaction and measure
product = pure_reaction.run(37, duration_hours=4)
yield_mg_ml = measure_protein_yield(product)
incorporation_efficiency = measure_by_mass_spec(product, AzF_mass)
```

## Quantitative Performance

For well-optimized ncAA PURE systems:
- Protein yield: 50–200 µg/mL (lower than natural protein due to competition)
- Suppression efficiency at a single UAG site: 80–99% (PURE, RF1-omitted)
- Multiple UAG sites: yield decreases multiplicatively with number of sites

The Sutro Biopharma platform — which operates cell-free ncAA incorporation at 100-liter scale — produces pharmaceutical-grade antibody-drug conjugate intermediates at yields and homogeneity not achievable by other conjugation methods.

## Why This Matters

Non-natural amino acid incorporation in cell-free systems is not merely an experimental curiosity — it is a pharmaceutical manufacturing platform. By enabling site-specific, stoichiometrically defined conjugation of drug payloads, fluorescent labels, or new chemical functionalities onto proteins, cell-free ncAA technology solves problems that have frustrated protein engineers for decades. The inability to control conjugation site in traditional ADC production directly impacts drug efficacy and toxicity (heterogeneous DAR distributions). Cell-free ncAA production solves this at industrial scale. More broadly, this application illustrates the general principle that cell-free systems' openness — the ability to add, remove, or replace any component — is not just a convenience but a fundamental capability difference that enables entire new classes of molecular biology.
