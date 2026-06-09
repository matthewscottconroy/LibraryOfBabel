# Coarse-Grained Molecular Dynamics

Consider the assembly of a viral capsid — 60 identical protein subunits spontaneously arranging themselves into a precise icosahedral shell around a genome. The entire process takes minutes in vitro but involves roughly 3 million atoms and proceeds on timescales of milliseconds to hours. No all-atom simulation will reach this phenomenon in any human lifetime of GPU time. Or consider a lipid raft — a cholesterol-enriched domain in the plasma membrane, perhaps 100 nanometers across, containing dozens of membrane proteins, emerging from the statistical mechanics of thousands of lipid molecules diffusing laterally over microseconds. The atoms are there; the detail is not needed. All-atom MD resolves every hydrogen atom and bond vibration — a level of detail that is frequently unnecessary and computationally prohibitive for studying processes that occur on microsecond-to-millisecond timescales or involve large complexes (membranes, ribosomes, viral capsids). **Coarse-grained (CG) models** group multiple atoms into single interaction sites ("beads"), reducing the number of degrees of freedom by 10–100× and enabling simulations that are 2–4 orders of magnitude faster.

## The Coarse-Graining Philosophy

The key insight is that biological processes at the mesoscale (membrane dynamics, large-scale protein conformational changes, assembly of macromolecular complexes) are governed by collective motions that do not depend on the precise position of every hydrogen atom. A CG model retains only the degrees of freedom relevant to the process of interest.

The trade-off is accuracy at short length scales: CG models cannot reproduce atomistic details of specific chemical interactions. The appropriate model always depends on the scientific question.

## The MARTINI Force Field

**MARTINI** (Marrink et al., 2007; MARTINI 3, 2021) is the most widely used CG force field for biomolecular simulations. It maps approximately 4 heavy atoms to a single bead:

$$\text{4 heavy atoms} \rightarrow \text{1 MARTINI bead} \approx 72 \text{ Da}$$

MARTINI defines a chemical space of bead types based on polarity and charge:

| Type | Example | Character |
|---|---|---|
| Q (charged) | +/- beads | Ionic head groups |
| P (polar) | N, O-containing | Backbone, polar sidechains |
| N (nonpolar) | Uncharged, no H-bond | Ether-type groups |
| C (apolar) | Aliphatic carbons | Hydrophobic core, lipid tails |

Interactions between beads are described by Lennard-Jones and Coulomb potentials with bead-type-dependent parameters. Bonds and angles are maintained by harmonic potentials, and protein backbone topology uses elastic network restraints (ENM) to maintain secondary structure.

```python
# Using MDAnalysis + MARTINI CG topology analysis
import MDAnalysis as mda
import numpy as np

# Load a MARTINI simulation of a DPPC membrane
u = mda.Universe("martini_membrane.tpr", "martini_trajectory.xtc")

# Lipid headgroup selection in MARTINI nomenclature
# For DPPC: PO4 bead = phosphate headgroup
headgroups = u.select_atoms("resname DPPC and name PO4")
print(f"System: {u.atoms.n_atoms} MARTINI beads")
print(f"Approximate atomistic atoms: {u.atoms.n_atoms * 4}")
print(f"DPPC lipid headgroups: {headgroups.n_atoms}")

# Membrane thickness: distance between upper and lower leaflet PO4 beads
def compute_membrane_thickness(universe, step=10):
    """Compute bilayer thickness from PO4 bead positions over trajectory."""
    headgroups = universe.select_atoms("resname DPPC and name PO4")
    thicknesses = []

    for ts in universe.trajectory[::step]:
        z_pos = headgroups.positions[:, 2]
        z_mid = z_pos.mean()
        upper = z_pos[z_pos > z_mid].mean()
        lower = z_pos[z_pos < z_mid].mean()
        thicknesses.append(upper - lower)

    thickness = np.mean(thicknesses)
    print(f"Membrane thickness: {thickness:.2f} Å (typical DPPC: ~38–40 Å)")
    return np.array(thicknesses)
```

## Building a MARTINI Membrane System

```python
import subprocess

def build_martini_membrane(lipid_composition, protein_pdb=None,
                           box_size_nm=15, thickness_nm=4):
    """
    Build a MARTINI membrane system using memgen or insane.py.
    lipid_composition: dict like {"DPPC": 0.7, "CHOL": 0.3}
    """
    # Format composition string for insane.py
    upper_leaflet = " ".join([f"-u {lip}:{frac}"
                               for lip, frac in lipid_composition.items()])
    lower_leaflet = upper_leaflet.replace("-u", "-l")

    cmd = ["python", "insane.py",
           "-f", protein_pdb or "none",
           upper_leaflet.split(),
           lower_leaflet.split(),
           "-sol", "W",         # MARTINI water
           "-salt", "0.15",     # 150 mM NaCl
           "-x", str(box_size_nm),
           "-y", str(box_size_nm),
           "-z", str(box_size_nm),
           "-o", "membrane.gro",
           "-p", "topol.top"]
    print(f"Building membrane: {lipid_composition}")
    # subprocess.run(cmd, check=True)  # uncomment to run
    print("  Output: membrane.gro, topol.top")
    print("  Next: energy minimization and equilibration (see CHARMM-GUI MARTINI builder)")
```

## MARTINI 3: Updated Parameters

MARTINI 3 (Souza et al., 2021) corrected several deficiencies of MARTINI 2:

- Overfitting of the protein-protein interaction energy: M3 correctly models protein aggregation thermodynamics
- Three bead sizes: regular (R), small (S), tiny (T) for better molecular geometry
- Improved small molecule parameters for drug-like compounds

```bash
# Install MARTINI 3 and run a protein-membrane simulation
# Using martinize2 for protein topology
pip install vermouth  # martinize2 dependency

martinize2 -f protein.pdb \
           -o protein_martini.top \
           -x protein_martini.pdb \
           -ff martini3001 \      # MARTINI 3.0.0
           -dssp mkdssp \         # DSSP for secondary structure
           -elastic \             # elastic network for secondary structure
           -ef 500 \              # elastic network force constant (kJ/mol/nm²)
           -el 0.5 \              # lower cutoff for elastic network (nm)
           -eu 0.9 \              # upper cutoff
           -p backbone            # position restraints on backbone beads
```

## Timescale Speedup: The CG Advantage

The speedup from CG MD has two components:

1. **Fewer particles**: $N_\text{CG} \approx N_\text{all-atom}/4$; pairwise force computation scales as $O(N^2)$ in the worst case (or $O(N)$ with cutoffs and neighbor lists), giving ~4–16× speedup

2. **Faster dynamics**: with smoother potential energy surfaces and larger time steps ($\Delta t_\text{CG} \approx 20$–40 fs), CG dynamics are intrinsically faster. MARTINI uses a **time conversion factor** $\tau^* \approx 4$: 1 ns CG = ~4 ns effective time

3. **Combined**: total speedup of 100–1000× over all-atom, enabling microsecond CG simulations in hours rather than months

```python
def estimate_speedup(n_atoms_aa=100000, dt_aa_fs=2, dt_cg_fs=20,
                     cg_time_factor=4):
    """
    Estimate effective speedup of MARTINI CG vs. all-atom MD.
    """
    n_atoms_cg = n_atoms_aa // 4  # MARTINI mapping

    # Force computation speedup (assuming O(N) with cutoffs)
    force_speedup = n_atoms_aa / n_atoms_cg

    # Time step speedup
    timestep_speedup = dt_cg_fs / dt_aa_fs

    # Effective time speedup (CG dynamics are faster)
    total_speedup = force_speedup * timestep_speedup * cg_time_factor

    print(f"All-atom: {n_atoms_aa} atoms, dt = {dt_aa_fs} fs")
    print(f"CG:       {n_atoms_cg} beads, dt = {dt_cg_fs} fs, "
          f"τ* = {cg_time_factor}")
    print(f"\nSpeedup components:")
    print(f"  Fewer particles:  {force_speedup:.0f}×")
    print(f"  Larger time step: {timestep_speedup:.0f}×")
    print(f"  Faster dynamics:  {cg_time_factor}×")
    print(f"  Total effective:  {total_speedup:.0f}×")
    print(f"\nEquivalent: 1 day of CG ≈ {total_speedup/365:.0f} years of all-atom")

estimate_speedup()
```

## Backmapping: CG to All-Atom

CG simulations can identify relevant conformations (membrane protein binding mode, large-scale conformational change); these can be back-converted to all-atom resolution for high-resolution analysis.

```bash
# Backward: CG to all-atom backmapping (Wassenaar et al.)
python backward.py -f cg_structure.gro \
                   -o aa_structure.gro \
                   -to charmm36 \       # target all-atom force field
                   -p topol.top

# After backmapping: energy minimization required to resolve clashes
gmx mdrun -v -deffnm em_backmapped -maxh 0.5
```

## Go-Like Models

**Go-like (Gō) models** are an even simpler CG approach specifically for studying protein folding. Each residue is represented as a single bead at the $C_\alpha$ position. Contacts present in the native structure are stabilizing (attractive LJ-like potential); non-native contacts are neutral or repulsive.

$$V_\text{Go} = \sum_\text{bonds} \frac{k_b}{2}(r - r_0)^2 + \sum_\text{native contacts} \varepsilon\left[5\left(\frac{\sigma}{r}\right)^{12} - 6\left(\frac{\sigma}{r}\right)^{10}\right]$$

Go-like models fold small proteins in microseconds, enabling:
- Identifying folding pathways and transition states
- Measuring mechanical unfolding response (compare to AFM experiments)
- Screening mutational effects on folding rates

## Why This Matters

Coarse-grained MD makes biologically important simulations tractable that would otherwise require decades of GPU time. Membrane biophysics — lateral diffusion of proteins in lipid bilayers, membrane curvature sensing, lipid raft formation, fusion pore dynamics — is almost entirely studied at the CG level because the relevant timescales (microseconds to milliseconds) and length scales (tens of nanometers) are inaccessible to all-atom MD. The assembly of viral capsids (millions of atoms, millisecond timescale), the mechanical properties of cytoskeletal polymers, and the conformational dynamics of intrinsically disordered proteins are all active areas where CG simulations provide insights unobtainable by any other computational approach.
