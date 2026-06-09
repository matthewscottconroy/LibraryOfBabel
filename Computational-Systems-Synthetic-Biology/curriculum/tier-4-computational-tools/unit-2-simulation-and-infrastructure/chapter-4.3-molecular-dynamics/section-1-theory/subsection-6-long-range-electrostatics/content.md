# Long-Range Electrostatics and Particle Mesh Ewald

The most charged objects in the cell are nucleic acids — DNA carries two negative charges per base pair, and the interior of a ribosome resembles a dense forest of negative charge. Even a modest protein like lysozyme has a net charge of +8 at physiological pH. These charges interact across long distances: Coulomb's law falls off only as $1/r$, compared to the $1/r^6$ decay of van der Waals forces. This slow decay means that a simple cutoff — adequate for LJ interactions beyond 1.2 nm — is catastrophically wrong for electrostatics: truncation introduces large artifacts in the structure and thermodynamics of ionic and polar systems. Proper treatment of long-range electrostatics is one of the most important technical challenges in MD simulation.

## The Problem with Truncating Electrostatics

For a cutoff at radius $r_c$, the fraction of the Coulomb interaction that is discarded is:

$$\text{fraction discarded} = \int_{r_c}^{\infty} \frac{4\pi r^2 \cdot g(r)}{r} \, dr$$

For a uniform medium (ignoring $g(r)$ structure), this integral diverges — no finite cutoff captures the full electrostatic energy. In practice, naively truncated electrostatics produces:
- Artificial structuring of water
- Incorrect ion distributions
- Errors in electrostatic solvation energies of ~10–100 kJ/mol
- Wrong conformational preferences for charged proteins

## The Ewald Sum

The **Ewald method** (1921) solves long-range electrostatics for periodic systems by splitting the Coulomb interaction into two rapidly converging parts:

$$V_\text{elec} = V_\text{real} + V_\text{recip} + V_\text{self}$$

**Real space sum** (short range): The original charges are screened by a Gaussian charge distribution of opposite sign. The screened interaction decays rapidly and is summed in real space with a cutoff $r_c$:

$$V_\text{real} = \frac{1}{2}\sum_\mathbf{n}\sum_{i,j} \frac{q_i q_j}{4\pi\varepsilon_0} \frac{\text{erfc}(\alpha r_{ij,\mathbf{n}})}{r_{ij,\mathbf{n}}}$$

where $\alpha$ is the Ewald splitting parameter and $\text{erfc}$ is the complementary error function.

**Reciprocal space sum** (long range): The compensating Gaussian charge distributions are summed in Fourier (reciprocal) space:

$$V_\text{recip} = \frac{1}{2\varepsilon_0 V}\sum_{\mathbf{k}\neq\mathbf{0}} \frac{1}{k^2} e^{-k^2/4\alpha^2} \left|\sum_j q_j e^{i\mathbf{k}\cdot\mathbf{r}_j}\right|^2$$

**Self-energy correction** (removes the self-interaction of each Gaussian with itself):

$$V_\text{self} = -\frac{\alpha}{\sqrt{\pi}\varepsilon_0}\sum_i q_i^2$$

The Ewald sum is **exact** for periodic systems. Its computational cost scales as $O(N^{3/2})$ — better than naively computing all pairs ($O(N^2)$) but still expensive for large systems.

## Particle Mesh Ewald (PME)

**PME** (Darden, York, Pedersen, 1993) reduces the cost of the reciprocal-space sum from $O(N^{3/2})$ to $O(N \log N)$ by interpolating charges onto a regular grid and using the Fast Fourier Transform (FFT).

The algorithm:

1. **Spread**: interpolate atomic charges $q_j$ onto a uniform 3D grid using B-spline basis functions (PME order 4–6)
2. **FFT**: compute the 3D FFT of the charge density $\hat{\rho}(\mathbf{k})$
3. **Green's function**: multiply by $\hat{G}(\mathbf{k}) = e^{-k^2/4\alpha^2}/k^2\varepsilon_0$ in reciprocal space
4. **Inverse FFT**: transform back to real space to obtain electrostatic potential
5. **Gather**: interpolate potential back to atomic positions to compute forces

```python
import numpy as np

def ewald_direct(charges, positions, box_length, alpha=0.35, cutoff=1.2):
    """
    Real-space (direct) part of Ewald sum.
    charges: (N,) partial charges in units of elementary charge
    positions: (N, 3) in nm
    alpha: Ewald parameter (1/nm); controls split between real and reciprocal
    cutoff: real-space cutoff in nm
    Returns: energy in kJ/mol (using SI-derived AMBER/GROMACS units)
    """
    from scipy.special import erfc
    COULOMB = 138.935  # kJ/mol/nm/e^2 (= 1/(4*pi*eps0) in GROMACS units)

    N = len(charges)
    energy = 0.0
    for i in range(N - 1):
        for j in range(i + 1, N):
            rij = positions[i] - positions[j]
            rij -= box_length * np.round(rij / box_length)
            r = np.linalg.norm(rij)
            if r < cutoff:
                energy += COULOMB * charges[i] * charges[j] * erfc(alpha * r) / r
    return energy

def ewald_self_correction(charges, alpha):
    """Self-energy correction (subtract self-interaction of screening charge)."""
    COULOMB = 138.935
    return -COULOMB * alpha / np.sqrt(np.pi) * np.sum(charges**2)

# Note: for real simulations, use GROMACS/AMBER/OpenMM PME — this is illustrative
```

## Configuring PME in GROMACS

```bash
# GROMACS mdp settings for PME
cutoff-scheme    = Verlet    ; neighbor list algorithm
coulombtype      = PME       ; particle mesh Ewald
rcoulomb         = 1.2       ; real-space cutoff (nm)
fourierspacing   = 0.12      ; grid spacing for FFT (nm); ~1/4 of rcoulomb
pme-order        = 4         ; cubic spline interpolation (4 = standard)
ewald-rtol       = 1e-5      ; direct/reciprocal error tolerance

; Van der Waals
vdwtype          = Cut-off
rvdw             = 1.2       ; must be ≤ rcoulomb for Verlet scheme

; IMPORTANT: rcoulomb and rvdw must match (Verlet scheme requirement)
```

## Ewald Parameters: Balancing Accuracy and Speed

The three key parameters controlling PME accuracy and cost:

| Parameter | Typical value | Effect of increasing |
|---|---|---|
| $r_c$ (real-space cutoff) | 1.0–1.2 nm | More accurate direct sum; slower |
| Grid spacing (1/fourierspacing) | ~8–10 points/nm | More accurate reciprocal sum; slower FFT |
| PME order | 4 (cubic) | Higher-order interpolation; more accurate; slower |
| $\alpha$ (Ewald splitting) | Optimized automatically | Higher $\alpha$ → smaller real sum, larger reciprocal |

GROMACS automatically optimizes $\alpha$ to balance real-space and reciprocal-space timing. Manual tuning is rarely needed.

## GPU Acceleration of PME

Modern GPU implementations (GROMACS, AMBER, OpenMM) offload both the real-space force computation and the PME FFT to GPU:

```bash
# GROMACS with GPU-accelerated PME
gmx mdrun -ntmpi 1 -ntomp 4 -gpu_id 0 \
          -pme gpu \     # run PME on GPU
          -bonded gpu \  # bonded forces on GPU
          -nb gpu        # non-bonded on GPU
```

GPU PME achieves 10–20× speedup over CPU for systems with >10,000 atoms, making microsecond-scale simulations of 100,000-atom systems routine on a single GPU.

## Alternatives: Reaction Field and Force Switching

For systems where PME is unavailable or for coarse-grained simulations:
- **Reaction field**: treat the system beyond the cutoff as a continuum dielectric. Better than hard cutoff; acceptable for CG models but not all-atom proteins.
- **Force-switch/potential-switch**: smoothly taper forces to zero at the cutoff. Reduces discontinuity artifacts but does not solve the fundamental long-range problem.

## Why This Matters

PME is not a technical detail — it is the foundation of modern biomolecular MD. Simulations of ion channels, charged drug molecules, nucleic acids, and membrane proteins would be qualitatively wrong without proper long-range electrostatics. The $O(N \log N)$ scaling of PME is what makes routine simulations of 100,000+ atom systems feasible; the alternative (naively computing all $N^2$ pairs) would be 1000× slower. Understanding PME is essential for correctly configuring simulations, interpreting energy components, and diagnosing problems when simulations fail.
