# Time Step Selection in Molecular Dynamics

Every oscillation in a protein vibrates at a characteristic frequency. A C-H bond stretches and compresses roughly 90 trillion times per second. A backbone torsion angle rotates back and forth about 3 trillion times per second. To simulate these motions accurately, your time step must be short enough to resolve the fastest one — otherwise the integrator overshoots, atoms overlap, and the simulation explodes in a shower of infinite energies. But spend all your time resolving C-H vibrations and you will never accumulate enough simulation time to see a domain motion or a ligand unbinding event. The **time step** $\Delta t$ is one of the most consequential parameters in an MD simulation, and choosing it correctly requires understanding which timescales you actually need to resolve.

## The Nyquist Criterion for MD

The fastest relevant motion in the system sets the upper bound on $\Delta t$. A stable integrator requires approximately 20–50 steps per oscillation period of the fastest degree of freedom. The Nyquist criterion, by analogy with signal processing, requires at minimum 2 steps per period, but MD requires more for numerical stability and accuracy.

The fastest bond vibrations in biomolecules are X-H stretches (X = C, N, O):

| Bond type | Frequency (cm$^{-1}$) | Period (fs) | Max $\Delta t$ (fs) |
|---|---|---|---|
| C-H stretch | ~3000 | 11 | ~0.5 |
| O-H stretch | ~3200–3550 | 9–10 | ~0.5 |
| N-H stretch | ~3300 | ~10 | ~0.5 |
| C=O stretch | ~1750 | ~19 | ~1 |
| C-C stretch | ~1000 | ~33 | ~1.5 |
| Backbone torsion | ~100 | ~330 | ~15 |

Without any constraints, a time step of 0.5–1 fs is required. This is prohibitively expensive for microsecond-scale simulations.

## Standard Time Steps in Practice

| Setup | Constraints | $\Delta t$ | Notes |
|---|---|---|---|
| No constraints | None | 0.5–1 fs | Only for special cases (e.g., testing) |
| H-bond constraints | LINCS/SHAKE on X-H | **2 fs** | Standard; most common choice |
| All-bond constraints | LINCS on all bonds | 2–3 fs | Marginally faster |
| H-mass repartitioning (HMR) | All-bond + HMR | **4 fs** | Growing standard for long simulations |
| Coarse-grained | MARTINI beads | 10–40 fs | Faster motions removed by coarse-graining |

**H-mass repartitioning** transfers some mass from heavy atoms (C, N, O) to bonded hydrogens, slowing H motion and allowing larger time steps without changing the physics of the system at biological timescales. AMBER and OpenMM support HMR natively; GROMACS supports it via topology modification.

## Monitoring Simulation Stability

Energy drift is the primary diagnostic for time step appropriateness:

```python
import numpy as np
import matplotlib.pyplot as plt

def analyze_energy_conservation(energy_file, dt_ps=0.002, skip_equilibration_ns=1.0):
    """
    Analyze energy conservation from GROMACS energy output.
    energy_file: numpy array with columns [time_ps, potential, kinetic, total]
    """
    data = np.loadtxt(energy_file, comments=["#", "@"])
    time_ns = data[:, 0] / 1000.0  # ps -> ns
    total_energy = data[:, 3]      # kJ/mol

    # Skip equilibration
    mask = time_ns > skip_equilibration_ns
    t_prod = time_ns[mask]
    E_prod = total_energy[mask]

    # Energy drift: linear fit over production
    coeffs = np.polyfit(t_prod, E_prod, 1)
    drift_per_ns = coeffs[0]  # kJ/mol/ns

    # Energy fluctuations: standard deviation
    E_mean = E_prod.mean()
    E_std = E_prod.std()
    E_fluctuation = E_std / abs(E_mean) * 100  # as % of mean

    print(f"Energy drift: {drift_per_ns:.3f} kJ/mol/ns")
    print(f"  Acceptable: |drift| < 0.1 kJ/mol/ns per degree of freedom")
    print(f"Energy fluctuation: {E_fluctuation:.2f}% (normal: 0.01–0.1%)")
    print(f"  Large fluctuation -> dt too large or system issue")

    fig, axes = plt.subplots(2, 1, figsize=(10, 6))
    axes[0].plot(t_prod, E_prod, lw=0.5, color="navy")
    axes[0].plot(t_prod, np.polyval(coeffs, t_prod), 'r--', label=f"Drift: {drift_per_ns:.3f} kJ/mol/ns")
    axes[0].set_ylabel("Total energy (kJ/mol)")
    axes[0].legend()

    axes[1].plot(t_prod, E_prod - np.polyval(coeffs, t_prod), lw=0.5, color="steelblue")
    axes[1].set_xlabel("Time (ns)")
    axes[1].set_ylabel("Detrended energy (kJ/mol)")

    plt.tight_layout()
    return drift_per_ns, E_fluctuation

# Acceptable thresholds (GROMACS guidelines):
# |drift| < 0.01 kJ/mol/ns per atom for production NVE
# Fluctuations: normal for NVT/NPT; only NVE should be perfectly flat
```

## Worked Example: GROMACS mdp Settings for Time Step

```bash
# GROMACS .mdp file: production run at 2 fs with H-bond constraints
integrator        = md          ; velocity Verlet (md = leap-frog in GROMACS terminology)
dt                = 0.002       ; time step in ps (= 2 fs)
nsteps            = 500000000   ; 500M steps × 2 fs = 1 µs
nstxout-compressed = 5000       ; save coordinates every 5000 × 2 fs = 10 ps

; Constraints
constraints       = h-bonds     ; constrain all X-H bonds
constraint_algorithm = lincs
lincs_iter        = 1
lincs_order       = 4

; Bond parameters
continuation      = yes         ; restart from NPT equilibration
```

```bash
# H-mass repartitioning (4 fs time step): modify AMBER topology
# Using ParmEd:
python3 -c "
import parmed as pmd
struct = pmd.load_file('system.prmtop', 'system.inpcrd')
action = pmd.tools.HMassRepartition(struct, '3.0')  # transfer mass to H up to 3.0 amu
action.execute()
struct.save('system_hmr.prmtop')
print('HMR applied: H masses increased')
"

# Verify: all H atoms should now have mass ~3 amu
python3 -c "
import parmed as pmd
s = pmd.load_file('system_hmr.prmtop')
h_masses = [a.mass for a in s.atoms if a.element == 1]
print(f'H mass range: {min(h_masses):.2f}–{max(h_masses):.2f} amu')
"
```

## Effect of Time Step on Sampling Quality

A smaller time step is not always better — it simply samples the same phase space more finely without exploring more conformational states in a given wall-clock time. The relevant question is: what is the fastest degree of freedom you *need* to resolve?

For most biomolecular applications:
- **Conformational sampling**: time step irrelevant as long as simulation is stable; 2–4 fs is optimal
- **Vibrational spectroscopy** (IR, Raman): must use 0.5–1 fs to resolve bond vibration frequencies
- **Free energy calculations** (FEP): 2 fs; time step error in free energies is negligible compared to force field error
- **Coarse-grained models** (MARTINI): 10–40 fs; much faster harmonic modes absent

## Why This Matters

The time step directly determines the ratio of simulated time to wall-clock time — the "throughput" of the simulation. At 2 fs with a modern GPU, a 100,000-atom system might achieve ~400 ns/day. At 4 fs (HMR), this doubles to ~800 ns/day. For a target of 1 µs, that is the difference between 2.5 days and 5 days of GPU time. Over many projects, these differences become decisive. More importantly, choosing too large a time step — and failing to diagnose the resulting energy drift — leads to sampling artifacts that can be mistaken for genuine conformational dynamics.
