# MD Software: GROMACS, AMBER, NAMD, and OpenMM

There is something reassuring about the fact that four completely independent software packages — written by different groups, in different programming languages, optimized for different hardware — all implement the same equations of motion and, when given the same force field and starting structure, produce statistically identical trajectories. The physics is not in the software; the software is merely a vehicle for the physics. Four major software packages dominate biomolecular MD simulation: GROMACS, AMBER, NAMD, and OpenMM. They share the same fundamental algorithms but differ in force field support, performance characteristics, programming interfaces, licensing, and community ecosystems. Choosing the right tool depends on system type, computational environment, and downstream analysis needs.

## GROMACS

**GROMACS** (GROningen MAchine for Chemical Simulations) is an open-source, high-performance MD engine originally developed at the University of Groningen. It is the most widely used MD package in academic settings.

**Strengths:**
- Fastest performance for protein/membrane simulations on CPU and GPU clusters
- Excellent CHARMM force field support; AMBER parameters can be converted
- Comprehensive analysis tools (`gmx energy`, `gmx rms`, `gmx trjconv`, etc.)
- Active community; extensive documentation and tutorials
- Native support for PLUMED (enhanced sampling)

**Limitations:**
- Command-line oriented; no native Python API for controlling running simulations
- GPU PME requires NVIDIA GPU with recent CUDA (less flexible than OpenMM)
- Not ideal for QM/MM or polarizable force fields

```bash
# Typical GROMACS workflow
# 1. Prepare topology
gmx pdb2gmx -f protein.pdb -ff charmm36m-iw -water tip3p -o protein.gro

# 2. Energy minimization
gmx grompp -f em.mdp -c protein.gro -p topol.top -o em.tpr
gmx mdrun -v -deffnm em

# 3. NVT equilibration
gmx grompp -f nvt.mdp -c em.gro -r em.gro -p topol.top -o nvt.tpr
gmx mdrun -ntmpi 1 -ntomp 8 -gpu_id 0 -deffnm nvt

# 4. NPT equilibration
gmx grompp -f npt.mdp -c nvt.gro -r nvt.gro -t nvt.cpt -p topol.top -o npt.tpr
gmx mdrun -ntmpi 1 -ntomp 8 -gpu_id 0 -deffnm npt

# 5. Production
gmx grompp -f md.mdp -c npt.gro -t npt.cpt -p topol.top -o md.tpr
gmx mdrun -ntmpi 1 -ntomp 8 -gpu_id 0 -deffnm md -maxh 24

# 6. Basic analysis
echo "Potential" | gmx energy -f md.edr -o potential.xvg
echo "C-alpha" | gmx rms -s md.tpr -f md.xtc -o rmsd.xvg
```

## AMBER

**AMBER** (Assisted Model Building with Energy Refinement) consists of a force field library (the "AMBER force fields") and simulation programs (sander, pmemd, pmemd.cuda). The AMBER force fields (ff14SB, ff19SB, GAFF2) are the gold standard for protein simulations.

**Strengths:**
- Best protein force fields (ff14SB, ff19SB, OL3 for RNA)
- `pmemd.cuda`: fastest single-GPU MD implementation; often 20–30% faster than GROMACS on GPU
- Tight integration with AMBER force field ecosystem
- `MMPBSA.py` for binding free energy; `cpptraj` for analysis
- H-mass repartitioning natively supported

**Limitations:**
- Commercial license (AmberTools free; AMBER itself ~\$500)
- Less convenient for non-AMBER force fields (CHARMM requires conversion)
- Analysis tools (cpptraj) have a learning curve

```python
# AMBER Python API via ParmEd for topology manipulation
import parmed as pmd

# Load AMBER system
struct = pmd.load_file("system.prmtop", "system.inpcrd")

# Apply H-mass repartitioning
action = pmd.tools.HMassRepartition(struct, "3.0")
action.execute()
struct.save("system_hmr.prmtop")

# Inspect system
print(f"Atoms: {len(struct.atoms)}")
print(f"Residues: {len(struct.residues)}")
print(f"Total charge: {sum(a.charge for a in struct.atoms):.3f} e")

# Convert AMBER topology to GROMACS format
pmd.gromacs.GromacsTopologyFile.from_structure(struct).write("topol.top")
struct.save("system.gro")
```

## NAMD

**NAMD** (Nanoscale MD) is developed at the University of Illinois Urbana-Champaign (Theoretical and Computational Biophysics Group). It excels at large-scale parallel simulations and has strong VMD integration.

**Strengths:**
- Excellent scalability to thousands of CPU cores (Blue Waters, Frontier)
- Tightly integrated with VMD for visualization and scripting
- Supports CHARMM force fields natively (same group that develops CHARMM)
- GPU-resident mode (NAMD3): all forces computed on GPU; excellent for single-node
- Steered MD, alchemical FEP, collective variables (Colvars module) built in

**Limitations:**
- Configuration files (NAMD scripts) less intuitive than GROMACS mdp
- No Python API (Tcl scripting); less convenient for programmatic control
- Free for non-commercial academic use

```bash
# NAMD configuration file (namd.conf)
structure          system.psf       # CHARMM PSF topology
coordinates        system.pdb       # starting coordinates
parameters         par_all36m_prot.prm  # CHARMM36m protein parameters
parameters         toppar_water_ions.str
paraTypeCharmm     on

outputName         production       # prefix for output files
outputEnergies     500              # energy output frequency (steps)
dcdFreq            2500             # trajectory output every 5 ps (dt=2fs)
restartFreq        25000            # checkpoint every 50 ps

# Periodic boundary conditions
cellBasisVector1   80.0  0.0  0.0
cellBasisVector2   0.0   80.0 0.0
cellBasisVector3   0.0   0.0  80.0
cellOrigin         0.0   0.0  0.0

# Non-bonded settings
cutoff             12.0
switching          on
switchDist         10.0
pairListDist       14.0
PME                yes
PMEGridSpacing     1.0

# Thermostat (Langevin)
langevin           on
langevinDamping    1.0              # 1/ps friction coefficient
langevinTemp       310

# Barostat
LangevinPiston     on
LangevinPistonTarget 1.01325        # 1 bar
LangevinPistonPeriod 200
LangevinPistonDecay  100

# Run
timestep           2.0              # 2 fs
run                50000000         # 100 ns
```

## OpenMM

**OpenMM** is an open-source, Python-native MD library that treats MD as a programmable framework. Rather than an application you configure, OpenMM is a library you program.

**Strengths:**
- Full Python API: every simulation parameter is programmatically controllable
- Most flexible for custom force fields, collective variables, and novel simulation methods
- Best GPU performance for single-node (CUDA, OpenCL, Metal — cross-platform GPU support)
- Native integration with ML force fields (TorchMD-Net, NequIP, MACE via OpenMM-ML)
- Supports AMBER, CHARMM, OpenFF/SMIRNOFF force fields

**Limitations:**
- Requires more programming than GROMACS; not a push-button tool
- Analysis tools not built in (use MDAnalysis or MDTraj)
- Less optimized for multi-node HPC (GROMACS or NAMD better for many CPUs)

```python
from openmm import app, unit, LangevinMiddleIntegrator, MonteCarloBarostat
import openmm as mm
from openmm.app import PDBFile, ForceField, PME, HBonds

def setup_openmm_simulation(pdb_file, ff_files=["amber14-all.xml", "amber14/tip3pfb.xml"],
                             T_K=310, P_bar=1.0, dt_fs=2.0, cutoff_nm=1.2):
    """
    Set up a production NPT simulation in OpenMM.
    Returns a configured Simulation object ready to run.
    """
    # Load structure and force field
    pdb = PDBFile(pdb_file)
    ff = ForceField(*ff_files)

    # Create system
    system = ff.createSystem(
        pdb.topology,
        nonbondedMethod=PME,
        nonbondedCutoff=cutoff_nm * unit.nanometer,
        constraints=HBonds,
        hydrogenMass=4 * unit.amu          # H-mass repartitioning for 4 fs step
    )

    # NPT ensemble: Langevin thermostat + MC barostat
    integrator = LangevinMiddleIntegrator(
        T_K * unit.kelvin,
        1.0 / unit.picosecond,             # friction coefficient
        dt_fs * unit.femtoseconds
    )
    system.addForce(MonteCarloBarostat(P_bar * unit.bar, T_K * unit.kelvin))

    # Select fastest available platform
    for platform_name in ["CUDA", "OpenCL", "CPU"]:
        try:
            platform = mm.Platform.getPlatformByName(platform_name)
            print(f"Using platform: {platform_name}")
            break
        except Exception:
            continue

    simulation = app.Simulation(pdb.topology, system, integrator, platform)
    simulation.context.setPositions(pdb.positions)

    return simulation

def run_production(simulation, n_steps=50_000_000, report_interval=5000,
                   output_prefix="production"):
    """Run production MD with RMSD reporter."""
    from openmm.app import DCDReporter, StateDataReporter
    import sys

    # DCD trajectory (compact binary format)
    simulation.reporters.append(DCDReporter(f"{output_prefix}.dcd", report_interval))

    # State data: energy, temperature, pressure, box volume
    simulation.reporters.append(StateDataReporter(
        f"{output_prefix}.log",
        report_interval,
        step=True, time=True, potentialEnergy=True,
        kineticEnergy=True, temperature=True, density=True,
        speed=True  # ns/day
    ))

    # Also print to stdout for monitoring
    simulation.reporters.append(StateDataReporter(
        sys.stdout, report_interval * 10,
        step=True, time=True, temperature=True, density=True, speed=True
    ))

    print(f"Running {n_steps * 2e-6:.0f} ns production MD...")
    simulation.step(n_steps)
    print("Production complete.")
```

## Performance Comparison

For a typical 100,000-atom membrane protein system (single A100 GPU):

| Software | Speed (ns/day) | Notes |
|---|---|---|
| GROMACS | 350–450 | GPU PME; highly optimized |
| AMBER (pmemd.cuda) | 400–500 | Fastest for AMBER FF; CUDA-native |
| NAMD3 (GPU-resident) | 300–400 | Best for large systems; multi-node |
| OpenMM | 350–450 | Most flexible; cross-platform GPU |

All four codes give identical physics (same force fields, same algorithms); performance differences are implementation details that matter for large production runs but not for method development or short simulations.

## Choosing the Right Tool

| Situation | Best choice |
|---|---|
| Standard protein/membrane simulation, CHARMM FF | GROMACS |
| Protein simulation, AMBER FF, fastest GPU | AMBER (pmemd.cuda) |
| Large-scale HPC, many CPU cores | NAMD |
| Custom algorithms, ML force fields, Python workflows | OpenMM |
| Coarse-grained (MARTINI) | GROMACS |
| Enhanced sampling (metadynamics via PLUMED) | GROMACS + PLUMED |

## Why This Matters

The four major MD packages collectively simulate thousands of research systems every day. Understanding each package's strengths and APIs allows you to choose the right tool for each project and to switch between them as needed. More importantly, understanding that all four implement the same fundamental physics — Newton's equations with a force field — demystifies the software and focuses attention on the scientifically important choices: force field selection, simulation length, and analysis methodology.
