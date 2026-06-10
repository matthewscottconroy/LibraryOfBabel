# Section 35.1: The Path to the Field Equations

---

## The Guiding Constraints

Einstein needed a tensor equation of the form:
$$\text{(something built from the metric)} = \kappa\, T_{\mu\nu}$$
where $\kappa = 8\pi G/c^4$ is a constant to be determined from the Newtonian limit.

The constraints on "something" are:
1. **Symmetric rank-2 tensor** (since $T_{\mu\nu}$ is symmetric, the left side must be too).
2. **Divergence-free:** $\nabla_\mu\,(\text{LHS})^{\mu\nu} = 0$, because $\nabla_\mu T^{\mu\nu} = 0$ is energy-momentum conservation, which must hold automatically.
3. **Built from the metric and its first two derivatives** (to give second-order differential equations — like Poisson's equation $\nabla^2\Phi = 4\pi G\rho$, which is second order).
4. **Reduces to Newtonian gravity** in the weak-field, slow-motion limit: $\nabla^2 g_{00} \approx 8\pi G\rho$.
5. **Linear in second derivatives of $g_{\mu\nu}$** (Einstein's original additional requirement, later relaxed).

Lovelock's theorem (1971) proves that in 4 spacetime dimensions, the *unique* symmetric, divergence-free, second-order tensor satisfying these conditions is:
$$H_{\mu\nu} = \alpha\left(R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R\right) + \Lambda g_{\mu\nu}$$
for constants $\alpha$ and $\Lambda$. Setting $\alpha = 1$ and $\Lambda = 0$ (for now) gives the Einstein tensor.

This means the Einstein field equations are *forced* by the constraints. There is essentially no freedom in their form.

---

## The Newtonian Limit

The constant $\kappa = 8\pi G/c^4$ is fixed by requiring the field equations to reduce to Poisson's equation in the appropriate limit.

Consider a slowly moving ($v \ll c$), weakly gravitating ($g_{\mu\nu} \approx \eta_{\mu\nu} + h_{\mu\nu}$, $|h_{\mu\nu}| \ll 1$), slowly varying (time derivatives negligible compared to spatial derivatives) system. The stress-energy tensor is dominated by the rest mass energy: $T^{00} \approx \rho c^2$, all other components negligible.

From Chapter 24 (linearized gravity), the $00$-component of the Einstein tensor in Lorenz gauge is:
$$G_{00} \approx -\nabla^2 h_{00}$$
The $00$-component of the field equations gives:
$$-\nabla^2 h_{00} = \kappa T_{00} = \kappa\rho c^2$$

From the equivalence principle (Section 34.3), the gravitational potential is related to the metric by:
$$g_{00} = -\left(1 + \frac{2\Phi}{c^2}\right) \implies h_{00} = -\frac{2\Phi}{c^2}$$
Substituting:
$$\frac{2}{c^2}\nabla^2\Phi = \kappa\rho c^2 \implies \nabla^2\Phi = \frac{\kappa\rho c^4}{2}$$
For this to match Poisson's equation $\nabla^2\Phi = 4\pi G\rho$:
$$\kappa = \frac{8\pi G}{c^4}$$

This fixes the constant. The complete Einstein field equations are:
$$\boxed{G_{\mu\nu} = \frac{8\pi G}{c^4}T_{\mu\nu}}$$
or in natural units ($G = c = 1$): $G_{\mu\nu} = 8\pi T_{\mu\nu}$.

---

## Einstein's Actual Path: Eight Years of Struggle

The history is worth knowing, because it shows that even the greatest physicists can struggle — and because the struggle illuminates the conceptual obstacles.

**1905**: SR. Einstein knows Newtonian gravity is inconsistent with SR (it's instantaneous; it doesn't transform correctly under Lorentz boosts). He needs a relativistic theory of gravity.

**1907**: The equivalence principle. Einstein realizes that a uniformly accelerating frame is locally equivalent to a uniform gravitational field. This is the seed.

**1907–1911**: Einstein tries to construct a scalar theory of gravity (analogous to the Newtonian potential $\Phi$, but now a Lorentz scalar). This fails: a scalar theory cannot account for all the relativistic effects of a gravitational field.

**1912**: Einstein moves to Zürich, meets Marcel Grossmann. Grossmann identifies the Riemannian geometry (Ricci-Levi-Civita absolute differential calculus) as the right mathematical tool. Einstein begins working with the metric tensor $g_{\mu\nu}$ as the fundamental field.

**1913**: The "Entwurf" paper (Einstein and Grossmann). A first attempt at a generally covariant theory. The field equations proposed are *wrong* — they are not generally covariant, and Einstein knows it but cannot find the right ones. He incorrectly argues that generally covariant equations cannot yield the correct Newtonian limit.

**1913–1915**: Einstein struggles with the "hole argument" (Lochbetrachtung): if the equations are generally covariant, can a given metric distribution $T_{\mu\nu}$ determine the metric uniquely? He concludes (wrongly) that general covariance is impossible, and works with restricted covariance. This two-year detour costs him greatly.

**1915, June**: Einstein gives lectures in Göttingen, presenting his incomplete theory. Hilbert attends and becomes interested.

**1915, November**: The final month. Einstein realizes his 1913 argument against general covariance was wrong. In four consecutive weekly lectures to the Prussian Academy (November 4, 11, 18, 25), he presents increasingly correct versions:
- Nov 4: Corrects the vacuum equations to be generally covariant.
- Nov 11: Proposes $R_{\mu\nu} = \kappa(T_{\mu\nu} - \frac{1}{2}g_{\mu\nu}T)$ (restricting to $T = 0$, i.e., matter with $T = T^\mu_{\ \mu} = 0$).
- Nov 18: Uses the correct equations to compute Mercury's perihelion precession for the first time: **43 arcseconds per century**. Matches the anomaly exactly. Einstein wrote to Sommerfeld: "I was beside myself with joy and excitement for days."
- Nov 25: Presents the final field equations $G_{\mu\nu} = \kappa T_{\mu\nu}$, now not restricted to traceless $T$.

**Nov 20**: Hilbert submits a paper deriving the same equations from the variational principle. The proofs show his original submission had a gap; both Einstein and Hilbert arrived at the final equations independently.

---

## The Einstein-Hilbert Action

The most elegant derivation of the field equations uses the variational principle. Hilbert showed that the action:
$$S_{\rm EH} = \frac{c^4}{16\pi G}\int R\sqrt{-g}\,d^4x$$
where $R$ is the Ricci scalar and $g = \det(g_{\mu\nu})$, plus a matter action $S_{\rm matter}$, gives the Einstein equations by varying with respect to the metric $g^{\mu\nu}$.

The variation of the Ricci scalar under $g^{\mu\nu} \to g^{\mu\nu} + \delta g^{\mu\nu}$ is (Palatini identity):
$$\delta R = R_{\mu\nu}\delta g^{\mu\nu} + g^{\mu\nu}\delta R_{\mu\nu}$$
The second term $g^{\mu\nu}\delta R_{\mu\nu}$ is a total divergence (the Palatini identity) and integrates to a boundary term. The variation of $\sqrt{-g}$ is $\delta\sqrt{-g} = -\frac{1}{2}\sqrt{-g}g_{\mu\nu}\delta g^{\mu\nu}$.

The gravitational part of the variation gives:
$$\delta S_{\rm EH} = \frac{c^4}{16\pi G}\int\left(R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R\right)\delta g^{\mu\nu}\sqrt{-g}\,d^4x = \frac{c^4}{16\pi G}\int G_{\mu\nu}\delta g^{\mu\nu}\sqrt{-g}\,d^4x$$

The matter action is defined via:
$$T_{\mu\nu} = -\frac{2}{\sqrt{-g}}\frac{\delta S_{\rm matter}}{\delta g^{\mu\nu}}$$
This definition of $T_{\mu\nu}$ is covariant by construction and automatically symmetric.

Setting $\delta S_{\rm EH} + \delta S_{\rm matter} = 0$ for arbitrary $\delta g^{\mu\nu}$ gives:
$$G_{\mu\nu} = \frac{8\pi G}{c^4}T_{\mu\nu}$$

The Einstein-Hilbert action is the simplest generally covariant scalar action built from the metric — it is the integral of the Ricci scalar, which is the simplest curvature invariant. This elegance is remarkable: the entire theory of gravity is contained in $S \sim \int R$.

---

## The Cosmological Constant

The most general action consistent with Lovelock's theorem in 4D is:
$$S = \frac{c^4}{16\pi G}\int(R - 2\Lambda)\sqrt{-g}\,d^4x + S_{\rm matter}$$
where $\Lambda$ is the cosmological constant. This gives:
$$G_{\mu\nu} + \Lambda g_{\mu\nu} = \frac{8\pi G}{c^4}T_{\mu\nu}$$

Einstein introduced $\Lambda$ in 1917 to allow a static universe. When Hubble discovered cosmic expansion (1929), Einstein called this his "greatest blunder" and removed it.

In 1998, two teams (Perlmutter et al.; Riess et al.) discovered from supernova observations that the universe's expansion is *accelerating* — which requires $\Lambda > 0$ (or some equivalent dark energy). The measured value is $\Lambda \approx 1.1\times 10^{-52}$ m$^{-2}$, corresponding to a dark energy density $\rho_\Lambda \approx 6\times 10^{-27}$ kg/m$^3$.

This value is tiny but nonzero. The cosmological constant problem — why is $\Lambda$ so small compared to particle physics predictions ($\sim 10^{123}$ times larger from QFT estimates) yet nonzero — is the deepest unsolved problem in theoretical physics.

