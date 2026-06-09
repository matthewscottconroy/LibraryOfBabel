# Chapter Notes — Chapter 8

A mentor's guide to the stability theory literature — what's primary, what's worth skimming, and what repays deep study.

---

**The original source.** Lyapunov's *The General Problem of the Stability of Motion* (1892, translated and republished by Taylor & Francis, 1992) introduced both the linearization method and the direct method. It is the foundation of stability theory for dynamical systems and control theory. Reading the original is rewarding — Lyapunov's geometric intuitions are remarkably clear, and the historical context illuminates why the problems he solved were considered difficult.

**The engineering perspective.** Khalil's *Nonlinear Systems* (3rd edition, Prentice Hall, 2002) is the standard graduate control text and gives an accessible, application-oriented treatment of all the material in this chapter. The emphasis is on finding Lyapunov functions for specific system classes and on robustness analysis. Chapter 4 (Lyapunov stability) and Chapter 5 (input-output stability) cover what's here. The exercises are excellent.

**The pure mathematics treatment.** Bhatia and Szegő's *Stability Theory of Dynamical Systems* (Springer, 1970) is the rigorous mathematical reference. It's harder and more abstract than Khalil, but it covers the topological aspects of stability theory (including the connection to omega-limit sets and the Poincaré-Bendixson theorem) that the engineering books tend to skip.

**The Oseledec theorem.** Oseledec's 1968 paper is the primary source, but it's dense. The cleanest accessible presentation is in the appendix of Katok-Hasselblatt's *Introduction to the Modern Theory of Dynamical Systems* (Cambridge, 1995). Mañé's *Ergodic Theory and Differentiable Dynamics* (Springer, 1987) also contains a complete proof, in the context of a broader development of smooth ergodic theory.

**Pesin's formula.** Pesin's 1977 paper (*Math. USSR Izvestiya*) is where the entropy-exponent formula appears. The modern treatment is in Liu and Qian's *Smooth Ergodic Theory for Endomorphisms* (Springer, 2009). For the connection to SRB measures (which the formula extends to non-invariant measures), see Ledrappier-Young's 1985 papers in the *Annals of Mathematics* — hard but essential if you want to understand smooth ergodic theory at a research level.

**Floquet theory.** The best presentations are in ordinary differential equations texts rather than dynamics books. Chicone's *Ordinary Differential Equations with Applications* (Springer, 1999) has a complete and clear treatment. For the application to stability of limit cycles and Hopf bifurcation, see Guckenheimer-Holmes, Chapter 3.

**A word on finding Lyapunov functions.** In practice, finding a Lyapunov function for a specific system is hard. Recent progress has come from sum-of-squares (SOS) optimization: representing $V$ as a polynomial and using semidefinite programming to certify that it's positive definite and $\dot{V}$ is negative definite. Parrilo's work (Parrilo 2000, MIT thesis) is the starting point. For simple systems, the energy function is usually the first thing to try; for more complex systems, you often need to construct $V$ from the structure of the problem.
