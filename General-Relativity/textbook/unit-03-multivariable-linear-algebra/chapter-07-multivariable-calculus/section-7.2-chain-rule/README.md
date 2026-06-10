# Section 7.2: The Chain Rule and Directional Derivatives

---

## Section Introduction

The chain rule is the most important differentiation tool in multivariable calculus. It tells us how to differentiate a composition f ∘ g, where g: ℝⁿ → ℝᵐ and f: ℝᵐ → ℝᵏ. The answer: the Jacobian of the composition is the product of the Jacobians.

This is exactly the statement that differentiation is a **functor**: it assigns to each smooth map a linear map (its Jacobian), and it is compatible with composition (the chain rule). This functorial perspective is the foundation of the theory of manifolds.

---

## 7.2.1 The Chain Rule in Multiple Dimensions

**Theorem** (Chain Rule): Let g: ℝⁿ → ℝᵐ be differentiable at **a** and f: ℝᵐ → ℝᵏ be differentiable at g(**a**). Then f ∘ g: ℝⁿ → ℝᵏ is differentiable at **a**, and:

$$D(f \circ g)(\mathbf{a}) = Df(g(\mathbf{a})) \cdot Dg(\mathbf{a})$$

In terms of Jacobian matrices: **J**_{f∘g}(**a**) = **J**_f(g(**a**)) · **J**_g(**a**).

In components: if **y** = g(**x**) and **z** = f(**y**), then:

$$\frac{\partial z_i}{\partial x_j} = \sum_{k=1}^m \frac{\partial z_i}{\partial y_k} \frac{\partial y_k}{\partial x_j}$$

This is matrix multiplication of the Jacobians.

**Proof**: Write f(g(**a**+**h**)) − f(g(**a**)) as follows. Let **k** = g(**a**+**h**) − g(**a**) = Dg(**a**)**h** + o(|**h**|). Then f(g(**a**)+**k**) − f(g(**a**)) = Df(g(**a**))**k** + o(|**k**|) = Df(g(**a**))Dg(**a**)**h** + o(|**h**|). Dividing by |**h**| and taking the limit gives the chain rule. □

**Special case (scalar output)**: If f: ℝⁿ → ℝ and **x**(t): ℝ → ℝⁿ, then:

$$\frac{d}{dt} f(\mathbf{x}(t)) = \nabla f(\mathbf{x}(t)) \cdot \dot{\mathbf{x}}(t) = \sum_{i=1}^n \frac{\partial f}{\partial x_i} \frac{dx_i}{dt}$$

This is the "dot product of gradient with velocity" formula used throughout physics.

---

## 7.2.2 Directional Derivatives

**Definition**: The **directional derivative** of f: ℝⁿ → ℝ at **a** in the direction **v** (a unit vector) is:

$$D_\mathbf{v} f(\mathbf{a}) = \lim_{t \to 0} \frac{f(\mathbf{a} + t\mathbf{v}) - f(\mathbf{a})}{t}$$

**Theorem**: If f is differentiable at **a**, then D_**v** f(**a**) = ∇f(**a**) · **v** = Df(**a**)(**v**).

*Proof*: Apply the chain rule to f(**a** + t**v**) at t = 0: d/dt[f(**a**+t**v**)]|_{t=0} = Df(**a**)(**v**). □

The directional derivative is the rate of change of f in any specified direction. It is the inner product of the gradient with the direction vector.

**When partial derivatives exist but directional derivatives don't**: If only the partial derivatives (along coordinate axes) exist, but f is not differentiable, then off-axis directional derivatives may not exist or may not equal ∇f · **v**.

---

## 7.2.3 The Gradient in Curvilinear Coordinates

In Cartesian coordinates, ∇f = (∂f/∂x, ∂f/∂y, ∂f/∂z). In other coordinate systems, the gradient takes different forms.

**Polar coordinates** (r, θ) in ℝ²: x = r cos θ, y = r sin θ.

By the chain rule: ∂f/∂r = (∂f/∂x)(∂x/∂r) + (∂f/∂y)(∂y/∂r) = (∂f/∂x) cos θ + (∂f/∂y) sin θ.

The gradient in polar coordinates is: ∇f = (∂f/∂r) **ê**_r + (1/r)(∂f/∂θ) **ê**_θ.

The factor 1/r in the θ-component accounts for the fact that the arc length element in the θ-direction is r dθ, not just dθ.

**Spherical coordinates** (r, θ, φ): the gradient is ∇f = (∂f/∂r) **ê**_r + (1/r)(∂f/∂θ) **ê**_θ + (1/(r sin θ))(∂f/∂φ) **ê**_φ.

In general curvilinear coordinates (u¹, ..., uⁿ) with metric gᵢⱼ (the matrix of inner products of coordinate basis vectors), the gradient components satisfy:

$$(\nabla f)^i = g^{ij} \frac{\partial f}{\partial u^j}$$

where g^{ij} is the inverse metric. This is the direct precursor to raising indices with the metric in GR.

---

## 7.2.4 Change of Variables and the Chain Rule

The chain rule is the tool for changing variables in derivatives. This arises constantly in physics: changing from Cartesian to polar coordinates, from lab frame to center-of-mass frame, from coordinate basis to orthonormal basis.

**Example**: The wave equation ∂²f/∂t² = c²∂²f/∂x² in (x, t) coordinates. Change variables to u = x − ct, v = x + ct (light-cone coordinates). By the chain rule:

$$\frac{\partial}{\partial x} = \frac{\partial u}{\partial x}\frac{\partial}{\partial u} + \frac{\partial v}{\partial x}\frac{\partial}{\partial v} = \frac{\partial}{\partial u} + \frac{\partial}{\partial v}$$

$$\frac{\partial}{\partial t} = \frac{\partial u}{\partial t}\frac{\partial}{\partial u} + \frac{\partial v}{\partial t}\frac{\partial}{\partial v} = -c\frac{\partial}{\partial u} + c\frac{\partial}{\partial v}$$

Substituting into the wave equation: ∂²f/∂u∂v = 0. The general solution is f(u, v) = F(u) + G(v) = F(x − ct) + G(x + ct) — a right-moving wave plus a left-moving wave. Light-cone coordinates diagonalize the wave operator, just as null coordinates diagonalize the Minkowski metric in GR.

---

## References

- Apostol, T.M. (1974). *Mathematical Analysis*, 2nd ed. Addison-Wesley. [Chapter 12.6 on the chain rule in ℝⁿ.]
- Spivak, M. (1965). *Calculus on Manifolds*. W.A. Benjamin. [Chapter 2; the chain rule is Theorem 2-2.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation*. W.H. Freeman. [Chapter 3 on tensor algebra; the gradient as a 1-form is explained in §3.2.]
