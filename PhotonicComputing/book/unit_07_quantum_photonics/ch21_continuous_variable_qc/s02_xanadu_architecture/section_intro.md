# Section 21.2: Xanadu's Photonic Quantum Computer

Xanadu Quantum Technologies, founded in Toronto in 2016 by Christian Weedbrook, is the company most fully committed to the continuous-variable paradigm developed in Section 21.1. Where PsiQuantum (Chapter 20) pursues fault tolerance with dual-rail single-photon qubits and fusion gates, Xanadu's stack is built on squeezed light: deterministic squeezers on silicon nitride chips, Gaussian interferometers, photon-number-resolving detectors to supply the mandatory non-Gaussian ingredient, and — on the fault-tolerance roadmap — GKP qubits woven into CV cluster states.

This strategy has already produced one of the field's landmark results. In June 2022, Xanadu's *Borealis* machine performed Gaussian boson sampling on 216 squeezed temporal modes and reported a quantum computational advantage: samples in tens of microseconds that the best known classical algorithms would need thousands of years to replicate exactly. Beyond the raw claim, Borealis mattered for *how* it was built — a single squeezer and three fiber delay loops, time-multiplexed into a large entangled state, fully programmable from the cloud — a preview of how CV hardware sidesteps the component-count explosion that haunts spatial-mode architectures.

Xanadu's other outsized contribution is software. PennyLane, its open-source framework for differentiable quantum programming, treats quantum circuits the way PyTorch treats neural networks — as parametrized functions you can take gradients of — and has become one of the most widely used quantum software tools on any platform, agnostic to whether the backend is photonic, superconducting, or trapped-ion.

This section examines the hardware (21.2.1), the software (21.2.2), and the algorithms the platform is designed to run (21.2.3), closing with a candid assessment of how the CV approach's tradeoffs compare with the discrete-variable alternative.

## Subsections

- **21.2.1 — The Borealis Architecture**: Time-domain multiplexing; squeezed-light sources; the three-loop interferometer; photon-number-resolving readout; the 2022 quantum-advantage experiment and its classical-simulation counterattacks; the path from Borealis toward the fault-tolerant Aurora architecture.
- **21.2.2 — The PennyLane Software Framework**: Differentiable quantum programming; parameter-shift gradients; Strawberry Fields and CV simulation backends; hardware-agnostic hybrid quantum-classical workflows.
- **21.2.3 — Quantum Machine Learning and CV Algorithms**: Gaussian boson sampling applications (graphs, vibronic spectra); CV quantum neural networks; quantum kernels; what classical simulability does and does not rule out; CV versus DV tradeoffs.
