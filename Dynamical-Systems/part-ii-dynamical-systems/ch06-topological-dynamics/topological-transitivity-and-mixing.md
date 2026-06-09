# 6.3 Topological Transitivity and Mixing

Recurrence says an orbit comes back close to its starting point. Transitivity asks for something much more ambitious: can a single orbit get close to *every* point in the space? If yes, the system has an irreducible, globally coherent quality — you can't break it into independent pieces.

**Definition 6.3.1.** $(X, f)$ is *topologically transitive* if there exists a point $x \in X$ with dense orbit: $\overline{\mathcal{O}(x)} = X$.

Equivalently (for compact metric spaces with countable basis): for every pair of nonempty open sets $U, V \subseteq X$, there exists $n \geq 0$ with $f^n(U) \cap V \neq \emptyset$.

The two characterizations are worth comparing. The first says: one orbit is dense. The second says: any open set eventually hits any other open set. These sound different but are equivalent (for nice spaces), and both capture the idea that the system is "globally connected" — the map can take any region of the space near any other region.

Now here's a strictly stronger notion:

**Definition 6.3.2.** $(X, f)$ is *topologically mixing* if for every nonempty open $U, V \subseteq X$, there exists $N$ such that $f^n(U) \cap V \neq \emptyset$ for all $n \geq N$ (not just for some $n$).

Mixing implies transitivity but not vice versa (an irrational rotation is transitive but not mixing).

The difference is subtle but important. Transitivity says you can connect any two open sets along *some* orbit segment. Mixing says that once the time is long enough, *every* long orbit segment from $U$ will reach $V$. The set $f^n(U)$ becomes "uniformly spread" across the space as $n \to \infty$.

The irrational rotation is a perfect illustration of the gap. It's transitive — any orbit is dense. But it's not mixing. The image of a small arc rotates rigidly, so it returns periodically to its starting position and misses other arcs along the way. There's no asymptotic spreading; the map is too orderly.

---

## Transitivity and the Baire Category Theorem

There's a beautiful interaction between transitivity and the Baire category theorem. It says that for a transitive homeomorphism, the "exceptional" points — those without dense orbits — are topologically negligible.

**Theorem 6.3.3 (Baire Category and Transitivity).** For a compact metric space without isolated points and a homeomorphism $f$: $(X, f)$ is transitive iff the set of points with dense orbits is a dense $G_\delta$ (residual) set.

*(proof sketch)* The set $\{x : f^n(x) \in V\}$ is open and dense for each open $V$ (by transitivity). The set of points with dense orbits is $\bigcap_{V \in \mathcal{V}} \bigcup_{n \geq 0} f^{-n}(V)$ over a countable basis $\mathcal{V}$ — a countable intersection of open dense sets, hence residual by Baire.

What this is really saying: transitivity is not a property of one lucky orbit. It's a *generic* property — almost every orbit (in the topological sense of "all but a meager set") is dense. The Baire theorem lets us promote a single dense orbit to a residual set of dense orbits. This is one of the places where the Baire category theorem earns its keep in dynamics.

Transitivity is necessary but not sufficient for the richest kind of dynamics. In the next section, we examine what happens when *every* orbit — not just a residual set — is dense.
