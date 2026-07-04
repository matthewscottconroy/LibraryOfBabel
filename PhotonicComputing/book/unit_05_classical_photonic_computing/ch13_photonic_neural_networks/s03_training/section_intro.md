# Section 13.3: Training Photonic Neural Networks

## What This Section Is About

Backpropagation assumes you know the function you are differentiating. A photonic neural network's true forward function is a physical process — phases with fabrication offsets, couplers a percent off nominal, detectors with noise — that never exactly matches any digital description of it. Training is therefore not just an algorithm question but an *epistemology* question: how do you optimize a system you cannot perfectly model?

The field's answers form an escalation ladder, ordered by how much the physical hardware participates in its own training:

**13.3.1: Offline Training** — Train a digital twin, then transfer the weights. The baseline approach behind nearly every demonstration; works exactly as well as the twin is faithful, and the transfer gap (the "sim-to-real" gap, borrowed from robotics) is its defining pathology.

**13.3.2: In-Situ Training** — Make the hardware compute its own gradients: zeroth-order methods that need only forward evaluations (finite differences, SPSA), and the elegant adjoint result of Hughes et al. — backpropagation as physical backward light propagation through the reciprocal mesh — experimentally realized by Pai et al. (2023).

**13.3.3: Noise-Aware Training** — Inject the hardware's stochasticity into the digital training loop so the optimizer finds weights that are *flat* against perturbations. Cheap, effective against zero-mean noise, powerless against systematic miscalibration; the distinction between the two error classes organizes the whole subsection.

**13.3.4: Hardware-in-the-Loop Training** — Put the physical forward pass inside the training loop and backpropagate through a learned or approximate model, as in physics-aware training (Wright et al. 2022) and the adaptive training of large diffractive systems. The pragmatic frontier: it absorbs *all* static hardware error without requiring a differentiable hardware.

A single organizing question runs through all four: *where does the gradient come from, and what does it cost to get it?*
