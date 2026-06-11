# Unit Overview: Vector Calculus

## Why Vector Calculus Is the Language of Mathematical Physics

Every fundamental law in classical physics is stated in the language of vector calculus. Maxwell's equations for electromagnetism, Fourier's law of heat conduction, Navier-Stokes for fluid flow, Newton's gravitational field — all of these are relations among vector fields and their derivatives (divergence, curl, gradient) or integrals (flux, circulation). The deep reason is that physical laws are local: they describe what happens at a point based on the immediate neighborhood of that point. The divergence captures outflow; the curl captures rotation; the gradient captures the steepest direction of change. And the integral theorems — Green's, Stokes', the Divergence Theorem — are the precise statements that connect local information (what the field does at each point) to global information (what the field does across an entire surface or throughout an entire volume).

For differential equations specifically, vector calculus provides the setting in which partial differential equations are formulated and their solutions interpreted. The heat equation $u_t = k\Delta u$ arises from Fourier's law $\mathbf{q} = -k\nabla u$ and the divergence theorem: the rate of change of heat in a region equals the flux of $\mathbf{q}$ through its boundary, and the divergence theorem converts that surface integral to a volume integral, yielding the PDE. The wave equation, Laplace's equation, and virtually every other classical PDE has a similar derivation that runs through the integral theorems of vector calculus.

## Central Theorems

**Theorem (Fundamental Theorem for Line Integrals).** If $\mathbf{F} = \nabla\varphi$ is a conservative vector field on an open connected set $U \subset \mathbb{R}^n$, and $C$ is any piecewise smooth curve from $\mathbf{a}$ to $\mathbf{b}$ lying in $U$, then
$$\int_C \mathbf{F} \cdot d\mathbf{r} = \varphi(\mathbf{b}) - \varphi(\mathbf{a}).$$
In particular, the line integral of a conservative field around any closed curve is zero.

A vector field $\mathbf{F}$ on a simply connected open set is conservative if and only if $\text{curl}\,\mathbf{F} = \mathbf{0}$ (in $\mathbb{R}^3$) or $\partial F_2/\partial x - \partial F_1/\partial y = 0$ (in $\mathbb{R}^2$).

**Theorem (Green's Theorem).** Let $D \subset \mathbb{R}^2$ be a bounded region with piecewise smooth positively oriented boundary $\partial D$. For $C^1$ functions $P, Q$ on $\overline{D}$:
$$\oint_{\partial D} P\,dx + Q\,dy = \iint_D \left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right)\,dA.$$

Green's theorem relates a line integral around a closed curve to a double integral over the enclosed region. It is the two-dimensional prototype for both Stokes' theorem and the Divergence Theorem.

**Special case (Area formula):** Taking $P = -y/2$, $Q = x/2$ gives $\text{Area}(D) = \frac{1}{2}\oint_{\partial D} x\,dy - y\,dx$.

**Theorem (Stokes' Theorem).** Let $S$ be an oriented surface in $\mathbb{R}^3$ with piecewise smooth boundary $\partial S$, oriented consistently. For a $C^1$ vector field $\mathbf{F}$ on an open set containing $S$:
$$\oint_{\partial S} \mathbf{F}\cdot d\mathbf{r} = \iint_S (\nabla \times \mathbf{F}) \cdot d\mathbf{S}.$$
The circulation of $\mathbf{F}$ around the boundary equals the flux of the curl through the surface.

**Theorem (Divergence Theorem / Gauss's Theorem).** Let $V \subset \mathbb{R}^3$ be a bounded solid region with piecewise smooth outward-oriented boundary $\partial V$. For a $C^1$ vector field $\mathbf{F}$ on an open set containing $\overline{V}$:
$$\oiint_{\partial V} \mathbf{F}\cdot d\mathbf{S} = \iiint_V (\nabla\cdot\mathbf{F})\,dV.$$
The outward flux through the boundary equals the total divergence throughout the volume.

These three theorems — Fundamental Theorem for Line Integrals, Stokes', Divergence — are all instances of one master theorem: $\int_M d\omega = \int_{\partial M} \omega$ for a differential form $\omega$ on an oriented manifold with boundary $M$.

**Theorem (Irrotational implies Conservative).** On a simply connected open set $U \subset \mathbb{R}^3$, $\nabla \times \mathbf{F} = \mathbf{0}$ implies $\mathbf{F} = \nabla\varphi$ for some scalar potential $\varphi$.

The simply connected hypothesis is essential: on $U = \mathbb{R}^3 \setminus \{\text{z-axis}\}$, the field $\mathbf{F} = (-y, x, 0)/(x^2+y^2)$ has zero curl but nonzero circulation around the $z$-axis.

**Theorem (Helmholtz Decomposition).** Any smooth vector field $\mathbf{F}$ on all of $\mathbb{R}^3$ (with sufficient decay at infinity) can be decomposed uniquely as
$$\mathbf{F} = -\nabla\varphi + \nabla\times\mathbf{A}$$
where $\varphi$ is a scalar potential (the irrotational part) and $\mathbf{A}$ is a vector potential (the solenoidal part). This is the Helmholtz decomposition; the condition $\nabla\cdot\mathbf{F} = 0$ characterizes the solenoidal part, and $\nabla\times\mathbf{F} = \mathbf{0}$ characterizes the irrotational part.

This decomposition is fundamental in electromagnetism (where $\mathbf{E}$ has an irrotational component and $\mathbf{B}$ is solenoidal) and in fluid mechanics (where incompressible flows are solenoidal).

## How the Sections Build

**Unit 1 (Vector Fields and Differential Operators):** Defines gradient ($\nabla f$), divergence ($\nabla \cdot \mathbf{F}$), curl ($\nabla \times \mathbf{F}$), and Laplacian ($\Delta f = \nabla\cdot\nabla f$). Develops the vector identities: $\nabla\cdot(\nabla\times\mathbf{F}) = 0$ (curl-free is automatically divergence-free), $\nabla\times(\nabla f) = \mathbf{0}$ (gradient fields have zero curl). These identities are not coincidences; they reflect that $d^2 = 0$ for exterior derivatives. Introduces conservative fields and their potentials.

**Unit 2 (Line Integrals):** Defines scalar and vector line integrals, develops the Fundamental Theorem for Line Integrals, characterizes conservative fields by path independence, and introduces the circulation integral $\oint \mathbf{F}\cdot d\mathbf{r}$.

**Unit 3 (Surface Integrals):** Parametrizes surfaces, defines the surface area element $d\mathbf{S} = \mathbf{n}\,dS$ using the cross product of tangent vectors, defines flux integrals, and computes examples for spheres, cylinders, paraboloids.

**Unit 4 (Fundamental Theorems):** Proves Green's theorem by reducing to the fundamental theorem of calculus on horizontal and vertical slices. Derives Stokes' theorem as a generalization. States and proves the Divergence Theorem. Introduces differential forms and the Generalized Stokes' Theorem.

Each unit provides vocabulary and techniques that are essential for the next. Surface integrals cannot be defined without parametrization (Unit 1 vocabulary); Stokes' theorem relates surface integrals to line integrals (requires both Units 2 and 3).

## Worked Examples of Key Techniques

### Example 1: Computing a Flux Integral

Find the outward flux of $\mathbf{F} = (x, y, z)$ through the unit sphere $S: x^2+y^2+z^2=1$.

By the Divergence Theorem: $\oiint_S \mathbf{F}\cdot d\mathbf{S} = \iiint_V \nabla\cdot\mathbf{F}\,dV$.

$\nabla\cdot\mathbf{F} = \partial x/\partial x + \partial y/\partial y + \partial z/\partial z = 3$.

$\iiint_V 3\,dV = 3 \cdot \text{Vol}(B^3) = 3 \cdot (4\pi/3) = 4\pi$.

Direct computation (for verification): Parametrize by $\mathbf{r}(\theta,\phi) = (\sin\phi\cos\theta, \sin\phi\sin\theta, \cos\phi)$. On the unit sphere $\mathbf{F} = \mathbf{r}$, so $\mathbf{F}\cdot\mathbf{n} = |\mathbf{r}| = 1$, and $\oiint_S 1\,dS = \text{Area}(S^2) = 4\pi$. Agrees.

### Example 2: Using Stokes' Theorem

Compute $\oint_C \mathbf{F}\cdot d\mathbf{r}$ where $\mathbf{F} = (y^2, z, x)$ and $C$ is the intersection of the sphere $x^2+y^2+z^2=4$ and the plane $z = 1$, traversed counterclockwise when viewed from above.

Let $S$ be the disk $x^2+y^2\leq 3$, $z=1$ with upward normal $\mathbf{n} = (0,0,1)$.

$\nabla\times\mathbf{F} = \begin{vmatrix}\mathbf{i}&\mathbf{j}&\mathbf{k}\\ \partial_x&\partial_y&\partial_z\\ y^2&z&x\end{vmatrix} = (0-1, 0-1, 0-2y) = (-1, -1, -2y)$.

$\iint_S (\nabla\times\mathbf{F})\cdot(0,0,1)\,dA = \iint_S -2y\,dA$.

By symmetry, $\iint_{x^2+y^2\leq 3} y\,dA = 0$ (odd function over a symmetric domain). So the circulation is $0$.

### Example 3: Finding a Potential Function

Verify that $\mathbf{F} = (2xy + z^2, x^2 + 2yz, y^2 + 2xz)$ is conservative on $\mathbb{R}^3$ and find its potential.

Check: $\nabla\times\mathbf{F}$:
- $\partial_y(y^2+2xz) - \partial_z(x^2+2yz) = 2y - 2y = 0$
- $\partial_z(2xy+z^2) - \partial_x(y^2+2xz) = 2z - 2z = 0$
- $\partial_x(x^2+2yz) - \partial_y(2xy+z^2) = 2x - 2x = 0$

So $\nabla\times\mathbf{F} = \mathbf{0}$ on $\mathbb{R}^3$ (simply connected), hence $\mathbf{F} = \nabla\varphi$.

From $\partial\varphi/\partial x = 2xy+z^2$: $\varphi = x^2y + xz^2 + g(y,z)$.
From $\partial\varphi/\partial y = x^2 + \partial g/\partial y = x^2 + 2yz$: $\partial g/\partial y = 2yz$, so $g = y^2z + h(z)$.
From $\partial\varphi/\partial z = 2xz + y^2 + h'(z) = y^2 + 2xz$: $h'(z) = 0$, $h = C$.

Potential: $\varphi = x^2y + xz^2 + y^2z + C$.

### Example 4: Deriving the Heat Equation

Let $V$ be an arbitrary solid region with smooth boundary $S$, and let $u(x,y,z,t)$ be temperature.

Conservation of energy: $\frac{d}{dt}\iiint_V c\rho u\,dV = -\oiint_S \mathbf{q}\cdot\mathbf{n}\,dS + \iiint_V Q\,dV$

where $c\rho$ is heat capacity per unit volume, $\mathbf{q}$ is heat flux, and $Q$ is heat source density.

Fourier's law: $\mathbf{q} = -k\nabla u$ (heat flows opposite to temperature gradient).

By the Divergence Theorem: $\oiint_S \mathbf{q}\cdot\mathbf{n}\,dS = \iiint_V \nabla\cdot\mathbf{q}\,dV = -\iiint_V k\Delta u\,dV$.

Substituting and using $\frac{d}{dt}\iiint_V = \iiint_V \frac{\partial}{\partial t}$ (for $V$ fixed):
$$\iiint_V c\rho\frac{\partial u}{\partial t}\,dV = \iiint_V (k\Delta u + Q)\,dV.$$

Since $V$ is arbitrary: $c\rho\frac{\partial u}{\partial t} = k\Delta u + Q$, i.e., $u_t = \kappa\Delta u + Q/(c\rho)$ where $\kappa = k/(c\rho)$.

This is how the heat equation arises — not as a postulate, but as a consequence of the Divergence Theorem plus Fourier's law.

## Historical Notes

**Isaac Newton (1643–1727)** introduced the idea of a vector as a directed quantity and worked with what we now call vector fields in his gravitational theory, though without the formal language we use today.

**Joseph-Louis Lagrange** and **Leonhard Euler** developed the calculus of variations and laid groundwork for field theory, but the modern language of vector calculus was not yet available to them.

**George Green (1793–1841)** stated and proved Green's theorem in a private pamphlet, *An Essay on the Application of Mathematical Analysis to the Theories of Electricity and Magnetism* (1828). Green worked as a self-taught mathematician in Nottingham, and his essay was largely unknown until William Thomson (Lord Kelvin) rediscovered it in 1845. The "Green's functions" that now appear throughout PDE theory are named for this work.

**Carl Friedrich Gauss (1777–1855)** proved the Divergence Theorem as part of his work on gravitational attraction and is often credited with the theorem, which also carries the name Gauss's theorem.

**George Gabriel Stokes (1819–1903)** stated what is now called Stokes' theorem in a letter to Thomson in 1850, and it appeared as a prize problem in the Cambridge Mathematical Tripos examination of 1854.

**James Clerk Maxwell (1831–1879)** was the most important figure in making vector calculus a working tool of physics. His *Treatise on Electricity and Magnetism* (1873) reformulated electrodynamics in the language of vector fields and their divergences and curls. The four Maxwell equations, in vector calculus notation, are a masterpiece of mathematical compression, encoding all of classical electrodynamics in four lines.

**Oliver Heaviside (1850–1925)** introduced the modern notation $\nabla$ (del) for the gradient operator and systematized the vector calculus that physicists use today. Heaviside also developed the formalism of the Heaviside step function and the operational calculus for solving differential equations — a precursor to the Laplace transform method.

**Élie Cartan (1869–1951)** developed the modern theory of differential forms and exterior calculus, which unified all the classical theorems into the single formula $\int_M d\omega = \int_{\partial M} \omega$. This generalization is essential for the formulation of PDEs on curved spaces (Riemannian manifolds) and for modern mathematical physics.

## Connections to Other Units

**Prerequisites from earlier units:**
- Unit 00 (Foundations): convergence, Lipschitz continuity, the structure of $\mathbb{R}^n$.
- Unit 01 (Multivariable Calculus): partial derivatives, Jacobian, multiple integrals, the Implicit Function Theorem.

**Downstream in this course:**
- Unit 03 (ODEs): the geometric theory of ODEs (phase portraits, vector fields, flow) uses divergence and curl to analyze properties of flows. Liouville's theorem on volume preservation by Hamiltonian flows is a consequence of the Divergence Theorem.
- Unit 05 (PDEs): the three canonical second-order PDEs (heat, wave, Laplace) are all derived and analyzed using the Divergence Theorem. Green's identities (derived from the Divergence Theorem) are the main tool for energy estimates and uniqueness proofs.
- Unit 06 (Complex Analysis): complex differentiation and integration are vector calculus in disguise. The Cauchy-Riemann equations are a rotation condition on the Jacobian matrix; Cauchy's theorem is Green's theorem applied to a specific combination of the components of an analytic function.
- Unit 08 (Advanced Topics): Differential forms on manifolds generalize the vector calculus of this unit. The exterior derivative $d$ unifies gradient, curl, and divergence. The Generalized Stokes' Theorem ($\int_M d\omega = \int_{\partial M}\omega$) includes all the integral theorems of this unit as special cases.

## Key Theorems at a Glance

1. **Fundamental Theorem for Line Integrals:** $\int_C \nabla\varphi\cdot d\mathbf{r} = \varphi(\mathbf{b}) - \varphi(\mathbf{a})$.
2. **Path Independence Criterion:** On a simply connected domain, $\oint_C \mathbf{F}\cdot d\mathbf{r} = 0$ for all closed $C$ iff $\nabla\times\mathbf{F} = \mathbf{0}$.
3. **Green's Theorem:** $\oint_{\partial D}(P\,dx + Q\,dy) = \iint_D(\partial Q/\partial x - \partial P/\partial y)\,dA$.
4. **Stokes' Theorem:** $\oint_{\partial S}\mathbf{F}\cdot d\mathbf{r} = \iint_S(\nabla\times\mathbf{F})\cdot d\mathbf{S}$.
5. **Divergence Theorem:** $\oiint_{\partial V}\mathbf{F}\cdot d\mathbf{S} = \iiint_V \nabla\cdot\mathbf{F}\,dV$.
6. **Generalized Stokes' Theorem:** $\int_M d\omega = \int_{\partial M}\omega$ (exterior calculus formulation unifying all the above).
7. **Irrotational $\Rightarrow$ Conservative (on simply connected domains):** $\nabla\times\mathbf{F} = \mathbf{0}$ implies $\mathbf{F} = \nabla\varphi$.
8. **Solenoidal $\Rightarrow$ Vector Potential (on simply connected domains):** $\nabla\cdot\mathbf{F} = \mathbf{0}$ implies $\mathbf{F} = \nabla\times\mathbf{A}$.
9. **Helmholtz Decomposition:** $\mathbf{F} = -\nabla\varphi + \nabla\times\mathbf{A}$ (irrotational plus solenoidal parts).
10. **Green's Identities:** $\iiint_V (u\Delta v - v\Delta u)\,dV = \oiint_{\partial V}(u\nabla v - v\nabla u)\cdot d\mathbf{S}$; used for uniqueness and representation theorems for harmonic functions.
