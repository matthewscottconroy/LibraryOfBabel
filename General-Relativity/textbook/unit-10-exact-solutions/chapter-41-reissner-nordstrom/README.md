# Chapter 41: The Reissner-Nordström Solution

---

## Chapter Introduction

The Schwarzschild solution describes an uncharged, non-rotating black hole — the simplest possible case. In the real universe, black holes are almost certainly rotating (stars that collapse generally carry angular momentum), and in principle they could carry electric charge (though in practice, charge is rapidly neutralized by accretion of opposite charges from the plasma environment). Nevertheless, the charged black hole — described by the **Reissner-Nordström (RN) solution** — is theoretically important for several reasons.

The RN solution introduces a qualitatively new feature absent from Schwarzschild: the possibility of two event horizons and an inner **Cauchy horizon** — a surface where determinism breaks down. The structure of the maximal Reissner-Nordström spacetime is far richer than Schwarzschild: it admits extensions to an infinite chain of exterior and interior regions. It also introduces the concept of an **extremal black hole** — a black hole with the maximum possible charge-to-mass ratio — which is important in string theory and the study of black hole entropy.

The RN solution is also the gateway to understanding the Kerr-Newman solution (rotating and charged) and to the **no-hair theorem**: a black hole in equilibrium is completely characterized by just three parameters — mass $M$, angular momentum $J$, and charge $Q$.

---

## The Reissner-Nordström Metric

The unique spherically symmetric, charged, asymptotically flat solution to the Einstein-Maxwell equations:

$$ds^2 = -\Delta(r)\frac{c^2dt^2}{r^2} + \frac{r^2 dr^2}{\Delta(r)} + r^2d\Omega^2, \quad \Delta(r) = r^2 - r_s r + r_Q^2$$

Or equivalently:
$$ds^2 = -f(r)c^2dt^2 + \frac{dr^2}{f(r)} + r^2d\Omega^2, \quad f(r) = 1 - \frac{r_s}{r} + \frac{r_Q^2}{r^2}$$

where:
- $r_s = 2GM/c^2$ (Schwarzschild radius as before)
- $r_Q^2 = GQ^2/(4\pi\varepsilon_0 c^4) = GQ^2k_e/c^4$ where $k_e = 1/(4\pi\varepsilon_0)$ is Coulomb's constant

The electromagnetic field: $F_{\mu\nu}$ is that of a radial electric field, $\mathbf{E} = Q\hat{r}/(4\pi\varepsilon_0 r^2)$.

**Condition for charge to affect the metric**: $r_Q^2 = GQ^2k_e/c^4$. For an electron: $r_Q = \sqrt{Gk_ee^2}/c^2 \approx 1.4\times 10^{-36}$ m — completely negligible. For the metric to be significantly affected by charge, the charge-to-mass ratio would need to be enormously large: $Q/M \sim c^2/\sqrt{Gk_e} \approx 10^{18}$ C/kg — far beyond any macroscopic object.

---

## The Three Cases

The function $f(r) = 1 - r_s/r + r_Q^2/r^2$ has zeros at:
$$r_\pm = \frac{r_s \pm\sqrt{r_s^2 - 4r_Q^2}}{2} = \frac{GM}{c^2}\pm\sqrt{\frac{G^2M^2}{c^4} - \frac{Gk_eQ^2}{c^4}}$$

Three cases depending on the discriminant $r_s^2 - 4r_Q^2 = 4G(G M^2 - k_e Q^2)/c^4$:

**Case 1: $GM^2 > k_e Q^2$ (under-extremal, $|Q| < Q_{\rm ext}$)**: Two real roots $r_+ > r_-$:
- $r_+$: outer event horizon — analogous to Schwarzschild
- $r_-$: inner Cauchy horizon — a new feature; determinism breaks down here

**Case 2: $GM^2 = k_e Q^2$ (extremal, $|Q| = Q_{\rm ext}$)**: Double root $r_+ = r_- = r_s/2 = GM/c^2$:
- Single degenerate horizon
- Surface gravity $\kappa = 0$ — extremal BH has zero Hawking temperature
- $T = 0$ is the "third law of black hole mechanics"

**Case 3: $GM^2 < k_e Q^2$ (over-extremal, $|Q| > Q_{\rm ext}$)**: No real roots:
- $f(r) > 0$ everywhere (except $r = 0$)
- No horizon — the singularity at $r = 0$ is **naked**
- Apparent violation of cosmic censorship; likely unphysical (cannot be formed from regular initial data)

The **extremal condition**: $Q_{\rm ext} = M\sqrt{G/(k_e)} = Mc^2/\sqrt{k_e G}$. For the extremal Reissner-Nordström: the gravitational attraction exactly balances the electrostatic repulsion. Two extremal RN black holes with the same sign of charge can be in static equilibrium at any separation (Majumdar-Papapetrou solution).

---

## The Cauchy Horizon

The **Cauchy horizon** at $r = r_-$ is a genuinely new and troubling feature. An observer who falls into the outer horizon at $r = r_+$ and continues inward reaches $r = r_-$ in finite proper time. At $r = r_-$:

- The determinism of the Cauchy problem breaks down: the domain of dependence of any initial Cauchy surface does not extend beyond $r = r_-$
- The entire "future history of the outside universe" is visible — compressed into a finite affine parameter. The blueshift at the inner horizon is infinite: infalling radiation is infinitely blueshifted
- Perturbations are infinitely amplified at the Cauchy horizon

**Strong Cosmic Censorship** (Penrose): The Cauchy horizon is unstable. Any generic perturbation produces an infinite blueshift and turns $r = r_-$ into a (weak) singularity, making the spacetime inextendible and restoring determinism.

**Recent results** (Dafermos-Luk 2017): The singularity at the inner horizon is indeed generic under small perturbations, but it is a *weak* singularity — observers can pass through it (the metric is continuous, though not differentiable). This has reopened the debate about the status of SCCC.

---

## The Maximal Extension

The maximally extended RN spacetime (for $|Q| < Q_{\rm ext}$) has an infinite ladder structure: an alternating sequence of exterior regions (like Region I of Schwarzschild), interior regions (between $r_+$ and $r_-$), and timelike singularities ($r = 0$).

Key features:
- The singularity at $r = 0$ is **timelike** (unlike Schwarzschild's spacelike singularity) — in principle, observers can avoid it by maneuvering inside the horizon
- The inner horizon ($r = r_-$) is a Cauchy horizon — instabilities develop there
- Multiple asymptotic regions exist (like Schwarzschild's Regions I and IV) but there are infinitely many of them

The Penrose diagram of the maximal RN spacetime resembles an infinite "diamond" chain: exterior → between horizons → interior → between horizons → exterior → ...

---

## Extremal Black Holes and String Theory

The extremal Reissner-Nordström black hole ($|Q| = Q_{\rm ext}$) has several special properties:

- **Zero Hawking temperature**: $T_H = \hbar\kappa/(2\pi ck_B) = 0$ (since $\kappa = 0$)
- **Non-zero entropy**: $S_{\rm BH} = k_B A/(4\ell_P^2) = \pi k_B r_+^2/\ell_P^2\neq 0$
- **Near-horizon geometry**: $AdS_2\times S^2$ — a product of two-dimensional anti-de Sitter space with a two-sphere. This arises as $r\to r_+$ for the extremal metric.
- **Supersymmetric BPS states**: In supergravity, extremal BHs saturate the Bogomol'nyi-Prasad-Sommerfield (BPS) bound and are stable — their mass equals their charge (in natural units). They are the stringy "D-branes" of type II string theory.

**Strominger-Vafa entropy count** (1996): For specific extremal BHs in string theory, the microscopic entropy (counting D-brane microstates) exactly matches the Bekenstein-Hawking macroscopic entropy $S = A/(4\ell_P^2)$. This was the first microscopic derivation of black hole entropy — a major success of string theory.

---

## The No-Hair Theorem

The **no-hair theorem** (or black hole uniqueness theorem): A stationary, asymptotically flat black hole in electrovacuum (Einstein-Maxwell equations) is uniquely specified by three parameters: mass $M$, angular momentum $J$, and electric charge $Q$.

- Schwarzschild: $J = 0$, $Q = 0$
- Reissner-Nordström: $J = 0$, $Q\neq 0$
- Kerr: $J\neq 0$, $Q = 0$
- Kerr-Newman: $J\neq 0$, $Q\neq 0$

No other "hair" (no other parameters, such as the star's internal structure, magnetic field configuration, etc.) survives the collapse to a black hole. All the information about how the black hole formed is hidden behind the event horizon or radiated away as gravitational and electromagnetic waves.

**Proof**: The uniqueness theorems were proven by Israel (1967, Schwarzschild), Carter (1971, Kerr-Newman), Robinson (1975, Kerr), and others. They use the Ernst potential formulation of stationary axisymmetric solutions and rely on specific energy identities.

**Caveats**: The no-hair theorem applies to stationary (equilibrium) black holes in Einstein-Maxwell theory with no other fields. Modified gravity theories (scalar-tensor, $f(R)$) may have black hole "hair" — a scalar field profile outside the horizon. Neutron stars, which are not black holes, certainly carry information about their nuclear equation of state.

---

## Important Concepts

- **Reissner-Nordström metric**: Charged, spherically symmetric black hole; $f(r) = 1 - r_s/r + r_Q^2/r^2$
- **Outer event horizon** $r_+$: Analogous to Schwarzschild $r_s$; causal boundary
- **Inner Cauchy horizon** $r_-$: Determinism breaks down; unstable under perturbations
- **Three cases**: Subextremal ($r_+ > r_-$), extremal ($r_+ = r_-$), naked singularity ($r_+ = r_- =$ none)
- **Extremal black hole**: $|Q| = Q_{\rm ext}$; $\kappa = 0$; $T_H = 0$; $S_{\rm BH}\neq 0$; near-horizon $AdS_2\times S^2$
- **Timelike singularity**: $r = 0$ in RN is timelike; avoidable (in principle) unlike Schwarzschild's spacelike singularity
- **Strong Cosmic Censorship**: Cauchy horizon unstable; perturbations make it singular; restores determinism
- **No-hair theorem**: Stationary electrovacuum BH characterized by $(M, J, Q)$ only

---

## Further Reading

**Primary Sources**
- Reissner, H. (1916). "Über die Eigengravitation des elektrischen Feldes nach der Einsteinschen Theorie." *Annalen der Physik*, 50, 106.
- Nordström, G. (1918). "On the Energy of the Gravitational Field in Einstein's Theory." *Verhandl. Koninkl. Ned. Akad. Wetenschap.*, 26, 1201.
- Carter, B. (1968). "Hamilton-Jacobi and Schrödinger Separable Solutions of Einstein's Equations." *Comm. Math. Phys.*, 10, 280.
- Strominger, A. & Vafa, C. (1996). "Microscopic Origin of the Bekenstein-Hawking Entropy." *Phys. Lett. B*, 379, 99.

**Textbooks**
- Wald, R.M. (1984). *General Relativity*. Chapter 12 on black hole uniqueness.
- Chandrasekhar, S. (1983). *The Mathematical Theory of Black Holes*. Oxford.
- Carroll, S.M. (2004). *Spacetime and Geometry*. Addison-Wesley. — Section 7.5.

---

## Exercises

**41.1.** *Horizon structure.*

(a) For a Reissner-Nordström black hole with $M = 10M_\odot$ and $Q = 10^{10}$ C: compute $r_s$, $r_Q$, and $r_\pm$. Is this subextremal or extremal or superextremal?

(b) Prove that for the extremal case $r_+ = r_-$: the surface gravity $\kappa = c^2f'(r_+)/2$ (where $f'$ is the derivative of $f$ with respect to $r$) is zero.

(c) For a near-extremal BH with $|Q| = (1-\varepsilon)Q_{\rm ext}$: show $r_+ - r_- \propto\sqrt{\varepsilon}$. How do the two horizons merge as $\varepsilon\to 0$?

---

**41.2.** *The Cauchy horizon and determinism.*

(a) In the Penrose diagram for subextremal RN: an observer falls through $r_+$ and reaches $r_-$. At $r_-$, they receive the entire future history of the outside universe in finite proper time. Explain why this leads to infinite blueshift.

(b) The blueshift at the inner horizon: a photon emitted at $t = t_0$ from outside $r_+$ arrives at the Cauchy horizon with frequency $\omega_{\rm obs}/\omega_{\rm em}\propto e^{2\kappa_- t_0}$ where $\kappa_-$ is the surface gravity of the inner horizon. For $t_0\to\infty$: what happens?

(c) How does this relate to Strong Cosmic Censorship? If every perturbation of the initial data is infinitely amplified at the Cauchy horizon, what does this imply for the smoothness of any extension through $r_-$?

---

**41.3.** *Extremal BH entropy.*

The Bekenstein-Hawking entropy $S = k_B A/(4\ell_P^2)$.

(a) For extremal RN: $A = 4\pi r_+^2$ with $r_+ = GM/c^2$. Express $S$ in terms of $M$ and fundamental constants. At what mass is $S = k_B$?

(b) The Strominger-Vafa derivation counts D-brane configurations. For a specific extremal 5D black hole, the number of microstates is $\Omega = e^{2\pi\sqrt{n_1 n_5 n_P}}$ where $n_i$ are quantized charges. The entropy is $S = k_B\ln\Omega = 2\pi k_B\sqrt{n_1 n_5 n_P}$. This equals the macroscopic $S = A/(4\ell_P^2)$ — this matching at strong coupling is the key result. Discuss why the matching is surprising (the microscopic count is done at weak coupling, the macroscopic formula applies at strong coupling).

(c) Does the Bekenstein-Hawking entropy formula $S = k_B A/(4\ell_P^2)$ apply to the extremal black hole? What is the thermodynamic significance of a system with $T = 0$ and $S\neq 0$ (i.e., a degenerate ground state)?

---

**Thought Experiment T41.1.** *The information paradox for RN black holes.*

A Reissner-Nordström black hole has a timelike singularity — observers can, in principle, avoid it. Consider an observer who falls through $r_+$, avoids the singularity, and passes through $r_-$ into a new exterior region (the extended spacetime).

This observer has effectively "escaped" from the black hole into a new universe. But they carry with them their memories — information about the original exterior universe. Does this resolve the information paradox, or create a new one?

The issues:
1. The Cauchy horizon at $r_-$ is unstable — the observer would be killed by infinite radiation before reaching $r_-$ (at least in classical perturbation theory)
2. Even if they survive, they cannot send information back to the original exterior — they are causally disconnected
3. The Hawking radiation that evaporates the original black hole must somehow carry all the information about the infalling matter — but if the matter passed into a new universe, how can the radiation encode it?

Is the RN "wormhole to a new universe" a loophole in the information paradox, or a red herring? What would you need to know about quantum gravity to answer this question?
