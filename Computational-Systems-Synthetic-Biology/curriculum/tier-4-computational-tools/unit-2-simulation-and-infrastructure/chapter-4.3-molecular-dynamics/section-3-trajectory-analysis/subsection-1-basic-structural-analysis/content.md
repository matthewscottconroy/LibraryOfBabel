# Basic Structural Analysis of MD Trajectories

You have run 500 nanoseconds of simulation. You have a trajectory file the size of a modest novel, storing the positions of 80,000 atoms at 50,000 time points. Now what? Collecting an MD trajectory is only half the work. Extracting meaningful biological information requires systematic analysis of the trajectory — quantifying structural deviations, flexibility, and conformational changes. Three fundamental structural metrics — RMSD, RMSF, and radius of gyration — form the foundation of any trajectory analysis workflow. They are the first things you should look at, and the last things you should feel satisfied about having looked at before moving to more sophisticated analyses.

## The MDAnalysis Framework

**MDAnalysis** is the standard Python library for trajectory analysis. It reads virtually all trajectory formats (XTC, DCD, TRR, NetCDF) and topology files (GRO, PSF, PRMTOP) and provides atom selection, transformation, and analysis tools.

```python
import MDAnalysis as mda
import numpy as np
import matplotlib.pyplot as plt
from MDAnalysis.analysis import rms, align

# Load trajectory
u = mda.Universe("topology.tpr", "trajectory.xtc")

# Atom selections (MDAnalysis selection syntax)
protein    = u.select_atoms("protein")
backbone   = u.select_atoms("backbone")           # N, CA, C, O
alpha_c    = u.select_atoms("protein and name CA")
heavy      = u.select_atoms("protein and not name H*")

print(f"System: {u.atoms.n_atoms} atoms total")
print(f"Protein: {protein.n_atoms} atoms, {protein.n_residues} residues")
print(f"Trajectory: {u.trajectory.n_frames} frames, dt = {u.trajectory.dt:.1f} ps")
```

## Root Mean Square Deviation (RMSD)

**RMSD** measures the average positional deviation of selected atoms between a current frame and a reference structure, after optimal alignment (rotation + translation):

$$\text{RMSD}(t) = \sqrt{\frac{1}{N}\sum_{i=1}^{N} \left|\mathbf{r}_i(t) - \mathbf{r}_i^\text{ref}\right|^2}$$

where the sum runs over $N$ selected atoms (typically $C_\alpha$ or backbone atoms) and $\mathbf{r}_i^\text{ref}$ is the reference position (often the first frame or the crystal structure) after least-squares superposition.

**Interpretation**:
- RMSD plateauing at < 2 Å: stable simulation; structure close to reference
- RMSD increasing monotonically: conformational drift; large domain motion or instability
- RMSD fluctuating: conformational sampling between distinct states

```python
from MDAnalysis.analysis.rms import RMSD

# RMSD of Cα atoms relative to starting structure
R = RMSD(
    atomgroup=alpha_c,
    reference=alpha_c,          # use first frame as reference (auto)
    select="name CA",
    groupselections=["backbone", "name CA and resnum 10-50"],  # additional groups
    ref_frame=0                 # reference frame index
)
R.run(verbose=True)

# R.rmsd shape: (n_frames, 3+n_groups)
# columns: frame, time (ps), RMSD (Å), [additional group RMSDs]
time_ns = R.rmsd[:, 1] / 1000  # ps -> ns
rmsd_ca = R.rmsd[:, 2]

# Plot
fig, ax = plt.subplots(figsize=(10, 3))
ax.plot(time_ns, rmsd_ca, lw=0.8, color="navy", alpha=0.8)
ax.axhline(rmsd_ca[len(rmsd_ca)//2:].mean(), color="red", ls="--",
           label=f"Mean (2nd half): {rmsd_ca[len(rmsd_ca)//2:].mean():.2f} Å")
ax.set_xlabel("Time (ns)")
ax.set_ylabel("RMSD (Å)")
ax.set_title("Backbone Cα RMSD")
ax.legend()
plt.tight_layout()

# RMSD matrix: pairwise RMSD between all frames (reveals clusters)
from MDAnalysis.analysis.rms import RMSF
from itertools import combinations

def rmsd_matrix(universe, selection="name CA", step=10):
    """Compute pairwise RMSD matrix (subsample by step for speed)."""
    ag = universe.select_atoms(selection)
    frames = list(range(0, universe.trajectory.n_frames, step))
    n = len(frames)
    matrix = np.zeros((n, n))

    coords = []
    for i, frame in enumerate(frames):
        universe.trajectory[frame]
        coords.append(ag.positions.copy())

    for i in range(n):
        for j in range(i, n):
            # Align frame j to frame i
            R_ij = rms.rmsd(coords[i], coords[j], superposition=True)
            matrix[i, j] = matrix[j, i] = R_ij

    return matrix, frames
```

## Root Mean Square Fluctuation (RMSF)

**RMSF** is the per-residue time-averaged positional deviation from the mean position. Unlike RMSD (which gives a single number per frame), RMSF gives one value per residue, mapping flexibility onto the protein structure:

$$\text{RMSF}_i = \sqrt{\langle |\mathbf{r}_i(t) - \langle\mathbf{r}_i\rangle|^2 \rangle_t}$$

High RMSF indicates flexible regions (loops, termini, linkers); low RMSF indicates rigid regions (secondary structure elements, hydrophobic core).

```python
from MDAnalysis.analysis.rms import RMSF

# Align all frames to first frame before computing RMSF
aligner = align.AlignTraj(u, u, select="backbone", in_memory=False)
aligner.run()

# Compute RMSF per residue
rmsf_analyzer = RMSF(alpha_c, verbose=True)
rmsf_analyzer.run()

# rmsf_analyzer.rmsf: (n_residues,) array
rmsf_values = rmsf_analyzer.rmsf
residue_ids  = [res.resid for res in alpha_c.residues]

fig, ax = plt.subplots(figsize=(12, 3))
ax.fill_between(residue_ids, rmsf_values, alpha=0.7, color="steelblue")
ax.plot(residue_ids, rmsf_values, lw=0.8, color="navy")
ax.set_xlabel("Residue number")
ax.set_ylabel("RMSF (Å)")
ax.set_title("Per-residue flexibility")
ax.axhline(2.0, color="red", ls="--", alpha=0.5, label="2 Å threshold")
ax.legend()
plt.tight_layout()

# High RMSF residues (candidate flexible regions)
flexible_threshold = 3.0
flexible_residues = [resid for resid, rmsf in zip(residue_ids, rmsf_values)
                     if rmsf > flexible_threshold]
print(f"Residues with RMSF > {flexible_threshold} Å: {flexible_residues}")
```

## Radius of Gyration

The **radius of gyration** $R_g$ measures the overall compactness of the protein:

$$R_g(t) = \sqrt{\frac{\sum_i m_i |\mathbf{r}_i(t) - \mathbf{r}_\text{com}(t)|^2}{\sum_i m_i}}$$

where $\mathbf{r}_\text{com}$ is the center of mass. A protein that unfolds will show increasing $R_g$; a protein that compacts will show decreasing $R_g$.

```python
def compute_radius_of_gyration(universe, selection="protein", skip=1):
    """Compute Rg over trajectory."""
    ag = universe.select_atoms(selection)
    rg_values = []
    times = []

    for ts in universe.trajectory[::skip]:
        rg_values.append(ag.radius_of_gyration())
        times.append(ts.time)

    return np.array(times) / 1000, np.array(rg_values)

time_ns, rg = compute_radius_of_gyration(u)
print(f"Mean Rg: {rg.mean():.2f} ± {rg.std():.2f} Å")
print(f"  Stable protein: Rg fluctuation < 1 Å")
print(f"  Unfolding protein: Rg increases > 5 Å from native")
```

## Combining Metrics: Conformational Clustering

Combining RMSD and Rg reveals the conformational landscape:

```python
from sklearn.cluster import KMeans

def cluster_trajectory(rmsd_matrix, n_clusters=5):
    """
    Cluster trajectory frames by pairwise RMSD.
    Uses K-means on the RMSD matrix rows (each row = distances to all other frames).
    """
    kmeans = KMeans(n_clusters=n_clusters, random_state=42, n_init=10)
    cluster_labels = kmeans.fit_predict(rmsd_matrix)

    print("Cluster populations:")
    for c in range(n_clusters):
        count = np.sum(cluster_labels == c)
        print(f"  Cluster {c}: {count} frames ({count/len(cluster_labels)*100:.1f}%)")

    return cluster_labels

# Worked example: RMSD-based clustering with output
def full_structural_report(u, output_dir="analysis/"):
    """Run complete structural analysis and write summary."""
    import os
    os.makedirs(output_dir, exist_ok=True)

    # RMSD
    R = RMSD(u.select_atoms("name CA"), ref_frame=0)
    R.run()

    # RMSF
    align.AlignTraj(u, u, select="backbone").run()
    rf = RMSF(u.select_atoms("name CA"))
    rf.run()

    # Rg
    _, rg = compute_radius_of_gyration(u)

    # Summary statistics
    n_prod = len(R.rmsd) // 2  # use second half as "production"
    report = {
        "mean_rmsd_A":    R.rmsd[n_prod:, 2].mean(),
        "std_rmsd_A":     R.rmsd[n_prod:, 2].std(),
        "mean_rmsf_A":    rf.rmsf.mean(),
        "max_rmsf_A":     rf.rmsf.max(),
        "max_rmsf_resid": u.select_atoms("name CA").residues[rf.rmsf.argmax()].resid,
        "mean_rg_A":      rg[n_prod:].mean(),
    }
    print(f"Structural summary:")
    for k, v in report.items():
        print(f"  {k}: {v:.2f}")
    return report
```

## Why This Matters

RMSD, RMSF, and $R_g$ are the first three analyses performed on any new MD trajectory. They answer the most basic questions: Did the simulation remain stable? Where is the protein flexible? Did any major conformational change occur? These metrics directly connect to experimental observables: RMSF correlates with crystallographic B-factors ($B = 8\pi^2 \langle u^2 \rangle / 3$, where $u$ is the positional fluctuation), and $R_g$ can be compared to small-angle X-ray scattering (SAXS) data. Failure to perform these basic analyses before more sophisticated work is a common mistake that leads to reporting conformational changes that are actually simulation artifacts.
