# 9.4 The Shadowing Lemma

Here is a question that should concern anyone who runs computer simulations of chaotic systems: are the orbits you compute real? Every floating-point computation introduces a small error. After many iterations, these errors accumulate. Does the sequence of points your computer produces correspond to any actual orbit of the dynamical system?

The shadowing lemma says yes — for hyperbolic systems.

The shadowing lemma is the rigorous bridge between numerical simulations and theoretical analysis. It says: approximate orbits (with small errors) are approximated by true orbits.

**Definition 9.4.1.** A *$\delta$-pseudo-orbit* of $f$ is a sequence $(x_n)_{n \in {\mathbb Z}}$ with $d(f(x_n), x_{n+1}) \leq \delta$ for all $n$.

A pseudo-orbit is a sequence that almost satisfies the dynamical rule: at each step, the error is at most $\delta$. This is exactly what a numerical simulation produces — the computer applies $f$ approximately, with roundoff error at most $\delta \approx 10^{-16}$.

**Theorem 9.4.2 (Shadowing Lemma / Anosov-Bowen).** Let $\Lambda$ be a hyperbolic set of $f$. For every $\varepsilon > 0$ there exists $\delta > 0$ such that: every $\delta$-pseudo-orbit $(x_n)$ in $\Lambda$ is $\varepsilon$-shadowed by a true orbit: there exists $y \in M$ with $d(f^n(y), x_n) \leq \varepsilon$ for all $n$.

If $\Lambda = M$ (Anosov), the true orbit is unique and the shadowing point depends continuously on the pseudo-orbit.

*(proof sketch)* The proof reduces to finding a fixed point of a certain operator in the space of sequences $(y_n)$ with the property that $y_{n+1} = f(y_n) + \text{small error}$. The hyperbolic splitting ensures this operator is a contraction.

What this is really saying: pseudo-orbits are shadowed by true orbits. The $\delta$ (size of errors at each step) and $\varepsilon$ (closeness of the shadowing orbit) are related, and for small $\delta$ you get small $\varepsilon$. So numerical simulations with errors $\delta$ are within $\varepsilon$ of a true orbit — the simulation is "real" in this precise sense.

**Numerical Consequence:** Computer simulations of hyperbolic systems produce pseudo-orbits (due to floating-point errors $\approx 10^{-16}$). The shadowing lemma guarantees these pseudo-orbits are close to true orbits — so numerical simulations of Anosov systems are *valid*.

The shadowing lemma is also a key tool in proofs. If you can construct a pseudo-orbit with certain properties, you can conclude that a true orbit with similar properties exists. This is how many existence theorems for periodic orbits of hyperbolic systems are proved.

The proof uses the hyperbolic splitting in an essential way. The stable direction contracts forward, the unstable direction contracts backward. A pseudo-orbit has errors in both directions, but the splitting lets you correct them: stable errors can be corrected by adjusting the past (using the contraction of $f^{-n}$ in the unstable direction), and unstable errors can be corrected by adjusting the future. The result is a fixed-point argument in a suitable Banach space.

The next section develops the tool that makes the symbolic coding of the horseshoe work for general Anosov systems: Markov partitions.
