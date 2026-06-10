# Section 11.4: Green's Functions and the Dirichlet Problem

---

## Section Introduction

The **Green's function** is the response of a linear PDE to a point source — the solution when the "forcing" is a delta function δ(x − x₀). Once the Green's function is known, the response to any source is obtained by superposition: integrate the Green's function against the source. This is one of the most powerful techniques in mathematical physics.

Green's functions appear throughout GR. The gravitational wave emitted by a binary system is computed using the Green's function of the wave operator on Minkowski spacetime. The Schwarzschild solution is the Green's function for the linearized Einstein equations in a certain sense (it represents the field of a point mass). And in quantum field theory in curved spacetime, the Hawking and Unruh effects are computed using Green's functions of the wave operator on curved backgrounds.

---

## 11.4.1 The Fundamental Solution of the Laplacian

**Definition**: The **fundamental solution** (or free-space Green's function) of the Laplacian −∇² in ℝⁿ is the function G(x) satisfying:

$$-\nabla^2 G(x) = \delta^n(x)$$

By rotational symmetry, G depends only on r = |x|. Outside the origin, −∇²G = 0 (Laplace's equation), so G = G(r) satisfies:

$$\frac{1}{r^{n-1}}\frac{d}{dr}\left(r^{n-1} \frac{dG}{dr}\right) = 0 \quad (r \neq 0)$$

Solutions: G = A + B/(n−2)r^{n-2} for n ≥ 3, and G = A + B ln r for n = 2.

**Normalization**: Integrating −∇²G = δ over a ball B_ε of radius ε and using the divergence theorem:

$$-\int_{B_\epsilon} \nabla^2 G \, dV = -\oint_{\partial B_\epsilon} \frac{dG}{dr} dS = 1$$

This gives B = 1/ω_n(n−2) where ω_n = 2π^{n/2}/Γ(n/2) is the surface area of the unit (n−1)-sphere.

**Result**: In ℝ³ (n = 3):

$$G(x) = \frac{1}{4\pi |x|}$$

This is the Newtonian gravitational potential (up to sign and factors): Φ(x) = −GM·G(x) satisfies ∇²Φ = 4πGM δ³(x), which is Poisson's equation for a point mass M at the origin.

**In 2D** (relevant for plane problems): G(x) = −(1/2π) ln|x|.

---

## 11.4.2 The Green's Function for a Domain

**Definition**: The **Green's function** G(x, y) for the Dirichlet problem on a domain Ω is the function satisfying:
- −∇²_x G(x, y) = δ(x − y) for x ∈ Ω (point source at y)
- G(x, y) = 0 for x ∈ ∂Ω (zero boundary condition)

**Decomposition**: G(x, y) = Φ(x − y) + H(x, y), where Φ is the fundamental solution and H is a harmonic correction satisfying H(x, y) = −Φ(x − y) on ∂Ω. The correction H "cancels" Φ on the boundary.

**Solution to the Dirichlet problem**: With G in hand, the solution to −∇²u = f in Ω, u = g on ∂Ω is:

$$u(x) = \int_\Omega G(x, y) f(y) \, dy - \oint_{\partial\Omega} g(y) \frac{\partial G}{\partial n_y}(x, y) \, dS$$

The first term is the contribution from the interior source f; the second is from the boundary data g, weighted by the normal derivative of G (the **Poisson kernel**).

**Example — Half-space** (ℝ³_+ = {z > 0}): The Green's function is:

$$G(x, y) = \frac{1}{4\pi|x - y|} - \frac{1}{4\pi|x - y^*|}$$

where y* is the reflection of y across the boundary z = 0 (the **method of images**). The reflected source − at y* cancels the potential of + at y on the boundary z = 0. This is the exact same construction as the method of images in electrostatics.

---

## 11.4.3 Green's Functions for the Wave Equation

For the wave operator $\Box = -\partial_t^2/c^2 + \nabla^2$ in Minkowski spacetime ℝ^{1,3}, the Green's function (retarded propagator) satisfies:

$$\Box G_R(x, y) = \delta^4(x - y)$$

with the condition that G_R = 0 for t_x < t_y (no signal arrives before it is sent).

**Result** in 3+1 dimensions:

$$G_R(x, y) = \frac{c}{4\pi} \frac{\delta(t_x - t_y - |x-y|/c)}{|x - y|}$$

The delta function enforces propagation at exactly speed c: a point source at event y produces a signal only on the future light cone of y.

**Retarded solution**: The field produced by a source J(x) is:

$$\phi(x) = \int G_R(x, y) J(y) d^4y = \frac{c}{4\pi} \int \frac{J(y^*, t - |x-y^*|/c)}{|x - y^*|} d^3y^*$$

where y* = y (spatial part) and the time argument is retarded: t_{\rm ret} = t − |x−y|/c (signals take time |x−y|/c to travel from y to x).

**For gravitational waves**: The linearized Einstein equations in harmonic gauge take the form $\Box \bar{h}_{\mu\nu} = -16\pi G T_{\mu\nu}/c^4$. The solution is:

$$\bar{h}_{\mu\nu}(t, \mathbf{x}) = \frac{4G}{c^4} \int \frac{T_{\mu\nu}(t_\text{ret}, \mathbf{x}')}{|\mathbf{x} - \mathbf{x}'|} d^3x'$$

This is the **linearized metric perturbation** — the gravitational wave field emitted by the source T_{μν}. In the far field (|x| ≫ source size), this leads to the quadrupole formula for gravitational wave emission (Unit XI).

---

## 11.4.4 Distributional Solutions and the Delta Function

Green's functions involve the **Dirac delta function** δ(x), which is not an ordinary function but a **distribution** (generalized function). The theory of distributions, developed by Laurent Schwartz (1945), provides the rigorous foundation.

**Definition**: A **distribution** (generalized function) on ℝⁿ is a continuous linear functional T: C∞_c(ℝⁿ) → ℝ, where C∞_c is the space of smooth compactly supported functions (test functions). The pairing is written ⟨T, φ⟩.

**Examples**:
- The delta distribution: ⟨δ_x₀, φ⟩ = φ(x₀).
- Any locally integrable function f defines a distribution: ⟨f, φ⟩ = ∫ f(x)φ(x) dx.
- The **principal value** P.V. (1/x) is a distribution, not a function: ⟨P.V.(1/x), φ⟩ = lim_{ε→0} ∫_{|x|>ε} φ(x)/x dx.

**Derivatives of distributions**: ⟨T', φ⟩ = −⟨T, φ'⟩ (integration by parts, with no boundary terms since φ is compactly supported). Every distribution is infinitely differentiable in this sense. In particular:
- δ' is the "derivative" of the delta function: ⟨δ', φ⟩ = −φ'(0).
- The Heaviside step function H(x) (= 0 for x < 0, = 1 for x > 0) has distributional derivative H'(x) = δ(x). This is consistent: a sudden jump in a function corresponds to a point source.

**The GR metric as a distribution**: At a spacetime singularity (r = 0 in Schwarzschild), the metric diverges. The distributional interpretation of the metric, and of the curvature tensors, requires careful treatment. The curvature of a distributional metric may involve delta functions supported on the singularity — which correspond to the "point mass" source in the Einstein equations.

**Colombeau algebras**: For nonlinear PDEs (like the full Einstein equations), the product of distributions is not always well-defined. Colombeau's algebra of generalized functions (1984) provides one framework for handling this, relevant to distributional solutions in GR.

---

## 11.4.5 Spectral Theory and the Green's Function

For a self-adjoint operator L = −∇² on a domain Ω with appropriate boundary conditions, the spectral theorem gives a basis of eigenfunctions:

$$L\phi_n = \lambda_n \phi_n$$

with eigenvalues 0 ≤ λ₁ ≤ λ₂ ≤ ... → ∞ and orthonormal eigenfunctions {φ_n}. The **spectral expansion** of the Green's function is:

$$G(x, y) = \sum_{n=1}^\infty \frac{\phi_n(x) \phi_n(y)}{\lambda_n}$$

(assuming λ_n > 0; if 0 is an eigenvalue, the operator is not invertible).

**Heat kernel and spectral zeta function**: The heat kernel K(x, y; t) = e^{−tL}(x, y) has the expansion $K(x, y; t) = \sum e^{-\lambda_n t} \phi_n(x)\phi_n(y)$. The trace:

$$\text{Tr}(e^{-tL}) = \int_\Omega K(x, x; t) dx = \sum_{n=1}^\infty e^{-\lambda_n t}$$

encodes the full spectrum. Its small-t asymptotics give the **heat kernel coefficients** (Seeley-DeWitt coefficients), which appear in the calculation of quantum field theory effective actions in curved spacetime and in the zeta function regularization of path integrals.

---

## References

- Green, G. (1828). *Essay on the Application of Mathematical Analysis to the Theories of Electricity and Magnetism.* [Introduces Green's function and the method of images in the context of electrostatics.]
- Schwartz, L. (1951). *Théorie des distributions.* Hermann, Paris. [Rigorously founds the theory of distributions (generalized functions). Schwartz was awarded the Fields Medal for this work in 1950.]
- Evans, L.C. (2010). *Partial Differential Equations*, 2nd ed. AMS. [Chapter 2.2: Laplace's equation; fundamental solution; Green's functions; Poisson kernel.]
- DeWitt, B.S. (1965). *Dynamical Theory of Groups and Fields.* Gordon and Breach. [The DeWitt expansion (Seeley-DeWitt coefficients) for the heat kernel in curved spacetime; foundational for quantum field theory in curved backgrounds.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation.* W.H. Freeman. [§36.10 on the retarded Green's function for gravitational waves; derivation of the quadrupole formula.]
- Peters, P.C. (1964). "Gravitational radiation and the motion of two point masses." *Physical Review*, 136, B1224–B1232. [Uses the retarded Green's function (section 11.4.3) to compute gravitational wave energy loss from a binary system, predicting orbital decay — verified 10 years later by Hulse and Taylor.]
