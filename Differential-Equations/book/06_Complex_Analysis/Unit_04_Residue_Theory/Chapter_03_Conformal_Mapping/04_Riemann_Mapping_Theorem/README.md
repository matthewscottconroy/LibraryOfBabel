# The Riemann Mapping Theorem

The Riemann Mapping Theorem is one of the deepest and most beautiful results in complex analysis. It asserts that all simply connected proper open subsets of $\mathbb{C}$ are conformally equivalent to one another — and in particular, to the open unit disk $\mathbb{D}$. This is a stunning uniformity result: the upper half-plane, the interior of any triangle, any simply connected domain bounded by a smooth curve, an infinite strip — all are "the same" from the point of view of complex analysis. The theorem is purely an existence result and gives no formula for the mapping; constructing the mapping explicitly (as in the Schwarz-Christoffel theory) requires additional work.

## Statement

**Theorem (Riemann Mapping Theorem).** Let $D$ be a simply connected open subset of $\mathbb{C}$ with $D \neq \mathbb{C}$. Then there exists a conformal bijection $f : D \to \mathbb{D}$, where $\mathbb{D} = \{z : |z| < 1\}$ is the open unit disk.

Moreover, given any point $z_0 \in D$, there exists a unique such map with $f(z_0) = 0$ and $f'(z_0) > 0$ (i.e., the derivative at $z_0$ is real and positive).

## Why $\mathbb{C}$ Itself Is Excluded

The theorem requires $D \neq \mathbb{C}$. Indeed, there is no conformal bijection from $\mathbb{C}$ to $\mathbb{D}$: any analytic function $f : \mathbb{C} \to \mathbb{D}$ would be entire and bounded, hence constant by Liouville's theorem. The class of simply connected domains conformally equivalent to $\mathbb{C}$ consists of $\mathbb{C}$ itself (and, on the Riemann sphere, $\hat{\mathbb{C}}$). This is the uniformization theorem for genus-$0$ surfaces, of which the Riemann mapping theorem is the planar case.

## Proof Sketch

The standard proof, due to Koebe (1912), constructs the map as the maximizer of a certain functional over a family of analytic functions.

**Step 1: The family $\mathcal{F}$.** Fix $z_0 \in D$ and consider the family
$$\mathcal{F} = \{f : D \to \mathbb{D} \mid f \text{ analytic and injective}, f(z_0) = 0\}.$$
Since $D \neq \mathbb{C}$, there exists $a \notin D$; the function $1/(z - a)$ is bounded on $D$, and via a composition with a square root and a Mobius transformation, one shows $\mathcal{F} \neq \emptyset$.

**Step 2: Maximizing $|f'(z_0)|$.** By Cauchy's inequalities, the set $\{|f'(z_0)| : f \in \mathcal{F}\}$ is bounded above. Let $M = \sup_{f \in \mathcal{F}} |f'(z_0)|$.

**Step 3: The supremum is attained.** By Montel's theorem (the family $\mathcal{F}$, being uniformly bounded by $1$, is normal), a subsequence $f_n \to f^*$ converges uniformly on compact subsets. By Hurwitz's theorem, the limit $f^*$ is either constant or injective. Since $|{f^*}'(z_0)| = M > 0$, $f^*$ is nonconstant and hence injective, with $f^* \in \mathcal{F}$ and $|{f^*}'(z_0)| = M$.

**Step 4: The extremal map is surjective.** Suppose $f^*$ is not surjective: $w_0 \in \mathbb{D} \setminus f^*(D)$. Then the Mobius transformation $\phi(w) = (w - w_0)/(1 - \bar{w}_0 w)$ has no zero in $f^*(D)$, so $\sqrt{\phi(f^*(z))}$ is a well-defined analytic function on $D$. Composing with another Mobius transformation (to fix $z_0$ and normalize the derivative) yields $g \in \mathcal{F}$ with $|g'(z_0)| > |{f^*}'(z_0)| = M$, contradicting the maximality of $M$. Hence $f^*$ is surjective. $\square$

## Uniqueness

Given the normalization $f(z_0) = 0$ and $f'(z_0) > 0$, the map is unique. If $f$ and $g$ are both conformal maps $D \to \mathbb{D}$ with these normalizations, then $h = g \circ f^{-1} : \mathbb{D} \to \mathbb{D}$ is a conformal automorphism with $h(0) = 0$ and $h'(0) > 0$. By the Schwarz lemma, $h(z) = e^{i\theta}z$ with $e^{i\theta} > 0$, so $h = \mathrm{id}$, giving $f = g$.

## Consequences and Extensions

**The Riemann surface picture.** Every compact Riemann surface of genus $0$ is conformally equivalent to $\hat{\mathbb{C}}$, every compact surface of genus $1$ to a flat torus $\mathbb{C}/\Lambda$ (an elliptic curve), and every compact surface of genus $g \geq 2$ to a quotient of the upper half-plane by a Fuchsian group. This is the uniformization theorem.

**Boundary behavior.** The Riemann mapping theorem says nothing about what happens on the boundary. The Caratheodory extension theorem asserts that if $D$ has a Jordan curve boundary (a simple closed curve), then the conformal map $f : D \to \mathbb{D}$ extends to a homeomorphism $\bar{D} \to \bar{\mathbb{D}}$. The boundary extension can be very complicated in general.

**Riemann maps and harmonic measure.** The Riemann map $f : D \to \mathbb{D}$ transforms harmonic measure on $\partial\mathbb{D}$ (the standard arc length measure, divided by $2\pi$) to harmonic measure on $\partial D$ from the point $z_0 = f^{-1}(0)$. This connection between conformal maps and harmonic measure is fundamental in potential theory.

## The Unit Disk as Universal Domain

The Riemann mapping theorem shows that the unit disk is the universal model for simply connected domains. Much of complex function theory on simply connected domains can be reduced to the unit disk:

- The Poisson integral formula gives the harmonic function in $\mathbb{D}$ with prescribed boundary values, and via Riemann mapping this solves the Dirichlet problem on any simply connected domain.
- The Hardy spaces $H^p(\mathbb{D})$ of analytic functions on the disk, and their boundary behavior, serve as a model for the theory of analytic functions on general simply connected domains.
- Extremal problems (finding the analytic function with maximum or minimum of some quantity) can often be reduced to the disk by conformal mapping.

## Contrast with Higher Dimensions

The Riemann mapping theorem has no analogue in $\mathbb{C}^n$ for $n \geq 2$. In several complex variables, the ball $\{z \in \mathbb{C}^n : |z| < 1\}$ and the polydisk $\{z \in \mathbb{C}^n : |z_k| < 1 \text{ for all } k\}$ are both simply connected but are not biholomorphically equivalent. The classification of domains in $\mathbb{C}^n$ is far more subtle and remains an active area of research.
