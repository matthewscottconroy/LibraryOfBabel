# Chapter 50: Cosmic Inflation

---

## Chapter Introduction

Three puzzles about the initial conditions of the Big Bang demand explanation. The **horizon problem**: why is the CMB so uniform ($\Delta T/T \sim 10^{-5}$) across the sky, when regions on opposite sides could never have been in causal contact in the standard hot Big Bang model? The **flatness problem**: why is $\Omega_k$ so close to zero today when deviations from flatness grow with time — requiring extraordinary fine-tuning at the Planck epoch? The **relic problem**: Grand Unified Theory (GUT) phase transitions at $T \sim 10^{16}$ GeV should produce magnetic monopoles ($\sim 1$ per Hubble volume), but none have been observed.

In 1980–81, Alan Guth proposed a solution: a brief period of **exponential expansion** ("inflation") in the early universe, driven by the potential energy of a scalar field. During inflation, the scale factor grows by $e^{60}$–$e^{100}$ or more in $\sim 10^{-32}$ seconds. This:
1. Erases the horizon problem: our entire observable universe inflated from a region small enough to have been in causal contact
2. Solves the flatness problem: inflation drives $\Omega_k \to 0$ exponentially
3. Dilutes relics: monopoles and other dangerous relics are diluted to negligible density

But inflation's greatest triumph is not solving these problems — it is its **prediction of primordial perturbations**. Quantum fluctuations of the inflaton field, inflated to cosmic scales, seed the primordial density perturbations that grow into the CMB anisotropies and large-scale structure we observe. The predicted spectrum: slightly tilted from scale-invariance ($n_s = 0.96$), nearly Gaussian, with a characteristic amplitude fixed by the inflaton potential. All of these were confirmed by the CMB observations.

This chapter derives the inflationary solution to the three puzzles, introduces the slow-roll scalar field dynamics, and derives the power spectrum of quantum fluctuations.

---

## The Three Problems Quantified

**The horizon problem.** The comoving Hubble radius $\mathcal{H}^{-1} = (aH)^{-1}$ is the characteristic scale of causal contact at time $t$. In the standard hot Big Bang (matter + radiation):
$$\mathcal{H}^{-1} \propto \frac{1}{aH} \propto \begin{cases}a & \text{matter domination}\\ a^{1/2} & \text{radiation domination}\end{cases}$$

The comoving Hubble radius is **increasing** in the standard cosmology. This means that the comoving scale of the particle horizon at recombination was much smaller than the horizon today. Quantitatively: the CMB at $z_* = 1100$ had a particle horizon $\sim 200$ Mpc comoving, while our current horizon is $\sim 46,500$ Mpc. The CMB was divided into $\sim (46500/200)^2 \approx 50,000$ causally disconnected patches — yet they all have the same temperature to $10^{-5}$. This is the horizon problem.

**The flatness problem.** The first Friedmann equation can be written:
$$|\Omega - 1| = \frac{|kc^2|}{a^2 H^2} \propto \frac{1}{\mathcal{H}^2}$$

If $\mathcal{H}^{-1}$ increases (standard cosmology), $|\Omega - 1|$ grows. Having $|\Omega_k| < 0.002$ today requires $|\Omega - 1| < 10^{-60}$ at the Planck time. An extraordinary fine-tuning.

**Relic problem.** GUT magnetic monopoles, cosmic strings, and domain walls are produced at phase transitions in the early universe. Their density scales as $\rho_{\rm relic} \propto a^{-3}$ (like matter), while radiation scales as $a^{-4}$. At $T_{\rm GUT} \sim 10^{15}$ GeV, one monopole is produced per Hubble volume. By today, their energy density would exceed the critical density by $\sim 10^{14}$ — the universe would have recollapsed long ago.

---

## Inflation: Exponential Expansion

Inflation is defined by $\ddot{a} > 0$ — accelerating expansion. From the second Friedmann equation:
$$\frac{\ddot{a}}{a} = -\frac{4\pi G}{3}\left(\rho + \frac{3p}{c^2}\right) + \frac{\Lambda c^2}{3} > 0$$

requires $\rho + 3p/c^2 < 0$, i.e., $w < -1/3$. A scalar field $\phi$ with potential $V(\phi)$ has:
$$\rho_\phi = \frac{\dot{\phi}^2}{2} + V(\phi), \quad p_\phi = \frac{\dot{\phi}^2}{2} - V(\phi)$$

If potential energy dominates ($\dot{\phi}^2 \ll V$), then $p_\phi \approx -\rho_\phi c^2$ — a cosmological-constant-like equation of state. The Friedmann equation becomes:
$$H^2 \approx \frac{8\pi G}{3}V(\phi) = \text{const}$$

giving exponential expansion $a \propto e^{Ht}$ — de Sitter spacetime.

**Solving the problems:**

*Horizon*: The comoving Hubble radius $(aH)^{-1} \propto e^{-Ht}$ **decreases** during inflation. A region that was causally connected before inflation gets inflated to a scale much larger than the Hubble horizon. Our entire observable universe emerged from a tiny patch of pre-inflationary spacetime — much smaller than the Hubble radius at that time.

*Flatness*: $|\Omega - 1| \propto (aH)^{-2} \propto e^{-2Ht} \to 0$ exponentially during inflation. 60 e-folds of inflation reduces $|\Omega - 1|$ by $e^{120}$ — more than enough to explain the observed flatness.

*Relics*: GUT relics produced before inflation are diluted by $e^{3N}$ where $N$ is the number of e-folds. For $N = 60$: dilution by $e^{180}$ — any relics produced before inflation are completely negligible.

---

## Slow-Roll Inflation

The inflaton field $\phi$ obeys the Klein-Gordon equation in an FRW background:
$$\ddot{\phi} + 3H\dot{\phi} + V'(\phi) = 0$$

where $V' = dV/d\phi$. The $3H\dot{\phi}$ term is "Hubble friction" — the expanding universe damps the motion.

**The slow-roll approximation**: If $\dot{\phi}^2 \ll V(\phi)$ and $|\ddot{\phi}| \ll |3H\dot{\phi}|$:
$$3H\dot{\phi} \approx -V'(\phi) \implies \dot{\phi} \approx -\frac{V'}{3H}$$
$$H^2 \approx \frac{8\pi G}{3}V$$

These are the slow-roll equations. They are valid when the slow-roll parameters are small:
$$\epsilon \equiv -\frac{\dot{H}}{H^2} = \frac{M_{\rm Pl}^2}{2}\left(\frac{V'}{V}\right)^2 \ll 1$$
$$\eta \equiv \frac{\dot{\epsilon}}{\epsilon H} = M_{\rm Pl}^2\frac{V''}{V} \ll 1$$

(using the reduced Planck mass $M_{\rm Pl} = (8\pi G)^{-1/2}$). Inflation continues as long as $\epsilon < 1$. It ends when $\epsilon \approx 1$, after which the inflaton oscillates around its potential minimum and decays, reheating the universe ("reheating").

**Number of e-folds:**
$$N = \int_{t_{\rm start}}^{t_{\rm end}}H\,dt = \int_{\phi_{\rm end}}^{\phi_{\rm start}}\frac{H}{\dot{\phi}}d\phi \approx \frac{1}{M_{\rm Pl}^2}\int_{\phi_{\rm end}}^{\phi_{\rm start}}\frac{V}{V'}d\phi$$

For successful inflation solving the horizon and flatness problems: $N \gtrsim 60$.

**Example: Chaotic inflation.** For $V(\phi) = \frac{1}{2}m^2\phi^2$ (large-field inflation):
$$\epsilon = \frac{2M_{\rm Pl}^2}{\phi^2}, \quad \eta = \frac{2M_{\rm Pl}^2}{\phi^2} = \epsilon$$

Inflation ends when $\epsilon = 1$: $\phi_{\rm end} = \sqrt{2}M_{\rm Pl}$. For $N = 60$ e-folds: $\phi_{\rm start} \approx \sqrt{4\times 60 + 2}M_{\rm Pl} \approx 15.5 M_{\rm Pl}$. The inflaton must roll over super-Planckian field values. This is now constrained by the tensor-to-scalar ratio $r = 16\epsilon \approx 0.13$ — in tension with CMB upper limits ($r < 0.036$), and so $\phi^2$ inflation is disfavored.

---

## Quantum Fluctuations and the Origin of Structure

The most remarkable prediction of inflation is that quantum vacuum fluctuations in the inflaton field — stretched to macroscopic scales by the expansion — seed the primordial density perturbations.

**Quantum fluctuations in de Sitter space.** A massless scalar field in de Sitter space has fluctuations:
$$\langle|\delta\phi_k|^2\rangle = \frac{H^2}{2k^3}$$

(derived from quantizing the inflaton in de Sitter background). When a mode with comoving wavenumber $k$ exits the Hubble horizon during inflation ($k = aH$), its amplitude freezes out at:
$$|\delta\phi_k| = \frac{H}{2\pi}$$

**Curvature perturbation.** The gauge-invariant curvature perturbation $\mathcal{R}_k$ is related to inflaton fluctuations by $\mathcal{R}_k = H\delta\phi_k/\dot{\phi}$. Its power spectrum:
$$\mathcal{P}_\mathcal{R}(k) = \frac{k^3}{2\pi^2}|\mathcal{R}_k|^2 = \frac{H^2}{8\pi^2 M_{\rm Pl}^2\epsilon}\bigg|_{k=aH}$$

Since $H$ and $\epsilon$ vary slowly during inflation, different modes exit with slightly different amplitudes. The spectral index:
$$n_s - 1 \equiv \frac{d\ln\mathcal{P}_\mathcal{R}}{d\ln k} = -2\epsilon - \eta$$

For slow-roll inflation: $n_s \approx 1 - 2\epsilon - \eta < 1$. The red tilt (power slightly decreasing with $k$) arises because modes that exit earlier see a slightly higher $H$ (inflation hasn't slowed yet).

**Gravitational waves.** Tensor perturbations (gravitational waves) from inflation have power spectrum:
$$\mathcal{P}_T(k) = \frac{2H^2}{\pi^2 M_{\rm Pl}^2}\bigg|_{k=aH}$$

The tensor-to-scalar ratio:
$$r \equiv \frac{\mathcal{P}_T}{\mathcal{P}_\mathcal{R}} = 16\epsilon$$

**Observations.** Planck 2018 measures: $A_s = 2.1\times 10^{-9}$ (amplitude), $n_s = 0.9649 \pm 0.0042$ (spectral index, $>7\sigma$ from scale-invariance). The measured $n_s < 1$ is a direct confirmation of slow-roll inflation: the near-scale-invariance arises from near-exponential expansion, and the departure from exact scale-invariance arises from the slow evolution of $H$ and $\epsilon$.

**Inflation predicts a nearly Gaussian, nearly scale-invariant spectrum** — confirmed to extraordinary precision. No other known mechanism produces primordial perturbations with these properties.

---

## The Inflationary Zoo: Models and Constraints

There are hundreds of inflationary models, constrained by the $n_s$-$r$ plane:

| Model | $n_s$ | $r$ | Status |
|-------|--------|-----|--------|
| $V = \phi^2$ (chaotic) | $1 - 2/N$ | $8/N$ | Disfavored ($r \approx 0.13$) |
| $V = \phi^4$ | $1 - 3/N$ | $16/N$ | Disfavored ($r \approx 0.27$) |
| Starobinsky ($R^2$) | $1 - 2/N$ | $12/N^2$ | Favored ($r \approx 0.003$) |
| Natural inflation | $1 - (2/N)(1+r_\Lambda)$ | small | Compatible |
| Hilltop | $1 - 2/N$ | small | Compatible |

The Starobinsky model $S = \int\sqrt{-g}(R + R^2/(6M^2))d^4x$, the first inflationary model (Starobinsky 1980), predicts $n_s = 1 - 2/N \approx 0.967$ and $r = 12/N^2 \approx 0.003$ — right in the sweet spot of current observational constraints.

---

## Important Concepts

**E-folds**: $N = \int H\,dt = \ln(a_{\rm end}/a_{\rm start})$. The observable universe requires $N \gtrsim 60$. The scale factor grows by $e^{60}$–$e^{100}$.

**Reheating**: After inflation ends, the inflaton oscillates and decays into Standard Model particles, thermalizing the universe and producing the hot plasma of the Hot Big Bang. The reheating temperature $T_{\rm reh}$ determines which relics were produced.

**The measure problem**: Inflation produces exponentially many regions ("pocket universes"). In eternal inflation, inflation never ends globally — only locally. Comparing probabilities of different observations in an infinite multiverse requires a "measure" — a way of counting. This is unsolved.

**Primordial non-Gaussianity**: Single-field slow-roll inflation predicts $f_{\rm NL} \ll 1$ (nearly Gaussian perturbations). Deviations signal multi-field inflation, non-canonical kinetic terms, or other departures from the simplest scenario. Current CMB constraints: $f_{\rm NL}^{\rm local} = -0.9 \pm 5.1$ (Planck) — consistent with zero.

---

## Important Figures

**Alan Guth (born 1947)**: Proposed inflationary cosmology in December 1980 (published 1981). Recognized that exponential expansion in the early universe could solve the horizon, flatness, and monopole problems. Guth's original "old inflation" model involved a first-order phase transition and had the "graceful exit" problem (the universe wouldn't reheat uniformly). The fix came from Linde and Albrecht-Steinhardt's "new inflation."

**Andrei Linde (born 1948)**: Developed "new inflation" (1982, slow-roll mechanism) and "chaotic inflation" (1983, large-field inflation with $V = m^2\phi^2$). Also developed eternal inflation (some regions of the universe inflate forever, creating a fractal multiverse) and the concept of the inflationary landscape. The most prolific theorist of the inflationary epoch.

**Paul Steinhardt (born 1952)**: Co-developed new inflation with Albrecht (1982). Later became a critic of inflation and proposed alternatives (ekpyrosis, cyclic universe). The ongoing debate between Steinhardt and Linde about the testability and status of inflationary cosmology is instructive about how science evaluates untestable theories.

**Alexei Starobinsky (1948–2023)**: Developed the $R^2$ inflation model in 1980 — predating Guth but without recognizing the solution to the horizon problem. His model is currently the best-fitting single inflationary model to CMB data. Also developed the Harrison-Zel'dovich-Mukhanov theory of inflationary perturbations.

**Viatcheslav Mukhanov (born 1956)**: Developed (with Chibisov) the quantum theory of inflationary perturbations in 1981–82, predicting the nearly scale-invariant power spectrum. Also wrote the standard textbook on inflation.

---

## Further Reading

**Guth, A.H. (1981). "Inflationary Universe: A Possible Solution to the Horizon and Flatness Problems." *Physical Review D*, 23, 347.**
The original inflation paper.

**Linde, A.D. (1982). "A New Inflationary Universe Scenario." *Physics Letters B*, 108, 389.**
Slow-roll inflation ("new inflation").

**Mukhanov, V.F. and Chibisov, G.V. (1981). "Quantum Fluctuation and 'Nonsingular' Universe." *JETP Letters*, 33, 532.**
The first derivation of the inflationary power spectrum.

**Starobinsky, A.A. (1980). "A New Type of Isotropic Cosmological Models Without Singularity." *Physics Letters B*, 91, 99.**
The $R^2$ inflation model.

**Planck Collaboration (2020). "Planck 2018 Results X: Constraints on Inflation." *Astronomy & Astrophysics*, 641, A10.**
Current observational constraints on inflationary models from CMB.

**Baumann, D. (2022). *Cosmology.* Cambridge University Press.**
Chapters 8–9: the best modern pedagogical treatment of inflation and its observational signatures.

**Mukhanov, V. (2005). *Physical Foundations of Cosmology.* Cambridge University Press.**
The authoritative textbook on inflation and structure formation by one of the field's founders.

---

## Exercises

**50.1.** *Inflation and the horizon problem.*

(a) Show that the comoving Hubble radius $(aH)^{-1} = (d\ln a/dt)^{-1}/a$ decreases during inflation ($\ddot{a} > 0$) and increases during radiation and matter domination.

(b) The minimum number of e-folds $N$ required to solve the horizon problem is approximately the number of e-folds needed for the comoving Hubble radius at the start of inflation to exceed the current comoving Hubble radius: $(aH)^{-1}\big|_{\rm start} > (a_0 H_0)^{-1}$.

Show that $N > \ln(a_{\rm end}/a_0) + \ln(H_{\rm end}/H_0) \approx 60$.

(c) Why does $N = 60$ solve the horizon problem but not $N = 10$?

---

**50.2.** *Slow-roll dynamics for $V = V_0(1 - \phi^2/\mu^2)$ (hilltop inflation).*

(a) Compute $\epsilon(\phi)$ and $\eta(\phi)$ in the slow-roll approximation.

(b) Show that slow-roll ($\epsilon, |\eta| < 1$) requires $\phi$ to be near the top of the potential ($\phi \ll \mu$).

(c) Compute $n_s = 1 - 2\epsilon - \eta$ as a function of $\phi_*$ (the field value when the pivot scale exits the horizon). For $N = 60$ e-folds, what value of $\phi_*$ is required?

(d) For $\mu = M_{\rm Pl}$ (Planck scale), compute $r$ and show that hilltop inflation predicts $r \ll 0.1$, in agreement with current bounds.

---

**50.3.** *The power spectrum of inflationary perturbations.*

(a) In de Sitter space ($H = \text{const}$), the mode function for a massless scalar field satisfies:
$$u_k'' + \left(k^2 - \frac{2}{\eta^2}\right)u_k = 0$$
(using conformal time $\eta = -1/(aH)$). Show that the solution with Bunch-Davies initial conditions is:
$$u_k(\eta) = \frac{e^{-ik\eta}}{\sqrt{2k}}\left(1 - \frac{i}{k\eta}\right)$$

(b) The power spectrum of $\delta\phi = u_k/a$ in the super-horizon limit ($k\eta \to 0$) is:
$$\mathcal{P}_{\delta\phi}(k) = \frac{k^3}{2\pi^2}\frac{|u_k|^2}{a^2}\bigg|_{k\eta\to 0} = \frac{H^2}{4\pi^2}$$

Verify this result.

(c) The curvature perturbation power spectrum is $\mathcal{P}_\mathcal{R} = H^2/(8\pi^2 M_{\rm Pl}^2\epsilon)$. The Planck-normalized amplitude is $A_s = \mathcal{P}_\mathcal{R}(k_*) = 2.1\times 10^{-9}$ at $k_* = 0.05$ Mpc$^{-1}$. For $\epsilon = 0.01$, compute $H_*$ at horizon crossing. Express as a fraction of $M_{\rm Pl}$.

---

**Thought Experiment T50.1.** *Is inflation a scientific theory?*

Inflation solves the horizon, flatness, and monopole problems. It predicts $n_s < 1$ and gravitational waves with $r = 16\epsilon$. But inflation is "eternal" in most models — the multiverse is infinite, and different pocket universes can have different values of the cosmological constant, the inflationary potential, and possibly even the laws of physics.

In an infinite multiverse, any observation is possible with nonzero probability. Can inflation then be falsified? What would it take to rule out inflation? Is a prediction like $n_s = 0.96$ with 10% accuracy (as current observations achieve) enough to make inflation a scientific theory? Or does the multiverse swamp any definite prediction?

**Thought Experiment T50.2.** *Before the Big Bang?*

Inflation has a beginning — it requires special initial conditions (a region of space with sufficiently smooth inflaton field and low kinetic energy). What preceded inflation? The Borde-Guth-Vilenkin theorem (2003) proves that any spacetime with $\langle H\rangle > 0$ along any past-directed geodesic is geodesically past-incomplete — inflation cannot be past-eternal. The universe had a beginning.

What does it mean to ask "what was before the beginning"? Is this a meaningful physical question? Proposals include: quantum creation of the universe from nothing (Vilenkin), the Hartle-Hawking no-boundary proposal, a bouncing cosmology (loop quantum cosmology), a cyclic universe, or just accepting that time began at the Big Bang. Can any of these be observationally tested?
