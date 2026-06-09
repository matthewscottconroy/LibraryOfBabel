# 7.2 Poincaré Recurrence Theorem

In Chapter 6, we proved a topological version of Poincaré recurrence: for any homeomorphism of a compact space and any nonempty open set $U$, some iterate of $U$ intersects $U$. This was purely qualitative.

The measure-theoretic version is dramatically sharper. It tells us not just that *some* points return, but that *almost every* point in $A$ returns to $A$ — and does so infinitely often.

**Theorem 7.2.1 (Poincaré Recurrence).** Let $(X, \mathcal{B}, \mu, f)$ be an MPT and $A \in \mathcal{B}$ with $\mu(A) > 0$. Then $\mu$-a.e. point $x \in A$ returns to $A$ infinitely often: for a.e. $x \in A$, the set $\{n \geq 1 : f^n(x) \in A\}$ is infinite.

*(proof)* Let $B = \{x \in A : f^n(x) \notin A \text{ for all } n \geq 1\}$ (the set of points that never return). The sets $B, f^{-1}(B), f^{-2}(B), \ldots$ are pairwise disjoint:
if $x \in f^{-m}(B) \cap f^{-n}(B)$ with $m < n$, then $f^m(x) \in B$, so $f^{n-m}(f^m(x)) = f^n(x) \notin B$ (since $f^m(x)$ never visits $A$), but then $f^n(x) \notin B$, contradiction. Since all $f^{-k}(B)$ have the same measure (MPT) and are disjoint, and $\mu(X) = 1$, we need $\sum_k \mu(f^{-k}(B)) = \sum_k \mu(B) \leq 1$, forcing $\mu(B) = 0$. So a.e. point of $A$ returns at least once. Applying this to $f^n(A)$ for each $n$ shows infinitely many returns.

The proof is a beautiful argument. The pairwise disjointness of $B, f^{-1}(B), f^{-2}(B), \ldots$ follows from the definition of $B$: if a point lands in $B$, its future iterates are excluded from $A$, so its future iterates can't land in $B$ either. The measure-preservation forces all these disjoint sets to have the same measure. Since their total measure is at most 1, the individual measure must be 0.

**Remark 7.2.2.** The measure-theoretic version is much stronger than the topological Poincaré theorem: it says a.e. point (not just some point) returns to every set of positive measure.

This distinction is worth sitting with. The topological theorem guarantees that *the set* $U$ is hit by some iterate of $U$ — a set of positive measure returns to itself. The measure-theoretic theorem says *individual points* return — almost every specific point will come back. The difference is the difference between "the system visits $A$ again" and "you, starting from $x$, will visit $A$ again" — and the measure-theoretic version gives you the personal guarantee.

This sets up the central question: how often does the orbit return? How long does it take? The answer to the first question is given by the Birkhoff ergodic theorem, which we prove in the next section.
