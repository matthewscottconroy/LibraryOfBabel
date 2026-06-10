# Section 4.4: Implicit Differentiation and Related Rates

---

## Section Introduction

Not every curve is the graph of a function. The unit circle x² + y² = 1 defines y as a function of x near most points — but not as a single global function, since there are two values of y for each x in (−1, 1). Implicit differentiation lets us compute slopes along such curves without solving for y explicitly. Related rates extends this: when two quantities are connected by a constraint equation, differentiating that equation with respect to time tells us how their rates of change are related.

These techniques are prerequisites for Section 7.3 on implicit functions and for the treatment of constraint surfaces in classical mechanics (Chapter 15). More abstractly, the implicit function theorem — the rigorous theorem behind implicit differentiation — is one of the central results of analysis, with direct applications to the structure of solution manifolds in GR.

---

## 4.4.1 Implicit Differentiation

**Setup**: Suppose a curve is defined by the equation F(x, y) = 0, and suppose y is implicitly a function of x near some point. We can differentiate both sides of F(x, y) = 0 with respect to x, treating y as an unknown function of x and applying the chain rule.

**Example 1**: Find dy/dx for the circle x² + y² = 1.

Differentiate both sides with respect to x:

$$2x + 2y \frac{dy}{dx} = 0 \implies \frac{dy}{dx} = -\frac{x}{y}$$

This is valid wherever y ≠ 0 (the top and bottom semicircles, separately). At the point (1/√2, 1/√2), the slope is −1 — a 45° decline, as expected from the circle's geometry.

**Example 2**: Find dy/dx for the ellipse x²/a² + y²/b² = 1.

$$\frac{2x}{a^2} + \frac{2y}{b^2} \frac{dy}{dx} = 0 \implies \frac{dy}{dx} = -\frac{b^2 x}{a^2 y}$$

**Example 3**: Find dy/dx for y⁵ + xy = 1.

$$5y^4 \frac{dy}{dx} + y + x \frac{dy}{dx} = 0 \implies (5y^4 + x)\frac{dy}{dx} = -y \implies \frac{dy}{dx} = \frac{-y}{5y^4 + x}$$

Here we cannot solve for y explicitly in terms of x (no algebraic formula for the root of a general quintic — Abel-Ruffini theorem). Yet we can differentiate.

**Why it works**: The chain rule. If y = y(x) implicitly satisfies F(x, y(x)) = 0, then differentiating with respect to x gives:

$$\frac{\partial F}{\partial x} + \frac{\partial F}{\partial y} \frac{dy}{dx} = 0$$

Solving: $\frac{dy}{dx} = -\frac{\partial F/\partial x}{\partial F/\partial y}$ (valid when ∂F/∂y ≠ 0).

This is the **implicit differentiation formula**. Partial derivatives ∂F/∂x, ∂F/∂y are introduced properly in Chapter 7; for now, they mean "differentiate with respect to x (or y) while holding the other variable constant." We will revisit this formula in Section 7.3 with full rigor.

---

## 4.4.2 The Implicit Function Theorem (Preview)

The calculation above assumed that y *is* a function of x locally. When is this assumption valid?

**Theorem** (Implicit Function Theorem, IFT — informal statement): Suppose F(x₀, y₀) = 0 and ∂F/∂y|(x₀,y₀) ≠ 0. Then near (x₀, y₀), the equation F(x, y) = 0 defines y uniquely as a smooth function of x, with derivative dy/dx = −(∂F/∂x)/(∂F/∂y).

The full theorem, in n dimensions, is one of the central results of multivariable analysis. We prove it in Section 7.3 using the **contraction mapping theorem** (fixed-point theorem). The condition ∂F/∂y ≠ 0 is the "non-degeneracy" condition that prevents the curve from having a vertical tangent at the point in question.

**In GR**: The IFT appears in multiple places. One important instance: the condition that the metric tensor gᵤᵥ is non-degenerate (det(gᵤᵥ) ≠ 0) is the GR analogue of ∂F/∂y ≠ 0 — it ensures that coordinates can be used as well-behaved variables, and that the inverse metric g^{μν} exists. When det(gᵤᵥ) = 0, something singular is happening: the coordinate system is breaking down, or the spacetime is genuinely singular.

---

## 4.4.3 Related Rates

**Setup**: Two (or more) quantities change with time, constrained by a relationship. Differentiate the relationship with respect to time to find how their rates of change are related.

**The method**:
1. Identify the relationship between the quantities (usually a geometric or physical equation).
2. Differentiate both sides with respect to time t (using the chain rule as needed).
3. Substitute known values and solve for the unknown rate.

**Example 1 (ladder problem)**: A 5 m ladder leans against a wall. The bottom slides away from the wall at 1 m/s. How fast is the top sliding down when the bottom is 3 m from the wall?

Relationship: x² + y² = 25, where x is the horizontal distance (bottom from wall) and y is the height (top on wall).

Differentiate with respect to t: $2x \frac{dx}{dt} + 2y \frac{dy}{dt} = 0$.

When x = 3: y = √(25 - 9) = 4. Substituting: 2(3)(1) + 2(4)(dy/dt) = 0, so dy/dt = −3/4 m/s. The top is sliding down at 3/4 m/s.

**Example 2 (expanding sphere)**: Gas fills a spherical balloon. Volume increases at 100 cm³/s. How fast is the radius increasing when r = 5 cm?

Relationship: V = (4/3)πr³. Differentiate: dV/dt = 4πr² · dr/dt. Substituting: 100 = 4π(25) · dr/dt, so dr/dt = 1/π ≈ 0.318 cm/s.

**Example 3 (physical — gravitational redshift)**: This foreshadows GR. A photon emitted at radius r₁ in a gravitational field is received at radius r₂ > r₁ with a lower frequency. The approximate formula (in weak fields) is Δν/ν = −GM(1/r₁ − 1/r₂)/c², where G, M, c are constants. If r₂ → ∞ and r₁ = R (the star's surface), and if R is increasing (the star is expanding at rate dR/dt), how fast is the redshift changing?

Differentiate Δν/ν = GM/(c²r₁) with respect to t (with r₁ = R(t)):

$$\frac{d}{dt}\left(\frac{\Delta\nu}{\nu}\right) = -\frac{GM}{c^2 R^2} \frac{dR}{dt}$$

A star expanding at rate dR/dt > 0 shows a decreasing gravitational redshift. This is a related-rates calculation with physical content. [Pound, R.V. and Rebka, G.A. (1959). "Gravitational Red-Shift in Nuclear Resonance." *Physical Review Letters*, 3, 439–441. The Pound-Rebka experiment measured exactly this effect in a terrestrial gravitational field.]

---

## 4.4.4 Differentials

The Leibniz notation dy/dx suggests thinking of dy and dx as small quantities whose ratio is the derivative. Making this precise requires the concept of a **differential**.

**Definition**: If f is differentiable at x, the **differential** df is defined by:

$$df = f'(x) \, dx$$

Here, dx is an independent variable (an "increment in x"), and df is the corresponding increment in f as predicted by the linear approximation. In particular, the actual change in f is:

$$\Delta f = f(x + \Delta x) - f(x) \approx f'(x) \Delta x = df$$

when Δx = dx is small.

This notation is powerful. It makes the chain rule transparent: if y = f(g(x)), then dy = f'(g(x)) · g'(x) dx. And substitution in integrals (Section 5.3) is precisely the chain rule in differential notation.

**In differential geometry**: Differentials become **1-forms** — the fundamental objects of exterior calculus. At a point p on a manifold, the differential df_p is a linear map from the tangent space T_pM to ℝ. The tensor analysis and differential form language of GR is the generalization of this simple idea to curved, multi-dimensional spaces.

The move from "f'(x) is a number" to "df is a linear functional on tangent vectors" is a shift in perspective that pays enormous dividends in Chapter 28. When we write the metric ds² = gᵤᵥ dxᵘ dx^ν, the dxᵘ are exactly this — differential 1-forms, not small numbers.

---

## References

- Courant, R. and John, F. (1989). *Introduction to Calculus and Analysis*, Vol. I. Springer. [Chapter 3 on differentiation, including implicit functions and related rates.]
- Pound, R.V. and Rebka, G.A. (1959). "Gravitational Red-Shift in Nuclear Resonance." *Physical Review Letters*, 3, 439–441. [First experimental confirmation of gravitational redshift, a direct test of GR's prediction. The differential relationship between redshift and altitude is a related-rates calculation.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [The implicit function theorem is in Chapter 9, proved via the contraction mapping principle.]
- Spivak, M. (1994). *Calculus*, 3rd ed. Publish or Perish. [Chapter 12 on differentiation; Spivak's treatment of differentials as linear maps foreshadows the differential-form perspective.]
