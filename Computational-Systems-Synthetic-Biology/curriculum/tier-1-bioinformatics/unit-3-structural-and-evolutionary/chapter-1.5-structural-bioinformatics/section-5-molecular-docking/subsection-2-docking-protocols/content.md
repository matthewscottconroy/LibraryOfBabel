# Docking Protocols

Knowing that docking exists is not enough to use it effectively. Successful molecular docking depends critically on careful preparation of both the protein receptor and the small molecule ligand before running the docking algorithm. Each preparation step has a direct impact on docking success rate, and poorly prepared inputs are a leading cause of incorrect docking results — probably more often than the docking algorithm itself is at fault.

Here is a useful way to think about this: the docking algorithm is only as good as its inputs. If your protein is missing hydrogens, the hydrogen-bond network in the binding site will be wrong, and the scoring function will evaluate poses in a physically unrealistic environment. If your ligand has the wrong protonation state, every pose will have incorrect charges and the electrostatic contribution to scoring will be systematically wrong. These are not subtle errors; they are the structural biology equivalent of building on a bad foundation.

## Step 1: Binding Site Identification

The docking algorithm must know where to search in the protein's 3D structure. Possible approaches:

**Known binding site**: If the structure contains a bound ligand, its position defines the binding site. The docking grid box is centered on the ligand's centroid. This is the most reliable approach.

**Structural comparison**: If a homologous protein has a known binding site, the corresponding pocket in the query structure can be inferred by structural superposition.

**Computational site prediction**:
- **SiteMap** (Schrödinger): Uses a grid-based algorithm to identify protein surface concavities with favorable properties (enclosed volume, hydrophobic contact, hydrogen bonding potential). Reports a SiteScore and druggability score.
- **fpocket** (open source): Voronoi tessellation of the protein surface to identify and characterize pockets. Fast, suitable for large-scale analyses.
- **DoGSiteScorer**: Graph-based pocket detection with druggability scoring.

It turns out that not all pockets are equal. A protein may have many surface cavities, but most are shallow, polar, or too small to accommodate drug-like molecules. The druggability score — which estimates how likely a pocket is to support high-affinity binding by small molecules — is an important filter. The presence of a pocket is necessary but not sufficient for successful drug discovery.

## Step 2: Receptor Preparation

Starting from a PDB structure, the following preparation steps are required:

**Add hydrogen atoms**: Crystal structures typically lack hydrogen atoms (invisible at standard X-ray resolutions). All polar hydrogens must be added at appropriate positions for the crystallographic pH (~7.4 for most biological complexes). Tools: **Schrödinger Protein Preparation Wizard**, **OpenBabel**, **Maestro**.

**Assign charges**: Partial atomic charges (for force-field-based scoring) must be assigned. The **OPLS4** or **AMBER** force field parameter sets are standard.

**Handle protonation states**: Histidine can be neutral (HID, HIE) or positively charged (HIP) depending on local environment. Aspartate, glutamate, arginine, and lysine protonation states at pH 7.4 must be assigned correctly (pKa prediction tools: PROPKA, H++).

**Remove waters** (usually): Water molecules in the binding site are typically removed for standard docking, except when a structural water is conserved across crystal structures and known to be critical for ligand binding (e.g., the "structural water" in hinge-binding kinase inhibitors).

**Fix structural errors**: Address alternate conformations, missing side chains, and chain breaks (see Section 1.3 on structure quality).

Each of these steps can substantially affect docking outcomes. A misassigned histidine protonation state can flip a hydrogen bond from favorable to unfavorable. A retained water molecule that is actually displaced by ligand binding adds a desolvation cost that is not captured by naive rigid docking. Receptor preparation is where domain knowledge of the biology matters — you need to understand the binding site chemistry to make good decisions.

## Step 3: Ligand Preparation

Docking requires a 3D all-atom representation of the ligand with appropriate geometry and charges:

**3D conformer generation**: Convert the SMILES string or 2D structure to 3D by geometry optimization. Tools: RDKit (`AllChem.EmbedMolecule`), OpenBabel, Omega (OpenEye).

**Protonation state**: The ligand must be protonated for physiological pH (7.4). Basic amines (pKa ~8–10) will be protonated (positively charged); carboxylic acids (pKa ~4–5) will be deprotonated (negatively charged). **Epik** (Schrödinger) or **Dimorphite-DL** enumerate tautomers and protonation states.

**Tautomers**: Many drug-like compounds can exist as multiple tautomers. Docking should ideally sample all relevant tautomers.

**Charge assignment**: Assign partial charges (AM1-BCC, MMFF94, Gasteiger) for use in the force-field scoring function.

```python
from rdkit import Chem
from rdkit.Chem import AllChem

mol = Chem.MolFromSmiles('CC(=O)Nc1ccc(O)cc1')  # Acetaminophen
mol = Chem.AddHs(mol)
AllChem.EmbedMolecule(mol, randomSeed=42)
AllChem.MMFFOptimizeMolecule(mol)
Chem.MolToMolFile(mol, 'acetaminophen_3D.mol')
```

## Step 4: Sampling Algorithm

Different docking programs use different algorithms to explore the pose space:

**Systematic search**: Enumerate all rotatable bond conformations at fixed increments. Exact but exponentially slow with increasing degrees of freedom.

**Monte Carlo (MC)**: Random perturbations to pose parameters, accepting moves that improve score (or with Boltzmann probability). Used by **AutoDock** legacy versions.

**Genetic Algorithm (GA)**: Evolve a population of ligand poses by crossover and mutation. Used by **AutoDock** and many commercial programs.

**Fragment-based**: Dock a rigid fragment of the ligand into the site, then grow/merge the full ligand using the fragment as an anchor. Used by **Glide** (Schrödinger) in its core algorithm.

The choice of sampling algorithm is less critical than the preparation steps for most applications — the major programs have been optimized sufficiently that their sampling is rarely the bottleneck. What limits docking accuracy in practice is almost always the rigid receptor approximation and the scoring function.

## AutoDock Vina: Standard Protocol

**AutoDock Vina** is the most widely used free docking program. Standard protocol:

```bash
# Prepare receptor and ligand with prepare_receptor4.py and prepare_ligand4.py
# or use Meeko (modern version)

# Define docking box centered on known binding site
# center_x/y/z = coordinates of binding site center
# size = box dimensions in Angstrom

vina --receptor receptor.pdbqt \
     --ligand ligand.pdbqt \
     --center_x -10.0 --center_y 5.0 --center_z 20.0 \
     --size_x 20 --size_y 20 --size_z 20 \
     --exhaustiveness 16 \
     --num_modes 9 \
     --out ligand_docked.pdbqt
```

The `--exhaustiveness` parameter controls how much sampling is performed (higher = more thorough but slower; default 8, typical for VS = 8–16). Output poses are ranked by Vina score (kcal/mol estimated binding free energy).

When running virtual screening at scale, the standard practice is to reduce exhaustiveness to 4–8 for the primary screen (trading accuracy for speed), then re-dock top hits at exhaustiveness 16–32 to get better pose estimates. This is the computational equivalent of a rough filter followed by careful re-evaluation — a recurring pattern in computational biology workflows.

## Why This Matters

Proper receptor and ligand preparation is as important as the docking algorithm itself — systematic errors in protonation states, missing hydrogens, or incorrect 3D geometry lead to consistently wrong binding poses regardless of scoring function quality, making preparation protocols the practical foundation of any successful docking campaign. In real drug discovery, failures in virtual screening campaigns are often traced not to the docking algorithm but to inadequate input preparation. The most technically sophisticated docking score means nothing if the receptor has the wrong histidine tautomer or the ligand has the wrong charge.
