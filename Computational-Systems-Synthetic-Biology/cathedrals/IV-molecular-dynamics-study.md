# Cathedral IV: A Molecular Dynamics Study of a Protein of Interest

---

## The Question

How does protein conformational dynamics relate to function — and can computational simulation predict mutations that alter that function?

---

## Prerequisites

- [Tier 0.2](../curriculum/tier-0-bedrock/0.2-chemistry.md): Protein structure, enzyme kinetics
- [Tier 1.5](../curriculum/tier-1-bioinformatics/1.5-structural-bioinformatics.md): PDB, homology modeling, AlphaFold2
- [Tier 4.3](../curriculum/tier-4-computational-tools/4.3-molecular-dynamics.md): MD theory and practice

---

## The Project

### Phase 1: Protein Selection and Motivation

1. Choose a protein with biological and computational interest:
   - **Synthetic biology**: an enzyme used in metabolic engineering (e.g., isoprene synthase, farnesyl diphosphate synthase)
   - **Drug target**: a kinase or protease with known inhibitors (well-validated against)
   - **CRISPR engineering**: Cas9 or a Cas variant — conformational changes upon guide RNA binding

2. Biological question to answer with MD:
   - What conformational change occurs upon substrate/inhibitor binding?
   - Which residues are important for catalysis or selectivity?
   - How do known mutations affect protein dynamics?
   - What is the allosteric communication pathway between two sites?

3. Literature survey: what is known about the protein's mechanism from structural and biochemical studies?

### Phase 2: Structure Preparation

4. Obtain starting structure:
   - Best: high-resolution X-ray structure (<2 Å) with substrate/inhibitor bound
   - Acceptable: AlphaFold2 prediction (confirm with pLDDT; model flexible loops carefully)
   - Multiple conformations: use ensemble starting points for conformational sampling

5. Structure quality assessment:
   - MolProbity: Ramachandran outliers, clashscore
   - Missing residues: model with MODELLER or manually
   - Protonation: PropKa calculation; manually verify active site protonation state

6. System preparation (GROMACS or AMBER):
   - Add hydrogens (pdb2gmx or tleap)
   - Solvate in rhombic dodecahedron, 12 Å protein-to-edge distance
   - Add 150 mM NaCl
   - Force field: CHARMM36m for proteins (best current parameterization for IDRs and loops)

### Phase 3: Simulation Protocol

7. Energy minimization: steepest descent until Fmax < 1000 kJ/mol/nm

8. NVT equilibration (100 ps, backbone restrained)

9. NPT equilibration (500 ps, backbone restrained)

10. Progressive release of restraints (1 ns with decreasing k)

11. Production MD:
    - At minimum: 100–500 ns total
    - Better: 3 independent replicates of 100–200 ns each (more reliable statistics than one long run)
    - GPU-accelerated: modern GPU does ~100 ns/day for a protein in water (~50,000 atoms)

12. Trajectory output: save every 10–100 ps depending on analysis needs

### Phase 4: Basic Analysis

13. Equilibration assessment:
    - RMSD vs. time: confirm plateau after ~10-50 ns
    - Temperature, pressure, density: confirm stability
    - Potential energy: no drift

14. Structural analysis:
    - RMSD per residue (RMSF): identify flexible vs. rigid regions; compare to B-factors
    - Secondary structure content: is the fold maintained?
    - Key contact distances: track active site geometry over time

15. Comparison to known structure:
    - If substrate-bound simulation: does substrate remain in binding site?
    - If multiple conformations known: does simulation sample both?

### Phase 5: Advanced Analysis

16. Principal Component Analysis (PCA) of trajectory:
    - Major collective motions captured in first few PCs
    - Visualize: project trajectory on PC1/PC2 plane
    - Identify: conformational clusters, transitions, closed/open states

17. Dynamic Network Analysis (protein structure network):
    - Build residue contact network from trajectory contacts
    - Community detection: identify dynamically coupled modules
    - Critical pathways (betweenness): allosteric communication paths from one site to another

```python
from networkx import betweenness_centrality
import MDAnalysis as mda
import numpy as np

# Build contact matrix from trajectory
u = mda.Universe('topology.tpr', 'trajectory.xtc')
protein = u.select_atoms('protein and name CA')
n_res = len(protein.residues)
contact_frequency = np.zeros((n_res, n_res))

for ts in u.trajectory[::10]:  # every 10th frame
    positions = protein.positions
    for i in range(n_res):
        for j in range(i+4, n_res):
            dist = np.linalg.norm(positions[i] - positions[j])
            if dist < 8.0:  # 8 Å threshold
                contact_frequency[i, j] += 1

contact_frequency /= len(u.trajectory[::10])
```

18. Free energy analysis (optional):
    - If sampling allows: construct PMF along a reaction coordinate
    - Umbrella sampling for: binding pocket opening, substrate positioning, product release
    - WHAM analysis of PMF

### Phase 6: Mutation Predictions

19. Design mutations based on simulation insights:
    - Residues showing high RMSF in an otherwise-rigid region: may be important for dynamics
    - Highly connected network nodes: potential allosteric communication hubs
    - Active site contacts: residues consistently within catalytic distance

20. Simulate mutant proteins:
    - Create point mutant with GROMACS pdb2gmx or AMBER tleap
    - Run same protocol as wildtype
    - Compare: RMSD, RMSF, active site geometry, network structure

21. Predict functional consequence:
    - Changed active site geometry → altered substrate positioning → changed catalysis
    - Changed flexibility → altered substrate binding/release kinetics
    - Disrupted allosteric pathway → altered regulatory response

22. Validate against known data:
    - Find published mutations in this protein from alanine scanning or directed evolution
    - Does your simulation predict the known phenotypes?

---

## Expected Output

- Complete simulation protocol and analysis scripts in GitHub
- MD trajectory analysis figures: RMSD, RMSF, PCA, contact maps
- Network analysis: allosteric pathway identification
- Mutation predictions with mechanistic rationale
- Comparison to published biochemical data

---

## Key Tools

- System preparation: GROMACS (pdb2gmx), AMBER (tleap), VMD (psf generation)
- MD simulation: GROMACS (primary), NAMD or OpenMM (alternatives)
- Analysis: MDAnalysis (Python), CPPTRAJ (AMBER), VMD
- PCA: MDAnalysis PCA module
- Network analysis: NetworkX, dynetan (Python)
- Visualization: VMD, PyMOL, UCSF ChimeraX
