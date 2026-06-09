# The Maximum Principle for the Heat Equation

The maximum principle is one of the most powerful and elegant results in the theory of parabolic equations. It asserts that a solution of the heat equation cannot attain its maximum or minimum in the interior of the space-time domain at a later time — the extrema are always achieved at $t=0$ or on the spatial boundary. This single result implies uniqueness, continuous dependence on data, comparison of solutions, and qualitative bounds.

## The Parabolic Domain

Let $\Omega \subset \mathbb{R}^n$ be a bounded open set and $T > 0$. The **parabolic domain** is $Q_T = \Omega \times (0,T]$. The **parabolic boundary** is $\partial_p Q_T = (\Omega\times\{0\}) \cup (\partial\Omega\times[0,T])$ — the bottom and lateral sides of the space-time cylinder, but not the top $\Omega\times\{T\}$. The parabolic boundary is where initial and boundary data are prescribed.

## The Weak Maximum Principle

**Theorem (Weak Maximum Principle).** Let $u \in C^{2,1}(Q_T) \cap C(\overline{Q}_T)$ satisfy $u_t - \kappa\Delta u \leq 0$ (a supersolution) in $Q_T$. Then

$$\max_{\overline{Q}_T} u = \max_{\partial_p Q_T} u.$$

In particular, if $u$ is a solution of the heat equation ($u_t = \kappa\Delta u$), then its maximum over $\overline{Q}_T$ is attained on the parabolic boundary.

**Proof.** Suppose the maximum is attained at an interior point $(x_0,t_0) \in Q_T$ with $t_0 > 0$, i.e., $u(x_0,t_0) = M > \max_{\partial_p Q_T} u$. At an interior maximum:
- $\nabla u(x_0,t_0) = 0$ (spatial gradient vanishes).
- $\Delta u(x_0,t_0) \leq 0$ (Laplacian at a spatial maximum is nonpositive — the second derivative test).
- $u_t(x_0,t_0) \geq 0$ (since $t_0$ is the first time the maximum is attained, $u$ could not have been decreasing there).

But then $u_t - \kappa\Delta u \geq 0$ at $(x_0,t_0)$, contradicting $u_t - \kappa\Delta u \leq 0$.

A subtle issue: what if the maximum is attained at $t_0 = T$? Then $u_t \geq 0$ need not hold. A small perturbation $u_\varepsilon = u - \varepsilon t$ satisfies $\partial_t u_\varepsilon - \kappa\Delta u_\varepsilon = -\varepsilon < 0$ and cannot attain its max at an interior point; send $\varepsilon\to 0$.

## The Strong Maximum Principle

**Theorem (Strong Maximum Principle).** Under the same assumptions, if $u$ attains its maximum $M$ at an interior point $(x_0,t_0) \in Q_T$, then $u \equiv M$ in $Q_{t_0} = \Omega\times(0,t_0]$.

In other words, if the maximum is attained inside the cylinder, the solution is identically equal to that maximum everywhere before (and at) that time. This is a much stronger conclusion: a nonconstant solution cannot attain its interior maximum at any time in $(0,T]$.

The strong maximum principle for the heat equation is proved by studying the propagation of the maximum backward in time along any connected path and using the properties of the heat kernel.

## Consequences

**Uniqueness.** Suppose $u_1$ and $u_2$ solve the heat equation with the same initial and boundary data. Let $w = u_1 - u_2$, so $w_t = \kappa\Delta w$ with $w = 0$ on $\partial_p Q_T$. The weak maximum principle gives $\max w \leq 0$ and $\min w = -\max(-w) \geq 0$, hence $w \equiv 0$.

**Comparison principle.** If $u_1 \leq u_2$ on the parabolic boundary, then $u_1 \leq u_2$ throughout $\overline{Q}_T$. This follows by applying the maximum principle to $w = u_1 - u_2$.

**A priori bounds.** The solution satisfies $\min_{\partial_p Q_T} u \leq u(x,t) \leq \max_{\partial_p Q_T} u$ for all $(x,t) \in \overline{Q}_T$. In particular, for Dirichlet conditions $u=g$ on $\partial\Omega$ and initial data $u(x,0)=f(x)$:

$$\min(\min_\Omega f, \min_{\partial\Omega\times[0,T]} g) \leq u(x,t) \leq \max(\max_\Omega f, \max_{\partial\Omega\times[0,T]} g).$$

**Continuous dependence.** If $f_1 \leq f_2$ pointwise on the parabolic boundary, then $u_1 \leq u_2$ everywhere. Small perturbations in boundary and initial data produce correspondingly small perturbations in the solution.

## Maximum Principle on $\mathbb{R}$

For the heat equation on all of $\mathbb{R}$, the maximum principle requires a growth condition to avoid Tychonoff counterexamples (nonunique solutions that grow faster than $e^{x^2}$):

**Theorem.** Suppose $u$ solves $u_t = \kappa u_{xx}$ in $\mathbb{R}\times(0,T]$ and satisfies the growth condition $|u(x,t)| \leq Me^{ax^2}$ for some constants $M, a > 0$ with $a < 1/(4\kappa T)$. If $u(x,0) \leq 0$ for all $x$, then $u(x,t) \leq 0$ for all $x \in \mathbb{R}$, $0 \leq t \leq T$.

The growth condition cannot be omitted: Tychonoff (1935) constructed a nontrivial solution of the heat equation on $\mathbb{R}$ with zero initial data that grows faster than any $e^{ax^2}$, showing that the Cauchy problem is not uniquely solvable without growth restrictions.

## Physical Interpretation

The maximum principle has a clear physical meaning for heat conduction: in a body with no internal heat sources, the hottest temperature is always achieved either at the initial moment or on the boundary (where external heat may be applied). Heat can only flow from hot to cold; an interior maximum would require heat to flow inward from cooler surroundings, violating the direction of Fourier's law.

Similarly, the minimum (coldest temperature) must be on the parabolic boundary: an interior cold spot would have heat flowing in from all sides, warming it up instantly.

## Application: The Mean Value Property for the Heat Equation

There is an analogue of the mean value property for the heat equation (cf. the mean value property for harmonic functions). If $u$ solves $u_t = \kappa u_{xx}$ in a neighborhood of $(x_0,t_0)$, then

$$u(x_0,t_0) = \frac{1}{4\kappa r^2}\iint_{E(x_0,t_0,r)} u(x,t)\,\frac{(x-x_0)^2}{(t_0-t)^2}\,dx\,dt,$$

where $E(x_0,t_0,r) = \{(x,t): K(x-x_0,t_0-t) \geq (4\pi\kappa r^2)^{-1}\}$ is a "heat ball" — a region whose boundary is a level set of the heat kernel. This parabolic mean value property is the direct analogue of the spherical mean value property for harmonic functions and is used to prove the strong maximum principle via a path-connectivity argument.
