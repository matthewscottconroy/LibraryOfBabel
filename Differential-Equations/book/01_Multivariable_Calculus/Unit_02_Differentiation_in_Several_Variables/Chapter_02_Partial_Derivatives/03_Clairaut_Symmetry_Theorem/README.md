# Clairaut's Symmetry Theorem

When differentiating a function of several variables twice, one obtains mixed partial derivatives like $f_{xy} = \frac{\partial^2 f}{\partial y \partial x}$ and $f_{yx} = \frac{\partial^2 f}{\partial x \partial y}$, which differentiate in opposite orders. A natural question is whether the order of differentiation matters. In most cases encountered in practice, it does not: $f_{xy} = f_{yx}$. But this equality is a theorem, not a tautology, and it requires hypotheses. The theorem is classical, attributed to Alexis Claude Clairaut (1740) and later rigorously proved by Herman Schwarz, and it is one of the most useful results in multivariable calculus.

## Statement of the Theorem

**Theorem (Clairaut / Schwarz).** Let $f: D \subseteq \mathbb{R}^n \to \mathbb{R}$, and suppose that both mixed partial derivatives $f_{x_i x_j}$ and $f_{x_j x_i}$ exist on an open set $U \subseteq D$ and are continuous at a point $\mathbf{a} \in U$. Then

$$f_{x_i x_j}(\mathbf{a}) = f_{x_j x_i}(\mathbf{a}).$$

For functions of two variables: if $f_{xy}$ and $f_{yx}$ both exist and at least one is continuous at $(a,b)$, then $f_{xy}(a,b) = f_{yx}(a,b)$.

In particular, if $f \in C^2(U)$ (i.e., all second partial derivatives exist and are continuous on $U$), then all mixed partials of the same order are equal.

## Proof Sketch

The proof reduces the equality of mixed partials to the single-variable mean value theorem, applied twice.

Define the "second mixed difference" of $f$ at $(a,b)$:

$$\Delta = f(a+h, b+k) - f(a+h, b) - f(a, b+k) + f(a, b).$$

Define $g(x) = f(x, b+k) - f(x, b)$. Then $\Delta = g(a+h) - g(a)$. By the mean value theorem, $\Delta = h\,g'(\xi) = h[f_x(\xi, b+k) - f_x(\xi, b)]$ for some $\xi$ between $a$ and $a+h$. Applying MVT again to the function $t \mapsto f_x(\xi, t)$: $\Delta = hk\,f_{xy}(\xi, \eta)$ for some $\eta$ between $b$ and $b+k$.

By a symmetric argument (defining $\tilde{g}(y) = f(a+h, y) - f(a, y)$), $\Delta = hk\,f_{yx}(\xi', \eta')$ for some $\xi'$, $\eta'$.

Dividing by $hk$ and taking $(h,k)\to(0,0)$: if $f_{xy}$ is continuous at $(a,b)$, then $f_{xy}(\xi, \eta)\to f_{xy}(a,b)$, so the limit of the left side is $f_{xy}(a,b)$. Similarly, the limit of the right side is $f_{yx}(a,b)$. Therefore $f_{xy}(a,b) = f_{yx}(a,b)$.

## A Counterexample Without Continuity

The theorem requires continuity of the mixed partials. Without it, $f_{xy}$ and $f_{yx}$ can differ. The classic counterexample is:

$$f(x,y) = \begin{cases} \frac{xy(x^2-y^2)}{x^2+y^2} & (x,y)\neq(0,0) \\ 0 & (x,y)=(0,0) \end{cases}.$$

By direct computation using the definition:

$f_x(0,y) = \lim_{h\to 0}\frac{f(h,y)-f(0,y)}{h} = \lim_{h\to 0}\frac{hy(h^2-y^2)/(h^2+y^2)}{h} = -y$ (for $y\neq 0$, and $f_x(0,0) = 0$). So $f_x(0,y) = -y$ for all $y$.

Therefore $f_{yx}(0,0) = \frac{\partial}{\partial y}f_x(0,y)\big|_{y=0} = \frac{\partial}{\partial y}(-y)\big|_{y=0} = -1$.

Similarly, $f_y(x,0) = x$, so $f_{xy}(0,0) = \frac{\partial}{\partial x}f_y(x,0)\big|_{x=0} = 1$.

Thus $f_{xy}(0,0) = 1 \neq -1 = f_{yx}(0,0)$.

The mixed partials exist at the origin but are not continuous there (they disagree), and indeed they are unequal. This counterexample confirms that continuity is essential.

## Consequences

**1. Hessian matrix is symmetric.** The Hessian $H_f$ is the $n\times n$ matrix of second partial derivatives with $(H_f)_{ij} = f_{x_i x_j}$. Clairaut's theorem ensures that if $f\in C^2$, then $H_f$ is symmetric ($H_f = H_f^T$). Symmetric matrices have real eigenvalues and are diagonalizable over $\mathbb{R}$, which is what makes the second derivative test work cleanly.

**2. Reduction in computation.** For a function of $n$ variables, there are $n^2$ second-order partial derivatives, but only $n(n+1)/2$ distinct ones (the $n$ pure second derivatives plus $\binom{n}{2}$ pairs of mixed partials). For $n = 3$: $9$ entries but only $6$ distinct values; for $n = 10$: $100$ entries but only $55$ distinct values.

**3. Generalization to higher order.** For $f\in C^k$, any two $k$-th order partial derivatives that differ only in the order of differentiation are equal. So $f_{xxy} = f_{xyx} = f_{yxx}$ for $f\in C^3$.

**4. Integrability conditions.** In the theory of differential forms and exact differential equations, the condition $\partial M/\partial y = \partial N/\partial x$ (for $M\,dx + N\,dy$ to be exact) is precisely an instance of Clairaut's theorem: exactness requires that the cross-partials of the potential function are equal.

## Application to PDEs

In partial differential equations, Clairaut's theorem is used constantly, usually implicitly. When one writes the Laplacian $\Delta f = f_{xx} + f_{yy}$, one assumes that $f_{xy}$ and $f_{yx}$ are irrelevant individually (they cancel out in the expression). When one derives the wave equation from Hamilton's principle, the symmetry of the Hessian of the Lagrangian is essential. When analyzing conservation laws, the symmetry of second derivatives of the energy functional is assumed throughout.

## Common Pitfalls

The theorem states that continuous mixed partials are equal; it does not state that merely existing mixed partials are equal. The counterexample above shows the distinction is real.

A related error is assuming that if $f_{xy}$ exists on an open set, it is automatically continuous there. The counterexample above demonstrates that even when $f_{xy}$ and $f_{yx}$ both exist everywhere, they can differ at isolated points (here the origin), and at such points, one or both mixed partials must be discontinuous.
