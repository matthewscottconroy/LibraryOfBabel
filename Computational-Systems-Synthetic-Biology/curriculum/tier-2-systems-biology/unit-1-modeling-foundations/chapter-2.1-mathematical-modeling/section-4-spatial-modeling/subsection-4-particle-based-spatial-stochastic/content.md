# Particle-Based Spatial Stochastic Simulation

## When Both Space and Stochasticity Matter

Consider the synapse. A synaptic vesicle releases roughly 2,000–5,000 glutamate molecules into a synaptic cleft that is 20 nanometers wide and a few hundred nanometers across. Those glutamate molecules must diffuse across the cleft and bind AMPA and NMDA receptors within milliseconds — otherwise the postsynaptic response fails to trigger. The spatial distribution of receptors, the exact landing position of the vesicle, and the stochastic binding kinetics all matter quantitatively. An ODE model would treat glutamate as uniformly distributed; a spatial PDE would treat binding probabilistically but molecules as continuous fluid; only a model that tracks each glutamate molecule individually can capture the full physics.

This is the regime of **particle-based spatial stochastic** (PBSS) simulation: systems where both spatial structure and molecular discreteness are essential, and where the number of molecules is small enough that individual encounters matter.

Reaction-diffusion PDEs treat molecular concentrations as smooth continuum fields — appropriate when molecule numbers are large ($\gg 100$ per compartment). The spatial Gillespie algorithm (voxel-based, also called RDME — Reaction-Diffusion Master Equation) treats space as a grid of well-mixed compartments and molecules as integers — appropriate for intermediate copy numbers. PBSS treats each molecule as an individual particle with explicit position $\mathbf{r}(t)$. Reactions occur probabilistically when particles come within a reaction radius.

## The Smoluchowski Framework

The theoretical basis for PBSS is the **Smoluchowski equation**: the probability density of a diffusing particle obeys:

$$\frac{\partial p(\mathbf{r}, t)}{\partial t} = D \nabla^2 p(\mathbf{r}, t)$$

with an absorbing boundary condition at a reaction radius $\sigma$ (the "encounter radius"). When two particles of types A and B diffuse and their center-to-center distance decreases to $\sigma$, a reaction occurs with probability determined by the intrinsic reactivity.

The **Smoluchowski rate** for diffusion-limited reactions between A and B is:

$$k_D = 4\pi D_{AB} \sigma$$

where $D_{AB} = D_A + D_B$ is the relative diffusion coefficient. Reactions slower than this limit include a finite-time delay after encounter (Collins-Kimball model).

This rate has a beautiful physical interpretation: $4\pi \sigma^2$ is the surface area of the encounter sphere, and $D_{AB}$ is the relative diffusion coefficient — so $k_D$ is essentially the rate at which molecules sweep through the volume of the encounter sphere per unit time. Diffusion-limited reactions are limited not by chemistry but by how fast the molecules find each other.

## Smoldyn: The Standard PBSS Tool

**Smoldyn** (Andrews & Bray, 2004) is the most widely used PBSS software in cell biology. It simulates Brownian diffusion of molecules in 1D, 2D, or 3D geometries (boxes, spheres, cell shapes defined by polygon meshes) with bimolecular reactions occurring when molecules overlap.

Core algorithm for each time step $\Delta t$:
1. Each molecule $i$ diffuses: $\mathbf{r}_i(t + \Delta t) = \mathbf{r}_i(t) + \boldsymbol{\xi}$ where $\boldsymbol{\xi} \sim \mathcal{N}(\mathbf{0}, 2D\Delta t \mathbf{I})$
2. Unimolecular reactions fire with probability $k_\text{uni} \Delta t$
3. Bimolecular reactions: pairs of reactant molecules within binding radius $\sigma_b$ react with probability $k_\text{bi} \Delta t / (4\pi D_{AB} \sigma_b^2)$
4. Boundary conditions applied (reflection, absorption, membrane reactions)

The binding radius $\sigma_b$ is chosen to reproduce the correct macroscopic rate constant $k$ given $D_{AB}$.

```python
# Smoldyn is typically called via its Python API or configuration files
# Below is a representative configuration structure (smoldyn .txt format)
smoldyn_config = """
# Smoldyn config for receptor-ligand binding on a 2D membrane
dim 3
boundaries 0 -1 1; boundaries 1 -1 1; boundaries 2 0 0.1

species receptor ligand complex

difc receptor 0.1      # um^2/s, membrane diffusion
difc ligand 10.0       # free diffusion in cytoplasm

reaction binding  receptor + ligand -> complex  kon
reaction unbinding complex -> receptor + ligand  koff

kon 0.01               # um^3/s (3D bimolecular rate)
koff 0.1               # per second

# Place receptors on membrane surface
surface_action all(receptor) reflect all
mol 100 receptor u     # random placement
mol 1000 ligand u      # random placement in volume

time_start 0
time_stop 100
time_step 0.001
output_files stdout
cmd n 100 molcount stdout
"""
```

## MCell: Synaptic Physiology at the Nanoscale

**MCell** was developed specifically for modeling neurotransmitter diffusion and receptor activation in synaptic clefts. A synaptic cleft is ~20 nm wide and contains a few thousand glutamate molecules per vesicle release — clearly a regime where both spatial structure and stochasticity matter.

MCell uses Monte Carlo sampling to:
- Propagate each neurotransmitter molecule by Brownian diffusion
- Calculate receptor binding probabilistically based on local concentration
- Model receptor state transitions (closed → open → desensitized)
- Handle complex 3D geometries from electron microscopy reconstructions

Applications include: computing the time course of AMPA and NMDA receptor activation, understanding how release probability and vesicle placement affect synaptic strength, and predicting the effects of uptake transporter density on spillover. The spatial heterogeneity of the synapse — receptors clustered at specific locations, transporters at others — matters quantitatively for the time course and amplitude of the postsynaptic response. Only PBSS captures this correctly.

## ReaDDy: Reaction-Diffusion Dynamics in Python

**ReaDDy** (Reaction-Diffusion Dynamics) provides a Python-accessible framework for PBSS with:
- Brownian dynamics with inter-particle potentials (excluded volume, attraction)
- Reaction networks with arbitrary order
- GPU acceleration for large systems

It is particularly suited for modeling protein complex assembly, actin polymerization, and receptor clustering — processes where molecule size and steric effects matter.

```python
import readdy

system = readdy.ReactionDiffusionSystem(box_size=[10., 10., 10.])
system.add_species("A", diffusion_constant=1.0)
system.add_species("B", diffusion_constant=1.0)
system.add_species("C", diffusion_constant=0.5)

# Bimolecular reaction A + B -> C
system.reactions.add("fusion: A +(1.0) B -> C", rate=1.0)
# Unimolecular reaction C -> A + B
system.reactions.add("fission: C -> A +(1.0) B", rate=0.1)

sim = system.simulation(kernel="CPU")
sim.add_particles("A", positions=np.random.uniform(-5, 5, (100, 3)))
sim.add_particles("B", positions=np.random.uniform(-5, 5, (100, 3)))

sim.run(n_steps=10000, timestep=1e-3)
```

## Computational Cost

PBSS is the most expensive spatial modeling approach. Computational cost scales as $O(N^2)$ per step for naive pairwise distance checks (improved to $O(N)$ with spatial hashing). Typical systems:
- Bacterial cell ($\sim 10^4$ molecules): feasible, seconds to minutes
- Mammalian cell ($\sim 10^8$ molecules): infeasible without coarse-graining

For large systems, a common strategy is **hybrid simulation**: PBSS for the spatially critical subvolume (e.g., the synapse or the plasma membrane) coupled to RDME or ODE models for the bulk cytoplasm. This allows mechanistic detail where it matters most while keeping computation tractable.

## Why This Matters

Particle-based spatial stochastic simulation is the most mechanistically complete framework for modeling intracellular dynamics. It correctly captures crowding effects (molecules cannot overlap), diffusion-limited reaction kinetics, spatial correlations between reactants, and stochastic fluctuations at low copy numbers. These effects are non-negligible at the nanoscale: receptor clustering alters signaling sensitivity, molecular crowding slows enzyme kinetics, and spatial correlations between kinase and substrate molecules can change apparent rate constants by orders of magnitude.

As super-resolution microscopy resolves molecular positions in living cells at nanometer resolution, PBSS becomes the natural modeling counterpart — a tool for testing whether proposed molecular mechanisms are quantitatively consistent with observed spatial distributions. When a PBSS model of receptor clustering predicts the same spatial statistics as super-resolution PALM/STORM data, you have a much stronger test of the mechanism than any bulk measurement could provide. The computational cost is high, but for questions that require this level of spatial and stochastic detail, it is the only tool that will give the right answer.
