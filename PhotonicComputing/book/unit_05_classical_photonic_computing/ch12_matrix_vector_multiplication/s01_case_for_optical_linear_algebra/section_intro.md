# Section 12.1: The Case for Optical Linear Algebra

## What This Section Is About

Before examining how optical systems perform matrix multiplication, we should ask whether they can do so more efficiently than electronics, and under what conditions. This section makes the physical argument precisely, distinguishes two fundamentally different approaches (coherent and incoherent), and establishes the figures of merit that determine when optical matrix processors are genuinely advantageous.

Three subsections:

**12.1.1: Computational Complexity and the Optical Advantage** — Why matrix-vector products are $O(N^2)$ operations; why a coherent optical system performs them "for free" (in the time it takes light to traverse the processor); the energy-delay product comparison between optical and electronic implementations.

**12.1.2: Analog vs. Digital Computing** — What it means for a computation to be "analog"; the precision-energy tradeoff; why 6–8 bits of precision is adequate for neural network inference; the role of error correction in extending effective precision.

**12.1.3: The Optical Multiply-Accumulate (MAC)** — How optical interference implements a complex-valued multiply-and-add; what one photonic MAC costs in energy; the comparison to electronic MAC energy at the same precision.
