# Chapter 30 — Optimal Transport and Dynamical Systems

> *Moving a pile of dirt to a hole: this is the Monge problem. Kantorovich relaxed it to couplings. Villani showed the optimal coupling has geometric regularity. The Wasserstein distance is the geometry of probability measures — and it connects directly to the geometry of dynamical systems.*

**Prerequisites:** Chapter 2 (measure theory, weak convergence), Chapter 5 (linear algebra, optimization), Chapter 16 (entropy, KL divergence).

---

In 1781, Gaspard Monge posed a problem that sounds entirely practical: given a pile of dirt and a collection of holes, what is the most efficient way to move the dirt into the holes? "Efficient" means minimizing the total distance traveled, weighted by the amount of dirt moved. This is an optimization problem, and for 150 years it resisted complete solution.

Leonid Kantorovich reformulated it in 1942 and received the Nobel Prize in Economics partly for this work (he applied the theory to resource allocation in the Soviet Union during World War II). His key move was to relax the problem from "transport maps" (where each grain of dirt goes to a unique hole) to "transport plans" (where dirt can be split). This relaxation turns a hard nonlinear problem into a linear programming problem with clean duality theory.

The deeper structure emerged later. Yann Brenier showed in 1991 that the optimal transport map, for the squared-distance cost function, is always the gradient of a convex function. This is a statement about differential geometry. John Lott and Cédric Villani (and independently Karl-Theodor Sturm) showed in 2006 that this gradient structure gives a way to define Ricci curvature for spaces far more general than smooth manifolds — metric measure spaces, graphs, fractals. Optimal transport turned into a theory of curvature.

And the connection to dynamics: the Wasserstein space of probability measures, equipped with the $W_2$ metric, is the natural phase space for the evolution of probability distributions under a huge class of PDEs. The heat equation is the gradient flow of entropy in Wasserstein space. The Fokker-Planck equation is the gradient flow of free energy. Optimal transport provides the geometry in which these evolutions are most naturally understood.

---

## Sections

- [30.1 — The Monge-Kantorovich Problem](the-monge-kantorovich-problem.md)
- [30.2 — Brenier's Theorem and Geometry](breniers-theorem-and-geometry.md)
- [30.3 — Wasserstein Space as a Metric Space](wasserstein-space-as-a-metric-space.md)
- [30.4 — Gradient Flows in Wasserstein Space](gradient-flows-in-wasserstein-space.md)
- [30.5 — Entropy and Curvature — Lott-Sturm-Villani](entropy-and-curvature-lott-sturm-villani.md)
- [30.6 — Optimal Transport and Information Theory](optimal-transport-and-information-theory.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
