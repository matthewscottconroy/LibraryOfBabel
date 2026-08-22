# Chapter 07: Riemann Integration

Integration is the mathematical operation for computing accumulated quantities: areas, total displacements, total mass distributions, and — crucially for differential equations — the conversion of a rate equation into a total change equation. The Riemann integral defines this operation rigorously through the process of approximating an area by rectangles and taking a limit. Unlike differentiation, which is a local operation, integration is inherently global: the value of $\int_a^b f(x)\,dx$ depends on $f$ over the entire interval.

## Riemann Sums and Integrability

Section 1 constructs the Riemann integral. A **partition** $\mathcal{P} = \{a = x_0 < x_1 < \cdots < x_n = b\}$ of $[a,b]$ divides it into subintervals. On each subinterval $[x_{i-1}, x_i]$, the function is approximated by a constant, chosen as the infimum (lower Riemann sum $L(f, \mathcal{P})$) or the supremum (upper Riemann sum $U(f, \mathcal{P})$). The function is **Riemann integrable** on $[a,b]$ if $\sup_{\mathcal{P}} L(f,\mathcal{P}) = \inf_{\mathcal{P}} U(f,\mathcal{P})$, and the common value is the integral. The Riemann criterion (integrability iff for every $\varepsilon > 0$ there exists a partition with $U - L < \varepsilon$) and the theorem that every continuous function is integrable are the main results.

## The Fundamental Theorem of Calculus

Section 2 proves both parts of the Fundamental Theorem. The first part states that if $f$ is integrable on $[a,b]$, then the accumulation function $F(x) = \int_a^x f(t)\,dt$ is continuous; and if $f$ is continuous at $x$, then $F'(x) = f(x)$. The second part states that if $F' = f$ is integrable on $[a,b]$, then $\int_a^b f(x)\,dx = F(b) - F(a)$. Together, these two parts reveal that differentiation and integration are inverse operations — the foundational connection that makes all of calculus cohere.

## Integration Techniques

Section 3 develops the two main techniques: substitution (change of variables) and integration by parts. Substitution, $\int f(g(x))g'(x)\,dx = \int f(u)\,du$, is the integral version of the chain rule; its rigorous justification uses the chain rule and the Fundamental Theorem. Integration by parts, $\int f'g = fg - \int fg'$, is the integral version of the product rule.

## Improper Integrals

Section 4 extends integration to unbounded intervals ($\int_a^\infty f$) and to functions with singularities ($\int_a^b f$ when $f$ blows up at an endpoint). Both are defined as limits of proper integrals. Convergence tests for improper integrals (comparison, limit comparison, $p$-test) mirror the tests for series. The Laplace transform, central to ODE theory, is an improper integral $\mathcal{L}\{f\}(s) = \int_0^\infty e^{-st}f(t)\,dt$, and its convergence requires exactly the theory of Section 4.

## Connection to Differential Equations

The link between integration and differential equations is the Fundamental Theorem: every solution to $y' = f(x)$ is of the form $y(x) = y(x_0) + \int_{x_0}^x f(t)\,dt$. More generally, the Picard iteration transforms the ODE $y' = f(x,y)$, $y(x_0) = y_0$ into the integral equation $y(x) = y_0 + \int_{x_0}^x f(t,y(t))\,dt$, making integration the vehicle for proving existence. Improper integrals appear in Laplace transform analysis, in the study of solutions near irregular singular points, and in energy methods for PDEs.
