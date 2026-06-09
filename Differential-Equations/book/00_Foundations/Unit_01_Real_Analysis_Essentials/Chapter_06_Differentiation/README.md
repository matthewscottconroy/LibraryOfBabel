# Chapter 06: Differentiation

Differentiation is the mathematical operation that extracts a rate of change. Defined rigorously as a limit, the derivative of a function $f$ at a point $a$ measures how rapidly $f$ changes near $a$ — it is the slope of the best linear approximation to $f$ at that point. For differential equations, the derivative is the fundamental object: an ODE is a relation between a function and its derivatives, and understanding what derivatives are, when they exist, and what their algebraic and analytic properties are is prerequisite to solving or analyzing any ODE.

## The Derivative as a Limit

The derivative of $f$ at $a$ is defined as
$$f'(a) = \lim_{h \to 0} \frac{f(a+h) - f(a)}{h},$$
when this limit exists. Section 1 develops this definition rigorously, proves that differentiability implies continuity (but not vice versa), and connects the derivative to the idea of a tangent line and a linear approximation. The limit is a quotient, and the manipulations used to compute it require the algebraic limit theorems from Chapter 3.

## Differentiation Rules

Section 2 derives the rules for computing derivatives: linearity ($(\alpha f + \beta g)' = \alpha f' + \beta g'$), the product rule ($(fg)' = f'g + fg'$), the quotient rule, and the chain rule ($(f \circ g)'(x) = f'(g(x)) \cdot g'(x)$). These are theorems deduced from the definition, not heuristic formulas. The chain rule in particular requires a careful proof that avoids the issue of dividing by $g(a+h) - g(a)$, which might be zero.

## The Mean Value Theorem

Section 3 proves the Mean Value Theorem (MVT): if $f$ is continuous on $[a,b]$ and differentiable on $(a,b)$, then there exists $c \in (a,b)$ with $f'(c) = (f(b) - f(a))/(b-a)$. This is one of the most versatile theorems in analysis — it converts local information about derivatives into global information about function values. Monotonicity criteria (if $f' > 0$ then $f$ is increasing), the identity criterion (if $f' = 0$ then $f$ is constant), and most convergence-rate estimates for numerical methods all follow from the MVT.

## L'Hopital's Rule

Section 4 addresses the computation of limits of quotients $f(x)/g(x)$ when both numerator and denominator approach $0$ or $\pm\infty$ — the indeterminate forms. L'Hopital's rule reduces these to the limit of $f'(x)/g'(x)$. The proof uses the Cauchy Mean Value Theorem, a generalization of the MVT.

## Taylor's Theorem

Section 5 extends the linear approximation idea to higher-order polynomial approximations. Taylor's theorem states that if $f$ is $(n+1)$ times differentiable on an interval containing $a$, then
$$f(x) = \sum_{k=0}^n \frac{f^{(k)}(a)}{k!}(x-a)^k + R_n(x)$$
where the **Lagrange remainder** $R_n(x) = \frac{f^{(n+1)}(c)}{(n+1)!}(x-a)^{n+1}$ for some $c$ between $a$ and $x$. This remainder estimate is the key to bounding the error when a function is approximated by a polynomial — essential in power series methods for ODEs.

## How the Sections Connect

Each section builds on the previous: the rules of Section 2 require the definition of Section 1; the MVT of Section 3 uses the Extreme Value Theorem from Chapter 5; L'Hopital's rule uses the MVT; and Taylor's theorem uses both repeated differentiation and the MVT. The chapter as a whole prepares the student for integration (Chapter 7) and for the full theory of ODEs, where derivatives are the primary object of study.
