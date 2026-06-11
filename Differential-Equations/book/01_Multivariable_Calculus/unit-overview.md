# Unit Overview: Multivariable Calculus

## Why Multivariable Calculus Is Unavoidable

Single-variable calculus describes change along a line. The moment we ask how temperature varies across a room, how a membrane vibrates in two dimensions, or how a fluid flows through a pipe, we are dealing with functions of more than one variable, and the single-variable theory is simply insufficient. The derivative $df/dx$ becomes a matrix (the Jacobian), the single integral becomes a multiple integral over a region in $\mathbb{R}^n$, and the fundamental theorems of calculus ramify into a family of theorems — Green's, Stokes', the Divergence Theorem — that relate integrals over regions to integrals over their boundaries.

For the study of differential equations, multivariable calculus is not background: it is the primary language. Partial differential equations — the heat equation, wave equation, Laplace equation — are written in terms of partial derivatives. The existence theory for ODEs in the phase plane requires understanding differentiability in $\mathbb{R}^2$. The method of characteristics for first-order PDEs is a geometric argument in $\mathbb{R}^3$. The Jacobian matrix governs the linearization of nonlinear systems near equilibria, which is the foundation of local stability analysis. There is no path around this material.

There is also a conceptual payoff. Multivariable calculus is where the undergraduate curriculum first encounters the tight interplay between algebra and geometry that characterizes modern mathematics. The gradient is not just a computational device; it is the unique vector field that best describes the directional behavior of a function, and its orthogonality to level sets is a geometric theorem with algebraic proof. The Jacobian matrix is not just a table of partial derivatives; it is the linear map that best approximates a smooth function near a point, and the chain rule is the statement that approximating linear maps compose correctly.

## Central Theorems

**Theorem (Differentiability).** A function $f : \mathbb{R}^n \to \mathbb{R}^m$ is differentiable at $\mathbf{a}$ if there exists a linear map $L : \mathbb{R}^n \to \mathbb{R}^m$ such that
$$\lim_{\mathbf{h} \to \mathbf{0}} \frac{\|f(\mathbf{a} + \mathbf{h}) - f(\mathbf{a}) - L(\mathbf{h})\|}{\|\mathbf{h}\|} = 0.$$
The linear map $L$, when it exists, is unique and is represented by the Jacobian matrix $Df(\mathbf{a})$ whose $(i,j)$ entry is $\partial f_i/\partial x_j$.

The critical point: existence of all partial derivatives does not imply differentiability. The function $f(x,y) = xy/(x^2+y^2)$ for $(x,y)\neq 0$ has both partial derivatives equal to zero at the origin, yet is not continuous there (and hence not differentiable). The correct criterion is that partial derivatives exist and are continuous (which implies differentiability, by the $C^1$ criterion).

**Theorem (Chain Rule in Several Variables).** If $g : \mathbb{R}^n \to \mathbb{R}^m$ is differentiable at $\mathbf{a}$ and $f : \mathbb{R}^m \to \mathbb{R}^p$ is differentiable at $g(\mathbf{a})$, then $f \circ g$ is differentiable at $\mathbf{a}$ and
$$D(f \circ g)(\mathbf{a}) = Df(g(\mathbf{a})) \cdot Dg(\mathbf{a}).$$
(Matrix product of Jacobians.) This is the abstract form of all the specific chain rule formulas: $\partial z/\partial s = (\partial z/\partial x)(\partial x/\partial s) + (\partial z/\partial y)(\partial y/\partial s)$, etc.

**Theorem (Clairaut/Schwarz).** If $f : U \subset \mathbb{R}^n \to \mathbb{R}$ has continuous second-order partial derivatives on an open set $U$, then the mixed partials are equal: $\partial^2 f/\partial x_i \partial x_j = \partial^2 f/\partial x_j \partial x_i$.

This theorem is used constantly in verifying that PDEs are exact (for the method of characteristics) and in reducing the number of independent second-order coefficients in PDEs.

**Theorem (Extreme Value Theorem).** A continuous function on a closed bounded set in $\mathbb{R}^n$ attains its maximum and minimum.

**Theorem (Second Derivative Test).** Let $f$ be $C^2$ on an open set in $\mathbb{R}^2$ and let $(a,b)$ be a critical point ($\nabla f = 0$). Let $H = \begin{pmatrix} f_{xx} & f_{xy} \\ f_{yx} & f_{yy} \end{pmatrix}$ be the Hessian at $(a,b)$.
- $\det H > 0$ and $f_{xx} > 0$: local minimum.
- $\det H > 0$ and $f_{xx} < 0$: local maximum.
- $\det H < 0$: saddle point.
- $\det H = 0$: test inconclusive.

**Theorem (Implicit Function Theorem).** Let $F : \mathbb{R}^{n+m} \to \mathbb{R}^m$ be $C^1$ near $(\mathbf{a}, \mathbf{b}) \in \mathbb{R}^n \times \mathbb{R}^m$ with $F(\mathbf{a}, \mathbf{b}) = \mathbf{0}$. If the $m \times m$ matrix $D_\mathbf{y} F(\mathbf{a}, \mathbf{b})$ (partial Jacobian with respect to the last $m$ variables) is invertible, then there exists a neighborhood $U$ of $\mathbf{a}$ and a unique $C^1$ function $g : U \to \mathbb{R}^m$ with $g(\mathbf{a}) = \mathbf{b}$ and $F(\mathbf{x}, g(\mathbf{x})) = \mathbf{0}$ for all $\mathbf{x} \in U$. Moreover,
$$Dg(\mathbf{x}) = -[D_\mathbf{y} F(\mathbf{x}, g(\mathbf{x}))]^{-1} D_\mathbf{x} F(\mathbf{x}, g(\mathbf{x})).$$

The Implicit Function Theorem is the workhorse theorem behind the local theory of ODEs (the existence theorem can be formulated as an IFT result), the theory of manifolds, and the Lagrange multiplier method.

**Theorem (Inverse Function Theorem).** Let $f : U \subset \mathbb{R}^n \to \mathbb{R}^n$ be $C^1$ near $\mathbf{a}$ with $Df(\mathbf{a})$ invertible. Then $f$ is a local $C^1$ diffeomorphism near $\mathbf{a}$: there exist neighborhoods $V$ of $\mathbf{a}$ and $W$ of $f(\mathbf{a})$ such that $f : V \to W$ is a bijection with $C^1$ inverse $f^{-1} : W \to V$.

**Theorem (Change of Variables).** If $\phi : U \to V$ is a $C^1$ diffeomorphism between open sets in $\mathbb{R}^n$, and $f : V \to \mathbb{R}$ is integrable, then
$$\int_V f(\mathbf{y})\,d\mathbf{y} = \int_U f(\phi(\mathbf{x}))|\det D\phi(\mathbf{x})|\,d\mathbf{x}.$$

In spherical coordinates ($r,\theta,\phi$) in $\mathbb{R}^3$, the Jacobian determinant is $r^2\sin\theta$, so $d\mathbf{y} = r^2\sin\theta\,dr\,d\theta\,d\phi$. This is not a convention or a mnemonic; it is a theorem.

## How the Sections Build

**Unit 1 (Geometry in $\mathbb{R}^n$):** Establishes the language — vectors, dot product, cross product, lines, planes, quadric surfaces, parametric curves, Frenet-Serret formulas for curvature and torsion. This is the geometric vocabulary; everything algebraic and analytic that follows is interpreted in these geometric terms.

**Unit 2 (Differentiation in Several Variables):** The theoretical core. Introduces limits in $\mathbb{R}^n$ (with the subtlety that paths of approach matter), defines continuity, partial derivatives, and then the total derivative (Jacobian). Builds gradient, directional derivatives, the chain rule, Taylor expansion in several variables, critical points and the Hessian, constrained optimization via Lagrange multipliers, and the Implicit and Inverse Function Theorems. Each concept is strictly more general than its one-variable antecedent.

**Unit 3 (Integration in Several Variables):** Develops double and triple integrals as iterated integrals (Fubini's theorem), change of variables (Jacobian determinant), and applications to mass, center of mass, moments of inertia. This prepares the transition to vector calculus, where the same machinery applies to line and surface integrals.

## Worked Examples of Key Techniques

### Example 1: The Gradient and Level Sets

Let $f(x,y) = x^2 + 2y^2$. The gradient is $\nabla f = (2x, 4y)$. At the point $(1, 1)$, $\nabla f = (2, 4)$.

The level set through $(1,1)$ is $\{(x,y) : x^2 + 2y^2 = 3\}$, an ellipse. To verify orthogonality: the tangent vector to the ellipse at $(1,1)$ satisfies $2x\,dx + 4y\,dy = 0$, i.e., $2\,dx + 4\,dy = 0$, giving direction $\mathbf{t} = (4, -2)$ (or any scalar multiple). Then $\nabla f \cdot \mathbf{t} = (2)(4) + (4)(-2) = 8 - 8 = 0$. The gradient is indeed perpendicular to the level set.

This is not a coincidence. For any smooth $f$ and any point $\mathbf{a}$ on the level set $\{f = c\}$, the gradient $\nabla f(\mathbf{a})$ is perpendicular to every tangent vector to the level set. Proof: if $\boldsymbol{\gamma}(t)$ is any smooth curve on the level set with $\boldsymbol{\gamma}(0) = \mathbf{a}$, then $f(\boldsymbol{\gamma}(t)) = c$ for all $t$. Differentiating at $t=0$: $\nabla f(\mathbf{a}) \cdot \boldsymbol{\gamma}'(0) = 0$.

### Example 2: Lagrange Multipliers

Maximize $f(x,y,z) = xyz$ subject to $g(x,y,z) = x^2 + y^2 + z^2 = 3$ (on the sphere of radius $\sqrt{3}$).

At a constrained extremum, $\nabla f = \lambda \nabla g$:
$$yz = 2\lambda x, \quad xz = 2\lambda y, \quad xy = 2\lambda z.$$
Multiplying the three equations: $(xyz)^2 = 8\lambda^3 xyz$. If $xyz \neq 0$, then $xyz = 8\lambda^3$. Dividing the first equation by the second: $y/x = x/y$, so $x^2 = y^2$, hence $x = \pm y$. Similarly $y = \pm z$. On the sphere: $3x^2 = 3$, $x = \pm 1$. The maximum value of $xyz$ is $1$ (achieved at, e.g., $(1,1,1)$), and the minimum is $-1$.

Lagrange multipliers transform a constrained optimization problem into an unconstrained system. The geometric insight: at a constrained extremum, the level surfaces of $f$ are tangent to the constraint surface, which means their normal vectors (the gradients) are parallel.

### Example 3: The Jacobian and Change of Variables

Evaluate $\iint_R (x^2 + y^2)\,dA$ where $R$ is the disk $x^2 + y^2 \leq 4$.

In polar coordinates: $x = r\cos\theta$, $y = r\sin\theta$, $\det D\phi = r$. So
$$\iint_R (x^2+y^2)\,dA = \int_0^{2\pi}\int_0^2 r^2 \cdot r\,dr\,d\theta = 2\pi \int_0^2 r^3\,dr = 2\pi \cdot 4 = 8\pi.$$

The factor $r$ in the area element $r\,dr\,d\theta$ is the absolute value of the Jacobian determinant of the polar coordinate map.

### Example 4: Second Derivative Test

Find and classify the critical points of $f(x,y) = x^3 - 3x + y^3 - 3y$.

$\nabla f = (3x^2 - 3, 3y^2 - 3) = 0 \Rightarrow x^2 = 1$, $y^2 = 1$. Four critical points: $(\pm 1, \pm 1)$.

Hessian: $H = \begin{pmatrix}6x & 0 \\ 0 & 6y\end{pmatrix}$.

At $(1,1)$: $H = \begin{pmatrix}6&0\\0&6\end{pmatrix}$, $\det H = 36 > 0$, $f_{xx} = 6 > 0$: local minimum.
At $(-1,-1)$: $H = \begin{pmatrix}-6&0\\0&-6\end{pmatrix}$, $\det H = 36 > 0$, $f_{xx} = -6 < 0$: local maximum.
At $(1,-1)$ and $(-1,1)$: $\det H = -36 < 0$: saddle points.

## Historical Notes

**René Descartes (1596–1650)** introduced coordinates in his *La Géométrie* (1637), making it possible to translate geometric statements into algebraic equations. This was the origin of analytic geometry, and the idea that a function $f(x,y)$ is a geometric object (its graph is a surface) goes back to him.

**Gottfried Wilhelm Leibniz (1646–1716)** introduced partial derivatives, using the notation $\partial$ that we still use today. The notation was deliberate: the symbol $\partial$ was chosen to remind readers that a partial derivative holds other variables fixed, contrasting with $d$ for the total differential.

**Jean le Rond d'Alembert (1717–1783)** developed the theory of the total differential $df = (\partial f/\partial x)\,dx + (\partial f/\partial y)\,dy$ and used it to study exact equations. His work on the wave equation (1747) required thinking carefully about functions of two variables simultaneously.

**Alexis Claude Clairaut (1713–1765)** proved the equality of mixed partials under continuity conditions, a theorem now bearing his name (also attributed to Schwarz).

**Joseph-Louis Lagrange (1736–1813)** developed the multiplier method for constrained optimization in his *Mécanique analytique* (1788), originally in the context of mechanics: the constraint forces appear as Lagrange multipliers in the equations of motion.

**Augustin-Louis Cauchy** and **Karl Weierstrass** gave rigorous treatments of limits, continuity, and differentiability in several variables in the mid-nineteenth century, replacing heuristic arguments about infinitesimals with $\epsilon$-$\delta$ definitions that clearly separated the roles of continuity and differentiability.

The Implicit Function Theorem and Inverse Function Theorem were stated and proved rigorously by Cauchy and later by Dini (1878). Their formulation as theorems about the invertibility of the Jacobian matrix reflects the insight — first made precise in the nineteenth century — that smooth maps are locally approximated by their best linear approximation, and invertibility of that linear approximation implies local invertibility of the map itself.

## Connections to Other Units

**Upstream prerequisites:**
- Unit 00 (Foundations) provides the $\epsilon$-$\delta$ framework and convergence theory that makes multivariable limits rigorous.
- Single-variable calculus provides the foundation; this unit is its generalization.

**Downstream in this course:**
- Unit 02 (Vector Calculus) applies multivariable differentiation to vector fields (gradient, divergence, curl) and develops the integral theorems (Green, Stokes, Divergence).
- Unit 03 (ODEs) uses the Jacobian for linearization near equilibria; the existence-uniqueness theorem requires Lipschitz continuity in several variables.
- Unit 05 (PDEs) is entirely written in the language of partial derivatives; the classification of PDEs as elliptic/parabolic/hyperbolic is an algebraic condition on the Hessian of the equation.
- Unit 07 (Dynamical Systems) uses the Jacobian matrix to compute the linearization at a fixed point, whose eigenvalues determine local stability.
- Unit 08 (Advanced Topics) requires smooth manifolds, differential forms, and the full apparatus of differential geometry, which begins here.

## Key Theorems at a Glance

1. **Differentiability in $\mathbb{R}^n$:** The Jacobian $Df(\mathbf{a})$ is the unique linear map satisfying $\|f(\mathbf{a}+\mathbf{h}) - f(\mathbf{a}) - Df(\mathbf{a})\mathbf{h}\| / \|\mathbf{h}\| \to 0$.
2. **$C^1$ implies differentiable:** If all partial derivatives exist and are continuous, the function is differentiable.
3. **Chain Rule:** $D(f\circ g)(\mathbf{a}) = Df(g(\mathbf{a})) \cdot Dg(\mathbf{a})$ (product of Jacobians).
4. **Clairaut's Theorem:** $C^2$ implies $\partial^2 f/\partial x_i\partial x_j = \partial^2 f/\partial x_j\partial x_i$.
5. **Gradient Orthogonality:** $\nabla f(\mathbf{a})$ is perpendicular to the level set $\{f = f(\mathbf{a})\}$ at $\mathbf{a}$.
6. **Second Derivative Test:** Hessian definiteness determines the character of critical points.
7. **Lagrange Multiplier Theorem:** Constrained extrema satisfy $\nabla f = \lambda \nabla g$.
8. **Implicit Function Theorem:** Invertible partial Jacobian $\Rightarrow$ local $C^1$ parametrization of a zero set.
9. **Inverse Function Theorem:** Invertible total Jacobian $\Rightarrow$ local $C^1$ diffeomorphism.
10. **Change of Variables:** $\int_V f\,d\mathbf{y} = \int_U f(\phi(\mathbf{x}))|\det D\phi|\,d\mathbf{x}$; volume distortion is measured by $|\det D\phi|$.
