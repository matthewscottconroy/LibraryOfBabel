# Chapter 15 — Further Reading and References

---

## Essential References

### [Gauthier2021] — The NVAR Paper

**Gauthier, D.J., Bollt, E., Griffith, A., & Barbosa, W.A.S. (2021). Next generation reservoir computing. *Nature Communications*, 12, 5564.**

The paper that introduced NVAR. Short, clearly written, and contains everything needed to reproduce the main results. The supplementary materials give full algorithmic details. Required reading for this chapter. The code is available on GitHub (linked in the paper).

### [Rahimi2007] — Random Features

**Rahimi, A. & Recht, B. (2007). Random features for large-scale kernel machines. *Advances in Neural Information Processing Systems*, 20.**

The paper proving that random cosine features approximate any shift-invariant kernel. The proof is concise and elegant. Understanding this result is essential for the ESN-as-kernel-method perspective.

### [Bollt2021] — Theoretical Analysis of NG-RC

**Bollt, E. (2021). On explaining the surprising success of reservoir computing forecaster of chaos? The universal machine learning dynamical system with contractive maps. *Chaos*, 31(1), 013108.**

The deepest theoretical analysis of why NVAR works. Bollt shows that both NVAR and ESN can be understood as contractive random maps and characterizes the conditions under which polynomial features are sufficient. Essential for readers who want a rigorous understanding of the Gauthier et al. results.

---

## Background: Volterra Series

### [Schetzen1980]

**Schetzen, M. (1980). *The Volterra and Wiener Theories of Nonlinear Systems*. Wiley.**

The standard reference for Volterra series methods. Chapter 2 (discrete-time Volterra series) is directly relevant to NVAR. Shows that NVAR is not a new idea but a new implementation of a classical framework.

### [Boyd1985]

**Boyd, S. & Chua, L.O. (1985). Fading memory and the problem of approximating nonlinear operators with Volterra series. *IEEE Transactions on Circuits and Systems*, 32(11), 1150–1161.**

As discussed in Chapter 1: the theoretical foundation showing that any fading-memory functional can be approximated by Volterra series. NVAR's guarantees rest on this result.

---

## Background: Kernel Methods

### [Scholkopf2002]

**Schölkopf, B. & Smola, A.J. (2002). *Learning with Kernels: Support Vector Machines, Regularization, Optimization, and Beyond*. MIT Press.**

The definitive reference for kernel methods. Chapters 2–4 cover RKHS theory, Mercer's theorem, and the connection to regularization. Chapter 16 covers kernels on sequences, which is the framework for temporal kernel methods.

### [Rahimi2009]

**Rahimi, A. & Recht, B. (2009). Weighted sums of random kitchen sinks: Replacing minimization with randomization in learning. *NIPS 2009*, 21.**

A follow-up that gives sharper concentration bounds and discusses the statistical properties of random feature regression in more depth.

---

## Background: Lorenz System and Chaos Forecasting

### [Lorenz1963]

**Lorenz, E.N. (1963). Deterministic nonperiodic flow. *Journal of Atmospheric Sciences*, 20(2), 130–141.**

The original Lorenz system paper. Introduces the equations and the concept of sensitive dependence on initial conditions. Foundational context for the prediction benchmark.

### [Pathak2018]

**Pathak, J., Hunt, B., Girvan, M., Lu, Z., & Ott, E. (2018). Model-free prediction of large spatiotemporally chaotic systems from data: A reservoir computing approach. *Physical Review Letters*, 120(2), 024102.**

The ESN on KS equation paper (detailed in Chapter 20). Represents the counter-case to NVAR: for high-dimensional chaotic systems, ESN significantly outperforms the polynomial/Volterra approach.

---

## Hybrid and Extensions

### [Barbosa2022]

**Barbosa, W.A.S., Griffith, A., & Gauthier, D.J. (2022). Learning spatiotemporal chaos using next-generation reservoir computing. *Chaos*, 32(9), 093137.**

Extends NVAR to spatiotemporal systems by combining polynomial features with spatial structure. Partially bridges the gap between NVAR and ESN for high-dimensional systems.

### [Griffith2019]

**Griffith, A., Pomerance, A., & Gauthier, D.J. (2019). Forecasting chaotic systems with very low connectivity reservoir computers. *Chaos*, 29(12), 123108.**

Precursor to the NVAR paper. Shows that ESNs with very sparse connectivity can perform well on chaos prediction, motivating the question of whether explicit features could do even better.
