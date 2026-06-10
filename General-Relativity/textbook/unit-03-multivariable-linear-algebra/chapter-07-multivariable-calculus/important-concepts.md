# Chapter 7: Important Concepts

---

**Partial Derivative**: The derivative of a function of several variables with respect to one variable while all others are held fixed. Written ∂f/∂xⁱ. Not the same as the total derivative: a function can have partial derivatives at a point while failing to be differentiable there (the partials exist but the limit from other directions fails to match).

**Total Derivative (Fréchet Derivative)**: The linear map Df(p): ℝⁿ → ℝᵐ that best approximates the change in f near p: f(p+h) = f(p) + Df(p)h + o(|h|). The matrix of Df(p) in standard coordinates is the Jacobian matrix. This is the multivariable analogue of the single-variable derivative, and the correct generalization that foreshadows the pushforward on manifolds.

**Jacobian Matrix**: The m×n matrix [∂fⁱ/∂xʲ] of partial derivatives of a map f: ℝⁿ → ℝᵐ. Represents the total derivative in standard coordinates. Determines how f stretches and rotates near each point.

**Jacobian Determinant**: The determinant of the Jacobian matrix when m = n. Measures the local volume scaling factor of the map f: volumes scale by |det(Df)|. Appears in the change-of-variables formula for multiple integrals and in the transformation of the volume element √(-g)d⁴x in GR.

**Clairaut's Theorem (Schwarz's Theorem)**: If f: U ⊂ ℝⁿ → ℝ and the mixed partial derivatives ∂²f/∂xⁱ∂xʲ and ∂²f/∂xʲ∂xⁱ exist and are continuous at p, then they are equal: ∂²f/∂xⁱ∂xʲ = ∂²f/∂xʲ∂xⁱ. The symmetry of the Hessian matrix follows. This is not true without the continuity hypothesis; the counterexample f(x,y) = xy(x²−y²)/(x²+y²) at the origin is instructive.

**Gradient**: The vector of partial derivatives ∇f = (∂f/∂x¹, ..., ∂f/∂xⁿ). Points in the direction of steepest ascent and is perpendicular to level sets. In GR, the gradient of a scalar field is a covector (1-form), not a vector — it transforms as ∂f/∂x'^i = (∂xʲ/∂x'^i)(∂f/∂xʲ), with the inverse Jacobian.

**Hessian Matrix**: The matrix of second partial derivatives [∂²f/∂xⁱ∂xʲ] at a point p. Symmetric by Clairaut's theorem. Determines the nature of critical points: positive definite → local minimum; negative definite → local maximum; indefinite → saddle point. In GR, the Hessian of the action integral (second variation) determines stability of the classical solution.

**Second Derivative Test**: At a critical point (∇f = 0), the Hessian determines the nature of the extremum. The eigenvalues of the Hessian (all positive: minimum; all negative: maximum; mixed sign: saddle) give the classification. In degenerate cases (zero eigenvalues) higher-order terms must be examined.

**Chain Rule (Multivariable)**: If f: ℝⁿ → ℝᵐ and g: ℝᵐ → ℝᵏ, then D(g ∘ f)(p) = Dg(f(p)) · Df(p) (product of Jacobian matrices). The single most important computational tool for coordinate changes in GR: the transformation law for tensor components is just the chain rule applied repeatedly.

**Directional Derivative**: The rate of change of f in a direction u: D_u f(p) = lim_{t→0} [f(p+tu) − f(p)]/t = Df(p) · u = ∇f · u. Equals the dot product of the gradient with the direction vector. In GR, the directional derivative along the 4-velocity is the "proper time derivative" d/dτ.

**Inverse Function Theorem**: If f: ℝⁿ → ℝⁿ is C¹ and Df(p) is invertible, then f is a local diffeomorphism near p: there exist open sets U ∋ p and V ∋ f(p) such that f: U → V is a bijection with C¹ inverse. The derivative of the inverse is [Df(p)]⁻¹. This is used constantly in differential geometry to show that smooth coordinate changes are valid.

**Implicit Function Theorem**: If F: ℝⁿ × ℝᵐ → ℝᵐ and D_y F(p₀, y₀) is invertible, then near (p₀, y₀) the equation F(x, y) = 0 defines y as a smooth function of x. The theorem that allows level sets to be treated as submanifolds, constrained optimization to be performed, and coordinates to be changed locally.

**Regular Value**: A value c ∈ ℝᵐ of f: ℝⁿ → ℝᵐ is a regular value if Df(p) has full rank at every preimage p ∈ f⁻¹(c). By the regular value theorem, f⁻¹(c) is a smooth (n−m)-dimensional manifold. This is how spheres, tori, and spacetime hypersurfaces arise as submanifolds of Euclidean space.

**Lagrange Multipliers**: To find extrema of f: ℝⁿ → ℝ subject to constraint g(x) = 0, solve ∇f = λ∇g for (x, λ). The geometric meaning: at a constrained extremum, the gradient of f is proportional to the gradient of the constraint — the level sets are tangent. In GR, the geodesic equation arises as an Euler-Lagrange equation with the metric constraint gᵤᵥ ẋ^μẋ^ν = −1 (timelike).

**Fubini's Theorem**: Under appropriate integrability conditions, the multiple integral ∫∫ f(x,y) dA can be evaluated as an iterated integral in either order. The key content: the order of integration doesn't matter (for Riemann-integrable functions on compact domains, or for Lebesgue-integrable functions). This is what makes calculations in ℝⁿ tractable.

**Change of Variables Formula**: For a diffeomorphism φ: U → V, ∫_V f(y) dy = ∫_U f(φ(x)) |det Dφ(x)| dx. The absolute value of the Jacobian determinant is the volume scaling factor. In GR: ∫ ℒ √(-g) d⁴x transforms correctly under coordinate changes because √(-g) supplies the Jacobian determinant of the metric.

**Volume Element √(-g) d⁴x**: The coordinate-invariant volume form on a 4-dimensional pseudo-Riemannian manifold. Under a coordinate change x → x', the metric transforms and √(-g) picks up the inverse of the Jacobian determinant, exactly compensating the transformation of d⁴x. The Einstein-Hilbert action S = ∫ R √(-g) d⁴x is well-defined precisely because of this.

**Critical Point**: A point where ∇f = 0. Critical points are where extrema can occur, but not all critical points are extrema (saddle points, degenerate points). In the calculus of variations (and GR), the field equations arise as the conditions for the action functional to be stationary — the "critical points" of the action.
