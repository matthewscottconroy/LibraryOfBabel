# Integration Techniques

The Fundamental Theorem of Calculus reduces the computation of definite integrals to finding antiderivatives. But finding antiderivatives is a nontrivial task: unlike differentiation, which has systematic rules for all elementary functions, integration frequently requires ingenuity. The main systematic techniques — substitution (change of variables) and integration by parts — transform one integral into another that is simpler. Partial fractions and trigonometric substitutions are further tools that handle specific functional forms. These techniques are used constantly in solving ODEs analytically.

## Substitution (Change of Variables)

The substitution rule transforms $\int f(g(x))g'(x)\,dx$ into $\int f(u)\,du$ via $u = g(x)$, $du = g'(x)\,dx$.

**Procedure for definite integrals:**
1. Choose $u = g(x)$ and compute $du = g'(x)\,dx$.
2. Change the limits: when $x = a$, $u = g(a)$; when $x = b$, $u = g(b)$.
3. Substitute and integrate.

**Example.** $\int_0^1 2x e^{x^2}\,dx$.

Let $u = x^2$, $du = 2x\,dx$. When $x=0$, $u=0$; when $x=1$, $u=1$.
$$\int_0^1 2x e^{x^2}\,dx = \int_0^1 e^u\,du = [e^u]_0^1 = e - 1.$$

**Example.** $\int \sin^3 x \cos x\,dx$.

Let $u = \sin x$, $du = \cos x\,dx$:
$$\int u^3\,du = \frac{u^4}{4} + C = \frac{\sin^4 x}{4} + C.$$

Substitution reverses the chain rule. In ODE contexts, it appears in separation of variables: $\frac{dy}{dx} = f(x)g(y)$ is separated as $\frac{dy}{g(y)} = f(x)\,dx$ and integrated on both sides.

## Integration by Parts

Integration by parts (IBP) reverses the product rule: $\int u\,dv = uv - \int v\,du$, or equivalently $\int u(x)v'(x)\,dx = u(x)v(x) - \int u'(x)v(x)\,dx$.

**Strategy.** Choose $u$ to be the factor that becomes simpler upon differentiation, and $dv$ to be the factor that can be easily integrated.

A useful mnemonic is LIATE: prefer $u$ to be Logarithm, Inverse trig, Algebraic, Trigonometric, Exponential (in that order), and $dv$ to be the rest.

**Example.** $\int x e^x\,dx$.

Let $u = x$, $dv = e^x\,dx$. Then $du = dx$, $v = e^x$.
$$\int x e^x\,dx = xe^x - \int e^x\,dx = xe^x - e^x + C = e^x(x-1)+C.$$

**Example.** $\int \ln x\,dx$.

Let $u = \ln x$, $dv = dx$. Then $du = dx/x$, $v = x$.
$$\int \ln x\,dx = x\ln x - \int x \cdot \frac{1}{x}\,dx = x\ln x - x + C.$$

**Repeated IBP.** Sometimes IBP must be applied multiple times, or the integral cycles back to the original (which gives a solvable equation).

**Example.** $\int e^x \sin x\,dx$.

IBP with $u = \sin x$, $dv = e^x\,dx$: $\int e^x \sin x\,dx = e^x \sin x - \int e^x \cos x\,dx$.

IBP again with $u = \cos x$: $\int e^x\cos x\,dx = e^x\cos x + \int e^x\sin x\,dx$.

Substituting back:
$$\int e^x \sin x\,dx = e^x\sin x - e^x\cos x - \int e^x\sin x\,dx.$$
Solve: $2\int e^x\sin x\,dx = e^x(\sin x - \cos x) + C$, so $\int e^x\sin x\,dx = \frac{e^x(\sin x - \cos x)}{2} + C$.

## Partial Fractions

A rational function $P(x)/Q(x)$ (with $\deg P < \deg Q$) can be decomposed into partial fractions corresponding to the factors of $Q$. Each partial fraction can then be integrated using logarithms and arctangents.

**Example.** $\int \frac{x+1}{x^2-x-2}\,dx$.

Factor: $x^2-x-2 = (x-2)(x+1)$. Decompose:
$$\frac{x+1}{(x-2)(x+1)} = \frac{x+1}{(x-2)(x+1)}.$$
Cancel: $= \frac{1}{x-2}$ for $x \neq -1$. So $\int \frac{x+1}{x^2-x-2}\,dx = \ln|x-2| + C$ (away from $x = -1$).

**More general example.** $\int \frac{3x+1}{(x-1)(x^2+1)}\,dx$.

$$\frac{3x+1}{(x-1)(x^2+1)} = \frac{A}{x-1} + \frac{Bx+C}{x^2+1}.$$
Multiply through: $3x+1 = A(x^2+1) + (Bx+C)(x-1)$. At $x=1$: $4 = 2A$, $A=2$. Expanding and matching coefficients gives $B = -2$, $C = -1$. Thus:
$$\int \frac{3x+1}{(x-1)(x^2+1)}\,dx = 2\ln|x-1| - \ln(x^2+1) - \arctan x + C.$$

Partial fractions are the primary tool for computing Laplace transform inverses, which require integrating rational functions of $s$.

## Trigonometric Substitution

For integrals involving $\sqrt{a^2 - x^2}$, $\sqrt{a^2 + x^2}$, or $\sqrt{x^2 - a^2}$, the substitutions $x = a\sin\theta$, $x = a\tan\theta$, or $x = a\sec\theta$ (respectively) eliminate the radical using Pythagorean identities.

**Example.** $\int \sqrt{1-x^2}\,dx$.

Let $x = \sin\theta$, $dx = \cos\theta\,d\theta$, $\sqrt{1-x^2} = \cos\theta$:
$$\int \cos^2\theta\,d\theta = \int \frac{1+\cos 2\theta}{2}\,d\theta = \frac{\theta}{2} + \frac{\sin 2\theta}{4} + C = \frac{\arcsin x}{2} + \frac{x\sqrt{1-x^2}}{2} + C.$$

## Reduction Formulas

For integrals like $\int \sin^n x\,dx$ or $\int x^n e^x\,dx$, IBP yields a **reduction formula** that expresses the $n$-th integral in terms of the $(n-2)$-th or $(n-1)$-th, allowing inductive computation.

$$\int \sin^n x\,dx = -\frac{\sin^{n-1}x\cos x}{n} + \frac{n-1}{n}\int\sin^{n-2}x\,dx.$$

This connects to Gamma function theory, which arises in the study of Bessel functions and other special functions of ODE theory.

## Connection to ODE Solving

Every technique in this section appears directly in ODE solution methods:

- Substitution: separation of variables, Bernoulli equations, homogeneous equations.
- IBP: variation of parameters (the solution formula involves $\int G(t,s)f(s)\,ds$ where $G$ is a Green's function, often requiring IBP).
- Partial fractions: inversion of Laplace transforms.
- Trig substitution: evaluation of integrals arising in arc length, phase plane analysis, and Fourier coefficients.
