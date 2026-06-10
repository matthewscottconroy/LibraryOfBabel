# Fibrations

## The Homotopy Lifting Property

A *fibration* is a map with the *homotopy lifting property*: given any homotopy in the base and any partial lift starting in the total space, you can complete the lift.

**Definition (Hurewicz Fibration).** A continuous map $p : E \to B$ is a *Hurewicz fibration* if for every space $X$, every homotopy $H : X \times [0,1] \to B$, and every map $\tilde{h}_0 : X \to E$ with $p \circ \tilde{h}_0 = H|_{t=0}$, there exists a homotopy $\tilde{H} : X \times [0,1] \to E$ with $p \circ \tilde{H} = H$ and $\tilde{H}|_{t=0} = \tilde{h}_0$.

**Definition (Serre Fibration).** A map $p : E \to B$ is a *Serre fibration* if the lifting property holds for $X = [0,1]^n$ (cubes) for all $n \geq 0$.

Hurewicz fibrations are the more classical notion; Serre fibrations are weaker but sufficient for computing homotopy groups. Every Hurewicz fibration is a Serre fibration. Every covering map is a Hurewicz fibration.

The key property: for any Serre fibration $p : E \to B$, any path $\gamma : [0,1] \to B$, and any $e_0 \in p^{-1}(\gamma(0))$, the path lifts to a path in $E$ starting at $e_0$. This is the path-lifting property — the same as for covering spaces, but now the fiber $p^{-1}(b)$ can be an arbitrary space, not just a discrete set.

## Fiber Bundles

A *fiber bundle* with fiber $F$ is a map $p : E \to B$ such that every point $b \in B$ has a neighborhood $U$ with $p^{-1}(U) \cong U \times F$ (homeomorphism compatible with $p$). Every fiber bundle is a Serre fibration.

Key examples:
- The trivial bundle: $B \times F \to B$ (projection).
- The Möbius band: a non-trivial bundle of intervals over $S^1$.
- The tangent bundle of a manifold: a vector bundle over the manifold.
- The Hopf fibration: $S^1 \to S^3 \to S^2$ — a non-trivial bundle of circles over $S^2$.

## The Long Exact Sequence of a Fibration

This is the most powerful computational tool in homotopy theory.

**Theorem.** Let $p : E \to B$ be a Serre fibration with fiber $F = p^{-1}(b_0)$ over the basepoint $b_0 \in B$. Choose a basepoint $e_0 \in F$. There is a long exact sequence of homotopy groups:
$$\cdots \to \pi_n(F, e_0) \xrightarrow{i_*} \pi_n(E, e_0) \xrightarrow{p_*} \pi_n(B, b_0) \xrightarrow{\partial} \pi_{n-1}(F, e_0) \to \cdots \to \pi_0(B, b_0)$$

The boundary map $\partial : \pi_n(B, b_0) \to \pi_{n-1}(F, e_0)$ is defined as follows: given a map $f : S^n \to B$, lift the nullhomotopy of $f$ (which exists in principle by lifting to $E$) to get a map $S^{n-1} \to F$; this is the boundary map.

**Why it's exact:** At each group in the sequence, the image of the incoming map equals the kernel of the outgoing map. The exactness encodes the fact that "what kills the boundary" in the fiber corresponds to "what can be lifted" in the total space.

## The Path-Loop Fibration

The fundamental example of a fibration is the *path-loop fibration*:
$$\Omega(X, x_0) \hookrightarrow P(X, x_0) \xrightarrow{\text{ev}_1} X$$

where $P(X, x_0) = \{\gamma : [0,1] \to X : \gamma(0) = x_0\}$ is the space of paths starting at $x_0$, and $\text{ev}_1(\gamma) = \gamma(1)$ is evaluation at the endpoint.

The fiber over $x_0$ is $\{\gamma \in P(X, x_0) : \gamma(1) = x_0\} = \Omega(X, x_0)$, the loop space.

The path space $P(X, x_0)$ is contractible: every path $\gamma$ can be contracted to the constant path at $x_0$ via $H(\gamma, t)(s) = \gamma(ts)$ (shortening the path). So $\pi_n(P(X, x_0)) = 0$ for all $n$.

The long exact sequence of the path-loop fibration gives:
$$\cdots \to \pi_n(P(X)) \to \pi_n(X) \xrightarrow{\partial} \pi_{n-1}(\Omega X) \to \pi_{n-1}(P(X)) \to \cdots$$

Since $P(X)$ is contractible, $\pi_n(P(X)) = 0$ for all $n$. The sequence collapses to:
$$0 \to \pi_n(X) \xrightarrow{\partial} \pi_{n-1}(\Omega X) \to 0$$

giving an isomorphism $\pi_n(X) \cong \pi_{n-1}(\Omega X)$. This is the fundamental relation between the homotopy groups of a space and those of its loop space.

## The Hopf Fibration

The *Hopf fibration* is the most beautiful fibration in mathematics:
$$S^1 \hookrightarrow S^3 \xrightarrow{p} S^2$$

**Construction.** View $S^3 = \{(z_1, z_2) \in \mathbb{C}^2 : |z_1|^2 + |z_2|^2 = 1\}$ and $S^2 = \mathbb{CP}^1$ (the Riemann sphere). Define $p(z_1, z_2) = [z_1 : z_2]$ (complex projective coordinates). 

The fiber over $[z_1 : z_2]$ is $\{(e^{i\theta} z_1, e^{i\theta} z_2) : \theta \in [0, 2\pi)\} \cong S^1$: all unit complex multiples of $(z_1, z_2)$, forming a circle.

This is a fiber bundle with fiber $S^1$ and base $S^2$. The total space is $S^3$. The bundle is non-trivial — $S^3$ is not homeomorphic to $S^1 \times S^2$ (they have different homotopy groups).

**The computation $\pi_3(S^2) = \mathbb{Z}$.** Apply the long exact sequence to the Hopf fibration $S^1 \to S^3 \to S^2$:

$$\cdots \to \pi_3(S^1) \to \pi_3(S^3) \to \pi_3(S^2) \xrightarrow{\partial} \pi_2(S^1) \to \pi_2(S^3) \to \pi_2(S^2) \xrightarrow{\partial} \pi_1(S^1) \to \pi_1(S^3) \to \cdots$$

We know:
- $\pi_k(S^1) = 0$ for $k \geq 2$ (covering space theory: the universal cover $\mathbb{R}$ is contractible).
- $\pi_3(S^3) = \mathbb{Z}$ (the identity map is the generator).
- $\pi_2(S^3) = 0$ (from cellular approximation: any map $S^2 \to S^3$ is homotopic to a map into the 2-skeleton of $S^3$; since $S^3$ has a CW structure with one 0-cell and one 3-cell, the 2-skeleton is a point).
- $\pi_2(S^2) = \mathbb{Z}$ (Hurewicz: $S^2$ is 1-connected and $H_2(S^2) = \mathbb{Z}$).
- $\pi_1(S^1) = \mathbb{Z}$.
- $\pi_1(S^3) = 0$ (3-sphere is simply connected).

The relevant segment of the long exact sequence is:
$$0 = \pi_3(S^1) \to \pi_3(S^3) \to \pi_3(S^2) \xrightarrow{\partial} \pi_2(S^1) = 0$$

So $0 \to \mathbb{Z} \to \pi_3(S^2) \to 0$ is exact, giving $\pi_3(S^2) \cong \mathbb{Z}$.

The generator of $\pi_3(S^2)$ is the Hopf fibration $p : S^3 \to S^2$ itself, viewed as a map from the 3-sphere to the 2-sphere.

This computation is remarkable: a 2-dimensional sphere has a non-trivial 3-dimensional homotopy group. This was Hopf's discovery (1931), and it shocked the mathematical world, which had assumed $\pi_k(S^n) = 0$ for $k > n$. The Hopf fibration inaugurated the modern study of higher homotopy groups and showed that the homotopy theory of spheres is rich and surprising.

## Fibrations in HoTT as Dependent Types

In HoTT, a fibration $p : E \to B$ corresponds to a *dependent type family* $P : B \to \mathcal{U}$. The total space is $\sum_{b:B} P(b)$; the projection $\pi_1 : \sum_{b:B} P(b) \to B$ is the fibration map; the fiber over $b : B$ is $P(b)$.

The homotopy lifting property corresponds to the *dependent elimination rule* for $\Sigma$-types: to define a function out of $\sum_{b:B} P(b)$, it suffices to define it on pairs $(b, p)$ where $b : B$ and $p : P(b)$.

The long exact sequence of a fibration corresponds, in HoTT, to the fiber sequence:
$$\Omega B \to F \to E \to B$$
where $F = P(b_0)$ is the fiber. The homotopy groups of this sequence are computed via the encode-decode method or the Freudenthal suspension theorem.

The Hopf fibration exists in HoTT as a map $H : S^3 \to S^2$, constructed using the multiplication of unit quaternions (since $S^3$ is homeomorphic to the unit quaternions $Sp(1)$). The computation $\pi_3(S^2) = \mathbb{Z}$ in HoTT is a theorem proved by Brunerie (2016), and its proof uses the Hopf fibration in the synthetic setting.

## Summary Table

| Fibration | Fiber | Base | Total Space | Key Consequence |
|---|---|---|---|---|
| Path-loop | $\Omega X$ | $X$ | $PX \simeq *$ | $\pi_n(X) \cong \pi_{n-1}(\Omega X)$ |
| Hopf | $S^1$ | $S^2$ | $S^3$ | $\pi_3(S^2) = \mathbb{Z}$ |
| Universal cover | $\pi_1(X)$ (discrete) | $X$ | $\tilde{X}$ (simply connected) | Classification of covering spaces |
| Vector bundle | $\mathbb{R}^n$ | $M$ | $TM$ | Characteristic classes |
