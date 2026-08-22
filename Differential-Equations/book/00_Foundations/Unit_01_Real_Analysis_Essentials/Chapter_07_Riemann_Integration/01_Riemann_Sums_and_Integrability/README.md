# Riemann Sums and Integrability

The area under a curve cannot be computed directly by algebra — it requires a passage to the limit. The Riemann integral formalizes this passage: divide the region into thin vertical strips, approximate each strip by a rectangle, sum the areas of the rectangles, and take the limit as the strips become infinitely thin. The question is whether this limit exists and is well-defined, independent of how the approximation is constructed.

## Partitions and Riemann Sums

A **partition** of $[a,b]$ is a finite set $\mathcal{P} = \{x_0, x_1, \ldots, x_n\}$ with $a = x_0 < x_1 < \cdots < x_n = b$. The $i$-th subinterval is $[x_{i-1}, x_i]$ with width $\Delta x_i = x_i - x_{i-1}$. The **mesh** of the partition is $\|\mathcal{P}\| = \max_i \Delta x_i$.

Given a bounded function $f: [a,b] \to \mathbb{R}$ and a partition $\mathcal{P}$, define:
$$m_i = \inf_{x \in [x_{i-1}, x_i]} f(x), \qquad M_i = \sup_{x \in [x_{i-1}, x_i]} f(x).$$

The **lower Riemann sum** and **upper Riemann sum** are:
$$L(f, \mathcal{P}) = \sum_{i=1}^n m_i \Delta x_i, \qquad U(f, \mathcal{P}) = \sum_{i=1}^n M_i \Delta x_i.$$

Always $L(f,\mathcal{P}) \leq U(f,\mathcal{P})$.

A **Riemann sum** $R(f, \mathcal{P}, \xi)$ is any sum $\sum_{i=1}^n f(\xi_i)\Delta x_i$ where $\xi_i \in [x_{i-1}, x_i]$ is any sample point; it satisfies $L(f,\mathcal{P}) \leq R(f,\mathcal{P},\xi) \leq U(f,\mathcal{P})$.

## Refinements and Monotonicity

A partition $\mathcal{Q}$ is a **refinement** of $\mathcal{P}$ if $\mathcal{P} \subseteq \mathcal{Q}$ (every partition point of $\mathcal{P}$ is also in $\mathcal{Q}$, with possibly more points added).

**Lemma.** If $\mathcal{Q}$ is a refinement of $\mathcal{P}$, then $L(f,\mathcal{P}) \leq L(f,\mathcal{Q})$ and $U(f,\mathcal{Q}) \leq U(f,\mathcal{P})$.

*Proof.* Adding a partition point splits one subinterval into two, and the infimum of the function on the smaller subinterval is at least as large as on the original (so the lower sum increases). Similarly the supremum decreases. $\square$

**Corollary.** For any two partitions $\mathcal{P}$ and $\mathcal{Q}$ (not necessarily related), $L(f,\mathcal{P}) \leq U(f,\mathcal{Q})$.

*Proof.* Let $\mathcal{R} = \mathcal{P} \cup \mathcal{Q}$ (the common refinement). Then $L(f,\mathcal{P}) \leq L(f,\mathcal{R}) \leq U(f,\mathcal{R}) \leq U(f,\mathcal{Q})$. $\square$

## The Lower and Upper Integrals

Define the **lower integral** $\underline{\int}_a^b f = \sup_{\mathcal{P}} L(f,\mathcal{P})$ and the **upper integral** $\overline{\int}_a^b f = \inf_{\mathcal{P}} U(f,\mathcal{P})$.

By the corollary, $\underline{\int} f \leq \overline{\int} f$ always.

## Riemann Integrability

**Definition.** $f$ is **Riemann integrable** on $[a,b]$ if $\underline{\int}_a^b f = \overline{\int}_a^b f$. The common value is $\int_a^b f(x)\,dx$.

**Theorem (Riemann Criterion).** $f$ is Riemann integrable iff for every $\varepsilon > 0$, there exists a partition $\mathcal{P}$ with $U(f,\mathcal{P}) - L(f,\mathcal{P}) < \varepsilon$.

*Proof.* ($\Rightarrow$) If integrable, $\sup L = \inf U = I$. Choose $\mathcal{P}_1$ with $L(f,\mathcal{P}_1) > I - \varepsilon/2$ and $\mathcal{P}_2$ with $U(f,\mathcal{P}_2) < I + \varepsilon/2$. Take $\mathcal{P} = \mathcal{P}_1 \cup \mathcal{P}_2$. Then $U(f,\mathcal{P}) - L(f,\mathcal{P}) \leq U(f,\mathcal{P}_2) - L(f,\mathcal{P}_1) < \varepsilon$.

($\Leftarrow$) If for each $\varepsilon > 0$ such $\mathcal{P}$ exists, then $\overline{\int}f - \underline{\int}f \leq U(f,\mathcal{P}) - L(f,\mathcal{P}) < \varepsilon$ for all $\varepsilon > 0$, giving $\overline{\int}f = \underline{\int}f$. $\square$

## Classes of Integrable Functions

**Theorem.** Every continuous function $f: [a,b] \to \mathbb{R}$ is Riemann integrable.

*Proof.* By the Heine-Cantor theorem, $f$ is uniformly continuous on $[a,b]$: given $\varepsilon > 0$, there exists $\delta > 0$ with $|f(x)-f(y)| < \varepsilon/(b-a)$ whenever $|x-y| < \delta$. Take any partition $\mathcal{P}$ with mesh $\|\mathcal{P}\| < \delta$. On each subinterval, $M_i - m_i < \varepsilon/(b-a)$ (since the oscillation of $f$ on a subinterval of diameter less than $\delta$ is less than $\varepsilon/(b-a)$). Therefore:
$$U(f,\mathcal{P}) - L(f,\mathcal{P}) = \sum_i (M_i - m_i)\Delta x_i < \frac{\varepsilon}{b-a} \sum_i \Delta x_i = \varepsilon. \quad \square$$

**Theorem.** Every monotone function $f: [a,b] \to \mathbb{R}$ is Riemann integrable.

*Proof.* For a partition $\mathcal{P}$ with equal subintervals $\Delta x = (b-a)/n$, the oscillation on each subinterval is $M_i - m_i = f(x_i) - f(x_{i-1})$ (for increasing $f$). Summing: $U - L = \frac{b-a}{n}(f(b)-f(a)) \to 0$. $\square$

**Non-integrable example.** Dirichlet's function $f(x) = 1$ for $x$ rational, $0$ for $x$ irrational, is not Riemann integrable on $[0,1]$: on any subinterval, $M_i = 1$ and $m_i = 0$, so $U - L = 1 - 0 = 1$ for every partition. (It is Lebesgue integrable, equal to $0$.)

## Properties of the Integral

**Linearity:** $\int_a^b (\alpha f + \beta g) = \alpha\int_a^b f + \beta\int_a^b g$.

**Monotonicity:** If $f(x) \leq g(x)$ on $[a,b]$, then $\int_a^b f \leq \int_a^b g$.

**Additivity:** $\int_a^b f = \int_a^c f + \int_c^b f$ for $c \in (a,b)$.

**Bound:** $\left|\int_a^b f\right| \leq \int_a^b |f| \leq M(b-a)$ where $M = \sup_{[a,b]} |f|$.

**Mean Value Theorem for Integrals:** If $f$ is continuous on $[a,b]$, then there exists $c \in (a,b)$ with $\int_a^b f = f(c)(b-a)$.

## Connection to ODE Theory

The integral formulation of an ODE initial value problem $y' = f(t,y)$, $y(t_0) = y_0$ is $y(t) = y_0 + \int_{t_0}^t f(s, y(s))\,ds$. This transformation — from a differential equation to an integral equation — is possible precisely because the Fundamental Theorem of Calculus (next section) equates the two. The Riemann integral provides the rigorous foundation for this transformation, and the bound $|\int| \leq M(b-a)$ is the key estimate in the Picard iteration.
