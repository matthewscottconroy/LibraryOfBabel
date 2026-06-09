# 33.1 Orbit Equivalence for Measure-Preserving Systems

Here's a radical idea: two dynamical systems might be "the same" even if the groups acting on them are completely different. Not the same in the usual sense — not isomorphic as group actions — but the same in a coarser sense that only cares about which points end up in the same orbit.

This is orbit equivalence, and it turns out to be exactly the right notion of "sameness" for many purposes. Two systems are orbit equivalent if there's a measure-preserving bijection between the spaces that sends orbits to orbits — you're allowed to relabel the "time steps" completely, as long as you preserve which points travel together.

**Definition 33.1.1.** Two free ergodic measure-preserving actions $\Gamma \curvearrowright (X, \mu)$ and $\Lambda \curvearrowright (Y, \nu)$ are *orbit equivalent (OE)* if there is a measure space isomorphism $\phi: X \to Y$ such that $\phi$ maps $\Gamma$-orbits to $\Lambda$-orbits: $\phi(\Gamma \cdot x) = \Lambda \cdot \phi(x)$ for a.e. $x$.

**Definition 33.1.2.** The *orbit equivalence relation* of $\Gamma \curvearrowright X$ is:
$$\mathcal{R}_\Gamma = \{(x, y) \in X \times X : y \in \Gamma \cdot x\}.$$

Two actions are OE iff their orbit equivalence relations are isomorphic as equivalence relations (there is a measure-space isomorphism identifying them).

The key point: orbit equivalence doesn't ask how the group moves points — it only asks which points share an orbit. Two completely different groups acting in completely different ways could produce the same equivalence relation on the underlying space.

**Key Question:** Which pairs $(\Gamma, \Lambda)$ of groups can have orbit-equivalent free ergodic actions? Is OE-class determined by the group, or can very different groups have the same orbit structure?

The answer, as we'll see, depends dramatically on whether the groups are amenable.
