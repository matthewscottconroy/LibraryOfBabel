# 34.5 Topological Sofic Entropy

Everything so far has been about measure-theoretic entropy. But there's a topological version too. For amenable groups, topological entropy and measure-theoretic entropy are related by the variational principle: the topological entropy equals the supremum of the measure-theoretic entropy over all invariant measures. Does this survive to the sofic setting?

Kerr and Li defined topological sofic entropy in their 2011 paper.

**Definition 34.5.1 (Kerr-Li, 2011).** For a continuous action $\Gamma \curvearrowright X$ of a sofic group on a compact metric space:
$$h_\Sigma^{\text{top}}(\Gamma \curvearrowright X) = \sup_{\mu \in M_\Gamma(X)} h_\Sigma(\Gamma \curvearrowright (X, \mu)),$$
where the sup is over $\Gamma$-invariant probability measures.

This is the direct definition by taking the supremum over invariant measures. There's also a definition using "microstate spaces" for the topological action — counting the number of approximate equivariant maps from the sofic approximation to $X$, in a metric sense.

**Theorem 34.5.2 (Variational Principle for Sofic Entropy).** For sofic group actions on compact spaces:
$$h_\Sigma^{\text{top}}(\Gamma \curvearrowright X) \geq \sup_\mu h_\Sigma(\Gamma \curvearrowright (X, \mu)),$$
with equality when the action is expansive.

For expansive actions, the variational principle holds: the topological entropy equals the supremum of the measure-theoretic entropies. But the variational principle can fail in general.

**Remark 34.5.3.** The variational principle may fail in general for sofic entropy — this is a fundamental difference from the classical (amenable) case, where the variational principle always holds.

This failure is not a defect of the theory — it's a genuine feature. For non-amenable groups, the relationship between topological and measure-theoretic complexity is more subtle. There can be compact systems where the topological entropy is strictly larger than the supremum of the measure-theoretic entropies. Understanding when the variational principle holds, and when it fails, is part of the ongoing research program in sofic entropy theory.
