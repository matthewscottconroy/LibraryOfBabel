# Section 8.3: The Divergence Theorem and Stokes' Theorem

---

## Section Introduction

The integral theorems of vector calculus are among the most important theorems in all of physics. They connect local differential information (divergence, curl) to global integral information (total flux, total circulation). Conservation laws, Maxwell's equations, Newtonian gravity, fluid dynamics, and GR all have their most elegant forms when these theorems are available.

---

## 8.3.1 Green's Theorem

**Theorem** (Green, 1828): Let D be a simple closed region in ℝ² with smooth boundary ∂D, oriented counterclockwise. Let P and Q be C¹ on D. Then:

$$\iint_D \left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right) dA = \oint_{\partial D} (P \, dx + Q \, dy)$$

**Special cases**:
- Area formula: Area(D) = ½ ∮_{\partial D} (x dy − y dx).
- P = −y, Q = 0: Area(D) = −∮ y dx.

**Connection to curl**: The integrand ∂Q/∂x − ∂P/∂y is the z-component of ∇ × **F** where **F** = (P, Q, 0). So Green's theorem is a 2D version of Stokes' theorem.

---

## 8.3.2 The Divergence Theorem

**Theorem** (Gauss, 1813; Ostrogradsky, 1831): Let V be a compact region in ℝ³ with smooth boundary ∂V, oriented with outward-pointing normal. Let **F** be C¹ on V. Then:

$$\iiint_V (\nabla \cdot \mathbf{F}) \, dV = \oiint_{\partial V} \mathbf{F} \cdot d\mathbf{S}$$

**Proof sketch**: Prove for a rectangular box (direct computation using the FTC on each term of ∇·**F**). Then decompose general V into boxes. The interior faces cancel (adjacent boxes have opposite orientations), leaving the outer boundary. □

**Physical interpretations**:
- The total "source strength" (∫∫∫ ∇·**F** dV) equals the total outflow through the boundary.
- For an incompressible fluid: ∇·**v** = 0 implies ∫∫ **v**·dS = 0 (no net flux through any closed surface).
- Gauss's law for gravity: ∫∫ **g**·dA = −4πGM (total gravitational flux through any closed surface surrounding mass M).
- In GR: the contracted Bianchi identity ∇_μ G^{μν} = 0 and conservation ∇_μ T^{μν} = 0 take integral form via the covariant divergence theorem: ∫_∂V T^{μν} dΣ_μ = 0 for any spacetime volume V (Gauss's theorem on the manifold). [Misner, Thorne, Wheeler (1973), §15.3.]

---

## 8.3.3 Stokes' Theorem

**Theorem** (Stokes, 1854; stated by Thomson 1850): Let S be a smooth oriented surface in ℝ³ with smooth boundary ∂S, consistently oriented. Let **F** be C¹. Then:

$$\iint_S (\nabla \times \mathbf{F}) \cdot d\mathbf{S} = \oint_{\partial S} \mathbf{F} \cdot d\mathbf{l}$$

**Physical interpretations**:
- Faraday's law: ∮_{\partial S} **E**·dl = −(d/dt)∫∫_S **B**·dS. This is Stokes' theorem applied to **E**.
- Ampère's law: ∮_{\partial S} **B**·dl = μ₀ I_enc = μ₀ ∫∫_S **J**·dS. Stokes' applied to **B**.
- Path independence of conservative fields: ∮ **F**·dl = 0 ⟺ ∇×**F** = 0 (in simply connected domain).

---

## 8.3.4 The Generalized Stokes' Theorem

**All three theorems are special cases of**:

$$\int_M d\omega = \int_{\partial M} \omega$$

where:
- M is an oriented n-dimensional manifold with boundary
- ω is a smooth (n−1)-form on M
- dω is the exterior derivative of ω
- ∂M is the boundary of M with induced orientation

**Specializations**:
- n = 1, ω = f (0-form): $\int_a^b f'(x) dx = f(b) - f(a)$ — the FTC.
- n = 2, ω = P dx + Q dy: Green's theorem.
- n = 3, ω = **F**·d**S** (2-form): divergence theorem.
- n = 2, ω = **F**·dl (1-form): Stokes' theorem.

The exterior derivative d unifies gradient (d on 0-forms), curl (d on 1-forms in ℝ³), and divergence (d on 2-forms in ℝ³). The identity d² = 0 encodes:
- ∇ × (∇f) = 0
- ∇ · (∇ × **F**) = 0

**In GR**: The generalized Stokes' theorem is used to derive conservation laws from the contracted Bianchi identity. If ∇_μ J^μ = 0 (covariant conservation), then by the covariant divergence theorem, the "charge" Q = ∫_Σ J^μ dΣ_μ is the same on all Cauchy surfaces Σ — the charge is conserved. But in GR, the situation is more subtle: for total energy-momentum, there is no globally conserved current unless spacetime has a Killing vector symmetry. This subtlety — the reason energy is not globally conserved in general GR — goes to the heart of what makes GR different from special relativistic field theories. [Misner, Thorne, Wheeler (1973), §15.3; Wald (1984), §12.2.]

---

## References

- Green, G. (1828). *An Essay on the Application of Mathematical Analysis to the Theories of Electricity and Magnetism*. Nottingham. [Contains Green's theorem and Green's functions for the Laplacian.]
- Griffiths, D.J. (2017). *Introduction to Electrodynamics*, 4th ed. Cambridge University Press. [§1.3 for the integral theorems in the context of electromagnetism.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation*. W.H. Freeman. [§15.3 on conservation laws in GR via the covariant divergence theorem.]
- Spivak, M. (1965). *Calculus on Manifolds*. W.A. Benjamin. [The culmination of the book is the generalized Stokes' theorem (Chapter 5). The cleanest modern proof.]
- Wald, R.M. (1984). *General Relativity*. University of Chicago Press. [§12.2 on the non-existence of global energy-momentum conservation in general GR.]
