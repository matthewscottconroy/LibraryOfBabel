# Chapter 56: The Unruh Effect

---

## Chapter Introduction

You are sitting still, reading these words. The quantum field around you is in its vacuum state — no particles, no radiation, silence at the quantum level. Now imagine accelerating at $a = 2\times 10^{20}$ m/s$^2$ (about $2\times 10^{19}$ g). According to William Unruh's 1976 calculation, you would find yourself bathed in thermal radiation at a temperature of approximately 1 Kelvin.

This is the Unruh effect: a uniformly accelerating observer in flat Minkowski spacetime, in the vacuum state of an inertial observer, detects a thermal spectrum of particles at temperature $T_U = \hbar a/(2\pi ck_B)$. The vacuum state of inertial observers is a thermal state for accelerating observers.

The Unruh effect is not merely a curiosity — it is the cleanest, most tractable version of Hawking radiation. The physics is the same: the event horizon (in this case, a Rindler horizon) leads to particle creation via the Bogoliubov mechanism. The derivation is more transparent because the spacetime is flat, and one can track exactly what each observer sees. Understanding the Unruh effect at the deepest level is understanding Hawking radiation.

The temperatures involved are extraordinarily small — for experimentally accessible accelerations ($a \sim 10^{20}$ m/s$^2$ corresponds to $T_U \sim 1$ K; typical lab accelerations give $T_U \ll 10^{-20}$ K) — so the Unruh effect has never been directly detected. But its indirect manifestations — the Unruh-DeWitt detector formalism, its role in the derivation of the Hawking effect, Unruh radiation as the Rindler version of Hawking radiation — make it central to quantum gravity.

---

## Rindler Spacetime

An observer undergoing **uniform acceleration** $a$ along the $x$-axis in Minkowski spacetime follows the worldline:
$$t = \frac{c}{a}\sinh\left(\frac{a\tau}{c}\right), \quad x = \frac{c^2}{a}\cosh\left(\frac{a\tau}{c}\right)$$

where $\tau$ is the proper time. This is a hyperbola in the $(t,x)$ Minkowski plane: $x^2 - c^2t^2 = c^4/a^2$.

The **Rindler coordinates** $(\xi, \eta)$ adapted to this motion:
$$t = \frac{1}{a}\sinh(a\eta/c)\cdot e^{a\xi/c^2}, \quad x = \frac{c^2}{a}\cosh(a\eta/c)\cdot e^{a\xi/c^2}$$

(various conventions exist; here $\eta$ is the Rindler time and $\xi$ is the Rindler spatial coordinate).

The Minkowski metric in Rindler coordinates:
$$ds^2 = -e^{2a\xi/c^2}c^2d\eta^2 + d\xi^2 + dy^2 + dz^2$$

This is Rindler spacetime. Notice: the metric is time-independent (the accelerating observer sees a static geometry), and there is a **Rindler horizon** at $x = c|t|$ — the light sheets from the future and past null infinity of the origin. The Rindler horizon is exactly analogous to a black hole horizon: the right Rindler wedge ($x > |ct|$) is the analogue of the black hole exterior; signals from the left Rindler wedge ($x < -|ct|$) never reach the accelerating observer.

---

## The Derivation: Minkowski vs. Rindler Vacua

**Minkowski modes**: In flat spacetime, the natural positive-frequency modes are:
$$u_k \propto e^{-i\omega_k t + ikx}, \quad \omega_k = c|k| > 0$$

The Minkowski vacuum $|0_M\rangle$ is defined by $\hat{a}_k|0_M\rangle = 0$.

**Rindler modes**: In the Rindler wedge, the natural positive-frequency modes (positive frequency with respect to Rindler time $\eta$) are:
$$v_\Omega \propto e^{-i\Omega\eta}\cdot(\text{spatial function})$$

where $\Omega > 0$ is the Rindler frequency.

**Key calculation**: The Minkowski vacuum $|0_M\rangle$ is NOT the Rindler vacuum $|0_R\rangle$. Expanding Minkowski modes in terms of Rindler modes gives Bogoliubov coefficients:

$$\alpha_{\Omega k} \propto \frac{e^{\pi\Omega c/(2a)}}{\sinh(\pi\Omega c/a)}\delta(k-\text{related}), \quad \beta_{\Omega k} \propto \frac{e^{-\pi\Omega c/(2a)}}{\sinh(\pi\Omega c/a)}\delta(k-\text{related})$$

The key relation: $|\beta_\Omega|^2/|\alpha_\Omega|^2 = e^{-2\pi\Omega c/a}$.

The mean Rindler particle number in the Minkowski vacuum:
$$\langle 0_M|\hat{N}_\Omega^R|0_M\rangle = |\beta_\Omega|^2 = \frac{1}{e^{2\pi\Omega c/a} - 1}$$

This is a **Planck distribution** with temperature:
$$T_U = \frac{\hbar a}{2\pi ck_B}$$

The Minkowski vacuum looks like a thermal state at temperature $T_U$ to a Rindler (uniformly accelerating) observer.

---

## The Thermofield Double State

The Unruh effect has a beautiful algebraic formulation. The Minkowski vacuum can be written as an **entangled state** of left and right Rindler wedge modes:

$$|0_M\rangle = \prod_\Omega\frac{1}{\cosh r_\Omega}\sum_{n=0}^\infty\tanh^n r_\Omega\,|n_\Omega\rangle_R|n_\Omega\rangle_L$$

where $\tanh r_\Omega = e^{-\pi\Omega c/a}$ and $|n_\Omega\rangle_{R,L}$ are Rindler number states in the right/left wedge.

This is the **thermofield double (TFD) state** — a purification of the thermal state. An observer in the right Rindler wedge, who has no access to the left wedge, traces over the left modes:
$$\rho_R = \text{Tr}_L|0_M\rangle\langle 0_M| = \frac{e^{-\hat{H}_R/T_U}}{Z}$$

This is exactly the thermal density matrix at temperature $T_U$.

**Deep implication**: The Minkowski vacuum is entangled between the two Rindler wedges. An observer confined to one wedge sees a thermal state because entanglement with the inaccessible wedge has been "traced out." The thermality is a consequence of entanglement + restricted access (a horizon).

This picture generalizes to black holes: the Hartle-Hawking state for a black hole is entangled between the exterior and interior. An exterior observer traces over the interior, obtaining the thermal Hawking state.

---

## The Unruh-DeWitt Detector

A concrete model for the Unruh effect: a two-level quantum system (the detector) with ground state $|g\rangle$ and excited state $|e\rangle$, energy gap $E = \hbar\Omega_0$, coupled to the field via:
$$\hat{H}_{\rm int} = g\hat\mu(\tau)\hat\phi(x(\tau))$$

where $\hat\mu = |e\rangle\langle g| + |g\rangle\langle e|$ is the monopole moment and $x(\tau)$ is the worldline.

First-order perturbation theory: transition rate from $|g, 0_M\rangle$ to $|e, \text{any field state}\rangle$:
$$\Gamma_{0\to E} = g^2\int_{-\infty}^\infty d\Delta\tau\,e^{-i\Omega_0\Delta\tau}W(x(\tau), x(\tau'))$$

where $W(x,x') = \langle 0_M|\hat\phi(x)\hat\phi(x')|0_M\rangle$ is the Wightman function.

For an **inertial detector** at rest: $W(x,x')$ depends only on $\Delta t = t - t'$ and decays at large separations. The integral gives $\Gamma \propto \delta(-\Omega_0)= 0$ — no excitation in vacuum.

For a **uniformly accelerating detector** (acceleration $a$, worldline $x^\mu(\tau) = (c\sinh(a\tau/c)/a, c^2\cosh(a\tau/c)/a, 0, 0)$):

The Wightman function along the worldline:
$$W(\tau, \tau') = -\frac{c^2\hbar}{4\pi^2}\frac{1}{(t-t'-i\varepsilon)^2c^2 - (x-x')^2}$$

$$= -\frac{\hbar}{4\pi^2}\frac{a^2/c^2}{4\sinh^2(a(\Delta\tau-i\varepsilon)/(2c))}$$

The integral gives the thermal spectrum:
$$\Gamma_{0\to E} = \frac{g^2\Omega_0}{2\pi c}\frac{1}{e^{2\pi\Omega_0 c/a} - 1}$$

This is exactly the thermal excitation rate at temperature $T_U = \hbar a/(2\pi ck_B)$.

---

## Experimental Status

The Unruh temperature $T_U = \hbar a/(2\pi ck_B) \approx (4\times 10^{-23}\ \text{K})\times (a/\text{m/s}^2)$.

For lab-accessible accelerations:
- Atomic beam, $a \sim 10^{15}$ m/s$^2$: $T_U \sim 4\times 10^{-8}$ K — unmeasurably small
- Electron in laser focus, $a \sim 10^{26}$ m/s$^2$: $T_U \sim 10^3$ K — but the field itself is not a vacuum

**Analog systems**: Schwinger pair creation in strong electric fields is related by the same Bogoliubov mechanism. Sonoluminescence (light from collapsing bubbles) has been proposed as an acoustic Unruh effect, but the connection is disputed.

**Relativistic heavy-ion collisions**: The Unruh temperature at the deceleration rates in RHIC/LHC may be comparable to the QCD deconfinement temperature ($\sim 150$ MeV $\sim 10^{12}$ K). Some signatures in the hadron multiplicity distributions may be Unruh-related.

**Bell-Leinaas proposal** (1987): Circular electron storage rings — the centripetal acceleration could slightly depolarize the beam through the Unruh effect. Marginal evidence at $10^{-2}$ level, not conclusive.

The direct detection of the Unruh effect remains an open experimental challenge.

---

## Connection to Hawking Radiation

The Unruh effect and Hawking radiation are identical in structure:

| | Unruh Effect | Hawking Radiation |
|---|---|---|
| Spacetime | Flat Minkowski | Black hole (Schwarzschild) |
| Horizon | Rindler horizon | Event horizon |
| Observer | Uniformly accelerating | Static at large $r$ |
| Temperature | $T_U = \hbar a/(2\pi ck_B)$ | $T_H = \hbar c^3/(8\pi GMk_B)$ |
| Entanglement | Between Rindler wedges | Between exterior and interior |
| Natural state | Minkowski vacuum | Hartle-Hawking state |

The surface gravity at the Schwarzschild horizon $\kappa = c^4/(4GM)$ plays the role of $a$: $T_H = \hbar\kappa/(2\pi ck_B)$.

The Unruh effect has an exact equivalence (via the equivalence principle): an observer accelerating in Minkowski space with acceleration $a$ is locally equivalent to an observer in a gravitational field with $g = a$. The Unruh temperature equals the Hawking temperature at the Rindler horizon, which is the same as the Hawking temperature at the Schwarzschild horizon in the appropriate limit.

This connection — that the Hawking effect is just the Unruh effect in disguise — was made explicit by Unruh himself and by Fulling, Davies, and others in the 1970s.

---

## Important Concepts

- **Unruh effect**: Uniformly accelerating observer sees Minkowski vacuum as thermal bath at $T_U = \hbar a/(2\pi ck_B)$
- **Rindler spacetime**: Flat Minkowski metric in coordinates adapted to uniformly accelerating observer; has Rindler horizon
- **Rindler horizon**: Causal boundary for uniformly accelerating observer; analogous to black hole event horizon
- **Bogoliubov mechanism**: Minkowski and Rindler mode functions related by $\beta\neq 0$ coefficients; gives thermal spectrum
- **Thermofield double**: Minkowski vacuum = entangled state of left/right Rindler modes; restricted observer sees thermal state
- **Tracing out**: Thermal character arises from entanglement + inability to access the other side of the horizon
- **Unruh-DeWitt detector**: Concrete model; accelerating detector excitation rate equals thermal Planck distribution
- **Equivalence principle connection**: Unruh + equivalence principle → Hawking; both follow from $T = \hbar\kappa/(2\pi ck_B)$
- **Experimental inaccessibility**: $T_U \sim 10^{-23}$ K per m/s$^2$; requires enormous accelerations for observable effects

---

## Important Figures

**William Unruh** (1945–): Discovered the Unruh effect (1976); also pioneered the acoustic black hole (dumb hole) as an analogue for Hawking radiation. 

**Paul Davies** (1946–) and **Stephen Fulling** (1945–): Independently discovered related aspects of the Rindler vacuum and its thermal properties (1975–1976).

**Werner Israel** (1931–2022): Derived the thermofield double decomposition of the Schwarzschild eternal black hole (1976); connecting Hawking radiation to entanglement.

**Bryce DeWitt** (1923–2004): Introduced the Unruh-DeWitt detector model; systematic treatment of acceleration radiation.

---

## Further Reading

**Primary Sources**
- Unruh, W.G. (1976). "Notes on Black-Hole Evaporation." *Phys. Rev. D*, 14, 870.
- Davies, P.C.W. (1975). "Scalar Production in Schwarzschild and Rindler Metrics." *J. Phys. A*, 8, 609.
- Fulling, S.A. (1973). "Nonuniqueness of Canonical Field Quantization in Riemannian Space-Time." *Phys. Rev. D*, 7, 2850.
- Israel, W. (1976). "Thermo-Field Dynamics of Black Holes." *Physics Letters A*, 57, 107.

**Reviews**
- Crispino, L.C.B., Higuchi, A., & Matsas, G.E.A. (2008). "The Unruh Effect and Its Applications." *Rev. Mod. Phys.*, 80, 787. — Comprehensive review with applications.
- Earman, J. (2011). "The Unruh Effect for Philosophers." *Studies in History and Philosophy of Modern Physics*, 42, 81.

---

## Exercises

**56.1.** *Rindler coordinates.*

(a) Verify that the Rindler worldline $x^2 - c^2t^2 = c^4/a^2$, $y = z = 0$ has constant proper acceleration $a$. Compute $d^2x^\mu/d\tau^2$ and take the norm.

(b) Show that the Rindler horizon $x = c|t|$ is at Rindler coordinate $\xi\to -\infty$. What does this mean for the accelerating observer?

(c) Draw a Minkowski diagram showing: the accelerating observer's worldline, the Rindler horizon, and the left/right Rindler wedges. Mark which region the accelerating observer can send/receive signals to/from.

---

**56.2.** *Thermal spectrum from Bogoliubov coefficients.*

The ratio $|\beta_\Omega|^2/|\alpha_\Omega|^2 = e^{-2\pi\Omega c/a}$ (from the mode matching calculation). 

(a) Using the Bogoliubov condition $|\alpha_\Omega|^2 - |\beta_\Omega|^2 = 1$, solve for $|\beta_\Omega|^2$.

(b) Show $|\beta_\Omega|^2 = 1/(e^{2\pi\Omega c/a} - 1)$ — the Bose-Einstein distribution at temperature $T_U$.

(c) For fermions, the Bogoliubov condition is $|\alpha_\Omega|^2 + |\beta_\Omega|^2 = 1$ (anticommutation). Repeat: show $|\beta_\Omega|^2 = 1/(e^{2\pi\Omega c/a} + 1)$ — the Fermi-Dirac distribution at temperature $T_U$.

---

**56.3.** *The Unruh temperature and Hawking radiation.*

(a) An electron in a synchrotron radiation experiment undergoes centripetal acceleration $a = v^2/R$. For the LEP electron storage ring ($R = 3.1$ km, $v \approx c$), compute $T_U$. Is this measurable?

(b) For a Schwarzschild black hole, the surface gravity is $\kappa = c^4/(4GM)$. Express $T_H = \hbar\kappa/(2\pi ck_B)$ in Kelvin as a function of $M/M_\odot$. For $M = 10M_\odot$: what is $T_H$?

(c) The "local" Hawking temperature at radius $r$ outside a Schwarzschild black hole (the temperature seen by a static ZAMO observer): $T_{\rm loc}(r) = T_H/\sqrt{1-r_s/r}$ (Tolman factor). As $r\to r_s$: $T_{\rm loc}\to\infty$. This divergence corresponds to the Unruh temperature for the acceleration needed to remain static at $r_s$. Compute this acceleration at $r = r_s + \varepsilon$ and verify it equals $T_{\rm loc}/T_U = c^2/(2r_s)\cdot 1/\sqrt{1-r_s/r}$.

---

**Thought Experiment T56.1.** *Heat from nothing?*

The Unruh effect says: an accelerating observer sees thermal radiation where an inertial observer sees vacuum. But energy is conserved — where does the energy come from?

The answer involves the work done by the accelerating agent. The external force maintaining the constant acceleration does work that is radiated away as Unruh radiation (from the inertial observer's perspective, this is Larmor radiation from an accelerating charge, or its gravitational analogue). The energy is not "created from nothing" — it comes from the energy source maintaining the acceleration.

But now consider: the accelerating observer sees thermal photons and absorbs one, gaining energy. From the inertial frame, this corresponds to the agent doing work. But the agent was accelerating uniformly — how does the absorption of one quantum in the Rindler frame affect the work done by the accelerating agent in the inertial frame?

This puzzle — the consistent description of a single quantum event from two frames — is at the heart of what makes the Unruh effect conceptually difficult. Work through the energy accounting carefully. Does the equivalence principle save you?
