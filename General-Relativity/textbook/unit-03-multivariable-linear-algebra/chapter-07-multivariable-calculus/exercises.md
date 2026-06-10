# Chapter 7 Exercises: Multivariable Calculus

---

## Section 7.1: Partial Derivatives and Differentiability

**Exercise 7.1.1** *(computing partial derivatives)*  
Find all first and second partial derivatives of each function:
- (a) f(x, y) = x³y − xy³
- (b) f(x, y, z) = ln(x² + y² + z²)
- (c) f(x, y) = arctan(y/x) (for x > 0)
- (d) f(x, y) = ∫₀ˣ e^{-t²} cos(ty) dt (apply the Leibniz rule for differentiating under the integral sign)

**Exercise 7.1.2** *(differentiability and continuity)*  
Define f: ℝ² → ℝ by f(x, y) = xy(x² − y²)/(x² + y²) for (x,y) ≠ (0,0) and f(0,0) = 0.
- (a) Compute ∂f/∂x(0,0) and ∂f/∂y(0,0) directly from the definition.
- (b) Compute ∂²f/∂x∂y(0,0) and ∂²f/∂y∂x(0,0). Are they equal?
- (c) Verify that the mixed partial derivatives are discontinuous at (0,0). Identify which hypothesis of Clairaut's theorem fails.

**Exercise 7.1.3** *(the Hessian and second-order analysis)*  
Find and classify all critical points of each function using the Hessian:
- (a) f(x, y) = x³ − 3xy² (monkey saddle — a saddle point with three descending valleys)
- (b) f(x, y) = x⁴ + y⁴ − 4xy (Analyze: what does H tell you at the critical points? What extra work is needed?)
- (c) f(x, y, z) = x² + 2y² + 3z² − 2xy − 2yz

---

## Section 7.2: Chain Rule and Directional Derivatives

**Exercise 7.2.1** *(chain rule computations)*  
- (a) Let f(x, y) = x²y and x(t) = cos t, y(t) = sin t. Compute df/dt (i) by substituting and differentiating, (ii) by the chain rule. Verify they agree.
- (b) Let f(u, v) = u ln v, u = xy, v = x + y. Find ∂f/∂x and ∂f/∂y.
- (c) Let z = f(x, y) and let x = r cos θ, y = r sin θ. Express ∂z/∂r and ∂z/∂θ in terms of ∂z/∂x and ∂z/∂y.

**Exercise 7.2.2** *(directional derivatives and gradient)*  
For f(x, y, z) = x² + 2y² + 3z²:
- (a) Find ∇f at the point (1, 1, 1).
- (b) Find the directional derivative at (1,1,1) in the direction of **v** = (1, 2, −1)/√6.
- (c) In what direction does f increase most rapidly at (1,1,1)? What is the rate of increase in that direction?
- (d) Find the equation of the tangent plane to the level surface f = 6 at the point (1,1,1).

**Exercise 7.2.3** *(coordinate transformations in GR)*  
In special relativity, the Lorentz boost from frame S to S' (moving at velocity v in the x-direction) is:

$$t' = \gamma(t - vx/c^2), \quad x' = \gamma(x - vt), \quad y' = y, \quad z' = z$$

where γ = (1 − v²/c²)^{-1/2}.
- (a) Compute the Jacobian matrix ∂(t', x', y', z')/∂(t, x, y, z).
- (b) Verify that det(J) = 1. What does this mean for volume preservation?
- (c) Use the chain rule to find ∂/∂t in terms of ∂/∂t' and ∂/∂x'. This gives the time derivative operator in terms of the boosted frame operators.
- (d) The d'Alembertian operator □ = −∂²/∂t² + ∂²/∂x² + ∂²/∂y² + ∂²/∂z² (in units c=1) is Lorentz-invariant. Verify that ∂²/∂t² − ∂²/∂x² has the same form in the primed coordinates.

---

## Section 7.3: Implicit Function Theorem

**Exercise 7.3.1** *(implicit surfaces)*  
For each equation, determine where the IFT allows us to solve for y as a function of x (or z as a function of (x,y)), and find the derivative.
- (a) x³ + y³ − 3xy = 0 (the folium of Descartes). Near which points does the IFT apply? Find dy/dx.
- (b) x² + y² + z² − 2xyz = 1. Find ∂z/∂x near the point (1, 0, 1).
- (c) The equation E(r, θ) = r − θ tan θ = 0 (Kepler's equation for θ as a function of r). Where does the IFT guarantee θ is a smooth function of r?

**Exercise 7.3.2** *(Lagrange multipliers)*  
- (a) Find the maximum and minimum values of f(x, y) = xy subject to x² + y² = 1.
- (b) Find the point on the plane 2x + 3y + z = 6 closest to the origin.
- (c) *(Classical mechanics)* For a system with two masses m₁ = 1 kg and m₂ = 2 kg, find the maximum kinetic energy T = (m₁v₁² + m₂v₂²)/2 subject to the constraints that total momentum p = m₁v₁ + m₂v₂ = 3 kg⋅m/s and total mechanical energy E = T = 2 J. (The constraint that energy equals kinetic energy means potential energy is zero.)

---

## Section 7.4: Multiple Integrals

**Exercise 7.4.1** *(evaluating multiple integrals)*  
- (a) ∬_D x² y dA where D is the triangle with vertices (0,0), (1,0), (0,1).
- (b) ∭_V z dV where V is the ball x² + y² + z² ≤ 1. (Use spherical coordinates.)
- (c) ∬_{x²+y²≤1} e^{-(x²+y²)} dA. (Convert to polar; the result involves the error function.)
- (d) ∭_V √(x²+y²) dV where V is the cylinder x²+y² ≤ 1, 0 ≤ z ≤ 2.

**Exercise 7.4.2** *(change of variables in GR context)*  
The flat Minkowski metric in Cartesian coordinates is ds² = −c²dt² + dx² + dy² + dz². In spherical coordinates (t, r, θ, φ):

$$ds^2 = -c^2 dt^2 + dr^2 + r^2 d\theta^2 + r^2 \sin^2\theta \, d\phi^2$$

- (a) Verify this by computing the Jacobian of the coordinate transformation (t,x,y,z) → (t,r,θ,φ) and applying the tensor transformation law g'_{μν} = (∂x^α/∂x'^μ)(∂x^β/∂x'^ν) gαβ.
- (b) The invariant volume element in these coordinates is √(−g) dt dr dθ dφ. Compute g = det(gᵤᵥ) and hence √(−g).
- (c) Verify that ∫∫∫ r² sin θ dr dθ dφ gives the correct volume of a ball of radius R.

---

## Thought Experiments

**Thought Experiment 7.1** *(the meaning of the total derivative)*  
The total derivative Df(**a**) of f: ℝⁿ → ℝᵐ at **a** is a linear map. Why should the derivative be a linear map?

Here is one way to see it: think of the derivative as the "best linear approximation" to f near **a**. A linear approximation must satisfy superposition (f(λ**h**) ≈ λ f(**h**), f(**h** + **k**) ≈ f(**h**) + f(**k**)). This is not a choice but a requirement if we want an approximation that captures the local structure of f consistently.

Now: in differential geometry, the "tangent map" or "pushforward" df_p: T_pM → T_{f(p)}N is the derivative of a smooth map f: M → N between manifolds. It is linear by the same logic — it is the best linear approximation to f at p. The total derivative of this chapter is the special case M = ℝⁿ, N = ℝᵐ.

Explain in your own words: why must the derivative of a smooth map between manifolds be linear?

**Thought Experiment 7.2** *(Lagrange multipliers and constraint forces)*  
In mechanics, a particle constrained to a surface experiences a **normal force** — a force perpendicular to the surface that maintains the constraint. The Lagrange multiplier λ is related to this constraint force.

Specifically, in the Lagrange multiplier method for optimizing f subject to g = 0, the condition ∇f = λ ∇g says: at the extremum, the force ∇f (the "unconstrained" tendency) is balanced by the constraint force λ ∇g (normal to the constraint surface).

In GR, a timelike geodesic is a path that "extremizes" proper time. The constraint that the path lies on the manifold (satisfies the metric condition) plays the role of the constraint in the Lagrange multiplier problem. The Christoffel symbols Γᵅ_{μν} are, in a sense, the "Lagrange multipliers" that enforce the constraint. Discuss this analogy.

---

## Laboratory Projects

**Lab 7.1** *(visualizing gradient fields)*  
For f(x, y) = sin(x) cos(y):
- Plot the function as a surface and as a contour map.
- Compute and plot the gradient field ∇f.
- Verify visually that ∇f is perpendicular to the level curves at each point.
- Find all critical points in [0, 2π] × [0, 2π] and classify them using the Hessian.

**Lab 7.2** *(numerical Jacobian and condition number)*  
For the function f: ℝ² → ℝ², f(x,y) = (x² − y², 2xy) (complex squaring):
- Compute the Jacobian analytically.
- Verify that det(J) = 4(x² + y²).
- Where is the Inverse Function Theorem inapplicable?
- Numerically compute the Jacobian using finite differences at (1, 1) and compare with the analytic result.
- Compute the **condition number** of the Jacobian (ratio of largest to smallest singular value). This measures how much the map distorts volumes locally.
