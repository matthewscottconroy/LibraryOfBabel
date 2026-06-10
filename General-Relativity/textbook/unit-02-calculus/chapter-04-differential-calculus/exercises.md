# Chapter 4 Exercises: Differential Calculus

---

## Section 4.1: Limits and Continuity

**Exercise 4.1.1** *(ε-δ from scratch)*  
Prove from the ε-δ definition that lim_{x→4} √x = 2. (Hint: |√x − 2| = |x − 4|/|√x + 2|. The denominator is your friend.)

**Exercise 4.1.2** *(the δ depends on ε)*  
Prove that the function f(x) = x² is not uniformly continuous on ℝ. (A function is uniformly continuous if: ∀ε > 0, ∃δ > 0 such that |x − y| < δ ⟹ |f(x) − f(y)| < ε for all x, y. Show that no single δ works for all x, y simultaneously.) Contrast this with: f is uniformly continuous on [0, 1].

**Exercise 4.1.3** *(order matters)*  
For each of the following, determine whether the limit exists, and if so, find its value. Justify rigorously:
- (a) lim_{x→0} sin(1/x)
- (b) lim_{x→0} x sin(1/x)  
- (c) lim_{x→0⁺} x^x
- (d) lim_{x→∞} (1 + 1/x)^x

**Exercise 4.1.4** *(building continuity)*  
Define f: ℝ → ℝ by f(x) = x² if x is rational, f(x) = 0 if x is irrational. Prove that f is continuous only at x = 0, and discontinuous everywhere else. (Hint: use the density of both ℚ and ℝ\ℚ in ℝ.)

**Exercise 4.1.5** *(IVT in action)*  
Prove that the equation x⁵ − 3x + 1 = 0 has at least three real solutions. (Find three sign changes.) Then prove that for any continuous function f: [0, 1] → [0, 1], there is a fixed point: some c with f(c) = c. (Apply IVT to g(x) = f(x) − x.)

**Exercise 4.1.6** *(quantifier analysis)*  
Write out the precise ε-δ definition for each of the following, using quantifiers (∀, ∃):
- (a) lim_{x→∞} f(x) = L
- (b) lim_{x→a} f(x) = ∞
- (c) f is continuous from the right at a

Then write out the negation of each — the condition for the statement to fail.

---

## Section 4.2: The Derivative

**Exercise 4.2.1** *(pure definition)*  
Use the limit definition of the derivative (do not use rules) to compute:
- (a) f'(x) for f(x) = 1/x (at x ≠ 0)
- (b) f'(x) for f(x) = √x (at x > 0)
- (c) f'(0) for f(x) = x|x|

**Exercise 4.2.2** *(differentiability is fragile)*  
Consider f(x) = x^α for α > 0 (defined for x ≥ 0). For which values of α is f differentiable at x = 0? For which values is f twice differentiable at x = 0? Generalize: f is n-times differentiable at 0 iff α ≥ n, and Cⁿ at 0 iff α ≥ n (for integer n).

**Exercise 4.2.3** *(product rule proof)*  
The product rule says (fg)' = f'g + fg'. By induction, prove the **general Leibniz rule**:

$$(fg)^{(n)} = \sum_{k=0}^{n} \binom{n}{k} f^{(k)} g^{(n-k)}$$

Observe the formal analogy with the binomial theorem. This is not a coincidence — both arise from the same algebraic structure.

**Exercise 4.2.4** *(chain rule iteration)*  
Let f(x) = sin(e^{sin x}). Compute f'(x) by applying the chain rule repeatedly. How many applications of the chain rule are needed? Write the answer in closed form.

**Exercise 4.2.5** *(nowhere differentiable — exploration)*  
Define W_N(x) = Σ_{n=0}^{N} (1/2)^n cos(3^n π x). For N = 1, 2, 3, 5, 10, plot W_N on [0, 2] (using any computational tool). Observe how the graph becomes increasingly jagged as N increases. The limit N → ∞ is the Weierstrass function. Write a short explanation (1 paragraph) of why a function can be continuous at every point but differentiable at none.

---

## Section 4.3: Applications of the Derivative

**Exercise 4.3.1** *(MVT with content)*  
Prove the following using the Mean Value Theorem:
- (a) |sin x − sin y| ≤ |x − y| for all x, y ∈ ℝ.
- (b) If f'(x) ≤ M for all x ∈ [a, b], then f(b) − f(a) ≤ M(b − a).
- (c) Prove that eˣ ≥ 1 + x for all x ∈ ℝ, with equality only at x = 0.

**Exercise 4.3.2** *(Taylor approximation quality)*  
Let f(x) = ln(1 + x) for x > −1.

- (a) Compute the nth-degree Taylor polynomial of f about a = 0.
- (b) For what values of x does the Taylor series converge to f(x)? (You will need the remainder estimate.)
- (c) Use the 5th-degree Taylor polynomial to approximate ln(1.5). Estimate the error. Compare with the true value.
- (d) Show that ln 2 = 1 − 1/2 + 1/3 − 1/4 + ⋯ by substituting x = 1. (The convergence at x = 1 requires a separate argument — Abel's theorem — but the series identity is correct.)

**Exercise 4.3.3** *(optimization — light in gravity)*  
In a medium where the speed of light depends on position as c(x, y) (a **gradient-index medium**), light travels along paths that minimize total travel time T = ∫ ds/c, where ds is arc length. For a layer-cake medium where c depends only on y (not x) and changes smoothly, use the Euler-Lagrange equation to derive Snell's law: c₁/sin θ₁ = c₂/sin θ₂, where θ is the angle to the vertical at each layer. (This is the variational version of Fermat's principle. No Lagrangian machinery needed — set up T as an integral and minimize.)

**Exercise 4.3.4** *(L'Hôpital and indeterminate forms)*  
Evaluate the following limits, using L'Hôpital's rule (possibly multiple times) or rewriting as 0/0 or ∞/∞:
- (a) lim_{x→0} (eˣ − 1 − x − x²/2) / x³
- (b) lim_{x→∞} x^{1/x}
- (c) lim_{x→0⁺} x^x
- (d) lim_{x→0} (cos x)^{1/x²}

For (d): the answer is e^{-1/2}. Identify the form, take logarithms, and apply L'Hôpital.

---

## Section 4.4: Implicit Differentiation and Related Rates

**Exercise 4.4.1** *(implicit surfaces in physics)*  
The level set of a function F(x, y, z) = c defines a surface in 3D. Suppose a particle moves on this surface: F(x(t), y(t), z(t)) = c for all t.

- (a) Differentiate with respect to t to show that (∇F) · v = 0, where v = (ẋ, ẏ, ż) is the velocity. Conclude that ∇F is always perpendicular to the surface.
- (b) The surface x² + y² + z² = R² is a sphere. Verify that ∇F = 2(x, y, z) is indeed always perpendicular to any velocity along the sphere.
- (c) Why is this relevant to constraint forces in mechanics? (A constraint force does no work — think about what "no work" means in terms of F · v = 0.)

**Exercise 4.4.2** *(the Schwarzschild metric — a glimpse ahead)*  
In Schwarzschild geometry, the radial coordinate r and time coordinate t for a radially infalling particle satisfy:

$$\left(\frac{dr}{dt}\right)^2 = \left(1 - \frac{r_s}{r}\right)^2 \left[\left(\frac{E}{m c^2}\right)^2 - \left(1 - \frac{r_s}{r}\right)\right]$$

where rₛ = 2GM/c² is the Schwarzschild radius and E/mc² is a constant. This is an implicit relationship between r and t.

- (a) For a particle falling from rest at r = ∞ (so E/mc² = 1), simplify the equation. What is dr/dt at r = 2rₛ?
- (b) By differentiating the simplified equation with respect to t, find d²r/dt².
- (c) As r → rₛ, what happens to dr/dt? What does this say about whether an outside observer sees the particle cross the Schwarzschild radius?

(This exercise previews Chapter 43 on the Schwarzschild solution. The math is just implicit differentiation; the physics is profound.)

---

## Thought Experiments

**Thought Experiment 4.1** *(the shape of derivatives)*  
Imagine a function f: ℝ → ℝ whose derivative f' is also a continuous function. Is f' necessarily differentiable? Construct an example of a differentiable function whose derivative is continuous but not differentiable. (Hint: consider f(x) = x|x|.) This illustrates that the classes C⁰ ⊃ C¹ ⊃ C² ⊃ ⋯ ⊃ C∞ are genuinely nested — C¹ does not imply C².

Now think about what this means for spacetime: GR requires the metric to be at least C² (so the curvature tensor is well-defined). What would it mean physically for spacetime to be only C¹ but not C²? (The curvature would not be well-defined. This is exactly what happens at a thin shell of matter — matched across the shell using junction conditions.)

**Thought Experiment 4.2** *(differentiability and predictability)*  
Newton's laws say F = ma — the force determines the second derivative of position. If we know the initial position x(0) and velocity v(0), and if F is a nice function, then the future trajectory is uniquely determined (Picard-Lindelöf theorem, Chapter 10). 

What happens if the force function is not differentiable? Can the uniqueness of solutions fail? (Look up: Norton's dome — a frictionless dome shape whose equation allows both a stationary particle and a particle that spontaneously begins rolling, starting at any time t₀ > 0. The dome's profile involves a non-analytic function.) Does this worry you about physical reality? Should it?

**Thought Experiment 4.3** *(limits of measurement)*  
The ε-δ definition says: the limit lim_{x→a} f(x) = L exists if we can make f(x) *arbitrarily* close to L. But in physical measurement, we can never achieve arbitrarily small ε — instruments have finite precision. Does this mean limits are not physically meaningful? Or are limits idealizations that physical reality approximates? How does this bear on the mathematical requirement in GR that spacetime be a smooth manifold (infinitely differentiable at every point)? Is smoothness a physical claim or a mathematical convenience?

**Thought Experiment 4.4** *(the derivative in time)*  
Position is a function of time; velocity is its derivative; acceleration is the second derivative. In GR, the "acceleration" of a freely falling body is *zero* in the geometric sense — free fall follows a geodesic, which is the generalization of "straight line." Yet from the perspective of an observer on Earth's surface, the same body appears to accelerate at g ≈ 9.8 m/s². How can the same motion be both zero acceleration (in spacetime geometry) and nonzero acceleration (in the observer's reference frame)? The resolution involves the distinction between covariant derivatives and ordinary derivatives — but before you can understand that distinction, make sure you have a clean mental model of what the ordinary derivative does and does not capture.

---

## Laboratory Projects

**Lab 4.1** *(numerical derivatives and roundoff)*  
The finite difference approximation f'(a) ≈ [f(a+h) − f(a)]/h has two sources of error: **truncation error** (from the limit approximation, O(h)) and **roundoff error** (from floating-point arithmetic, O(ε_machine/h)). 

- Implement this for f(x) = sin x at x = 1, using values of h from 10⁻¹ to 10⁻¹⁶.
- Plot the actual error |f'_numerical − f'_exact| vs h on a log-log scale.
- Observe the optimal h where total error is minimized, and estimate it from the formula h_opt ≈ √(ε_machine).
- Repeat using the **centered difference** f'(a) ≈ [f(a+h) − f(a−h)]/(2h), which has truncation error O(h²). Where is the new optimal h?

This experiment reveals why "taking h small" does not always improve numerical differentiation. The lesson generalizes to all numerical algorithms: mathematical and computational convergence are different things.

**Lab 4.2** *(Taylor polynomial convergence)*  
Pick two functions: f(x) = sin x and g(x) = 1/(1 − x).

- For each, compute T_n(x) (the nth Taylor polynomial about a = 0) for n = 1, 3, 5, 7, 9 (for sin) and n = 1, 2, 3, 5, 10 (for g).
- Plot each T_n and the original function on [−3, 3] (for sin) and [−0.9, 0.9] (for g).
- For g(x) = 1/(1−x), observe that T_n converges to g on |x| < 1 but diverges on |x| > 1. The radius of convergence is 1 — the distance to the nearest singularity (x = 1). This previews Chapter 12 on complex analysis: the radius of convergence of a Taylor series is determined by the nearest singularity *in the complex plane*.
- Estimate the remainder |f(x) − T_n(x)| at x = 0.5 for n = 5, 10, 20 for sin x. Does it match the theoretical Lagrange remainder bound?

**Lab 4.3** *(the action principle — numerical)*  
The principle of stationary action says that the physical trajectory minimizes (or more precisely, makes stationary) the action S = ∫_{t₁}^{t₂} (T − V) dt, where T = mv²/2 and V is potential energy. For a particle in a quadratic potential V(x) = kx²/2 with initial conditions x(0) = 0, x(T) = d (fixed endpoints):

- Discretize time into N steps. Represent the path as a vector (x₁, x₂, ..., x_{N-1}).
- Compute the discretized action S as a sum.
- Use a numerical minimizer (gradient descent or any optimization routine) to find the path that minimizes S.
- Compare with the exact analytic solution (simple harmonic oscillator).
- Now try V(x) = 0 (free particle) and V(x) = −gx (gravity). Verify that the minimum-action path is a straight line (free) or parabola (gravity).

This lab demonstrates that the laws of physics can be stated as optimization problems — the foundation of the variational approach to mechanics and GR.
