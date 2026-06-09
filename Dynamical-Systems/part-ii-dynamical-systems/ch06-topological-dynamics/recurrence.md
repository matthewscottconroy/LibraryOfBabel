# 6.2 Recurrence

One of the oldest and most fundamental observations in dynamics is this: systems that are "bounded" tend to return. Henri Poincaré proved a version of this in 1890 while studying the three-body problem. The precise statement requires some care, because "returning" can mean different things.

**Definition 6.2.1.** A point $x \in X$ is:
- *recurrent* if $x \in \omega_f(x)$ (i.e., the orbit of $x$ returns arbitrarily close to $x$)
- *nonwandering* if for every open $U \ni x$ there exists $n \geq 1$ with $f^n(U) \cap U \neq \emptyset$

Every periodic point is recurrent; every recurrent point is nonwandering.

Recurrence says: *this specific point* comes back close to itself. Nonwandering says: *some nearby point* eventually returns to the neighborhood. The distinction matters — you can have nonwandering points that are not recurrent (transitive points of a transitive system, for instance). But both notions capture the intuition that the system is "confined," not drifting away forever.

**Definition 6.2.2.** The *nonwandering set* $\Omega(f)$ consists of all nonwandering points. It is a closed $f$-invariant set.

The nonwandering set is often much smaller than the whole space, but it's where all the interesting long-term behavior lives. Fixed points, periodic orbits, and limit sets all live inside $\Omega(f)$.

---

## The Poincaré Recurrence Theorem

Here is the theorem that started it all. The proof is beautifully simple.

**Theorem 6.2.3 (Poincaré Recurrence Theorem — Topological Version).** Let $f: X \to X$ be a homeomorphism of a compact metric space. For any open set $U \neq \emptyset$, there exists $n \geq 1$ with $f^n(U) \cap U \neq \emptyset$.

*Proof:* Consider the sets $U, f^{-1}(U), f^{-2}(U), \ldots$ If they were pairwise disjoint, they could not all fit in a compact space (since $X$ has finite "covering number"). So some $f^{-m}(U) \cap f^{-n}(U) \neq \emptyset$ for $m < n$, i.e., $f^{n-m}(U) \cap U \neq \emptyset$.

What this is really saying: if your compact space were sliced into infinitely many pairwise-disjoint copies of any nonempty open set, it would have to be infinite-dimensional in some sense — certainly not compact. Compactness is doing all the work here. The theorem is pure pigeonhole principle at the topological level.

**Corollary 6.2.4.** $\Omega(f) = X$ for any homeomorphism of a compact space preserving a full-support measure. In particular, every point is nonwandering — orbits keep returning.

This topological version of Poincaré recurrence is somewhat weak: it says *some* points in $U$ return to $U$, but tells us nothing about *which* points or *how often*. The measure-theoretic version in Chapter 7 is much sharper: it says Lebesgue-almost-every point returns infinitely often. But even in its topological form, the recurrence theorem is a profound constraint on dynamics.

In the next section, we ask whether a system does something even stronger than recurrence: can a single orbit visit *every* part of the space?
