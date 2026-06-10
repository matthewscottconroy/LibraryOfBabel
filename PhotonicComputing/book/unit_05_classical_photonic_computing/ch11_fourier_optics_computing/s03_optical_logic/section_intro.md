# Section 11.3: Optical Logic Gates

## What This Section Is About

The history of optical computing includes a sustained, expensive, and ultimately unsuccessful effort to build Boolean logic gates from light — transistor-equivalent devices where optical inputs control optical outputs. Understanding why this effort failed, and what physical principles made it inevitable that it would fail, is not a digression: it is essential for evaluating the claims made for modern photonic computing.

The word "computing" carries an implicit model: universal computation, capable of executing arbitrary programs, including conditional branching and memory operations. Classical photonic computing (MZI meshes, diffractive networks) does not fit this model — it performs specific linear algebra operations efficiently but cannot branch or remember without electronic help. The optical logic project aimed to build a truly all-optical computer, and its failure teaches us something fundamental about what light can and cannot do.

Three subsections:

**11.3.1: Why Optical Logic Is Hard** — The physical requirements for a logic gate; why optical nonlinearity is required; why the energy-per-operation in optical nonlinear elements is unfavorable; the threshold switching problem.

**11.3.2: Semiconductor Optical Amplifier (SOA) Gates** — Cross-gain and cross-phase modulation in SOAs; all-optical XOR and NOT; why SOA logic fails to compete with CMOS; the carrier lifetime bottleneck.

**11.3.3: Phase-Change Material Logic** — GST/GSST non-volatile switches as optical memory; PCM-based optical logic demonstrations; honest assessment of PCM logic vs. CMOS.
