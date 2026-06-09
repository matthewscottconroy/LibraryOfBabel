# Secondary Structure Analysis

Helices and strands are not permanent features of a protein — they breathe. The classic image of an alpha helix as a rigid rod, stable and unchanging, is a crystallographic artifact. In solution, at body temperature, the ends of helices fray and re-form on nanosecond timescales. Loops connecting secondary structure elements can adopt multiple distinct conformations. And in intrinsically disordered proteins — now known to make up perhaps a third of the human proteome — secondary structure elements form transiently, existing only fractions of the time but playing critical roles in function. MD trajectories reveal how secondary structure elements form, break, and interconvert over time. Tracking secondary structure evolution is particularly important for disordered proteins, protein-protein interaction interfaces, and conformational changes associated with ligand binding or allosteric transitions.

## The DSSP Algorithm

**DSSP** (Dictionary of Secondary Structure of Proteins, Kabsch and Sander, 1983) is the standard algorithm for assigning secondary structure from 3D coordinates. It analyzes backbone hydrogen bonds to identify eight secondary structure classes:

| Code | Class | Description |
|---|---|---|
| H | α-helix | 4-turn helix; H-bond pattern $i \rightarrow i+4$ |
| G | 3$_{10}$-helix | 3-turn helix; $i \rightarrow i+3$ |
| I | π-helix | 5-turn helix; $i \rightarrow i+5$; rare |
| E | β-strand | Part of a β-sheet |
| B | β-bridge | Isolated β-bridge |
| T | Turn | H-bonded turn |
| S | Bend | Geometrically defined bend |
| C | Coil | Irregular; not assigned above |

The DSSP criteria are based on backbone H-bond strength (electrostatic energy < -0.5 kcal/mol) and $\phi$/$\psi$ dihedral angles.

## Computing DSSP Over a Trajectory

```python
import MDAnalysis as mda
from MDAnalysis.analysis.dssp import DSSP
import numpy as np
import matplotlib.pyplot as plt

u = mda.Universe("topology.tpr", "trajectory.xtc")
protein = u.select_atoms("protein")

# Run DSSP on all frames (or stride with step)
dssp = DSSP(protein).run(step=10, verbose=True)
# dssp.results.dssp: (n_frames, n_residues) array of secondary structure codes
ss_array = dssp.results.dssp

print(f"DSSP array shape: {ss_array.shape}")
print(f"Unique codes observed: {np.unique(ss_array)}")
```

## Visualizing Secondary Structure Evolution

```python
# Map DSSP codes to integers for plotting
ss_mapping = {"H": 0, "G": 1, "I": 2, "E": 3, "B": 4, "T": 5, "S": 6, "C": 7}
color_map = {0: "#E41A1C",  # helix: red
             1: "#FF7F00",  # 3-10 helix: orange
             2: "#984EA3",  # pi-helix: purple
             3: "#377EB8",  # strand: blue
             4: "#4DAF4A",  # bridge: green
             5: "#F781BF",  # turn: pink
             6: "#A65628",  # bend: brown
             7: "#FFFFFF"}  # coil: white

def encode_dssp(ss_array, mapping):
    encoded = np.zeros(ss_array.shape, dtype=int)
    for code, val in mapping.items():
        encoded[ss_array == code] = val
    return encoded

ss_encoded = encode_dssp(ss_array, ss_mapping)

# Create colormap
from matplotlib.colors import ListedColormap
cmap = ListedColormap([color_map[i] for i in range(8)])

fig, ax = plt.subplots(figsize=(14, 6))
im = ax.imshow(ss_encoded.T, aspect="auto", cmap=cmap, vmin=0, vmax=7,
               origin="lower",
               extent=[0, ss_array.shape[0] * 10 / 1000, 1,
                       ss_array.shape[1] + 1])  # time in ns, residue number

ax.set_xlabel("Time (ns)")
ax.set_ylabel("Residue")
ax.set_title("Secondary structure evolution")

# Colorbar with labels
cb = plt.colorbar(im, ax=ax, fraction=0.03)
cb.set_ticks([0.5, 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5])
cb.set_ticklabels(["α-helix", "3₁₀-helix", "π-helix", "β-strand",
                    "β-bridge", "Turn", "Bend", "Coil"])
plt.tight_layout()
plt.savefig("dssp_evolution.pdf", bbox_inches="tight")
```

## Helix and Strand Occupancy

For each residue, compute the fraction of simulation time spent in each secondary structure class:

```python
def compute_ss_occupancy(ss_array, ss_codes=("H", "G", "E"), n_residues=None):
    """
    Compute per-residue secondary structure occupancy.
    Returns: dict of {code: (n_residues,) occupancy array}
    """
    if n_residues is None:
        n_residues = ss_array.shape[1]
    n_frames = ss_array.shape[0]

    occupancy = {}
    for code in ss_codes:
        occupancy[code] = (ss_array == code).sum(axis=0) / n_frames

    return occupancy

occ = compute_ss_occupancy(ss_array)
residue_ids = np.arange(1, ss_array.shape[1] + 1)

fig, ax = plt.subplots(figsize=(12, 3))
ax.bar(residue_ids, occ["H"], label="α-helix", color="#E41A1C", alpha=0.8)
ax.bar(residue_ids, occ["E"], bottom=occ["H"], label="β-strand",
       color="#377EB8", alpha=0.8)
ax.set_xlabel("Residue")
ax.set_ylabel("Fraction of time")
ax.set_title("Secondary structure occupancy")
ax.set_ylim(0, 1)
ax.legend()
plt.tight_layout()

# Identify dynamic secondary structure elements (low occupancy = unstable)
unstable_helices = [(i+1, occ["H"][i])
                    for i in range(len(residue_ids))
                    if 0.2 < occ["H"][i] < 0.8]
if unstable_helices:
    print("Dynamically unstable helical residues (20-80% helix occupancy):")
    for resid, frac in unstable_helices:
        print(f"  Residue {resid}: {frac*100:.0f}% helix")
```

## Ramachandran Analysis from MD

The Ramachandran plot ($\phi$/$\psi$ dihedral angles) provides a complementary view of backbone conformations sampled during the simulation:

```python
from MDAnalysis.analysis.dihedrals import Ramachandran

# Compute phi/psi over trajectory
rama = Ramachandran(protein).run(step=5)
# rama.results.angles: (n_frames, n_residues-2, 2) — phi and psi for each residue

# Flatten over frames for plotting
phi = rama.results.angles[:, :, 0].flatten()
psi = rama.results.angles[:, :, 1].flatten()

fig, ax = plt.subplots(figsize=(5, 5))
ax.hexbin(phi, psi, gridsize=80, cmap="Blues", mincnt=1)
ax.set_xlabel("φ (degrees)")
ax.set_ylabel("ψ (degrees)")
ax.set_xlim(-180, 180)
ax.set_ylim(-180, 180)
ax.axhline(0, color="gray", lw=0.5)
ax.axvline(0, color="gray", lw=0.5)
ax.set_title("Ramachandran plot (all frames)")
plt.tight_layout()

# Fraction of residues in allowed regions
alpha_mask = (phi < -30) & (phi > -160) & (psi < 80) & (psi > -80)
beta_mask  = ((phi < -50) & (phi > -180) & (psi > 80)) | \
             ((phi < -50) & (phi > -180) & (psi < -150))
print(f"Alpha-helical region: {alpha_mask.mean()*100:.1f}%")
print(f"Beta region: {beta_mask.mean()*100:.1f}%")
```

## Comparing Secondary Structure to Experimental Data

Secondary structure predictions from CD spectroscopy give bulk fractions (% helix, % sheet, % random coil). These can be compared to MD time-averaged values:

```python
def compare_to_cd(ss_occupancy, cd_measurements):
    """
    Compare MD secondary structure content to CD spectroscopy.
    cd_measurements: dict with keys 'helix', 'sheet', 'coil' (fractions)
    """
    # MD averages over all residues
    md_helix = (ss_occupancy["H"] + ss_occupancy.get("G", 0)).mean()
    md_sheet  = ss_occupancy["E"].mean()
    md_coil   = 1 - md_helix - md_sheet

    print(f"Secondary structure comparison:")
    print(f"{'Metric':<20} {'MD simulation':>15} {'CD spectroscopy':>15}")
    print("-" * 52)
    for label, md_val, cd_val in [
        ("α-helix",   md_helix,  cd_measurements.get("helix", float("nan"))),
        ("β-sheet",   md_sheet,  cd_measurements.get("sheet", float("nan"))),
        ("Coil/other", md_coil,  cd_measurements.get("coil", float("nan")))
    ]:
        print(f"{label:<20} {md_val*100:>14.1f}%  {cd_val*100:>14.1f}%")

# Example usage
cd_data = {"helix": 0.35, "sheet": 0.20, "coil": 0.45}
compare_to_cd(occ, cd_data)
```

## Why This Matters

Secondary structure tracking directly connects MD simulations to biologically interpretable changes. Monitoring DSSP over a simulation of a disordered protein bound to a chaperone reveals which regions gain structure upon binding — a conformational selection or induced fit mechanism. For a drug-target simulation, observing the binding pocket loop transitioning from coil to helix upon ligand binding is an "induced fit" signal that pure docking cannot capture. Secondary structure analysis also serves as a sanity check: a protein that should be 40% helical but shows 5% helical content in simulation indicates a force field or preparation problem that should be addressed before proceeding to more sophisticated analyses.
