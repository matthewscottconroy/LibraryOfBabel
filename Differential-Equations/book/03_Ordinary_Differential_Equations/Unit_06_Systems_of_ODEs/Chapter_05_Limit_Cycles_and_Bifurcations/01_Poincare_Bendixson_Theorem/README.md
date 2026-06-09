# The Poincaré-Bendixson Theorem

The Poincaré-Bendixson theorem is one of the central results in the qualitative theory of planar differential equations. It characterizes the possible limit sets of bounded trajectories in the plane, ruling out complex recurrent behavior and showing that bounded trajectories must eventually settle into one of three simple configurations: approaching an equilibrium, approaching a closed orbit, or approaching a graphic (a union of equilibria and heteroclinic/homoclinic orbits). As a corollary, chaos is impossible in autonomous planar systems — one of the deepest and most useful structural facts in two-dimensional dynamics.

## Limit Sets

Let $(x(t), y(t))$ be a solution of the autonomous system $x' = f(x,y)$, $y' = g(x,y)$, defined for $t \geq t_0$. The **$\omega$-limit set** of this trajectory is:

$$\omega(\mathbf{p}) = \bigcap_{t \geq t_0}\overline{\{(x(s), y(s)) : s \geq t\}},$$

where the overline denotes closure. Equivalently, a point $\mathbf{q}$ belongs to $\omega(\mathbf{p})$ if there exists a sequence $t_n \to +\infty$ such that $(x(t_n), y(t_n)) \to \mathbf{q}$. The $\omega$-limit set is the set of accumulation points of the trajectory as $t \to +\infty$.

Properties of $\omega$-limit sets: they are closed, positively invariant (if $\mathbf{q} \in \omega(\mathbf{p})$, the entire orbit through $\mathbf{q}$ lies in $\omega(\mathbf{p})$), and connected (if $\omega(\mathbf{p})$ is compact). If the trajectory is bounded — confined to a compact region — then $\omega(\mathbf{p})$ is nonempty, compact, connected, and consists entirely of entire orbits.

## The Theorem

**Theorem (Poincaré-Bendixson).** Let $f$ and $g$ be continuously differentiable, and let $(x(t), y(t))$ be a solution that remains in a compact (closed and bounded) region $D$ for all $t \geq t_0$. Then the $\omega$-limit set $\omega(\mathbf{p})$ is one of the following:

(a) A single equilibrium point.

(b) A closed orbit (periodic solution).

(c) A **graphic**: a set consisting of finitely many equilibria $\mathbf{q}_1, \ldots, \mathbf{q}_k$ and orbits connecting them, where each orbit tends to some $\mathbf{q}_i$ as $t \to -\infty$ and to some $\mathbf{q}_j$ as $t \to +\infty$. (These are heteroclinic or homoclinic orbits.)

**Corollary.** If the compact region $D$ contains no equilibria and a trajectory remains in $D$ for all $t \geq t_0$, then the trajectory is periodic or approaches a closed orbit.

This corollary is the key tool for proving existence of limit cycles: one constructs a trapping region (annular region bounded by two closed curves, one containing the other) that contains no equilibria and from which trajectories cannot escape. Then the Poincaré-Bendixson theorem guarantees a closed orbit inside.

## Proof Sketch

The proof relies on the Jordan curve theorem (a simple closed curve divides the plane into two regions) and the transversality of trajectories across curves. The key steps:

If $\omega(\mathbf{p})$ contains no equilibria, then every point $\mathbf{q} \in \omega(\mathbf{p})$ lies on an orbit that is also in $\omega(\mathbf{p})$. Consider the orbit through $\mathbf{q}$; it must also have its $\omega$-limit set within $\omega(\mathbf{p})$. A transversal $\sigma$ at $\mathbf{q}$ (a short arc transverse to the vector field) intersects the original trajectory at a monotone sequence of points approaching $\mathbf{q}$ (by continuity and the transversality argument). If two distinct orbits through different points $\mathbf{q}_1, \mathbf{q}_2 \in \omega(\mathbf{p})$ both hit the transversal $\sigma$, the Jordan curve theorem applied to the region bounded by arcs of these orbits and $\sigma$ leads to a contradiction with the uniqueness of trajectories. This forces $\omega(\mathbf{p})$ to consist of a single orbit, which must be periodic (since it is in the limit set and cannot wander to infinity).

## Applying the Theorem: Trapping Regions

To use the theorem to prove a limit cycle exists, one must:

(1) Construct a compact region $R$ (often an annulus $\{r_1 \leq r \leq r_2\}$ in polar coordinates, or a region bounded by two simple closed curves).

(2) Show that $R$ contains no equilibria.

(3) Show that the vector field points strictly inward on $\partial R$: trajectories starting in $R$ cannot leave $R$.

From these three conditions, the Poincaré-Bendixson theorem guarantees a closed orbit in $R$.

**Verification of inward pointing.** Along an outer boundary curve $\partial R_{\text{out}}$, one needs $\dot{V} \leq 0$ for a suitable Lyapunov-like function $V$ that equals $r_2$ on $\partial R_{\text{out}}$; along the inner boundary, one needs $\dot{V} \geq 0$ (pointing outward, into the annulus). Alternatively, if the outer boundary is a circle $r = r_2$ and the inner boundary is a circle $r = r_1$, one computes $\dot{r} = (xx' + yy')/r$ and checks $\dot{r} < 0$ on $r = r_2$ and $\dot{r} > 0$ on $r = r_1$.

## Example: Van der Pol Limit Cycle

The Van der Pol system $x' = y$, $y' = \mu(1-x^2)y - x$ (with $\mu > 0$) has a unique equilibrium at the origin. The linearization at the origin has eigenvalues with positive real part ($\mu/2 \pm i$ for small $\mu$), so the origin is an unstable spiral — trajectories spiral outward from it.

For large $r = \sqrt{x^2+y^2}$, compute $\dot{r}$:

$$r\dot{r} = xx' + yy' = xy + y[\mu(1-x^2)y - x] = \mu(1-x^2)y^2.$$

For $|x| > 1$, $(1-x^2) < 0$, so $\dot{r} < 0$ when $y \neq 0$. More carefully, one can show that for $r$ sufficiently large, trajectories move inward. Taking $r_1$ small (so the inner circle encloses the unstable origin) and $r_2$ large (so the outer circle lies entirely in the region $|x| > 1$ on average, by a more careful integral argument), one constructs an annular trapping region. No equilibria lie in the annulus (only the origin, inside the inner circle). By Poincaré-Bendixson, there is a limit cycle in the annulus.

This argument proves existence but not uniqueness; uniqueness of the Van der Pol limit cycle follows from Liénard's theorem.

## Bendixson's Criterion and Dulac's Theorem

These results give conditions under which closed orbits are absent — they complement Poincaré-Bendixson by identifying when no limit cycles exist.

**Bendixson's criterion:** If $\partial f/\partial x + \partial g/\partial y$ (the divergence of the vector field) does not change sign in a simply connected region $D$, then there are no closed orbits in $D$.

**Proof.** If a closed orbit $C$ existed in $D$, Green's theorem would give $\oint_C (f\,dy - g\,dx) = \iint_{\text{int}(C)}(\partial f/\partial x + \partial g/\partial y)\,dA \neq 0$. But along the orbit, $\mathbf{x}' = (f,g)$, so $f\,dy - g\,dx = f(g\,dt) - g(f\,dt) = 0$. Contradiction.

**Dulac's theorem** extends Bendixson's criterion: if there exists a $C^1$ function $h(x,y)$ such that $\partial(hf)/\partial x + \partial(hg)/\partial y$ does not change sign in $D$, then there are no closed orbits in $D$. Taking $h = 1$ recovers Bendixson's criterion.

## The Impossibility of Chaos in the Plane

A dramatic consequence of the Poincaré-Bendixson theorem is that bounded trajectories of autonomous planar systems cannot exhibit chaotic behavior. Chaos requires sensitive dependence on initial conditions and complex recurrent dynamics — but these are impossible if every bounded trajectory either approaches an equilibrium or a closed orbit. The simplest (in terms of state space dimension) chaotic systems must live in at least three dimensions, as the Lorenz system demonstrates.

This is one of the rare cases where a rigorous theorem of mathematics directly constrains what physical phenomena can occur in low-dimensional systems — a satisfying example of mathematics imposing structure on the physical world.
