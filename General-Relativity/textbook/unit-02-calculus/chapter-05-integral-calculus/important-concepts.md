# Chapter 5 Important Concepts

---

**Riemann Sum** — An approximation to the area under a curve f on [a, b], formed by dividing the interval into subintervals, choosing sample points, and summing rectangle areas: Σ f(cᵢ) Δxᵢ. The Riemann integral is the limit of Riemann sums as the partition mesh → 0.

**Partition** — A finite set of points a = x₀ < x₁ < ⋯ < xₙ = b that divides [a, b] into subintervals. The norm (mesh) of a partition is max Δxᵢ.

**Upper Sum / Lower Sum** — The Riemann sum with sample points chosen to maximize (upper) or minimize (lower) the function value on each subinterval. Upper sums overestimate; lower sums underestimate. A function is Riemann integrable iff the infimum of upper sums equals the supremum of lower sums.

**Riemann Integrable** — A bounded function f: [a, b] → ℝ is Riemann integrable if the limit of Riemann sums exists and is the same for all choices of partitions and sample points as the mesh → 0. Equivalently: inf U(f, P) = sup L(f, P).

**Definite Integral** — The common value ∫ₐᵇ f(x) dx to which Riemann sums converge for an integrable function. Represents (when f ≥ 0) the area under the graph of f on [a, b].

**Indefinite Integral** — The family of antiderivatives of f, written ∫ f(x) dx = F(x) + C. Not a number but a family of functions, all differing by constants.

**Antiderivative (Primitive)** — A function F such that F'(x) = f(x). Antiderivatives are unique up to an additive constant.

**Fundamental Theorem of Calculus, Part I** — If f is integrable and continuous at c, then the area accumulation function F(x) = ∫ₐˣ f(t) dt is differentiable at c with F'(c) = f(c). Differentiation undoes integration.

**Fundamental Theorem of Calculus, Part II** — If F is an antiderivative of f on [a, b], then ∫ₐᵇ f(x) dx = F(b) − F(a). Integration of the derivative gives the net change.

**Substitution Rule** — The chain rule in reverse: ∫ f(g(x))g'(x) dx = ∫ f(u) du with u = g(x). The most important integration technique; generalizes to change of variables in multiple dimensions (with the Jacobian).

**Integration by Parts** — The product rule in reverse: ∫ u dv = uv − ∫ v du. Trades one integral for another that is (hopefully) simpler.

**Partial Fractions** — A technique for integrating rational functions P(x)/Q(x) by decomposing them into simpler fractions corresponding to the factors of Q(x).

**Improper Integral** — An integral with infinite limits or an unbounded integrand, defined as a limit of proper integrals. Converges if the limit is finite; diverges otherwise.

**Absolute Convergence** — An improper integral ∫f converges absolutely if ∫|f| converges. Absolute convergence implies convergence. An integral that converges but not absolutely converges conditionally.

**p-test for Convergence** — ∫₁^∞ x^{-p} dx converges iff p > 1; ∫₀¹ x^{-p} dx converges iff p < 1. A benchmark for comparison tests.

**Comparison Test** — If 0 ≤ f(x) ≤ g(x) and ∫g converges, then ∫f converges. If 0 ≤ g(x) ≤ f(x) and ∫g diverges, then ∫f diverges.

**Gamma Function** — Γ(s) = ∫₀^∞ t^{s-1} e^{-t} dt for s > 0. Satisfies Γ(s+1) = sΓ(s) and Γ(n) = (n-1)! for positive integers. Extends the factorial to all positive reals (and, by analytic continuation, to all complex numbers except non-positive integers).

**Gaussian Integral** — ∫_{-∞}^∞ e^{-x²} dx = √π. One of the most important integrals in mathematics and physics; appears in probability distributions, quantum field theory path integrals, and heat kernels.

**Average Value** — The average value of f on [a, b] is (1/(b-a)) ∫ₐᵇ f(x) dx. By the MVT for integrals, f attains this value at some interior point.

**Mean Value Theorem for Integrals** — If f is continuous on [a, b], then ∫ₐᵇ f dx = f(c)(b-a) for some c ∈ (a, b). The integral equals the area of the rectangle with width (b-a) and height equal to the function's average value.

**Arc Length** — The length of the curve y = f(x) from x = a to x = b: L = ∫ₐᵇ √(1 + [f'(x)]²) dx. Generalizes to curves in higher dimensions and, in GR, to the proper time along a worldline.

**Simpson's Rule** — Numerical integration approximating f on each pair of subintervals by a quadratic. Error O(h⁴) for n subintervals. More accurate than the trapezoid rule for smooth functions.

**Cauchy Principal Value** — A regularization of an improper integral with symmetric limits: P.V. ∫f = lim_{R→∞} ∫_{-R}^R f. Used for integrals that do not converge absolutely but have a well-defined symmetric limit.

**Wallis Product** — The identity π/2 = ∏_{n=1}^∞ (2n)(2n)/[(2n-1)(2n+1)], derived from the reduction formula for ∫₀^{π/2} sinⁿx dx. One of the first infinite product representations of π.

**Action Functional** — In physics, S[q] = ∫ L(q, q̇, t) dt. A functional assigns a number to each path q(t). The physical trajectory is the stationary point of the action (Hamilton's principle). The generalization to GR gives the Einstein-Hilbert action S = ∫ R√(-g) d⁴x.
