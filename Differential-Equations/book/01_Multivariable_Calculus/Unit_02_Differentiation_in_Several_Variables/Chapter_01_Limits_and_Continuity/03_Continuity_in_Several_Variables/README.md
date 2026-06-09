# Continuity in Several Variables

A function is continuous if small changes in input produce small changes in output — there are no sudden jumps, tears, or blow-ups. In one variable, this intuition is captured by the $\epsilon$-$\delta$ definition. The same definition works in several variables, with the Euclidean norm measuring "closeness" in $\mathbb{R}^n$. Most functions encountered in practice are continuous on their natural domains, but the places where continuity fails — and the different ways it can fail — carry important geometric and analytic information.

## Definition

A function $f: D \subseteq \mathbb{R}^n \to \mathbb{R}$ is **continuous at $\mathbf{p} \in D$** if

$$\lim_{\mathbf{x} \to \mathbf{p}} f(\mathbf{x}) = f(\mathbf{p}).$$

This requires three conditions to hold simultaneously: (1) $f(\mathbf{p})$ is defined; (2) the limit $\lim_{\mathbf{x}\to\mathbf{p}} f(\mathbf{x})$ exists; (3) the limit equals the function value. The function is **continuous on $D$** if it is continuous at every point of $D$.

In $\epsilon$-$\delta$ language: $f$ is continuous at $\mathbf{p}$ if for every $\epsilon > 0$, there exists $\delta > 0$ such that $\|\mathbf{x}-\mathbf{p}\| < \delta$ and $\mathbf{x} \in D$ imply $|f(\mathbf{x}) - f(\mathbf{p})| < \epsilon$. (Note: unlike the limit definition, $\mathbf{x} = \mathbf{p}$ is allowed here.)

## Standard Continuous Functions

The following classes of functions are continuous on their natural domains:
- **Polynomials:** Any polynomial $p(x_1, \ldots, x_n) = \sum a_{i_1\cdots i_n} x_1^{i_1}\cdots x_n^{i_n}$ is continuous on all of $\mathbb{R}^n$.
- **Rational functions:** $p(\mathbf{x})/q(\mathbf{x})$ is continuous wherever $q(\mathbf{x}) \neq 0$.
- **Composed continuous functions:** If $f$ is continuous at $\mathbf{p}$ and $g$ is continuous at $f(\mathbf{p})$, then $g\circ f$ is continuous at $\mathbf{p}$.
- **Component-wise continuous vector functions:** $\mathbf{F} = (f_1, \ldots, f_m): D \to \mathbb{R}^m$ is continuous iff each component $f_i$ is continuous.

## Preservation Under Operations

**Theorem.** If $f$ and $g$ are continuous at $\mathbf{p}$, then so are $f + g$, $fg$, $|f|$, $\max(f, g)$, $\min(f, g)$, and $f/g$ (provided $g(\mathbf{p}) \neq 0$).

This follows from the corresponding limit laws. The composition rule is particularly powerful: if $f(x, y) = e^{x^2 + \sin y}$, this is a composition of the exponential (continuous) with the polynomial-and-sine function $x^2 + \sin y$ (continuous), hence continuous everywhere.

## The Extreme Value Theorem

**Theorem.** If $f: K \to \mathbb{R}$ is continuous and $K \subseteq \mathbb{R}^n$ is compact (closed and bounded), then $f$ attains its maximum and minimum values on $K$: there exist $\mathbf{p}_{\min}, \mathbf{p}_{\max} \in K$ such that $f(\mathbf{p}_{\min}) \leq f(\mathbf{x}) \leq f(\mathbf{p}_{\max})$ for all $\mathbf{x} \in K$.

This theorem is why optimization problems on closed bounded domains always have solutions. Without compactness, a continuous function can be unbounded (e.g., $f(x,y) = x$ on $\mathbb{R}^2$) or bounded but not attain its supremum (e.g., $f(x,y) = x$ on the open square $(0,1)^2$).

## The Intermediate Value Theorem

**Theorem.** If $f: D \to \mathbb{R}$ is continuous, $D$ is connected, and $\mathbf{a}, \mathbf{b} \in D$ with $f(\mathbf{a}) < c < f(\mathbf{b})$, then there exists $\mathbf{p} \in D$ with $f(\mathbf{p}) = c$.

In one variable, connectedness means $D$ is an interval. In $\mathbb{R}^n$, it means $D$ cannot be split into two nonempty disjoint open pieces — typically, $D$ is a path-connected domain.

## Uniform Continuity

A function is **uniformly continuous** on $D$ if the $\delta$ in the continuity definition can be chosen to depend only on $\epsilon$, not on the particular point $\mathbf{p}$: for every $\epsilon > 0$, there exists $\delta > 0$ such that $\|\mathbf{x}-\mathbf{y}\| < \delta$ implies $|f(\mathbf{x})-f(\mathbf{y})| < \epsilon$ for all $\mathbf{x}, \mathbf{y} \in D$.

**Theorem (Heine-Cantor).** A continuous function on a compact set is uniformly continuous.

Uniform continuity is important for analysis — it is what allows passing limits inside integrals — and will appear when justifying Fubini's theorem and related results.

## Worked Example

Show that $f(x, y) = \frac{x^2 y}{x^4 + y^2}$ for $(x,y) \neq (0,0)$ cannot be extended to a continuous function at $(0,0)$ for any value assigned to $f(0,0)$.

Along the path $y = x^2$: $f(x, x^2) = \frac{x^2 \cdot x^2}{x^4 + x^4} = \frac{x^4}{2x^4} = \frac{1}{2} \to \frac{1}{2}$.

Along the path $y = 0$: $f(x, 0) = 0 \to 0$.

Since different paths give different values, no value $f(0,0)$ can make $f$ continuous at the origin. The function has an essential singularity at $(0,0)$.

## Continuity of Vector-Valued Functions

A map $\mathbf{F}: D \subseteq \mathbb{R}^n \to \mathbb{R}^m$ is continuous at $\mathbf{p}$ if and only if each component $F_i: D \to \mathbb{R}$ is continuous at $\mathbf{p}$. Equivalently, $\lim_{\mathbf{x}\to\mathbf{p}} \mathbf{F}(\mathbf{x}) = \mathbf{F}(\mathbf{p})$ in the Euclidean norm on $\mathbb{R}^m$. This component-wise characterization is what makes the theory tractable.

## Connection to Differentiability

A differentiable function is necessarily continuous (proved in Chapter 3), but not vice versa. In one variable, continuity is almost necessary for reasonable functions to have derivatives; in several variables, the situation is more subtle: a function can have partial derivatives at a point while failing to be continuous there. This striking fact — elaborated in Chapters 2 and 3 — makes the proper definition of differentiability (the total derivative) all the more important.
