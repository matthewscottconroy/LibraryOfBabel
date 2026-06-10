# Chapter 19: Spacetime and the Minkowski Metric

---

## Chapter Introduction

In 1905, Einstein proposed two postulates and derived their consequences: time is not absolute; simultaneity is relative; moving clocks run slow; moving rulers shrink. These are the kinematic consequences of special relativity, and they follow unavoidably from the constancy of the speed of light.

But the deeper geometric understanding came from Hermann Minkowski in 1908. Minkowski recognized that Einstein's kinematics could be understood as geometry — not the geometry of 3D space and separate 1D time, but of a 4-dimensional spacetime with a specific (indefinite) metric:

$$ds^2 = -c^2 dt^2 + dx^2 + dy^2 + dz^2$$

Minkowski presented this at the 80th Assembly of German Natural Scientists in Cologne in 1908 with the famous words: "Henceforth space by itself, and time by itself, are doomed to fade away into mere shadows, and only a kind of union of the two will preserve an independent reality."

Einstein was initially irritated — he felt Minkowski had merely re-expressed his physical insight in unnecessary mathematical language. He changed his mind when he found, a few years later, that the generalizing of this flat-spacetime geometry to curved spacetime (GR) required exactly the mathematical framework that Minkowski had provided.

This chapter develops Minkowski spacetime: the metric, the invariant interval, the causal structure (timelike, spacelike, null), the Lorentz group, and the kinematic consequences (time dilation, length contraction, relativistic addition of velocities).

---

## Chapter Contents

- **Section 19.1**: Minkowski spacetime; the spacetime interval; timelike, spacelike, and null separation; the light cone; the causal structure of spacetime

- **Section 19.2**: Lorentz transformations; the Lorentz group; time dilation; length contraction; the relativity of simultaneity; the twin paradox

---

## The Two Postulates

Einstein's 1905 paper "On the electrodynamics of moving bodies" (*Zur Elektrodynamik bewegter Körper*, *Annalen der Physik*, 17, 891–921) begins with just two postulates:

1. **Principle of Relativity**: The laws of physics (including electrodynamics) are the same in all inertial frames.

2. **Constancy of the Speed of Light**: Light in vacuum propagates at speed $c$ in all inertial frames, regardless of the motion of the source or observer.

These two postulates are in apparent conflict: if A moves relative to B, how can both measure the same speed for the same light beam? The resolution requires abandoning absolute simultaneity — the question "what is happening now at a distant location" has no frame-independent answer.

From these two postulates, the entire kinematic structure of special relativity follows: the Lorentz transformation, time dilation, length contraction, the relativity of simultaneity, and the invariance of the spacetime interval $ds^2 = -c^2 dt^2 + d\mathbf{r}^2$.

**The connection to GR**: The Minkowski metric $\eta_{\mu\nu}$ is the flat-spacetime metric. In GR, it is replaced by the dynamical metric $g_{\mu\nu}$ of curved spacetime. All the formulas of special relativity generalize to GR by the minimal coupling principle: replace $\eta_{\mu\nu}$ with $g_{\mu\nu}$ and $\partial_\mu$ with $\nabla_\mu$ (the covariant derivative). Special relativity is the $g_{\mu\nu} = \eta_{\mu\nu}$ limit.
