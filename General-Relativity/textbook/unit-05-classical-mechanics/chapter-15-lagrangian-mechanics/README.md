# Chapter 15: Lagrangian Mechanics

---

## Chapter Introduction

Every course in classical mechanics begins with Newton: $F = ma$. Forces act on masses; the equations of motion follow from identifying the forces and applying the second law. This approach is direct, intuitive, and works beautifully for simple systems — a single particle moving in a potential, a spring, a pendulum. But it struggles with constraints. If a bead must stay on a wire, you must track the normal force of the wire on the bead, even though that force does no work and ultimately cancels out. In a system of ten coupled pendula, Newton's approach demands tracking 30 equations of motion plus all the constraint forces.

The Lagrangian approach, developed by Joseph-Louis Lagrange in 1788, makes constraints transparent and reduces any mechanical system to a set of equations of motion for the *independent* degrees of freedom. The key idea: choose coordinates that automatically satisfy the constraints (generalized coordinates), and write the equations of motion in terms of a single scalar function — the **Lagrangian** $L = T - V$. The equations of motion follow from an extremization principle.

The real payoff comes when you realize that the Lagrangian approach is not just a computational trick — it is a different way of thinking about physics. The **action principle** (Hamilton's principle) says: of all the paths a system could take between two configurations, it takes the one that extremizes the action $S = \int L\,dt$. This principle is not derived from Newton's laws — it is a different, deeper formulation that generalizes far beyond classical mechanics. The Lagrangian approach underlies quantum mechanics (path integrals), field theory (Lagrangian density), and general relativity (the Hilbert action).

---

## Generalized Coordinates and Constraints

Consider a system of $N$ particles in 3D space: $3N$ coordinates. Suppose there are $k$ holonomic constraints — constraints of the form $f_j(\mathbf{r}_1,\ldots,\mathbf{r}_N, t) = 0$. Then the system has $n = 3N - k$ **degrees of freedom**.

**Generalized coordinates** $q^1, \ldots, q^n$ are any $n$ independent parameters that completely specify the configuration of the system subject to the constraints. The Cartesian coordinates are expressible as $\mathbf{r}_i = \mathbf{r}_i(q^1,\ldots,q^n, t)$.

**Examples**:
- Pendulum in a plane: 2 Cartesian coordinates $(x,y)$ with constraint $x^2+y^2 = L^2$. One degree of freedom: angle $\theta$.
- Double pendulum: 4 Cartesian coordinates, 2 constraints ($r_1 = L_1$, $r_2 = L_2$). Two degrees of freedom: angles $\theta_1$, $\theta_2$.
- Rigid body: 6 degrees of freedom (3 translational + 3 rotational).
- Bead on a wire: 3 Cartesian coordinates, 2 constraints (wire curve). One degree of freedom: arc length $s$.

**Virtual displacement** $\delta q^i$: An infinitesimal change in the generalized coordinates, consistent with constraints, at a fixed time. Constraint forces do no work under virtual displacements (d'Alembert's principle).

---

## The Euler-Lagrange Equations

The **Lagrangian** of a mechanical system is:
$$L(q^i, \dot{q}^i, t) = T - V$$

where $T$ is the kinetic energy and $V$ is the potential energy, both expressed in generalized coordinates.

The **action** is:
$$S[q] = \int_{t_1}^{t_2}L(q^i(t), \dot{q}^i(t), t)\,dt$$

**Hamilton's principle** (principle of stationary action): The actual trajectory $q^i(t)$ of the system extremizes the action, among all paths with fixed endpoints $q^i(t_1) = q^i_1$ and $q^i(t_2) = q^i_2$.

**Derivation**: Consider variations $q^i(t) \to q^i(t) + \varepsilon\eta^i(t)$ with $\eta^i(t_{1,2}) = 0$. The first variation:
$$\delta S = \int_{t_1}^{t_2}\left(\frac{\partial L}{\partial q^i}\eta^i + \frac{\partial L}{\partial\dot{q}^i}\dot{\eta}^i\right)dt = \int_{t_1}^{t_2}\left(\frac{\partial L}{\partial q^i} - \frac{d}{dt}\frac{\partial L}{\partial\dot{q}^i}\right)\eta^i\,dt + \left[\frac{\partial L}{\partial\dot{q}^i}\eta^i\right]_{t_1}^{t_2}$$

The boundary term vanishes. For $\delta S = 0$ for all $\eta^i$, the integrand must vanish:

$$\boxed{\frac{d}{dt}\frac{\partial L}{\partial\dot{q}^i} - \frac{\partial L}{\partial q^i} = 0}$$

These are the **Euler-Lagrange equations** — the equations of motion.

---

## Generalized Momenta and Forces

The **generalized momentum** conjugate to $q^i$:
$$p_i = \frac{\partial L}{\partial\dot{q}^i}$$

The **generalized force** conjugate to $q^i$:
$$Q_i = \frac{\partial L}{\partial q^i}$$

The Euler-Lagrange equation is then $\dot{p}_i = Q_i$ — Newton's second law in generalized coordinates.

If $L$ does not depend on $q^i$ (the coordinate is **cyclic** or **ignorable**), then $Q_i = 0$ and $p_i = \text{const}$ — a conservation law. This is Lagrangian mechanics' first glimpse of a deep connection between symmetry and conservation laws.

**Coordinate independence**: The Euler-Lagrange equations hold in any set of generalized coordinates. If $(q^i)$ and $(\tilde{q}^j)$ are two sets of generalized coordinates related by $q^i = q^i(\tilde{q}^j)$, both give the same physical equations of motion.

---

## Examples

**Simple pendulum**: $T = \frac{1}{2}mL^2\dot\theta^2$, $V = -mgL\cos\theta$:
$$L = \frac{1}{2}mL^2\dot\theta^2 + mgL\cos\theta$$
$$\frac{d}{dt}(mL^2\dot\theta) + mgL\sin\theta = 0 \implies \ddot\theta + \frac{g}{L}\sin\theta = 0$$

**Atwood machine**: Two masses $m_1$, $m_2$ connected by a massless string over a pulley. One degree of freedom $q = x_1$ (position of $m_1$); $x_2 = \text{const} - x_1$. $T = \frac{1}{2}(m_1+m_2)\dot{q}^2$, $V = -m_1 gq - m_2 g(\text{const}-q)$:
$$\ddot{q} = \frac{(m_1-m_2)g}{m_1+m_2}$$

No need to track the tension in the string — it is eliminated by the choice of generalized coordinate.

**Central force problem**: In polar coordinates $T = \frac{1}{2}m(\dot{r}^2 + r^2\dot\phi^2)$, $V = V(r)$:
$$L = \frac{1}{2}m(\dot{r}^2 + r^2\dot\phi^2) - V(r)$$

$\phi$ is cyclic: $p_\phi = mr^2\dot\phi = \ell$ (angular momentum conserved).

For $r$: $m\ddot{r} - mr\dot\phi^2 + V'(r) = 0$, i.e., $m\ddot{r} = \ell^2/(mr^3) - V'(r)$ — the central force equation with centrifugal term.

---

## Noether's Theorem

**Noether's theorem** (1915): Every continuous symmetry of the action corresponds to a conserved quantity.

**Derivation**: Suppose the action is invariant under a one-parameter family of transformations $q^i\to q^i + \varepsilon K^i(q, t)$ (for all paths, not just solutions). Then:
$$\frac{d}{dt}\left(\frac{\partial L}{\partial\dot{q}^i}K^i\right) = 0$$

The conserved **Noether charge** is $Q = p_i K^i = \text{const}$ along any solution of the equations of motion.

**Examples**:
- **Time translation** ($t\to t+\varepsilon$, $L$ explicitly independent of $t$): Conserved quantity is $H = p_i\dot{q}^i - L$ (the Hamiltonian, equal to total energy for standard kinetic terms).
- **Space translation** ($q^i\to q^i + \varepsilon n^i$ for direction $\hat{n}$, $L$ independent of that translation): Conserved quantity is $p_i n^i$ — the component of momentum in direction $\hat{n}$.
- **Rotation** (for rotationally symmetric $L$): Conserved quantity is the component of angular momentum $\mathbf{L} = \mathbf{r}\times\mathbf{p}$ along the rotation axis.

Noether's theorem is profound: it tells us that the conservation laws of physics are consequences of the symmetries of the laws of physics. Energy is conserved because the laws of physics don't change with time. Momentum is conserved because the laws of physics don't change with position. Angular momentum is conserved because the laws of physics don't depend on direction.

This connection — symmetry $\leftrightarrow$ conservation law — generalizes to field theory, quantum mechanics, and GR. In GR, the absence of a global time-translation symmetry in a general curved spacetime is related to the non-conservation of energy for gravitational fields.

---

## Non-Conservative Forces and Generalized Potentials

For non-conservative forces (friction, velocity-dependent forces), the Euler-Lagrange framework extends:

**Generalized potential**: If the generalized force can be written as $Q_i = -\partial U/\partial q^i + d(\partial U/\partial\dot{q}^i)/dt$, define $L = T - U$. The Euler-Lagrange equations still hold.

**Example — Lorentz force**: The electromagnetic force $\mathbf{F} = q(\mathbf{E} + \mathbf{v}\times\mathbf{B})$ can be derived from $L = T - q\phi + q\mathbf{A}\cdot\mathbf{v}$ where $\phi, \mathbf{A}$ are the scalar and vector potentials.

**Rayleigh dissipation function**: For linear dissipative forces $Q_i = -\partial\mathcal{F}/\partial\dot{q}^i$, the modified equations are:
$$\frac{d}{dt}\frac{\partial L}{\partial\dot{q}^i} - \frac{\partial L}{\partial q^i} + \frac{\partial\mathcal{F}}{\partial\dot{q}^i} = 0$$

---

## Important Concepts

- **Generalized coordinates**: Independent coordinates satisfying constraints; $n = 3N - k$ for $N$ particles with $k$ holonomic constraints
- **Lagrangian**: $L = T - V$; a scalar function of generalized coordinates, velocities, and time
- **Action**: $S[q] = \int L\,dt$; the fundamental object of Hamilton's principle
- **Hamilton's principle**: Physical trajectories extremize the action ($\delta S = 0$)
- **Euler-Lagrange equations**: $d/dt(\partial L/\partial\dot{q}^i) - \partial L/\partial q^i = 0$; the equations of motion
- **Cyclic coordinate**: $\partial L/\partial q^i = 0$ implies $p_i = \text{const}$ — a conservation law
- **Generalized momentum**: $p_i = \partial L/\partial\dot{q}^i$; conjugate to $q^i$
- **Noether's theorem**: Continuous symmetry $\Rightarrow$ conservation law; the deepest insight of analytical mechanics
- **Coordinate independence**: Euler-Lagrange equations hold in any generalized coordinates

---

## Important Figures

**Joseph-Louis Lagrange** (1736–1813): Formulated analytical mechanics in *Mécanique Analytique* (1788); replaced geometric/force-based methods with algebraic, coordinate-based analysis.

**William Rowan Hamilton** (1805–1865): Reformulated Lagrangian mechanics in the Hamiltonian framework (1833–1834); introduced the action principle; predicted conical refraction. 

**Emmy Noether** (1882–1935): Proved the theorem connecting symmetries and conservation laws (1915); arguably the most important mathematical theorem in theoretical physics.

**Carl Gustav Jacob Jacobi** (1804–1851): Developed Hamilton-Jacobi theory; crucial contribution to the solvability of the action principle.

---

## Further Reading

**Primary Sources**
- Lagrange, J.L. (1788). *Mécanique Analytique*. Paris. — The founding text; in French but available in English translation.
- Noether, E. (1918). "Invariante Variationsprobleme." *Nachrichten der Ges. der Wiss. zu Göttingen*, 235–257. [English translation in Transport Theory and Statistical Physics, 1971]

**Textbooks**
- Goldstein, H., Poole, C., & Safko, J. (2002). *Classical Mechanics* (3rd ed.). Addison-Wesley. — The standard graduate reference; Chapters 1–2.
- Landau, L.D. & Lifshitz, E.M. (1976). *Mechanics* (3rd ed.). Butterworth-Heinemann. — Elegant and concise; action principle from the start.
- Taylor, J.R. (2005). *Classical Mechanics*. University Science Books. — Excellent undergraduate text; Chapter 7 on Lagrangian mechanics.

---

## Exercises

**15.1.** *Lagrangian setup and Euler-Lagrange equations.*

(a) A bead of mass $m$ slides without friction on a wire bent into the shape of a parabola $z = ax^2$ in a gravitational field $g$ (vertical). Choose $x$ as the generalized coordinate, write the Lagrangian, and derive the equation of motion.

(b) A particle moves on the surface of a sphere of radius $R$ under gravity. Choose polar angles $(\theta, \phi)$ as generalized coordinates. Write the Lagrangian $L = T - V$ and derive the two Euler-Lagrange equations.

(c) For part (b): identify any cyclic coordinates and the corresponding conserved quantities.

---

**15.2.** *Noether's theorem applied.*

(a) A particle moves in the central potential $V(r) = \alpha/r$. Identify the symmetries of the action (time translation, rotation). State the corresponding conserved quantities.

(b) A system of two particles interacts via $V = V(|\mathbf{r}_1 - \mathbf{r}_2|)$ (depends only on relative position). Show that the total momentum $\mathbf{P} = m_1\dot{\mathbf{r}}_1 + m_2\dot{\mathbf{r}}_2$ is conserved by identifying the relevant symmetry.

(c) Consider the anisotropic harmonic oscillator: $L = \frac{1}{2}m(\dot{x}^2 + \dot{y}^2) - \frac{1}{2}m(\omega_x^2 x^2 + \omega_y^2 y^2)$ with $\omega_x\neq\omega_y$. Is angular momentum conserved? Explain via symmetry.

---

**15.3.** *Electromagnetic Lagrangian.*

The Lagrangian for a charged particle in an electromagnetic field is $L = \frac{1}{2}mv^2 - q\phi + q\mathbf{A}\cdot\mathbf{v}$.

(a) Compute the generalized momentum $p_x = \partial L/\partial\dot{x}$.

(b) Write the Euler-Lagrange equation for the $x$-component. Use $\mathbf{E} = -\nabla\phi - \partial_t\mathbf{A}$ and $\mathbf{B} = \nabla\times\mathbf{A}$ to show the result is $m\ddot{x} = q(E_x + (\dot{\mathbf{r}}\times\mathbf{B})_x)$.

(c) The gauge transformation $\phi\to\phi - \partial_t\chi$, $\mathbf{A}\to\mathbf{A} + \nabla\chi$ changes $L$ by $dF/dt$ where $F = q\chi$. Show this leaves the Euler-Lagrange equations unchanged (a total time derivative in $L$ doesn't affect the equations of motion).

---

**Thought Experiment T15.1.** *Why the action principle?*

Hamilton's principle says that physical systems follow paths that extremize the action. But why should nature care about an extremal principle? Newton's laws are local: the force at a point determines the acceleration at that point. The action principle is global: to find the path, you need to know the endpoints.

Feynman's path integral formulation of quantum mechanics offers an answer: quantum mechanically, the particle takes *all* paths, each weighted by $e^{iS/\hbar}$. In the classical limit $\hbar\to 0$, contributions from paths away from the stationary-action path cancel (destructive interference), while contributions near the extremal path add up (constructive interference). The classical trajectory is the one where all nearby quantum paths are in phase.

Does this explanation make you more comfortable with the action principle, or does it just push the mystery to the quantum level? Is the action principle a deep fact about nature, or a convenient calculational tool?
