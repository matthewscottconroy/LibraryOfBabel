# 6.6 Topological Conjugacy

In any mathematical subject, the isomorphism relation tells you what it means for two objects to be "the same." For groups it's group isomorphism. For topological spaces it's homeomorphism. For dynamical systems the right notion is topological conjugacy.

**Definition 6.6.1.** Two TDSs $(X, f)$ and $(Y, g)$ are *topologically conjugate* if there exists a homeomorphism $h: X \to Y$ with $h \circ f = g \circ h$. If $h$ is only continuous (not necessarily a homeomorphism), it is a *factor map* and $(Y, g)$ is a *topological factor* of $(X, f)$.

The equation $h \circ f = g \circ h$ is the key. It says: applying $f$ in the $X$-world and then translating to $Y$ via $h$ is the same as translating first and then applying $g$. The dynamics commute with the change of coordinates. So $h$ carries the orbit structure of $(X, f)$ onto the orbit structure of $(Y, g)$ exactly.

Topological conjugacy is the correct notion of isomorphism for TDSs: conjugate systems have identical orbit structures, periodic points, entropy, and all topological invariants.

A factor map is a weaker version: the map $h$ need not be invertible. In that case, $(Y, g)$ is a simpler "image" of $(X, f)$, carrying some but potentially not all dynamical information.

---

## The Canonical Example: Quadratic and Tent Maps

**Example 6.6.2 (Conjugacy of Quadratic and Tent Maps).** The tent map $T: [0,1] \to [0,1]$, $T(x) = 1 - |2x-1|$, is topologically conjugate to $f_{-2}: [-2,2] \to [-2,2]$, $f_{-2}(x) = x^2 - 2$. The conjugacy is $h(x) = -2\cos(\pi x)$ (or $x = (1/\pi)\arccos(-y/2)$).

This is one to verify by hand. Check that $h \circ T = f_{-2} \circ h$ using $\cos(\pi(1-|2x-1|)) = \cos(\pi - 2\pi|x - 1/2|) = -\cos(2\pi|x - 1/2|)$, and the double-angle formula. The conjugacy is explicit and clean.

Why does this matter? Because it means the tent map and $f_{-2}$ have *exactly the same dynamics*, just drawn in different coordinates. Any theorem you prove about periodic points, transitivity, or entropy of the tent map automatically transfers to $f_{-2}$ and vice versa. Conjugacy is a powerful labor-saving device.

---

## What Conjugacy Preserves

**Theorem 6.6.3.** Topological conjugacy preserves:
- Minimality and topological transitivity
- Topological entropy (see Chapter 22)
- The set of periods of periodic orbits (by period)
- Equicontinuity, distality

The proof of each item is a short exercise: you use the conjugacy to translate the property from one system to the other. For instance, if $(X, f)$ has a dense orbit $\mathcal{O}(x)$, then $h(\mathcal{O}(x)) = \mathcal{O}(h(x))$ is a dense orbit in $(Y, g)$ (since $h$ is a homeomorphism). So transitivity is preserved.

The most important conjugacy invariant we haven't discussed yet is topological entropy — a single number that captures the exponential growth rate of orbits. We'll build that theory in Chapter 22. For now, just note that the tent map and $f_{-2}$ have the same entropy, which is $\log 2$.

In the next section, we formalize what it means for a dynamical system to be "chaotic."
