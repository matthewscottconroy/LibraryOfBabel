# Chapter 19: Exercises

---

## Section 19.1 — Spacetime and the Interval

**19.1.1.** *The causal structure of spacetime.*

Event $A$ is at the origin $(t, x, y, z) = (0, 0, 0, 0)$.

(a) For each of the following events, determine whether it is in the timelike past, timelike future, spacelike separated, or on the light cone relative to $A$:
- $B$: $(t, x, y, z) = (3 \text{ s}, 1 \text{ m}, 0, 0)$
- $C$: $(t, x, y, z) = (1 \text{ ns}, 1 \text{ m}, 0, 0)$
- $D$: $(t, x, y, z) = (1 \text{ year}, 1 \text{ light-year}, 0, 0)$
- $E$: $(t, x, y, z) = (-2 \text{ s}, 1 \text{ m}, 0, 0)$

(b) For the timelike pairs, compute the proper time between them (the spacetime interval in units of time). For the spacelike pairs, compute the proper distance (the spacetime interval in units of length).

(c) Is causal structure (timelike/spacelike/null separation) Lorentz-invariant? Prove it by showing that $ds^2 = -c^2dt^2 + dx^2 + dy^2 + dz^2$ is preserved under a Lorentz boost.

(d) Can the time-ordering of two spacelike-separated events be reversed by a Lorentz transformation? Prove or disprove. What would it mean physically if time-ordering of causally connected events could be reversed?

---

**19.1.2.** *Proper time and the twin paradox.*

The "twins" $A$ and $B$ start together at rest. $A$ remains on Earth; $B$ accelerates to $v = 0.8c$, travels to a star 4 light-years away, and returns.

(a) In Earth's frame: the round-trip takes $t = 2 \times (4 \text{ ly})/(0.8c) = 10$ years for $A$. Compute the elapsed proper time for $B$ using $d\tau = dt\sqrt{1-v^2/c^2}$. What is $\gamma$ for $v = 0.8c$?

(b) The twin "paradox" seems to say that in $B$'s frame, $A$ should age less. Resolve the paradox: what is the physical asymmetry between the two twins? (Hint: which worldline is inertial throughout?)

(c) Proper time is the length of a worldline in Minkowski spacetime (with the Lorentzian sign convention). The twin who travels a longer path through *space* travels a shorter path through *spacetime*. Explain this geometric statement carefully: why does the longest spacetime path correspond to staying at rest?

(d) The Hafele-Keating experiment (1972) flew atomic clocks around the world on commercial airliners. The predicted time difference (for the eastbound flight) was $-59 \pm 10$ ns (special relativistic effect) plus $+144 \pm 14$ ns (gravitational blueshift from altitude). The observed value was $+273 \pm 7$ ns for the westbound flight and $-59 \pm 10$ ns for the eastbound. Estimate the special relativistic contribution for a plane flying at $v = 900$ km/h, and check whether it has the right sign and magnitude.

---

**19.1.3.** *The Minkowski metric and index gymnastics.*

In Minkowski space with metric $\eta_{\mu\nu} = \text{diag}(-1, +1, +1, +1)$ (signature $-+++$):

(a) Raise and lower indices:
- If $V^\mu = (c, v_x, v_y, v_z)$, what is $V_\mu$?
- If $p^\mu = (E/c, p_x, p_y, p_z)$, compute $p_\mu p^\mu$.
- If $k^\mu = (\omega/c, k_x, k_y, k_z)$ with $k^\mu k_\mu = 0$, what is the dispersion relation?

(b) Compute $\eta^{\mu\nu}$ (the inverse of $\eta_{\mu\nu}$). Show $\eta^{\mu\alpha}\eta_{\alpha\nu} = \delta^\mu_{\ \nu}$.

(c) The 4-velocity of a massive particle satisfies $u_\mu u^\mu = -c^2$. Show this is consistent with $u^\mu = \gamma(c, \mathbf{v})$.

(d) A photon has 4-momentum $k^\mu = \hbar(\omega/c, k_x, 0, 0)$ with $k_\mu k^\mu = 0$. A massive electron has 4-momentum $p^\mu = (E/c, p, 0, 0)$ at rest, so $p^\mu = (m_e c, 0, 0, 0)$. Compute the Lorentz invariant $k_\mu p^\mu$ and show that in the Compton scattering formula, the energy of the scattered photon depends on this invariant.

---

## Section 19.2 — Lorentz Transformations

**19.2.1.** *Relativistic velocity addition and rapidity.*

Rapidity $\phi$ is defined by $v = c\tanh\phi$, so $\gamma = \cosh\phi$ and $\beta\gamma = \sinh\phi$.

(a) A Lorentz boost in the $x$-direction with velocity $v = c\tanh\phi$ has the matrix form:
$$\Lambda^\mu_{\ \nu} = \begin{pmatrix}\cosh\phi & -\sinh\phi & 0 & 0 \\ -\sinh\phi & \cosh\phi & 0 & 0 \\ 0 & 0 & 1 & 0 \\ 0 & 0 & 0 & 1\end{pmatrix}$$
Show that two successive boosts with rapidities $\phi_1$ and $\phi_2$ compose to a single boost with rapidity $\phi_1 + \phi_2$.

(b) What is the relativistic velocity addition formula in terms of rapidity? Derive the formula $v_{12} = (v_1 + v_2)/(1 + v_1 v_2/c^2)$ from rapidity addition.

(c) A particle moves at $v_1 = 0.9c$ relative to frame $S'$, which moves at $v_2 = 0.9c$ relative to frame $S$. What is the particle's speed relative to $S$? Compare the Newtonian prediction ($1.8c$) to the relativistic result.

(d) What is the rapidity of a photon? What rapidity corresponds to $v = 0.999999c$ (a particle in the Large Hadron Collider where protons reach $\gamma \approx 7000$)?

---

**19.2.2.** *Time dilation and the muon experiment.*

Cosmic ray muons are produced at altitude $h \approx 15$ km with speed $v \approx 0.998c$ and lifetime $\tau_0 = 2.2$ μs in their rest frame.

(a) Classically ($v = 0.998c$ but no time dilation), what is the maximum distance a muon could travel in the lab frame before decaying? Can it reach sea level?

(b) Relativistically, compute $\gamma$ for $v = 0.998c$. What is the muon's lab-frame lifetime? What fraction of muons (initially $N_0$) survive to reach sea level?

(c) In the muon's rest frame, the atmosphere is Lorentz-contracted. What is the muon's rest-frame distance to the Earth's surface at production? Show that in the muon's frame, it travels a proper time $\tau_0$ at $v \approx 0.998c$ and barely reaches the ground.

(d) Both observers (lab frame and muon rest frame) agree on whether the muon reaches sea level. They disagree on the *reason* (time dilation vs. length contraction). Explain why this is consistent. What is the Lorentz-invariant statement about whether the muon reaches the ground?

---

**19.2.3.** *Relativistic aberration and the headlight effect.*

A star emits light isotropically in its rest frame. An observer moves toward the star at speed $v$.

(a) In the star's rest frame, half the photons are emitted into the hemisphere facing the observer. In the observer's frame, what fraction of photons are emitted into the forward hemisphere? (Use the relativistic aberration formula: if a photon makes angle $\theta'$ in the star's frame, it makes angle $\theta$ in the observer's frame with $\cos\theta = (\cos\theta' + \beta)/(1 + \beta\cos\theta')$.)

(b) At $v = 0.99c$ ($\gamma \approx 7$), the light is concentrated into a forward cone of half-angle $\theta_{\rm half} \approx 1/\gamma$. Calculate this half-angle.

(c) The "headlight effect" is crucial for astrophysics: a relativistic jet in an AGN emits radiation beamed toward Earth. The apparent luminosity of a jet pointed at angle $\theta$ to the line of sight is boosted by a factor $\mathcal{D}^4$ where $\mathcal{D} = [\gamma(1-\beta\cos\theta)]^{-1}$ is the Doppler factor. For $v = 0.99c$ and $\theta = 5°$, compute $\mathcal{D}$ and the luminosity boost factor.

(d) Superluminal motion: a jet blob at $v = 0.99c$ at angle $\theta = 5°$ to the line of sight. The apparent transverse velocity (from how fast the blob appears to move across the sky) is $v_{\rm app} = v\sin\theta/(1-\beta\cos\theta)$. Compute $v_{\rm app}/c$. Can it exceed $c$? Is this a violation of special relativity?

---

## Thought Experiments

**T19.1.** *Simultaneity and the relativity of now.*

Two lightning bolts strike the front and back of a train simultaneously in the ground frame. A passenger at the midpoint of the train observes that the bolt at the front struck *first* (the front-strike light reaches them before the back-strike light, since they are moving toward the front).

But if the passenger claims the front bolt struck first, and the ground observer claims they were simultaneous, who is right? Neither is wrong — simultaneity is relative to a frame of reference. There is no absolute "now" that extends across space.

This has radical implications: if Event A happens "now" and is spacelike separated from you, then in another reference frame, Event A happens "in the future" or "in the past." The set of events happening "now" is different for every observer moving at a different velocity.

Does this mean the future is already determined? Does it mean time travel is possible if you move fast enough? Work through the causal structure carefully: what prevents using the relativity of simultaneity to send information backward in time?

---

**T19.2.** *What does the light cone say about causality?*

The light cone divides spacetime into regions: timelike future, timelike past, and spacelike. The principle of causality says that no physical influence can propagate faster than $c$ — which means no cause can affect an event outside its future light cone.

Quantum mechanics seems to challenge this: the EPR/Bell correlations between entangled particles are instantaneous, in the sense that measuring one particle instantly "collapses" the other no matter how far apart they are. Yet Bell's theorem shows these correlations cannot be used to send information faster than light.

Why can't the "instantaneous collapse" be used for faster-than-light signaling? Construct the most careful possible argument. Your argument should invoke the no-communication theorem and explain precisely where any proposed FTL-communication scheme fails.

---

## Laboratory Exercise: Relativistic Kinematics with Cosmic Ray Muons

**L19.1.** *Measuring muon flux at different altitudes.*

The muon flux at sea level is approximately 10,000 muons per square meter per minute. The lifetime of muons in their rest frame is $\tau_0 = 2.197$ μs. Cosmic ray muons are produced at $\sim 15$ km altitude with speeds close to $c$.

**Task 1 (calculation):** For each speed $v/c \in \{0.9, 0.95, 0.99, 0.999\}$, compute (a) $\gamma$, (b) the lab-frame lifetime $\gamma\tau_0$, (c) the distance traveled in one lifetime, and (d) the fraction of muons surviving to sea level (assuming production at 15 km).

**Task 2 (experimental):** Using a pair of Geiger-Müller tubes (or a commercial muon detector like the CosmicWatch or QuarkNet board), measure the muon count rate at sea level and, if possible, at a higher elevation (the roof of a tall building, or a nearby hill — each 100 m of altitude makes a measurable difference over a few hours).

**Task 3 (analysis):** Compare the measured count rate ratio at two altitudes to the prediction from relativistic time dilation. The non-relativistic prediction (using the rest-frame lifetime but classical mechanics) would give a much lower count rate at sea level than observed. By how much does the relativistic prediction differ from the non-relativistic one?

