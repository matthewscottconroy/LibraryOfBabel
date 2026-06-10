# Section 4.3: Applications of the Derivative

---

## Section Introduction

The derivative answers specific questions. Given a smooth curve, where does it rise and fall? Where does a function achieve its largest or smallest value? How well does a polynomial approximate an arbitrary smooth function? These are not merely mathematical curiosities — they govern the physics of extremal principles, from Fermat's principle of least time to the action principle underlying all of classical mechanics and general relativity.

This section develops the main tools: the Mean Value Theorem (the central result of differential calculus), L'Hôpital's rule for indeterminate forms, Taylor's theorem for polynomial approximation, and the application of derivatives to optimization. Each of these has a direct counterpart in the more advanced mathematics this book builds toward.

---

## 4.3.1 Local Extrema and Critical Points

**Definition**: f has a **local maximum** at c if f(c) ≥ f(x) for all x in some open interval containing c. Similarly for **local minimum**. A **local extremum** is either kind.

**Theorem** (Fermat, ca. 1637): If f has a local extremum at c and f is differentiable at c, then f'(c) = 0.

**Proof**: Suppose f has a local maximum at c. For small h > 0: f(c+h) ≤ f(c), so [f(c+h) - f(c)]/h ≤ 0, and taking h → 0⁺ gives f'(c) ≤ 0. For small h < 0: f(c+h) ≤ f(c), so [f(c+h) - f(c)]/h ≥ 0, and taking h → 0⁻ gives f'(c) ≥ 0. Both conditions together force f'(c) = 0. □

**Warning**: The converse is false. f'(c) = 0 does not guarantee an extremum. The function f(x) = x³ has f'(0) = 0 but x = 0 is neither a local maximum nor minimum (it is an **inflection point**). A point where f'(c) = 0 is called a **critical point** (or **stationary point**); it is a *candidate* for an extremum, not a guarantee.

**The second derivative test**: If f'(c) = 0 and f''(c) > 0, then c is a local minimum. If f''(c) < 0, then c is a local maximum. If f''(c) = 0, the test is inconclusive.

*Proof*: If f''(c) > 0, then f' is increasing at c. Since f'(c) = 0, f' < 0 just left of c and f' > 0 just right of c, so f decreases then increases through c — a local minimum. □

---

## 4.3.2 The Mean Value Theorem

**Theorem** (Rolle's Theorem): Let f be continuous on [a, b] and differentiable on (a, b). If f(a) = f(b), then there exists c ∈ (a, b) such that f'(c) = 0.

**Proof**: By the Extreme Value Theorem (Section 4.1.7), f achieves its maximum and minimum on [a, b]. If both are achieved at endpoints, then since f(a) = f(b), f must be constant on [a, b], and f' = 0 everywhere — take any c. If either extremum is achieved at an interior point c, then by Fermat's theorem f'(c) = 0. □

**Theorem** (Mean Value Theorem, MVT): Let f be continuous on [a, b] and differentiable on (a, b). Then there exists c ∈ (a, b) such that:

$$f'(c) = \frac{f(b) - f(a)}{b - a}$$

In words: at some point in the interior, the instantaneous rate of change equals the average rate of change.

**Proof**: Define h(x) = f(x) - [f(a) + ((f(b)-f(a))/(b-a)) · (x-a)]. This is f minus the secant line, so h(a) = h(b) = 0. By Rolle's Theorem, ∃c with h'(c) = 0. But h'(x) = f'(x) - (f(b)-f(a))/(b-a), so f'(c) = (f(b)-f(a))/(b-a). □

**Consequences**:

1. If f'(x) = 0 on an interval, then f is constant on that interval.

*Proof*: For any a < b in the interval, MVT gives f'(c) = (f(b)-f(a))/(b-a) for some c. If f' = 0 everywhere, then f(b) = f(a). □

2. If f'(x) > 0 on (a,b), then f is strictly increasing on [a,b]. (Proof: same argument shows f(b) - f(a) = f'(c)(b-a) > 0.)

3. If f'(x) = g'(x) on an interval, then f and g differ by a constant.

These consequences are used constantly in calculus and analysis. In the context of GR, consequence 1 is the backbone of uniqueness proofs: if a tensor quantity has zero covariant derivative (is "covariantly constant"), it is constant along geodesics.

---

## 4.3.3 L'Hôpital's Rule

When evaluating limits of the form lim f(x)/g(x) where both f(x) → 0 and g(x) → 0 (or both → ±∞), the ratio is "0/0" or "∞/∞" — indeterminate forms. L'Hôpital's rule handles these.

**Theorem** (L'Hôpital's Rule): Suppose f and g are differentiable near a (but possibly not at a), and g'(x) ≠ 0 near a. If lim_{x→a} f(x) = lim_{x→a} g(x) = 0 (or both ±∞), and if lim_{x→a} f'(x)/g'(x) = L (including L = ±∞), then:

$$\lim_{x \to a} \frac{f(x)}{g(x)} = \lim_{x \to a} \frac{f'(x)}{g'(x)} = L$$

*Note*: We differentiate the numerator and denominator separately — this is not the quotient rule.

**Proof** (0/0 case): By the Cauchy Mean Value Theorem (a generalization of MVT): for x ≠ a,

$$\frac{f(x) - f(a)}{g(x) - g(a)} = \frac{f'(c)}{g'(c)}$$

for some c between x and a. If f(a) = g(a) = 0, this gives f(x)/g(x) = f'(c)/g'(c). As x → a, c → a, so f'(c)/g'(c) → L. □

**Examples**:

1. $\lim_{x \to 0} \frac{\sin x}{x}$: Both → 0. Apply L'Hôpital: $\lim \frac{\cos x}{1} = 1$.

2. $\lim_{x \to 0} \frac{e^x - 1 - x}{x^2}$: Both → 0. Apply L'Hôpital: $\lim \frac{e^x - 1}{2x}$. Still 0/0. Apply again: $\lim \frac{e^x}{2} = 1/2$.

3. $\lim_{x \to \infty} x e^{-x} = \lim \frac{x}{e^x}$: ∞/∞ form. L'Hôpital: $\lim \frac{1}{e^x} = 0$.

---

## 4.3.4 Taylor's Theorem

The derivative tells us the slope at a point. The second derivative tells us the concavity. Can we use all higher derivatives at a single point to reconstruct the function? Taylor's theorem says: up to an error that vanishes to high order, yes.

**Theorem** (Taylor's Theorem with Remainder): Let f be n+1 times continuously differentiable on [a, x]. Then:

$$f(x) = f(a) + f'(a)(x-a) + \frac{f''(a)}{2!}(x-a)^2 + \cdots + \frac{f^{(n)}(a)}{n!}(x-a)^n + R_n(x)$$

where the **Lagrange remainder** is:

$$R_n(x) = \frac{f^{(n+1)}(c)}{(n+1)!}(x-a)^{n+1}$$

for some c between a and x.

**Proof**: Define F(t) = f(x) - [f(t) + f'(t)(x-t) + f''(t)(x-t)²/2! + ⋯ + f⁽ⁿ⁾(t)(x-t)ⁿ/n!]. Note F(x) = 0 and F(a) = f(x) - Tₙ(x), where Tₙ is the Taylor polynomial. Define G(t) = (x-t)^{n+1}. Apply the MVT (generalized, i.e. Cauchy) to F and G on [a, x]. The key computation: F'(t) = -f^{(n+1)}(t)(x-t)ⁿ/n! (all other terms cancel in the telescoping). So F'(c)/G'(c) = f^{(n+1)}(c)/(n+1)!, and F(a)/G(a) = R_n(x)/(x-a)^{n+1} = f^{(n+1)}(c)/(n+1)!. □

**Significance**: The Taylor polynomial $T_n(x) = \sum_{k=0}^{n} \frac{f^{(k)}(a)}{k!}(x-a)^k$ is the unique polynomial of degree ≤ n that matches f and its first n derivatives at a. The remainder $R_n(x)$ tells us how good the approximation is: if |f^{(n+1)}| ≤ M on the interval, then $|R_n(x)| \leq M|x-a|^{n+1}/(n+1)!$.

**Key expansions about a = 0** (Maclaurin series):

$$e^x = 1 + x + \frac{x^2}{2!} + \frac{x^3}{3!} + \cdots$$

$$\sin x = x - \frac{x^3}{3!} + \frac{x^5}{5!} - \cdots$$

$$\cos x = 1 - \frac{x^2}{2!} + \frac{x^4}{4!} - \cdots$$

$$(1+x)^\alpha = 1 + \alpha x + \frac{\alpha(\alpha-1)}{2!}x^2 + \cdots \quad \text{(binomial series)}$$

**Connection to physics**: The weak-field expansion of GR is a Taylor expansion. The Schwarzschild metric, in the limit of small curvature, gives Newtonian gravity as the leading term, with relativistic corrections as higher-order terms. Post-Newtonian approximations are essentially Taylor expansions in (v/c)² and GM/rc² simultaneously. [Will, C.M. (1993). *Theory and Experiment in Gravitational Physics*, revised ed. Cambridge University Press. Chapter 4 on post-Newtonian approximations.]

---

## 4.3.5 Optimization

**Global extrema on closed intervals**: By the EVT, a continuous function on [a, b] achieves its maximum and minimum. To find them: evaluate f at all critical points in (a, b) and at the endpoints a, b; the largest value is the maximum, the smallest the minimum.

**Unconstrained optimization in physics — extremal principles**: Many fundamental laws of physics can be stated as: the actual trajectory is the one that makes some functional stationary (critical). 

- *Fermat's principle* (1662): light travels along the path that minimizes travel time. This single principle implies reflection, refraction (Snell's law), and the bending of light in curved spacetime.

- *Hamilton's principle* (1834): the actual path of a mechanical system from configuration q(t₁) to q(t₂) is the one that makes the **action** S = ∫ L dt stationary, where L = T - V (kinetic minus potential energy). This is the **principle of stationary action**, and it generates all of classical mechanics through the **Euler-Lagrange equations**: d/dt(∂L/∂q̇) - ∂L/∂q = 0.

- *The Einstein-Hilbert action* (1915): the field equations of GR follow from making stationary the action S = ∫ R √(-g) d⁴x + S_matter, where R is the Ricci scalar curvature. Varying this action with respect to the metric yields Einstein's equations Gᵤᵥ = 8πTᵤᵥ. [Einstein, A. (1915). "Die Feldgleichungen der Gravitation." *Sitzungsberichte der Preussischen Akademie der Wissenschaften*, 844–847.]

The calculus of variations (Chapter 18) makes the notion of "stationary functional" precise. But its spirit is already visible here: we look for the critical point, not necessarily the minimum.

---

## References

- Apostol, T.M. (1967). *Calculus*, Vol. 1, 2nd ed. Wiley. [Chapters 4–7 on the MVT, Taylor's theorem, and applications.]
- Einstein, A. (1915). "Die Feldgleichungen der Gravitation." *Sitzungsberichte der Preussischen Akademie der Wissenschaften zu Berlin*. 844–847. [The original field equations of GR.]
- Fermat, P. de (ca. 1662). Letter to Cureau de la Chambre. [Fermat's principle of least time in optics.]
- Hamilton, W.R. (1834). "On a General Method in Dynamics." *Philosophical Transactions of the Royal Society*, Part II, 247–308. [Hamilton's principle of stationary action.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [Chapter 5 on the MVT and Taylor's theorem.]
- Will, C.M. (1993). *Theory and Experiment in Gravitational Physics*, revised ed. Cambridge University Press. [Post-Newtonian expansions as Taylor series in the weak-field/slow-motion limit.]
