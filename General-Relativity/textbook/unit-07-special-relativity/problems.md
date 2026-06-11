# Unit VII Problems: Special Relativity

*Lorentz transformations, 4-vectors, relativistic dynamics, spacetime diagrams, and the geometric structure of Minkowski space.*

**Difficulty:** ★ Introductory, ★★ Intermediate, ★★★ Advanced

---

## Part 1: Lorentz Transformations

**Problem 1.1** ★
The Lorentz transformation for a boost in the $x$-direction with velocity $v$ ($\beta = v/c$, $\gamma = 1/\sqrt{1-\beta^2}$):

$$ct' = \gamma(ct - \beta x), \quad x' = \gamma(x - \beta ct), \quad y' = y, \quad z' = z$$

(a) Verify that the spacetime interval $\Delta s^2 = -(c\Delta t)^2 + \Delta x^2 + \Delta y^2 + \Delta z^2$ is preserved.

(b) A clock at rest at $x = 0$ in $S$ ticks from $t = 0$ to $t = T$. In $S'$: what is the elapsed time $T'$? This is **time dilation**: $T' = \gamma T$.

(c) A rod of rest length $L_0$ lies along the $x$-axis in $S$. What is its length in $S'$? This is **length contraction**: $L = L_0/\gamma$.

(d) Two events occur at the same location in $S$ at times $t_1 = 0$ and $t_2 = T$. Can they be simultaneous in $S'$? (Simultaneity of events at different locations is frame-dependent; events at the same location with the same $t$ satisfy $c\,dt = 0$, so $dt' = \gamma(dt - \beta\,dx/c) = \gamma\,dt$ — not simultaneously zero unless $dt = 0$.)

**Problem 1.2** ★
Relativistic velocity addition: if a particle moves at velocity $u'$ in the $x'$-direction in frame $S'$, its velocity in frame $S$ is:

$$u = \frac{u' + v}{1 + u'v/c^2}$$

(a) Verify that if $u' = c$ (a photon in $S'$), then $u = c$ in $S$.
(b) Two spaceships travel toward each other, each at $0.8c$ relative to Earth. What is the relative velocity of one ship as measured by the other?
(c) Derive this formula from the Lorentz transformation by computing $dx/dt = (dx'/dt' + v)/(1 + v\,dx'/(c^2\,dt'))$.

**Problem 1.3** ★★
The Lorentz group: the set of all Lorentz transformations $\Lambda^\mu_{\ \nu}$ satisfying $\Lambda^\mu_{\ \alpha}\eta_{\mu\nu}\Lambda^\nu_{\ \beta} = \eta_{\alpha\beta}$ forms a group $O(3,1)$.

(a) Show that $\det\Lambda = \pm 1$.
(b) The proper orthochronous Lorentz group $SO^+(3,1)$ has $\det\Lambda = +1$ and $\Lambda^0_{\ 0}\geq 1$. Why is this the physically relevant subgroup?
(c) Boost in the $x$-direction by rapidity $\eta$ (where $\tanh\eta = v/c$):
$$\Lambda = \begin{pmatrix}\cosh\eta & -\sinh\eta & 0 & 0\\ -\sinh\eta & \cosh\eta & 0 & 0\\ 0 & 0 & 1 & 0\\ 0 & 0 & 0 & 1\end{pmatrix}$$
Show that boosts compose by adding rapidities: $\Lambda(\eta_1)\Lambda(\eta_2) = \Lambda(\eta_1+\eta_2)$. This is analogous to adding angles in rotation.

(d) Find the boost parameter $\eta$ such that $v = 0.99c$. What is $\gamma$?

---

## Part 2: Four-Vectors and Relativistic Dynamics

**Problem 2.1** ★★
4-velocity and 4-momentum:

(a) The 4-velocity $u^\mu = dx^\mu/d\tau$ where $\tau$ is proper time. Show $u^\mu u_\mu = -c^2$.
(b) In terms of coordinate velocity $\mathbf{v}$: $u^\mu = \gamma(c, \mathbf{v})$. Verify the normalization.
(c) The 4-momentum $p^\mu = mu^\mu = (E/c, \mathbf{p})$. Show that $p^\mu p_\mu = -m^2c^2$, equivalent to $E^2 = p^2c^2 + m^2c^4$.
(d) For a massless photon ($m = 0$): what does $p^\mu p_\mu = 0$ say about the relationship between energy and momentum?

**Problem 2.2** ★★
Relativistic energy and collisions:

(a) A proton ($m_p c^2 = 938$ MeV) traveling at $0.9c$ collides with a stationary proton. Is there enough energy in the CM frame to produce a proton-antiproton pair ($2m_p c^2$ additional rest mass)? [Use the invariant $s = -(p_1+p_2)^2c^2$ for the center-of-mass energy squared.]

(b) The threshold energy for $p + p\to p + p + p + \bar{p}$ (production of a proton-antiproton pair at rest in the CM frame): find the minimum lab-frame energy of the incident proton.

(c) Compton scattering: a photon of wavelength $\lambda$ scatters off an electron at rest. Derive the Compton formula $\lambda' - \lambda = (h/m_e c)(1 - \cos\theta)$ using 4-momentum conservation.

**Problem 2.3** ★★
Spacetime diagrams (Minkowski diagrams):

(a) Draw a spacetime diagram showing: the worldline of a stationary observer, the worldline of a uniformly moving observer, the light cone.

(b) Mark two events $A = (ct=0, x=0)$ and $B = (ct=3, x=2)$ (in units where $c = 1$). Are they timelike, spacelike, or null separated? Could an observer with $v < c$ be present at both events?

(c) Draw the worldline of a uniformly accelerated observer (hyperbola $x^2 - c^2t^2 = c^4/a^2$). This is **Rindler motion** — the worldline has constant proper acceleration $a$. Show that this observer never receives signals from the region $x < ct$ (the Rindler horizon).

(d) The twin paradox: Alice travels at $v = 0.8c$ to a star 4 light-years away and returns. Draw both worldlines on a spacetime diagram. Compute the proper time elapsed for Alice and for Bob (who stays home). Who ages less and why is there no paradox despite the "relative motion" argument?

---

## Part 3: Toward General Relativity

**Problem 3.1** ★★
The equivalence principle: Einstein's gedanken experiment — a uniformly accelerating elevator and a uniform gravitational field are locally indistinguishable.

(a) In a freely falling frame (an inertial frame in GR), Newton's law takes the form $m\ddot{\xi}^i = F^i$ where $\xi$ are local inertial coordinates. In a static gravitational field with Newtonian potential $\Phi$, the equations of motion are $\ddot{x}^i = -\partial\Phi/\partial x^i$. Make the coordinate change $\xi^i = x^i - \frac{1}{2}\partial_i\Phi(x_0)\cdot(t-t_0)^2 + \ldots$ and show that the equation of motion simplifies in the freely-falling frame.

(b) The gravitational redshift: a photon emitted at height $z = 0$ with frequency $\nu_0$ is received at height $z = h$. Using the equivalence principle (the receiver accelerates upward by $g$ during the photon's flight): derive $\Delta\nu/\nu = -gh/c^2$.

(c) In GR, the gravitational redshift $\Delta\nu/\nu = \Delta\Phi/c^2$ is exact for static spacetimes. For a clock on the surface of the Earth versus a clock in GPS orbit ($h = 20{,}200$ km): compute the fractional frequency difference due to gravity and velocity (both effects matter for GPS accuracy).

**Problem 3.2** ★★★
Curved spacetime — first steps: Consider the metric $ds^2 = -(1-r_s/r)c^2dt^2 + (1-r_s/r)^{-1}dr^2 + r^2d\Omega^2$ (Schwarzschild metric, where $r_s = 2GM/c^2$).

(a) The proper time elapsed for a stationary observer at $r = R$ is $d\tau = \sqrt{1-r_s/R}\,dt$. For $R = 2r_s$: what is the gravitational time dilation factor?

(b) A radially falling photon satisfies $ds^2 = 0$: $c\,dt = \pm dr/(1-r_s/r)$. Integrate to find $t(r)$ for an inward-falling photon. What happens as $r\to r_s$?

(c) Eddington-Finkelstein ingoing coordinate $v = ct + r + r_s\ln|r/r_s - 1|$: in $(v,r)$ coordinates, the photon equation is $dv = 0$ (ingoing) or $dv = 2dr/(1-r_s/r)$ (outgoing). Show that at $r = r_s$: outgoing photons have $dv/dr = \infty$ — they are "frozen" at the horizon in Schwarzschild coordinates.

(d) In Eddington-Finkelstein coordinates, the metric is nonsingular at $r = r_s$. A massive particle can cross $r = r_s$ in finite proper time. Qualitatively, what happens to its worldline after crossing?

**Problem 3.3** ★★★
The hole argument and diffeomorphism invariance: in GR, the gauge symmetry is the group of diffeomorphisms (smooth coordinate changes). Unlike in electromagnetism (where $A_\mu\to A_\mu+\partial_\mu\Lambda$ leaves $F_{\mu\nu}$ invariant), a diffeomorphism $\phi: M\to M$ can move points of the manifold while leaving the physics unchanged.

(a) Explain in words why the metric $g_{\mu\nu}(x)$ and its pullback $\phi^*g$ under a diffeomorphism represent the same physical spacetime.

(b) Einstein's original "hole argument": he thought that diffeomorphism invariance meant the field equations were not deterministic (different metrics in a "hole" — a region with no matter — could satisfy the equations). Resolve the paradox: why is GR in fact deterministic?

(c) Gauge fixing in GR: the harmonic gauge condition $\Box x^\mu = 0$ (where $x^\mu$ are the coordinates themselves, viewed as scalar functions) reduces the Einstein equations to a quasilinear wave equation, making the initial value problem well-posed. Compare to the Lorenz gauge in electromagnetism.
