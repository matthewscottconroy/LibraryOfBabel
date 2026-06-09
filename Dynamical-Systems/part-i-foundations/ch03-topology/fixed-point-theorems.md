# 3.7 Fixed-Point Theorems

Fixed points are where dynamics stops — or rather, where dynamics looks static. But fixed-point theorems say something deeper: they say that under certain conditions, a map *must* have a fixed point, regardless of the specific dynamics. The topology of the space forces it.

## 3.7.1 Brouwer's Theorem

The most classical result:

**Theorem 3.7.1 (Brouwer Fixed Point Theorem).** Every continuous map $f: D^n \to D^n$ (the closed $n$-disk) has a fixed point.

*(proof sketch, $n=2$)* Suppose $f$ has no fixed point. Define $g(x) = $ the point on $\partial D^2$ where the ray from $f(x)$ through $x$ exits the disk. Then $g: D^2 \to \partial D^2 = S^1$ is a continuous retraction of the disk onto its boundary with $g|_{S^1} = \text{id}$. But $H_2(D^2) = 0$ while $H_1(S^1) = \mathbb{Z}$, and a retraction would give a contradiction on homology.

The proof by contradiction is illuminating. If there's no fixed point, you can "project" every interior point radially toward the boundary — but this would give a retraction of the disk onto its boundary. And no such retraction can exist: the disk is simply connected (can be contracted to a point), but the circle is not. The topology of $D^2$ and $S^1$ are different enough to make this impossible.

**Corollary 3.7.2.** Every continuous map from a convex compact subset of $\mathbb{R}^n$ to itself has a fixed point.

This follows because any convex compact set is homeomorphic to a disk. The corollary is what's actually used in most applications.

## 3.7.2 Schauder Fixed Point Theorem

For infinite-dimensional spaces, Brouwer's theorem no longer applies directly (its proof is intrinsically finite-dimensional). Schauder's theorem is the right generalization:

**Theorem 3.7.3 (Schauder).** Let $K$ be a compact convex subset of a Banach space, and $f: K \to K$ continuous. Then $f$ has a fixed point.

**Application.** Many proofs of existence of invariant objects in dynamics — equilibria, periodic orbits, invariant measures, invariant functions — reduce to applying Brouwer or Schauder. The setup is always the same: identify the right space, construct a map that sends that space to itself, and invoke the fixed-point theorem.

## 3.7.3 Poincaré-Hopf and Lefschetz Theorems

These two theorems count fixed points (or zeros) of maps and vector fields using algebraic topology. They're not just existence results — they give quantitative information.

**Theorem 3.7.4 (Poincaré-Hopf).** Let $M$ be a compact smooth manifold and $V$ a smooth vector field with finitely many zeros. Then
$$\sum_{V(p)=0} \text{index}(V, p) = \chi(M),$$
where $\chi(M) = \sum_k (-1)^k \text{rank}(H^k(M))$ is the Euler characteristic.

The *index* of an isolated zero is an integer that measures how the vector field rotates around that zero: if the field rotates once counterclockwise around the zero (like a source or sink), the index is $+1$; if it rotates clockwise (a saddle), the index is $-1$.

What this is really saying: the sum of indices of all zeros must equal the Euler characteristic. The topology of the manifold forces the "total rotation" of any smooth vector field. You can move the zeros around, merge them, split them — but the sum of indices is fixed.

**Corollary 3.7.5.** Every vector field on $S^{2n}$ has a zero. (Since $\chi(S^{2n}) = 2 \neq 0$.) But on $S^{2n+1}$ and $\mathbb{T}^n$ (where $\chi = 0$), nonzero vector fields can exist.

This corollary is the celebrated "Hairy Ball Theorem": you can't comb a hairy ball flat. Any continuous vector field on $S^2$ must have at least one zero. On the torus, in contrast, you can have a vector field that's everywhere nonzero (just point constantly in one direction).

**Theorem 3.7.6 (Lefschetz Fixed Point Theorem).** Let $f: M \to M$ be a continuous map on a compact manifold. The *Lefschetz number*
$$L(f) = \sum_k (-1)^k \text{tr}(f_*: H^k(M; \mathbb{Q}) \to H^k(M; \mathbb{Q}))$$
satisfies: if $L(f) \neq 0$, then $f$ has a fixed point.

The Lefschetz number counts fixed points algebraically. For the identity map, $L(\text{id}) = \chi(M)$ — so the Lefschetz theorem implies that every map homotopic to a map with $L \neq 0$ has a fixed point.

Here's a concrete dynamical application. For a diffeomorphism $f: \mathbb{T}^2 \to \mathbb{T}^2$ given by a matrix $A \in SL(2, \mathbb{Z})$, the Lefschetz number works out to $L(f) = 2 - \text{tr}(A)$. When $|\text{tr}(A)| > 2$, $A$ is hyperbolic (the linear Anosov diffeomorphism), and $L(f) \neq 0$ guarantees fixed points — which we can verify directly as the integer points $\mathbb{Z}^2/\mathbb{Z}^2$ on the torus.

The Lefschetz theorem is a bridge between algebraic topology (cohomology) and dynamics (fixed points). It will reappear when we study the topology of iterated maps and the distribution of periodic orbits.
