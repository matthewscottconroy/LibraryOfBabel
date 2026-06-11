# Unit V Problems: Classical Mechanics

*Lagrangian mechanics, Hamiltonian mechanics, Noether's theorem, and orbits — the classical foundation for the geodesic equation and conservation laws in GR.*

**Difficulty:** ★ Introductory, ★★ Intermediate, ★★★ Advanced

---

## Part 1: Newtonian and Lagrangian Mechanics

**Problem 1.1** ★
A particle of mass $m$ moves in one dimension with potential $V(x) = \frac{1}{2}kx^2$ (harmonic oscillator).

(a) Write the Lagrangian $L = T - V = \frac{1}{2}m\dot{x}^2 - \frac{1}{2}kx^2$.
(b) Derive the equation of motion from the Euler-Lagrange equation $\frac{d}{dt}\frac{\partial L}{\partial\dot{x}} - \frac{\partial L}{\partial x} = 0$.
(c) What is the period of oscillation $T = 2\pi\sqrt{m/k}$?
(d) The energy $E = T + V$. Show that $dE/dt = 0$ (energy conservation) using the equation of motion.

**Problem 1.2** ★
Constraints and generalized coordinates: a simple pendulum of length $\ell$ and mass $m$ in 2D.

(a) The constraint is $x^2 + y^2 = \ell^2$. Use the angle $\theta$ as the generalized coordinate. Express $T$ and $V$ in terms of $\theta$.
(b) Derive the equation of motion. For small oscillations ($\sin\theta\approx\theta$): what is the oscillation frequency?
(c) How many degrees of freedom does this system have? How many constraints? Apply the formula $N_\text{dof} = N_\text{coordinates} - N_\text{constraints}$.

**Problem 1.3** ★★
The Lagrangian for a particle of mass $m$ in a central potential $V(r)$ in 3D, using spherical coordinates:

$$L = \frac{m}{2}(\dot{r}^2 + r^2\dot{\theta}^2 + r^2\sin^2\theta\,\dot{\phi}^2) - V(r)$$

(a) Write the three Euler-Lagrange equations.
(b) The $\phi$ coordinate is cyclic (absent from $L$ explicitly). What conserved quantity does this give?
(c) Use the $\theta$ equation to show that if the initial conditions are $\theta = \pi/2$, $\dot{\theta} = 0$: the motion remains in the plane $\theta = \pi/2$ (equatorial plane). This is the origin of "confinement to a plane" in 2-body gravitational orbits.
(d) The effective potential for the radial motion: $V_\text{eff}(r) = V(r) + L^2/(2mr^2)$. Sketch $V_\text{eff}(r)$ for $V(r) = -GM m/r$ (gravity) and identify circular orbit conditions.

**Problem 1.4** ★★
Noether's theorem: if the Lagrangian is invariant under a continuous transformation $q^i\to q^i + \epsilon \xi^i(q,\dot{q},t)$, then the quantity $Q = \frac{\partial L}{\partial\dot{q}^i}\xi^i$ is conserved.

(a) For spatial translation $x^i\to x^i + \epsilon\hat{n}^i$ (where $\hat{n}$ is a unit vector): what is the conserved quantity? What symmetry corresponds to conservation of linear momentum?

(b) For rotation about the $z$-axis: $\phi\to\phi+\epsilon$. The corresponding conserved quantity is $L_z = \frac{\partial L}{\partial\dot{\phi}}$. For the central force problem: express $L_z$ in Cartesian coordinates.

(c) For time translation: $t\to t+\epsilon$, $L\to L$ (no explicit time dependence). The conserved quantity is the **energy** $H = \dot{q}^i\frac{\partial L}{\partial\dot{q}^i} - L$. Derive this from the requirement $dL/dt = 0$ (for $\partial L/\partial t = 0$).

(d) In GR, Killing vectors $\xi^\mu$ satisfying $\nabla_{(\mu}\xi_{\nu)} = 0$ are the analogues of continuous symmetries. For a Killing vector $\xi^\mu$ and a geodesic with tangent $u^\mu$: show that $\xi_\mu u^\mu$ is constant along the geodesic. This is the GR version of Noether's theorem.

---

## Part 2: Hamiltonian Mechanics

**Problem 2.1** ★★
The Hamiltonian $H(q,p,t) = p_i\dot{q}^i - L$ (Legendre transform), where $p_i = \partial L/\partial\dot{q}^i$ are the conjugate momenta.

(a) For $L = \frac{1}{2}m\dot{x}^2 - V(x)$: find $p$, then $H(x,p)$.
(b) Hamilton's equations: $\dot{q}^i = \partial H/\partial p_i$, $\dot{p}_i = -\partial H/\partial q^i$. Verify they reproduce Newton's second law.
(c) For the harmonic oscillator $H = p^2/(2m) + kx^2/2$: solve Hamilton's equations and find the trajectory in phase space.

**Problem 2.2** ★★★
The Poisson bracket: $\{f,g\} = \frac{\partial f}{\partial q^i}\frac{\partial g}{\partial p_i} - \frac{\partial f}{\partial p_i}\frac{\partial g}{\partial q^i}$.

(a) Verify $\{q^i, p_j\} = \delta^i_{\ j}$, $\{q^i,q^j\} = 0$, $\{p_i,p_j\} = 0$ (canonical Poisson brackets).
(b) Hamilton's equations: $\dot{f} = \{f, H\}$ for any observable $f(q,p)$. Verify for $f = q^i$ and $f = p_i$.
(c) The Poisson bracket satisfies the Jacobi identity $\{f,\{g,h\}\} + \{g,\{h,f\}\} + \{h,\{f,g\}\} = 0$. This makes the space of observables a **Lie algebra**. In quantum mechanics, the Poisson bracket is replaced by $(1/i\hbar)$ times the commutator. What does this analogy say about canonical quantization?

---

## Part 3: Orbits and Kepler's Laws

**Problem 3.1** ★★
Kepler's laws from the central force Lagrangian:

(a) Second law (equal areas in equal times): Show that $dA/dt = L/(2m) = \text{const}$ using $dA = \frac{1}{2}r^2 d\phi$.

(b) First law: For the $1/r^2$ gravitational force $V = -GMm/r$, the orbit satisfies $u'' + u = GMm^2/L^2$ where $u = 1/r$ and primes are $d/d\phi$. Solve for $u(\phi)$ and identify the result as a conic section.

(c) Third law: $T^2 \propto a^3$ (orbital period squared proportional to semi-major axis cubed). Derive from the orbital parameters of an ellipse.

**Problem 3.2** ★★★
Relativistic corrections: the GR geodesic equation in the Schwarzschild metric gives an effective potential:

$$V_\text{eff}(r) = -\frac{GM}{r} + \frac{L^2}{2m^2r^2} - \frac{GML^2}{m^2c^2r^3}$$

The last term is the relativistic correction.

(a) Find the location of circular orbits by setting $V_\text{eff}' = 0$. Show there are two solutions and identify the **innermost stable circular orbit (ISCO)** at $r = 6GM/c^2$.

(b) The GR correction to the orbit equation: $u'' + u = GMm^2/L^2 + 3GMu^2/c^2$. Treating the last term as a small perturbation, show that circular orbits precess by:
$$\Delta\phi = \frac{6\pi GM}{c^2 a(1-e^2)}$$
per orbit (where $a$ is semi-major axis, $e$ eccentricity). This is the famous 43" per century for Mercury.

(c) For Mercury ($a = 5.79\times10^{10}$ m, $e = 0.206$, orbital period $T = 88$ days): verify the $43''/\text{century}$ prediction numerically.

**Problem 3.3** ★★
Action-angle variables: a classically integrable system with one degree of freedom has action variable $J = \oint p\,dq/(2\pi)$ (integral over one cycle in phase space) and angle variable $\theta$ (conjugate to $J$).

(a) For the harmonic oscillator with energy $E$: the phase space orbit is an ellipse. Compute $J$ as a function of $E$.

(b) The frequency of oscillation $\omega = \partial H/\partial J$. Verify this gives the correct $\omega = \sqrt{k/m}$.

(c) The action is an adiabatic invariant: if the spring constant $k(t)$ changes slowly (over many oscillation periods), $J$ is approximately constant but the energy $E = \omega J$ changes as $\omega$ changes. This is directly analogous to the quantum adiabatic theorem. State the analogy precisely.
