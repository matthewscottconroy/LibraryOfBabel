# 43.4 Homotopy Groups and Dynamical Invariants

## 43.4.1 Fundamental Groups as Dynamical Invariants

The fundamental group $\pi_1(X, x_0)$ is a topological invariant — it's preserved by homeomorphisms. Since topological conjugacy preserves the topology of the phase space, the fundamental group is a topological conjugacy invariant of any dynamical system on $X$.

**Theorem 43.4.1.** For a topological dynamical system $(X, f)$, the fundamental group $\pi_1(X)$ is an invariant (preserved by homeomorphism). More refined: the homotopy type of $X$ is a topological conjugacy invariant.

**In HoTT:** The fundamental group $\pi_1(A, a) := \Omega^1(A, a) = (a =_A a)$ (the type of self-paths at basepoint $a$). For a 1-type (groupoid): every equality has a proof, and the group of "proof equalities" is exactly $\pi_1(A, a)$.

The HoTT formulation makes the fundamental group a purely type-theoretic object: it's the type of paths from the basepoint to itself. Self-paths are loops. And the group operation is path concatenation. This is exactly the classical definition of $\pi_1$, but now it's internal to type theory.

**Definition 43.4.2 (Loop Space Dynamics).** For a based topological space $(X, x_0)$, the loop space $\Omega X = \{f: [0,1] \to X : f(0) = f(1) = x_0\}$ with the concatenation operation is a monoid. The iteration $\Omega^n X$ gives the $n$-th loop space.

**Connection to Ergodic Theory:** For a flow $\phi_t: X \to X$, a *periodic orbit* is a loop in $X$. The homotopy class of this loop in $\pi_1(X)$ is a topological invariant of the orbit. The *Massey products* in the cohomology of $X$ constrain which homotopy classes can support periodic orbits.

This connection between periodic orbits and the topology of the phase space is the subject of topological dynamics and Conley index theory. Periods and homotopy classes are linked by the Lefschetz fixed point theorem: the number of fixed points of a map (counted with multiplicity) equals the Lefschetz number, which is computed from the action of $f$ on homology.

In HoTT, the loop space $\Omega X$ is the type of self-paths at $x_0$, and the $n$-th iterated loop space $\Omega^n X$ captures the $n$-th homotopy group. Studying the dynamics on the loop space — how the map $f$ acts on $\pi_n(X)$ — is an active area connecting dynamics to algebraic topology.
