# Cathedral VI: A Spatial Stochastic Simulation of a Cellular Process

---

## The Question

When does spatial organization matter for biological dynamics — and how do spatial and stochastic effects together produce behavior that neither framework predicts alone?

---

## Prerequisites

- [Tier 0.1](../curriculum/tier-0-bedrock/0.1-mathematics.md): PDEs, stability analysis, Turing conditions
- [Tier 2.1](../curriculum/tier-2-systems-biology/2.1-mathematical-modeling.md): Reaction-diffusion PDEs, stochastic simulation
- [Tier 4.1](../curriculum/tier-4-computational-tools/4.1-scientific-computing.md): PDE numerics, performance

---

## Candidate Systems

Choose one biological system where spatial organization is central:

**A. Min Protein Oscillations in *E. coli***
- MinD and MinE oscillate pole-to-pole; restrict FtsZ ring to midcell
- Well-characterized: known kinetics, diffusion coefficients
- Clear readout: oscillation period, wavelength, spatial pattern
- Benchmark against: Howard & Bhattacharyya 2009 (PDE model); Huang et al. 2003

**B. Turing Pattern Formation**
- Activator-inhibitor with differential diffusion → spatial patterns
- Canonical: Gierer-Meinhardt model; or more realistic skin pigmentation model
- Questions: how parameter changes switch between spots, stripes, labyrinths?
- Benchmark against: published parameter surveys of pattern formation

**C. Receptor Clustering and Signaling**
- Ligand-receptor interactions + lateral diffusion in membrane
- Clustering emerges from cytoplasmic scaffold proteins
- Question: when does clustering amplify or suppress signaling?
- Well-studied in: T cell receptor signaling, EGFR signaling

**D. Morphogen Gradient Interpretation**
- Bicoid gradient in Drosophila embryo: source → diffusion → degradation → graded concentration
- How do cells read concentration threshold reliably despite diffusion noise?
- Question: what is the precision of positional information from a noisy gradient?

---

## The Project

### Phase 1: Literature and Model Survey

1. Read the primary literature for your chosen system (3-5 key papers)
2. Identify the existing mathematical model:
   - What ODEs or PDEs are published?
   - What parameters are measured? Which are estimated?
3. Implement the published model first:
   - Reproduce at least one figure from a published paper
   - This verifies your implementation before you extend it

### Phase 2: Deterministic PDE Model

4. Implement the reaction-diffusion PDE system using method of lines:
   ```python
   import numpy as np
   from scipy.integrate import solve_ivp
   
   def min_system(t, state, params):
       """1D Min protein oscillation PDE via method of lines"""
       n_grid = len(state) // 4  # four species: MinD_c, MinD_m, MinDE_m, MinE_c
       
       # Unpack state
       MinD_c = state[:n_grid]
       MinD_m = state[n_grid:2*n_grid]
       MinDE_m = state[2*n_grid:3*n_grid]
       MinE_c = state[3*n_grid:]
       
       L, D_MinD, D_MinE, D_MinD_m, D_MinDE_m = params
       dx = L / n_grid
       
       # Laplacian operator (finite difference)
       def laplacian(u):
           lu = np.zeros_like(u)
           lu[1:-1] = (u[2:] - 2*u[1:-1] + u[:-2]) / dx**2
           lu[0] = (u[1] - u[0]) / dx**2      # Neumann BC
           lu[-1] = (u[-2] - u[-1]) / dx**2
           return lu
       
       # Reaction terms (Huang, Meier et al. 2003)
       ...  # add reaction kinetics here
       
       # Diffusion terms
       dMinD_c = D_MinD * laplacian(MinD_c) + reaction_MinD_c
       ...
       
       return np.concatenate([dMinD_c, dMinD_m, dMinDE_m, dMinE_c])
   ```

5. Explore parameter space:
   - Reproduce oscillation period vs. cell length
   - Map phase diagram: oscillation vs. no oscillation
   - Vary diffusion ratio: what minimum D_inhibitor/D_activator gives patterns?

### Phase 3: Stochastic PDE or Particle-Based Simulation

6. Implement stochastic version to ask: when does discreteness matter?

   **Option A: Tau-leaping on spatial grid**
   - Partition space into voxels (3D lattice)
   - Each voxel is a well-stirred compartment
   - Reactions: apply tau-leaping within each voxel
   - Diffusion: treat as first-order reactions between adjacent voxels
   - Diffusion rate: k_diff = D / dx²

   **Option B: Smoldyn**
   - Particle-based; each molecule is an individual particle
   - Diffusion: Brownian motion (exact)
   - Reactions: probability-based; computed from reaction radius and rate

   ```python
   # Tau-leaping on spatial grid
   def diffuse_voxels(n_molecules, D, dx, dt, n_voxels):
       """Simple diffusion via voxel-to-voxel hopping"""
       k_hop = D / dx**2
       n_hop = np.random.poisson(k_hop * n_molecules * dt)  # tau-leaping
       # Distribute hops to left/right neighbors (50/50)
       hop_left = np.random.binomial(n_hop, 0.5)
       hop_right = n_hop - hop_left
       
       new_n = n_molecules.copy()
       new_n[1:] += hop_left[:-1]
       new_n[:-1] += hop_right[1:]
       new_n[:-1] -= hop_left[:-1]
       new_n[1:] -= hop_right[1:]
       return new_n
   ```

7. Compare stochastic to deterministic:
   - Run N realizations of stochastic simulation
   - Compute mean and variance of key observables over time
   - Does mean match deterministic prediction?
   - What is the variance? Does it depend on molecule number?

### Phase 4: Systematic Comparison

8. Low copy number regime:
   - Reduce total molecule number by 10×, 100×
   - Does pattern still form? Does it become noisier?
   - What is the minimum molecule number for reliable pattern formation?

9. Cell size effects:
   - Vary spatial domain size (cell length)
   - How does pattern wavelength scale?
   - Does stochastic system show the same scaling?

10. Noise-induced transitions:
    - Are there parameter regimes where deterministic model predicts steady state but stochastic model shows oscillation (or vice versa)?
    - This is a genuine research question — not all systems have been analyzed this way

### Phase 5: Novel Result

11. Ask a question the published models have not answered:
    - In the Min system: how does cell geometry (shape, poles) affect oscillation regularity?
    - In Turing patterns: what is the effect of domain growth (organism growing) on pattern formation?
    - In morphogen gradients: can mutual repressor gradients improve positional accuracy?

12. Implement your novel variation; simulate; analyze

13. Interpret: does the novel result match biological observations? Does it generate predictions?

---

## Expected Output

- Clean, modular simulation code in Python or Julia
- Reproduction of one key result from published model (validation)
- Systematic comparison of PDE vs. stochastic behavior
- At least one novel analysis not in the published literature
- Figures: space-time plots, phase diagrams, stochastic variability analysis

---

## Key Tools

- SciPy solve_ivp: stiff ODE solver for method of lines
- DifferentialEquations.jl: faster, Julia-native
- Smoldyn: particle-based spatial stochastic simulation
- VCell: GUI-based spatial ODE/stochastic simulation
- Matplotlib + imageio: space-time animations
