# 11.3 Strange Attractors

The Lorenz attractor is the archetypal example of a phenomenon that wasn't supposed to exist: a set that attracts all nearby orbits, but is neither a point nor a periodic orbit nor a torus — something genuinely fractal, geometrically wild, dynamically chaotic. Let's formalize what we mean by "attractor" and "strange," and then meet another key example.

## Attractors

An attractor should be a set that orbits are drawn to. But we want to avoid including spurious transient structure. The right definition focuses on a whole neighborhood converging to the set, not just nearby orbits.

**Definition 11.3.1.** An *attractor* of a dynamical system is a compact invariant set $\Lambda$ such that some open neighborhood $U \supseteq \Lambda$ has
$$\bigcap_{t \geq 0} \Phi_t(U) = \Lambda$$
(all orbits starting in $U$ converge to $\Lambda$ and remain there asymptotically).

An attractor is *strange* if it is fractal (non-integer Hausdorff dimension) and the dynamics on it has sensitive dependence on initial conditions.

What makes strange attractors counterintuitive is the combination of two competing pressures: the system is dissipative (volumes shrink, so the attractor has zero volume), but the dynamics on the attractor is expanding (nearby orbits diverge). These cannot both happen in all directions simultaneously. The resolution is that the attractor is a fractal: stretched and folded in an intricate, self-similar way. It is thin in some directions and complicated in others.

## The Hénon Map

The Lorenz attractor lives in three dimensions and is hard to visualize completely. The Hénon map provides a two-dimensional example where the geometry is more accessible.

**Example 11.3.2 (Hénon Map).** The map $H_{a,b}: \mathbb{R}^2 \to \mathbb{R}^2$ defined by
$$H_{a,b}(x,y) = (1 - ax^2 + y, \ bx)$$
for $a = 1.4$, $b = 0.3$, has a strange attractor in $\mathbb{R}^2$.

The attractor has a characteristic fractal structure: if you zoom into any piece of it, you see a pattern of parallel curves that looks remarkably similar to the whole. This self-similar structure is a signature of the horseshoe-like dynamics underlying it — the map stretches in one direction and folds in another, repeatedly, producing the fractal layers.

The parameter $b$ controls the dissipation: $|\det DH_{a,b}| = |b| < 1$, so the map contracts area by a factor of $|b|$ at each step. The Hénon attractor has Hausdorff dimension approximately $1.26$ — not 1 (a curve) and not 2 (a region), but something in between.

For a long time, the Hénon attractor was known numerically but proving its existence rigorously was elusive. The breakthrough came from Benedicks and Carleson:

**Theorem 11.3.3 (Benedicks-Carleson, 1991).** For Lebesgue-a.e. $b$ close to $0$ and for $a$ in a positive measure set near $a = 2$, the Hénon map has a strange attractor with a unique SRB measure.

What this is saying is: the parameters where a genuine strange attractor exists form a set of positive measure in parameter space. This is not a codimension-zero condition — it is not *all* parameters near $(2, 0)$, but a "fat" set in the measure-theoretic sense. The existence of an SRB measure means there is a natural, physically meaningful probability measure on the attractor that describes the statistics of typical orbits.

SRB measures (Sinai-Ruelle-Bowen measures) are the "right" invariant measures for chaotic attractors: they are the measures you observe if you start a typical initial condition and watch the long-run statistics of orbits. They generalize the role that Liouville measure plays in Hamiltonian systems. We will return to them in Chapter 9 and again in Chapter 23.

The Hénon map — and the analysis by Benedicks and Carleson — will reappear in Section 11.4, where we compute its fractal dimension using the Lyapunov exponents.
