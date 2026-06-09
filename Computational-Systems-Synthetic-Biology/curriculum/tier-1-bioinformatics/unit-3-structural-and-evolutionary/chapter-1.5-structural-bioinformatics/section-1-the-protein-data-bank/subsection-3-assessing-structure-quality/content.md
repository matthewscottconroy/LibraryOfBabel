# Assessing Structure Quality

Imagine you have identified the perfect drug target — a kinase that drives tumor growth, with a beautiful binding pocket and no close human homologs. You search the PDB, find an existing structure at 3.2 Å resolution, and start designing inhibitors based on the active site geometry. Six months and considerable resources later, your synthesized compounds fail across the board. When you finally obtain a 1.8 Å structure of the same protein, you discover that several active site side chains were misplaced in the lower-resolution model, and the binding pocket you were designing into is substantially different from reality.

This scenario is not hypothetical. It has played out in pharmaceutical programs more than once. Not all structures in the PDB are equally reliable. Structures are deposited with a wide range of resolution, refinement quality, and model accuracy. A naïve user who treats all PDB structures as ground truth will encounter serious errors — incorrect hydrogen bond networks, misidentified ligands, or wrong side chain conformations. Systematic assessment of structure quality before any analysis is therefore not optional; it is the first step of every structural bioinformatics workflow.

## Resolution

**Resolution** is the single most important quality indicator. Expressed in Angstroms (Å), it describes the minimum spatial frequency present in the experimental data:

- **≤ 1.5 Å (excellent)**: Individual atoms resolved; hydrogen positions sometimes visible; water structure detailed; alternative conformations accurately identified. Suitable for the most demanding applications (charge density analysis, very accurate distance measurements).
- **1.5–2.5 Å (good)**: Backbone and most side chains accurately placed; water molecules visible; the large majority of PDB structures fall here. Suitable for drug binding analysis, mechanistic studies.
- **2.5–3.5 Å (moderate)**: Backbone generally reliable; side chain positions less certain (especially long, flexible residues); many water positions ambiguous. Interpret side chain contacts with caution.
- **> 3.0 Å (low)**: Secondary structure visible; individual residue positions uncertain; side chains should not be used for quantitative analysis. Often seen for large membrane protein complexes or difficult-to-crystallize proteins.

It turns out that resolution is not just a number in the header — it reflects a fundamental tradeoff between crystal quality, beam intensity, and data completeness. A large, well-ordered crystal at a modern synchrotron can yield data to 1.0 Å. A small, imperfectly packed crystal of a flexible membrane protein might give data only to 3.5 Å, no matter how many hours you spend on the beamline.

For cryo-EM structures, the global resolution reported by the FSC 0.143 criterion may be misleading because local resolution varies enormously — the core of a particle is typically resolved at 2–3 Å while flexible periphery may be 5–10 Å. Use tools like **ResMap** or the local resolution estimation in RELION to assess local quality. A cryo-EM paper reporting "2.8 Å resolution" may mean the core enzyme active site is at 2.8 Å while flexible regulatory domains are at 6–8 Å — the same atomic model, with very different reliability across different regions.

## R-factor and R_free

The **R-factor** (crystallographic residual) measures the discrepancy between the observed diffraction data and the data calculated from the atomic model:

$$R = \frac{\sum_{hkl} \left|F_{\text{obs}} - F_{\text{calc}}\right|}{\sum_{hkl} F_{\text{obs}}}$$

For well-refined structures at ~2 Å resolution, R ≈ 0.15–0.22. However, R can be artificially reduced by overfitting — adding too many parameters (atoms, anisotropic B-factors) to fit noise in the data. A crystallographer who adds spurious atoms to the model to make the R-factor look better is essentially memorizing the training data. This is where R_free comes in.

**R_free** (Brünger, 1992) is calculated on a set of ~5% of reflections randomly excluded from refinement ("test set"), making it immune to overfitting. It is the true measure of model quality. Think of it exactly like the train/test split in machine learning — R is training error, R_free is test error. Guidelines:

- R_free < 0.25 (at 2 Å): Acceptable
- R_free − R < 0.05: No significant overfitting
- R_free − R > 0.10: Overfitting suspected; use with caution

The PDB header contains both values (e.g., `_refine.ls_R_factor_R_work` and `_refine.ls_R_factor_R_free` in mmCIF).

## B-factors (Temperature Factors)

The **B-factor** (Debye-Waller factor, also called the temperature factor or atomic displacement parameter) quantifies the spread of atomic position around its mean:

$$B = 8\pi^2 \langle u^2 \rangle$$

where $\langle u^2 \rangle$ is the mean-square displacement of the atom in Å². High B-factors (>80 Å²) indicate either genuine atomic mobility (flexible loops, disordered termini) or poor electron density fit. Low B-factors (<10 Å²) indicate well-ordered, confidently placed atoms.

B-factors give you a spatial map of confidence. The active site of an enzyme should have low B-factors if the protein is well-behaved — catalytic residues that are genuinely mobile would not function well. Terminal regions and surface loops typically have high B-factors. If a binding site residue has an anomalously high B-factor, treat its position with skepticism regardless of the overall structure resolution.

**Normalized B-factors** (Z-scores relative to the mean and standard deviation of the structure) are useful for identifying unusually mobile or ordered regions. Coloring a structure by B-factor in PyMOL immediately reveals flexible regions:

```
# In PyMOL:
spectrum b, blue_white_red, minimum=0, maximum=100
```

## Ramachandran Plot Quality

The **Ramachandran plot** shows the backbone dihedral angles φ (phi) and ψ (psi) for each non-glycine, non-proline residue. Sterically favored regions (allowed conformations) are well-defined from first principles: α-helices cluster at φ ≈ −57°, ψ ≈ −47°; β-sheets at φ ≈ −120°, ψ ≈ +130°. Residues in disallowed regions of the Ramachandran plot are structural errors unless supported by very high-resolution electron density.

The Ramachandran plot is deeply satisfying as a quality check because it derives from basic chemistry, not empirical training data. The forbidden regions are sterically impossible — the protein backbone simply cannot adopt those dihedral angles without clashing. A residue in a "disallowed" region is almost certainly misplaced in the electron density map.

**MolProbity** (Richardson lab, Duke University) is the standard validation server. It reports:
- **Ramachandran outliers**: Fraction of residues outside allowed regions (< 0.5% in a good structure).
- **Rotamer outliers**: Fraction of side chain conformations inconsistent with the rotamer library (< 1% in a good structure).
- **Clashscore**: Number of serious steric clashes per 1,000 atoms (< 20 is acceptable; < 5 is excellent). Clashes indicate incorrect atom placement.
- **MolProbity score**: A composite Z-score combining all metrics.

## PDB-REDO

**PDB-REDO** (Vriend lab, CMBI) provides re-refined and updated versions of all PDB structures, automatically addressing common errors found by modern validation tools. The PDB-REDO pipeline applies state-of-the-art refinement protocols and geometric libraries to every deposited structure, often substantially improving Ramachandran statistics and clashscore relative to the originally deposited model.

It turns out that many older PDB structures were refined with outdated software and geometric constraints. Re-refining them with modern tools routinely reveals and corrects errors that have been silently present for years. Using PDB-REDO structures instead of the original deposited structures significantly improves the reliability of downstream structural analyses, and for any structure deposited before ~2010, it is worth checking PDB-REDO first.

## Practical Validation Workflow

Before using any PDB structure, treat this as a checklist, not a suggestion:
1. Check resolution and R/R_free in the PDB header.
2. Run MolProbity online or locally.
3. Visually inspect in PyMOL: check for unexplained electron density, appropriate bond lengths, and absence of gross structural errors.
4. For drug discovery: check that the ligand occupancy is 1.0 and B-factors are not anomalously high.

## Why This Matters

Using a low-quality PDB structure with incorrect side chain positions for drug design or mechanistic analysis can lead to completely wrong conclusions about binding contacts and drug optimization. Structure quality assessment is therefore a non-negotiable first step in any structural bioinformatics workflow. The protein folding problem may have been solved by AlphaFold2, but the structure quality assessment problem remains entirely human — machines can generate structures, but understanding which parts of those structures to trust still requires the kind of informed skepticism that this section is designed to cultivate.
