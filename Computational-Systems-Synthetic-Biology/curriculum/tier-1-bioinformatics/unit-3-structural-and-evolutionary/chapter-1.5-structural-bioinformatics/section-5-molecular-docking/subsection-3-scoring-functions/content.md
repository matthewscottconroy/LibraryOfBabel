# Docking Scoring Functions

At the heart of every docking program is a function that takes a protein-ligand configuration — specific 3D coordinates for every atom of both the receptor and the ligand — and returns a number estimating how favorable that configuration is. This **scoring function** is simultaneously the most important and the most poorly understood component of molecular docking. Finding the correct binding pose is only useful if the scoring function can distinguish the true pose from incorrect ones, and rank true binders above non-binders in virtual screening.

Scoring functions represent the major unresolved challenge in structure-based drug discovery. The problem is not lack of understanding — we know what the relevant physical interactions are: van der Waals packing, electrostatics, hydrogen bonds, hydrophobic burial, entropic penalties. The problem is accurate and fast computation. Rigorous calculation of protein-ligand binding free energy using quantum mechanics or explicit-solvent molecular dynamics takes days of computation per compound. Docking scoring functions must run in milliseconds. Something has to give, and what gives is accuracy.

## Force-Field-Based Scoring Functions

Force-field-based scoring functions sum the non-bonded interaction energies between the protein and ligand, using molecular mechanics energy terms:

$$\Delta G_{\text{bind}} \approx E_{\text{vdW}} + E_{\text{elec}} + E_{\text{desolv}}$$

**Van der Waals (Lennard-Jones) term**:

$$E_{\text{vdW}} = \sum_{i,j} \epsilon_{ij}\left[\left(\frac{r_{ij}^{\min}}{r_{ij}}\right)^{12} - 2\left(\frac{r_{ij}^{\min}}{r_{ij}}\right)^6\right]$$

The attractive $r^{-6}$ term captures dispersion/London forces; the repulsive $r^{-12}$ term prevents atomic overlap. The minimum-energy distance $r_{ij}^{\min}$ and well depth $\epsilon_{ij}$ are tabulated for each atom pair in the force field.

**Electrostatics (Coulomb)**:

$$E_{\text{elec}} = \sum_{i,j} \frac{q_i q_j}{\epsilon r_{ij}}$$

where $q_i, q_j$ are partial charges on atoms $i$ (protein) and $j$ (ligand), $\epsilon$ is the dielectric constant (often a distance-dependent dielectric to implicitly account for screening).

**Desolvation**: The energy cost of removing water from the protein binding site and the ligand surface upon binding. Commonly approximated by atomic solvation parameters × solvent-accessible surface area change.

Force-field scoring is physically motivated and interpretable. You can decompose the score into individual interaction terms and ask "which residue is contributing most to binding?" But it handles solvation poorly — water is one of the dominant determinants of binding affinity, and a surface-area approximation is a crude model of what is really a complex thermodynamic process.

**Programs using FF scoring**: AutoDock (3.x, before Vina), DOCK.

## Empirical Scoring Functions

**Empirical scoring functions** fit weighted linear combinations of structural terms to a training set of protein-ligand complexes with known $\Delta G$ values. Typical terms include: hydrogen bond count, metal-ligand interactions, hydrophobic contact area, rotatable bond penalty (conformational entropy loss), and ring planarity. Coefficients are fit by regression.

- **AutoDock Vina**: Uses an empirical function with Gaussian steric interactions, repulsion, hydrogen bonding, hydrophobic, and rotational penalty terms. Coefficients fit to ~85 protein-ligand complexes from PDBBIND. Fast and accurate; one of the most widely validated programs.
- **Glide SP/XP** (Schrödinger): Standard Precision (SP) is empirical; Extra Precision (XP) adds penalty terms for poses that lack specific structural features expected for strong binders. Glide SP is the most widely used commercial docking program.

The strength of empirical scoring is that it is tuned directly on experimental data. The weakness is that it is only as good as its training set — a scoring function trained primarily on kinases will perform best on kinase-like targets, and may perform poorly on targets with unusual binding site chemistry.

## Knowledge-Based Scoring Functions

**Knowledge-based (or statistical potential) scoring functions** derive potentials of mean force from the PDB: they count the frequency of observing each protein atom type within a given distance of each ligand atom type, and convert this to a free energy using:

$$\Delta G_{ij}(r) = -k_BT \ln\left[\frac{f_{ij}(r)}{f_{ij}^{\text{ref}}(r)}\right]$$

where $f_{ij}(r)$ is the observed pair frequency and $f_{ij}^{\text{ref}}(r)$ is the expected frequency under a reference distribution (random mixing). Atom pairs that are observed together more than randomly correspond to favorable interactions (negative $\Delta G$). 

Examples: **DrugScore**, **PMF** (Potentials of Mean Force), **X-Score**.

Knowledge-based functions exploit the enormous information content of the PDB — thousands of structures of protein-ligand complexes represent billions of years of evolutionary and chemical optimization. If a particular protein-ligand atomic contact is common in the PDB, it is probably favorable. This reasoning is sound but indirect — the PDB is not a random sample of all possible protein-ligand interactions.

## Machine Learning Scoring Functions

Recent years have seen ML-based scoring functions trained on large databases of protein-ligand structures with experimental affinities (PDBBIND ~20,000 complexes):

- **RF-Score**: Random forest trained on protein-ligand pairwise atom-count features. Outperforms classical scoring functions on CASF benchmarks.
- **NNScore**: Neural network scoring function using the same features.
- **ΔΔG from structure**: Graph neural networks (e.g., SchNet, DimeNet++) that operate directly on atomic coordinates and learn energy functions from quantum chemical data.

**Gnina**: Integrates a CNN scoring function trained on docking poses (distinguishing correct from incorrect poses) with AutoDock Vina's sampling. Substantially improves re-docking success rates.

**DiffDock** (Corso et al., 2022): A diffusion model that predicts binding poses without a docking grid; approaches or exceeds traditional docking on blind pose prediction benchmarks.

DiffDock is conceptually striking: it learns to generate binding poses using the same diffusion framework that DALL-E uses to generate images. This connection to generative AI suggests that the boundary between "search algorithms" and "generative models" is blurring in structural prediction, just as it has in image and protein design.

## Rank Order vs. Absolute Affinity Prediction

A crucial distinction:
- **Rank ordering**: Scoring functions are reasonably good at ranking a set of compounds docked to the same target — enriching actives in the top of the list (EF = 5–20 at 1%).
- **Absolute affinity**: Docking scores are poor predictors of absolute $K_d$ or IC₅₀ values. The gap between docking score (in kcal/mol) and experimental IC₅₀ is enormous — the Pearson correlation between Glide score and measured affinity is typically r ≈ 0.3–0.5 across diverse compound sets.

This means: **never use docking scores to predict whether a compound is a "nanomolar" vs. "micromolar" binder** without experimental validation. Use scores only for relative ranking. This is a lesson that even experienced computational chemists need to relearn periodically — it is tempting to trust the numbers, and the numbers will mislead you.

## MM-GBSA Rescoring

**MM-GBSA** (Molecular Mechanics Generalized Born Surface Area) is a post-docking rescoring method that is more physically rigorous than standard docking scores. After docking, a short energy minimization of each pose is performed, and the binding free energy is estimated as:

$$\Delta G_{\text{bind}}^{\text{MM-GBSA}} = \Delta H_{\text{MM}} + \Delta G_{\text{GB}} + \Delta G_{\text{SA}} - T\Delta S$$

MM-GBSA rescoring correlates better with experimental affinities than docking scores (r ≈ 0.5–0.7 for congeneric series) but requires ~1–10 CPU minutes per compound — too slow for primary screening of millions of compounds but appropriate for rescoring the top 1% of a primary screen.

## Why This Matters

The accuracy of the scoring function ultimately determines whether virtual screening identifies real drugs — understanding why current scoring functions fail (rigid receptor assumption, implicit solvation, limited training data) motivates the development of next-generation ML-based methods and sets appropriate expectations for what computational docking can currently achieve. The progression from force-field to empirical to knowledge-based to machine learning scoring functions is a story of gradually closing the gap between computational speed and physical accuracy — a gap that is still significant, but narrowing with each generation of methods.
