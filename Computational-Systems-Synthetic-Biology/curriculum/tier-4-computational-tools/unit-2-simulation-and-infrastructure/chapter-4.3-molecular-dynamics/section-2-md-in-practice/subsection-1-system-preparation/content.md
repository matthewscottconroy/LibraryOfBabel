# System Preparation for Molecular Dynamics

In 2010, a simulation of a GPCR — a membrane protein responsible for sensing hormones and neurotransmitters — was published that had taken months of careful preparation and years of compute time. The results were biologically significant. A year later, a similar simulation was retracted because the protein had been placed in the membrane in the wrong orientation, backward relative to the bilayer normal. Every calculation was technically correct. The physics was sound. The preparation was wrong, and the results were meaningless. A molecular dynamics simulation is only as good as the system that goes into it. Poor preparation — missing atoms, wrong protonation states, clashing atoms, insufficient solvation — will cause simulations to crash or, worse, to run but produce meaningless results. System preparation is not a mundane preprocessing step; it requires understanding of biochemistry, structure quality assessment, and careful decision-making at multiple stages.

## Starting Structure: Sources and Quality Assessment

The starting point for most simulations is an experimental structure from the Protein Data Bank (PDB) or a predicted structure from AlphaFold2.

```python
import requests
from pathlib import Path

def download_pdb(pdb_id, output_dir="."):
    """Download a PDB file from the RCSB."""
    url = f"https://files.rcsb.org/download/{pdb_id.upper()}.pdb"
    response = requests.get(url)
    response.raise_for_status()
    output_path = Path(output_dir) / f"{pdb_id.lower()}.pdb"
    output_path.write_text(response.text)
    print(f"Downloaded: {output_path}")
    return output_path

def assess_structure_quality(pdb_file):
    """
    Quick quality assessment of a PDB file.
    Reports: resolution, missing residues, non-standard residues, alternate conformations.
    """
    from Bio.PDB import PDBParser, DSSP
    import warnings

    parser = PDBParser(QUIET=True)
    structure = parser.get_structure("prot", pdb_file)
    model = structure[0]

    # Count atoms and residues
    atoms = list(model.get_atoms())
    residues = list(model.get_residues())
    std_residues = [r for r in residues if r.get_id()[0] == " "]
    het_residues = [r for r in residues if r.get_id()[0].startswith("H")]

    # Check for alternate conformations
    alt_conf = [a for a in atoms if a.get_altloc() not in (" ", "A")]

    print(f"Structure: {pdb_file}")
    print(f"  Standard residues: {len(std_residues)}")
    print(f"  HETATM (ligands/ions/water): {len(het_residues)}")
    print(f"  Total atoms: {len(atoms)}")
    print(f"  Alternate conformations: {len(alt_conf)} atoms")

    return {"n_residues": len(std_residues), "n_hetatm": len(het_residues),
            "alt_conf": len(alt_conf)}

# Example: ubiquitin
pdb_path = download_pdb("1ubq")
assess_structure_quality(pdb_path)
```

**Key quality checks:**
- **Resolution**: X-ray structures < 2.0 Å are high quality; > 3.0 Å may have significant coordinate errors
- **Missing residues**: loops missing from electron density must be modeled (Modeller, Rosetta, AlphaFold2)
- **Missing atoms**: sidechain atoms missing due to disorder must be rebuilt (pdb2gmx does this automatically)
- **Alternate conformations**: only one conformation can be used — choose the higher-occupancy conformer
- **Non-standard residues**: phosphorylated residues, covalent modifications require special parameters

## Protonation State Assignment

X-ray structures do not resolve hydrogens; protonation states must be determined computationally. This is critical because wrong protonation — especially at histidine, glutamate, aspartate, and lysine near their $\text{p}K_a$ — can fundamentally alter simulation behavior.

```bash
# PropKa: predict pKa values for all ionizable residues
propka3 protein.pdb

# Output: pKa for each ionizable residue
# Example:
# HIS 57 A:  pKa =  6.8 (model = 6.5); at pH 7 -> protonated (HIE/HIP?)
# ASP 102 A: pKa = 3.2 (model = 3.8); at pH 7 -> deprotonated (ASP)
# GLU 35 A:  pKa = 7.1 (model = 4.4); at pH 7 -> borderline!

# For each histidine, determine tautomer:
# HIE: epsilon-nitrogen protonated (most common)
# HID: delta-nitrogen protonated
# HIP: both nitrogens protonated (pKa > 7; charged)
```

```python
def assign_his_tautomers(structure, ph=7.0, propka_output="propka.dat"):
    """
    Parse PropKa output and assign histidine protonation states.
    Returns dict: {residue_number: 'HIE'|'HID'|'HIP'}
    """
    his_states = {}

    with open(propka_output) as f:
        for line in f:
            if "HIS" in line and "pKa" in line:
                parts = line.split()
                resnum = int(parts[1])
                pka = float(parts[-1])
                if pka < ph - 1:
                    his_states[resnum] = "HIE"  # deprotonated
                elif pka > ph + 1:
                    his_states[resnum] = "HIP"  # protonated
                else:
                    his_states[resnum] = "HIE"  # borderline: default HIE
                    print(f"WARNING: HIS {resnum} pKa={pka:.1f} near pH {ph}; "
                          f"check manually — may affect active site")

    return his_states
```

## Water Model and Solvation

The choice of water model must match the force field:

| Force field | Recommended water | Alternative |
|---|---|---|
| AMBER ff14SB/ff19SB | TIP3P | OPC (more accurate) |
| CHARMM36m | TIP3P (modified) | SPC/E |
| OpenFF (SMIRNOFF) | TIP3P-FB | OPC |

```bash
# GROMACS: full system preparation workflow

# 1. Convert PDB to GROMACS topology
gmx pdb2gmx -f protein.pdb \
            -o protein_processed.gro \
            -water tip3p \
            -ff charmm36m-iw \
            -ignh         # ignore existing H atoms; rebuild from scratch
            # Interactive: select histidine protonation states

# 2. Define simulation box (rhombic dodecahedron, 1.2 nm from protein edge)
gmx editconf -f protein_processed.gro \
             -o protein_box.gro \
             -bt dodecahedron \
             -d 1.2

# 3. Solvate
gmx solvate -cp protein_box.gro \
            -cs spc216.gro \
            -o protein_solvated.gro \
            -p topol.top

# 4. Add ions (neutralize + physiological NaCl concentration)
gmx grompp -f ions.mdp -c protein_solvated.gro -p topol.top -o ions.tpr
gmx genion -s ions.tpr \
           -o protein_ions.gro \
           -p topol.top \
           -neutral \       # neutralize system charge
           -conc 0.15 \    # 150 mM NaCl
           -pname NA \
           -nname CL

# Check: final system summary
gmx editconf -f protein_ions.gro -o /dev/null
```

## Membrane System Preparation (CHARMM-GUI)

For membrane proteins, manual preparation is complex. **CHARMM-GUI Membrane Builder** automates:
1. Orient protein in membrane (PPM server-based orientation)
2. Build lipid bilayer with specified composition (POPC, POPE, cholesterol, etc.)
3. Solvate with water and ions on both leaflets
4. Generate GROMACS, AMBER, NAMD, or OpenMM input files

```python
# After downloading CHARMM-GUI output for GROMACS:
# Directory structure:
# charmm-gui-xxxx/
#   gromacs/
#     step5_input.gro     # system coordinates
#     topol.top           # topology
#     step6.0_minimization.mdp  # minimization settings
#     step6.1_equilibration.mdp # NVT equilibration
#     step6.2_equilibration.mdp # NPT equilibration (various stages)
#     step7_production.mdp      # production

import subprocess
import os

def run_charmm_gui_gromacs_workflow(charmm_gui_dir):
    """Execute the CHARMM-GUI GROMACS protocol."""
    gromacs_dir = os.path.join(charmm_gui_dir, "gromacs")

    steps = [
        # (mdp, gro_in, gro_out, description)
        ("step6.0_minimization.mdp", "step5_input.gro",
         "step6.0.gro", "Energy minimization"),
        ("step6.1_equilibration.mdp", "step6.0.gro",
         "step6.1.gro", "NVT equilibration (restraints)"),
        ("step6.2_equilibration.mdp", "step6.1.gro",
         "step6.2.gro", "NPT equilibration (releasing restraints)"),
    ]

    for mdp, gro_in, gro_out, desc in steps:
        print(f"\n{'='*50}")
        print(f"Step: {desc}")
        tpr = gro_out.replace(".gro", ".tpr")

        subprocess.run([
            "gmx", "grompp",
            "-f", os.path.join(gromacs_dir, mdp),
            "-c", os.path.join(gromacs_dir, gro_in),
            "-p", os.path.join(gromacs_dir, "topol.top"),
            "-o", os.path.join(gromacs_dir, tpr)
        ], check=True)

        subprocess.run([
            "gmx", "mdrun", "-v",
            "-deffnm", os.path.join(gromacs_dir, gro_out.replace(".gro", ""))
        ], check=True)
        print(f"Completed: {desc}")
```

## Common Preparation Mistakes and How to Detect Them

```python
def preparation_checklist(gro_file, top_file):
    """Automated checks before running production MD."""
    import subprocess

    checks = {
        "System charge neutral": False,
        "Reasonable number of waters": False,
        "No overlapping atoms (check after minimization)": False,
    }

    # Check charge from topology
    result = subprocess.run(["gmx", "grompp", "-f", "ions.mdp",
                             "-c", gro_file, "-p", top_file,
                             "-o", "/tmp/check.tpr", "-maxwarn", "10"],
                            capture_output=True, text=True)
    if "total charge" in result.stderr.lower():
        for line in result.stderr.split("\n"):
            if "System has non-zero total charge" in line:
                print(f"WARNING: {line}")
            elif "total charge" in line.lower():
                print(f"INFO: {line}")

    # Estimate number of waters (should be > 1000 for small proteins)
    with open(top_file) as f:
        for line in f:
            if line.startswith("SOL"):
                n_water = int(line.split()[-1])
                checks["Reasonable number of waters"] = n_water > 1000
                print(f"Water molecules: {n_water}")
    return checks
```

## Why This Matters

System preparation failures are the leading cause of MD simulation problems. An incorrect protonation state at the active site of an enzyme will produce a simulation where the catalytic mechanism is energetically inaccessible — the simulation looks fine but the biochemistry is wrong. Insufficient box size causes the protein to interact with its own periodic image, introducing artificial constraints on conformational flexibility. Neglecting to add ions produces an unphysical zero-salt environment that alters electrostatic screening throughout the simulation. Careful preparation, guided by structural biology knowledge and verified by simple diagnostic checks, is what separates meaningful simulations from expensive noise generation.
