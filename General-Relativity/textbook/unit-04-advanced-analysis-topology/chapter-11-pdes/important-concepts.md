# Chapter 11: Important Concepts

---

**Partial Differential Equation (PDE)**: An equation relating a function of several variables to its partial derivatives. The "degree" (order) is the highest derivative present; "linear" means linear in u and its derivatives (no u², u·u_x, etc.); "quasilinear" means linear in the highest-order derivatives only.

**Classification (Elliptic/Parabolic/Hyperbolic)**: For the second-order equation Au_{xx} + 2Bu_{xy} + Cu_{yy} + ... = 0, the type is determined by B²−AC: negative (elliptic), zero (parabolic), positive (hyperbolic). Elliptic: equilibrium problems; parabolic: diffusion/evolution; hyperbolic: waves/propagation.

**Well-Posedness (Hadamard)**: A PDE problem is well-posed if it has existence (a solution exists), uniqueness (only one solution), and stability (continuous dependence on initial/boundary data). The minimal requirement for a physical model.

**Characteristics**: Curves (in 2D) or surfaces (in higher dimensions) along which information propagates in a hyperbolic PDE. For the wave equation, the characteristics are the light cone directions x ± ct = const. The PDE "degenerates" along characteristics: it cannot be solved for the highest-order normal derivative there.

**Domain of Dependence**: The set of points on the initial hypersurface whose values influence the solution at a given spacetime point. For the wave equation with speed c, u(x, t) depends only on initial data in the ball |y − x| ≤ ct. This is the PDE analogue of the past light cone.

**Domain of Influence**: The set of spacetime points whose solution is influenced by given initial data. For the wave equation, initial data at x₀ influences u(x, t) only for |x − x₀| ≤ ct. This is the PDE analogue of the future light cone. The finite domain of influence is the PDE expression of causality.

**d'Alembert's Solution**: u(x, t) = f(x − ct) + g(x + ct) — the general solution of the 1D wave equation as superposition of right- and left-traveling waves. The explicit formula with Cauchy data is u(x,t) = [u₀(x−ct) + u₀(x+ct)]/2 + (1/2c)∫u₁.

**Huygens' Principle**: In odd spatial dimensions ≥ 3, the solution of the wave equation at (x₀, t₀) depends only on the initial data on the sphere |y−x₀| = ct₀ (not in its interior). In even dimensions, the solution depends on all initial data in the ball — there are "tails." Huygens' principle in 3D: light has no afterglow.

**Heat Kernel**: The fundamental solution K(x, t) = (4πκt)^{−n/2}exp(−|x|²/4κt) of the heat equation — the temperature distribution at time t due to a point heat source at the origin at t = 0. A Gaussian that broadens over time (width σ ∝ √(κt)). The solution to any initial data is the convolution of the heat kernel with the initial data.

**Maximum Principle**: For the heat equation: the maximum of a solution on [0,L]×[0,T] is attained on the parabolic boundary (the initial slice or the spatial boundaries). For the Laplace equation: the maximum of a harmonic function on a bounded domain is attained on the boundary. These principles are fundamental tools for uniqueness and comparison arguments.

**Smoothing Property (Heat Equation)**: Even discontinuous or distributional initial data produce smooth solutions for t > 0. The heat equation instantly regularizes any initial data. This is in contrast to the wave equation, which propagates discontinuities and singularities along characteristics.

**Green's Function G(x, y)**: The fundamental solution of −∇²G = δ(x−y) for an operator on a domain Ω, satisfying zero boundary conditions. Once G is found, the solution to any boundary value problem is an integral: u(x) = ∫G(x,y)f(y)dy + boundary terms. G is the "impulse response" of the elliptic operator.

**Fundamental Solution of the Laplacian**: G(x) = 1/(4π|x|) in ℝ³, satisfying −∇²G = δ³(x). This is the Newtonian gravitational potential of a point mass: Φ = −GM/(4πr) satisfies ∇²Φ = 4πGMδ³.

**Retarded Green's Function**: For the wave operator □, G_R(x, y) = c/(4π) · δ(t_x − t_y − |x−y|/c)/|x−y| — the solution with retarded boundary condition (no signal before the source). The gravitational wave field of a source T_{μν} is the convolution of G_R with T_{μν}.

**Distribution (Generalized Function)**: A continuous linear functional on the space of smooth compactly supported test functions. Dirac's delta δ, its derivatives, and the principal value P.V.(1/x) are distributions. Every distribution has a well-defined derivative (via integration by parts). Distributions provide the rigorous foundation for Green's functions.

**Fourier Transform**: $\hat{f}(k) = \int f(x) e^{-ik\cdot x} dx$. Converts differentiation to multiplication (∂_x → ik_x). Diagonalizes the Laplacian and wave operator: $\widehat{\nabla^2 f}(k) = -|k|^2 \hat{f}(k)$. Used to find dispersion relations: substitute a plane wave e^{i(k·x − ωt)} and find the relationship ω = ω(k).

**Dispersion Relation**: The relationship ω = ω(k) between frequency and wavenumber for plane wave solutions. For the wave equation: ω = ck (linear, non-dispersive). For the Klein-Gordon equation: ω² = c²k² + m² (dispersive, massive). For gravitational waves in GR: ω = ck (massless, non-dispersive) — confirmed by LIGO observations.

**Cauchy Problem**: An initial value problem specifying a function and its normal derivative on a hypersurface. The natural well-posed formulation for hyperbolic equations (wave equation). Ill-posed for elliptic equations.

**Ricci Flow**: Hamilton's equation ∂g_{μν}/∂t = −2R_{μν} for a family of Riemannian metrics. The "heat equation for the metric." Ricci flow uniformizes curvature over time. Used by Perelman to prove the Poincaré conjecture (2003) by showing that Ricci flow with surgery on a 3-manifold converges to a canonical metric.

**Sobolev Space H^k(Ω)**: The completion of smooth functions in the norm ||u||^2_{H^k} = Σ_{|α|≤k} ∫|∂^α u|²dx. Functions in H^k have weak derivatives up to order k in L². The natural function space for solutions to elliptic PDEs (Sobolev embedding theorems control continuity). The initial data for the Einstein equations lives in Sobolev spaces.

**Seeley-DeWitt Heat Kernel Coefficients**: The coefficients a_n(x) in the small-t expansion K(x,x;t) ~ (4πt)^{-n/2}(a_0 + a_1 t + a_2 t² + ...) of the heat kernel on a curved manifold. These coefficients are polynomials in the curvature and its covariant derivatives. They appear in the one-loop effective action in quantum field theory in curved spacetime and in the calculation of Hawking radiation.
