# Chapter 4 Important Concepts

---

**Limit** — The value that a function approaches as its input approaches a given point. Defined precisely by the ε-δ formalism: lim_{x→a} f(x) = L means ∀ε > 0 ∃δ > 0 such that 0 < |x − a| < δ ⟹ |f(x) − L| < ε.

**ε-δ definition** — The rigorous formulation of the limit concept due to Weierstrass. The ε quantifies the required closeness of the output; the δ quantifies the resulting constraint on the input. The definition requires δ to exist for every ε, no matter how small.

**Continuity** — A function f is continuous at a if lim_{x→a} f(x) = f(a). Equivalently: small changes in input produce small changes in output. A function continuous on a closed interval has strong properties (IVT, EVT).

**Continuity vs. Differentiability** — Differentiability implies continuity, but not vice versa. The function |x| is continuous at 0 but not differentiable. The Weierstrass function is continuous everywhere and differentiable nowhere. These are not pathological exceptions — they are typical of generic continuous functions.

**Derivative** — The instantaneous rate of change. f'(a) = lim_{h→0} [f(a+h) − f(a)]/h. Geometrically: the slope of the tangent line to the graph at (a, f(a)).

**Difference Quotient** — The average rate of change over [a, a+h]: [f(a+h) − f(a)]/h. The derivative is the limit of difference quotients as h → 0.

**Differentiability** — A function is differentiable at a if the difference quotient limit exists. Differentiability is a *local* property — a function can be differentiable at some points and not others.

**Leibniz Notation** — The notation dy/dx for the derivative, introduced by Leibniz. Formally, dy and dx are "differentials" — infinitesimal increments. The chain rule in Leibniz notation, dy/dx = (dy/du)(du/dx), is suggestive and correct, though dy/dx is not literally a fraction of infinitesimals.

**Tangent Line** — The line through (a, f(a)) with slope f'(a). The tangent line is the unique linear function that best approximates f near a. The existence of a tangent line is exactly what differentiability means.

**Linear Approximation** — f(a+h) ≈ f(a) + f'(a)h. The derivative gives the best linear approximation to f near a. This is the central idea that generalizes to Jacobians in multiple dimensions and to the tangent map on manifolds.

**Differential** — df = f'(x) dx. The differential of f at x is the linear map h ↦ f'(x)h. In differential geometry, differentials become 1-forms.

**Power Rule** — (xⁿ)' = nxⁿ⁻¹, valid for all real n. One of the most used rules in calculus.

**Product Rule** — (fg)' = f'g + fg'. Proved from the limit definition using the add-and-subtract trick.

**Quotient Rule** — (f/g)' = (f'g − fg')/g², valid where g ≠ 0.

**Chain Rule** — If y = f(g(x)), then dy/dx = f'(g(x)) · g'(x). The most important differentiation rule; it governs how derivatives transform under change of variables, and generalizes to the pushforward on manifolds.

**Critical Point (Stationary Point)** — A point c where f'(c) = 0. Fermat's theorem says that every local extremum of a differentiable function is a critical point. Not all critical points are extrema (e.g., inflection points).

**Second Derivative Test** — If f'(c) = 0 and f''(c) > 0, then c is a local minimum; if f''(c) < 0, a local maximum. If f''(c) = 0, the test is inconclusive.

**Mean Value Theorem (MVT)** — If f is continuous on [a, b] and differentiable on (a, b), then f'(c) = [f(b) − f(a)]/(b − a) for some c ∈ (a, b). The most important theorem of differential calculus; used in proofs throughout analysis.

**Rolle's Theorem** — The special case of the MVT where f(a) = f(b): then f'(c) = 0 for some interior c. The MVT is proved by reducing to Rolle's theorem.

**L'Hôpital's Rule** — For indeterminate forms 0/0 or ∞/∞: lim f(x)/g(x) = lim f'(x)/g'(x), under appropriate conditions. Proved via the Cauchy MVT.

**Taylor Polynomial** — The nth-degree polynomial that matches f and its first n derivatives at a: T_n(x) = Σ f^(k)(a)/k! · (x−a)^k. The best polynomial approximation to f near a.

**Taylor's Theorem** — Expresses f as a Taylor polynomial plus a remainder term. The Lagrange form of the remainder: R_n(x) = f^(n+1)(c)/(n+1)! · (x−a)^(n+1) for some c between a and x. Quantifies how well the Taylor polynomial approximates f.

**Maclaurin Series** — A Taylor series centered at a = 0. Key examples: eˣ = Σ xⁿ/n!, sin x = Σ (−1)ⁿ x^{2n+1}/(2n+1)!, cos x = Σ (−1)ⁿ x^{2n}/(2n)!.

**Smoothness Class Cⁿ** — A function is Cⁿ if its nth derivative exists and is continuous. C⁰ = continuous; C¹ = continuously differentiable; C∞ = smooth (all derivatives exist). The hierarchy C⁰ ⊋ C¹ ⊋ C² ⊋ ⋯ ⊋ C∞ is strict at each level.

**Analytic Function** — A function that equals its Taylor series in a neighborhood of every point. All analytic functions are C∞, but not conversely. Example: e^{−1/x²} (defined to be 0 at x = 0) is C∞ but not analytic at 0 (its Taylor series is identically 0, but the function is not 0 for x ≠ 0).

**Implicit Differentiation** — Differentiating both sides of an equation F(x, y) = 0 with respect to x, treating y as a function of x. Yields dy/dx = −(∂F/∂x)/(∂F/∂y). The rigorous basis is the Implicit Function Theorem.

**Implicit Function Theorem (IFT)** — Under the condition ∂F/∂y ≠ 0, the equation F(x, y) = 0 defines y as a smooth function of x near any solution point. The IFT generalizes to functions of several variables and is a cornerstone of differential topology.

**Related Rates** — Using the chain rule to relate the rates of change of two or more quantities that are connected by an equation. A technique, but the underlying tool is always implicit differentiation with respect to time.

**Indeterminate Form** — An expression like 0/0, ∞/∞, 0·∞, ∞ − ∞, 0⁰, 1∞, ∞⁰ whose limit cannot be determined without further analysis. L'Hôpital's rule and algebraic manipulation are the standard tools.

**Principle of Stationary Action** — The physical law that the actual trajectory of a system is the one that makes the action functional S = ∫ L dt stationary. Generates the equations of motion (Euler-Lagrange equations). The fundamental organizing principle of classical and relativistic mechanics.
