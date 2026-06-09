# Thermostats and Barostats

When you put a protein in a test tube, it is not thermally isolated — it is bathed in a room-temperature environment, buffered at physiological pH, at atmospheric pressure. Thermodynamic quantities you measure in a calorimeter or fluorescence experiment reflect this ensemble: constant temperature, constant pressure, Boltzmann-weighted sampling of configurations. To make MD simulations match those conditions, you need to couple the simulation to a heat bath and a pressure reservoir. The choice of thermostat and barostat determines which statistical ensemble the simulation samples. A physically correct simulation of a protein in solution should sample the NPT (canonical-isothermal-isobaric) ensemble — constant number of atoms, pressure, and temperature — matching the conditions of most biochemical experiments. Choosing the wrong thermostat or barostat can subtly corrupt the statistical distribution of sampled configurations, producing incorrect thermodynamic averages even when the simulation appears stable.

## Temperature and Statistical Mechanics

In classical statistical mechanics, the **canonical ensemble** (NVT) is characterized by all configurations $\{\mathbf{r}, \mathbf{p}\}$ having probability proportional to the Boltzmann weight:

$$P(\mathbf{r}, \mathbf{p}) \propto e^{-H(\mathbf{r},\mathbf{p})/k_B T}$$

where $H = K + V$ is the Hamiltonian, $k_B$ is Boltzmann's constant, and $T$ is temperature. The instantaneous kinetic temperature is related to the kinetic energy $K$ by:

$$T_\text{inst} = \frac{2K}{N_\text{dof} k_B}$$

where $N_\text{dof} = 3N - 3$ (subtracting 3 for center-of-mass motion). A **thermostat** modifies atomic velocities to maintain $\langle T_\text{inst} \rangle = T_\text{target}$.

## Berendsen Thermostat

The **Berendsen thermostat** (Berendsen et al., 1984) rescales velocities to exponentially drive the temperature toward the target:

$$\frac{dT}{dt} = \frac{T_\text{target} - T}{\tau_T}$$

This is implemented as a velocity rescaling: at each step, multiply all velocities by a factor $\lambda$:

$$\lambda = \left[1 + \frac{\Delta t}{\tau_T}\left(\frac{T_\text{target}}{T} - 1\right)\right]^{1/2}$$

**Advantage**: very stable; quickly suppresses temperature fluctuations during equilibration.  
**Disadvantage**: does not generate the correct canonical ensemble. The distribution of kinetic energies is wrong — it suppresses the natural fluctuations that arise in a true NVT ensemble. The Berendsen thermostat should only be used for equilibration, never for production MD or free energy calculations.

## Velocity Rescaling (V-Rescale) Thermostat

The **V-rescale thermostat** (Bussi et al., 2007) is the GROMACS default for production. It corrects the Berendsen scheme by adding a stochastic term that restores the correct kinetic energy distribution:

$$dK = (K_\text{target} - K)\frac{dt}{\tau_T} + 2\sqrt{\frac{K K_\text{target}}{N_\text{dof}}} \frac{dW}{\sqrt{\tau_T}}$$

where $dW$ is a Wiener process (white noise). This generates the correct canonical ensemble while maintaining the stability of Berendsen. **Use V-rescale for all production simulations in GROMACS.**

## Nosé-Hoover Thermostat

The **Nosé-Hoover** (NH) thermostat is an extended system method: a virtual degree of freedom $s$ (the "heat bath") is added to the Hamiltonian with its own kinetic energy and potential:

$$H_\text{NH} = \sum_i \frac{\mathbf{p}_i^2}{2m_i} + V(\mathbf{r}) + \frac{p_s^2}{2Q} + N_\text{dof} k_B T \ln s$$

The heat bath exchanges energy with the system through friction-like coupling. The resulting equations of motion:

$$\dot{\mathbf{r}}_i = \frac{\mathbf{p}_i}{m_i}, \quad \dot{\mathbf{p}}_i = \mathbf{F}_i - \frac{p_s}{Q}\mathbf{p}_i, \quad \dot{p}_s = \sum_i \frac{\mathbf{p}_i^2}{m_i} - N_\text{dof} k_B T$$

NH generates the correct canonical ensemble with deterministic equations of motion. For proteins, the **Nosé-Hoover chain** (multiple coupled heat baths) is more ergodic. In NAMD and OpenMM, NH is the standard production thermostat.

```bash
; GROMACS mdp: thermostat comparison
; For equilibration:
tcoupl        = berendsen
tau_t         = 0.1 0.1
ref_t         = 310 310

; For production (choose one):
tcoupl        = V-rescale       ; preferred in GROMACS; correct ensemble
; OR:
tcoupl        = Nose-Hoover    ; also correct; default in NAMD/OpenMM
tau_t         = 1.0 1.0        ; longer tau for NH (more stable)
```

```python
import numpy as np
import matplotlib.pyplot as plt

def compare_thermostat_distributions(T_berendsen, T_vrescale, T_target=310.0, n_dof=150000):
    """
    Compare temperature distributions from Berendsen vs V-rescale thermostats.
    The correct chi-squared distribution for kinetic energy gives:
    P(T) ~ T^(n_dof/2 - 1) exp(-n_dof * T / (2*T_target))
    """
    # Expected distribution of temperature (chi-squared / n_dof)
    T_range = np.linspace(T_target - 5, T_target + 5, 1000)
    from scipy.stats import chi2
    # Kinetic energy ~ chi2(n_dof) * k_B * T_target / 2
    # Temperature = 2K / (n_dof * k_B) ~ chi2(n_dof) * T_target / n_dof
    sigma_expected = T_target * np.sqrt(2 / n_dof)
    print(f"Expected T fluctuation (canonical): σ = {sigma_expected:.3f} K")

    # Compare empirical distributions
    fig, ax = plt.subplots(figsize=(8, 4))
    ax.hist(T_berendsen, bins=50, density=True, alpha=0.5, label="Berendsen", color="red")
    ax.hist(T_vrescale,  bins=50, density=True, alpha=0.5, label="V-rescale", color="blue")

    # Overlay expected Gaussian (valid for large n_dof)
    from scipy.stats import norm
    T_range2 = np.linspace(T_target - 4, T_target + 4, 200)
    ax.plot(T_range2, norm.pdf(T_range2, T_target, sigma_expected),
            'k-', lw=2, label=f"Expected (σ={sigma_expected:.3f} K)")
    ax.set_xlabel("Temperature (K)")
    ax.set_ylabel("Probability density")
    ax.legend()
    plt.tight_layout()
    print("Note: Berendsen distribution is too narrow (suppresses fluctuations)")
```

## Barostats: Pressure Coupling

Pressure control adds an additional degree of freedom — the box volume — allowing the simulation box to expand or contract to maintain the target pressure (1 bar for most biological simulations).

### Berendsen Barostat

Scales box dimensions and coordinates by a factor $\mu$:

$$\mu = \left[1 - \frac{\kappa \Delta t}{\tau_P}(P_0 - P)\right]^{1/3}$$

where $\kappa$ is the isothermal compressibility (~$4.5 \times 10^{-5}$ bar$^{-1}$ for water) and $\tau_P$ is the coupling time.

**Same problem as Berendsen thermostat**: does not generate the correct NPT ensemble. Use only for equilibration.

### Parrinello-Rahman Barostat

Extended Lagrangian method for the box matrix $\mathbf{h}$. The box is treated as a dynamic variable with its own equations of motion:

$$\dot{\mathbf{p}}_i = \mathbf{F}_i - m_i \mathbf{G}^{-1}\dot{\mathbf{G}}\mathbf{p}_i - \mathbf{M}^{-1}\mathbf{\Pi}\mathbf{h}^T\mathbf{v}_i$$

where $\mathbf{G} = \mathbf{h}^T\mathbf{h}$ and $\mathbf{M}$ is the "mass" of the box. This generates the correct NPT ensemble with proper volume fluctuations. **Use Parrinello-Rahman for all production NPT simulations in GROMACS.**

```bash
; Production NPT: correct thermodynamics
tcoupl        = V-rescale         ; or Nose-Hoover
pcoupl        = Parrinello-Rahman
pcoupltype    = isotropic         ; for globular proteins
tau_p         = 2.0               ; longer than Berendsen; more stable
ref_p         = 1.0               ; 1 bar
compressibility = 4.5e-5          ; water compressibility

; For membrane simulations: semi-isotropic pressure coupling
; pcoupltype    = semiisotropic
; ref_p         = 1.0 1.0         ; xy and z independently
; compressibility = 4.5e-5 4.5e-5
```

### Monte Carlo Barostat (OpenMM)

OpenMM implements a Monte Carlo barostat that periodically attempts random volume changes and accepts/rejects based on Metropolis criterion. Thermodynamically exact and simpler to implement than Parrinello-Rahman. OpenMM default for NPT.

```python
from openmm import app, unit
from openmm import LangevinMiddleIntegrator, MonteCarloBarostat
import openmm as mm

def create_npt_system(prmtop_file, inpcrd_file, T_K=310, P_bar=1.0):
    """Create an NPT OpenMM system with correct thermostat and barostat."""
    prmtop = app.AmberPrmtopFile(prmtop_file)
    inpcrd = app.AmberInpcrdFile(inpcrd_file)

    system = prmtop.createSystem(
        nonbondedMethod=app.PME,
        nonbondedCutoff=1.2*unit.nanometer,
        constraints=app.HBonds,
        hydrogenMass=4*unit.amu  # H-mass repartitioning for 4 fs step
    )

    # Langevin integrator: stochastic thermostat built in
    integrator = LangevinMiddleIntegrator(
        T_K * unit.kelvin,          # temperature
        1.0 / unit.picosecond,      # friction coefficient
        0.004 * unit.picoseconds    # 4 fs time step (with HMR)
    )

    # Monte Carlo barostat for NPT
    barostat = MonteCarloBarostat(P_bar * unit.bar, T_K * unit.kelvin)
    system.addForce(barostat)

    platform = mm.Platform.getPlatformByName("CUDA")
    simulation = app.Simulation(prmtop.topology, system, integrator, platform)
    simulation.context.setPositions(inpcrd.positions)
    if inpcrd.boxVectors is not None:
        simulation.context.setPeriodicBoxVectors(*inpcrd.boxVectors)

    print(f"System: {system.getNumParticles()} atoms")
    print(f"NPT conditions: {T_K} K, {P_bar} bar")
    return simulation
```

## Why This Matters

Using the Berendsen thermostat for production MD produces configurations that do not follow the Boltzmann distribution. Free energy calculations, protein stability analyses, and entropy estimates derived from such trajectories are systematically wrong. The errors may be small (a few kJ/mol in binding free energies) or large (incorrect ordering of conformational states), and they are invisible from the trajectory itself — a protein simulation looks the same regardless of which thermostat is used. The correct choice (V-rescale or Nosé-Hoover for thermostat; Parrinello-Rahman or MC barostat for pressure) is well-established in the community and costs nothing in computational time.
