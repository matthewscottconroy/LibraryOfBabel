# Section 5.3: Techniques of Integration

---

## Section Introduction

Finding antiderivatives is an art. Differentiation is algorithmic — given any expression built from elementary functions, the chain rule, product rule, and table of derivatives will produce the derivative mechanically. Integration has no such algorithm. Some combinations of elementary functions have antiderivatives expressible in closed form; many do not. (Liouville's theorem, 1835, makes this precise: the integral ∫ e^{-x²} dx, for instance, cannot be expressed in terms of elementary functions.)

The techniques in this section — substitution, integration by parts, trigonometric substitution, partial fractions — cover most cases that arise in physics. Knowing these techniques fluently is a prerequisite for computing anything in quantum field theory, electromagnetism, or GR.

---

## 5.3.1 Substitution (Change of Variables)

**Theorem** (Substitution): If g: [a, b] → ℝ is differentiable and f is continuous on the range of g, then:

$$\int_a^b f(g(x)) g'(x) \, dx = \int_{g(a)}^{g(b)} f(u) \, du$$

**Proof**: Let F be an antiderivative of f. Then d/dx[F(g(x))] = F'(g(x))g'(x) = f(g(x))g'(x). By the FTC:

$$\int_a^b f(g(x))g'(x) \, dx = F(g(b)) - F(g(a)) = \int_{g(a)}^{g(b)} f(u) \, du \quad \square$$

The substitution rule is the chain rule in reverse. In differential notation: if u = g(x), then du = g'(x) dx, and the substitution replaces f(g(x)) g'(x) dx with f(u) du.

**Examples**:

1. $\int x \cos(x^2) \, dx$: Let u = x², du = 2x dx. Then $\int \cos u \cdot \frac{du}{2} = \frac{\sin u}{2} + C = \frac{\sin x^2}{2} + C$.

2. $\int_0^1 x e^{x^2} \, dx$: Let u = x², du = 2x dx. When x = 0, u = 0; when x = 1, u = 1. $\int_0^1 e^u \frac{du}{2} = \frac{1}{2}[e^u]_0^1 = \frac{e-1}{2}$.

3. $\int \frac{2x}{x^2 + 1} dx$: Let u = x² + 1. $\int \frac{du}{u} = \ln|u| + C = \ln(x^2+1) + C$.

---

## 5.3.2 Integration by Parts

**Theorem** (Integration by Parts): If u and v are differentiable, then:

$$\int u \, dv = uv - \int v \, du$$

In definite form: $\int_a^b u(x) v'(x) \, dx = [u(x)v(x)]_a^b - \int_a^b v(x) u'(x) \, dx$.

**Proof**: The product rule says (uv)' = u'v + uv', so uv' = (uv)' − u'v. Integrating both sides gives the formula. □

Integration by parts is the product rule in reverse. It trades ∫ u dv for ∫ v du, which is useful when the latter is simpler.

**The LIATE mnemonic** (for choosing which factor to call u): Logarithmic > Inverse trig > Algebraic > Trigonometric > Exponential. Choose u to be the factor earlier in this list.

**Examples**:

1. $\int x e^x \, dx$: Let u = x, dv = eˣ dx. Then du = dx, v = eˣ. Result: xeˣ − ∫ eˣ dx = xeˣ − eˣ + C = (x−1)eˣ + C.

2. $\int \ln x \, dx$: Let u = ln x, dv = dx. Then du = (1/x) dx, v = x. Result: x ln x − ∫ x · (1/x) dx = x ln x − x + C.

3. *Reduction formula*: $\int x^n e^x \, dx$. Let u = xⁿ, dv = eˣ dx. Result: xⁿeˣ − n ∫ x^{n-1} eˣ dx. This is a **reduction formula** — it reduces the problem to the same integral with n replaced by n−1. Applying it n times yields a complete formula.

4. *Cyclic case*: $\int e^x \sin x \, dx$. Let u = sin x, dv = eˣ dx. Result: eˣ sin x − ∫ eˣ cos x dx. Apply by parts again to ∫ eˣ cos x dx: eˣ cos x + ∫ eˣ sin x dx. We get I = eˣ sin x − eˣ cos x − I, so 2I = eˣ(sin x − cos x), giving I = eˣ(sin x − cos x)/2 + C.

---

## 5.3.3 Trigonometric Substitution

For integrals involving √(a² − x²), √(a² + x²), or √(x² − a²), the substitution x = a sin θ, x = a tan θ, or x = a sec θ, respectively, converts the square root into a simpler trigonometric expression.

**Example**: $\int \sqrt{1-x^2} \, dx$. Let x = sin θ, dx = cos θ dθ, √(1−x²) = cos θ (for θ ∈ [−π/2, π/2]). The integral becomes $\int \cos^2 \theta \, d\theta = \int \frac{1 + \cos 2\theta}{2} d\theta = \frac{\theta}{2} + \frac{\sin 2\theta}{4} + C$. Converting back: θ = arcsin x, sin 2θ = 2x√(1−x²). Result: (arcsin x)/2 + x√(1−x²)/2 + C. (This is the area of a circular sector — as expected.)

These substitutions are useful in computing path lengths, areas in curved coordinates, and integrals appearing in the Schwarzschild and Kerr geometries.

---

## 5.3.4 Partial Fractions

Any rational function P(x)/Q(x) (ratio of polynomials, with deg P < deg Q) can be decomposed into simpler fractions, each of which is easily integrated.

**The method**: Factor Q(x) completely over ℝ into linear and irreducible quadratic factors. Write P(x)/Q(x) as a sum of terms:
- For each factor (x − r)ᵏ: terms A₁/(x−r) + A₂/(x−r)² + ⋯ + Aₖ/(x−r)ᵏ
- For each irreducible quadratic factor (x²+px+q)ᵏ: terms (B₁x+C₁)/(x²+px+q) + ⋯

The constants A, B, C are determined by clearing denominators and matching coefficients.

**Example**: $\int \frac{x}{(x-1)(x+2)} dx$.

Partial fractions: x/[(x−1)(x+2)] = A/(x−1) + B/(x+2). Multiply through: x = A(x+2) + B(x−1). Setting x = 1: 1 = 3A, A = 1/3. Setting x = −2: −2 = −3B, B = 2/3.

$$\int \frac{x}{(x-1)(x+2)} dx = \frac{1}{3} \ln|x-1| + \frac{2}{3} \ln|x+2| + C$$

**Physical application**: Partial fractions appear in Laplace transform calculations, transfer functions in control theory, and in computing the deflection of light in Schwarzschild geometry. The integral for the light-deflection angle in GR involves a rational function of the Schwarzschild radial coordinate that must be decomposed by partial fractions. [Misner, Thorne, Wheeler (1973), Gravitation, Box 25.7.]

---

## 5.3.5 Numerical Integration

When no antiderivative in closed form is available, we evaluate the integral numerically. The three main methods:

**Trapezoid rule**: Approximate f on each subinterval [xᵢ₋₁, xᵢ] by a line segment. The error for n equal subintervals with h = (b−a)/n is O(h²) — exact for linear f.

$$\int_a^b f \, dx \approx h\left[\frac{f(x_0)}{2} + f(x_1) + f(x_2) + \cdots + f(x_{n-1}) + \frac{f(x_n)}{2}\right]$$

**Simpson's rule**: Approximate f on each pair of subintervals by a parabola. Error is O(h⁴) — exact for polynomials of degree ≤ 3. For n equal subintervals (n even):

$$\int_a^b f \, dx \approx \frac{h}{3}\left[f(x_0) + 4f(x_1) + 2f(x_2) + 4f(x_3) + \cdots + 4f(x_{n-1}) + f(x_n)\right]$$

**Gaussian quadrature**: Chooses both the nodes xᵢ and weights wᵢ to maximize accuracy for a given number of function evaluations. n-point Gaussian quadrature is exact for polynomials of degree up to 2n−1. The most powerful method when f is smooth.

**In GR**: Numerical relativity requires solving partial differential equations on curved spacetimes. The integrals that appear must be evaluated numerically, using extensions of these ideas to higher dimensions. [Baumgarte, T.W. and Shapiro, S.L. (2010). *Numerical Relativity: Solving Einstein's Equations on the Computer*. Cambridge University Press.]

---

## References

- Abramowitz, M. and Stegun, I.A. (1964). *Handbook of Mathematical Functions*. National Bureau of Standards. [Comprehensive reference for integrals and special functions. Many integrals not computable by elementary techniques but expressible in terms of special functions.]
- Baumgarte, T.W. and Shapiro, S.L. (2010). *Numerical Relativity: Solving Einstein's Equations on the Computer*. Cambridge University Press. [Numerical integration in the context of GR simulations.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation*. W.H. Freeman. [Box 25.7 and related discussions for integrals in Schwarzschild geometry.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [Chapters 6–7 on the integral and series.]
- Spivak, M. (1994). *Calculus*, 3rd ed. Publish or Perish. [Chapters 15–19 on integration techniques.]
