# What Molecular Dynamics Simulates

In 1977, a paper appeared in Nature describing the first molecular dynamics simulation of a protein — bovine pancreatic trypsin inhibitor, a small 58-residue molecule — run for a grand total of 9.2 picoseconds. The authors, McCammon, Gelin, and Karplus, concluded that proteins are not rigid objects but dynamic ensembles of interconverting structures. This was a conceptual revolution: the static pictures in crystallography textbooks were snapshots of something that never stood still. Today, MD simulations of the same protein run for milliseconds on a single GPU. But the central idea is unchanged — you let atoms move according to Newton's laws, record where they go, and read the biology off the trajectory.

What you get from an MD simulation is a **trajectory** — a time-ordered sequence of atomic coordinates from which structural, thermodynamic, and kinetic properties can be extracted. MD bridges the gap between static structural biology (X-ray crystallography, cryo-EM) and the dynamic molecular processes that underlie biological function.

## The Physical Basis

MD treats atoms as classical particles obeying Newton's second law. For each atom $i$ with mass $m_i$ at position $\mathbf{r}_i$:

$$\mathbf{F}_i = m_i \mathbf{a}_i = m_i \ddot{\mathbf{r}}_i = -\frac{\partial V(\mathbf{r})}{\partial \mathbf{r}_i}$$

where $V(\mathbf{r})$ is the **potential energy function** (force field) evaluated over all atomic coordinates. Forces are computed as the negative gradient of this potential; positions and velocities are updated at each time step $\Delta t$ (typically 1–2 femtoseconds).

The simulation produces $(N_\text{atoms} \times 3)$ coordinates at each step. For a 50,000-atom solvated protein simulated for 1 µs at 2 fs steps, this yields $5 \times 10^8$ frames — requiring compressed trajectory formats (XTC, DCD) and efficient analysis pipelines.

## What MD Can and Cannot Do

**MD directly provides:**
- Atomic-resolution structural dynamics: local conformational fluctuations, loop motions, side-chain rotations
- Binding and unbinding events (for timescales accessible to simulation)
- Solvent behavior, ion placement, membrane dynamics
- Thermodynamic averages: free energies, entropy, heat capacity
- Kinetic information: transition rates, diffusion coefficients

**MD cannot directly provide (without special techniques):**
- Events on timescales longer than the simulation (protein folding for large proteins, allosteric transitions requiring ms)
- Quantum mechanical effects: bond breaking/formation, electronic polarization, proton transfer
- Biological processes requiring cellular context (crowding, membrane-to-membrane signaling, gene expression)

## Timescales and Length Scales

Understanding what is accessible to MD requires knowing the relevant biological timescales:

| Process | Typical timescale | Accessible by standard MD? |
|---|---|---|
| Bond vibration | 10–100 fs | Yes (but integrates over) |
| Side-chain rotation | 1–100 ps | Yes |
| Loop motion | 1–10 ns | Yes (100 ns – 1 µs simulations) |
| Protein domain motion | 10 ns–1 µs | Yes (with long simulations) |
| Protein folding (small) | 1 µs–1 ms | Marginal; enhanced sampling needed |
| Receptor-ligand binding | 1 ms–1 s | No; use enhanced sampling or estimation |
| Enzyme catalysis | Variable | No (QM/MM needed for bond breaking) |

## A Minimal MD Simulation in Python

The following illustrates the core algorithm without a force field — a simple Lennard-Jones fluid — to make the logic of MD explicit before using production software:

```python
import numpy as np
import matplotlib.pyplot as plt

def lj_energy_forces(positions, box_length, epsilon=1.0, sigma=1.0):
    """
    Compute Lennard-Jones potential energy and forces for N particles.
    Uses minimum image convention for periodic boundary conditions.
    positions: (N, 3) array; box_length: scalar (cubic box)
    """
    N = len(positions)
    forces = np.zeros_like(positions)
    energy = 0.0

    for i in range(N - 1):
        for j in range(i + 1, N):
            rij = positions[i] - positions[j]
            # Minimum image convention
            rij -= box_length * np.round(rij / box_length)
            r2 = np.dot(rij, rij)

            if r2 < (3.0 * sigma) ** 2:  # cutoff at 3σ
                sr6 = (sigma ** 2 / r2) ** 3
                sr12 = sr6 ** 2
                energy += 4 * epsilon * (sr12 - sr6)

                # Force: -dV/dr projected onto rij direction
                f_mag = 24 * epsilon * (2 * sr12 - sr6) / r2
                fvec = f_mag * rij
                forces[i] += fvec
                forces[j] -= fvec  # Newton's third law

    return energy, forces

def velocity_verlet_step(positions, velocities, forces, masses, dt):
    """One step of the velocity Verlet integrator."""
    # Update positions
    acc = forces / masses[:, np.newaxis]
    positions_new = positions + velocities * dt + 0.5 * acc * dt**2
    return positions_new, acc  # velocities updated after new forces computed

def md_simulation(N=100, box_length=10.0, dt=0.005, n_steps=5000, T_target=1.0):
    """Simple NVE Lennard-Jones MD."""
    # Random initial positions (within box)
    rng = np.random.default_rng(42)
    positions = rng.uniform(0, box_length, size=(N, 3))
    # Maxwell-Boltzmann initial velocities
    masses = np.ones(N)
    velocities = rng.normal(0, np.sqrt(T_target), size=(N, 3))
    velocities -= velocities.mean(axis=0)  # remove center-of-mass drift

    _, forces = lj_energy_forces(positions, box_length)

    energies = []
    for step in range(n_steps):
        positions, acc_old = velocity_verlet_step(positions, velocities, forces, masses, dt)
        positions %= box_length  # apply PBC

        _, forces_new = lj_energy_forces(positions, box_length)
        # Complete velocity update
        velocities += 0.5 * (acc_old + forces_new / masses[:, np.newaxis]) * dt
        forces = forces_new

        if step % 100 == 0:
            ke = 0.5 * np.sum(masses[:, np.newaxis] * velocities**2)
            pe, _ = lj_energy_forces(positions, box_length)
            energies.append(ke + pe)

    return energies

# Run and visualize energy conservation
energies = md_simulation(N=64, n_steps=10000)
plt.plot(energies, lw=0.8)
plt.xlabel("Frame (×100 steps)")
plt.ylabel("Total energy (LJ units)")
plt.title("NVE MD: total energy should be conserved")
plt.tight_layout()
```

This toy code would be far too slow for real proteins — production MD uses neighbor lists, GPU acceleration, and optimized force computation — but the algorithm is identical in structure to GROMACS, AMBER, and OpenMM.

## The MD Ensemble

MD simulations sample different statistical ensembles depending on what is held constant:
- **NVE** (microcanonical): constant number $N$, volume $V$, energy $E$. Energy is conserved; used for testing integrators.
- **NVT** (canonical): constant $N$, $V$, temperature $T$. A thermostat adds/removes heat. Used for equilibration and many analyses.
- **NPT** (isothermal-isobaric): constant $N$, pressure $P$, $T$. Box size fluctuates; most realistic for biomolecules in solution. Standard for production simulations.

## Why This Matters

MD simulation generates mechanistic hypotheses that are impossible to extract from static structures alone. A crystal structure of an enzyme shows one conformation; an MD trajectory reveals how the active site opens and closes, which residues are flexible, and how a ligand enters and explores the binding pocket. These dynamic insights guide rational drug design, explain allosteric mechanisms, and reveal cryptic binding sites not visible in any experimental structure. With GPU acceleration, microsecond-scale simulations of 100,000-atom systems are now routine on a single workstation.
