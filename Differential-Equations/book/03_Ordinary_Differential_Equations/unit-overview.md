# Unit Overview: Ordinary Differential Equations

## Why ODEs Are the Core of the Subject

An ordinary differential equation is a relation of the form $F(x, y, y', \ldots, y^{(n)}) = 0$ or, in normal form, $y^{(n)} = f(x, y, y', \ldots, y^{(n-1)})$, where $y$ is an unknown function of a single independent variable $x$. The qualifier "ordinary" distinguishes these from partial differential equations; the single variable is what makes them tractable by the analytic and algebraic methods developed in this unit.

ODEs model everything that evolves or changes according to a local rule. Radioactive decay is governed by $dN/dt = -\lambda N$. A simple pendulum satisfies $\theta'' + (g/L)\sin\theta = 0$. Population competition between two species obeys Lotka-Volterra equations $x' = ax - bxy$, $y' = -cy + dxy$. The charge on a capacitor in an RLC circuit satisfies a linear second-order ODE with constant coefficients. Newton's second law is itself a second-order ODE. Quantum mechanics, via Schrödinger's equation, reduces for stationary states to an ODE eigenvalue problem. The reach of ordinary differential equations is extraordinary.

What makes this unit intellectually coherent is the interplay of three perspectives that recur throughout: the analytic (explicit formulas and solution techniques), the geometric (phase plane, vector fields, flows), and the qualitative (existence, uniqueness, stability, long-term behavior). No single perspective is sufficient. Explicit formulas exist for only a small fraction of ODEs that arise in practice; numerical methods (Unit 7) and qualitative analysis (phase portraits, Lyapunov stability) are essential for everything else. A student who can solve a separable equation but cannot draw a phase portrait, or who can find eigenvalues but cannot interpret what they say about stability, has incomplete preparation.

## The Existence-Uniqueness Theorem: Picard-Lindelöf

The foundational theorem of the entire subject is the following.

**Theorem (Picard-Lindelöf, 1890).** Suppose $f(x, y)$ is defined on a rectangle $R = \{(x,y) : |x - x_0| \leq a, |y - y_0| \leq b\}$ and satisfies:
1. $f$ is continuous on $R$.
2. $f$ satisfies a Lipschitz condition in $y$ on $R$: there exists a constant $L > 0$ such that $|f(x, y_1) - f(x, y_2)| \leq L|y_1 - y_2|$ for all $(x, y_1), (x, y_2) \in R$.

Let $M = \max_R |f|$ and $h = \min(a, b/M)$. Then the initial value problem $y' = f(x, y)$, $y(x_0) = y_0$ has a unique solution on $[x_0 - h, x_0 + h]$.

**Proof sketch (Picard iteration).** Rewrite the IVP as the integral equation $y(x) = y_0 + \int_{x_0}^x f(t, y(t))\,dt$. Define the Picard iterates: $y_0(x) = y_0$ (constant), $y_{n+1}(x) = y_0 + \int_{x_0}^x f(t, y_n(t))\,dt$.

Step 1: Show each $y_n$ stays in the rectangle $R$ on $[x_0-h, x_0+h]$ (by induction, using $M \cdot h \leq b$).

Step 2: Show the iterates converge uniformly. One establishes the bound $|y_{n+1}(x) - y_n(x)| \leq \frac{ML^n}{(n+1)!}|x-x_0|^{n+1}$, which is the general term of a convergent series. By the Weierstrass M-test, $y_n$ converges uniformly to a continuous function $y$.

Step 3: Pass to the limit in the integral equation to show $y$ solves it; hence $y$ is differentiable and $y' = f(x,y)$.

Step 4: Uniqueness by Gronwall's inequality: if $y$ and $z$ are two solutions, $|y(x) - z(x)| \leq L\int_{x_0}^x |y(t)-z(t)|\,dt$, and Gronwall's lemma forces $|y-z| \equiv 0$.

**Why the Lipschitz condition is sharp.** The equation $y' = y^{1/2}$, $y(0) = 0$ does not satisfy the Lipschitz condition at $y=0$ (since $|y^{1/2} - 0^{1/2}|/|y - 0| = y^{-1/2} \to \infty$). And indeed, it has multiple solutions: $y = 0$ and $y = (x/2)^2$ for $x > 0$ (and patched versions). Uniqueness genuinely fails.

**Maximal solutions.** Picard-Lindelöf gives a local solution. The maximal interval of existence is the largest interval on which a solution can be defined. For $y' = y^2$, $y(0) = 1$, the solution is $y = 1/(1-x)$, defined on $(-\infty, 1)$ — it blows up in finite time. This blow-up is not an artifact; it is a genuine property of the equation.

## Linear ODEs: Structure of the Solution Space

For a linear $n$th-order ODE $y^{(n)} + p_{n-1}(x)y^{(n-1)} + \cdots + p_0(x)y = q(x)$, the solution structure is completely determined by the following theorem.

**Theorem.** If $p_0, \ldots, p_{n-1}$ are continuous on an interval $I$, then:
1. The set of solutions to the homogeneous equation $L[y] = 0$ is an $n$-dimensional vector space (the kernel of $L$).
2. Any $n$ linearly independent solutions $y_1, \ldots, y_n$ form a basis (a fundamental set of solutions).
3. Every solution to the nonhomogeneous equation $L[y] = q$ has the form $y = y_p + c_1 y_1 + \cdots + c_n y_n$, where $y_p$ is any particular solution.

The dimension assertion is equivalent to Picard-Lindelöf: for each $(y_0, y_0', \ldots, y_0^{(n-1)}) \in \mathbb{R}^n$, there exists a unique solution with those initial values. The map $(c_1, \ldots, c_n) \mapsto c_1 y_1 + \cdots + c_n y_n$ is a bijection from $\mathbb{R}^n$ to the solution space, making it $n$-dimensional.

**Wronskian.** The functions $y_1, \ldots, y_n$ are linearly independent (on $I$) if and only if their Wronskian $W[y_1,\ldots,y_n](x) = \det[y_i^{(j-1)}(x)]_{ij} \neq 0$ for some (equivalently, all) $x \in I$. Abel's theorem: $W(x) = W(x_0)\exp\!\left(-\int_{x_0}^x p_{n-1}(t)\,dt\right)$; this shows $W$ is either identically zero or never zero.

## Variation of Parameters

**Theorem (Variation of Parameters for $n$th Order).** Let $y_1, \ldots, y_n$ be a fundamental set of solutions to $L[y] = 0$. A particular solution to $L[y] = q$ is
$$y_p(x) = \sum_{k=1}^n y_k(x)\int \frac{W_k(x)}{W(x)} q(x)\,dx$$
where $W_k$ is the determinant of the Wronskian matrix with the $k$th column replaced by $(0, 0, \ldots, 0, 1)^T$.

For the second-order case $y'' + py' + qy = f$ with fundamental solutions $y_1, y_2$:
$$y_p = -y_1\int\frac{y_2 f}{W}\,dx + y_2\int\frac{y_1 f}{W}\,dx.$$

Variation of parameters requires only that the homogeneous equation be solvable; it places no restriction on the form of $f$, unlike the method of undetermined coefficients (which requires $f$ to be a polynomial, exponential, or sinusoidal). The derivation: assume $y_p = u_1 y_1 + u_2 y_2$ with $u_1' y_1 + u_2' y_2 = 0$ (the constraint that eliminates $u_i''$ terms). Then $u_1' y_1' + u_2' y_2' = f$. Solve this linear system by Cramer's rule.

## Linear Systems: Eigenvalue Methods

An $n$th-order linear ODE with constant coefficients is equivalent to the first-order system $\mathbf{x}' = A\mathbf{x} + \mathbf{b}$ where $A$ is an $n \times n$ constant matrix.

**Theorem.** The general solution to $\mathbf{x}' = A\mathbf{x}$ is $\mathbf{x}(t) = e^{At}\mathbf{x}(0)$, where the matrix exponential is defined by $e^{At} = \sum_{k=0}^\infty (At)^k/k!$.

When $A = PDP^{-1}$ is diagonalizable: $e^{At} = Pe^{Dt}P^{-1}$ where $e^{Dt} = \text{diag}(e^{\lambda_1 t}, \ldots, e^{\lambda_n t})$. The general solution decouples into $n$ independent scalar equations.

When $A$ has a Jordan block $J = \lambda I + N$ (where $N^m = 0$): $e^{Jt} = e^{\lambda t}\sum_{k=0}^{m-1} (Nt)^k/k!$, which is a polynomial in $t$ times $e^{\lambda t}$.

**Phase Portraits for $2 \times 2$ Systems.** The qualitative behavior of $\mathbf{x}' = A\mathbf{x}$ is determined by the eigenvalues of $A$:
- Two real negative eigenvalues: stable node (all trajectories $\to \mathbf{0}$, tangent to slow eigenvector).
- Two real positive eigenvalues: unstable node.
- Real eigenvalues of opposite sign: saddle (stable and unstable manifolds; trajectories hyperbolic).
- Complex eigenvalues $\alpha \pm \beta i$ with $\alpha < 0$: stable spiral.
- Complex eigenvalues with $\alpha > 0$: unstable spiral.
- Purely imaginary eigenvalues ($\alpha = 0$): center (closed elliptical orbits; neutrally stable).
- Repeated eigenvalue, $A = \lambda I$: star node.
- Repeated eigenvalue, $A$ has Jordan form: improper node.

**Linearization of Nonlinear Systems.** For $\mathbf{x}' = \mathbf{F}(\mathbf{x})$ with equilibrium $\mathbf{F}(\mathbf{x}^*) = \mathbf{0}$, the linearized system is $\mathbf{u}' = D\mathbf{F}(\mathbf{x}^*)\mathbf{u}$ where $\mathbf{u} = \mathbf{x} - \mathbf{x}^*$. By the Hartman-Grobman theorem, if $D\mathbf{F}(\mathbf{x}^*)$ has no zero or purely imaginary eigenvalues (hyperbolic equilibrium), the phase portrait of the nonlinear system near $\mathbf{x}^*$ is topologically equivalent to the linearized phase portrait.

## Worked Examples

### Example 1: Picard Iteration

Solve $y' = y$, $y(0) = 1$ by Picard iteration.

$y_0 = 1$, $y_{n+1} = 1 + \int_0^x y_n(t)\,dt$.

$y_1 = 1 + x$, $y_2 = 1 + x + x^2/2$, $y_3 = 1 + x + x^2/2 + x^3/6$, and by induction $y_n = \sum_{k=0}^n x^k/k!$. The limit is $e^x$.

### Example 2: Variation of Parameters

Solve $y'' + y = \sec x$.

Homogeneous solutions: $y_1 = \cos x$, $y_2 = \sin x$, $W = \cos x \cdot \cos x - \sin x \cdot (-\sin x) = 1$.

$u_1' = -y_2 \sec x / W = -\sin x \sec x = -\tan x$, so $u_1 = \ln|\cos x|$.
$u_2' = y_1 \sec x / W = \cos x \sec x = 1$, so $u_2 = x$.

$y_p = \cos x \ln|\cos x| + x\sin x$.

General solution: $y = c_1\cos x + c_2\sin x + \cos x \ln|\cos x| + x\sin x$.

### Example 3: Phase Portrait Analysis

For the Lotka-Volterra system $x' = x(1-y)$, $y' = y(x-1)$:

Equilibria: $(0,0)$ and $(1,1)$.

Jacobian: $J = \begin{pmatrix}1-y & -x \\ y & x-1\end{pmatrix}$.

At $(0,0)$: $J = \begin{pmatrix}1&0\\0&-1\end{pmatrix}$, eigenvalues $\pm 1$: saddle (unstable).

At $(1,1)$: $J = \begin{pmatrix}0&-1\\1&0\end{pmatrix}$, eigenvalues $\pm i$: center (for the linearized system). For the nonlinear system, a conserved quantity $H = x + y - \ln x - \ln y$ shows $(1,1)$ is truly a center.

### Example 4: Series Solution near a Regular Singular Point

Solve $2xy'' + y' + xy = 0$ (a Bessel-type equation). Regular singular point at $x = 0$.

Frobenius method: try $y = x^r\sum_{n=0}^\infty a_n x^n$. Substituting and collecting:

Indicial equation (from the $n=0$ term): $r(2r-1) = 0$, giving $r = 0$ or $r = 1/2$.

For $r = 1/2$: the larger root gives a solution regular near 0. Substituting and solving the recurrence yields $J_0(x)$ (Bessel function of order 0, up to normalization).

## Historical Notes

**Isaac Newton (1643–1727)** was the first to solve differential equations systematically, though he did not use the term. His method of fluxions (1671) solved what we would now call first-order ODEs by power series. The equation of motion for a particle under gravity is Newton's ODE.

**Gottfried Wilhelm Leibniz (1646–1716)** introduced the notation $dy/dx$ and the differential equation formalism. He solved the first-order linear ODE and several first-order nonlinear equations.

**Jakob Bernoulli (1654–1705)** solved the Bernoulli equation $y' + p(x)y = q(x)y^n$ (1695), which is now a standard problem in any ODE course.

**Leonhard Euler (1707–1783)** made contributions too numerous to list completely. He systematized the theory of first-order ODEs, introduced the integrating factor for linear first-order equations, developed Euler's method (the first numerical ODE solver), discovered the relation $e^{i\pi} + 1 = 0$, introduced the notion of the characteristic equation for linear ODEs with constant coefficients, and proved Abel's theorem on the Wronskian. His *Institutiones Calculi Integralis* (1768–70) is the first systematic textbook on ODEs.

**Joseph-Louis Lagrange (1736–1813)** developed the method of variation of parameters (1774), which remains the general method for particular solutions. He also introduced the formalism of Lagrangian mechanics, which converts Newton's equations for constrained systems into ODEs.

**Augustin-Louis Cauchy (1789–1857)** provided the first rigorous proof of the existence of solutions to ODEs, using what is now recognized as Picard iteration, around 1820. He also gave the first proof of the Fundamental Theorem for linear ODEs (the $n$-dimensional solution space).

**Émile Picard (1856–1941)** published the complete Picard iteration argument in 1890, with the Lipschitz condition made explicit. **Ernst Lindelöf (1870–1946)** independently gave a clean formulation shortly thereafter; the theorem bears both names.

**Henri Poincaré (1854–1912)** revolutionized the study of ODEs by introducing the qualitative/geometric approach. His *Mémoire sur les courbes définies par une équation différentielle* (1881–86) introduced phase portraits, equilibrium classification, the Poincaré-Bendixson theorem, and the concept of a limit cycle. Poincaré understood that most ODEs cannot be solved explicitly, and that the right question is about qualitative behavior — stability, periodicity, bifurcations.

**Aleksandr Lyapunov (1857–1918)** developed the direct method for stability (1892): a positive-definite function $V$ whose derivative along solutions is nonpositive certifies stability without explicit solution. This method is now the foundation of control theory.

## Connections to Other Units

**Prerequisites:**
- Unit 00 (Foundations): completeness (for Picard iteration), power series (for series solutions), eigenvalue theory (for linear systems).
- Units 01–02 (Multivariable Calculus, Vector Calculus): the phase plane is $\mathbb{R}^2$; linearization uses the Jacobian; Hamiltonian systems require the gradient.

**Downstream:**
- Unit 04 (Fourier Analysis): Fourier series arise as eigenfunction expansions for Sturm-Liouville problems (Unit 03, Unit 08 of ODEs). The heat and wave equations in Unit 05 are PDEs whose solution by separation of variables reduces to Sturm-Liouville ODEs.
- Unit 05 (PDEs): every PDE in this course is a "system of infinitely many ODEs" in some sense; the analogy drives intuition. Method of characteristics reduces first-order PDEs to systems of ODEs.
- Unit 07 (Dynamical Systems): the Phase portrait theory, Poincaré-Bendixson, bifurcation theory, and chaos theory are all extensions of the ODE theory in this unit.
- Unit 08 (Advanced Topics): Sobolev spaces and the theory of distributions extend the ODE existence theory to PDEs.

## Key Theorems at a Glance

1. **Picard-Lindelöf:** Under continuity and Lipschitz conditions, IVPs have unique solutions; proved by Picard iteration and the Banach contraction principle.
2. **Peano Existence Theorem:** Under continuity alone (no Lipschitz condition), IVPs have at least one local solution (but possibly not a unique one).
3. **Gronwall's Inequality:** $u(t) \leq \alpha + L\int_0^t u(s)\,ds$ implies $u(t) \leq \alpha e^{Lt}$. Used to prove uniqueness and continuous dependence.
4. **Fundamental Theorem for Linear ODEs:** The homogeneous solution space is $n$-dimensional; general solution equals particular plus homogeneous; Wronskian is either always zero or never zero (Abel's theorem).
5. **Variation of Parameters:** Explicit formula for particular solutions requiring only a fundamental set of homogeneous solutions.
6. **Matrix Exponential:** $\mathbf{x}(t) = e^{At}\mathbf{x}(0)$ solves $\mathbf{x}' = A\mathbf{x}$; computed via diagonalization or Jordan form.
7. **Phase Portrait Classification:** Eigenvalues of $A$ completely determine the topology of the phase portrait of $\mathbf{x}' = A\mathbf{x}$.
8. **Hartman-Grobman Theorem:** At a hyperbolic equilibrium, the nonlinear phase portrait is topologically equivalent to the linearized phase portrait.
9. **Sturm-Liouville Spectral Theorem:** A self-adjoint Sturm-Liouville operator has a complete orthonormal set of eigenfunctions with real, discrete eigenvalues — the infinite-dimensional analogue of the Spectral Theorem for symmetric matrices.
10. **Poincaré-Bendixson Theorem:** A bounded orbit in the phase plane of an autonomous $2\times 2$ system either approaches an equilibrium or a limit cycle; chaos is impossible in two dimensions.
