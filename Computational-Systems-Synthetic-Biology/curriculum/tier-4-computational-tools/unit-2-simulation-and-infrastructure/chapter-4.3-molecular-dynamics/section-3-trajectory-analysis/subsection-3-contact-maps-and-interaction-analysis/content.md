# Contact Maps and Interaction Analysis

Proteins are machines built from contacts. The hydrophobic core is a dense network of non-polar contacts stabilizing the tertiary fold. The active site of an enzyme is defined by the precise geometry of a handful of side chains held in contact by the rest of the protein. A drug binds because the contacts it makes with a binding pocket are collectively more favorable than the contacts it gives up with water. RMSD tells you how much the protein has moved; contact analysis tells you what molecular interactions have formed or broken. Contacts between residues encode the three-dimensional structure of a protein and reveal which interactions are formed, broken, or transient during a simulation. Contact maps, hydrogen bond networks, and salt bridge analysis provide a residue-level view of the simulation that complements global metrics like RMSD.

## Residue Contact Maps

A **contact map** is a matrix $C \in \{0,1\}^{N \times N}$ (or the continuous version $c_{ij} \in [0,1]$) where $c_{ij}$ is the fraction of simulation frames in which residues $i$ and $j$ are in contact. A common criterion is a distance threshold between $C_\alpha$ atoms (8 Å) or between heavy atoms (4–5 Å).

$$c_{ij} = \frac{1}{T}\sum_{t=1}^{T} \mathbf{1}\left[d_{ij}(t) < d_\text{cutoff}\right]$$

```python
import MDAnalysis as mda
import numpy as np
import matplotlib.pyplot as plt
from MDAnalysis.lib.distances import distance_array

def compute_contact_map(universe, selection="name CA",
                        cutoff=8.0, stride=10, seq_sep=4):
    """
    Compute pairwise contact frequency over trajectory.
    seq_sep: minimum sequence separation (exclude i,i+1,i+2,i+3 pairs)
    Returns: contact_freq (n_res, n_res) and residue IDs
    """
    ag = universe.select_atoms(selection)
    n_res = ag.n_atoms

    contact_count = np.zeros((n_res, n_res), dtype=np.float32)
    n_frames = 0

    for ts in universe.trajectory[::stride]:
        pos = ag.positions                          # (n_res, 3)
        dist = distance_array(pos, pos, box=ts.dimensions)  # (n_res, n_res), Å

        # Contacts: distance < cutoff, separated by >= seq_sep
        contact_frame = (dist < cutoff).astype(np.float32)
        np.fill_diagonal(contact_frame, 0)
        # Zero out nearby sequence neighbors
        for k in range(1, seq_sep):
            np.fill_diagonal(contact_frame[k:, :], 0)
            np.fill_diagonal(contact_frame[:, k:], 0)

        contact_count += contact_frame
        n_frames += 1

    contact_freq = contact_count / n_frames
    residue_ids = [res.resid for res in ag.residues]
    return contact_freq, residue_ids

u = mda.Universe("topology.tpr", "trajectory.xtc")
contact_freq, res_ids = compute_contact_map(u, cutoff=8.0, stride=10)

# Plot contact map
fig, ax = plt.subplots(figsize=(7, 6))
im = ax.imshow(contact_freq, cmap="hot_r", origin="lower", vmin=0, vmax=1,
               extent=[res_ids[0], res_ids[-1], res_ids[0], res_ids[-1]])
plt.colorbar(im, ax=ax, label="Contact frequency")
ax.set_xlabel("Residue")
ax.set_ylabel("Residue")
ax.set_title(f"Contact map (Cα < 8 Å, n={len(u.trajectory)//10} frames)")
plt.tight_layout()
```

## Difference Contact Maps: Comparing States

A difference contact map reveals contacts that form or break between two conditions (apo vs. holo, mutant vs. wild-type):

```python
def difference_contact_map(contact_map_1, contact_map_2, title="Difference map"):
    """
    Plot difference in contact frequencies between two conditions.
    Positive = more contact in condition 2; negative = less contact.
    """
    diff = contact_map_2 - contact_map_1

    fig, ax = plt.subplots(figsize=(7, 6))
    im = ax.imshow(diff, cmap="RdBu_r", origin="lower", vmin=-1, vmax=1)
    plt.colorbar(im, ax=ax, label="Δ contact frequency (state2 - state1)")
    ax.set_title(title)
    plt.tight_layout()

    # Report most changed contacts
    ij = np.argwhere(np.abs(diff) > 0.3)
    print(f"Contact pairs with |Δfreq| > 0.30:")
    for i, j in ij[i < j]:
        print(f"  Residues {i+1}-{j+1}: Δfreq = {diff[i,j]:+.2f}")
```

## Hydrogen Bond Analysis

Hydrogen bonds are the primary determinants of secondary structure, binding site architecture, and protein-ligand recognition. MDAnalysis implements H-bond detection using geometric criteria:

- **Distance criterion**: heavy atom donor-acceptor distance < 3.5 Å
- **Angle criterion**: donor-H-acceptor angle > 150°

```python
from MDAnalysis.analysis.hydrogenbonds import HydrogenBondAnalysis

hbond = HydrogenBondAnalysis(
    universe=u,
    between=["protein", "protein"],  # intraprotein H-bonds
    d_h_cutoff=1.2,      # donor-H max distance (Å)
    d_a_cutoff=3.5,      # donor-acceptor max distance (Å)
    d_h_a_angle_cutoff=150  # min D-H...A angle (degrees)
)
hbond.run(step=10, verbose=True)

# hbond.results.hbonds: array of [frame, donor_idx, H_idx, acceptor_idx, distance, angle]
hbonds_array = hbond.results.hbonds

# Count H-bonds per frame
frames = hbonds_array[:, 0].astype(int)
n_hbonds_per_frame = np.bincount(frames - frames.min())
print(f"Mean H-bonds: {n_hbonds_per_frame.mean():.1f} ± {n_hbonds_per_frame.std():.1f}")

# Persistent H-bonds: present in > 50% of frames
from MDAnalysis.analysis.hydrogenbonds import HydrogenBondAnalysis

def persistent_hbonds(hbonds_array, n_frames, threshold=0.5):
    """Identify H-bonds present in > threshold fraction of frames."""
    # Group by donor-acceptor pair
    from collections import defaultdict
    pair_counts = defaultdict(int)
    for frame, donor, h, acceptor, dist, angle in hbonds_array:
        pair_counts[(int(donor), int(acceptor))] += 1

    persistent = {
        pair: count / n_frames
        for pair, count in pair_counts.items()
        if count / n_frames > threshold
    }
    return persistent

n_frames = len(set(hbonds_array[:, 0].astype(int)))
persistent = persistent_hbonds(hbonds_array, n_frames)
print(f"Persistent H-bonds (> 50%): {len(persistent)}")
```

## Salt Bridge Analysis

Salt bridges (electrostatic interactions between oppositely charged residues) stabilize protein structure and can be disrupted at physiological pH or by mutations:

```python
def analyze_salt_bridges(universe, cutoff=4.0, stride=10):
    """
    Identify salt bridges between basic and acidic residues.
    Criterion: any heavy atom between charged groups < cutoff Å.
    """
    basic = universe.select_atoms(
        "(resname ARG and name NH1 NH2 NE) or "
        "(resname LYS and name NZ) or "
        "(resname HIP and name ND1 NE2)"
    )
    acidic = universe.select_atoms(
        "(resname ASP and name OD1 OD2) or "
        "(resname GLU and name OE1 OE2)"
    )

    from collections import defaultdict
    bridge_counts = defaultdict(int)
    n_frames = 0

    for ts in universe.trajectory[::stride]:
        distances = distance_array(basic.positions, acidic.positions,
                                   box=ts.dimensions)
        contacts = np.argwhere(distances < cutoff)
        for i, j in contacts:
            basic_resid  = basic.atoms[i].resid
            acidic_resid = acidic.atoms[j].resid
            bridge_counts[(basic_resid, acidic_resid)] += 1
        n_frames += 1

    # Convert to frequencies
    salt_bridge_freq = {
        pair: count / n_frames
        for pair, count in bridge_counts.items()
    }
    print("Salt bridges (> 10% occupancy):")
    for (r1, r2), freq in sorted(salt_bridge_freq.items(),
                                   key=lambda x: -x[1]):
        if freq > 0.1:
            r1_name = universe.select_atoms(f"resid {r1}").residues[0].resname
            r2_name = universe.select_atoms(f"resid {r2}").residues[0].resname
            print(f"  {r1_name}{r1} — {r2_name}{r2}: {freq*100:.0f}%")
    return salt_bridge_freq
```

## Protein-Ligand Interaction Fingerprints

For protein-ligand simulations, interaction fingerprints summarize which residues interact with the ligand over time:

```python
def protein_ligand_fingerprint(universe, ligand_sel="resname LIG",
                                cutoff=4.5, stride=10):
    """
    Compute per-residue interaction frequency between protein and ligand.
    """
    ligand = universe.select_atoms(ligand_sel)
    protein = universe.select_atoms("protein")

    residue_contacts = np.zeros(protein.n_residues)
    n_frames = 0

    for ts in universe.trajectory[::stride]:
        # Distance from each protein atom to nearest ligand atom
        dist_matrix = distance_array(protein.positions, ligand.positions,
                                      box=ts.dimensions)
        min_dist_per_atom = dist_matrix.min(axis=1)  # (n_protein_atoms,)

        # Map to residues: contact if any atom in residue is < cutoff
        for res_idx, res in enumerate(protein.residues):
            res_atom_indices = res.atoms.indices - protein.atoms[0].index
            if min_dist_per_atom[res_atom_indices].min() < cutoff:
                residue_contacts[res_idx] += 1
        n_frames += 1

    contact_freq = residue_contacts / n_frames
    res_ids = [res.resid for res in protein.residues]

    # Top binding residues
    top_n = 10
    top_idx = np.argsort(contact_freq)[-top_n:][::-1]
    print(f"Top {top_n} residues interacting with ligand:")
    for idx in top_idx:
        if contact_freq[idx] > 0.1:
            res = protein.residues[idx]
            print(f"  {res.resname}{res.resid}: {contact_freq[idx]*100:.0f}%")

    return contact_freq, res_ids

# Visualize as bar plot
freq, res_ids = protein_ligand_fingerprint(u)
fig, ax = plt.subplots(figsize=(14, 3))
ax.bar(res_ids, freq, color="steelblue", alpha=0.8)
ax.set_xlabel("Residue")
ax.set_ylabel("Interaction frequency")
ax.set_title("Protein-ligand interaction fingerprint")
plt.tight_layout()
```

## Why This Matters

Contact maps and interaction analysis extract the structural information that is most relevant to molecular function. In drug discovery, protein-ligand interaction fingerprints derived from MD trajectories identify which residues are critical for binding — information used directly to design more potent analogs. In enzyme engineering, tracking the hydrogen bond network in an active site across hundreds of nanoseconds reveals the dynamic basis of catalysis and guides the design of more stable variants. These analyses turn a trajectory file into biological insight.
