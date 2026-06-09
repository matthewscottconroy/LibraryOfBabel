# L'Hopital's Rule

Limits of the form $\lim_{x \to a} f(x)/g(x)$ where both $f(x) \to 0$ and $g(x) \to 0$ (the $0/0$ form), or both $f(x) \to \pm\infty$ and $g(x) \to \pm\infty$ (the $\infty/\infty$ form), cannot be evaluated by substituting the limit values directly — the ratio is indeterminate. L'Hopital's rule resolves such limits by replacing the ratio of functions with the ratio of their derivatives, reducing the problem to (often) simpler limits. The rule is proved using the Cauchy Mean Value Theorem.

## Statement for the $0/0$ Case

**Theorem (L'Hopital's Rule, $0/0$).** Suppose $f$ and $g$ are differentiable on $(a - \delta, a + \delta) \setminus \{a\}$ for some $\delta > 0$, with $g'(x) \neq 0$ there. If $\lim_{x\to a} f(x) = 0$, $\lim_{x\to a} g(x) = 0$, and
$$\lim_{x\to a} \frac{f'(x)}{g'(x)} = L$$
(where $L \in \mathbb{R}$ or $L = \pm\infty$), then
$$\lim_{x\to a} \frac{f(x)}{g(x)} = L.$$

*Proof.* Extend $f$ and $g$ by setting $f(a) = g(a) = 0$ (this makes them continuous at $a$). For $x > a$ in the punctured neighborhood, apply the Cauchy Mean Value Theorem on $[a, x]$: there exists $c \in (a, x)$ with
$$\frac{f(x) - f(a)}{g(x) - g(a)} = \frac{f'(c)}{g'(c)},$$
i.e., $\frac{f(x)}{g(x)} = \frac{f'(c)}{g'(c)}$.
As $x \to a^+$, the intermediate point $c \in (a, x)$ satisfies $c \to a^+$, so $\frac{f'(c)}{g'(c)} \to L$. Similarly from the left. $\square$

## The $\infty/\infty$ Case

**Theorem (L'Hopital's Rule, $\infty/\infty$).** Under the same differentiability conditions, if $\lim_{x\to a} |f(x)| = \infty$, $\lim_{x\to a} |g(x)| = \infty$, and $\lim_{x\to a} f'(x)/g'(x) = L$, then $\lim_{x\to a} f(x)/g(x) = L$.

The proof for this case is slightly more involved and uses the Cauchy MVT applied to the reciprocal functions. The statement is the same form and the rule is applied identically.

## Limits at Infinity

The rule also applies when $x \to \pm\infty$: if $f(x)/g(x)$ is $0/0$ or $\infty/\infty$ as $x \to \infty$, the substitution $t = 1/x$ reduces it to a limit as $t \to 0^+$.

## Worked Examples

**Example 1 (Basic $0/0$).** $\lim_{x\to 0} \frac{\sin x}{x}$.

Both numerator and denominator approach $0$. Differentiate: $(\sin x)' = \cos x$, $(x)' = 1$. L'Hopital gives $\lim_{x \to 0} \frac{\cos x}{1} = \cos 0 = 1$.

**Example 2 (Repeated application).** $\lim_{x\to 0} \frac{1 - \cos x}{x^2}$.

$0/0$ form. Differentiate: $\frac{\sin x}{2x}$, still $0/0$. Differentiate again: $\frac{\cos x}{2} \to \frac{1}{2}$ as $x \to 0$.

**Example 3 ($\infty/\infty$).** $\lim_{x\to\infty} \frac{x^n}{e^x}$ for any fixed $n \in \mathbb{N}$.

Apply L'Hopital $n$ times: $\frac{x^n}{e^x} \to \frac{n x^{n-1}}{e^x} \to \frac{n(n-1)x^{n-2}}{e^x} \to \cdots \to \frac{n!}{e^x} \to 0$.

So the exponential grows faster than any polynomial.

**Example 4 ($0 \cdot \infty$ form).** $\lim_{x\to 0^+} x \ln x$.

Write as $\frac{\ln x}{1/x}$, the $-\infty/\infty$ form. L'Hopital: $\frac{1/x}{-1/x^2} = \frac{x^2 \cdot (-1)}{x} \cdot (-1) = x \to 0$.

Wait, let us redo: $\frac{d}{dx}[\ln x] = 1/x$ and $\frac{d}{dx}[1/x] = -1/x^2$. The ratio is $(1/x)/(-1/x^2) = -x \to 0$. So $\lim_{x\to 0^+} x\ln x = 0$.

## Indeterminate Forms and Reduction

The standard indeterminate forms are $0/0$, $\infty/\infty$, $0 \cdot \infty$, $\infty - \infty$, $0^0$, $1^\infty$, and $\infty^0$. The non-ratio forms are reduced to $0/0$ or $\infty/\infty$ before applying L'Hopital:

- $0 \cdot \infty$: write $fg = f/(1/g)$ or $fg = g/(1/f)$.
- $\infty - \infty$: combine into a single fraction.
- $0^0$, $1^\infty$, $\infty^0$: write $f^g = e^{g\ln f}$ and evaluate the exponent using the earlier forms.

**Example ($1^\infty$).** $\lim_{x\to\infty} \left(1 + \frac{1}{x}\right)^x = e$.

Write as $e^{x \ln(1+1/x)}$. The exponent $x\ln(1+1/x) = \frac{\ln(1+1/x)}{1/x}$ is $0/0$ as $x\to\infty$. L'Hopital: $\frac{\frac{-1/x^2}{1+1/x}}{-1/x^2} = \frac{x}{x+1} \to 1$. So the limit is $e^1 = e$.

## When L'Hopital Does Not Apply

**The rule requires the hypotheses.** If the limit $f'(x)/g'(x)$ does not exist, L'Hopital gives no information (the conclusion requires this limit to exist). Example: $\lim_{x\to\infty} \frac{x + \sin x}{x}$. Writing as $1 + (\sin x)/x \to 1$ directly is valid, but applying L'Hopital gives $\frac{1 + \cos x}{1}$, which does not exist. The issue: L'Hopital's conclusion is valid only when $f'/g'$ has a limit.

**Not indeterminate.** $\lim_{x\to 0} \frac{x}{1} = 0$ is not indeterminate and should not be "fixed" with L'Hopital.

## Connection to Taylor Series

L'Hopital's rule is closely related to Taylor expansion. Near $x = a$:
$$\frac{f(x)}{g(x)} \approx \frac{f(a) + f'(a)(x-a) + \cdots}{g(a) + g'(a)(x-a) + \cdots}.$$
If $f(a) = g(a) = 0$, the leading terms are $f'(a)(x-a)$ and $g'(a)(x-a)$, giving ratio $f'(a)/g'(a)$ — exactly L'Hopital's conclusion for the $0/0$ case. Taylor series thus provide an alternative to repeated application of L'Hopital.

## Relevance to Differential Equations

L'Hopital's rule appears in ODE analysis when evaluating the behavior of solutions near singular points, in computing limits of solution families as parameters change, and in asymptotic analysis of solutions as $t \to \infty$. The stability of an equilibrium point — whether nearby solutions approach or diverge from it — is often analyzed by evaluating limits of the form $\lim_{y\to 0} f(y)/y$, which are $0/0$ and handled by L'Hopital.
