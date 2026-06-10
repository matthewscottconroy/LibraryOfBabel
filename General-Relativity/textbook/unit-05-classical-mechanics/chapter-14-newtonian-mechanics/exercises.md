# Chapter 14 Exercises: Newtonian Mechanics

---

## Section 14.1: Newton's Laws

**14.1.1** Newton defined absolute space as that which "remains always similar and immovable." Leibniz argued this was physically meaningless — only relative positions matter. Consider a universe with only two particles, A and B. (a) What physically observable quantities exist in Newton's universe? In Leibniz's? (b) Can you distinguish uniform linear motion from rest in Newton's universe? (c) Can you distinguish rotation from non-rotation? What does this suggest about which aspects of Newton's absolute space are empirically meaningful?

**14.1.2** The equivalence principle in its weak form: inertial mass $m_i$ (in $F = m_i a$) and gravitational mass $m_g$ (in $F_g = m_g g$) satisfy $m_i = m_g$ to precision $\sim 10^{-13}$ (Schlamminger et al., 2008). (a) In Newtonian gravity, if $m_i \neq m_g$, what would the trajectory of a freely-falling particle depend on? (b) The Braginsky-Panov experiment (1971) compared the accelerations of platinum and aluminum test masses in free fall. If $m_g/m_i$ differs by $\eta$ between two materials, derive the expected differential acceleration in terms of $\eta$ and Earth's gravitational field $g$. (c) The experiment achieved sensitivity $\eta < 10^{-12}$. Express this as a limit on the energy-equivalence correction to gravity for nuclear vs. electromagnetic binding energy.

**14.1.3** The Galilean group has 10 parameters: 3 rotations, 3 spatial translations, 3 velocity boosts, 1 time translation. (a) Write the transformation law for position and velocity under a Galilean boost $v \to v + V$. (b) Show that Newton's second law $F = ma$ is invariant under this transformation when the force depends only on relative positions. (c) Show that the Maxwell wave equation $\Box \varphi = 0$ (where $\Box = \partial_t^2/c^2 - \nabla^2$) is *not* invariant under Galilean boosts. (d) What does (c) imply about the relationship between Newtonian mechanics and electrodynamics?

**14.1.4** *(Thought experiment)* Einstein's elevator: An observer is in a sealed box with no windows. They feel a downward force. (a) Can they determine whether they are at rest on Earth (gravitational field $g$) or accelerating upward at $g$ in empty space? What observations inside the box could help? (b) Now suppose the box is in free fall near Earth. Describe what the observer experiences for a very small box vs. a very large box. (c) What is the spatial scale at which the "freely falling observer" approximation breaks down? Express in terms of $g$, the tidal gradient, and the precision of measurement. This is the *equivalence principle* — the foundation of GR.

---

## Section 14.2: Conservation Laws

**14.2.1** A satellite of mass $m$ is in a circular orbit of radius $R$ around Earth (mass $M$). (a) Find the orbital speed $v$, period $T$, kinetic energy $T_k$, potential energy $V$, and total energy $E$ in terms of $M$, $m$, $G$, $R$. (b) Verify the virial theorem: $\langle T_k \rangle = -E$ and $\langle V \rangle = 2E$. (c) To raise the orbit to radius $2R$, a rocket fires briefly, adding kinetic energy $\Delta T_k$. Paradoxically, the satellite ends up *slower* after the burn. Compute the initial speed, the final speed, and $\Delta T_k$. Explain the apparent contradiction using the virial theorem.

**14.2.2** A meteor of mass $m$ falls radially from infinity with zero initial velocity. (a) Find its speed when it reaches radius $r$ from Earth's center. (b) The crater diameter depends on kinetic energy at impact. If a meteor falls from infinity to Earth's surface ($r = R_\oplus$) and another falls from the Moon's orbit ($r = 60R_\oplus$), by what factor do their impact kinetic energies differ? (c) A real meteor enters with some angular momentum $\ell \neq 0$. Show that the angle of trajectory at radius $r$ (relative to the radial direction) is $\sin\theta = \ell/(mrv)$.

**14.2.3** *(Negative heat capacity of stars)* A star radiates energy $\Delta E < 0$ to space. Use the virial theorem to show that the star's temperature (proportional to kinetic energy) *increases*. Specifically: (a) Write $E = T + V$ and use the virial theorem $T = -E$ to express $T$ in terms of $E$. (b) Show $dT/dE = -1$: as the star loses energy, its kinetic energy (temperature) increases. (c) This means the star has negative heat capacity. Describe qualitatively what happens to a star that exhausts its nuclear fuel (which provided an energy source offsetting radiation losses). (d) What terminates the collapse in the cases of: a white dwarf, a neutron star, a black hole?

**14.2.4** Prove the virial theorem for a system of $N$ particles interacting via Newtonian gravity ($V_{ij} = -Gm_im_j/|\mathbf{r}_i - \mathbf{r}_j|$). (a) Define the moment of inertia $I = \sum_i m_i r_i^2$. Show that $\ddot{I}/2 = 2T + \sum_{i<j}\mathbf{F}_{ij}\cdot(\mathbf{r}_i - \mathbf{r}_j)$. (b) Evaluate $\sum_{i<j}\mathbf{F}_{ij}\cdot(\mathbf{r}_i - \mathbf{r}_j)$ for gravitational forces. (c) For a bound, time-averaged system with $\langle \ddot{I} \rangle = 0$, derive $2\langle T \rangle + \langle V \rangle = 0$ (the virial theorem for gravity). (d) Apply this to estimate the mass of a galaxy cluster from the observed velocity dispersion $\sigma_v$ and radius $R$.

---

## Section 14.3: Gravitational Potential

**14.3.1** (a) Show that the potential of a uniform spherical shell of mass $M$ and radius $R$ is $\Phi = -GM/r$ for $r > R$ and $\Phi = -GM/R$ (constant) for $r < R$. (*Hint*: use Gauss's law for gravity $\oint \mathbf{g}\cdot d\mathbf{A} = -4\pi G M_{\rm enc}$.) (b) Inside a uniform solid sphere of density $\rho_0$, show $\Phi(r) = -\frac{2\pi G\rho_0}{3}(3R^2 - r^2)$ for $r < R$. (c) Plot $\Phi(r)/\Phi(R)$ for the shell and the solid sphere. What does the flatness of $\Phi$ inside the shell imply for a particle placed inside?

**14.3.2** The quadrupole potential of Earth (an oblate spheroid): $\Phi = -GM_\oplus/r\left[1 - J_2(R_\oplus/r)^2 P_2(\cos\theta)\right]$ where $J_2 = 1.08 \times 10^{-3}$ and $\theta$ is the colatitude. (a) Compute the force on a satellite in a polar orbit at altitude $h = 500$ km. (b) Show that the quadrupole term causes the orbital plane to precess (nodal regression). Estimate the precession rate in degrees per day. (c) Compare this Earth oblateness effect to the GR precession of Mercury computed in Section 16.3. Which is larger, and by how much?

**14.3.3** *(Tidal forces and the Roche limit)* A moon of density $\rho_m$ orbits a planet of density $\rho_p$ at distance $d$. Tidal forces from the planet stretch the moon; the moon's self-gravity holds it together. (a) Estimate the tidal force on a rock of mass $\delta m$ on the near side of the moon (radius $r_m$), due to the planet (mass $M_p$, at distance $d$). Express as a tidal acceleration $\sim GM_p r_m/d^3$. (b) The moon's self-gravity on the same rock is $\sim G M_m/r_m^2$. Set these equal to find the Roche limit: $d_{\rm Roche} \sim r_m(M_p/M_m)^{1/3} \sim (M_p/\rho_m)^{1/3}$. (c) Saturn's ring system lies inside the Roche limit of Saturn. Compute $d_{\rm Roche}$ for a typical icy body ($\rho_m = 900$ kg/m³) around Saturn. Compare to the extent of Saturn's rings (75,000–140,000 km).

**14.3.4** *(Geodesic deviation and the Riemann tensor)* Two freely-falling particles start at positions $\mathbf{r}$ and $\mathbf{r} + \boldsymbol{\xi}$ with the same initial velocity. (a) Show that the relative acceleration satisfies $\ddot{\boldsymbol{\xi}} = -(\nabla\nabla\Phi)\cdot\boldsymbol{\xi}$ (the tidal tensor equation). (b) For $\Phi = -GM/r$, compute the tidal tensor $T_{ij} = -\partial^2\Phi/\partial r^i\partial r^j$ in spherical coordinates. (c) Show that $T_{ij}$ is trace-free: $\sum_i T_{ii} = 0$ in vacuum. What does this correspond to in GR? (d) A LIGO arm of length $L$ is oriented at angle $\alpha$ to a gravitational wave propagating in the $z$-direction. The wave has metric perturbation $h_{xx} = -h_{yy} = h_+\cos(\omega t)$. Write the equation of motion for a mirror at the end of the arm and compute $\Delta L/L$ as a function of $h_+$ and $\alpha$.

---

## Thought Experiments

**TE 14.1: Mach's Principle and the Bucket**
Newton filled a bucket with water, suspended it by a twisted rope, and let it spin. Initially the water surface was flat (bucket rotating, water not). Later it was concave (bucket and water rotating together). The concavity, he argued, proved rotation relative to absolute space.

Leibniz and Mach objected: rotation relative to what? Mach proposed that inertia arises from the distribution of distant matter. Imagine the universe with only the bucket and its water — no stars, no galaxies. Would the water surface be curved? (a) What does Newtonian mechanics predict? (b) What would Mach predict? (c) Einstein was inspired by Mach to seek a theory in which the geometry of spacetime is determined by matter. Does GR fully implement Mach's principle? (Consider: in GR, is there a solution where a rotating mass in an otherwise empty universe drags the local inertial frames?)

**TE 14.2: Weighing the Earth**
Cavendish (1798) measured $G$ by observing the force between lead spheres. He described his experiment as "weighing the Earth." (a) Once $G$ is known and the period $T$ of a satellite at radius $r$ is observed, how do you find Earth's mass $M$? (b) Before Cavendish, could Newton determine the absolute scale of planetary masses (not just mass ratios)? (c) The Sun's mass can be found from Earth's orbital parameters without knowing $G$ — you only need $G \times M_\odot$. Explain why $G$ and $M$ always appear together in celestial mechanics. What does this imply about fundamental constants in gravity?

---

## Laboratory Explorations

**Lab 14.1: Measuring $g$ with a Pendulum**
A pendulum of length $L$ has period $T = 2\pi\sqrt{L/g}$ for small angles. (a) Measure $T$ for several values of $L$ and plot $T^2$ vs. $L$. (b) Fit a line to determine $g$. (c) Assess systematic errors: the small-angle approximation breaks down for $\theta > 5°$; finite mass of the string; air resistance. (d) Estimate the altitude above sea level of your lab from your measured $g$ and the known variation $g(h) \approx g_0(1 - 2h/R_\oplus)$. Compare to the known altitude.

**Lab 14.2: Tidal Forces in the Lab**
Fill a cylindrical container with water. Spin it about its vertical axis at angular velocity $\omega$. (a) Predict the shape of the water surface using the condition that it is an equipotential surface in the rotating frame (where the centrifugal potential is $-\omega^2 r^2/2$). (b) Measure the height $h(r)$ at various radii and plot $h$ vs. $r^2$. (c) Extract $\omega$ from the slope and compare to the measured rotation rate. (d) Connect this to GR: the equipotential surface in a rotating frame is analogous to a Killing horizon in the Kerr spacetime.
