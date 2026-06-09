# Integrators in Molecular Dynamics

You have Newton's law: force equals mass times acceleration. You have the force field: a recipe for computing forces from positions. Now you need to propagate those positions forward in time. The mathematical machine that does this is the **integrator** — the numerical algorithm that advances atomic positions and velocities in time. Given current positions $\mathbf{r}(t)$ and forces $\mathbf{F}(t)$, the integrator computes $\mathbf{r}(t + \Delta t)$. The choice of integrator affects energy conservation, computational cost, and whether the simulation samples the correct statistical ensemble. It also answers a question that turns out to be subtler than it looks: how do you discretize a continuous differential equation without slowly destroying the physics?

## Requirements for a Good MD Integrator

A practical MD integrator should satisfy:
1. **Time reversibility**: the algorithm must work identically forward and backward in time (required for microcanonical ensemble sampling)
2. **Symplecticity**: preserves the phase-space volume element $d\mathbf{r} \, d\mathbf{p}$ (prevents artificial energy drift)
3. **Stability**: does not accumulate unbounded numerical errors
4. **Efficiency**: minimize force evaluations per step (force computation is 90-99% of MD cost)

## The Verlet Algorithm

The **Verlet integrator** is derived from a Taylor expansion of position:

$$\mathbf{r}(t + \Delta t) = \mathbf{r}(t) + \mathbf{v}(t)\Delta t + \frac{\mathbf{F}(t)}{2m}\Delta t^2 + O(\Delta t^4)$$

$$\mathbf{r}(t - \Delta t) = \mathbf{r}(t) - \mathbf{v}(t)\Delta t + \frac{\mathbf{F}(t)}{2m}\Delta t^2 - O(\Delta t^4)$$

Adding these two equations eliminates the velocity:

$$\mathbf{r}(t + \Delta t) = 2\mathbf{r}(t) - \mathbf{r}(t - \Delta t) + \frac{\mathbf{F}(t)}{m}\Delta t^2$$

The Verlet algorithm is **time-reversible** (swapping $t+\Delta t$ and $t-\Delta t$ recovers the same equation) and is a **symplectic** integrator. Its position accuracy is $O(\Delta t^4)$, but it requires storing two sets of positions ($t$ and $t-\Delta t$) and does not explicitly compute velocities (problematic for extracting kinetic energy and using thermostats).

## The Leapfrog Algorithm

The **leapfrog** algorithm is an equivalent reformulation that uses velocities at half-integer time steps:

$$\mathbf{v}(t + \tfrac{1}{2}\Delta t) = \mathbf{v}(t - \tfrac{1}{2}\Delta t) + \frac{\mathbf{F}(t)}{m}\Delta t$$

$$\mathbf{r}(t + \Delta t) = \mathbf{r}(t) + \mathbf{v}(t + \tfrac{1}{2}\Delta t)\Delta t$$

Positions and velocities "leapfrog" over each other — velocities are at half-steps, positions at full steps. This is the algorithm used by default in **GROMACS**. The leapfrog is mathematically equivalent to the Verlet algorithm with the same accuracy and conservation properties.

## The Velocity Verlet Algorithm

The **velocity Verlet** algorithm eliminates the half-step offset by computing velocities at full time steps. It requires two force evaluations per step (or one if forces from the previous step are saved):

$$\mathbf{r}(t + \Delta t) = \mathbf{r}(t) + \mathbf{v}(t)\Delta t + \frac{\mathbf{F}(t)}{2m}\Delta t^2$$

$$\mathbf{v}(t + \Delta t) = \mathbf{v}(t) + \frac{\mathbf{F}(t) + \mathbf{F}(t + \Delta t)}{2m}\Delta t$$

The update proceeds in two half-steps:
1. Advance positions; compute new forces $\mathbf{F}(t + \Delta t)$
2. Average old and new forces to update velocities

This form is used in **NAMD** and **OpenMM**. Unlike the Verlet algorithm, velocity Verlet avoids numerical cancellation when computing velocities from position differences (which becomes problematic at small $\Delta t$).

## Worked Example: Velocity Verlet Implementation

```python
import numpy as np

def velocity_verlet(positions, velocities, forces, masses, compute_forces, dt):
    """
    Single step of velocity Verlet integration.
    
    Args:
        positions: (N, 3) array in nm
        velocities: (N, 3) array in nm/ps
        forces: (N, 3) array in kJ/mol/nm (from previous step)
        masses: (N,) array in amu
        compute_forces: callable(positions) -> (energy, forces)
        dt: time step in ps
    Returns:
        new positions, new velocities, new forces, new energy
    """
    inv_mass = 1.0 / masses[:, np.newaxis]  # broadcast over x,y,z

    # Step 1: Update positions using current velocities and forces
    acc = forces * inv_mass  # acceleration = F/m
    new_positions = positions + velocities * dt + 0.5 * acc * dt**2

    # Step 2: Compute new forces at updated positions
    new_energy, new_forces = compute_forces(new_positions)

    # Step 3: Update velocities using average of old and new acceleration
    new_acc = new_forces * inv_mass
    new_velocities = velocities + 0.5 * (acc + new_acc) * dt

    return new_positions, new_velocities, new_forces, new_energy

def run_md(initial_positions, initial_velocities, masses,
           compute_forces, dt=0.002, n_steps=10000):
    """
    Run NVE MD using velocity Verlet. Reports energy conservation.
    """
    positions = initial_positions.copy()
    velocities = initial_velocities.copy()
    energy, forces = compute_forces(positions)

    energies = []
    for step in range(n_steps):
        positions, velocities, forces, pe = velocity_verlet(
            positions, velocities, forces, masses, compute_forces, dt
        )
        ke = 0.5 * np.sum(masses[:, np.newaxis] * velocities**2)
        total_energy = pe + ke
        energies.append(total_energy)

        if step % 1000 == 0:
            drift = abs(total_energy - energies[0]) / abs(energies[0])
            print(f"Step {step:6d}: E = {total_energy:.4f}, drift = {drift:.2e}")

    return positions, velocities, np.array(energies)

# Energy conservation test: drift should be < 1e-4 for dt = 2 fs
# Larger dt -> more drift -> simulation instability
```

## Constraint Algorithms: LINCS and SHAKE

The fastest bond vibrations in proteins (C-H, O-H stretches) have frequencies of ~3000 cm$^{-1}$, corresponding to periods of ~11 fs. With a time step of 2 fs, we need at least 5-6 steps per oscillation period — barely adequate. Constraining these bonds (fixing bond lengths at their equilibrium values) removes the fastest degrees of freedom and allows larger time steps.

**SHAKE** (GROMACS/AMBER): iterative algorithm that satisfies bond length constraints by adjusting positions after each Verlet step. Converges within 3-10 iterations for typical proteins.

**LINCS** (Linear Constraint Solver, GROMACS default): non-iterative; projects out constrained motions using a linear transformation. Faster than SHAKE; preferred for parallel simulations.

```bash
# GROMACS mdp file excerpt for constrained simulation
constraints = h-bonds    ; constrain H-X bonds only (allows dt = 2 fs)
constraint_algorithm = LINCS
lincs_iter = 1           ; one corrective rotation step
lincs_order = 4          ; expansion order; higher = more accurate

; Alternatively:
constraints = all-bonds  ; all bonds constrained (allows dt = 4 fs with H-mass repartitioning)
```

## Runge-Kutta Integrators: Why Not Used

Students familiar with ODE solvers may wonder why higher-order Runge-Kutta methods (RK4, RK45) are not used in MD. The answer is two-fold:
1. **Force evaluations are expensive**: RK4 requires 4 force evaluations per step vs. 1 for velocity Verlet. Accuracy per force evaluation favors Verlet.
2. **Symplecticity**: standard Runge-Kutta methods are not symplectic — they do not conserve phase-space volume — leading to systematic energy drift in long simulations. Velocity Verlet is symplectic.

## Why This Matters

The choice between leapfrog and velocity Verlet is largely implementation-level — they are equivalent in accuracy and conservation. What matters more is understanding the limits of the time step and the role of constraints. Using a 4 fs time step without constraints will cause immediate instability. Using LINCS with 2 fs allows 2× faster sampling per wall-clock hour. For enhanced sampling methods (Chapter 4.3 Section 4), some methods require access to velocities at integer time steps, making velocity Verlet preferable over leapfrog. Understanding integrators provides the foundation for reasoning about simulation stability and the validity of energy-based analyses.
