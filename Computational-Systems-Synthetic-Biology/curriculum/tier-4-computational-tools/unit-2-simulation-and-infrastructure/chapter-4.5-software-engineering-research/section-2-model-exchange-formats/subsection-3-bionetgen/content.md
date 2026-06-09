# BioNetGen: Rule-Based Modeling

The EGF receptor has five major tyrosine phosphorylation sites, two dimerization states, and binds at least three different adaptor proteins. Each of these features can independently be present or absent, which means the receptor alone can exist in 2^5 × 2 × 2^3 = 512 distinct molecular states. Now imagine writing an ordinary differential equation for every one of those states, and a rate expression for every transition between them. That is not a realistic model — it is a bookkeeping catastrophe waiting for a human to make an error on reaction number 847.

**BioNetGen** (BNG) is a rule-based modeling framework designed to address this **combinatorial complexity** problem in cell signaling. When receptors bind ligands, phosphorylate substrates, form complexes, and recruit adaptors, the number of possible molecular species grows exponentially — a single EGF receptor with 5 phosphorylation sites and 3 binding partners can form thousands of distinct molecular states. Writing an ODE or SBML model for each of these species is intractable. BioNetGen instead describes molecular interactions as **rules** that apply to patterns of molecular states, generating all possible species automatically.

## The Combinatorial Complexity Problem

Consider a receptor tyrosine kinase (EGFR) with:
- 1 ligand binding site (bound/unbound)
- 3 tyrosine phosphorylation sites (each phosphorylated/unphosphorylated)
- 2 adaptor protein binding sites (each bound to adaptor/free)

The number of possible molecular states: $2^1 \times 2^3 \times 2^2 = 32$ for a single EGFR monomer. For a dimer, the combinatorics multiply further. Writing individual ODEs for all states, while listing all interactions between them, is error-prone and unscalable. Rule-based modeling reduces this to ~10 rules that generate the full system automatically.

## BNGL Syntax

The BioNetGen Language (BNGL) describes molecules as structured objects with components:

```python
# BNGL model: simplified EGF-EGFR binding and activation
# This file would be saved as egfr_simple.bngl

bngl_model = """
begin model

begin parameters
  EGF_tot  1000  # total EGF ligand (molecules)
  EGFR_tot  500  # total EGFR receptor (molecules)
  kon    0.003   # EGF-EGFR association rate (1/molecule/s)
  koff   0.06    # EGF-EGFR dissociation rate (1/s)
  kphos  1.0     # EGFR phosphorylation rate (dimerized, 1/s)
  kdephos 0.01   # EGFR dephosphorylation rate (1/s)
end parameters

begin molecule types
  # EGFR: receptor with:
  #   lig = ligand binding site (empty or bound: 0 or bound state)
  #   Y1045 = phosphorylation site (U=unphosphorylated, P=phosphorylated)
  #   dim = dimerization site (empty or bound to another EGFR)
  EGFR(lig, Y1045~U~P, dim)

  # EGF ligand with receptor binding site
  EGF(rec)
end molecule types

begin seed species
  EGF(rec)       EGF_tot    # free EGF ligand
  EGFR(lig,Y1045~U,dim)  EGFR_tot  # unbound, unphosphorylated EGFR monomer
end seed species

begin reaction rules
  # Rule 1: EGF binds EGFR (bimolecular)
  EGFR(lig) + EGF(rec) <-> EGFR(lig!1).EGF(rec!1)  kon, koff

  # Rule 2: Two EGF-bound EGFRs dimerize
  EGFR(lig!+, dim) + EGFR(lig!+, dim) <-> EGFR(lig!+,dim!1).EGFR(lig!+,dim!1)  0.1*kon, koff

  # Rule 3: Phosphorylation of Y1045 (only in dimer, lig!+ = bound ligand)
  EGFR(lig!+, Y1045~U, dim!+) -> EGFR(lig!+, Y1045~P, dim!+)  kphos

  # Rule 4: Dephosphorylation (anywhere in cell)
  EGFR(Y1045~P) -> EGFR(Y1045~U)  kdephos
end reaction rules

begin observables
  # What we measure: fraction phosphorylated EGFR
  Molecules  EGF_bound_EGFR   EGFR(lig!+)
  Molecules  Phospho_EGFR     EGFR(Y1045~P)
  Molecules  EGFR_dimers      EGFR(dim!+)
end observables

end model

# Simulate using ODE or SSA
generate_network({overwrite=>1})
simulate({method=>"ode", t_end=>200, n_steps=>1000})
"""
```

Notice how readable each rule is as a biological statement. Rule 3 says: an EGFR that is bound to ligand, has an unphosphorylated Y1045, and is in a dimer becomes phosphorylated at Y1045. BioNetGen then enumerates all molecular contexts in which this pattern can occur and generates the corresponding ODEs automatically.

## Running BioNetGen from Python

```python
import subprocess
import os
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

def run_bionetgen(bngl_file, method="ode", t_end=200, n_steps=1000,
                   bng_path="BNG2.pl"):
    """
    Run a BioNetGen simulation from a .bngl file.
    
    Prerequisites: BioNetGen2 installed (bionetgen.org)
    bng_path: path to the BNG2.pl Perl script
    method: 'ode' (deterministic), 'ssa' (Gillespie), 'nf' (network-free)
    """
    # Write simulation command to temp bngl file
    base_name = bngl_file.replace(".bngl", "")

    cmd = [bng_path, bngl_file]
    result = subprocess.run(cmd, capture_output=True, text=True)

    if result.returncode != 0:
        print(f"BioNetGen error:\n{result.stderr}")
        return None

    print(f"BioNetGen completed: {result.stdout.split(chr(10))[-2]}")

    # Parse output: .gdat (observable trajectories) or .scan files
    gdat_file = f"{base_name}.gdat"
    if os.path.exists(gdat_file):
        return parse_gdat(gdat_file)
    return None

def parse_gdat(gdat_file):
    """Parse BioNetGen .gdat (observable over time) output file."""
    with open(gdat_file) as f:
        lines = f.readlines()

    # Header line starts with #
    header = [h.strip() for h in lines[0].lstrip("#").split()]

    data = []
    for line in lines[1:]:
        if not line.startswith("#"):
            data.append([float(x) for x in line.split()])

    df = pd.DataFrame(data, columns=header)
    print(f"Loaded: {len(df)} time points, observables: {header[1:]}")
    return df

# Alternative: use PyBioNetGen Python package
def run_bionetgen_python(bngl_string, method="ode", t_end=200, n_steps=1000):
    """
    Run BioNetGen via PyBioNetGen Python interface.
    pip install bionetgen
    """
    import bionetgen

    # Write model to temp file
    with open("/tmp/model.bngl", "w") as f:
        f.write(bngl_string)

    # Create and run model
    model = bionetgen.bngmodel("/tmp/model.bngl")
    model.setConcentration("time", t_end)

    # Simulate
    result = model.simulate(method=method, t_end=t_end, n_steps=n_steps)
    return result
```

## Network-Free Simulation (NFsim)

For very large species spaces where even generating the full network is intractable, **NFsim** (Network-Free simulation) samples individual molecular interactions stochastically without ever enumerating all possible species:

```python
def compare_bngl_methods():
    """
    Demonstrate when to use ODE vs. SSA vs. NFsim.
    """
    print("BioNetGen simulation methods:")
    print()
    print("ODE (deterministic, 'ode')")
    print("  - Generate network, then solve ODEs")
    print("  - Fast for small-medium networks")
    print("  - No stochastic noise")
    print("  - Fails if network too large to enumerate")
    print()
    print("SSA (Gillespie, 'ssa')")
    print("  - Exact stochastic simulation")
    print("  - Requires full network generation")
    print("  - Slow for large/complex systems")
    print("  - Correct for low molecule count regimes")
    print()
    print("NFsim (network-free, 'nf')")
    print("  - Stochastic; does NOT generate full network")
    print("  - Efficient for combinatorially complex models")
    print("  - Example: T cell receptor signaling (>10^7 possible species)")
    print("  - Can simulate 1000-component signaling complexes")
    print()
    print("Rule of thumb:")
    print("  ODE: < 1000 species, deterministic answer needed")
    print("  SSA: < 1000 species, stochasticity important")
    print("  NFsim: > 1000 species OR combinatorial complexity")
```

## Worked Example: Receptor Clustering Analysis

```python
receptor_clustering_model = """
begin model
begin parameters
  R_tot      1000   # receptor molecules per cell
  kon_lat    0.01   # lateral association rate (within membrane)
  koff_lat   0.1    # lateral dissociation rate
  kact       0.5    # activation rate for clustered receptor
  kinact     0.01   # inactivation rate
end parameters

begin molecule types
  R(lat1, lat2, state~inactive~active)
end molecule types

begin seed species
  R(lat1, lat2, state~inactive)  R_tot
end seed species

begin reaction rules
  # Lateral dimerization (in membrane)
  R(lat1) + R(lat2) <-> R(lat1!1).R(lat2!1)  kon_lat, koff_lat

  # Activation: dimers activate each other
  R(lat1!+, state~inactive) -> R(lat1!+, state~active)  kact

  # Deactivation: spontaneous
  R(state~active) -> R(state~inactive)  kinact
end reaction rules

begin observables
  Molecules  ActiveReceptors   R(state~active)
  Molecules  Dimers           R(lat1!+)
end observables
end model

simulate({method=>"nf", t_end=>100, n_steps=>500, print_functions=>1})
"""

# Analysis: cluster size distribution over time
def analyze_cluster_distribution(results_df):
    """
    From NFsim species output, compute receptor cluster size distribution.
    """
    if results_df is None:
        print("Simulation results not available")
        return

    time = results_df.iloc[:, 0].values
    active = results_df["ActiveReceptors"].values if "ActiveReceptors" in results_df else None
    dimers = results_df["Dimers"].values if "Dimers" in results_df else None

    if active is not None:
        fig, axes = plt.subplots(1, 2, figsize=(12, 4))
        axes[0].plot(time, active, "steelblue", lw=2, label="Active receptors")
        if dimers is not None:
            axes[0].plot(time, dimers, "red", lw=2, ls="--", label="Dimers")
        axes[0].set_xlabel("Time (s)")
        axes[0].set_ylabel("Molecule count")
        axes[0].legend()

        # Steady-state fraction
        ss_frac = active[-100:].mean() / 1000  # R_tot = 1000
        print(f"Steady-state active fraction: {ss_frac:.1%}")
```

## BioNetGen vs. SBML for Complex Models

| Scenario | SBML | BioNetGen |
|---|---|---|
| Simple mass-action kinetics | ✓ (preferred) | Overkill |
| Enzymatic pathway (< 20 reactions) | ✓ | Either |
| Receptor with 2–3 modifications | ✓ (manageable) | Easier |
| Receptor with 5+ phosphosites, multiple adaptors | Impractical | ✓ |
| Immune receptor signaling (T cell, B cell) | Impossible | ✓ (NFsim) |
| Model exchange with COPASI/roadrunner | ✓ | Via SBML export |

BioNetGen can export its generated networks to SBML, enabling the BNG → SBML → roadrunner workflow for medium-complexity models.

## Why This Matters

Rule-based modeling makes signaling network models tractable that are otherwise impossible. EGFR signaling, T cell receptor activation, NF-κB, and many other key pathways have been successfully modeled only with BioNetGen — explicit ODE formulations would require thousands of equations listing every possible protein complex. The rules in a BNG model also serve as structured biological knowledge: each rule is a concise statement of one molecular interaction mechanism that can be read, validated against literature, and modified independently. For synthetic biology, rule-based models enable the design of synthetic signaling circuits with multiple input channels and combinatorial logic, where the interaction between circuit components generates a spectrum of output states that would be invisible in a simplified model.
