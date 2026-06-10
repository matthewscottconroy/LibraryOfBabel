# Chapter 15 — Key Researchers

---

## Daniel J. Gauthier

**Affiliation:** Ohio State University, Department of Physics

**Contributions:** Gauthier is the lead author of the Nature Communications paper [Gauthier2021] that introduced the NVAR framework and demonstrated its competitive performance against ESNs on the Lorenz system. His research spans nonlinear dynamics, optical chaos, and machine learning for dynamical systems. The NVAR paper represents a convergence of these interests: the system studied (Lorenz) is a nonlinear dynamical system; the method used (polynomial regression on observables) is explicitly motivated by the Volterra series theory of nonlinear systems; and the evaluation metric (valid prediction time in Lyapunov units) comes from dynamical systems theory.

Gauthier's group has subsequently extended the NVAR framework to spatiotemporal systems, noisy observations, and hybrid NVAR-ESN architectures that combine the efficiency of polynomial features with the memory of reservoir states.

**Selected publications:**
- Gauthier, D.J., Bollt, E., Griffith, A., & Barbosa, W.A.S. (2021). Next generation reservoir computing. *Nature Communications*, 12, 5564.
- Griffith, A., Pomerance, A., & Gauthier, D.J. (2019). Forecasting chaotic systems with very low connectivity reservoir computers. *Chaos*, 29(12), 123108.

---

## Erik Bollt

**Affiliation:** Clarkson University, Department of Mathematics; C3S2 (Clarkson Center for Complex Systems Science)

**Contributions:** Co-author of the NVAR paper [Gauthier2021] and a leading theorist in the mathematics of dynamical systems and machine learning. Bollt's subsequent paper "On explaining the surprising success of reservoir computing forecaster of chaos" [Bollt2021] provides the deepest theoretical analysis of why NVAR works: he shows that both NVAR and ESN are instances of contractive random maps and characterizes the conditions under which polynomial features suffice. This analysis confirms and extends the Volterra series interpretation developed in Section 15.2.

**Selected publications:**
- Bollt, E. (2021). On explaining the surprising success of reservoir computing forecaster of chaos? The universal machine learning dynamical system with contractive maps. *Chaos*, 31(1), 013108.
- Bollt, E. & Santitissadeekorn, N. (2013). *Applied and Computational Measurable Dynamics*. SIAM.

---

## Ali Rahimi

**Affiliation:** Google Brain (Research Scientist; formerly UC Berkeley)

**Contributions:** Together with Benjamin Recht, Rahimi proved the random features theorem [Rahimi2007] that connects random nonlinear projections to kernel approximation. This theorem is the mathematical foundation for understanding ESNs as kernel machines and for placing both NVAR and ESN within the unified framework of kernel methods on time series. Rahimi gave the "Test of Time" award lecture at NeurIPS 2017 reflecting on the random features paper — an accessible and philosophically rich discussion of what the theorem means for machine learning practice.

**Selected publications:**
- Rahimi, A. & Recht, B. (2007). Random features for large-scale kernel machines. *NIPS 2007*, 20.
- Rahimi, A. & Recht, B. (2009). Weighted sums of random kitchen sinks: Replacing minimization with randomization in learning. *NIPS 2009*, 21.

---

## Benjamin Recht

**Affiliation:** University of California, Berkeley, Department of Electrical Engineering and Computer Sciences and Department of Statistics

**Contributions:** Co-inventor with Rahimi of random features [Rahimi2007], a technique that has had enormous influence in kernel methods, neural network theory, and reservoir computing. Recht's broader research on optimization, statistical learning theory, and the mathematics of machine learning provides the theoretical framework in which the random features result is best understood. His "Reflections on Random Kitchen Sinks" (2017, co-authored with Rahimi) is an excellent non-technical discussion.

**Selected publications:**
- Rahimi, A. & Recht, B. (2007). Random features for large-scale kernel machines. *NIPS 2007*.
- Recht, B. et al. (2010). Guaranteed minimum-rank solutions of linear matrix equations via nuclear norm minimization. *SIAM Review*, 52(3), 471–501.
