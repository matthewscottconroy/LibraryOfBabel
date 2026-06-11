# Historical Notes: The Development of Differential Equations

The history of differential equations is, in many respects, the history of mathematical physics. The subject emerged in the seventeenth century as the mathematical language of Newton's mechanics, matured through the eighteenth century in the hands of Euler and Lagrange, was placed on rigorous foundations in the nineteenth century by Cauchy, and was transformed in the twentieth century by the discovery of distributions, functional analysis, and chaos. What follows is a chronological account of the major figures and ideas, written at the level of precision appropriate for a student who has read this book.

---

## I. The Origins: Newton, Leibniz, and the Calculus of Fluxions

The differential equation, as a mathematical object, could not exist before the calculus. The calculus was invented independently — and almost simultaneously — by Isaac Newton and Gottfried Wilhelm Leibniz in the 1660s and 1670s.

**Isaac Newton (1643–1727)** developed what he called the "method of fluxions" in his unpublished tract *De Methodis Serierum et Fluxionum* (1671), though this was not published until 1736. Newton's central insight was that mechanical quantities — position, velocity, force — are variable quantities changing over time, and that the instantaneous rate of change (the fluxion) of one such quantity with respect to another is well-defined and computable. Newton immediately applied his method to differential equations. He solved $\dot{y} = 1 + y^2$ (what we now recognize as giving $y = \tan(t)$) by the method of power series, obtaining $y = t + t^3/3 + 2t^5/15 + \cdots$ Before Newton, questions about rates of change and tangents were answered geometrically or by *ad hoc* methods; Newton's method reduced all such questions to systematic computation.

Newton's laws of motion are themselves differential equations: $m\ddot{\mathbf{x}} = \mathbf{F}(\mathbf{x}, \dot{\mathbf{x}}, t)$. The equation for planetary motion under gravity — the two-body problem — is the ODE $m\ddot{\mathbf{r}} = -GMm\hat{\mathbf{r}}/r^2$. Newton solved this completely in the *Principia Mathematica* (1687), deriving Kepler's three laws (which had previously been empirical observations) as theorems. This was the first major triumph of differential equations: the reduction of celestial mechanics to computation.

**Gottfried Wilhelm Leibniz (1646–1716)** invented the calculus independently, publishing his results in 1684 (two years before Newton's *Principia*). Leibniz's notation — $dy/dx$, $\int$, $d^2y/dx^2$ — proved far superior to Newton's dot notation for systematic computation, and it is Leibniz's notation that we use today. Leibniz also introduced the concept of the differential equation explicitly: he wrote down equations relating differentials $dy$ and $dx$ and observed that these equations are solved by "integration." He solved several separable equations and the Bernoulli equation, and he introduced the integrating factor for linear first-order equations. His correspondence with Johann Bernoulli (1667–1748) and others shows that he clearly understood the general first-order ODE as a mathematical problem worthy of systematic study.

**Jakob Bernoulli (1654–1705)**, working in Leibniz's tradition, solved the Bernoulli equation $dy/dx + P(x)y = Q(x)y^n$ in 1695. He also studied the brachistochrone problem — finding the curve of fastest descent for a ball rolling under gravity — and the catenary (the shape of a hanging chain), both of which lead to ODEs or variational problems. The Bernoulli numbers, Bernoulli polynomials, and Bernoulli equation each bear his name.

---

## II. The Eighteenth Century: Euler and the Systematic Theory

The eighteenth century was dominated by **Leonhard Euler (1707–1783)**, whose contributions to differential equations are so extensive that they can only be sampled here.

Euler introduced the exponential function $e^x$ and the notation $e$ for its base, unified the trigonometric functions via the complex exponential ($e^{i\theta} = \cos\theta + i\sin\theta$), introduced $i = \sqrt{-1}$, and systematized the algebra of complex numbers — all tools that are fundamental to the solution of linear ODEs with constant coefficients. The characteristic equation method for solving $ay'' + by' + cy = 0$ is entirely due to Euler.

For power series solutions of ODEs, Euler developed what is now called the Frobenius method (though Frobenius published it later) in the context of the hypergeometric equation $x(1-x)y'' + [c-(a+b+1)x]y' - aby = 0$. He understood that a second-order ODE has two linearly independent solutions, that at an ordinary point both can be expressed as power series, and that at a singular point the second solution may require a logarithmic term.

In **numerical methods**, Euler introduced the Euler method: approximate the solution of $y' = f(x,y)$, $y(x_0) = y_0$ by the recurrence $y_{n+1} = y_n + hf(x_n, y_n)$ where $h$ is the step size. This is the simplest example of a one-step method and remains the foundation on which all higher-order Runge-Kutta methods are built. Euler understood its error (of order $h$ per step, order 1 overall) and proposed it as a practical method for cases where analytic solutions were unavailable.

**Systems of ODEs** were introduced systematically by Euler. He reduced the $n$th-order ODE $y^{(n)} = f(x, y, y', \ldots, y^{(n-1)})$ to a first-order system by introducing auxiliary variables, and he recognized that the theory of linear systems $\mathbf{x}' = A\mathbf{x}$ parallels the theory of linear algebraic systems. His three-volume *Institutiones Calculi Integralis* (1768–70) is the first systematic textbook of integration and ordinary differential equations.

**Joseph-Louis Lagrange (1736–1813)** made two contributions of permanent importance. First, his method of **variation of parameters** (1774) gave a general formula for the particular solution of a nonhomogeneous linear ODE, requiring only a fundamental set of homogeneous solutions. The idea is elegant: the constants in the general homogeneous solution are allowed to vary, and the conditions on their derivatives are determined by the requirement that the resulting expression satisfy the nonhomogeneous equation. Second, Lagrange's reformulation of mechanics — the *Mécanique analytique* (1788) — expressed all of classical mechanics in terms of the Lagrangian function and the Euler-Lagrange equations. This not only organized and generalized Newton's laws but introduced the calculus of variations as a systematic subject, with the Euler-Lagrange equation $d/dt(\partial L/\partial\dot{q}) - \partial L/\partial q = 0$ as the central result. Systems described by Lagrangian or Hamiltonian mechanics are conservative — they preserve the total energy — and this conservation property has deep consequences for the qualitative behavior of solutions.

---

## III. Fourier's Heat Equation and the Birth of Fourier Analysis

**Joseph Fourier (1768–1830)** submitted a memoir on heat conduction to the French Academy of Sciences in 1807. In it, he derived the partial differential equation $u_t = \kappa u_{xx}$ (the heat equation) from Fourier's law of conduction and solved it by a method that was entirely new: he proposed to expand the initial temperature distribution $u(x,0) = f(x)$ as an infinite series of sines and cosines, $f(x) = \sum_n b_n\sin(n\pi x/L)$, and then multiply each term by the corresponding decaying exponential $e^{-\kappa n^2\pi^2 t/L^2}$ to obtain the solution.

This paper was rejected by the Academy's referees. Lagrange, then the most eminent analyst in France, objected that the claim that "any function" could be represented as a trigonometric series was false. Lagrange had independently discovered the Fourier series some years earlier but had not published it, precisely because he was unsatisfied with the convergence questions. His skepticism was not unreasonable: it had been observed that the series of continuous functions $\sin(n\pi x/L)$ seemed to represent functions with jump discontinuities, which appeared paradoxical.

Fourier was undeterred. He published a revised version of his work as *Théorie analytique de la chaleur* in 1822. By then, the Fourier series had established itself as an indispensable tool in mathematical physics, even if its convergence remained mysterious.

The resolution of Fourier's question required new analysis. **Peter Gustav Lejeune Dirichlet (1805–1859)** proved in 1829 that a piecewise smooth $2\pi$-periodic function converges, at every $x$, to the arithmetic mean of its left and right limits. Dirichlet's proof introduced the Dirichlet kernel $D_N(x) = \sin((N+1/2)x)/\sin(x/2)$ and the technique of analyzing the partial sums via their integral representation — techniques that are still taught in every analysis course.

**Bernhard Riemann (1826–1866)** generalized the convergence theory and, in investigating sets on which Fourier series can fail to converge, introduced a function-theoretic notion of integration that bears his name. His Habilitation thesis (1854) laid the foundation for what would become Lebesgue integration.

**Georg Cantor (1845–1918)** proved in 1870 that if a Fourier series converges to zero except on a finite set, all its coefficients are zero. Attempting to generalize this to more complex exceptional sets, he introduced the notions of limit points and derived sets that became the foundation of set theory and transfinite arithmetic. The convergence question for Fourier series thus generated, as a byproduct, one of the deepest theories in all of mathematics.

---

## IV. Cauchy and the Rigorous Foundations

The eighteenth-century analysts — Euler and Lagrange above all — worked with extraordinary effectiveness, but without the rigorous foundations that we now regard as essential. They manipulated infinite series without checking convergence, interchanged limits and integrals without justification, and used the concept of the infinitesimal without a precise definition. The foundations were supplied by the nineteenth century, primarily by **Augustin-Louis Cauchy**.

**Augustin-Louis Cauchy (1789–1857)** is the central figure in the rigorous theory of differential equations. In his *Cours d'analyse* (1821), he gave the first rigorous definition of a limit: $f$ is continuous at $a$ if $|f(x) - f(a)| < \epsilon$ whenever $|x - a| < \delta$, for appropriate $\delta$ depending on $\epsilon$. This definition, and the $\epsilon$-$\delta$ language, is what we use today.

Cauchy applied his analytic tools to differential equations. He proved the first general **existence theorem** for ODEs: the initial value problem $y' = f(x,y)$, $y(x_0) = y_0$ has a solution when $f$ is continuous. His proof, which we now recognize as the Picard iteration argument applied to an integral equation, was the model for all subsequent existence proofs. Cauchy also proved the general existence theorem for systems of ODEs, and the Cauchy-Kovalevskaya theorem (proved in full by Sofia Kovalevskaya in 1874) for analytic PDEs: if the data and coefficients are analytic, the solution is analytic and is given locally by a convergent power series.

Cauchy also created **complex analysis** as a systematic theory. His theorem that $\oint_C f(z)\,dz = 0$ for an analytic function $f$ on a simply connected domain (1814, 1825) is the foundation of the entire subject. The Cauchy integral formula, Liouville's theorem, and the residue theorem all followed in rapid succession. For ODEs, complex analysis provided the theory of analytic continuation of solutions (showing that the solutions to linear ODEs with analytic coefficients extend analytically through any region where the coefficients remain analytic), and the classification of singular points as regular or irregular (Fuchs, 1866).

---

## V. Poincaré and the Qualitative Theory

By 1880, the analytic theory of ODEs was well developed: existence-uniqueness results were available, linear equations with constant or analytic coefficients were understood, series methods and Laplace transform methods were in use. But the available methods could solve only a tiny fraction of the ODEs arising in applications. It was clear that for the vast majority of equations — particularly nonlinear ones — explicit solutions were either impossible or impractical.

**Henri Poincaré (1854–1912)** recognized this limitation and responded by creating a new kind of mathematics. His four-part memoir *Sur les courbes définies par une équation différentielle* (1881–86) introduced the **qualitative theory of ODEs**, also called the theory of dynamical systems. Rather than asking "what is the formula for $y(t)$?", Poincaré asked: "What does the solution look like qualitatively? Is it periodic? Does it approach a limit? Is it stable?"

Poincaré introduced the **phase plane** — the plane of $(x, \dot{x})$ or $(x, y)$ for a two-dimensional autonomous system — and described the qualitative behavior of solutions in geometric terms: equilibria, limit cycles, separatrices, and the regions they bound. He classified the equilibria (nodes, spirals, saddles, centers) by the eigenvalues of the linearization, a classification that is standard today.

Poincaré's most far-reaching contribution was his discovery, in the course of the prize competition on the three-body problem (1887–89), of what we now call **sensitive dependence on initial conditions**: the stable and unstable manifolds of a saddle equilibrium, when they intersect transversally (a homoclinic intersection), create a wildly complicated tangle — the **homoclinic tangle** — in which solutions behave in a way that resists any systematic description. Poincaré wrote: "The curves defined by the differential equations in the neighborhood of these doubly asymptotic solutions are infinitely complicated." He was describing the geometric structure of chaos, eighty years before the word existed.

The **Poincaré-Bendixson theorem** (1901) is another landmark: in the plane, every bounded orbit of an autonomous system converges either to an equilibrium or to a closed orbit. This result places a hard limit on complexity in two dimensions: chaos is impossible for autonomous planar systems. The contrast with three-dimensional systems (where the Lorenz attractor lives) could not be sharper.

**Aleksandr Mikhailovich Lyapunov (1857–1918)** developed the **direct method for stability** in his doctoral dissertation *The General Problem of the Stability of Motion* (1892). The central idea: to determine whether an equilibrium is stable, construct a positive-definite function $V(\mathbf{x})$ (a Lyapunov function) and compute $\dot{V} = \nabla V\cdot\mathbf{F}$ along trajectories. If $\dot{V} \leq 0$, energy is non-increasing and the equilibrium is stable; if $\dot{V} < 0$, energy is strictly decreasing and the equilibrium is asymptotically stable. This method requires no explicit solution: it certifies stability through an algebraic condition. It is now the foundation of nonlinear control theory.

---

## VI. Sobolev, Schwartz, and Distribution Theory

The early twentieth century saw the development of functional analysis — the study of infinite-dimensional vector spaces and the operators acting on them — and its application to PDEs.

**Stefan Banach (1892–1945)** introduced the abstract theory of normed spaces in his 1920 doctoral thesis and developed it systematically in *Théorie des opérations linéaires* (1932). Banach's contribution was the recognition that many spaces of functions — continuous functions, square-integrable functions, differentiable functions — share a common algebraic and metric structure (that of a normed complete vector space), and that theorems about linear operators can be proved at the level of this abstract structure. The Banach contraction principle (a complete metric space version of the fixed-point argument) is the abstract form of the Picard iteration proof of existence-uniqueness.

**Sergei Sobolev (1908–1989)** introduced the spaces that bear his name in the 1930s, in the context of hyperbolic PDEs. The key idea: for a PDE of order $k$, what matters is not that the solution has classical derivatives but that it has derivatives in $L^2$ up to order $k$. The Sobolev space $W^{k,2} = H^k$ consists of all $L^2$ functions whose distributional derivatives up to order $k$ are also $L^2$. This is exactly the natural space for variational problems: the energy functional for second-order elliptic PDEs is bounded and coercive on $H^1$, which is precisely what is needed to apply the Lax-Milgram theorem.

Sobolev's embedding theorems — which relate membership in $H^k$ to classical smoothness — provided the bridge between "weak" solutions (defined by integral identities) and "strong" or classical solutions (satisfying the PDE pointwise). The theorem $H^k \hookrightarrow C^j$ for $k - n/2 > j$ says: if you have enough $L^2$ derivatives, you have classical derivatives. The threshold $k > n/2$ is sharp.

**Laurent Schwartz (1915–2002)** created the theory of distributions in the 1940s. The problem he addressed was fundamental: physicists had been using the Dirac delta "function" $\delta(x)$ since Dirac's quantum mechanics book (1930), computing with it as if it were a function despite the fact that no such function exists in the classical sense. Schwartz's resolution: define a distribution as a continuous linear functional on the space $C_c^\infty$ of smooth compactly supported test functions. In this framework, $\delta[\varphi] = \varphi(0)$ is a perfectly well-defined distribution. Every locally integrable function is a distribution. Distributions can be differentiated arbitrarily many times (by the formal adjoint of classical differentiation on test functions), so every distribution has all orders of derivatives — even if they exist only in the distributional sense.

The distributional framework unified and generalized the theory of fundamental solutions. The heat kernel $K(x,t) = (4\pi\kappa t)^{-1/2}e^{-x^2/(4\kappa t)}$ satisfies $K_t - \kappa K_{xx} = \delta(x)\delta(t)$ in the distributional sense: it is a fundamental solution of the heat operator. The solution to the heat equation with initial data $f$ is then $u = K * f$ (convolution in $x$, with $K$ evaluated at time $t$). This representation is not merely formal — it is rigorous in the distributional sense, and it works for initial data $f$ that are merely locally integrable, or even tempered distributions.

Schwartz received the Fields Medal in 1950 for this work, in the first year the medal was awarded.

---

## VII. Lorenz and the Discovery of Chaos

The discovery that simple, low-dimensional deterministic systems could behave in fundamentally unpredictable ways — the phenomenon of **chaos** — was one of the most unexpected mathematical discoveries of the twentieth century. In retrospect, Poincaré had seen it coming in 1889; but his discovery lay dormant, imperfectly understood, for over seventy years.

**Edward Lorenz (1917–2008)** was a meteorologist at MIT studying atmospheric convection. In 1961, while running a numerical simulation of a weather model, he re-entered a solution from the middle of a previous run, using a printout that showed three decimal places rather than the six used in the computer's internal representation. The new simulation diverged completely from the original within a few simulated months. Lorenz realized that the divergence was not due to numerical error but to the intrinsic sensitivity of the equations to initial conditions.

He investigated this systematically, deriving a simplified 12-variable model of atmospheric convection and eventually reducing it to the three-variable system:
$$\dot{x} = \sigma(y-x), \quad \dot{y} = x(\rho-z)-y, \quad \dot{z} = xy-\beta z$$
with $\sigma = 10$, $\rho = 28$, $\beta = 8/3$. He published his findings in "Deterministic Nonperiodic Flow" (*Journal of Atmospheric Sciences*, 1963).

In this paper, Lorenz observed:
1. The system is deterministic — no randomness.
2. Solutions are bounded (the system is dissipative: $\nabla\cdot\mathbf{F} = -\sigma-1-\beta < 0$).
3. Solutions are non-periodic: they never repeat.
4. Solutions starting at nearby initial conditions diverge exponentially.

The attractor — the fractal set on which the aperiodic solutions eventually live — is now called the Lorenz attractor. Its dimension is approximately $2.06$ (between 2 and 3), making it a strange attractor in the sense of Ruelle and Takens (1971).

Lorenz's paper sat largely unnoticed in the meteorology literature until 1972, when Lorenz gave a talk at a meeting of the American Association for the Advancement of Science. The title, suggested by the session organizer, was "Does the Flap of a Butterfly's Wings in Brazil Set Off a Tornado in Texas?" — and the butterfly effect became common vocabulary.

**Mitchell Feigenbaum (1944–2019)** made the next major discovery: universality in the route to chaos. In 1975, studying the logistic map $f_r(x) = rx(1-x)$, Feigenbaum found that as $r$ increases, the map undergoes a cascade of period-doubling bifurcations (period 1 $\to$ 2 $\to$ 4 $\to$ 8 $\to$ $\cdots$) at parameter values $r_1, r_2, r_3, \ldots$, and the ratios $\delta_n = (r_n - r_{n-1})/(r_{n+1}-r_n)$ converge to a universal constant $\delta = 4.669201\ldots$ independent of the specific unimodal map. Using renormalization group ideas from statistical mechanics, Feigenbaum explained why: the period-doubling cascade is governed by a universal fixed point of a renormalization operator acting on the space of unimodal maps.

The Feigenbaum constants were measured experimentally in dripping faucets, Rayleigh-Bénard convection, and electrical circuits, confirming that the universality is not a mathematical artifact but a genuine feature of nonlinear systems in the physical world.

---

## VIII. The Contemporary Picture

The twentieth century closed with differential equations in a state of extraordinary vitality. Poincaré's geometric approach had flowered into the KAM theory of Kolmogorov, Arnold, and Moser (1954–63), which showed that quasi-periodic orbits in nearly integrable Hamiltonian systems are remarkably robust — they persist under small perturbations — creating the intricate web of stability and chaos that characterizes the solar system. The Navier-Stokes equations for fluid flow, the Einstein equations for general relativity, the nonlinear Schrödinger equation for optical fibers and Bose-Einstein condensates — all are PDEs whose mathematical theory continues to advance.

The **regularity theory** for elliptic PDEs, developed by De Giorgi, Nash, Moser, Schauder, and Krylov-Safonov, showed that solutions to wide classes of second-order elliptic equations are smooth in the interior, even if the right-hand side is merely measurable. The **geometric analysis** program — pioneered by Yau, Hamilton, and Perelman — used the Ricci flow (a PDE for the evolution of a Riemannian metric) to prove the Poincaré conjecture and the Geometrization theorem, connecting differential equations to the deepest questions in topology.

The **numerical analysis** of PDEs has advanced in parallel, with the finite element method (based on Sobolev spaces and the Lax-Milgram theorem) becoming the dominant computational tool in engineering, and spectral methods providing exponential convergence for smooth problems. The Fast Fourier Transform algorithm, discovered by Cooley and Tukey in 1965, reduced the computational complexity of the Fourier transform from $O(N^2)$ to $O(N\log N)$, making Fourier-based methods practical for large-scale scientific computing.

The story is not finished. The **Navier-Stokes existence and smoothness problem** — whether smooth solutions to the 3D Navier-Stokes equations can develop singularities in finite time from smooth initial data — remains one of the seven Millennium Prize Problems. The **Riemann Hypothesis** — about the zeros of the analytic continuation of the Dirichlet series $\sum n^{-s}$ — is another. Both are, at their core, questions about differential equations, complex analysis, and the behavior of functions defined by infinite processes.

The student who has worked through this book stands at the threshold of these open questions. The tools are in place: real analysis, linear algebra, ODEs, Fourier analysis, PDEs, complex analysis, dynamical systems, distributions, and functional analysis. What remains is the application of these tools to problems whose answers are still unknown.

---

## Chronological Summary of Major Developments

| Period | Key Figures | Key Contributions |
|--------|-------------|-------------------|
| 1660s–1680s | Newton, Leibniz | Calculus; first ODEs; power series solutions |
| 1690s–1700s | J. Bernoulli, J. Bernoulli | Bernoulli equation; brachistochrone; catenary |
| 1700s–1780s | Euler | Linear ODEs with constant coefficients; numerical method; *Institutiones*; complex exponential |
| 1740s–1800s | d'Alembert, Lagrange | Wave equation; variation of parameters; Lagrangian mechanics |
| 1800s–1830s | Fourier, Cauchy | Heat equation and Fourier series; $\epsilon$-$\delta$ analysis; existence-uniqueness theorem |
| 1820s–1860s | Dirichlet, Riemann | Convergence of Fourier series; Riemann integration; complex analysis |
| 1840s–1890s | Weierstrass, Dedekind, Cantor | Rigorous real analysis; set theory; Weierstrass M-test |
| 1870s–1900s | Poincaré, Lyapunov | Qualitative theory; phase portraits; stability; Poincaré-Bendixson |
| 1890s–1900s | Picard, Lindelöf | Rigorous Picard-Lindelöf theorem |
| 1920s–1940s | Banach, Sobolev | Functional analysis; Sobolev spaces |
| 1940s–1950s | Schwartz, Lax-Milgram | Distribution theory; variational PDE existence |
| 1950s–1960s | Kolmogorov, Arnold, Moser | KAM theory; near-integrable Hamiltonians |
| 1963 | Lorenz | Chaotic attractor; sensitive dependence |
| 1975 | Feigenbaum | Universal constants for period-doubling |
| 1980s–2000s | Perelman | Ricci flow; Poincaré conjecture |
