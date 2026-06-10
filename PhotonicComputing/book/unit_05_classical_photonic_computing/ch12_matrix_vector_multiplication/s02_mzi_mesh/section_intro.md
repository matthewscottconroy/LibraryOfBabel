# Section 12.2: The MZI Mesh

## What This Section Is About

The MZI mesh is the most physically elegant implementation of optical matrix-vector multiplication. A $2 \times 2$ MZI performs a $2 \times 2$ unitary transformation. By connecting MZIs in a specific network topology, we can decompose any $N \times N$ unitary matrix into a product of $2 \times 2$ rotations — the photonic analog of the Givens rotation decomposition in numerical linear algebra.

This section derives this decomposition from first principles:

**12.2.1: MZI as a Unitary Gate** — Transfer matrix of a single MZI; parameterization by two phase angles; the space of achievable transformations.

**12.2.2: Reck Decomposition** — The 1994 Reck-Zeilinger scheme for decomposing any $N \times N$ unitary matrix into a triangular mesh of MZIs; column-by-column Givens rotation approach; depth $2N-3$.

**12.2.3: Clements Decomposition** — The 2016 Clements scheme; rectangular mesh with depth $N$; why it is more hardware-efficient than Reck; the diagonal element handling.

**12.2.4: Programming and Errors** — How to set the phase angles for a desired matrix $W$; sensitivity to phase errors; the calibration procedure; demonstrated performance in silicon photonic implementations.
