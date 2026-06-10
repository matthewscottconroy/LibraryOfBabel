# Section 13.3: Connectedness and Homotopy

---

## Section Introduction

**Connectedness** asks whether a space can be split into separate pieces. **Homotopy** asks whether continuous deformations can transform one map (or space) into another. Together, they form the beginning of **algebraic topology** — the program of understanding topological spaces through algebraic invariants.

Two spaces are **homeomorphic** if there exists a continuous bijection with a continuous inverse — they are "topologically the same." A circle and a triangle are homeomorphic; a circle and a figure-eight are not (removing one point from a circle leaves a connected space, but removing the center of a figure-eight disconnects it). The fundamental question of topology is: when are two spaces homeomorphic?

The **fundamental group** $\pi_1(X, x_0)$ is the first algebraic invariant. Two loops based at $x_0$ are **homotopic** if one can be continuously deformed into the other while keeping the basepoint fixed. The set of homotopy classes of loops forms a group under concatenation. For $\mathbb{R}^n$ or any contractible space: $\pi_1 = \{1\}$ (trivial). For the circle $S^1$: $\pi_1 = \mathbb{Z}$ — loops are classified by winding number. For a torus: $\pi_1 = \mathbb{Z}\times\mathbb{Z}$.

For GR, the topology of spacetime has physical consequences. The global structure of the Schwarzschild and Kerr spacetimes is revealed by their Penrose diagrams — a topological tool. The existence of closed timelike curves (time travel) is a topological question. The topology of spatial sections constrains what the universe "looks like" at large scales; the cosmic topology question (does the universe have nontrivial spatial topology, like a torus?) is observationally probed by patterns in the CMB. Spinors in GR are sensitive to the double-cover topology of $SO(3)$, connected to $\pi_1(SO(3)) = \mathbb{Z}/2$.

---

## Subsections

- [13.3.1: Connected and Path-Connected Spaces](13.3.1-connected.md)
- [13.3.2: Homotopy of Paths and Maps](13.3.2-homotopy.md)
- [13.3.3: The Fundamental Group](13.3.3-fundamental-group.md)
- [13.3.4: Higher Homotopy Groups](13.3.4-higher.md)
- [13.3.5: Topological Invariants and Classification](13.3.5-invariants.md)
