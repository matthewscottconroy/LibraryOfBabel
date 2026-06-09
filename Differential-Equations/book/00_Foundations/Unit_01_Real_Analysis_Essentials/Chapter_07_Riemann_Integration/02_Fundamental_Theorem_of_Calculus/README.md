# The Fundamental Theorem of Calculus

The Fundamental Theorem of Calculus (FTC) is the central theorem of all of classical analysis. It establishes that differentiation and integration, defined by completely different limiting processes — one local, one global — are inverses of each other. Before Newton and Leibniz, the computation of areas (integration) and the computation of tangent lines (differentiation) were regarded as separate, unrelated problems. The FTC reveals their deep unity.

## The Accumulation Function

Given an integrable function $f$ on $[a,b]$, define the **accumulation function**
$$F(x) = \int_a^x f(t)\,dt, \quad x \in [a,b].$$

$F$ measures the signed area under $f$ from $a$ to $x$. As $x$ changes, $F(x)$ changes: it accumulates area.

## The First Part of the FTC

**Theorem (FTC, Part 1).** If $f$ is integrable on $[a,b]$, then $F(x) = \int_a^x f(t)\,dt$ is continuous on $[a,b]$. Moreover, if $f$ is continuous at a point $x_0 \in (a,b)$, then $F$ is differentiable at $x_0$ and $F'(x_0) = f(x_0)$.

*Proof.* **Continuity:** For $x, y \in [a,b]$,
$$|F(x) - F(y)| = \left|\int_x^y f(t)\,dt\right| \leq M|x-y|,$$
where $M = \sup_{[a,b]}|f|$ (which is finite since $f$ is integrable on a compact interval, hence bounded). So $F$ is Lipschitz, hence continuous.

**Differentiability at $x_0$:** Let $\varepsilon > 0$. By continuity of $f$ at $x_0$, there exists $\delta > 0$ with $|f(t) - f(x_0)| < \varepsilon$ for $|t - x_0| < \delta$. For $0 < |h| < \delta$:
$$\frac{F(x_0+h) - F(x_0)}{h} - f(x_0) = \frac{1}{h}\int_{x_0}^{x_0+h} (f(t) - f(x_0))\,dt.$$
Taking absolute values and using $|f(t) - f(x_0)| < \varepsilon$ for $t$ between $x_0$ and $x_0+h$:
$$\left|\frac{F(x_0+h)-F(x_0)}{h} - f(x_0)\right| \leq \frac{1}{|h|} \cdot \varepsilon \cdot |h| = \varepsilon. \quad \square$$

## The Second Part of the FTC

**Theorem (FTC, Part 2).** If $f$ is integrable on $[a,b]$ and $f = G'$ for some function $G$ (i.e., $G$ is an antiderivative of $f$), then
$$\int_a^b f(x)\,dx = G(b) - G(a).$$

*Proof.* Let $F(x) = \int_a^x f(t)\,dt$. By the first part (applied at each point of continuity of $f$, or via a separate argument for the general integrable case), $F'(x) = f(x) = G'(x)$ wherever $f$ is continuous. By the corollary to the MVT, $F - G$ is constant on $[a,b]$: $F(x) - G(x) = C$ for all $x$. At $x = a$: $F(a) - G(a) = 0 - G(a) = -G(a)$, so $C = -G(a)$. At $x = b$: $F(b) = G(b) + C = G(b) - G(a)$. But $F(b) = \int_a^b f(t)\,dt$. $\square$

## Antiderivatives and Indefinite Integrals

An **antiderivative** of $f$ on $[a,b]$ is a function $G$ with $G' = f$. By the corollary to the MVT, any two antiderivatives of $f$ differ by a constant. The **indefinite integral** $\int f(x)\,dx$ denotes the family of all antiderivatives, $G(x) + C$.

FTC Part 2 reduces the computation of a definite integral to finding an antiderivative:
$$\int_a^b f(x)\,dx = \left[G(x)\right]_a^b = G(b) - G(a).$$

**Example.** $\int_0^2 x^3\,dx$. An antiderivative of $x^3$ is $G(x) = x^4/4$. So $\int_0^2 x^3\,dx = 16/4 - 0 = 4$.

## Leibniz's Rule for Differentiation under the Integral Sign

If the limits of integration are themselves functions of $x$:
$$\frac{d}{dx} \int_{u(x)}^{v(x)} f(t)\,dt = f(v(x))v'(x) - f(u(x))u'(x).$$

*Proof.* By the chain rule applied to $F(u, v) = \int_u^v f(t)\,dt$: $\frac{d}{dx}F(u(x),v(x)) = F_v v'(x) + F_u u'(x) = f(v(x))v'(x) - f(u(x))u'(x)$, using FTC Part 1 for $F_v = f(v)$ and $F_u = -f(u)$. $\square$

## The Change of Variables Theorem (Substitution)

**Theorem.** If $g: [c,d] \to [a,b]$ is differentiable with $g' \in C([c,d])$, and $f$ is continuous on $[a,b]$, then
$$\int_c^d f(g(t))g'(t)\,dt = \int_{g(c)}^{g(d)} f(x)\,dx.$$

*Proof.* Let $F$ be an antiderivative of $f$. By the chain rule, $(F \circ g)'(t) = f(g(t))g'(t)$. By FTC Part 2:
$$\int_c^d f(g(t))g'(t)\,dt = [F(g(t))]_c^d = F(g(d)) - F(g(c)) = \int_{g(c)}^{g(d)} f(x)\,dx. \quad \square$$

## Integration by Parts

**Theorem.** If $f$ and $g$ are differentiable with $f', g' \in C([a,b])$, then
$$\int_a^b f'(x)g(x)\,dx = [f(x)g(x)]_a^b - \int_a^b f(x)g'(x)\,dx.$$

*Proof.* The product rule gives $(fg)' = f'g + fg'$. Integrate both sides and apply FTC. $\square$

Integration by parts is the analytic tool behind the method of variation of parameters for non-homogeneous linear ODEs and behind the Laplace transform identity $\mathcal{L}\{f'\}(s) = s\mathcal{L}\{f\}(s) - f(0)$.

## The Integral Form of the ODE

The equivalence between the ODE $y' = f(t,y)$, $y(t_0) = y_0$ and the integral equation $y(t) = y_0 + \int_{t_0}^t f(s, y(s))\,ds$ is precisely the content of FTC Part 2 applied to the unknown function $y$. The advantage of the integral form is that it does not require $y$ to be differentiable — it only requires $y$ to be continuous, which is a weaker condition. This is why Picard iteration is set up in terms of the integral equation, not the differential equation directly.

## Common Pitfalls

**Forgetting to check the hypotheses.** FTC Part 2 requires $G' = f$ on $[a,b]$. If $f$ has singularities (e.g., $f(x) = 1/x$ on $[-1,1]$), the formula $\int_{-1}^1 \frac{1}{x}\,dx = [\ln|x|]_{-1}^1$ fails because $f$ is not integrable on $[-1,1]$.

**Confusing Part 1 and Part 2.** Part 1 says the accumulation function has derivative $f$. Part 2 says the integral equals the antiderivative evaluated at endpoints. Both are needed for a complete treatment.
