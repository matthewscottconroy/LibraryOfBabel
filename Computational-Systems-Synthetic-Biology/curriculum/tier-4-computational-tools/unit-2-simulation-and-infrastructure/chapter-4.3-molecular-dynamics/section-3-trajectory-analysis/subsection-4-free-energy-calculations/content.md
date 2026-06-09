# Free Energy Calculations from MD

A drug candidate binds its target with a dissociation constant of 50 nM; its analog, one methyl group different, binds at 500 nM. The structural difference is trivial. The energetic difference is about 1.4 kcal/mol — roughly twice the thermal energy at body temperature. Predicting this difference computationally, from first principles, is one of the central goals of computational drug discovery. The binding free energy $\Delta G_\text{bind}$ is the thermodynamic quantity most directly relevant to drug discovery and molecular recognition. MD-based free energy methods compute this from first principles, with accuracy potentially approaching 1 kcal/mol — sufficient to distinguish binders from non-binders and rank order compounds for medicinal chemistry. Three methods are widely used: MM-PBSA/GBSA (fast, approximate), free energy perturbation (FEP, rigorous, expensive), and umbrella sampling (for processes along a reaction coordinate).

## MM-PBSA: Rapid Binding Free Energy Estimation

**MM-PBSA** (Molecular Mechanics Poisson-Boltzmann Surface Area) decomposes the binding free energy into molecular mechanics and solvation terms:

$$\Delta G_\text{bind} = G_\text{complex} - G_\text{protein} - G_\text{ligand}$$

$$G = \underbrace{E_\text{MM}}_{\text{bonded + non-bonded}} + \underbrace{\Delta G_\text{solv,PB}}_{\text{polar solvation}} + \underbrace{\gamma \cdot \text{SASA}}_{\text{nonpolar solvation}} - \underbrace{T S}_{\text{entropy}}$$

The polar solvation term $\Delta G_\text{solv,PB}$ is solved via the linearized Poisson-Boltzmann equation; the nonpolar term uses the solvent-accessible surface area. Entropy is often neglected (MM-PBSA without entropy) for computational speed, which introduces systematic errors but is often acceptable for relative ranking.

```python
# Python implementation of MM-PBSA post-processing using gmx_MMPBSA
# (install: pip install gmx_MMPBSA)

import subprocess
import os

def run_mmpbsa(tpr_file, traj_file, topology_file, ligand_name="LIG",
               output_dir="mmpbsa/"):
    """
    Run MM-PBSA analysis using gmx_MMPBSA.
    Requires AMBER and gmx_MMPBSA installed.
    """
    os.makedirs(output_dir, exist_ok=True)

    # Create input file for gmx_MMPBSA
    mmpbsa_input = f"""
&general
startframe = 1,       ! first frame for analysis
endframe   = 9999,    ! last frame (all)
interval   = 10,      ! analyze every 10th frame
verbose    = 2,
/
&pb
istrng   = 0.15,      ! ionic strength (mol/L) = 150 mM NaCl
radiopt  = 0,         ! use atom radii from topology
/
"""
    input_file = os.path.join(output_dir, "mmpbsa.in")
    with open(input_file, "w") as f:
        f.write(mmpbsa_input)

    cmd = [
        "gmx_MMPBSA",
        "-O",                    # overwrite output
        "-i", input_file,
        "-cs", tpr_file,         # receptor+ligand system
        "-ct", traj_file,        # trajectory
        "-cp", topology_file,    # topology
        "-lm", f"mol_{ligand_name}.mol2",  # ligand mol2 file
        "-o", os.path.join(output_dir, "FINAL_RESULTS_MMPBSA.dat"),
        "-eo", os.path.join(output_dir, "MMPBSA_energy.csv"),
    ]
    subprocess.run(cmd, check=True)

def parse_mmpbsa_results(results_file):
    """Parse gmx_MMPBSA output and extract ΔG_bind components."""
    import re
    components = {}
    with open(results_file) as f:
        for line in f:
            match = re.search(r"(ΔTOTAL|ΔVDWAALS|ΔEEL|ΔPB|ΔSASA)\s+.*=\s+([-\d.]+)", line)
            if match:
                components[match.group(1)] = float(match.group(2))
    return components

# Typical output:
# ΔVDWAALS = -25.3 kcal/mol (van der Waals: usually favourable)
# ΔEEL     = -15.2 kcal/mol (electrostatics: variable)
# ΔPB      = +28.1 kcal/mol (polar desolvation penalty: usually unfavourable)
# ΔSASA    = -3.4  kcal/mol (hydrophobic burial: usually favourable)
# ΔTOTAL   = -15.8 kcal/mol (estimated ΔG_bind)
```

**MM-PBSA accuracy**: relative binding affinities for structurally similar ligands (congeneric series) within ~2 kcal/mol. Absolute binding affinities systematically wrong by 5–20 kcal/mol. Sufficient for hit ranking, not for quantitative affinity prediction.

## Free Energy Perturbation (FEP)

**FEP** computes the free energy difference between two states (typically molecule A and molecule B) by slowly transforming one into the other via a series of non-physical intermediate states parameterized by $\lambda \in [0, 1]$.

$$\Delta G_{A \to B} = -k_B T \ln \left\langle e^{-(V_B - V_A)/k_B T} \right\rangle_A$$

This exponential average converges slowly for large perturbations. The solution is to use many intermediate $\lambda$ windows with adjacent overlap:

$$\Delta G_{A \to B} = \sum_{i=0}^{n-1} \Delta G_{i \to i+1}$$

The **Bennett Acceptance Ratio (BAR)** estimator optimally combines forward and backward samples between each pair of adjacent windows:

$$\Delta G_{i \to i+1} = k_B T \ln \frac{\langle f(V_i - V_{i+1} + C)\rangle_{i+1}}{\langle f(V_{i+1} - V_i - C)\rangle_i} + C$$

where $f(x) = 1/(1 + e^{x/k_BT})$ is the Fermi function and $C$ is determined self-consistently.

```python
import numpy as np
from scipy.optimize import brentq

def bar_estimator(dE_forward, dE_backward, beta=1.0/(0.592)):
    """
    Bennett Acceptance Ratio estimator for free energy difference.
    dE_forward:  (n_samples,) energy differences V_i+1 - V_i sampled from state i
    dE_backward: (n_samples,) energy differences V_i - V_i+1 sampled from state i+1
    beta: 1/(kT) in kcal/mol^-1 units (default: 300 K)
    Returns: ΔG in kcal/mol
    """
    n_f = len(dE_forward)
    n_b = len(dE_backward)

    def f_fermi(x):
        return 1.0 / (1.0 + np.exp(beta * x))

    def bar_equation(C):
        """Equation that C must satisfy for BAR."""
        lhs = np.mean(f_fermi( dE_forward  - C))
        rhs = np.mean(f_fermi(-dE_backward + C))
        return lhs - rhs * (n_f / n_b)

    # Solve for C numerically
    try:
        C = brentq(bar_equation, -100, 100, xtol=1e-6)
    except ValueError:
        C = 0.0  # fallback if no root found

    return C + np.log(n_f / n_b) / beta  # ΔG in kcal/mol

# Thermodynamic cycle for relative binding free energy (RBFE)
# ΔΔG_bind(A→B) = ΔG_bind(B) - ΔG_bind(A)
#               = ΔG_complex(A→B) - ΔG_solvent(A→B)
#
# Both legs computed by FEP; difference gives ΔΔG_bind
print("Relative FEP protocol:")
print("  1. Prepare complex trajectory: protein + ligand A (λ=0) → ligand B (λ=1)")
print("  2. Prepare solvent trajectory: ligand A (λ=0) → ligand B (λ=1) in water")
print("  3. Each λ window: 2-5 ns simulation (12-20 windows total)")
print("  4. BAR estimator: combine adjacent windows")
print("  5. ΔΔG_bind = ΔG_complex(A→B) - ΔG_water(A→B)")
```

## Umbrella Sampling and the PMF

For processes with a clear reaction coordinate (ion permeation, ligand unbinding, membrane insertion), **umbrella sampling** biases the simulation with harmonic restraints at evenly spaced positions along the coordinate and reconstructs the full free energy profile (PMF = potential of mean force).

The bias potential at window $i$ centered at $s_i$:

$$V_\text{bias}^{(i)}(s) = \frac{k}{2}(s - s_i)^2$$

The PMF is reconstructed from overlapping distributions using **WHAM** (Weighted Histogram Analysis Method):

$$P_i(s) \propto e^{-\beta[W(s) - V_\text{bias}^{(i)}(s)]}$$

$$W(s) = -k_B T \ln \frac{\sum_i N_i P_i^\text{raw}(s) e^{\beta V_\text{bias}^{(i)}(s)}}{\sum_i N_i e^{-\beta [F_i - V_\text{bias}^{(i)}(s)]}}$$

solved self-consistently for free energies $\{F_i\}$.

```bash
# GROMACS umbrella sampling workflow
# Step 1: Pull simulation to generate starting configurations
# pull.mdp: constant-velocity pulling along z-axis
[pull]
pull-coord1-type = umbrella
pull-coord1-geometry = distance
pull-coord1-dim = N N Y           # pull along z only
pull-coord1-groups = 1 2          # groups 1 and 2
pull-coord1-init = 0.0
pull-coord1-rate = 0.01           # 0.01 nm/ps = 1 nm/100 ps
pull-coord1-k = 1000              # force constant (kJ/mol/nm²)
pull-nstfout = 500                # output frequency

# Step 2: Extract configurations at each window spacing
python3 extract_umbrella_windows.py --pull_traj pull.xtc --spacing 0.1  # nm

# Step 3: Run umbrella sampling at each window
for i in $(seq 0 19); do
    gmx grompp -f umbrella.mdp -c window_${i}.gro -r window_${i}.gro \
               -p topol.top -o umbrella_${i}.tpr
    gmx mdrun -deffnm umbrella_${i} -ntmpi 1 -ntomp 4 &
done; wait

# Step 4: Reconstruct PMF with WHAM
gmx wham -it tpr-files.dat -if pullf-files.dat -o pmf.xvg -hist histograms.xvg
```

```python
import numpy as np
import matplotlib.pyplot as plt

def plot_pmf(pmf_file, xlabel="Distance (nm)", units="kJ/mol"):
    """Plot the potential of mean force from GROMACS WHAM output."""
    data = np.loadtxt(pmf_file, comments=["#", "@"])
    coord, pmf, error = data[:, 0], data[:, 1], data[:, 2]

    # Shift minimum to zero
    pmf -= pmf.min()

    fig, ax = plt.subplots(figsize=(8, 4))
    ax.plot(coord, pmf, "navy", lw=2)
    ax.fill_between(coord, pmf - error, pmf + error, alpha=0.3, color="navy")
    ax.set_xlabel(xlabel)
    ax.set_ylabel(f"PMF ({units})")
    ax.set_title("Potential of Mean Force")

    barrier = pmf.max()
    print(f"Free energy barrier: {barrier:.1f} {units}")
    print(f"  Rate estimate: k ~ exp(-{barrier:.1f}/{0.592*4.18:.1f}) s^-1")
    plt.tight_layout()
    return coord, pmf, error
```

## Why This Matters

Free energy calculations are the gold standard for quantitative predictions from MD. MM-PBSA provides rapid ranking at the cost of accuracy; FEP provides near-experimental accuracy at the cost of 100–1000× more compute. In modern drug discovery, prospective FEP calculations with errors of ~0.5–1 kcal/mol are used to guide synthesis decisions — each kcal/mol in binding affinity corresponds to roughly a 5-fold change in $K_d$. The computational cost of FEP (days of GPU time per compound) is still far less than the cost of synthesis and experimental characterization, making FEP-guided drug design increasingly routine in pharmaceutical research.
