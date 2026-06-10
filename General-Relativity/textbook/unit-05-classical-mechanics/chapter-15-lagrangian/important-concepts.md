# Chapter 15: Important Concepts

---

**The Action Principle (Hamilton's Principle)**
The physical trajectory of a system is the one that makes the action $S[q] = \int_{t_1}^{t_2}L(q,\dot{q},t)\,dt$ stationary ($\delta S = 0$) among all paths connecting fixed endpoints. Not "minimum" action — stationary. This variational formulation encodes all of mechanics in a single scalar functional. The same structure (with appropriate $L$) yields Maxwell's equations, Klein-Gordon, Einstein's equations.

**Euler-Lagrange Equations**
$\partial L/\partial q^i - d/dt(\partial L/\partial \dot{q}^i) = 0$: the necessary and sufficient condition for $\delta S = 0$. Derived by integration by parts; equivalent to Newton's second law for $L = T - V$ in Cartesian coordinates, but valid in any coordinate system and for any constraints. In GR, the geodesic equation is the Euler-Lagrange equation for the geodesic Lagrangian $L = g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu$.

**Generalized Coordinates**
Any independent coordinates $q^i$ describing the configuration space. They need not be positions — angles, lengths, field amplitudes, etc. The E-L equations take the same form in any generalized coordinates. This **coordinate covariance** is the key advantage over Newton's laws (which must be re-derived for each new coordinate system).

**Cyclic Coordinates and Conservation**
If $L$ does not depend on $q^i$ (called a cyclic or ignorable coordinate): $\partial L/\partial q^i = 0$, and the E-L equation gives $\dot{p}_i = 0$ — the generalized momentum $p_i = \partial L/\partial \dot{q}^i$ is conserved. Angular momentum from cyclic $\phi$ in a spherically symmetric potential is the prototype. In GR, cyclic coordinates correspond to Killing vectors.

**Noether's Theorem**
Every one-parameter symmetry of the action (a transformation that changes $L$ by at most a total time derivative) gives a conserved Noether charge $J = (\partial L/\partial\dot{q}^i)K^i - F$. The theorem unifies all conservation laws: energy (time translation), momentum (space translation), angular momentum (rotation), charge (gauge symmetry). In GR: diffeomorphism invariance gives the contracted Bianchi identity $\nabla_\mu G^{\mu\nu} = 0$.

**Legendre Transform**
The Hamiltonian $H(q,p,t) = p_i\dot{q}^i - L(q,\dot{q},t)$ (summing over $i$), where $\dot{q}^i$ is expressed in terms of $p_i$ by inverting $p_i = \partial L/\partial\dot{q}^i$. The Legendre transform converts a function of velocities (Lagrangian) into a function of momenta (Hamiltonian). Requires $L$ to be strictly convex in $\dot{q}$ (non-degenerate). For degenerate Lagrangians (e.g., GR), the Dirac-Bergmann theory of constrained Hamiltonians applies.

**Hamilton's Equations**
$\dot{q}^i = \partial H/\partial p_i$ and $\dot{p}_i = -\partial H/\partial q^i$: a system of $2n$ first-order ODEs equivalent to the $n$ second-order E-L equations. Phase space has dimension $2n$. Hamilton's equations are a Hamiltonian vector field on phase space. Their elegant symmetry between $q$ and $p$ (up to a sign) is the beginning of the symplectic viewpoint.

**Poisson Brackets**
$\{f,g\} = \partial f/\partial q^i\,\partial g/\partial p_i - \partial f/\partial p_i\,\partial g/\partial q^i$: a bilinear, antisymmetric, Leibniz operation on phase-space functions. Generates time evolution: $\dot{f} = \{f, H\}$. Conserved quantities satisfy $\{f, H\} = 0$. Under canonical quantization: $\{f,g\} \to (1/i\hbar)[\hat{f},\hat{g}]$. The fundamental brackets $\{q^i, p_j\} = \delta^i_j$ become the canonical commutation relations.

**Symplectic Structure**
The 2-form $\omega = dp_i \wedge dq^i$ on phase space. Preserved by Hamiltonian flow (Liouville's theorem): $\mathcal{L}_{X_H}\omega = 0$. Phase-space volume (a power of $\omega$) is preserved — an ensemble of systems occupies a constant volume in phase space. This is the geometric foundation of statistical mechanics. In GR: the ADM phase space has $\Omega = \int \delta\pi^{ij} \wedge \delta h_{ij}\,d^3x$.

**Integrable Systems**
A system with $n$ degrees of freedom is completely integrable if it has $n$ independent conserved quantities in involution ($\{F_i, F_j\} = 0$). By the Arnol'd-Liouville theorem, motion is quasi-periodic on $n$-tori (invariant tori). The Kepler problem is integrable. The 3-body problem is not (in general).

**KAM Theorem**
Most invariant tori of a completely integrable Hamiltonian system persist under small perturbations (Kolmogorov 1954, Arnol'd 1963, Moser 1962). Tori with "sufficiently irrational" frequencies survive; those with rational or nearly rational frequency ratios are destroyed and replaced by chaotic layers. This explains why the solar system has (mostly) survived for $4.5 \times 10^9$ years despite perturbations.

**Carter Constant**
A fourth constant of motion for geodesics in Kerr (rotating black hole) spacetime, discovered by Brandon Carter in 1968. It is the conserved charge associated with a rank-2 Killing tensor $K_{\mu\nu}$ (not a Killing vector). With $E$, $L_z$, and the Carter constant $Q$, the Kerr geodesic equations are integrable — they separate into four 1D problems, enabling efficient computation of gravitational wave templates.

**The Energy Problem in GR**
Noether's theorem guarantees conserved energy when spacetime has a timelike Killing vector. But generic GR spacetimes (cosmological, dynamic, strongly curved) have no such Killing vector. There is no local, gauge-invariant gravitational energy density. The ADM energy (for asymptotically flat spacetimes) and the Bondi energy (for asymptotically flat spacetimes including radiation) are the well-defined global energy concepts in GR.
