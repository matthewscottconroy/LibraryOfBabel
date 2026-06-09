# 33.4 Popa's Deformation-Rigidity Theory

Sorin Popa's deformation-rigidity theory, developed through the 2000s, pushed orbit equivalence rigidity to its logical extreme. The central idea: some actions are so rigid that orbit equivalence implies isomorphism. If you can only move the orbit structure around, you can't actually change anything — the action itself is fixed.

This requires a group with special properties. The key property is Kazhdan's property (T).

**Definition 33.4.3 (Property (T)).** A group $\Gamma$ has *Kazhdan's property (T)* if every unitary representation with almost-invariant vectors has a nonzero fixed vector. Examples: $SL(n, {\mathbb Z})$ for $n \geq 3$, lattices in higher-rank Lie groups.

Property (T) is a "spectral gap" condition — the group cannot almost fix a vector in a representation without actually fixing one. This rigidity in representation theory translates into rigidity for measure-preserving actions.

Popa's cocycle superrigidity theorem is the technical heart of the theory.

**Theorem 33.4.1 (Popa's Cocycle Superrigidity, 2005).** For a Bernoulli action $\Gamma \curvearrowright (X, \mu) = (X_0, \mu_0)^\Gamma$ of a group with property (T) (or more generally, a "malleable" action), every measurable cocycle $c: \Gamma \times X \to \Lambda$ is cohomologous to a group homomorphism $\rho: \Gamma \to \Lambda$.

What does this say? A cocycle is a "twisted" homomorphism — it satisfies the cocycle equation $c(\gamma_1\gamma_2, x) = c(\gamma_1, \gamma_2 x) c(\gamma_2, x)$ but need not be a genuine group homomorphism because of the $x$-dependence. Cocycle superrigidity says the $x$-dependence is illusory: every cocycle is cohomologous to one with no $x$-dependence at all. The action is so rigid it forces all cocycles to be genuine homomorphisms.

From cocycle superrigidity, Popa derived OE-superrigidity:

**Theorem 33.4.2 (Popa, 2006).** For any countable groups $\Gamma$ with property (T) and $\Lambda$: any two free ergodic actions of $\Gamma$ that are orbit equivalent are actually isomorphic (not just orbit equivalent). This is *OE-superrigidity*.

OE-superrigidity is the sharpest possible rigidity result. Orbit equivalence, which is a coarser equivalence than isomorphism, nevertheless forces isomorphism. You cannot deform these actions without changing them fundamentally.

Even more striking is Ioana's 2011 theorem, which handles $SL(2, \mathbb{Z})$ — a group that does NOT have property (T) but is still OE-superrigid for Bernoulli actions:

**Theorem 33.4.4 (Ioana, 2011).** For $\Gamma = SL(2, {\mathbb Z})$, Bernoulli actions are OE-superrigid: two Bernoulli actions of $SL(2, {\mathbb Z})$ are orbit equivalent iff they are isomorphic iff they have the same base entropy.

The techniques Ioana used here — "spectral gap rigidity" for the action of $SL(2, \mathbb{Z})$ on its Bernoulli space — introduced new tools that have influenced the entire field since.
