# Chapter 62: Modified Gravity

---

## Chapter Introduction

Einstein's general relativity is extraordinarily successful. It passes every experimental test with exquisite precision: the perihelion precession of Mercury, the deflection of light, the Shapiro delay, gravitational waves, black holes, the large-scale structure of the universe. And yet — the universe does not behave as GR predicts without introducing two mysterious ingredients that together constitute 95% of the energy content: dark matter and dark energy.

This situation motivates asking: what if GR is not the exact theory of gravity? What if at cosmological scales, or at very low accelerations, or at very high energies, the gravitational force law deviates from GR? What is the space of consistent theories that reduces to GR in the solar-system limit but differs elsewhere?

Modified gravity theories can be classified by what they modify:
- **The kinetic term**: $f(R)$ gravity, Gauss-Bonnet gravity, scalar-tensor theories (Brans-Dicke)
- **The propagating degrees of freedom**: massive gravity (graviton mass), tensor-scalar-vector theories
- **The screening mechanism**: Chameleon, Vainshtein, symmetron mechanisms that hide modifications in dense environments
- **The non-metric structure**: Torsion (Einstein-Cartan), non-metricity (metric-affine gravity)
- **Emergent gravity**: Gravity as a thermodynamic or entropic force, not a fundamental interaction

No modified gravity theory has successfully replaced dark matter and dark energy while passing all precision tests. But the exploration of modified gravity has deepened our understanding of GR itself — by showing what is special about it, what alternatives are possible, and what observations are needed to distinguish them.

---

## Lovelock's Theorem and Why GR Is Special

**Lovelock's theorem** (1971): In 4D, the only second-order divergence-free symmetric $(0,2)$-tensor constructed solely from the metric and its derivatives (no other fields) is:
$$G_{\mu\nu} + \Lambda g_{\mu\nu} = 8\pi G T_{\mu\nu}/c^4$$

(up to constants). In other words, **GR with cosmological constant is the unique second-order, generally covariant, local gravitational theory in 4 dimensions**.

Any modification of GR must therefore:
1. Introduce extra fields (scalar, vector, tensor)
2. Allow higher-derivative terms (fourth-order equations)
3. Break general covariance
4. Work in more than 4 dimensions (Kaluza-Klein, braneworld)
5. Allow non-local interactions

Lovelock's theorem is the "no-go" result that constrains all modifications of GR.

---

## $f(R)$ Theories

Replace the Hilbert Lagrangian $\mathcal{L} = \sqrt{-g}R$ with $\mathcal{L} = \sqrt{-g}f(R)$:
$$S = \frac{1}{16\pi G}\int f(R)\sqrt{-g}\,d^4x + S_{\rm matter}$$

**Field equations** (metric variation):
$$f'(R)R_{\mu\nu} - \frac{1}{2}f(R)g_{\mu\nu} - (\nabla_\mu\nabla_\nu - g_{\mu\nu}\Box)f'(R) = \frac{8\pi G}{c^4}T_{\mu\nu}$$

This is a fourth-order equation for $g_{\mu\nu}$. It is equivalent (by a conformal transformation) to GR + a scalar field (the "scalaron") $\varphi = f'(R)$ with a specific potential.

**Starobinsky model** $f(R) = R + R^2/(6M^2)$: The scalaron is the inflaton; this model predicts $n_s = 1 - 2/N^2\approx 0.965$ and $r = 12/N^2\approx 0.003$ (for $N = 60$ e-folds) — in excellent agreement with Planck data.

**Solar system constraints**: Any $f(R)$ model must agree with GR in the solar system. The scalaron mass $m_\varphi^2 = 1/(3f''(R))$ must be large in high-density environments — otherwise solar system tests are violated. The **chameleon mechanism** achieves this: $m_\varphi$ depends on the local density, becoming large (heavy) in dense environments and small (light) in low-density regions.

**Hu-Sawicki model** (popular for cosmology): $f(R) = R - m^2 c_1(R/m^2)^n/(c_2(R/m^2)^n + 1)$ with $n$, $c_1/c_2^2$ as free parameters. Passes solar system tests via chameleon; produces cosmic acceleration.

---

## Scalar-Tensor Theories

**Brans-Dicke theory** (1961): The simplest scalar-tensor theory. The gravitational "constant" is promoted to a scalar field $\phi$:
$$S_{\rm BD} = \frac{1}{16\pi}\int\left(\phi R - \frac{\omega_{\rm BD}}{\phi}(\nabla\phi)^2\right)\sqrt{-g}\,d^4x + S_{\rm matter}[g_{\mu\nu}, \psi]$$

The Brans-Dicke parameter $\omega_{\rm BD}$ controls the coupling. For $\omega_{\rm BD}\to\infty$: $\phi\to$ const and the theory reduces to GR. Cassini spacecraft measurement: $\omega_{\rm BD} > 40{,}000$ (from Shapiro delay).

**Horndeski theory** (1974, rediscovered 2011): The most general scalar-tensor theory with second-order equations in 4D. The Lagrangian is:
$$\mathcal{L}_H = G_2(\phi, X) + G_3(\phi, X)\Box\phi + G_4(\phi, X)R + G_{4X}\left[(\Box\phi)^2 - (\nabla_\mu\nabla_\nu\phi)^2\right] + \ldots$$

where $X = -\frac{1}{2}(\nabla\phi)^2$ and $G_{4X} = \partial G_4/\partial X$. This encompasses: $f(R)$, Brans-Dicke, Galileon, kinetic gravity braiding, and more.

**GW170817 constraint**: The speed of GWs was measured: $|c_{\rm GW} - c|/c < 10^{-15}$. This eliminates all Horndeski models where $G_{4X}\neq 0$ or $G_{5}\neq 0$ (since these give $c_{\rm GW}\neq c$). The surviving Horndeski theories are $\mathcal{L}_H = G_2(\phi,X) + G_3(\phi,X)\Box\phi + G_4(\phi)R$ — a much restricted class.

---

## Massive Gravity

In GR, the graviton is massless ($m_g = 0$). What if the graviton has a tiny mass $m_g\neq 0$?

**Fierz-Pauli theory** (linear level): The unique ghost-free mass term at linear order:
$$\mathcal{L}_{\rm FP} = -\frac{m_g^2}{2}(h_{\mu\nu}h^{\mu\nu} - h^2)$$

But Fierz-Pauli theory has a van Dam-Veltman-Zakharov (vDVZ) discontinuity: even as $m_g\to 0$, predictions differ from massless GR (extra scalar mode survives). Predictions for light deflection, for example, differ by 25%.

**The vDVZ discontinuity is resolved by the Vainshtein mechanism**: nonlinear interactions become important within the Vainshtein radius $r_V = (r_s/(m_g^2/c^2)^2)^{1/5}$. Within $r_V$, massive gravity agrees with GR; outside $r_V$, deviations appear.

**de Rham-Gabadadze-Tolley (dRGT) massive gravity** (2010): The first ghost-free nonlinear massive gravity theory. It has 5 physical degrees of freedom (instead of GR's 2): 2 tensor + 2 vector + 1 scalar.

**Observational constraints**: $m_g < 1.27\times 10^{-23}$ eV/$c^2$ from GW150914 (graviton speed; $c_{\rm GW}/c$ consistent with 1 over 1.3 Gly travel distance). The bound from the Compton wavelength of GR is $\lambda_g > 10^{22}$ m.

---

## MOND and TeVeS

**MOND** (Modified Newtonian Dynamics, Milgrom 1983): Phenomenological modification of Newton's law at low accelerations:
$$\mu(a/a_0)\mathbf{a} = -\nabla\Phi_N$$

where $\mu(x)\to x$ for $x\ll 1$ and $\mu(x)\to 1$ for $x\gg 1$, and $a_0 \approx 1.2\times 10^{-10}$ m/s$^2$.

MOND accounts for galaxy rotation curves without dark matter: for $a\ll a_0$, $\mu(a/a_0)\approx a/a_0$, giving $a^2/a_0 = GM/r^2$, so $a = (GMa_0)^{1/2}/r$ — flat rotation curve!

But MOND is non-relativistic and violates momentum conservation for multi-body systems.

**TeVeS** (Tensor-Vector-Scalar theory, Bekenstein 2004): The relativistic completion of MOND. Contains the metric $g_{\mu\nu}$, a vector field $A_\mu$, and a scalar $\phi$. The matter action uses a different physical metric $\tilde{g}_{\mu\nu} = e^{-2\phi}(g_{\mu\nu} + A_\mu A_\nu) - e^{2\phi}A_\mu A_\nu$.

**GW170817 killed TeVeS**: The tensor and scalar modes in TeVeS propagate at different speeds. The measured $|c_{\rm GW} - c|/c < 10^{-15}$ rules out the original TeVeS.

MOND remains phenomenologically successful for isolated galaxies but fails for galaxy clusters (requires dark matter there too). The bullet cluster — a galaxy cluster merger where the gravitational center is offset from the gas (baryon) center — is strong evidence for collisionless dark matter and against MOND.

---

## Screening Mechanisms

Modified gravity theories must hide their modifications in well-tested environments (solar system, laboratory) while showing deviations at cosmological scales. Several **screening mechanisms** achieve this:

**Chameleon** (Khoury-Weltman 2004): The scalar's effective mass depends on the local density $\rho$: $m_{\rm eff}^2 \propto \rho$. In high-density environments (inside stars), $m_{\rm eff}$ is large — the scalar is short-range and undetectable. In low-density environments (between galaxies), $m_{\rm eff}$ is small — long-range deviations from GR.

**Vainshtein mechanism**: Derivative interactions suppress the scalar's coupling to matter within the Vainshtein radius. The extra degree of freedom in massive gravity is screened this way.

**Symmetron** (Hinterbichler-Khoury 2010): A $\mathbb{Z}_2$ symmetry ($\phi\to -\phi$) is broken spontaneously in low-density environments (VEV $\neq 0$, scalar couples to matter) and restored in high-density environments (VEV $= 0$, scalar decouples).

**Testing screening in the lab**: Torsion pendulum experiments (E\"ot-Wash), atom interferometry, and neutron bouncing experiments can test screened modifications at laboratory scales.

---

## Emergent Gravity

**Jacobson's thermodynamic derivation of GR** (1995): If the entropy of a causal horizon is $S = A/(4\ell_P^2)$ and the Clausius relation $\delta Q = T\,dS$ holds, then the Einstein equations follow as the equation of state of spacetime. GR emerges from thermodynamics.

**Verlinde's entropic gravity** (2010): Gravity is not a fundamental force but an entropic force — an effective force arising from the tendency of systems to maximize entropy. The gravitational attraction between two masses follows from the information content of the holographic screen between them.

**Emergent gravity challenges**: 
- Dark matter as an apparent effect: in Verlinde's theory, the "dark matter" distribution in galaxies is predicted from the baryonic distribution. Early results were suggestive; detailed comparisons with observations are mixed.
- Quantum gravity: if GR is emergent, the correct UV completion may not be a quantization of GR at all, but a deeper theory from which gravity emerges at low energies.

---

## Important Concepts

- **Lovelock's theorem**: GR + $\Lambda$ is unique second-order, covariant, local theory in 4D; modifications must break one of these conditions
- **$f(R)$ gravity**: Fourth-order equations; equivalent to GR + scalar (scalaron); Starobinsky model for inflation
- **Chameleon mechanism**: Screening GR modifications at high density; makes $f(R)$ solar-system-safe
- **Brans-Dicke theory**: Scalar-tensor prototype; $\omega_{\rm BD} > 40{,}000$ from Cassini; GR limit as $\omega_{\rm BD}\to\infty$
- **Horndeski theory**: Most general second-order scalar-tensor; GW170817 eliminated most; restricted to $G_4(\phi)R$ subclass
- **GW170817 constraints**: $|c_{\rm GW} - c|/c < 10^{-15}$ eliminated most alternative gravity theories
- **Massive gravity / dRGT**: Ghost-free nonlinear massive gravity; 5 DOF; Vainshtein screening
- **MOND**: Phenomenological modification at $a < a_0$; flat rotation curves; inconsistent with GW170817
- **Screening mechanisms**: Chameleon, Vainshtein, symmetron — hide modifications in tested environments
- **Emergent gravity**: GR as thermodynamics (Jacobson), entropic gravity (Verlinde); gravity not fundamental

---

## Important Figures

**Mordehai Milgrom** (1946–): Proposed MOND (1983); the flat rotation curve law $a = (GMa_0)^{1/2}/r$ is highly accurate for disk galaxies.

**Jacob Bekenstein** (1947–2023): Formulated TeVeS; also derived black hole entropy and the Bekenstein bound.

**David Lovelock** (1938–): Proved Lovelock's theorem (1971); fundamental constraint on modified gravity theories.

**Ted Jacobson** (1954–): Thermodynamic derivation of GR from the Clausius relation (1995); profound reformulation.

**Erik Verlinde** (1962–): Entropic gravity and dark matter as apparent effect (2010); motivates alternative to dark matter particle.

**Cédric Deffayet**, **Gregory Gabadadze**, **Antonio Nicolis**, **Claudia de Rham**: Ghost-free massive gravity (dRGT, 2010); Galileon models; DGP braneworld.

---

## Further Reading

**Primary Sources**
- Milgrom, M. (1983). "A Modification of the Newtonian Dynamics." *ApJ*, 270, 365.
- Jacobson, T. (1995). "Thermodynamics of Spacetime: The Einstein Equation of State." *Phys. Rev. Lett.*, 75, 1260.
- de Rham, C., Gabadadze, G., & Tolley, A.J. (2011). "Resummation of Massive Gravity." *Phys. Rev. Lett.*, 106, 231101.
- Sotiriou, T.P. & Faraoni, V. (2010). "f(R) Theories." *Rev. Mod. Phys.*, 82, 451. — Comprehensive review.
- Will, C.M. (2014). "The Confrontation Between General Relativity and Experiment." *Living Reviews in Relativity*, 17, 4.

---

## Exercises

**62.1.** *$f(R)$ gravity scalar.*

(a) The conformal transformation $\tilde{g}_{\mu\nu} = f'(R)g_{\mu\nu}$ maps $f(R)$ gravity to GR + scalar $\varphi = \sqrt{3/2}\ln f'(R)$. For $f(R) = R + R^2/(6M^2)$: find $\varphi(R)$ and the equivalent scalar potential $V(\varphi)$.

(b) For the Starobinsky model in the slow-roll limit: $V(\varphi) \approx \frac{3M^2}{4}(1-e^{-\sqrt{2/3}\varphi})^2$. Compute the slow-roll parameters $\epsilon = V'^2/(2V^2)$ and $\eta = V''/V$ for $\varphi\gg 1$. Show $\epsilon\approx 4/3e^{-2\sqrt{2/3}\varphi}$ and $n_s = 1 - 6\epsilon + 2\eta \approx 1 - 2/N^2$.

(c) For $N = 60$ e-folds: what is $n_s$? Compare to Planck 2018: $n_s = 0.9649\pm 0.0042$.

---

**62.2.** *MOND and galaxy rotation curves.*

(a) In the deep MOND limit ($a\ll a_0$), the force law $a^2/a_0 = GM/r^2$ gives flat rotation velocity $v_f^4 = GMa_0$ (the Tully-Fisher relation). For a galaxy with $v_f = 200$ km/s: compute $M$ using $a_0 = 1.2\times 10^{-10}$ m/s$^2$.

(b) The MOND acceleration scale $a_0 \approx cH_0/6$ — coincidence? Is there a theoretical reason MOND should know about the Hubble constant?

(c) The bullet cluster: two galaxy clusters that have passed through each other. X-ray emission traces hot gas (which is 90% of the baryon mass); weak gravitational lensing traces the mass distribution. The lensing center is offset from the X-ray center by $\sim 0.6$ Mpc. Explain why this is difficult for MOND to explain but natural for dark matter.

---

**62.3.** *GW170817 and modified gravity.*

(a) GW170817 was detected 1.7 seconds before GRB 170817A. The GW and EM signals traveled $\sim 40$ Mpc. Show that this gives $|c_{\rm GW} - c|/c < 10^{-15}$.

(b) Most Horndeski theories with $G_{4X}\neq 0$ give $c_{\rm GW}^2 = c^2(1 + 2G_{4X}\dot\phi^2/(G_4))$. For this to satisfy $|c_{\rm GW}/c - 1| < 10^{-15}$: what is the constraint on $G_{4X}\dot\phi^2/G_4$ in the present cosmological background?

(c) Does this constraint kill dark energy models based on Horndeski gravity? What subclass survives? (The surviving class is $G_4 = G_4(\phi)$, $G_5 = 0$.)

---

**Thought Experiment T62.1.** *Why not just accept dark matter and dark energy?*

The $\Lambda$CDM model works. With $\Omega_m = 0.315$ and $\Omega_\Lambda = 0.685$, it fits the CMB, BAO, supernovae, and large-scale structure with extraordinary precision. Dark matter is strongly constrained by independent lines of evidence: galaxy rotation curves, lensing, bullet cluster, structure formation, N-body simulations. Dark energy is consistent with a cosmological constant with no observed variation in $w$.

Given this success, why explore modified gravity? The standard arguments:
1. The cosmological constant has a value $\Lambda\sim 10^{-52}$ m$^{-2}$ that is $10^{120}$ times smaller than naively expected from quantum field theory. This is the "fine-tuning problem."
2. No dark matter particle has been detected despite decades of WIMP searches (LHC, direct detection, indirect detection).
3. MOND-like phenomenology is very successful at the galaxy scale — too successful to be pure coincidence.

Are these arguments sufficient to motivate modified gravity, given that $\Lambda$CDM is so successful? What would it take — what observation, what failure of $\Lambda$CDM — to compel a serious turn toward modified gravity? And what observation would definitively kill modified gravity as an alternative?
