# Differentiation Rules

Computing derivatives from the limit definition each time is impractical. Fortunately, the algebraic structure of differentiation — its interaction with addition, multiplication, and composition — produces rules that allow derivatives of complex functions to be computed from the derivatives of simpler parts. These rules are theorems, each proved from the limit definition, and together they constitute the computational apparatus of differential calculus.

## Linearity

**Theorem.** If $f$ and $g$ are differentiable at $a$, and $\alpha, \beta \in \mathbb{R}$, then $\alpha f + \beta g$ is differentiable at $a$ and
$$(\alpha f + \beta g)'(a) = \alpha f'(a) + \beta g'(a).$$

*Proof.* $\lim_{h\to 0} \frac{(\alpha f + \beta g)(a+h) - (\alpha f + \beta g)(a)}{h} = \alpha \lim \frac{f(a+h)-f(a)}{h} + \beta \lim \frac{g(a+h)-g(a)}{h} = \alpha f'(a) + \beta g'(a)$. $\square$

Linearity means that differentiation is a linear operator: $D(\alpha f + \beta g) = \alpha Df + \beta Dg$. This is the basis for the theory of linear ODEs: if $y_1$ and $y_2$ are solutions of $Ly = 0$ (where $L$ is a linear differential operator), then so is any linear combination $\alpha y_1 + \beta y_2$.

## The Product Rule

**Theorem.** If $f$ and $g$ are differentiable at $a$, then so is $fg$, and
$$(fg)'(a) = f'(a)g(a) + f(a)g'(a).$$

*Proof.* Add and subtract $f(a+h)g(a)$:
$$\frac{f(a+h)g(a+h) - f(a)g(a)}{h} = f(a+h) \cdot \frac{g(a+h)-g(a)}{h} + \frac{f(a+h)-f(a)}{h} \cdot g(a).$$
As $h \to 0$: $f(a+h) \to f(a)$ (by continuity), $\frac{g(a+h)-g(a)}{h} \to g'(a)$, and $\frac{f(a+h)-f(a)}{h} \to f'(a)$. $\square$

**Generalized Product Rule (Leibniz Rule).** For $n$-fold products or for higher derivatives of products:
$$(fg)^{(n)} = \sum_{k=0}^n \binom{n}{k} f^{(k)} g^{(n-k)}.$$
This mirrors the binomial theorem and is proved by induction.

## The Quotient Rule

**Theorem.** If $f$ and $g$ are differentiable at $a$ and $g(a) \neq 0$, then $f/g$ is differentiable at $a$ and
$$\left(\frac{f}{g}\right)'(a) = \frac{f'(a)g(a) - f(a)g'(a)}{g(a)^2}.$$

*Proof.* Apply the product rule to $f = (f/g) \cdot g$ and solve for $(f/g)'$. Alternatively, compute $\lim_{h\to 0} \frac{1}{h}\left[\frac{f(a+h)}{g(a+h)} - \frac{f(a)}{g(a)}\right]$ directly using algebraic manipulation and the limit of $1/g(a+h) \to 1/g(a)$ (which follows from continuity of $g$ and $g(a) \neq 0$). $\square$

## The Chain Rule

**Theorem.** If $g$ is differentiable at $a$ and $f$ is differentiable at $g(a)$, then $f \circ g$ is differentiable at $a$ and
$$(f \circ g)'(a) = f'(g(a)) \cdot g'(a).$$

*Proof (careful version).* Define $\phi: \mathbb{R} \to \mathbb{R}$ by
$$\phi(k) = \begin{cases} \dfrac{f(g(a)+k) - f(g(a))}{k} & k \neq 0 \\ f'(g(a)) & k = 0 \end{cases}$$
By differentiability of $f$ at $g(a)$, $\phi$ is continuous at $0$. Now:
$$\frac{f(g(a+h)) - f(g(a))}{h} = \phi(g(a+h)-g(a)) \cdot \frac{g(a+h)-g(a)}{h}.$$
As $h \to 0$: $g(a+h) - g(a) \to 0$ (continuity of $g$), so $\phi(g(a+h)-g(a)) \to \phi(0) = f'(g(a))$, and $\frac{g(a+h)-g(a)}{h} \to g'(a)$. $\square$

The introduction of $\phi$ avoids dividing by $g(a+h) - g(a)$, which might be zero for many values of $h$.

**Examples.**
$$\frac{d}{dx}\sin(x^2) = \cos(x^2) \cdot 2x, \qquad \frac{d}{dx}e^{-3x} = e^{-3x} \cdot (-3) = -3e^{-3x}.$$

The chain rule is indispensable in ODE theory: when a substitution is made (e.g., $u = 1/y$ to turn a Bernoulli equation into a linear one), the derivative of the new variable requires the chain rule.

## Derivatives of Elementary Functions

Using the definition and the rules above:

| Function | Derivative |
|---|---|
| $x^n$ ($n \in \mathbb{Z}$) | $nx^{n-1}$ |
| $x^r$ ($r \in \mathbb{R}$) | $rx^{r-1}$ (for $x > 0$) |
| $e^x$ | $e^x$ |
| $\ln x$ | $1/x$ |
| $\sin x$ | $\cos x$ |
| $\cos x$ | $-\sin x$ |
| $\arctan x$ | $1/(1+x^2)$ |

The derivative of $e^x$ is its own derivative, making it the natural basis function for solving linear ODEs with constant coefficients: if $y' = ky$ then $y = Ce^{kx}$.

## Implicit Differentiation

When $y$ is defined implicitly by an equation $F(x, y) = 0$, one differentiates both sides with respect to $x$, treating $y$ as a function of $x$, and uses the chain rule:
$$F_x(x,y) + F_y(x,y) \cdot y' = 0 \implies y' = -\frac{F_x(x,y)}{F_y(x,y)}.$$

This is the analytic basis of separable and exact ODE methods.

## Inverse Function Theorem (One Variable)

**Theorem.** If $f$ is differentiable at $a$ and $f'(a) \neq 0$, and $f$ is injective near $a$, then $f^{-1}$ is differentiable at $b = f(a)$ with $(f^{-1})'(b) = 1/f'(a)$.

*Proof.* $(f^{-1} \circ f)(x) = x$; differentiate both sides using the chain rule: $(f^{-1})'(f(x)) \cdot f'(x) = 1$. At $x = a$: $(f^{-1})'(b) = 1/f'(a)$. $\square$

**Example.** $(\arcsin x)' = 1/\cos(\arcsin x) = 1/\sqrt{1-x^2}$ for $|x| < 1$.

## Common Pitfalls

**Forgetting the chain rule.** The derivative of $f(g(x))$ is $f'(g(x)) \cdot g'(x)$, not $f'(g(x))$ alone or $f'(x) \cdot g'(x)$.

**Product rule vs. constant multiples.** $(c \cdot f)' = c \cdot f'$ for constant $c$ (linearity); $(f \cdot g)' = f'g + fg'$ when both are non-constant (product rule). Treating a function as a constant is a common error.

**Applying quotient rule when product rule suffices.** $\frac{d}{dx}[x^{-1}]$ can be computed as $(-1)x^{-2}$ by the power rule, not necessarily via the quotient rule.
