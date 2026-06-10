# Section 7.1: Partial Derivatives and Differentiability

---

## Section Introduction

The simplest way to differentiate a function of several variables is to vary one variable at a time, holding the others fixed. This gives **partial derivatives** — the slopes of the cross-sectional curves of the function's graph. Partial derivatives are computed just like ordinary derivatives, using the single-variable rules.

But partial derivatives can exist without the function being differentiable in the full multivariable sense. Differentiability — the existence of a good linear approximation — is a stronger condition, and the right one for building the theory.

---

## 7.1.1 Partial Derivatives

**Definition**: For f: ℝⁿ → ℝ and a point **a** = (a₁, ..., aₙ), the **partial derivative of f with respect to xᵢ at a** is:

$$\frac{\partial f}{\partial x_i}(\mathbf{a}) = \lim_{h \to 0} \frac{f(a_1, \ldots, a_i + h, \ldots, a_n) - f(a_1, \ldots, a_n)}{h}$$

if this limit exists.

**Computation**: Treat all variables other than xᵢ as constants and differentiate with respect to xᵢ using the ordinary rules.

**Examples**:
- f(x, y) = x²y³. Then ∂f/∂x = 2xy³, ∂f/∂y = 3x²y².
- f(x, y) = sin(xy). Then ∂f/∂x = y cos(xy), ∂f/∂y = x cos(xy).
- f(x, y, z) = e^{x²+y²+z²}. Then ∂f/∂x = 2x e^{x²+y²+z²} (and similarly for y, z).

**Geometric meaning**: ∂f/∂x(a, b) is the slope of the curve z = f(x, b) at x = a (fixing y = b). It is the rate of change of f in the x-direction.

**Higher-order partial derivatives**: 

$$\frac{\partial^2 f}{\partial x_j \partial x_i} = \frac{\partial}{\partial x_j}\left(\frac{\partial f}{\partial x_i}\right)$$

For most functions encountered in physics, **Clairaut's theorem** holds: mixed partials are equal.

**Theorem** (Clairaut/Schwarz): If f has continuous second partial derivatives on an open set, then ∂²f/∂xᵢ∂xⱼ = ∂²f/∂xⱼ∂xᵢ.

*Proof outline*: The four-point argument: f(a+h, b+k) − f(a+h, b) − f(a, b+k) + f(a, b) equals both (∂²f/∂x∂y)(c₁) hk and (∂²f/∂y∂x)(c₂) hk for nearby points c₁, c₂; as h, k → 0, both → (∂²f/∂x∂y)(a,b), so the mixed partials are equal. □

*Counterexample when continuity fails*: f(x, y) = xy(x² − y²)/(x² + y²) for (x,y) ≠ (0,0) and f(0,0) = 0. Here ∂²f/∂x∂y(0,0) = 1 ≠ −1 = ∂²f/∂y∂x(0,0). The partial derivatives exist everywhere but are not continuous at the origin. [This example is in Rudin (1976), p. 235.]

---

## 7.1.2 The Total Derivative and Differentiability

Having all partial derivatives at a point does not guarantee the function is differentiable there. The correct notion of differentiability in multiple dimensions is:

**Definition**: f: ℝⁿ → ℝᵐ is **differentiable at a** if there exists a linear map L: ℝⁿ → ℝᵐ such that:

$$\lim_{\mathbf{h} \to \mathbf{0}} \frac{\|f(\mathbf{a} + \mathbf{h}) - f(\mathbf{a}) - L\mathbf{h}\|}{\|\mathbf{h}\|} = 0$$

If such L exists, it is unique, called the **total derivative** or **Fréchet derivative** of f at **a**, written Df(**a**) or f'(**a**).

In coordinates: L is the **Jacobian matrix** J = (∂fᵢ/∂xⱼ), an m×n matrix. The derivative maps the displacement vector **h** ∈ ℝⁿ to the resulting displacement L**h** ∈ ℝᵐ.

**Theorem**: If all partial derivatives ∂fᵢ/∂xⱼ exist and are continuous near **a**, then f is differentiable at **a**, and the Jacobian is the matrix of the total derivative.

*Note*: Existence of partial derivatives alone does not imply differentiability. The continuity of the partial derivatives is the key additional condition.

**Counterexample** (partial derivatives exist, function not differentiable):

$$f(x, y) = \begin{cases} \frac{xy}{x^2 + y^2} & (x,y) \neq (0,0) \\ 0 & (x,y) = (0,0) \end{cases}$$

Both ∂f/∂x(0,0) and ∂f/∂y(0,0) exist and equal 0. But f is not continuous at (0,0): along the line y = x, f(x,x) = x²/(2x²) = 1/2 → 1/2 ≠ 0 = f(0,0). So f is not differentiable at (0,0). The issue: the partial derivatives probe only the coordinate axes, missing the behavior along other directions.

---

## 7.1.3 The Gradient

For f: ℝⁿ → ℝ (scalar-valued), the total derivative at **a** is a linear map from ℝⁿ to ℝ — i.e., a **covector** or 1-form. In the standard basis, it is represented by the **gradient**:

$$\nabla f(\mathbf{a}) = \left(\frac{\partial f}{\partial x_1}(\mathbf{a}), \frac{\partial f}{\partial x_2}(\mathbf{a}), \ldots, \frac{\partial f}{\partial x_n}(\mathbf{a})\right)$$

The linear approximation becomes: f(**a** + **h**) ≈ f(**a**) + ∇f(**a**) · **h**, where · is the dot product.

**Geometric meaning**: ∇f points in the direction of steepest ascent of f. The magnitude |∇f| is the rate of change in that direction. Level sets f = c are perpendicular to ∇f.

**Proof**: The directional derivative of f in direction **v** (with |**v**| = 1) is Df(**a**)(**v**) = ∇f(**a**) · **v** = |∇f| cos θ, where θ is the angle between ∇f and **v**. This is maximized when θ = 0, i.e., **v** = ∇f/|∇f|. □

**In GR**: The gradient is a covector (1-form). The distinction between vectors and covectors — between directions and "measuring" linear functions — is fundamental in tensor analysis. The gradient of a scalar field is naturally a covector, not a vector. We will formalize this in Chapter 9 and Chapter 27.

---

## 7.1.4 The Hessian

The second-order behavior of f near **a** is captured by the **Hessian matrix**:

$$H_f(\mathbf{a}) = \left(\frac{\partial^2 f}{\partial x_i \partial x_j}(\mathbf{a})\right)_{i,j=1}^n$$

By Clairaut's theorem, H is symmetric (when f is C²).

The second-order Taylor approximation is:

$$f(\mathbf{a} + \mathbf{h}) \approx f(\mathbf{a}) + \nabla f(\mathbf{a}) \cdot \mathbf{h} + \frac{1}{2} \mathbf{h}^T H_f(\mathbf{a}) \mathbf{h}$$

This generalizes the 1D formula f(a+h) ≈ f(a) + f'(a)h + f''(a)h²/2.

**Second derivative test**: At a critical point (∇f = 0):
- If H is positive definite (all eigenvalues > 0): local minimum.
- If H is negative definite (all eigenvalues < 0): local maximum.
- If H has both positive and negative eigenvalues: saddle point.
- If H is singular (some eigenvalue = 0): test inconclusive.

**Connection to GR**: The Hessian appears in the analysis of extrema of the action functional. In the second variation of the action, the Hessian of the Lagrangian appears. For a geodesic, the second variation of arc length gives a Jacobi operator involving the Hessian and the curvature — the **Jacobi equation**. This determines whether the geodesic is length-minimizing (Chapter 29).

---

## References

- Apostol, T.M. (1974). *Mathematical Analysis*, 2nd ed. Addison-Wesley. [Chapters 12–13 on multivariable calculus and differentiability.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [Chapter 9 on functions of several variables; the counterexample to Clairaut's theorem is on p. 235.]
- Spivak, M. (1965). *Calculus on Manifolds*. W.A. Benjamin. [A concise treatment of multivariable calculus with full proofs; the standard preparation for differential geometry. Chapters 1–2 on differentiation in ℝⁿ.]
- Munkres, J.R. (1991). *Analysis on Manifolds*. Addison-Wesley. [A more detailed and accessible version of Spivak's *Calculus on Manifolds*.]
