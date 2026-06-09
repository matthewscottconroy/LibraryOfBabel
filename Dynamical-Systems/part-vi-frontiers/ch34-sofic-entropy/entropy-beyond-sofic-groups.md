# 34.4 Entropy Beyond Sofic Groups

Sofic entropy is defined using sofic approximations. But what if the group isn't sofic? Or what if we don't know whether it's sofic? Is there an entropy theory that works for all groups?

The answer is yes. Brandon Seward developed Rokhlin entropy, which works for any free ergodic action of any countable group. No sofic approximation needed.

**Definition 34.4.1 (Rokhlin Entropy — Seward, 2014).** For any free ergodic action $\Gamma \curvearrowright (X, \mu)$ (not assuming soficity):
$$h_{\text{Rok}}(\Gamma \curvearrowright X) = \inf_\xi H(\xi | \xi^-),$$
where the infimum is over all generating partitions $\xi$ and $\xi^- = \bigvee_{e \neq \gamma \in \Gamma} \gamma\xi$ is the "past" partition.

The Rokhlin entropy measures the conditional entropy of a generating partition given all the other "translates" of that partition — the information that $\xi$ contains that isn't already in the rest of the partition. This is a direct generalization of the Rokhlin-Sinai characterization of KS entropy.

**Theorem 34.4.2 (Seward, 2020).** Rokhlin entropy agrees with sofic entropy for sofic groups: if $\Gamma$ is sofic and the action is free and ergodic:
$$h_{\text{Rok}}(\Gamma \curvearrowright X) = h_\Sigma(\Gamma \curvearrowright X).$$

This is a remarkable theorem. Two completely different definitions of entropy — one using microstates and sofic approximations, the other using conditional entropy of generating partitions — give the same answer for sofic group actions. This strongly suggests both are capturing the "right" notion of entropy.

**Theorem 34.4.3.** Rokhlin entropy is defined for all groups, not just sofic ones. It satisfies:
- $h_{\text{Rok}} \leq H(\xi)$ for any generating partition $\xi$
- For Bernoulli shifts: $h_{\text{Rok}}(\Gamma \curvearrowright (X_0)^\Gamma) = H(X_0)$
- For amenable groups: $h_{\text{Rok}}$ equals KS entropy

Seward used Rokhlin entropy to prove a far-reaching generalization of Krieger's finite generator theorem: every free ergodic action of any countable group with finite Rokhlin entropy has a finite generating partition. This extends a classical theorem from the amenable case to all groups.

What happens for possibly non-sofic groups? We don't know. If a non-sofic group exists, Rokhlin entropy would still be defined and would satisfy the Bernoulli formula — but we couldn't compare it to sofic entropy, because there is none.
