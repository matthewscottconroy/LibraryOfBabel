# 28.1 Categories of Dynamical Systems

What is a dynamical system? Until now, the answer has depended on context: a measure-preserving transformation for ergodic theory, a continuous map on a compact space for topological dynamics, a homeomorphism for invertible systems. Category theory asks us to be precise about this multiplicity: these are all objects in different categories, and the relations between them are functors.

## 28.1.1 The Basic Categories

The two primary categories we'll work with are the topological and the measure-theoretic.

**Definition 28.1.1.** The category $\mathbf{Top.Dyn}$ of *topological dynamical systems* has:
- *Objects*: pairs $(X, f)$ where $X$ is a compact metric space and $f: X \to X$ is continuous
- *Morphisms*: continuous maps $\phi: (X, f) \to (Y, g)$ with $\phi \circ f = g \circ \phi$ (factor maps)
- *Identity*: $\text{id}_X$
- *Composition*: composition of factor maps

The commutativity condition $\phi \circ f = g \circ \phi$ is the crucial constraint: a morphism must *intertwine* the two dynamics. This is the right notion of "map between dynamical systems" — it says that $\phi$ sends orbits of $f$ to orbits of $g$.

**Definition 28.1.2.** The category $\mathbf{Meas.Dyn}$ of *measure-preserving systems* has:
- *Objects*: $(X, \mathcal{B}, \mu, f)$ standard probability spaces with measure-preserving $f$
- *Morphisms*: measure-preserving maps $\phi$ with $\phi \circ f = g \circ \phi$ a.e.
- *Isomorphism*: $\phi$ is an isomorphism iff it is a.e. bijective (measure-theoretic conjugacy)

Here isomorphism is the right notion of "same system" in ergodic theory: two systems are isomorphic if there's a measure-preserving bijection intertwining the dynamics. This is what Ornstein's theorem classifies (for Bernoulli shifts) and what the Foreman-Rudolph-Weiss theorem says cannot be classified in general (Chapter 32).

**Observation 28.1.3.** There is a forgetful functor $\mathbf{Meas.Dyn} \to \mathbf{Top.Dyn}$ (forget the measure). Not every topological factor map preserves a given measure.

This forgetful functor is lossless in one direction: every measure-theoretic system is a topological system. But the reverse is not automatic — the topological structure doesn't determine the measure-theoretic structure.

## 28.1.2 Limits and Colimits in $\mathbf{Top.Dyn}$

One of the rewards of working categorically is that standard constructions — products, inverse limits — become recognizable as categorical limits, which means their universal properties are automatic.

**Products:** The product $(X \times Y, f \times g)$ is the categorical product in $\mathbf{Top.Dyn}$. An $f$-invariant measure on $X$ and a $g$-invariant measure on $Y$ yield an $(f \times g)$-invariant measure on $X \times Y$.

**Inverse Limits (Projective Limits):** If $(X_n, f_n)$ form a projective system with factor maps $\pi_{n+1}: (X_{n+1}, f_{n+1}) \to (X_n, f_n)$, the inverse limit $\varprojlim (X_n, f_n)$ is the dynamical system on:
$$X_\infty = \left\{(x_n) \in \prod_n X_n : \pi_{n+1}(x_{n+1}) = x_n\right\}$$
with $f_\infty(x_n) = (f_n(x_n))$.

**Example 28.1.4 (Natural Extension).** The *natural extension* of a non-invertible system $(X, f)$ is the inverse limit of the system $(X, f) \xleftarrow{f} (X, f) \xleftarrow{f} \cdots$. This is the smallest invertible extension, and it is the standard tool for studying non-invertible ergodic systems.

The natural extension is a perfect illustration of how categorical thinking generates mathematics. "The smallest invertible extension" is precisely the universal property of the inverse limit in $\mathbf{Top.Dyn}$. The fact that every system has a natural extension, and that it's unique up to isomorphism, follows from abstract category theory applied to the concrete situation.

Next: we ask how the category $\mathbf{Meas.Dyn}$ relates to categories of algebraic objects — Hilbert spaces, groupoids, and operator algebras.
