# Nonribosomal Peptides and Polyketides

Every time a physician prescribes erythromycin for a respiratory infection, or a transplant surgeon administers cyclosporin to prevent rejection, or a cancer patient receives doxorubicin, the drug being administered was assembled by an enzyme complex the size of a small ribosome — a molecular assembly line that strings together building blocks one at a time without ever consulting a messenger RNA. These are the nonribosomal peptides and polyketides, two vast families of natural products that include some of the most important antibiotics (penicillin, erythromycin, vancomycin, rapamycin), antifungals (amphotericin B), immunosuppressants (cyclosporin A, FK506/tacrolimus), and antitumor agents (epothilone, bleomycin). Their biosynthesis by enormous multi-enzyme complexes makes them simultaneously the most structurally complex and the most challenging class of natural products to engineer.

## The NRPS Machinery

**Nonribosomal peptide synthetases (NRPS)** are modular megaenzymes that assemble peptide natural products without using ribosomes, tRNAs, or mRNA templates. Each module in an NRPS assembly line incorporates one amino acid into the growing peptide chain.

### Module Architecture

A minimal NRPS module contains three catalytic domains:

**Adenylation (A) domain**: recognizes and activates a specific amino acid (or non-proteinogenic amino acid) by forming an aminoacyl-AMP intermediate, then transferring it to the peptidyl carrier protein.

**Peptidyl Carrier Protein (PCP/T domain)**: phosphopantetheine-armed domain that tethers the activated amino acid as a thioester. The 4′-phosphopantetheine arm (from CoA) acts as a flexible tether allowing the substrate to move between domains.

**Condensation (C) domain**: catalyzes peptide bond formation by condensing the upstream peptide chain (donor) with the current amino acid (acceptor).

Additional optional domains introduce structural diversity:
- **Epimerization (E) domain**: converts L to D configuration
- **N-methylation (N-MT) domain**: N-methylates the amino acid
- **Cyclization (Cy) domain**: forms heterocyclic rings (thiazoline, oxazoline)

**Thioesterase (TE) domain**: at the C-terminus of the last module; releases the completed peptide by hydrolysis or cyclization.

### The Collinearity Rule

In canonical NRPS assembly lines, the number of modules equals the number of amino acids in the product, and modules are read in N-to-C order. This **collinearity** (or co-linearity) is the key property exploited for combinatorial biosynthesis: by rearranging, replacing, or adding modules, new peptide products can in principle be generated.

## The PKS Machinery

**Polyketide synthases (PKS)** assemble polyketide natural products from acyl-CoA extender units (malonyl-CoA or methylmalonyl-CoA) using a similar modular assembly-line architecture.

**Type I modular PKS** (like DEBS — 6-deoxyerythronolide B synthase for erythromycin):

Each module contains:
- **KS (ketosynthase)**: forms C-C bond between upstream chain and new extender unit
- **AT (acyltransferase)**: selects and loads extender unit (malonyl-CoA → β-keto group; methylmalonyl-CoA → β-methyl-β-keto group)
- **ACP (acyl carrier protein)**: tethers intermediates

Optional reductive domains (modify β-keto group after condensation):
- **KR (ketoreductase)**: β-keto → β-hydroxy
- **DH (dehydratase)**: β-hydroxy → enoyl (double bond)
- **ER (enoyl reductase)**: enoyl → fully reduced β-methylene

The presence or absence of KR, DH, and ER in each module determines the degree of reduction at each carbon position, creating the characteristic oxygenation pattern of each polyketide scaffold.

## Engineering NRPs and PKs: Combinatorial Biosynthesis

The modular architecture of NRPS and PKS suggests that **swapping modules or domains between systems** could generate libraries of new natural products — "combinatorial biosynthesis."

### Domain Swapping

The most targeted approach: exchange individual A, KS, or AT domains to change substrate specificity at one position in the natural product.

**Example**: replacing the AT domain in DEBS module 2 (which loads methylmalonyl-CoA) with an AT domain from a different PKS that loads malonyl-CoA changes one methyl branch to a hydrogen in erythromycin, generating a novel erythromycin analog.

**Challenge**: domain boundaries are not always clearly defined; linker regions between domains are critical for catalytic efficiency. Incorrect domain boundary choice reduces yield dramatically.

**SCHEMA-guided domain swapping**: uses structural information to identify optimal recombination points that minimize structural disruption — applied to PKS domains to identify domain boundaries at structural loop regions.

### Module Swapping

More ambitious: replace entire modules with modules from other NRPS/PKS systems. This changes one residue in the product peptide or adds/removes a hydroxyl group.

**Partial success examples**:
- Swapping A domain specificity in tyrocidine NRPS to produce analogs with non-natural amino acids at specific positions
- Module additions in erythromycin PKS to extend the polyketide chain

**Key limitation**: inter-module communication requires compatible "docking domains" at module termini. Mismatched docking domains fail to form productive inter-module complexes, drastically reducing yield. Engineering docking domain compatibility is an active research area.

### Heterologous Expression

The primary metabolic engineering approach for NRPs/PKs is heterologous expression: transfer the entire biosynthetic gene cluster (BGC) from the native producer (often a slow-growing Streptomyces or actinomycete) to a faster-growing, more tractable host.

**Common heterologous hosts**:
- *Streptomyces coelicolor* or *S. lividans*: well-studied actinomycetes with established genetic tools
- *E. coli*: fastest for genetic manipulation; requires careful codon optimization and precursor supply engineering (malonyl-CoA, methylmalonyl-CoA)
- *Bacillus subtilis*: Gram-positive, good secretion, some NRP biosynthesis capability

**Challenges in heterologous expression**:
- Phosphopantetheinylation: PCP and ACP domains require a dedicated phosphopantetheinyl transferase (PPTase) that may not be present in the heterologous host; co-express Sfp PPTase from *B. subtilis*
- Precursor supply: polyketide synthesis requires large amounts of malonyl-CoA and methylmalonyl-CoA. In *E. coli*, malonyl-CoA supply is limited (competes with fatty acid synthesis). Engineer acetyl-CoA carboxylase overexpression or knockout competing pathways.
- Chaperone requirements: large NRPS/PKS megaenzymes (>300 kDa) often require specific chaperones for proper folding

### Synthetic Biology of BGCs

Modern approaches generate and test many BGC variants using high-throughput methods:

1. Synthesize multiple BGC variants with different domain arrangements (whole-cluster synthesis)
2. Transform into heterologous host library
3. Screen products by LC-MS/MS
4. Identify novel compounds from mass/fragmentation pattern
5. Characterize active compounds by NMR

This has been applied to rapamycin analogs (replacing FKBP12-binding domain modules) and to generating erythromycin analogs with improved properties.

## Computational Design of NRPS

The **antiSMASH** tool (antibiotics and secondary metabolites analysis shell) automatically identifies BGCs in genome sequences and predicts the structure of the encoded natural product. By analyzing hundreds of genomes:
- Identifies novel BGCs that have not been characterized
- Predicts substrate specificity of A domains from active site residues (Stachelhaus code)
- Reconstructs the biosynthetic logic to predict the NRP/PK structure

These predictions guide experimental prioritization: which BGCs to express heterologously, which A domain changes to attempt for analog production.

## Why This Matters

NRPS and PKS megaenzyme pathways are the source of antibiotics that treat tens of millions of infections annually and immunosuppressants that enable organ transplantation. The combinatorial biosynthesis concept — that new drugs can be generated by rearranging nature's modular biosynthetic logic — has motivated 30 years of research. While the reality has been more difficult than initially anticipated (domain and module swaps often fail due to interface incompatibility), the concept has been validated in multiple systems and continues to improve as our understanding of the structural biology of NRPS and PKS deepens. The combination of genome mining (finding new BGCs), heterologous expression (making them tractable), and computational domain boundary prediction makes this one of the most active frontiers in natural product discovery and metabolic engineering.
