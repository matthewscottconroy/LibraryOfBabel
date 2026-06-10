# Chapter 52: The Action Principle in General Relativity

---

## Chapter Introduction

Einstein derived his field equations in November 1915 through a combination of physical reasoning, mathematical elegance, and years of struggle. One month later, David Hilbert independently derived the same equations from an action principle — by writing down the simplest possible scalar Lagrangian density for the gravitational field and applying the calculus of variations.

The action principle is not merely a derivation trick. It is the deepest and most natural way to understand GR. It reveals what GR is "made of" — a specific choice of Lagrangian that encodes both the geometric content and the coupling to matter. It makes the theory's symmetries (diffeomorphism invariance) manifest. It allows for natural generalizations — add higher-curvature terms, add new fields, couple to fermions — all within a unified variational framework.

This chapter derives the Einstein field equations from the Einstein-Hilbert action, handles the subtleties that arise (the Gibbons-Hawking-York boundary term), discusses the coupling to matter, and explores modifications of the gravitational action. The technical machinery here — varying the metric determinant, varying the Ricci scalar — is the foundation for understanding both classical modifications of GR and semiclassical quantum gravity.

---

## The Einstein-Hilbert Action

The **Einstein-Hilbert action** for gravity in vacuum (with cosmological constant) is:

$$S_{\rm EH} = \frac{c^4}{16\pi G}\int_{\mathcal{M}}\left(R - 2\Lambda\right)\sqrt{-g}\,d^4x$$

where:
- $g = \det(g_{\mu\nu})$ (negative for Lorentzian signature)
- $R = g^{\mu\nu}R_{\mu\nu}$ is the Ricci scalar
- $\Lambda$ is the cosmological constant
- The prefactor $c^4/(16\pi G)$ is fixed by requiring the Newtonian limit

The total action including matter fields $\Psi$:
$$S = S_{\rm EH} + S_{\rm matter} = \frac{c^4}{16\pi G}\int(R - 2\Lambda)\sqrt{-g}\,d^4x + \int\mathcal{L}_{\rm matter}(g_{\mu\nu}, \Psi)\sqrt{-g}\,d^4x$$

---

## Varying the Action

We vary $S$ with respect to the inverse metric $g^{\mu\nu}$ (treating $g^{\mu\nu}$ as the independent variable). Requiring $\delta S/\delta g^{\mu\nu} = 0$ gives the field equations.

**Key variations**:

**1. Variation of $\sqrt{-g}$:**

Using $\ln(-g) = \text{tr}\ln g_{\mu\nu}$:
$$\delta\sqrt{-g} = -\frac{1}{2}\sqrt{-g}\,g_{\mu\nu}\delta g^{\mu\nu} = \frac{1}{2}\sqrt{-g}\,g^{\mu\nu}\delta g_{\mu\nu}$$

(Note: $g_{\mu\nu}\delta g^{\mu\nu} = -g^{\mu\nu}\delta g_{\mu\nu}$ from $g_{\mu\alpha}g^{\alpha\nu} = \delta^\nu_\mu$.)

**2. Variation of $R$:**

$$\delta R = \delta(g^{\mu\nu}R_{\mu\nu}) = R_{\mu\nu}\delta g^{\mu\nu} + g^{\mu\nu}\delta R_{\mu\nu}$$

The second term: $g^{\mu\nu}\delta R_{\mu\nu} = \nabla_\mu V^\mu$ where $V^\mu = g^{\alpha\beta}\delta\Gamma^\mu_{\alpha\beta} - g^{\mu\alpha}\delta\Gamma^\beta_{\alpha\beta}$.

This is a total covariant divergence (the **Palatini identity**): $g^{\mu\nu}\delta R_{\mu\nu} = \nabla_\mu(g^{\alpha\beta}\delta\Gamma^\mu_{\alpha\beta} - g^{\mu\beta}\delta\Gamma^\alpha_{\alpha\beta})$.

By Stokes' theorem, $\int\nabla_\mu V^\mu\sqrt{-g}\,d^4x = \int_{\partial\mathcal{M}}V^\mu n_\mu\sqrt{|h|}\,d^3x$ — a boundary term.

**If we ignore the boundary term** (assume $\delta g^{\mu\nu} = 0$ and $\partial_\alpha\delta g^{\mu\nu} = 0$ on $\partial\mathcal{M}$), the bulk variation gives:

$$\delta S_{\rm EH} = \frac{c^4}{16\pi G}\int\left(R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R + \Lambda g_{\mu\nu}\right)\delta g^{\mu\nu}\sqrt{-g}\,d^4x$$

Setting this to zero for arbitrary $\delta g^{\mu\nu}$:
$$R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R + \Lambda g_{\mu\nu} = 0 \quad (\text{vacuum Einstein equations with }\Lambda)$$

---

## Coupling to Matter: The Stress-Energy Tensor

The matter action contribution:
$$\delta S_{\rm matter} = \frac{1}{2}\int T_{\mu\nu}\delta g^{\mu\nu}\sqrt{-g}\,d^4x$$

defines the **stress-energy tensor** variationally:
$$T_{\mu\nu} \equiv -\frac{2}{\sqrt{-g}}\frac{\delta(\sqrt{-g}\mathcal{L}_{\rm matter})}{\delta g^{\mu\nu}} = -2\frac{\partial\mathcal{L}_{\rm matter}}{\partial g^{\mu\nu}} + g_{\mu\nu}\mathcal{L}_{\rm matter}$$

This is the Hilbert stress-energy tensor — it is automatically symmetric and covariantly conserved ($\nabla^\mu T_{\mu\nu} = 0$) as a consequence of diffeomorphism invariance (Noether's theorem for the diffeomorphism symmetry).

The full variation gives **Einstein's field equations**:
$$G_{\mu\nu} + \Lambda g_{\mu\nu} = \frac{8\pi G}{c^4}T_{\mu\nu}$$

---

## Examples of Matter Lagrangians

**Perfect fluid**: $\mathcal{L}_{\rm fluid} = -\rho c^2$ where $\rho$ is the rest-mass energy density. The stress-energy tensor:
$$T^{\mu\nu} = (\rho c^2 + P)u^\mu u^\nu + Pg^{\mu\nu}$$

where $P$ is the pressure and $u^\mu$ is the 4-velocity.

**Scalar field** $\phi$:
$$\mathcal{L}_\phi = -\frac{1}{2}g^{\mu\nu}\nabla_\mu\phi\nabla_\nu\phi - V(\phi)$$
$$T_{\mu\nu} = \nabla_\mu\phi\nabla_\nu\phi - g_{\mu\nu}\left(\frac{1}{2}g^{\alpha\beta}\nabla_\alpha\phi\nabla_\beta\phi + V(\phi)\right)$$

**Electromagnetic field**:
$$\mathcal{L}_{\rm EM} = -\frac{1}{4\mu_0}F_{\mu\nu}F^{\mu\nu}$$
$$T^{\mu\nu}_{\rm EM} = \frac{1}{\mu_0}\left(F^{\mu\rho}F^\nu_{\ \rho} - \frac{1}{4}g^{\mu\nu}F_{\rho\sigma}F^{\rho\sigma}\right)$$

---

## The Gibbons-Hawking-York Boundary Term

The Einstein-Hilbert action contains second derivatives of the metric through the Riemann tensor. When varying, these produce boundary terms that do not vanish even if $\delta g^{\mu\nu} = 0$ on the boundary — only $\partial_\alpha\delta g^{\mu\nu}|_{\partial\mathcal{M}} = 0$ eliminates them.

To have a well-posed variational problem requiring only $\delta g^{\mu\nu}|_{\partial\mathcal{M}} = 0$, one must add the **Gibbons-Hawking-York (GHY) boundary term**:

$$S_{\rm GHY} = \frac{c^4}{8\pi G}\int_{\partial\mathcal{M}}K\sqrt{|h|}\,d^3x$$

where $K = h^{ab}K_{ab}$ is the trace of the extrinsic curvature of the boundary $\partial\mathcal{M}$, and $h_{ab}$ is the induced metric on the boundary.

The complete well-posed action:
$$S = \frac{c^4}{16\pi G}\int_{\mathcal{M}}R\sqrt{-g}\,d^4x + \frac{c^4}{8\pi G}\int_{\partial\mathcal{M}}K\sqrt{|h|}\,d^3x + S_{\rm matter}$$

**Physical significance of GHY**: 
- Necessary for the path integral formulation of quantum gravity (Euclidean quantum gravity)
- Relates to the Bekenstein-Hawking entropy: the black hole entropy $S = k_B A/(4\ell_P^2)$ can be derived from the Euclidean path integral, where the GHY term evaluated on the horizon gives the entropy
- Important for AdS/CFT: the boundary term is related to the CFT energy-momentum tensor

---

## The Palatini Variation

An alternative variational principle treats the metric $g_{\mu\nu}$ and the connection $\Gamma^\rho_{\mu\nu}$ as **independent fields** (the Palatini or first-order formalism).

Varying $S_{\rm EH}$ independently:
- Variation with respect to $g^{\mu\nu}$: gives $R_{(\mu\nu)} = 0$ (symmetric Ricci tensor vanishes)
- Variation with respect to $\Gamma^\rho_{\mu\nu}$: gives $\nabla_\rho g_{\mu\nu} = 0$ (metric compatibility)

Together, these reproduce Einstein's equations — so the Palatini and metric variations are equivalent for the Einstein-Hilbert action.

**Modified gravity**: For $f(R)$ gravity $S = \int f(R)\sqrt{-g}\,d^4x$, the metric and Palatini variations give *different* theories. This is because $f(R) \neq f(g^{\mu\nu}R_{\mu\nu}[\Gamma])$ when $\Gamma$ is not the Levi-Civita connection.

---

## $f(R)$ and Higher-Curvature Gravity

**$f(R)$ gravity**: Replace $R$ with a function $f(R)$:
$$S = \frac{1}{16\pi G}\int f(R)\sqrt{-g}\,d^4x$$

Field equations (metric formalism):
$$f'(R)R_{\mu\nu} - \frac{1}{2}f(R)g_{\mu\nu} - (\nabla_\mu\nabla_\nu - g_{\mu\nu}\Box)f'(R) = \frac{8\pi G}{c^4}T_{\mu\nu}$$

This is a fourth-order equation in $g_{\mu\nu}$. For $f(R) = R + \alpha R^2$ (Starobinsky inflation), the extra scalar degree of freedom plays the role of the inflaton.

**Gauss-Bonnet term**: $\mathcal{G} = R^2 - 4R_{\mu\nu}R^{\mu\nu} + R_{\mu\nu\rho\sigma}R^{\mu\nu\rho\sigma}$. Adding $\int\mathcal{G}\sqrt{-g}\,d^4x$ to the action in 4D adds no new equations of motion (it is a total derivative in 4D). In 5D and above, it contributes — this is Gauss-Bonnet gravity, relevant to Kaluza-Klein theories and string theory.

---

## Important Concepts

- **Einstein-Hilbert action**: $S_{\rm EH} = (c^4/16\pi G)\int R\sqrt{-g}\,d^4x$; the unique generally covariant action with at most second-derivative equations
- **Palatini identity**: $g^{\mu\nu}\delta R_{\mu\nu} = \nabla_\mu V^\mu$; variation of Ricci scalar produces boundary term
- **Hilbert stress-energy tensor**: $T_{\mu\nu} = -2\delta(\sqrt{-g}\mathcal{L}_m)/(\sqrt{-g}\delta g^{\mu\nu})$; automatically symmetric and conserved
- **Gibbons-Hawking-York term**: Boundary term needed for well-posed variational principle; related to black hole entropy
- **Palatini variation**: First-order formalism treating $g$ and $\Gamma$ independently; equivalent to metric variation for GR
- **$f(R)$ gravity**: Generalization replacing $R$ with $f(R)$; introduces new scalar degree of freedom; Starobinsky model of inflation
- **Lovelock theorem**: Einstein-Hilbert action is the unique action giving second-order field equations in 4D

---

## Important Figures

**David Hilbert** (1862–1943): Derived Einstein's field equations from the action principle (November 1915, days before Einstein's final announcement); formulated the Einstein-Hilbert action.

**Albert Einstein** (1879–1955): Developed the field equations through physical reasoning; correspondence with Hilbert in November 1915.

**James York** (1939–2021) and **Gary Gibbons** (1946–), **Stephen Hawking** (1942–2018): Identified and derived the boundary term necessary for the well-posed variational problem (1977).

**Alexei Starobinsky** (1948–2023): Proposed the $R + R^2$ inflation model (1980); remains the best-fit model to CMB observations.

---

## Further Reading

**Primary Sources**
- Hilbert, D. (1915). "Die Grundlagen der Physik." *Nachrichten der Ges. der Wiss. zu Göttingen*, 395–407.
- York, J.W. (1972). "Role of Conformal Three-Geometry in the Dynamics of Gravitation." *Phys. Rev. Lett.*, 28, 1082.
- Gibbons, G.W. & Hawking, S.W. (1977). "Action Integrals and Partition Functions in Quantum Gravity." *Phys. Rev. D*, 15, 2752.

**Textbooks**
- Wald, R.M. (1984). *General Relativity*. University of Chicago Press. — Appendix E on variational principle.
- Carroll, S.M. (2004). *Spacetime and Geometry*. Addison-Wesley. — Chapter 4 on the action.
- Padmanabhan, T. (2010). *Gravitation: Foundations and Frontiers*. Cambridge. — Detailed treatment of variational principles in GR.

---

## Exercises

**52.1.** *Deriving the Einstein equations.*

(a) Verify the variation formula $\delta\sqrt{-g} = -\frac{1}{2}\sqrt{-g}\,g_{\mu\nu}\delta g^{\mu\nu}$ by differentiating $\det g$ with respect to $g^{\mu\nu}$.

(b) The Palatini identity states $\delta R^\rho_{\ \sigma\mu\nu} = \nabla_\mu(\delta\Gamma^\rho_{\nu\sigma}) - \nabla_\nu(\delta\Gamma^\rho_{\mu\sigma})$. Use this to show $g^{\mu\nu}\delta R_{\mu\nu} = \nabla_\mu V^\mu$ for some vector $V^\mu$.

(c) Combining (a) and (b), complete the variation of $S_{\rm EH}$ and derive $G_{\mu\nu} = 0$ in vacuum.

---

**52.2.** *Stress-energy tensors from action.*

(a) For a scalar field $\phi$ with $\mathcal{L} = -\frac{1}{2}(\nabla\phi)^2 - V(\phi)$, compute $T_{\mu\nu}$ using the variational definition.

(b) For a perfect fluid with $\mathcal{L}_{\rm fluid}$ in terms of $\rho$ and $u^\mu$, verify that the variational $T_{\mu\nu}$ gives $(\rho c^2 + P)u_\mu u_\nu + Pg_{\mu\nu}$.

(c) Show $\nabla^\mu T_{\mu\nu} = 0$ for the scalar field using the equations of motion (Klein-Gordon equation in curved spacetime: $\Box\phi = V'(\phi)$).

---

**52.3.** *The Gibbons-Hawking-York term and black hole entropy.*

The Euclidean Schwarzschild metric (Wick-rotate $t\to -i\tau$) is:
$$ds^2 = \left(1-\frac{r_s}{r}\right)c^2d\tau^2 + \frac{dr^2}{1-r_s/r} + r^2d\Omega^2$$

(a) The Euclidean action $S_E$ contributes $e^{-S_E/\hbar}$ to the partition function. The period of $\tau$ must be $\beta = 2\pi r_s/c$ to avoid a conical singularity at $r = r_s$. This gives $\beta = \hbar/(k_BT_H)$ — derive $T_H$.

(b) The GHY term for the Euclidean black hole gives $S_{\rm GHY} = -k_B A/(4\ell_P^2)$ (the dominant contribution). Identify this as the Bekenstein-Hawking entropy.

(c) The free energy is $F = T\cdot S_E/\hbar$. Compute $E = \partial(\beta F)/\partial\beta$ and show it gives the black hole mass $E = Mc^2$.

---

**Thought Experiment T52.1.** *What is gravity made of?*

The Einstein-Hilbert action $S = \int R\sqrt{-g}\,d^4x$ is unique (by Lovelock's theorem) as the action yielding second-order equations invariant under diffeomorphisms in 4D. Adding higher-derivative terms ($R^2$, $R_{\mu\nu}R^{\mu\nu}$, etc.) is allowed but introduces new degrees of freedom.

From the quantum perspective, the Einstein-Hilbert action is non-renormalizable: loop corrections generate an infinite series of higher-derivative terms $c_1 R^2/M_P^2 + c_2 R_{\mu\nu}R^{\mu\nu}/M_P^2 + \cdots$. At energies $\ll M_P$, these are negligible (GR is an effective field theory). Near $M_P$, they matter.

Does the uniqueness of the Einstein-Hilbert action at low energies tell us something fundamental about the nature of gravity, or is it an accident of working in 4D? Could the "true" theory of quantum gravity have a completely different action at high energies, with GR emerging only at low energies?
