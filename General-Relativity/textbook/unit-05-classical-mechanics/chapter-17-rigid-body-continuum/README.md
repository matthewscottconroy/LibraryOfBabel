# Chapter 17: Rigid Body Dynamics and Continuum Mechanics

---

## Chapter Introduction

Everything we have done so far — single particles, collections of particles, generalized coordinates — ultimately reduces the dynamics to point masses. But the real physical world consists of extended objects: spinning tops, rotating planets, vibrating strings, flowing fluids. These are the domains of rigid body dynamics and continuum mechanics.

The rigid body is the simplest extended object: a collection of particles whose mutual distances are fixed. It has six degrees of freedom — three for the position of the center of mass, and three for orientation. The orientation part is the hard part. Describing how a body rotates requires careful mathematics (rotation groups, Euler angles, moment of inertia tensor), and the dynamics can be surprisingly complex — from the simple precession of a gyroscope to the strange, tumbling motion of asymmetric bodies (Euler's equations, the Tennis Racket theorem).

Continuum mechanics generalizes further: a continuous material (fluid, elastic solid) has infinitely many degrees of freedom — a field at every point. The Lagrangian and Hamiltonian formulations extend to field theories, introducing the Lagrangian density $\mathcal{L}$ and the Euler-Lagrange equations for fields. This is the prototype for every quantum field theory and for GR itself — the Einstein-Hilbert action is the action for the "gravitational field" (the metric), and its Euler-Lagrange equations are Einstein's field equations.

---

## The Rotation Group

A **rigid body** has a fixed reference configuration. Its orientation at time $t$ is described by a rotation: a map $R(t)\in SO(3)$ such that any body-fixed vector $\mathbf{e}$ maps to $R\mathbf{e}$ in the lab frame.

The **rotation group** $SO(3)$ is the group of $3\times 3$ orthogonal matrices with determinant $+1$:
$$SO(3) = \{R\in M_{3\times 3}(\mathbb{R})\ |\ R^TR = I,\ \det R = +1\}$$

**Euler angles** $(\phi, \theta, \psi)$: Any rotation can be parametrized by three Euler angles — a composition of three elementary rotations. The standard ZXZ or ZYZ convention:
$$R = R_z(\phi)R_x(\theta)R_z(\psi)$$

Euler angles have a coordinate singularity (gimbal lock) at $\theta = 0$.

**The Lie algebra** $\mathfrak{so}(3)$: The tangent space at the identity in $SO(3)$ consists of antisymmetric matrices $\omega^{ij} = -\omega^{ji}$, which can be identified with vectors via $\omega_k = \frac{1}{2}\varepsilon_{kij}\omega^{ij}$. The angular velocity vector $\boldsymbol{\omega}$ generates the rotation:
$$\dot{R} = \tilde\omega R$$

where $\tilde\omega$ is the $3\times 3$ antisymmetric matrix with $\tilde\omega\mathbf{v} = \boldsymbol{\omega}\times\mathbf{v}$.

---

## The Inertia Tensor

For a rigid body rotating about a fixed point with angular velocity $\boldsymbol{\omega}$, the kinetic energy is:
$$T = \frac{1}{2}I_{ij}\omega^i\omega^j$$

where the **inertia tensor** is:
$$I_{ij} = \int_{\rm body}\rho(\mathbf{r})\left(\delta_{ij}r^2 - r_ir_j\right)d^3r = \sum_\alpha m_\alpha(\delta_{ij}|\mathbf{r}_\alpha|^2 - r_{\alpha i}r_{\alpha j})$$

The inertia tensor is a symmetric $(0,2)$-tensor. It can be diagonalized by choosing the **principal axes** — the eigenvectors of $I_{ij}$:
$$I = \begin{pmatrix}I_1 & & \\ & I_2 & \\ & & I_3\end{pmatrix}$$

The eigenvalues $I_1, I_2, I_3$ are the **principal moments of inertia**.

**Angular momentum**: $L^i = I^{ij}\omega_j$. In principal axes: $\mathbf{L} = (I_1\omega_1, I_2\omega_2, I_3\omega_3)$.

**Parallel axis theorem**: If $I_{\rm cm}$ is the inertia tensor about the center of mass, then about a point displaced by $\mathbf{d}$:
$$I_{ij} = I_{{\rm cm},ij} + M(d^2\delta_{ij} - d_id_j)$$

---

## Euler's Equations

In the body frame (rotating with the body), Newton's second law for angular momentum becomes:

$$I_1\dot\omega_1 - (I_2-I_3)\omega_2\omega_3 = N_1$$
$$I_2\dot\omega_2 - (I_3-I_1)\omega_3\omega_1 = N_2$$
$$I_3\dot\omega_3 - (I_1-I_2)\omega_1\omega_2 = N_3$$

where $N_i$ are the torque components in the body frame. These are **Euler's equations** for rigid body rotation.

**Torque-free motion** ($N_i = 0$): For a symmetric top ($I_1 = I_2 \neq I_3$), the equations reduce to:
$$\dot\omega_3 = 0, \quad \dot\omega_1 = \frac{I_2-I_3}{I_1}\omega_3\omega_2 \equiv \Omega\omega_2, \quad \dot\omega_2 = -\Omega\omega_1$$

Solution: $\omega_1 + i\omega_2 = A e^{i\Omega t}$ — precession at angular frequency $\Omega = (I_3-I_1)\omega_3/I_1$.

**Tennis racket theorem** (intermediate axis theorem): Torque-free rotation about the smallest and largest principal axes is stable; rotation about the intermediate axis is unstable. This can be demonstrated with any asymmetric object (a book, a tennis racket): thrown with a spin about the intermediate axis, it will tumble. This was famously demonstrated on the International Space Station.

---

## The Heavy Top

A symmetric top ($I_1 = I_2$) with one fixed point in a gravitational field has a particularly rich dynamics.

**Three conserved quantities**: 
1. $E = \frac{1}{2}(I_1(\omega_1^2+\omega_2^2) + I_3\omega_3^2) + MgR\cos\theta$ (energy)
2. $L_z = (I_1\sin^2\theta + I_3\cos^2\theta)\dot\phi + I_3\omega_3\cos\theta$ (angular momentum about vertical axis — space-fixed $z$)
3. $\omega_3 = $ const (angular velocity about body symmetry axis — since $\phi$ is cyclic in Euler angle Lagrangian)

With these three integrals, the motion reduces to a one-dimensional problem for $\theta(t)$ — it can be integrated in terms of elliptic functions.

**Steady precession**: For fast spinning ($\omega_3$ large), the top precesses steadily at:
$$\dot\phi = \frac{MgR}{I_3\omega_3}$$

The precession rate is inversely proportional to spin rate — a faster spin means slower precession (gyroscope effect).

**Earth as a top**: Earth's precession of the equinoxes ($\approx 26{,}000$ year period) arises from the torque of the Moon and Sun on Earth's equatorial bulge — exactly the heavy top problem at planetary scale.

---

## From Particles to Fields: The Continuum Limit

A continuous medium (string, fluid, elastic solid) can be treated as the continuum limit of a discrete system. Let $q_i$ be the displacement of the $i$-th particle; in the continuum limit, $q_i\to\phi(x,t)$ — a field.

**The Lagrangian density**: For a 1D elastic string with linear mass density $\mu$ and tension $T$:
$$L = \int\mathcal{L}(\phi, \partial_t\phi, \partial_x\phi)\,dx$$

where:
$$\mathcal{L} = \frac{\mu}{2}(\partial_t\phi)^2 - \frac{T}{2}(\partial_x\phi)^2$$

**Euler-Lagrange equation for a field**:
$$\frac{\partial\mathcal{L}}{\partial\phi} - \partial_\mu\frac{\partial\mathcal{L}}{\partial(\partial_\mu\phi)} = 0$$

For the string:
$$\mu\partial_t^2\phi - T\partial_x^2\phi = 0 \implies \partial_t^2\phi - v^2\partial_x^2\phi = 0$$

where $v = \sqrt{T/\mu}$ is the wave speed. This is the wave equation.

**In 3+1 dimensions**: The Lagrangian density $\mathcal{L}(\phi, \partial_\mu\phi)$ (where $\partial_\mu = (\partial_t/c, \nabla)$) gives:
$$\partial_\mu\frac{\partial\mathcal{L}}{\partial(\partial_\mu\phi)} - \frac{\partial\mathcal{L}}{\partial\phi} = 0$$

**Example**: The Klein-Gordon field $\mathcal{L} = \frac{1}{2}(\partial_\mu\phi)^2 - \frac{1}{2}m^2\phi^2$ gives $(\Box - m^2)\phi = 0$.

**The Einstein-Hilbert action** (previewing Unit IX):
$$S_{\rm EH} = \frac{c^4}{16\pi G}\int R\sqrt{-g}\,d^4x$$

The Lagrangian density is $\mathcal{L} = \sqrt{-g}R$; the field is the metric $g_{\mu\nu}$. The Euler-Lagrange equations are Einstein's field equations: $G_{\mu\nu} = 0$ (in vacuum). This is the same variational structure as the string, but the "field" is the geometry of spacetime itself.

---

## Fluid Mechanics: Euler and Navier-Stokes

**Perfect fluid** (inviscid, incompressible): Lagrangian density in terms of the velocity field $\mathbf{v}(\mathbf{r},t)$, density $\rho$, pressure $P$:
$$\mathcal{L} = \rho\left(\frac{1}{2}v^2 - w\right)$$

where $w$ is the specific internal energy. The Euler-Lagrange equations give the **Euler equation**:
$$\rho\left(\partial_t\mathbf{v} + (\mathbf{v}\cdot\nabla)\mathbf{v}\right) = -\nabla P$$

(Newton's second law for a fluid element: pressure gradient = acceleration.)

Combined with mass conservation $\partial_t\rho + \nabla\cdot(\rho\mathbf{v}) = 0$ (continuity equation), these describe perfect fluid flow.

**Viscous fluid** (Navier-Stokes):
$$\rho(\partial_t\mathbf{v} + (\mathbf{v}\cdot\nabla)\mathbf{v}) = -\nabla P + \eta\nabla^2\mathbf{v} + (\zeta+\eta/3)\nabla(\nabla\cdot\mathbf{v})$$

where $\eta$ is the dynamic viscosity and $\zeta$ is the bulk viscosity.

**Reynolds number**: $Re = \rho vL/\eta$ — the dimensionless ratio of inertial to viscous forces. For $Re\ll 1$: viscous flow (laminar). For $Re\gg 1$: inertia dominates, flow becomes turbulent. Whether the Navier-Stokes equations always have smooth solutions for $Re\gg 1$ in 3D is one of the Millennium Prize Problems.

---

## Stress-Energy Tensor (Preview)

The field-theoretic formulation naturally introduces the **stress-energy tensor** (or energy-momentum tensor):
$$T^{\mu\nu} = \frac{\partial\mathcal{L}}{\partial(\partial_\mu\phi)}\partial^\nu\phi - \eta^{\mu\nu}\mathcal{L}$$

This is the Noether current for spacetime translations — the conserved tensor corresponding to the symmetry of the action under $x^\mu\to x^\mu + \varepsilon^\mu$.

- $T^{00}$: energy density
- $T^{0i}$: energy flux (= momentum density)
- $T^{ij}$: momentum flux (stress tensor)

The conservation law: $\partial_\mu T^{\mu\nu} = 0$ encodes both energy conservation ($\nu = 0$) and momentum conservation ($\nu = i$) in a single covariant equation.

In GR, the stress-energy tensor $T_{\mu\nu}$ is the source of gravity in Einstein's field equations $G_{\mu\nu} = 8\pi G T_{\mu\nu}/c^4$. The fluid mechanics concepts of energy density, pressure, and stress all appear on the right-hand side of Einstein's equations.

---

## Important Concepts

- **Rigid body**: 6 DOF (3 translational + 3 rotational); orientation described by $SO(3)$
- **Inertia tensor** $I_{ij}$: Symmetric $(0,2)$-tensor encoding mass distribution; $T_{\rm rot} = \frac{1}{2}I_{ij}\omega^i\omega^j$
- **Principal axes**: Eigenvectors of $I_{ij}$; diagonal representation $I = \text{diag}(I_1, I_2, I_3)$
- **Euler's equations**: Equations of motion in body frame; describe torque-free motion, precession
- **Heavy top**: Integrable system with three conservation laws; precession, nutation, spin
- **Lagrangian density** $\mathcal{L}$: Continuum generalization of $L$; $L = \int\mathcal{L}\,d^3x$
- **Field Euler-Lagrange equations**: $\partial_\mu(\partial\mathcal{L}/\partial(\partial_\mu\phi)) = \partial\mathcal{L}/\partial\phi$
- **Stress-energy tensor** $T^{\mu\nu}$: Noether current for spacetime translations; source of gravity in GR
- **Wave equation**: Emerges from string Lagrangian; prototype for all field equations
- **Navier-Stokes**: Viscous fluid flow; turbulence transition at high Reynolds number

---

## Important Figures

**Leonhard Euler** (1707–1783): Derived the equations for rigid body rotation (Euler's equations); also Euler angles, the equations of fluid dynamics, and vast contributions throughout mathematics.

**Joseph-Louis Lagrange** (1736–1813): Treated rigid body dynamics with generalized coordinates; heavy top problem.

**Sofia Kovalevskaya** (1850–1891): Found the third integrable case of the heavy top (after Euler and Lagrange); remarkable work at a time when women were excluded from academic positions.

**Claude-Louis Navier** (1785–1836) and **George Gabriel Stokes** (1819–1903): Derived the viscous fluid equations; Stokes also proved his theorem connecting surface and line integrals.

**David Hilbert** (1862–1943): Derived the Einstein field equations from the action principle (Einstein-Hilbert action) simultaneously with Einstein (November 1915).

---

## Further Reading

**Textbooks**
- Goldstein, H., Poole, C., & Safko, J. (2002). *Classical Mechanics* (3rd ed.). Addison-Wesley. — Chapters 4–5 on rigid body rotation.
- Landau, L.D. & Lifshitz, E.M. (1976). *Mechanics*. Butterworth-Heinemann. — Chapters 9–10 on rigid body dynamics; beautiful and concise.
- Landau, L.D. & Lifshitz, E.M. (1987). *Fluid Mechanics* (2nd ed.). Butterworth-Heinemann. — The classic reference for continuum mechanics.
- Marsden, J.E. & Ratiu, T.S. (1999). *Introduction to Mechanics and Symmetry* (2nd ed.). Springer. — Geometric mechanics; rigorous treatment of rigid body as a Lie group problem.

---

## Exercises

**17.1.** *Inertia tensor.*

(a) Compute the inertia tensor of a uniform solid sphere of mass $M$ and radius $R$ about its center. Show $I_{ij} = \frac{2}{5}MR^2\delta_{ij}$.

(b) Compute the inertia tensor of a thin uniform rod of mass $M$ and length $L$ about its center, with the rod along the $x$-axis.

(c) Using the parallel axis theorem, find the moment of inertia of the rod in (b) about one end.

---

**17.2.** *The Tennis Racket theorem.*

A torque-free rigid body has principal moments $I_1 < I_2 < I_3$.

(a) Write Euler's equations for $\dot\omega_1, \dot\omega_2, \dot\omega_3$.

(b) Consider small perturbations about rotation along the $\hat{e}_1$ axis: $\boldsymbol{\omega} = \Omega\hat{e}_1 + (\varepsilon_2, \varepsilon_3)$ with $\varepsilon_i\ll\Omega$. Linearize Euler's equations. Show the perturbations $\varepsilon_2$, $\varepsilon_3$ satisfy a coupled oscillator equation and find the stability condition: stable iff $(I_2-I_1)(I_3-I_1) > 0$, i.e., iff $I_1$ is the smallest or largest moment.

(c) Repeat for rotation about $\hat{e}_2$. Show the intermediate axis is unstable.

---

**17.3.** *Field Euler-Lagrange equation.*

(a) For the Lagrangian density $\mathcal{L} = \frac{1}{2}(\partial_t\phi)^2/c^2 - \frac{1}{2}(\nabla\phi)^2 - V(\phi)$ in 3+1D, derive the field equation using $\partial_\mu(\partial\mathcal{L}/\partial(\partial_\mu\phi)) = \partial\mathcal{L}/\partial\phi$.

(b) For $V(\phi) = \frac{1}{2}m^2\phi^2$: show the field equation is the Klein-Gordon equation $(\Box - m^2)\phi = 0$.

(c) Compute the stress-energy tensor $T^{\mu\nu} = \partial^\mu\phi\,\partial^\nu\phi - \eta^{\mu\nu}\mathcal{L}$ and verify: (i) $T^{00} = \frac{1}{2}(\dot\phi/c)^2 + \frac{1}{2}(\nabla\phi)^2 + V(\phi)$ (energy density); (ii) $\partial_\mu T^{\mu\nu} = 0$ when $\phi$ satisfies the Klein-Gordon equation.

---

**Thought Experiment T17.1.** *Gravity as a field theory.*

The Einstein-Hilbert action $S = \int\sqrt{-g}R\,d^4x/(16\pi G)$ has the same structure as the actions we derived for elastic strings and fields: it is an integral of a Lagrangian density over spacetime, and Einstein's equations are its Euler-Lagrange equations.

But there is a crucial difference: the "field" here is the metric $g_{\mu\nu}$ — the very geometry of spacetime. The domain of integration itself depends on the field (through $\sqrt{-g}$). 

What does it mean to vary a metric? When you vary $g_{\mu\nu}\to g_{\mu\nu} + \delta g_{\mu\nu}$, you are changing the shape of spacetime. Is this analogous to varying a material field, or is it fundamentally different? Does the action principle for gravity "make sense" in the same way it does for a string?

Consider also: in Newtonian gravity, the gravitational potential $\Phi$ satisfies $\nabla^2\Phi = 4\pi G\rho$ — a field equation. In GR, the metric $g_{\mu\nu}$ satisfies Einstein's equations. Both are field theories. What is the essential difference between Newtonian gravity as a field theory and GR as a field theory?
