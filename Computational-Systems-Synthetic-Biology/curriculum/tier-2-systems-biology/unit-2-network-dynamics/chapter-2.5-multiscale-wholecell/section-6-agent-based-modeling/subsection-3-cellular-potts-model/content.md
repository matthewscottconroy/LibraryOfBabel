# The Cellular Potts Model

## Physical Intuition

Watch a soap bubble floating in the air. It is perfectly spherical — not because it is programmed to be spherical, but because the physics of surface tension drives any enclosed fluid toward the shape that minimizes surface area for a given volume. Now watch two soap bubbles merge: they immediately reorganize into a shape that minimizes the total surface energy, sharing a flat wall where their surfaces meet. No molecular machinery is directing this behavior. It emerges entirely from physics.

The **Cellular Potts Model (CPM)**, also called the Glazier-Graner-Hogeweg (GGH) model, describes cell shape and movement using exactly this kind of reasoning — the physics of surface minimization applied to living cells. The core idea is that cells behave like liquid droplets: they minimize their surface energy while maintaining a target volume, subject to random fluctuations.

Unlike particle-based ABMs where each cell is a point or sphere, CPM represents cells as extended objects on a lattice — collections of lattice sites that belong to the same cell. Cell boundaries are shared between adjacent lattice sites of different cells, and the energy of these boundaries depends on cell type. Cell movement corresponds to lattice sites at cell boundaries being "captured" by neighboring cells — a process driven by energy minimization with thermal fluctuations.

## The Hamiltonian

The CPM Hamiltonian (energy function) sums contributions from all interactions:

$$H = \underbrace{\sum_{\langle i,j \rangle} J(\sigma_i, \sigma_j)(1 - \delta_{\sigma_i, \sigma_j})}_{\text{adhesion}} + \underbrace{\sum_\sigma \lambda_V (\text{vol}(\sigma) - V_T(\sigma))^2}_{\text{volume}} + \underbrace{\sum_\sigma \lambda_S (A(\sigma) - A_T(\sigma))^2}_{\text{surface area}}$$

where:
- $\sigma_i$: cell identity at lattice site $i$ (0 for extracellular medium)
- $J(\sigma_i, \sigma_j)$: **adhesion energy** between cell types $\sigma_i$ and $\sigma_j$ at a boundary — higher $J$ means less adhesion (energetically costly to have these cell types adjacent)
- $\delta_{\sigma_i, \sigma_j} = 1$ if $\sigma_i = \sigma_j$ (same cell, no boundary): so $(1-\delta)$ counts only boundaries
- $\text{vol}(\sigma)$: actual volume of cell $\sigma$ (number of lattice sites)
- $V_T(\sigma)$: target volume for cell $\sigma$
- $\lambda_V$: volume elasticity constant (penalty for deviating from target volume)
- $A(\sigma)$, $A_T(\sigma)$: actual and target surface area
- $\lambda_S$: surface area elasticity constant

Additional terms can be added: chemotaxis (bias toward chemical gradients), haptotaxis (bias toward substrate adhesion gradients), and cytoskeletal forces.

## The Metropolis Update Algorithm

The CPM evolves via a modified Metropolis algorithm (Monte Carlo):

1. **Choose a random lattice site** $i$ and a random neighbor $j$
2. **Propose a copy move**: site $i$ copies the identity of site $j$ ($\sigma_i \to \sigma_j$)
3. **Compute energy change**: $\Delta H = H_{\text{after}} - H_{\text{before}}$
4. **Accept or reject**:
   - If $\Delta H \leq 0$: accept unconditionally (energy decreases → favorable)
   - If $\Delta H > 0$: accept with probability $P = e^{-\Delta H / T}$ where $T$ is the "temperature" (biological fluctuation amplitude)

The temperature parameter $T$ controls the level of random fluctuations:
- $T = 0$: purely deterministic energy minimization (cells find local energy minimum)
- Large $T$: highly random dynamics (cells ignore energy landscape)
- Biological $T$: intermediate, producing realistic cell shape fluctuations

```python
import numpy as np
from numpy.random import default_rng

class CPMSimulation:
    def __init__(self, width, height, temperature=10):
        self.width = width
        self.height = height
        self.T = temperature
        self.rng = default_rng(42)
        # Initialize grid: 0 = medium, positive integers = cell IDs
        self.grid = np.zeros((height, width), dtype=int)
        self.cells = {}  # cell_id -> {'type': str, 'target_vol': int, ...}
    
    def add_cell(self, cell_id, center, radius, cell_type, target_vol=25):
        """Place a circular cell on the grid."""
        y, x = center
        for dy in range(-radius, radius+1):
            for dx in range(-radius, radius+1):
                if dx**2 + dy**2 <= radius**2:
                    ny, nx = (y+dy) % self.height, (x+dx) % self.width
                    self.grid[ny, nx] = cell_id
        self.cells[cell_id] = {
            'type': cell_type, 'target_vol': target_vol,
            'lambda_V': 2.0
        }
    
    def compute_adhesion_energy(self, type1, type2):
        """Contact energy between cell types."""
        adhesion_table = {
            ('cell', 'cell'): 15,
            ('cell', 'medium'): 30,
            ('medium', 'medium'): 0,
        }
        key = (min(type1, type2), max(type1, type2))
        return adhesion_table.get(key, 25)
    
    def delta_H(self, y, x, new_sigma):
        """Compute energy change for copy attempt."""
        old_sigma = self.grid[y, x]
        if old_sigma == new_sigma:
            return 0
        
        # Adhesion energy change
        neighbors = [(y-1,x), (y+1,x), (y,x-1), (y,x+1)]
        dH_adhesion = 0
        for ny, nx in neighbors:
            ny, nx = ny % self.height, nx % self.width
            neighbor_sigma = self.grid[ny, nx]
            if neighbor_sigma != old_sigma:
                old_type = self.cells.get(old_sigma, {'type': 'medium'})['type']
                new_type = self.cells.get(new_sigma, {'type': 'medium'})['type']
                nb_type = self.cells.get(neighbor_sigma, {'type': 'medium'})['type']
                dH_adhesion += (self.compute_adhesion_energy(new_type, nb_type) -
                                self.compute_adhesion_energy(old_type, nb_type))
        
        # Volume constraint energy change
        dH_vol = 0
        if old_sigma != 0:  # losing a site from old cell
            cell = self.cells[old_sigma]
            vol = np.sum(self.grid == old_sigma)
            dH_vol += cell['lambda_V'] * ((vol-1 - cell['target_vol'])**2 
                                           - (vol - cell['target_vol'])**2)
        if new_sigma != 0:  # gaining a site for new cell
            cell = self.cells[new_sigma]
            vol = np.sum(self.grid == new_sigma)
            dH_vol += cell['lambda_V'] * ((vol+1 - cell['target_vol'])**2 
                                           - (vol - cell['target_vol'])**2)
        
        return dH_adhesion + dH_vol
    
    def mcs_step(self):
        """One Monte Carlo Step: N random copy attempts where N = grid size."""
        N = self.width * self.height
        for _ in range(N):
            # Random site
            y = self.rng.integers(0, self.height)
            x = self.rng.integers(0, self.width)
            # Random neighbor
            dy, dx = self.rng.choice([(0,1),(0,-1),(1,0),(-1,0)])
            ny, nx = (y+dy) % self.height, (x+dx) % self.width
            new_sigma = self.grid[ny, nx]
            
            dH = self.delta_H(y, x, new_sigma)
            if dH <= 0 or self.rng.random() < np.exp(-dH / self.T):
                self.grid[y, x] = new_sigma
```

## Biological Applications

### Cell Sorting

The **differential adhesion hypothesis** (Steinberg 1963) predicts that heterotypic mixtures of cells will spontaneously sort into organized configurations that minimize total adhesion energy. CPM directly tests this: cells with high homotypic adhesion (low $J$ between same-type cells) segregate from cells with lower homotypic adhesion.

Quantitative prediction: the sorting hierarchy (which cell type forms the core vs. periphery) follows from the ordering of adhesion energies. This has been validated for many embryonic cell types.

### Chemotaxis

Add a chemotaxis term to the Hamiltonian:

$$H_{\text{chemo}} = -\chi \sum_{\sigma} \nabla C \cdot \hat{n}_\sigma$$

where $\chi$ is the chemotaxis strength and $\nabla C$ is the chemical gradient evaluated at the cell front. Copy attempts toward higher concentration are energetically favored. This produces realistic cell migration patterns that match microfluidic chemotaxis assays.

### Tumor Invasion

CompuCell3D models of tumor spheroids include:
- Cancer cells (high proliferation, altered adhesion)
- Normal cells (lower adhesion)
- Extracellular matrix (represented as a separate field)
- Hypoxic core → necrosis (cells exceeding distance from vasculature die)
- Angiogenesis (VEGF-driven vessel growth toward the tumor)

These models predict invasion depth, spatial distribution of proliferative vs. quiescent vs. necrotic tumor zones, and the effect of E-cadherin loss (adhesion reduction → invasion enhancement).

## Limitations and Extensions

**Computational cost**: CPM scales as $O(N_{\text{sites}} \times N_{\text{MCS}})$; for large simulations (1000 cells, $100 \times 100$ grid), each MCS requires $\sim 10^4$ evaluations. Modern CPM simulations run for thousands of MCS in minutes on a workstation.

**No explicit cytoskeleton**: CPM captures the macroscopic effect of cellular forces but does not model the molecular machinery (actin, myosin) responsible. For questions about specific cytoskeletal mechanisms, subcellular models (Cytosim, VCell) are needed.

**Extensions**: Actomyosin contractility has been added to CPM as tension terms; nuclear deformation during 3D cell migration; subcellular element models (a CPM variant with internal structural detail).

## Why This Matters

The CPM is the dominant framework for modeling **multicellular morphogenesis** because it correctly captures the physics of cell-cell adhesion and cell shape changes without requiring explicit tracking of molecular motors or cytoskeletal dynamics. It is the bridge between molecular-scale signaling (which regulates adhesion molecules and cytoskeletal tension) and tissue-scale organization (which is the emergent result of those molecular signals summed over millions of cells). Understanding the CPM enables interpretation and design of experiments in developmental biology, cancer biology (invasion and metastasis modeling), and tissue engineering (predicting how engineered cell assemblies will self-organize into functional structures).
