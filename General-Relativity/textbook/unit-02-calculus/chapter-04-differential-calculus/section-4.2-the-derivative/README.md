# Section 4.2: The Derivative

---

## Section Introduction

The derivative is one of the great ideas in the history of thought. It captures, precisely and computationally, what it means for something to be changing *right now* — not on average over an interval, but instantaneously. Velocity is the derivative of position; acceleration is the derivative of velocity; electric field is the derivative (in an appropriate sense) of potential; the Einstein field equations express how spacetime curvature changes in terms of derivatives of the metric. The derivative is, in every sense, the central operation of physics.

The definition is a limit. We approximate the instantaneous rate of change by an average rate of change over a shrinking interval, then take the limit as the interval's length goes to zero. This is the same limit machinery we just developed in Section 4.1 — now applied to compute something physically meaningful.

---

## 4.2.1 Definition of the Derivative

**Definition**: Let f be a function defined on an open interval containing a. The **derivative of f at a**, written f'(a) or df/dx|ₐ or (d/dx)f(x)|_{x=a}, is defined by:

$$f'(a) = \lim_{h \to 0} \frac{f(a+h) - f(a)}{h}$$

if this limit exists. If f'(a) exists, we say f is **differentiable at a**. If f is differentiable at every point of an interval, f is **differentiable on that interval**.

Equivalently, using x in place of a+h:

$$f'(a) = \lim_{x \to a} \frac{f(x) - f(a)}{x - a}$$

The quantity $\frac{f(a+h) - f(a)}{h}$ is the **difference quotient** — it is the slope of the secant line through (a, f(a)) and (a+h, f(a+h)). The derivative is the limit of this slope as h → 0, which is the slope of the **tangent line** to the graph of f at the point (a, f(a)).

**Notation**: Multiple notations are in common use:
- Lagrange: f'(x) (most compact for single-variable functions)
- Leibniz: dy/dx or df/dx (most suggestive for chain rule and related computations)
- Newton: ẋ (used for time derivatives in physics; we write ẋ for dx/dt)
- Euler: Df(x) or D_xf (useful in operator notation)

Each notation captures something. The Leibniz notation dy/dx, while not literally a fraction, behaves like one in many formal manipulations (chain rule, separation of variables, substitution). This is not accidental — it reflects a deep structure that becomes transparent when we define differentials properly in Section 4.4 and later in the theory of differential forms (Chapter 28).

---

## 4.2.2 Computing Basic Derivatives from the Definition

**Example 1**: Compute the derivative of f(x) = x² at an arbitrary point a.

$$f'(a) = \lim_{h \to 0} \frac{(a+h)^2 - a^2}{h} = \lim_{h \to 0} \frac{a^2 + 2ah + h^2 - a^2}{h} = \lim_{h \to 0} \frac{2ah + h^2}{h} = \lim_{h \to 0} (2a + h) = 2a$$

So (x²)' = 2x. This is the power rule with n = 2.

**Example 2**: Compute the derivative of f(x) = xⁿ for positive integer n.

$$\frac{(a+h)^n - a^n}{h}$$

Using the binomial theorem: $(a+h)^n = \sum_{k=0}^{n} \binom{n}{k} a^{n-k} h^k = a^n + na^{n-1}h + \binom{n}{2}a^{n-2}h^2 + \cdots + h^n$.

So $(a+h)^n - a^n = na^{n-1}h + \binom{n}{2}a^{n-2}h^2 + \cdots + h^n$, and dividing by h:

$$\frac{(a+h)^n - a^n}{h} = na^{n-1} + \binom{n}{2}a^{n-2}h + \cdots + h^{n-1}$$

As h → 0, all terms with h vanish. Thus **f'(a) = na^{n-1}**, giving us the **power rule**:

$$\frac{d}{dx} x^n = nx^{n-1}$$

(This holds for all real n, once we define xⁿ for non-integer n — we will verify this.)

**Example 3**: Compute the derivative of f(x) = sin x.

We use the limit definition and the addition formula sin(a+h) = sin a cos h + cos a sin h:

$$\frac{\sin(a+h) - \sin a}{h} = \frac{\sin a \cos h + \cos a \sin h - \sin a}{h} = \sin a \cdot \frac{\cos h - 1}{h} + \cos a \cdot \frac{\sin h}{h}$$

Two fundamental limits are needed:

$$\lim_{h \to 0} \frac{\sin h}{h} = 1 \qquad \lim_{h \to 0} \frac{\cos h - 1}{h} = 0$$

The first of these is the most important limit in trigonometry; it can be proved geometrically by comparing areas (a unit circle sector has area h/2, the inscribed triangle has area sin h/2, and the circumscribed triangle has area tan h/2, so sin h/2 ≤ h/2 ≤ tan h/2, giving 1 ≤ h/sin h ≤ 1/cos h; as h → 0, both bounds tend to 1 by the squeeze theorem). The second follows from the first via cos h - 1 = -2sin²(h/2).

Therefore: $\frac{d}{dx} \sin x = \cos x$.

Similarly (exercise): $\frac{d}{dx} \cos x = -\sin x$.

**Example 4**: Compute the derivative of f(x) = eˣ.

This requires knowing that eˣ is defined by lim_{h→0} (e^h - 1)/h = 1, which is itself equivalent to the characterization of e as the base for which the exponential function equals its own derivative at 0. (The full proof uses the series definition e^x = Σ xⁿ/n!, which we develop in Chapter 6.)

$$\frac{e^{a+h} - e^a}{h} = e^a \cdot \frac{e^h - 1}{h} \to e^a \cdot 1 = e^a$$

Thus $\frac{d}{dx} e^x = e^x$. This is the miraculous self-referential property of the exponential function: it is its own derivative.

---

## 4.2.3 Differentiability and Continuity

**Theorem**: If f is differentiable at a, then f is continuous at a.

**Proof**: We have:

$$f(x) - f(a) = \frac{f(x) - f(a)}{x - a} \cdot (x - a)$$

Taking limits as x → a:

$$\lim_{x \to a} [f(x) - f(a)] = \lim_{x \to a} \frac{f(x) - f(a)}{x - a} \cdot \lim_{x \to a} (x - a) = f'(a) \cdot 0 = 0$$

So lim_{x→a} f(x) = f(a), which is the definition of continuity at a. □

**The converse is false.** Continuity does not imply differentiability.

*Example (corner)*: f(x) = |x| is continuous everywhere. At x = 0: for h > 0, (|h| - 0)/h = 1; for h < 0, (|h| - 0)/h = -1. The left and right limits disagree, so f'(0) does not exist.

*Example (cusp)*: f(x) = x^{2/3} is continuous at 0 but f'(0) = lim_{h→0} h^{-1/3}, which does not exist (it diverges).

*More extreme example*: Weierstrass (1872) exhibited a function that is continuous everywhere but differentiable nowhere — a function whose graph is jagged at every scale. Such functions were considered pathological curiosities at the time; they now appear naturally in fractal geometry and the theory of Brownian motion. The Weierstrass function is:

$$W(x) = \sum_{n=0}^{\infty} a^n \cos(b^n \pi x)$$

for 0 < a < 1, b a positive odd integer, with ab > 1 + 3π/2. Weierstrass's proof that this is nowhere differentiable used term-by-term operations on the series that required careful justification. [Weierstrass, K. (1872). "Über continuirliche Functionen eines reellen Arguments, die für keinen Werth des letzeren einen bestimmten Differentialquotienten besitzen." Königliche Akademie der Wissenschaften.]

---

## 4.2.4 The Differentiation Rules

Computing every derivative from the definition would be laborious. Instead, we prove general rules. **These rules are theorems**, not axioms; they follow from the limit definition.

**Theorem** (Differentiation Rules): Let f, g be differentiable at x, and let c ∈ ℝ. Then:

**(a) Constant Rule**: If f(x) = c, then f'(x) = 0.

*Proof*: $(c - c)/h = 0 \to 0$. □

**(b) Constant Multiple Rule**: $(cf)'(x) = cf'(x)$.

*Proof*: $\lim_{h \to 0} \frac{c f(x+h) - c f(x)}{h} = c \lim_{h \to 0} \frac{f(x+h) - f(x)}{h} = c f'(x)$, by the scalar limit law. □

**(c) Sum Rule**: $(f + g)'(x) = f'(x) + g'(x)$.

*Proof*: Apply the sum limit law. □

**(d) Product Rule**: $(fg)'(x) = f'(x)g(x) + f(x)g'(x)$.

*Proof*: The key is an algebraic trick — add and subtract f(x+h)g(x):

$$\frac{f(x+h)g(x+h) - f(x)g(x)}{h}$$
$$= \frac{f(x+h)g(x+h) - f(x+h)g(x) + f(x+h)g(x) - f(x)g(x)}{h}$$
$$= f(x+h) \cdot \frac{g(x+h) - g(x)}{h} + g(x) \cdot \frac{f(x+h) - f(x)}{h}$$

As h → 0: f(x+h) → f(x) (by continuity of differentiable functions), the first fraction → g'(x), and the second → f'(x). So the limit is f(x)g'(x) + g(x)f'(x). □

**(e) Quotient Rule**: If g(x) ≠ 0, then $(f/g)'(x) = \frac{f'(x)g(x) - f(x)g'(x)}{g(x)^2}$.

*Proof*: Write f/g = f · (1/g). By the product rule, we need (1/g)'. Apply the limit definition:

$$\frac{1/g(x+h) - 1/g(x)}{h} = \frac{g(x) - g(x+h)}{h \cdot g(x+h) \cdot g(x)} \to \frac{-g'(x)}{g(x)^2}$$

Then the product rule gives the quotient rule. □

**(f) Chain Rule**: If g is differentiable at x and f is differentiable at g(x), then $(f \circ g)'(x) = f'(g(x)) \cdot g'(x)$.

*Proof* (sketch): The naive attempt $\frac{f(g(x+h)) - f(g(x))}{h} = \frac{f(g(x+h)) - f(g(x))}{g(x+h) - g(x)} \cdot \frac{g(x+h) - g(x)}{h}$ fails when g(x+h) = g(x) (we cannot divide by zero). The rigorous proof uses a reformulation of differentiability: f is differentiable at a with derivative f'(a) if and only if:

$$f(a+k) = f(a) + f'(a)k + \varepsilon(k)k$$

where ε(k) → 0 as k → 0 (a "little-o" error term). Applying this to both g at x and f at g(x) and composing gives the chain rule. [See Rudin (1976), Theorem 5.5, for the full proof.] □

---

## 4.2.5 Derivatives of Elementary Functions

The differentiation rules, combined with the base cases we computed from the definition, allow us to differentiate all elementary functions.

**Complete list**:

| Function | Derivative |
|----------|-----------|
| c (constant) | 0 |
| xⁿ | nxⁿ⁻¹ |
| eˣ | eˣ |
| aˣ (a > 0) | aˣ ln a |
| ln x (x > 0) | 1/x |
| sin x | cos x |
| cos x | −sin x |
| tan x | sec² x |
| arcsin x | 1/√(1−x²) |
| arctan x | 1/(1+x²) |

**Derivations**:

- *aˣ*: Write aˣ = e^{x ln a}. By the chain rule: d/dx e^{x ln a} = e^{x ln a} · ln a = aˣ ln a.

- *ln x*: Let y = ln x. Then eʸ = x. Differentiating implicitly (see Section 4.4): eʸ · y' = 1, so y' = 1/eʸ = 1/x.

- *tan x*: tan x = sin x / cos x. By the quotient rule: (cos x · cos x − sin x · (−sin x)) / cos² x = (cos²x + sin²x)/cos²x = 1/cos²x = sec²x.

- *arctan x*: Let y = arctan x. Then tan y = x. Differentiating implicitly: sec²y · y' = 1, so y' = 1/sec²y = 1/(1 + tan²y) = 1/(1 + x²).

The power rule extends to all real exponents via the formula xⁿ = e^{n ln x} and the chain rule:
$$\frac{d}{dx} x^n = \frac{d}{dx} e^{n \ln x} = e^{n \ln x} \cdot \frac{n}{x} = x^n \cdot \frac{n}{x} = nx^{n-1}$$

This works for any real n (provided x > 0, or with appropriate care about the domain).

---

## 4.2.6 Higher-Order Derivatives

If f is differentiable on an interval, f' is itself a function. If f' is differentiable, its derivative is the **second derivative** f'' = d²f/dx². More generally, the nth derivative is:

$$f^{(n)}(x) = \frac{d^n f}{dx^n}$$

*Notation*: f', f'', f''', f⁽⁴⁾, ..., f⁽ⁿ⁾. In Leibniz notation: df/dx, d²f/dx², d³f/dx³, ..., dⁿf/dxⁿ.

**Physical meaning**:
- f' is velocity if f is position.
- f'' is acceleration.
- In the geodesic equation, the second derivative of a coordinate with respect to proper time appears directly: d²xᵘ/dτ² + Γᵘ_{αβ} (dxᵅ/dτ)(dx^β/dτ) = 0. The Christoffel symbols Γᵘ_{αβ} encode the geometry, but the structure is: second derivative = (correction term). This is a profound generalization of Newton's second law.

**Smoothness classes**: A function is **Cⁿ** (n-times continuously differentiable) if its first n derivatives all exist and are continuous. A function is **C∞** (smooth) if all derivatives exist. A function is **analytic** (Cω) if it equals its Taylor series locally.

For general relativity, we typically require spacetime to be a **C∞ manifold** — smoothness is assumed to ensure that differentiation can be applied without restriction. More precisely, the metric tensor gᵤᵥ is required to be C² (at minimum) so that the Riemann curvature tensor, which involves second derivatives of the metric, is well-defined. [Hawking, S.W. and Ellis, G.F.R. (1973). *The Large Scale Structure of Space-Time*. Cambridge University Press. Section 2.1.]

---

## 4.2.7 The Derivative as a Linear Map

There is a more abstract way to think about the derivative that will prove essential in differential geometry.

The function f is differentiable at a with derivative f'(a) if and only if:

$$f(a + h) = f(a) + f'(a) \cdot h + o(h) \quad \text{as } h \to 0$$

where o(h) denotes a term that satisfies o(h)/h → 0 as h → 0 (read: "little-o of h").

This says: near a, the function f is well-approximated by the linear function h ↦ f'(a) · h, called the **differential** of f at a. The derivative f'(a) is not a number but a **linear map from ℝ to ℝ**: the map that multiplies by f'(a).

This restatement generalizes immediately:
- In ℝⁿ → ℝᵐ: the derivative at a point becomes the **Jacobian matrix**, a linear map from ℝⁿ to ℝᵐ (Chapter 7).
- On a manifold: the derivative of a smooth function f: M → N at a point p becomes the **pushforward** (df)_p: T_pM → T_{f(p)}N, a linear map between tangent spaces (Chapter 27).

The fundamental insight — that differentiation is linear approximation — remains the same at every level of generality. This is why, in GR, infinitesimal geometry (which is the geometry of tangent spaces) is flat Minkowski geometry, while the large-scale geometry is curved. The derivative "sees" the flat local structure.

---

## References

- Apostol, T.M. (1967). *Calculus*, Vol. 1, 2nd ed. Wiley. [Chapter 4 covers differentiation rules with full proofs.]
- Hawking, S.W. and Ellis, G.F.R. (1973). *The Large Scale Structure of Space-Time*. Cambridge University Press. [Section 2.1 on the differentiability requirements for spacetime manifolds.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [Chapter 5 on differentiation; the chain rule proof is Theorem 5.5.]
- Spivak, M. (1994). *Calculus*, 3rd ed. Publish or Perish. [Chapters 9–11; Spivak's treatment of the derivative is particularly careful about the linear-approximation perspective.]
- Weierstrass, K. (1872). "Über continuirliche Functionen eines reellen Arguments, die für keinen Werth des letzeren einen bestimmten Differentialquotienten besitzen." *Königliche Akademie der Wissenschaften*, Berlin. [The original nowhere-differentiable continuous function.]
