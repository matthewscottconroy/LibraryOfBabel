# Energy Minimization and Equilibration

Think of the starting structure as a spring-loaded system. You have taken an experimental crystal structure and painstakingly solvated it in water, added ions, and built a simulation box — but none of those steps know anything about the force field you are about to use. The bond lengths are slightly wrong. The solvation algorithm placed water molecules in positions that clash with side chains. The crystal packing contacts left the protein in a geometry that minimizes X-ray R-factors, not potential energy. If you start MD from this state, the forces on some atoms will be enormous — thousands of kJ/mol/nm — and the integrator will immediately produce infinite coordinates. Before collecting production data, every MD simulation must pass through energy minimization and equilibration. These preparatory stages resolve structural clashes introduced during system building, bring the system to the correct temperature and pressure, and allow solvent degrees of freedom to relax around the solute. Skipping or shortcutting these stages is a common source of simulation instability and artifacts.

## Energy Minimization

**Energy minimization** (EM) finds a local minimum of the potential energy surface starting from the prepared structure. It does not involve dynamics — there are no velocities or temperature — and it removes unrealistic high-energy configurations ("bad contacts") that would cause enormous forces and immediate instability if the simulation started directly from the prepared coordinates.

### Why Minimization Is Necessary

Crystal structures are not force-field minima. They represent the electron density averaged over many unit cells and may have:
- Slightly wrong bond lengths and angles relative to force field reference values
- Steric clashes between solvated protein and added water molecules
- Clashes at the protein-membrane interface (for membrane systems)
- Residue sidechains in geometrically reasonable but energetically unfavorable rotamers

Even if clashes are not visible to the eye, forces of thousands of kJ/mol/nm can develop from atoms 1.5 Å apart that should be 2.0 Å apart.

### Minimization Algorithms

**Steepest descent**: moves along the negative gradient of the potential energy. Simple and robust; converges reliably even for highly distorted geometries. Standard choice for initial minimization.

$$\mathbf{r}_{n+1} = \mathbf{r}_n - \alpha \nabla V(\mathbf{r}_n)$$

**Conjugate gradient**: uses gradient information from previous steps to choose better search directions. Converges faster than steepest descent near the minimum.

**L-BFGS**: quasi-Newton method; very efficient near the minimum. Used when very tight convergence is needed (free energy calculations, normal mode analysis).

```bash
# GROMACS: energy minimization mdp file
; em.mdp
integrator   = steep    ; steepest descent algorithm
nsteps       = 50000    ; max steps (usually converges in < 5000)
emtol        = 1000.0   ; convergence criterion: max force < 1000 kJ/mol/nm
emstep       = 0.01     ; initial step size (nm)

; Non-bonded settings (same as production)
cutoff-scheme = Verlet
coulombtype   = PME
rcoulomb      = 1.2
vdwtype       = Cut-off
rvdw          = 1.2

; No thermostat or barostat during minimization
```

```bash
# Run minimization
gmx grompp -f em.mdp -c protein_ions.gro -p topol.top -o em.tpr
gmx mdrun -v -deffnm em  # -v: verbose output (shows energy each step)

# Verify convergence
gmx energy -f em.edr -o potential_em.xvg
# Select "Potential" (option 10 typically)
# Should decrease monotonically and flatten
```

```python
import subprocess
import numpy as np

def check_minimization_convergence(edr_file):
    """Extract potential energy from GROMACS .edr and check convergence."""
    # Extract potential energy
    proc = subprocess.run(
        ["gmx", "energy", "-f", edr_file, "-o", "/tmp/em_potential.xvg"],
        input="Potential\n", capture_output=True, text=True
    )

    data = np.loadtxt("/tmp/em_potential.xvg", comments=["#", "@"])
    steps, potential = data[:, 0], data[:, 1]

    initial_pe = potential[0]
    final_pe = potential[-1]
    reduction = (initial_pe - final_pe) / abs(initial_pe) * 100

    print(f"Initial potential energy: {initial_pe:.1f} kJ/mol")
    print(f"Final potential energy:   {final_pe:.1f} kJ/mol")
    print(f"Energy reduction: {reduction:.1f}%")

    if potential[-1] > potential[-10]:
        print("WARNING: Energy increased at end — may not have converged")
    else:
        print("Minimization converged (energy decreasing at end)")
    return steps, potential
```

## NVT Equilibration: Temperature Coupling

After minimization, atoms have no velocities. The first equilibration phase (NVT) assigns random velocities from a Maxwell-Boltzmann distribution at the target temperature $T$ and runs with a thermostat while keeping volume fixed.

During NVT equilibration, the protein backbone is typically restrained with harmonic position restraints ($k = 1000$ kJ/mol/nm²) to prevent structural distortion while the solvent equilibrates.

```bash
; nvt.mdp — NVT equilibration (100 ps)
integrator    = md
nsteps        = 50000      ; 50,000 × 2 fs = 100 ps
dt            = 0.002      ; 2 fs time step

; Position restraints on protein heavy atoms
define        = -DPOSRES   ; activates position restraint block in topology

; Thermostat: V-rescale (canonical ensemble, unlike Berendsen)
tcoupl        = V-rescale
tc-grps       = Protein Non-Protein
tau_t         = 0.1   0.1  ; coupling time constants (ps)
ref_t         = 310   310  ; target temperature (K) for each group

; No barostat during NVT
pcoupl        = no
```

**Target temperature**: physiological simulations use 310 K (37°C). Crystal structure-based analyses sometimes use 298 K (room temperature). Never start at exactly 0 K — always assign velocities at the target T.

```python
def check_nvt_convergence(edr_file, target_T=310.0, tolerance=5.0):
    """
    Check that temperature has converged to target in NVT equilibration.
    """
    proc = subprocess.run(
        ["gmx", "energy", "-f", edr_file, "-o", "/tmp/nvt_T.xvg"],
        input="Temperature\n", capture_output=True, text=True
    )
    data = np.loadtxt("/tmp/nvt_T.xvg", comments=["#", "@"])
    time_ps, T = data[:, 0], data[:, 1]

    # Check last 50% of equilibration
    n = len(T)
    T_mean = T[n//2:].mean()
    T_std  = T[n//2:].std()

    print(f"Temperature (last 50%): {T_mean:.1f} ± {T_std:.1f} K")
    if abs(T_mean - target_T) < tolerance:
        print(f"PASS: converged to {target_T} K")
    else:
        print(f"FAIL: mean temperature {T_mean:.1f} K deviates from target {target_T} K")
    return T_mean, T_std
```

## NPT Equilibration: Pressure Coupling

NPT equilibration allows the box volume to adjust at constant pressure (1 bar). The protein backbone restraints are gradually released (or kept for part of this phase). This stage equilibrates the system density — a freshly solvated box may have slightly too many or too few water molecules at the correct density.

```bash
; npt.mdp — NPT equilibration (1 ns)
integrator    = md
nsteps        = 500000     ; 500,000 × 2 fs = 1 ns
dt            = 0.002

; Thermostat: Nosé-Hoover (for production; not Berendsen)
tcoupl        = Nose-Hoover
tc-grps       = Protein Non-Protein
tau_t         = 1.0   1.0
ref_t         = 310   310

; Barostat: Parrinello-Rahman (correct NPT fluctuations)
pcoupl        = Parrinello-Rahman
pcoupltype    = isotropic  ; isotropic for proteins; semi-isotropic for membranes
tau_p         = 2.0        ; pressure coupling time constant (ps)
ref_p         = 1.0        ; target pressure (bar)
compressibility = 4.5e-5   ; water compressibility (bar^-1)

; Release backbone restraints gradually:
; run NPT with define = -DPOSRES_FC_BB, then without any restraints
```

```python
def check_npt_convergence(edr_file):
    """Check density, pressure, and box volume convergence in NPT."""
    metrics = {}
    for quantity, xvg_file in [("Density", "/tmp/npt_density.xvg"),
                                 ("Pressure", "/tmp/npt_pressure.xvg")]:
        subprocess.run(
            ["gmx", "energy", "-f", edr_file, "-o", xvg_file],
            input=f"{quantity}\n", capture_output=True, text=True
        )
        data = np.loadtxt(xvg_file, comments=["#", "@"])
        values = data[len(data)//2:, 1]  # last 50%
        metrics[quantity] = {"mean": values.mean(), "std": values.std()}

    density = metrics["Density"]
    print(f"Density: {density['mean']:.2f} ± {density['std']:.2f} kg/m³")
    print(f"  Water at 310 K: ~993 kg/m³ (TIP3P gives ~1002 kg/m³)")
    if abs(density['mean'] - 993) < 50:
        print("  PASS: density reasonable for aqueous protein simulation")
    return metrics
```

## Multi-Stage Equilibration for Membrane Systems

Membrane simulations require more careful equilibration because lipid bilayers take much longer to equilibrate than water. CHARMM-GUI provides a 6-stage protocol:

| Stage | Duration | Restraints | Thermostat | Barostat |
|---|---|---|---|---|
| 1 (NVT) | 125 ps | protein + lipid backbone | Berendsen | None |
| 2 (NPT) | 125 ps | protein + lipid backbone | Berendsen | Berendsen |
| 3 (NPT) | 125 ps | protein backbone, lighter lipid | Berendsen | Berendsen |
| 4 (NPT) | 125 ps | protein backbone only | Berendsen | Berendsen |
| 5 (NPT) | 500 ps | protein backbone | Nosé-Hoover | Parrinello-Rahman |
| 6 (NPT) | 500 ps | no restraints | Nosé-Hoover | Parrinello-Rahman |

## Why This Matters

Running production simulations without proper equilibration is one of the most common mistakes in computational biology. The consequences are not always obvious: a simulation starting from an unequilibrated state may appear to run fine but produce artifacts in structural observables during the early trajectory (which must then be discarded as "equilibration" in the analysis — but if equilibration was not performed, how long should be discarded?). Adequate equilibration ensures that RMSD plots show a genuine plateau before data collection begins and that thermodynamic observables (density, pressure, temperature) have settled to their stationary values.
