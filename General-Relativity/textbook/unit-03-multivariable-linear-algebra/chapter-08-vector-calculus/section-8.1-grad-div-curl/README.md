# Section 8.1: Gradient, Divergence, and Curl

---

## Section Introduction

The three differential operators of vector calculus — gradient, divergence, curl — can each be understood through the question: "what kind of thing goes in, and what comes out?" The gradient takes a scalar field and returns a vector field. The divergence takes a vector field and returns a scalar field. The curl takes a vector field and returns a vector field (in 3D). This is not coincidental — it reflects the algebraic structure of differential forms in 3D space.

---

## 8.1.1 The Gradient

For f: ℝ³ → ℝ:

$$\nabla f = \left(\frac{\partial f}{\partial x}, \frac{\partial f}{\partial y}, \frac{\partial f}{\partial z}\right)$$

**Properties**:
- ∇(f + g) = ∇f + ∇g
- ∇(fg) = f∇g + g∇f (product rule)
- ∇(f/g) = (g∇f − f∇g)/g² (quotient rule)
- ∇(f ∘ g) = (f' ∘ g)∇g (chain rule for scalar composition)

**Physical meaning**: For a scalar potential φ (gravitational or electric), the field is **F** = −∇φ. The gradient points "uphill"; the field points "downhill."

**Conservative fields**: A vector field **F** is **conservative** if **F** = ∇φ for some scalar potential φ. Conservative fields have zero work done around any closed loop: ∮ **F** · dl = 0.

---

## 8.1.2 The Divergence

For **F** = (F₁, F₂, F₃): ℝ³ → ℝ³:

$$\nabla \cdot \mathbf{F} = \frac{\partial F_1}{\partial x} + \frac{\partial F_2}{\partial y} + \frac{\partial F_3}{\partial z}$$

**Physical meaning**: ∇ · **F** at a point is the net outflow of **F** per unit volume at that point.
- ∇ · **F** > 0: source (net outward flow).
- ∇ · **F** < 0: sink (net inward flow).
- ∇ · **F** = 0: **F** is **divergence-free** (solenoidal, or **incompressible** if **F** is a flow velocity).

**Examples**:
- ∇ · **r** = ∂x/∂x + ∂y/∂y + ∂z/∂z = 3.
- ∇ · (**r**/r³) = 0 for **r** ≠ **0**. But ∫∫ (**r**/r³) · dA = 4π over any sphere surrounding the origin — the source is concentrated at **r** = 0, described by a delta function: ∇ · (**r**/r³) = 4π δ³(**r**).

This last example is Gauss's law for gravity: the divergence of the gravitational field is proportional to mass density.

**Product rules for divergence**:
- ∇ · (φ**F**) = φ(∇ · **F**) + **F** · ∇φ
- ∇ · (**F** × **G**) = **G** · (∇ × **F**) − **F** · (∇ × **G**)

---

## 8.1.3 The Curl

For **F** = (F₁, F₂, F₃):

$$\nabla \times \mathbf{F} = \left(\frac{\partial F_3}{\partial y} - \frac{\partial F_2}{\partial z}, \frac{\partial F_1}{\partial z} - \frac{\partial F_3}{\partial x}, \frac{\partial F_2}{\partial x} - \frac{\partial F_1}{\partial y}\right)$$

Equivalently (mnemonic using the determinant symbol):

$$\nabla \times \mathbf{F} = \det \begin{pmatrix} \hat{\mathbf{i}} & \hat{\mathbf{j}} & \hat{\mathbf{k}} \\ \partial/\partial x & \partial/\partial y & \partial/\partial z \\ F_1 & F_2 & F_3 \end{pmatrix}$$

**Physical meaning**: ∇ × **F** at a point is the "circulation" of **F** per unit area — the tendency of the field to rotate around that point. The direction of ∇ × **F** is the axis of rotation (by the right-hand rule).

**Examples**:
- **F** = (−y, x, 0)/r² (a circulation field). ∇ × **F** = (0, 0, 2)/r² except at the origin where there is a delta function singularity — analogous to the magnetic field of a line current.
- ∇ × (∇f) = **0** for any smooth f (the curl of a gradient is zero). This is the vector identity reflecting that d² = 0 for differential forms.
- ∇ · (∇ × **F**) = 0 for any smooth **F** (the divergence of a curl is zero). Another reflection of d² = 0.

---

## 8.1.4 The Laplacian

$$\nabla^2 f = \nabla \cdot (\nabla f) = \frac{\partial^2 f}{\partial x^2} + \frac{\partial^2 f}{\partial y^2} + \frac{\partial^2 f}{\partial z^2}$$

The Laplacian appears in:
- The **heat equation**: ∂T/∂t = κ ∇²T (temperature diffusion)
- The **wave equation**: ∂²ψ/∂t² = c² ∇²ψ
- **Laplace's equation**: ∇²φ = 0 (electrostatic/gravitational potential in vacuum)
- **Poisson's equation**: ∇²φ = −ρ/ε₀ (with sources)

**In GR**: The wave equation generalizes to the curved-spacetime wave equation □ψ ≡ g^{μν}∇_μ∇_νψ = 0 (where ∇_μ is the covariant derivative). This governs gravitational wave propagation, and its Green's functions describe how perturbations propagate on curved backgrounds.

**The vector Laplacian**: For a vector field: ∇²**F** = (∇²F₁, ∇²F₂, ∇²F₃) in Cartesian coordinates. This equals ∇(∇·**F**) − ∇×(∇×**F**) — an important identity.

---

## 8.1.5 The Key Vector Identities

These identities are used constantly in physics. They follow from the antisymmetry of the cross product and the commutativity of partial derivatives (Clairaut's theorem).

1. ∇ × (∇f) = **0** — curl of a gradient vanishes
2. ∇ · (∇ × **F**) = 0 — divergence of a curl vanishes
3. ∇²f = ∇ · (∇f) — definition of Laplacian
4. ∇²**F** = ∇(∇·**F**) − ∇×(∇×**F**) — vector Laplacian identity
5. ∇·(**F**×**G**) = **G**·(∇×**F**) − **F**·(∇×**G**)
6. ∇×(**F**×**G**) = **F**(∇·**G**) − **G**(∇·**F**) + (**G**·∇)**F** − (**F**·∇)**G**

Identities 1 and 2 are the 3D manifestations of d² = 0 in exterior algebra. They express: the composition of two consecutive "differentiations" is zero.

---

## References

- Griffiths, D.J. (2017). *Introduction to Electrodynamics*, 4th ed. Cambridge University Press. [Chapter 1 on vector calculus, with excellent physical intuition for gradient, divergence, and curl.]
- Marsden, J.E. and Tromba, A.J. (2012). *Vector Calculus*, 6th ed. W.H. Freeman. [Comprehensive coverage of all vector calculus with applications.]
- Schey, H.M. (2005). *Div, Grad, Curl, and All That*, 4th ed. W.W. Norton. [An informal but rigorous introduction to vector calculus in the context of electrostatics. The most readable introduction to the subject.]
