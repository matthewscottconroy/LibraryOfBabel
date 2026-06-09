# Stereochemistry

The drug thalidomide was prescribed in the late 1950s as a sedative for pregnant women suffering from morning sickness. It was sold as a racemic mixture — two mirror-image forms of the same molecule. One enantiomer, the R-form, provided the desired sedation. The other, the S-form, was teratogenic, causing severe limb malformations in thousands of infants before the drug was withdrawn. What made the tragedy irreversible was a chemical fact: thalidomide racemizes in the body, converting back and forth between the two forms. Even if you could isolate the "safe" enantiomer, the body would recreate the toxic one.

This story captures something fundamental about chemistry and biology: molecules are three-dimensional objects, and their three-dimensional shape determines their biological activity. The lesson of thalidomide was eventually written into FDA regulations requiring that chiral drugs be tested as individual enantiomers. The deeper lesson is that stereochemistry is not a detail — it is a fundamental property of every biological molecule.

Stereochemistry is the study of the three-dimensional arrangement of atoms in molecules. Biology is fundamentally chiral: evolution has selected for a world of L-amino acids and D-sugars, and the enzymes and receptors that evolved to recognize these molecules are exquisitely sensitive to stereochemistry. Understanding stereochemistry is essential for interpreting molecular recognition, enzyme specificity, drug binding, and the design of bioactive molecules.

## Chirality and Enantiomers

A molecule is **chiral** (Greek: "hand") if it is non-superimposable on its mirror image. The most common source of chirality is a **stereocenter** (chiral center) — a carbon atom bonded to four different substituents.

**Enantiomers** are the two mirror-image forms of a chiral molecule. They have identical physical properties (melting point, solubility, spectroscopic properties) in achiral environments, but rotate plane-polarized light in opposite directions:
- $(+)$ or $d$: dextrorotatory (rotates light clockwise)
- $(-)$ or $l$: levorotatory (rotates light counterclockwise)

The **R/S naming system** (Cahn-Ingold-Prelog): assign priorities 1-4 to substituents by atomic number. View from opposite the lowest-priority group. If the 1→2→3 sequence goes clockwise: **R** (rectus); counterclockwise: **S** (sinister).

**The biological world is asymmetric:**
- All 20 standard amino acids (except glycine) are **L-amino acids** (S configuration at the $\alpha$-carbon, with the exception of cysteine which is R due to the high atomic number of sulfur)
- All sugars in nucleic acids and glycolysis are **D-sugars** (R configuration at the highest-numbered stereocenter)
- This homochirality is thought to have originated stochastically and then been amplified by autocatalytic processes early in life's history

## Diastereomers and Meso Compounds

A molecule with $n$ stereocenters has up to $2^n$ stereoisomers. Stereoisomers that are not enantiomers are **diastereomers** — they can have different physical, chemical, and biological properties.

**Example — threonine:** Has two stereocenters (C2 and C3). The four stereoisomers are: L-Thr, D-Thr, L-allo-Thr, D-allo-Thr. Only L-Thr is incorporated into proteins.

**Meso compounds:** Molecules with stereocenters that are internally symmetric — they have non-superimposable mirror images that are actually identical due to an internal plane of symmetry. Meso compounds are achiral despite having stereocenters. Meso-tartaric acid is the classic example.

## R/S Nomenclature in Metabolic Intermediates

Stereochemistry is not just a naming convention — it determines biochemical function:

- **L-malate vs D-malate:** The TCA cycle enzyme fumarase specifically produces L-malate; D-malate is not a substrate for subsequent TCA enzymes
- **NADH vs NADPH:** Both are reduced nicotinamide cofactors, but NADH is used primarily in catabolic reactions (reoxidized by the electron transport chain) while NADPH is used in anabolic reactions (used as a reductant in biosynthesis)
- The hydride transfer from NAD$^+$/NADP$^+$ occurs specifically to the *pro-R* or *pro-S* face of the nicotinamide ring, depending on the enzyme — this stereospecificity is exploited in isotope labeling experiments

## Cis/Trans Isomerism

**Geometric isomers** (cis/trans) arise when rotation around a bond is restricted — most commonly in carbon-carbon double bonds.

**Fatty acid geometry:**
- **Saturated fatty acids:** No double bonds; all C-C bonds allow free rotation; can adopt an extended conformation and pack tightly → high melting point, solid at room temperature (e.g., palmitic acid, butter)
- **cis-unsaturated fatty acids:** The double bond introduces a ~30° kink in the carbon chain; packed chains cannot align tightly → lower melting point, liquid at room temperature (e.g., oleic acid, olive oil)
- **trans-unsaturated fatty acids:** The double bond does not kink the chain; packs like saturated fats → higher melting point; associated with cardiovascular disease (partially hydrogenated oils)

**Proline and protein backbone:** Proline is unique among amino acids in that its peptide bond can exist in both cis and trans configurations (other amino acids are almost exclusively trans). Cis-proline creates a sharp turn in protein structure and is kinetically trapped — cis-trans isomerization is slow (minutes) and catalyzed by **prolyl isomerases** (immunophilins, cyclophilin, FKBP12).

You might wonder why trans-fats are harmful if saturated fats are also solid at room temperature. The answer is partly metabolic: the body has evolved enzymes that metabolize cis unsaturated fatty acids efficiently, but handles trans fats poorly. Trans fats elevate LDL cholesterol while lowering HDL cholesterol — a doubly bad outcome. The geometry of a double bond, a seemingly minor structural detail, has turned out to have major public health consequences.

## Protein Active Sites as Chiral Environments

Enzyme active sites are chiral environments. Because they are made of L-amino acids with specific three-dimensional arrangements, they present a chiral binding pocket that can discriminate between enantiomers. This explains:

- **Enantiospecificity:** Most enzymes act on only one enantiomer of their substrate. L-lactate dehydrogenase only uses L-lactate; D-lactate is not a substrate (but some bacteria have D-lactate dehydrogenase)
- **Drug chirality:** Many drugs are chiral. Often only one enantiomer is biologically active (the "eutomer"); the other (distomer) may be inactive or even harmful. Thalidomide's tragedy: the R-enantiomer is a sedative; the S-enantiomer is teratogenic — and the drug racemizes in vivo
- **Directed evolution for enantioselective catalysis:** Engineering enzymes for asymmetric synthesis of chiral drug intermediates is a major area of biotechnology

## Why This Matters for Computational Biology

Stereochemistry is essential in computational chemistry and molecular modeling. In docking simulations, the correct stereochemistry of the ligand must be specified — an incorrect enantiomer will not fit the binding pocket correctly, or will bind in an entirely different mode. In SMILES notation and molecular fingerprints used in cheminformatics and drug discovery, stereochemistry is encoded explicitly. In homology modeling and protein structure prediction, the chirality of the template structure must be preserved. In metabolic flux analysis, stereospecific reactions (like the malate/fumarate distinction) must be correctly annotated to avoid incorrect predictions. Stereochemistry is not a detail — it is a fundamental property of every biological molecule.
