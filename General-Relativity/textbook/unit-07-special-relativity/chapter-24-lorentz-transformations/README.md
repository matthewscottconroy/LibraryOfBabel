# Chapter 24: Lorentz Transformations

---

## Chapter Introduction

The Lorentz transformation is the mathematical core of special relativity. It is the rule for converting coordinates between inertial frames — the relativistic replacement for the Galilean transformation. Where Galilean transformations simply shift positions and leave time unchanged, Lorentz transformations mix space and time in a way that preserves the spacetime interval.

This chapter derives the Lorentz transformation from first principles — from Einstein's two postulates, without assuming any prior knowledge of what the transformation should look like. We then extract its physical consequences: time dilation, length contraction, and velocity addition. We derive the Doppler effect and aberration. And we study the group structure: how successive boosts compose, and why they do not simply add.

Mastery of Lorentz transformations is not optional for understanding GR. In GR, at each point in spacetime, the tangent space is Minkowski spacetime. Local physics is governed by Lorentz transformations. The equivalence principle — gravity can be locally transformed away — is mathematically the statement that one can always choose coordinates in which the metric looks Lorentzian and the connection vanishes at a point. You cannot understand that statement without knowing what a Lorentz transformation is.

---

## Derivation from Postulates

Consider two inertial frames $S$ and $S'$ where $S'$ moves at constant velocity $v$ along the $x$-axis relative to $S$, with their origins coinciding at $t = t' = 0$.

We assume:
1. The transformation is linear (inertial frames map to inertial frames)
2. The transformation is homogeneous (origin maps to origin — we set up this way)
3. The speed of light is $c$ in both frames (second postulate)
4. Spatial directions perpendicular to the motion are unaffected (follows from isotropy)

**Step 1: Transverse directions.**

By symmetry and isotropy: $y' = y$, $z' = z$.

**Step 2: Linear transformation in $x$ and $t$.**

The most general linear transformation (with origin fixed) is:
$$x' = Ax + Bt, \quad t' = Cx + Dt$$

**Step 3: Apply the boundary conditions.**

*Condition 1*: The origin of $S'$ ($x' = 0$) moves at velocity $v$ in $S$ ($x = vt$). So:
$$0 = Avt + Bt \implies B = -Av$$

*Condition 2*: A light signal emitted at the origin satisfies $x = ct$ in $S$ and $x' = ct'$ in $S'$:
$$x' = A(x - vt), \quad t' = Cx + Dt$$
$$\frac{x'}{t'} = c \implies \frac{A(c-v)}{Cc + D} = c$$

*Condition 3*: A light signal going backward ($x = -ct$ in $S$, $x' = -ct'$ in $S'$):
$$\frac{A(-c-v)}{-Cc + D} = -c$$

Solving conditions 2 and 3:
$$D = A, \quad C = -Av/c^2$$

*Condition 4*: The transformation must be its own inverse (up to sign of $v$). Applying the transformation twice with velocities $+v$ and $-v$ must recover the identity. This gives:
$$A^2(1 - v^2/c^2) = 1 \implies A = \frac{1}{\sqrt{1-v^2/c^2}} \equiv \gamma$$

**The Lorentz transformation:**

$$\boxed{t' = \gamma\left(t - \frac{vx}{c^2}\right), \quad x' = \gamma(x - vt), \quad y' = y, \quad z' = z}$$

where $\gamma = 1/\sqrt{1-v^2/c^2}$.

**Inverse transformation** (replace $v\to -v$):
$$t = \gamma\left(t' + \frac{vx'}{c^2}\right), \quad x = \gamma(x' + vt')$$

---

## Matrix Form

In components $x^\mu = (ct, x, y, z)$, the boost along $x$ is:

$$\Lambda^\mu_{\ \nu} = \begin{pmatrix}\gamma & -\beta\gamma & 0 & 0 \\ -\beta\gamma & \gamma & 0 & 0 \\ 0 & 0 & 1 & 0 \\ 0 & 0 & 0 & 1\end{pmatrix}$$

where $\beta = v/c$. The condition $\Lambda^T\eta\Lambda = \eta$ (where $\eta = \text{diag}(-1,+1,+1,+1)$) can be verified directly and confirms that $\Lambda$ is a Lorentz transformation.

This is analogous to a rotation in Euclidean space:
$$R = \begin{pmatrix}\cos\theta & -\sin\theta \\ \sin\theta & \cos\theta\end{pmatrix}, \quad R^TIR = I$$

A boost is a "hyperbolic rotation" with:
$$\gamma = \cosh\phi, \quad \beta\gamma = \sinh\phi$$

where $\phi = \text{arctanh}(\beta)$ is the **rapidity**. In terms of rapidity:
$$\Lambda = \begin{pmatrix}\cosh\phi & -\sinh\phi & 0 & 0 \\ -\sinh\phi & \cosh\phi & 0 & 0 \\ 0 & 0 & 1 & 0 \\ 0 & 0 & 0 & 1\end{pmatrix}$$

Rapidity is additive for collinear boosts: $\phi_{\rm total} = \phi_1 + \phi_2$ (unlike velocities).

---

## Physical Consequences

### Time Dilation

Consider a clock at rest in $S'$ ($x' = 0$). Two ticks at $t'_1$ and $t'_2$ with $\Delta t' = t'_2 - t'_1$. In $S$:
$$\Delta t = \gamma\Delta t'$$

A moving clock runs slow by factor $\gamma$. This is not an illusion — it is confirmed by:
- Muon lifetime: cosmic-ray muons survive to sea level because their decay time is dilated by $\gamma \sim 10$–$100$
- Atomic clocks on aircraft (Hafele-Keating experiment, 1971)
- GPS corrections: clocks on satellites run faster by $\Delta f/f \approx v^2/(2c^2) \approx 7\,\mu$s/day (SR effect alone; GR adds a larger correction of opposite sign)
- Particle accelerators: beam lifetime extended by $\gamma$ compared to rest-frame decay time

### Length Contraction

Consider a rod at rest in $S'$ with endpoints at $x'_1 = 0$ and $x'_2 = L_0$. In $S$, measuring the endpoints *simultaneously* ($t_1 = t_2$):
$$L = L_0/\gamma$$

The rod is shorter by $\gamma$. The crucial point: the measurement requires simultaneity, which is frame-dependent. In $S'$, the measurements of the rod's endpoints by an $S$-observer are *not* simultaneous.

### Velocity Addition

If a particle moves at velocity $u'$ in $S'$ (along $x'$), its velocity in $S$ is:
$$u = \frac{u' + v}{1 + u'v/c^2}$$

For $u' = c$ (light): $u = (c+v)/(1+v/c) = c$. ✓

For $u' \ll c$ and $v \ll c$: $u \approx u' + v$ (Galilean limit). ✓

The transverse velocity addition:
$$u_y = \frac{u'_y}{\gamma(1 + u'_x v/c^2)}$$

Note: transverse velocities also transform (unlike coordinates). This is because the time transformation mixes components.

---

## The Relativistic Doppler Effect

For a source moving toward the observer at velocity $v$, emitting light at frequency $f_0$:
$$f_{\rm obs} = f_0\sqrt{\frac{1+\beta}{1-\beta}} \quad (\text{approaching})$$

$$f_{\rm obs} = f_0\sqrt{\frac{1-\beta}{1+\beta}} \quad (\text{receding})$$

For transverse motion (source moving perpendicular to the line of sight at the moment of emission):
$$f_{\rm obs} = f_0/\gamma \quad (\text{transverse Doppler})$$

The transverse Doppler effect has no classical analogue — it is a pure time-dilation effect. It was first confirmed experimentally by Ives and Stilwell (1938).

The relativistic Doppler formula is used for all astronomical redshift measurements. The cosmological redshift $z$ at small velocities reproduces the Doppler formula; at large velocities, it is the ratio $a_{\rm now}/a_{\rm emission}$ from the FLRW metric.

---

## Aberration of Light

The direction of a light ray transforms between frames. If a photon arrives at angle $\theta$ to the $x$-axis in $S$, it arrives at angle $\theta'$ in $S'$:
$$\cos\theta' = \frac{\cos\theta - \beta}{1 - \beta\cos\theta}$$

For $\beta\to 1$: even photons coming from behind ($\cos\theta = -1$) in frame $S$ appear to come from ahead in $S'$. This is the **headlight effect** — a moving source concentrates radiation in the forward direction. For highly relativistic sources (jets, synchrotron radiation), this beaming is astrophysically crucial.

Aberration was detected classically by James Bradley (1727) as the annual wobble in the position of stars due to Earth's orbital motion. The relativistic correction to Bradley's result is a precision test of SR.

---

## The Lorentz Group

The set of all Lorentz transformations forms the **Lorentz group** $O(1,3)$. It has four connected components:
- **Proper orthochronous** $SO^+(1,3)$: $\det\Lambda = +1$, $\Lambda^0_{\ 0} > 0$ (time-orientation preserving, orientation preserving)
- **Proper non-orthochronous**: time-reversal $T$
- **Improper orthochronous**: parity $P$
- **Improper non-orthochronous**: $PT$

The connected component $SO^+(1,3)$ is generated by three boosts and three rotations (six generators). As a Lie algebra:
$$[J_i, J_j] = \varepsilon_{ijk}J_k, \quad [K_i, K_j] = -\varepsilon_{ijk}J_k, \quad [J_i, K_j] = \varepsilon_{ijk}K_k$$

where $J_i$ are rotation generators and $K_i$ are boost generators.

**Non-commutativity of boosts**: Two boosts in different directions do not compose to a pure boost — they include a rotation. This is **Thomas precession**, with:
$$\Omega_{\rm Thomas} \approx -\frac{\gamma^2}{\gamma+1}\frac{\mathbf{v}\times\mathbf{a}}{c^2}$$

Thomas precession contributes a factor of $1/2$ to the spin-orbit coupling in hydrogen (the Thomas factor), and is measurable in precision gyroscope experiments.

---

## Simultaneity, Causality, and the Light Cone Again

The relativity of simultaneity has a precise form: two events with spacelike separation ($ds^2 > 0$) have *no* absolute time ordering. There exist frames in which they are simultaneous, and frames in which either one comes first.

For timelike separation ($ds^2 < 0$): the time ordering is absolute. If event $A$ is in the past of event $B$ in one frame, it is so in all frames.

This is not merely a formal statement — it is the foundation of causality. If faster-than-light signals existed, they could be used (via the relativity of simultaneity) to send signals backward in time in some reference frames, violating causality.

**Tachyons** (hypothetical FTL particles): Their spacetime trajectory is spacelike. In some frames, they travel backward in time. This leads to the causality paradoxes that make tachyons physically implausible.

---

## Important Concepts

- **Lorentz transformation**: $t' = \gamma(t - vx/c^2)$, $x' = \gamma(x-vt)$ — the coordinate transformation preserving $ds^2$
- **Lorentz factor**: $\gamma = 1/\sqrt{1-v^2/c^2}$; diverges as $v\to c$
- **Rapidity**: $\phi = \text{arctanh}(\beta)$; additive under collinear boosts
- **Time dilation**: $\Delta t = \gamma\Delta t'$; moving clocks run slow; confirmed by muon lifetime, GPS, atomic clocks
- **Length contraction**: $L = L_0/\gamma$; moving rods shorter; requires frame-dependent simultaneity measurement
- **Relativistic velocity addition**: $u = (u'+v)/(1+u'v/c^2)$; reduces to Galilean for $u', v\ll c$
- **Relativistic Doppler**: $f_{\rm obs} = f_0\sqrt{(1\pm\beta)/(1\mp\beta)}$ (approaching/receding)
- **Aberration**: Direction of light transforms; headlight effect for relativistic sources
- **Thomas precession**: Successive non-collinear boosts produce a rotation; affects atomic spectra
- **Lorentz group** $O(1,3)$: Symmetry group of Minkowski metric; six generators (3 boosts + 3 rotations)

---

## Important Figures

**Hendrik Lorentz** (1853–1928): Derived the transformation equations in 1904 as a dynamical effect; Nobel Prize 1902.

**Albert Einstein** (1879–1955): Derived the same equations from postulates in 1905, giving them their correct physical interpretation.

**Llewellyn Thomas** (1903–1992): Derived Thomas precession (1926), resolving the factor-of-2 discrepancy in spin-orbit coupling.

**Herbert Ives & G.R. Stilwell** (1938): First experimental confirmation of the transverse (second-order) Doppler effect.

**Joseph Hafele & Richard Keating** (1971): Flew atomic clocks around the world and confirmed SR time dilation (and GR gravitational redshift) to 10%.

---

## Further Reading

**Primary Sources**
- Einstein, A. (1905). "On the Electrodynamics of Moving Bodies." *Annalen der Physik*, 17, 891.
- Thomas, L.H. (1926). "The Motion of the Spinning Electron." *Nature*, 117, 514.
- Ives, H.E. & Stilwell, G.R. (1938). "An Experimental Study of the Rate of a Moving Atomic Clock." *JOSA*, 28, 215.
- Hafele, J. & Keating, R. (1972). "Around-the-World Atomic Clocks." *Science*, 177, 166.

**Textbooks**
- Taylor, E.F. & Wheeler, J.A. (1992). *Spacetime Physics*. Freeman. — Chapters 2–4.
- Jackson, J.D. (1999). *Classical Electrodynamics* (3rd ed.). Wiley. — Chapter 11 on relativistic kinematics.
- Rindler, W. (2006). *Relativity: Special, General, and Cosmological* (2nd ed.). Oxford. — Systematic and clean.

---

## Exercises

**24.1.** *Lorentz transformation: calculations.*

In frame $S$, event $A$ is at $(t, x) = (0, 0)$ and event $B$ is at $(t, x) = (4\ \mu\text{s},\ 600\ \text{m})$ (taking $c = 3\times 10^8$ m/s).

(a) Compute $ds^2$ between $A$ and $B$. Is the interval timelike, null, or spacelike?

(b) Find the frame $S'$ moving at velocity $v$ along $x$ in which the events are simultaneous ($t'_A = t'_B$). What is $v$?

(c) In the frame from (b), what is the spatial separation between the events?

(d) Is there a frame in which both events occur at the same location? Explain.

---

**24.2.** *Rapidity and velocity addition.*

(a) Express $v = \tanh\phi$ and show that $\gamma = \cosh\phi$, $\beta\gamma = \sinh\phi$.

(b) Two rockets, each moving at $v = 0.9c$ relative to Earth in opposite directions. What is the speed of rocket 2 as measured by rocket 1? Compute using velocity addition formula.

(c) Compute the same answer using rapidity: if $\phi_1$ is the rapidity of rocket 1 relative to Earth and $\phi_2$ is rocket 2's rapidity relative to Earth (opposite direction), what is the total rapidity? What is the corresponding velocity?

(d) Show that for $N$ collinear boosts with rapidity $\phi$ each, the total rapidity is $N\phi$ — but the total velocity is $\tanh(N\phi)$, which approaches $c$ but never reaches it.

---

**24.3.** *The muon experiment.*

Cosmic-ray pions decay at an altitude of $\sim 15$ km with lifetime $\tau_\pi \approx 26$ ns, producing muons with $\gamma_\mu \approx 25$ and proper lifetime $\tau_\mu = 2.2\ \mu$s.

(a) In Earth's frame: how far does a muon travel in one proper lifetime without time dilation? Compare to the 15 km to sea level.

(b) In Earth's frame with time dilation: compute the lab-frame lifetime $\tau_{\rm lab} = \gamma\tau_\mu$. Can the muon reach sea level?

(c) In the muon's frame: the atmosphere is length-contracted. What is the contracted thickness of the atmosphere? How does the muon "see" the 15 km fitting within its short lifetime?

(d) Both perspectives give the same physical result (muon reaches sea level). This illustrates that time dilation and length contraction are two sides of the same geometric fact. Identify what each frame uses to explain the survival.

---

**Thought Experiment T24.1.** *Can you exceed the speed of light by repeated kicks?*

Suppose a spacecraft is accelerating, with each rocket burn providing a fixed boost of $\Delta v$ in the rocket's instantaneous rest frame. In Newtonian mechanics, such a sequence of kicks would accelerate the rocket indefinitely. 

In SR, use rapidity to model $N$ kicks of rapidity $\Delta\phi$ each. What happens to the velocity after $N$ kicks as $N\to\infty$? What does the crew experience (proper acceleration vs. coordinate acceleration)? 

Why is it energetically possible to add arbitrarily many kicks (the spacecraft can carry more fuel or receive more energy) but kinematically impossible to exceed $c$? What physical quantity — energy, momentum, or something else — actually diverges, and what does that tell you about why $c$ is a barrier?
