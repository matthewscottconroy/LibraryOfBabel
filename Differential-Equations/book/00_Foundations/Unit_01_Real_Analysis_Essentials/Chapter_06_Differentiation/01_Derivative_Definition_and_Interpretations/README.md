# Derivative Definition and Interpretations

A number describes a value; a function describes how values change. A derivative describes how a function changes — it is the rate at which output changes per unit change in input, measured exactly at a point rather than over an interval. This local rate of change is defined by a limit, and its geometric, physical, and analytic interpretations make it the central concept in both calculus and differential equations.

## The Definition

**Definition.** Let $f$ be defined on an open interval containing $a$. The **derivative of $f$ at $a$** is
$$f'(a) = \lim_{h \to 0} \frac{f(a+h) - f(a)}{h},$$
provided this limit exists. If it does, $f$ is said to be **differentiable at $a$**.

Equivalently (setting $x = a + h$ and letting $x \to a$):
$$f'(a) = \lim_{x \to a} \frac{f(x) - f(a)}{x - a}.$$

The difference quotient $\frac{f(a+h)-f(a)}{h}$ is the slope of the secant line through $(a, f(a))$ and $(a+h, f(a+h))$. As $h \to 0$, the secant approaches the tangent: the derivative is the slope of the tangent line to the graph of $f$ at $a$.

**Example.** For $f(x) = x^2$, the derivative at $a$:
$$f'(a) = \lim_{h \to 0} \frac{(a+h)^2 - a^2}{h} = \lim_{h \to 0} \frac{2ah + h^2}{h} = \lim_{h \to 0} (2a + h) = 2a.$$
So $f'(a) = 2a$ for all $a$.

**Example.** For $f(x) = |x|$ at $a = 0$:
$$\lim_{h \to 0^+} \frac{|h| - 0}{h} = \lim_{h \to 0^+} 1 = 1, \qquad \lim_{h \to 0^-} \frac{|h|}{h} = \lim_{h \to 0^-} (-1) = -1.$$
The left and right limits differ, so $f'(0)$ does not exist. $|x|$ is not differentiable at $0$.

## Differentiability Implies Continuity

**Theorem.** If $f$ is differentiable at $a$, then $f$ is continuous at $a$.

*Proof.* Write
$$f(x) - f(a) = \frac{f(x)-f(a)}{x-a} \cdot (x-a) \to f'(a) \cdot 0 = 0 \text{ as } x \to a.$$
So $\lim_{x\to a} f(x) = f(a)$. $\square$

The converse fails: $|x|$ is continuous at $0$ but not differentiable there. Weierstrass constructed a function that is continuous everywhere and differentiable nowhere.

## The Derivative as a Linear Approximation

A more conceptual interpretation: $f'(a)$ is the unique real number $m$ such that
$$f(a+h) = f(a) + m \cdot h + o(h) \quad \text{as } h \to 0,$$
where $o(h)$ denotes a quantity with $o(h)/h \to 0$. In other words, the derivative is the coefficient of the best linear approximation to $f$ near $a$. The approximation $f(a+h) \approx f(a) + f'(a)h$ is the linearization of $f$ at $a$, and its error is $o(h)$ — smaller than $h$ in relative terms.

Formally: $f$ is differentiable at $a$ with derivative $m$ iff $\lim_{h\to 0} \frac{f(a+h) - f(a) - mh}{h} = 0$.

## One-Sided Derivatives

**Definition.** The **right derivative** at $a$ is $f'_+(a) = \lim_{h \to 0^+} \frac{f(a+h)-f(a)}{h}$, and the **left derivative** is $f'_-(a) = \lim_{h \to 0^-} \frac{f(a+h)-f(a)}{h}$.

$f$ is differentiable at $a$ iff both one-sided derivatives exist and are equal. This is used, for instance, at corners: a piecewise-defined function is differentiable at a breakpoint iff the slopes from both sides match.

## Higher-Order Derivatives

If $f$ is differentiable on an interval, one can ask whether $f'$ is itself differentiable. If it is, the result is the **second derivative** $f''(a) = (f')'(a)$. More generally, $f^{(n)}$ denotes the $n$-th derivative. Functions with continuous $n$-th derivatives on an interval are said to belong to the class $C^n$; functions with derivatives of all orders are $C^\infty$ (smooth).

For differential equations, the order of the equation is determined by the highest derivative appearing. The solution space of an $n$-th order linear ODE is an $n$-dimensional vector space of $C^n$ functions.

## Physical Interpretation

If $s(t)$ is position as a function of time, then $s'(t) = v(t)$ is velocity and $s''(t) = a(t)$ is acceleration. Newton's second law $F = ma$ is the ODE $ms''(t) = F(t, s(t), s'(t))$, a second-order ODE. The definition of derivative is what makes physical rates of change into mathematical objects.

## The Notation $dy/dx$

Leibniz's notation $\frac{dy}{dx}$ for the derivative of $y$ with respect to $x$ is suggestive of a ratio of infinitesimals. In standard analysis, $dy/dx$ is not a ratio of separate quantities $dy$ and $dx$ — it is the single quantity $f'(x)$. However, in differential forms theory, $dy$ and $dx$ can be given independent meaning. For ODE work, the notation $dy/dx$ is used freely, and the formal manipulation "$dy = f(x)\,dx$" (useful in separation of variables) is justified by the chain rule and the substitution theorem for integrals.

## Worked Example: Derivative from Definition

**Problem.** Find $f'(x)$ for $f(x) = \sqrt{x}$, $x > 0$.

$$f'(x) = \lim_{h\to 0} \frac{\sqrt{x+h} - \sqrt{x}}{h} = \lim_{h\to 0} \frac{(\sqrt{x+h}-\sqrt{x})(\sqrt{x+h}+\sqrt{x})}{h(\sqrt{x+h}+\sqrt{x})} = \lim_{h\to 0} \frac{h}{h(\sqrt{x+h}+\sqrt{x})} = \lim_{h\to 0} \frac{1}{\sqrt{x+h}+\sqrt{x}} = \frac{1}{2\sqrt{x}}.$$

The rationalization technique — multiplying by the conjugate — is a standard trick for limits involving square roots.

## Common Pitfalls

**Assuming differentiability from continuity.** Continuous functions need not be differentiable. The Weierstrass function is continuous but nowhere differentiable.

**Evaluating difference quotients incorrectly.** Algebraic simplification must be done before taking the limit; one cannot substitute $h = 0$ before simplifying the $h$ from the denominator.

**Conflating the derivative with the difference quotient.** The derivative is a limit, not the quotient for any finite $h$. The difference quotient is an approximation that becomes exact only in the limit.
