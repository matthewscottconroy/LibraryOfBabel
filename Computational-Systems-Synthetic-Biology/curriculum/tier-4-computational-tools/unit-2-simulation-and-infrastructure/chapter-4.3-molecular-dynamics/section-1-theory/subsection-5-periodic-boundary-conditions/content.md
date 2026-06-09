# Periodic Boundary Conditions

A protein in your cell lives in a vast ocean of water — something like 10,000 water molecules for every protein molecule in a typical cytoplasm. Simulating even a small fraction of that ocean is computationally feasible only if we use a finite simulation box. But a finite box creates a problem: atoms at the edge encounter a wall, an artificial vacuum interface, or the back of the box — none of which exist in biology. A protein near the surface of a water droplet would behave very differently from a protein in bulk solution; it would be influenced by surface tension, denaturation at the interface, and the wrong distribution of ions. **Periodic boundary conditions (PBC)** provide an elegant solution: the simulation box is replicated infinitely in all directions, so atoms exiting one face reappear from the opposite face, effectively simulating bulk behavior without artificial boundaries.

## The Minimum Image Convention

With PBC, each atom interacts with every other atom and its periodic images. To avoid double-counting interactions across copies, the **minimum image convention** specifies that each atom interacts only with the nearest periodic image of every other atom.

For a cubic box with side length $L$, the minimum image vector between atoms $i$ and $j$ is:

$$\mathbf{r}_{ij}^* = \mathbf{r}_{ij} - L \cdot \text{round}\!\left(\frac{\mathbf{r}_{ij}}{L}\right)$$

This is valid as long as the cutoff radius $r_c$ for non-bonded interactions satisfies:

$$r_c < \frac{L}{2}$$

If the cutoff exceeds half the box length, an atom could interact with its own periodic image — a catastrophic artifact.

```python
import numpy as np

def minimum_image_vector(r_ij, box_vectors):
    """
    Apply minimum image convention for a general (possibly non-cubic) box.
    r_ij: displacement vector (3,)
    box_vectors: (3, 3) matrix of box lattice vectors
    """
    # Transform to fractional coordinates
    box_inv = np.linalg.inv(box_vectors)
    s = box_inv @ r_ij
    # Wrap to [-0.5, 0.5)
    s -= np.round(s)
    # Transform back to Cartesian
    return box_vectors @ s

def wrap_positions(positions, box_length):
    """Wrap all atoms back into the primary box [0, L)."""
    return positions % box_length

# Example: verify minimum image for a cubic box
L = 6.0  # nm
r_i = np.array([5.8, 2.0, 3.0])  # near the +x face
r_j = np.array([0.3, 2.0, 3.0])  # near the -x face

r_ij_naive = r_i - r_j       # gives 5.5 nm — wrong (ignoring PBC)
r_ij_naive -= L * np.round(r_ij_naive / L)  # minimum image: -0.5 nm
print(f"Naive distance: {np.linalg.norm(r_i - r_j):.2f} nm")
print(f"Min image distance: {np.linalg.norm(r_ij_naive):.2f} nm")
```

## Box Shapes

The choice of box geometry affects how efficiently space is used (minimizing the number of water molecules needed to solvate the protein):

**Cubic box**: equal sides $a = b = c$, all angles 90°. Simplest to implement but least efficient — corners are vacuum for a spherical protein.

**Rhombic dodecahedron**: the Wigner-Seitz cell of the face-centered cubic lattice. It is the optimal shape for enclosing a sphere — it fills space with ~29% fewer water molecules than a cube of the same minimum protein-to-boundary distance. Used for globular proteins.

$$\text{Box vectors (rhombic dodecahedron):} \quad \mathbf{a} = a\hat{x}, \quad \mathbf{b} = \frac{a}{2}\hat{x} + \frac{a\sqrt{2}}{2}\hat{y}, \quad \mathbf{c} = \frac{a}{2}\hat{x} + \frac{a\sqrt{2}}{6}\hat{y} + \frac{a\sqrt{6}}{3}\hat{z}$$

**Hexagonal prism**: appropriate for membrane systems where the $z$-axis is normal to the bilayer. The $xy$-plane uses a hexagonal shape.

```bash
# GROMACS: solvate with rhombic dodecahedron, 1.0 nm protein-edge distance
# Create box around protein
gmx editconf -f protein.pdb -o protein_box.gro \
             -bt dodecahedron \
             -d 1.0         # minimum 1 nm between protein and box edge

# Solvate
gmx solvate -cp protein_box.gro \
            -cs spc216.gro  \  # pre-equilibrated SPC/E water box
            -o solvated.gro \
            -p topol.top       # updates topology with correct water count

# Check: verify box dimensions and number of water molecules
gmx editconf -f solvated.gro -o /dev/null  # prints box info
```

## Minimum Box Size: The 2× Cutoff Rule

The minimum image convention requires that no atom interacts with its own image. This imposes:

$$L_\text{min} = 2 r_c + d_\text{protein}$$

where $r_c$ is the non-bonded cutoff (typically 1.0–1.2 nm) and $d_\text{protein}$ is the largest dimension of the solute. For PME electrostatics with a 1.2 nm cutoff, the minimum protein-to-box-face distance should be at least 1.2–1.5 nm. In practice, 1.0 nm is a common minimum; larger boxes improve accuracy at greater cost.

```python
def check_box_size(protein_coords, box_vectors, cutoff_nm=1.2, margin_nm=0.2):
    """
    Verify that the simulation box is large enough for the given cutoff.
    Prints warnings if the protein-to-edge distance is insufficient.
    """
    # Find the bounding box of the protein
    prot_min = protein_coords.min(axis=0)
    prot_max = protein_coords.max(axis=0)
    prot_size = prot_max - prot_min  # max extent in each dimension

    # For a cubic or orthogonal box, box_vectors is diagonal
    box_lengths = np.diag(box_vectors) if box_vectors.shape == (3,3) else box_vectors

    for dim, (size, L, label) in enumerate(zip(prot_size, box_lengths, "xyz")):
        available_margin = (L - size) / 2  # space on each side
        status = "OK" if available_margin >= cutoff_nm + margin_nm else "WARNING"
        print(f"  {label}-dimension: protein {size:.2f} nm, box {L:.2f} nm, "
              f"margin {available_margin:.2f} nm [{status}]")
        if available_margin < cutoff_nm:
            print(f"    ERROR: margin < cutoff ({cutoff_nm} nm). Enlarge the box!")
```

## Artifacts from PBC

PBC introduces periodic images that can interact artificially with the primary simulation cell. Common artifacts:

1. **Self-interaction**: the protein's electrostatic potential interacts with its own image. For highly charged proteins (nucleic acids, IDPs), this can cause artifacts in the free energy of the system. Finite-size corrections are available for charge-charge interactions.

2. **Correlated motion**: for very small boxes, hydrodynamic fluctuations become correlated across periodic boundaries. Membrane protein diffusion is especially sensitive; larger boxes are needed.

3. **Wrong ensemble for interface systems**: for membrane simulations, the $z$-dimension must be long enough that adjacent periodic membrane images do not interact. Typical: $L_z \geq 10$ nm for a ~4 nm bilayer.

## Why This Matters

PBC transforms an ill-posed problem (simulating a finite droplet of protein in solution with an artificial vacuum interface) into a physically reasonable approximation of bulk behavior. Without PBC, the simulation would produce surface tension artifacts and incorrect solvation thermodynamics. The minimum image convention is simple enough to implement in 3 lines of code but has profound consequences: it is the reason MD simulations of proteins in solution can faithfully reproduce experimental observables like NMR chemical shifts, hydrogen-exchange rates, and binding free energies, despite simulating only ~100,000 atoms out of Avogadro's number.
