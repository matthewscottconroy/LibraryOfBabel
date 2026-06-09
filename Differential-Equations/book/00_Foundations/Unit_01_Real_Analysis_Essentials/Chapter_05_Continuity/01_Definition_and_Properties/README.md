# Definition and Properties of Continuity

The intuitive notion that a function is continuous when "you can draw its graph without lifting your pen" is geometrically appealing but mathematically imprecise. A function might be graphed continuously yet have discontinuities at individual points that a drawing cannot reveal. The epsilon-delta definition replaces intuition with a quantitative criterion: $f$ is continuous at $a$ when the values $f(x)$ can be kept arbitrarily close to $f(a)$ by keeping $x$ close enough to $a$.

## The Epsilon-Delta Definition

**Definition.** Let $f: D \to \mathbb{R}$ and $a \in D$. We say $f$ is **continuous at $a$** if
$$\forall \varepsilon > 0,\ \exists \delta > 0,\ \forall x \in D,\ |x-a| < \delta \Rightarrow |f(x) - f(a)| < \varepsilon.$$

We say $f$ is **continuous on $D$** if it is continuous at every point of $D$.

The definition has the same quantifier structure as the epsilon-N definition of sequence convergence: universal in the tolerance, existential in the threshold, and the threshold may depend on the tolerance (and also, for pointwise continuity, on the base point $a$).

**Example.** Prove $f(x) = 3x + 1$ is continuous at every $a \in \mathbb{R}$.

Given $\varepsilon > 0$, choose $\delta = \varepsilon/3$. For $|x - a| < \delta$:
$$|f(x) - f(a)| = |3x + 1 - (3a + 1)| = 3|x - a| < 3 \cdot \frac{\varepsilon}{3} = \varepsilon. \quad \square$$

**Example.** Prove $f(x) = x^2$ is continuous at $a$.

Given $\varepsilon > 0$, assume $|x - a| < 1$ (a preliminary bound). Then $|x| \leq |a| + 1$. Now:
$$|x^2 - a^2| = |x+a||x-a| \leq (|x| + |a|)|x-a| \leq (2|a|+1)|x-a|.$$
Choose $\delta = \min(1, \varepsilon/(2|a|+1))$. For $|x-a| < \delta$:
$$|f(x) - f(a)| \leq (2|a|+1)\delta \leq \varepsilon. \quad \square$$

## Sequential Characterization

**Theorem.** $f: D \to \mathbb{R}$ is continuous at $a \in D$ if and only if for every sequence $(x_n)$ in $D$ with $x_n \to a$, we have $f(x_n) \to f(a)$.

*Proof.* ($\Rightarrow$) Suppose $f$ is continuous at $a$ and $x_n \to a$. Given $\varepsilon > 0$, choose $\delta > 0$ by continuity. Since $x_n \to a$, there exists $N$ with $|x_n - a| < \delta$ for $n > N$. Then $|f(x_n) - f(a)| < \varepsilon$. So $f(x_n) \to f(a)$.

($\Leftarrow$) Suppose $f$ is not continuous at $a$. Then there exists $\varepsilon > 0$ such that for each $\delta = 1/n$, there is $x_n$ with $|x_n - a| < 1/n$ and $|f(x_n) - f(a)| \geq \varepsilon$. So $x_n \to a$ but $f(x_n) \not\to f(a)$. $\square$

The sequential characterization is often the easier tool for proving discontinuity: exhibit a sequence approaching $a$ for which the function values do not approach $f(a)$.

**Example.** $g(x) = \sin(1/x)$ for $x \neq 0$, $g(0) = 0$, is discontinuous at $0$. Consider $x_n = 1/(n\pi + \pi/2)$: then $g(x_n) = \sin(n\pi + \pi/2) = (-1)^n$, which does not converge to $g(0) = 0$.

## Types of Discontinuities

At a point $a$ where $f$ is not continuous, several scenarios arise:

- **Removable discontinuity**: $\lim_{x \to a} f(x)$ exists but equals $f(a)$'s wrong value (or $f(a)$ is not defined). Example: $f(x) = (x^2-1)/(x-1)$ at $x=1$.

- **Jump discontinuity**: the left and right limits both exist but are unequal. Example: the Heaviside step function at $0$.

- **Essential discontinuity**: the limit does not exist. Example: $\sin(1/x)$ at $0$.

## Algebraic Properties

**Theorem.** If $f$ and $g$ are continuous at $a$, then so are $f + g$, $f - g$, $fg$, and (if $g(a) \neq 0$) $f/g$.

*Proof for $f+g$.* Given $\varepsilon > 0$, choose $\delta_1$ so $|f(x) - f(a)| < \varepsilon/2$ for $|x-a| < \delta_1$, and $\delta_2$ so $|g(x) - g(a)| < \varepsilon/2$ for $|x-a| < \delta_2$. Take $\delta = \min(\delta_1, \delta_2)$. Then $|(f+g)(x) - (f+g)(a)| \leq |f(x)-f(a)| + |g(x)-g(a)| < \varepsilon$. $\square$

**Theorem (Composition).** If $g$ is continuous at $a$ and $f$ is continuous at $g(a)$, then $f \circ g$ is continuous at $a$.

*Proof.* Given $\varepsilon > 0$, continuity of $f$ at $g(a)$ gives $\delta_1$ with $|y - g(a)| < \delta_1 \Rightarrow |f(y) - f(g(a))| < \varepsilon$. Continuity of $g$ at $a$ gives $\delta$ with $|x-a| < \delta \Rightarrow |g(x) - g(a)| < \delta_1$. Combined: $|x-a| < \delta \Rightarrow |f(g(x)) - f(g(a))| < \varepsilon$. $\square$

## Common Continuous Functions

All polynomials are continuous on $\mathbb{R}$ (by the algebraic properties and the continuity of $f(x) = x$). All rational functions are continuous on their domains. The trigonometric functions $\sin$ and $\cos$ are continuous on $\mathbb{R}$. The exponential $e^x$ and the logarithm $\ln x$ are continuous on their domains. These are the building blocks from which right-hand sides of ODEs are typically constructed.

## Lipschitz Continuity

A stronger form relevant for ODEs: $f$ is **Lipschitz** on $D$ if there exists $K > 0$ with $|f(x) - f(y)| \leq K|x - y|$ for all $x, y \in D$. Lipschitz continuity implies uniform continuity (proved in Section 4). The Picard-Lindelof theorem requires $f(t, y)$ to be Lipschitz in $y$, uniformly in $t$, to guarantee both existence and uniqueness of solutions.

## Common Pitfalls

**Choosing $\delta$ that depends on $x$, not just $\varepsilon$ and $a$.** For pointwise continuity, $\delta$ may depend on both $\varepsilon$ and $a$. A $\delta$ that also depends on $x$ (the variable) does not establish continuity.

**Proving continuity of a composition in the wrong order.** The composition theorem requires $f$ continuous at $g(a)$, not at $a$.

**Discontinuity by oscillation.** A function like $\sin(1/x)$ is bounded but oscillates so fast near $0$ that no limit exists. The sequential criterion (choosing appropriate $x_n$) is the efficient tool for detecting such discontinuities.
