**Corresponds to:** Chapter 15 — Next-Generation Reservoir Computing: NVAR and the Kernel Connection (Unit 6)

**Prerequisites:** Chapter 5 (ESN basics), polynomial feature expansion, ridge regression, Lorenz system (Lab 02)

**Learning Objectives:**
- Implement NVAR (Nonlinear Vector Autoregression): delay embedding followed by degree-2 polynomial feature expansion, then ridge regression — no reservoir needed
- Understand why NVAR is remarkably parameter-efficient: the quadratic monomials directly encode the nonlinearities present in the Lorenz RHS (xy and xz terms)
- Compare NVAR and ESN on Lorenz prediction using NRMSE and Valid Prediction Time (VPT)
- Understand when NVAR wins (low-dimensional polynomial dynamics, known structure) vs. when ESN wins (high-dimensional, unknown nonlinearity)
- Connect NVAR to the random features / kernel approximation perspective (Rahimi & Recht 2007)
