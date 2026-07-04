# Section 13.2: Optical Activation Functions

## What This Section Is About

A stack of MZI meshes without nonlinearities between them is, mathematically, one mesh. The activation function is therefore not an implementation detail of a photonic neural network — it is the difference between a deep network and an expensive single matrix. And it is the place where the physics gets uncooperative: photons do not interact with one another in vacuum or in linear dielectrics, and the material nonlinearities that can make them appear to interact (Chapter 11 catalogued them: Kerr, saturable absorption, free-carrier and thermal effects, gain saturation) demand optical powers that sit orders of magnitude above the femtojoule budgets that make photonic computing attractive in the first place.

This section confronts the problem in three steps:

**13.2.1: The Problem** — Why linear networks collapse; the requirements list for a usable activation (nonlinear shape, cascadability, gain, fan-out, speed, energy, reproducibility, differentiability); why the optical logic verdict of Chapter 11 reappears here in analog form, and why the analog version is *less* damning.

**13.2.2: Electro-Optic Nonlinear Activation** — The pragmatic road: detect, transform in the electrical domain, re-modulate. The modulator transfer function as a free sigmoid; the Williamson/Hughes programmable electro-optic activation; the Ashtiani end-to-end on-chip classifier; energy and latency accounting, and the $O(N)$-vs-$O(N^2)$ argument that makes O-E-O tolerable.

**13.2.3: All-Optical Approaches** — Saturable absorption, cavity-enhanced Kerr and free-carrier bistability, SOA gain dynamics, and exotic media (atomic vapors); what has actually been demonstrated; the power-threshold arithmetic that keeps these devices out of deployed systems; and the architectures (reservoirs, single-nonlinear-layer networks) that get expressivity from physics without per-neuron nonlinear devices.
