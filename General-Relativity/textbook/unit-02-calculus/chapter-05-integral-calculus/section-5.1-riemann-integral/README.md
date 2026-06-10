# Section 5.1: The Riemann Integral

---

## Section Introduction

We want to define the area under the graph of a function f on an interval [a, b]. The strategy is ancient — Archimedes used it for parabolas around 250 BCE — but the rigorous execution is due to Bernhard Riemann in the 1850s. Divide the interval into subintervals, approximate the area of each strip by a rectangle, add them up, and take the limit as the strips become infinitely thin. The question is: does this limit exist, and is it independent of the choices made in the approximation?

The answer is yes — under a precise condition on f (Riemann integrability) — and the proof requires us to be careful about exactly what "limit" means here.

---

## 5.1.1 Riemann Sums

**Partition**: A **partition** of [a, b] is a finite collection of points a = x₀ < x₁ < x₂ < ⋯ < xₙ = b. The **norm** (or mesh) of the partition is ||P|| = max_{1≤i≤n} (xᵢ − xᵢ₋₁), the length of the largest subinterval.

**Riemann sum**: Given a partition P and a choice of **sample points** cᵢ ∈ [xᵢ₋₁, xᵢ] for each i, the **Riemann sum** is:

$$S(f, P, \{c_i\}) = \sum_{i=1}^{n} f(c_i) \Delta x_i \quad \text{where } \Delta x_i = x_i - x_{i-1}$$

This is the sum of rectangle areas: each rectangle has width Δxᵢ and height f(cᵢ).

**Special choices of cᵢ**:
- **Left Riemann sum**: cᵢ = xᵢ₋₁ (left endpoint of each subinterval)
- **Right Riemann sum**: cᵢ = xᵢ (right endpoint)
- **Midpoint rule**: cᵢ = (xᵢ₋₁ + xᵢ)/2
- **Upper sum** U(f, P): cᵢ chosen so that f(cᵢ) = sup_{[xᵢ₋₁,xᵢ]} f (or the supremum, if the sup is not achieved)
- **Lower sum** L(f, P): cᵢ chosen so that f(cᵢ) = inf_{[xᵢ₋₁,xᵢ]} f

For a non-negative function, the upper sum overestimates and the lower sum underestimates the true area (whatever that is). The true area, if it exists, should be squeezed between them.

---

## 5.1.2 The Riemann Integral: Definition

**Definition** (Riemann integrability): A bounded function f: [a, b] → ℝ is **Riemann integrable** if there exists a number I ∈ ℝ such that: for every ε > 0, there exists δ > 0 such that for every partition P with ||P|| < δ and every choice of sample points,

$$\left| S(f, P, \{c_i\}) - I \right| < \varepsilon$$

When this holds, we write:

$$\int_a^b f(x) \, dx = I$$

This is the **definite integral** of f from a to b.

**Interpretation**: The integral is the limit of Riemann sums as the partition is refined (its norm goes to 0). The requirement that the limit is the same for *all* choices of sample points is strong — it ensures the result is unambiguous.

**Alternative (Darboux) characterization**: f is Riemann integrable on [a, b] iff:

$$\sup_P L(f, P) = \inf_P U(f, P)$$

The common value is ∫ f dx. This is often easier to work with, since upper and lower sums have simpler expressions than general Riemann sums.

---

## 5.1.3 Which Functions Are Integrable?

**Theorem**: Every continuous function on [a, b] is Riemann integrable.

**Proof** (sketch): Since f is continuous on a closed bounded interval, it is **uniformly continuous**: ∀ε > 0 ∃δ > 0 such that |x − y| < δ ⟹ |f(x) − f(y)| < ε/(b−a). For any partition P with ||P|| < δ:

$$U(f, P) - L(f, P) = \sum_{i=1}^n (\sup_i f - \inf_i f) \Delta x_i \leq \sum_i \frac{\varepsilon}{b-a} \Delta x_i = \varepsilon$$

So the Darboux condition is satisfied. □

**Theorem**: Every monotone bounded function on [a, b] is Riemann integrable. (The proof uses a direct estimate on U − L for monotone functions.)

**Theorem**: If f is bounded and has only finitely many discontinuities, it is Riemann integrable. (Lebesgue's criterion generalizes this: f is Riemann integrable iff its set of discontinuities has *measure zero*.)

**Non-integrable example**: The Dirichlet function f(x) = 1 if x ∈ ℚ, f(x) = 0 if x ∉ ℚ is not Riemann integrable on [0, 1]. In every subinterval, inf f = 0 and sup f = 1, so L(f, P) = 0 and U(f, P) = 1 for every partition P. The upper and lower sums never agree. This function *is* Lebesgue integrable (with ∫₀¹ f dx = 0, since ℚ has measure zero), illustrating the greater generality of the Lebesgue integral.

---

## 5.1.4 Properties of the Integral

**Theorem** (Basic Properties): Let f, g be integrable on [a, b]. Then:

1. **Linearity**: ∫ₐᵇ [αf(x) + βg(x)] dx = α ∫ₐᵇ f dx + β ∫ₐᵇ g dx

2. **Additivity**: ∫ₐᵇ f dx = ∫ₐᶜ f dx + ∫ᶜᵇ f dx for any c ∈ [a, b]

3. **Monotonicity**: If f(x) ≤ g(x) on [a, b], then ∫ₐᵇ f dx ≤ ∫ₐᵇ g dx

4. **Boundedness**: If m ≤ f(x) ≤ M, then m(b−a) ≤ ∫ₐᵇ f dx ≤ M(b−a)

5. **Absolute value**: |∫ₐᵇ f dx| ≤ ∫ₐᵇ |f| dx

Property 5 is the integral analogue of the triangle inequality.

**Convention**: We define ∫ₐᵃ f dx = 0 and ∫ᵇᵃ f dx = −∫ₐᵇ f dx. With these conventions, the additivity property ∫ₐᵇ + ∫ᵇᶜ = ∫ₐᶜ holds for any ordering of a, b, c.

---

## 5.1.5 Mean Value Theorem for Integrals

**Theorem** (MVT for Integrals): If f is continuous on [a, b], then there exists c ∈ (a, b) such that:

$$\int_a^b f(x) \, dx = f(c)(b - a)$$

That is, f attains its "average value" at some interior point.

**Proof**: The average value (1/(b−a))∫ₐᵇ f dx lies between min f and max f on [a, b] (by the boundedness property). By the IVT, f achieves this value at some c. □

**The average value of f on [a, b]** is defined as:

$$\bar{f} = \frac{1}{b-a} \int_a^b f(x) \, dx$$

This is the continuous analogue of the arithmetic mean of a finite list of numbers. It arises constantly in physics — the average temperature over a region, the average velocity over a time interval, the average field in a cavity.

---

## References

- Apostol, T.M. (1967). *Calculus*, Vol. 1, 2nd ed. Wiley. [Chapter 1 gives an axiomatic development of the integral (area axioms), then relates it to Riemann sums. An unusual but illuminating approach.]
- Riemann, B. (1854/1868). "Über die Darstellbarkeit einer Function durch eine trigonometrische Reihe." *Abhandlungen der Königlichen Gesellschaft der Wissenschaften zu Göttingen*, 13, 87–132. [Riemann's Habilitation lecture (1854), published posthumously. Contains the first rigorous definition of the Riemann integral, introduced to resolve questions about which functions have convergent Fourier series.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [Chapter 6 develops the Riemann-Stieltjes integral (a generalization) with full proofs. The special case of the Riemann integral is included.]
- Spivak, M. (1994). *Calculus*, 3rd ed. Publish or Perish. [Chapters 13–14 on the integral; particularly clear on the definition and the role of uniform continuity in proving integrability of continuous functions.]
