# 34.3 The Non-Bernoulli Problem and Sofic Entropy

Ornstein's theorem — the classification of Bernoulli shifts by entropy — is the centerpiece of Chapter 7. But Ornstein's theorem only works for amenable groups. For non-amenable groups, no one knew for fifty years whether entropy classified Bernoulli shifts.

Bowen settled this in the same 2010 paper that introduced sofic entropy.

**The Classical Ornstein Problem:** For amenable groups, Bernoulli shifts with the same base entropy are isomorphic (Ornstein's theorem). What happens for non-amenable groups?

**Theorem 34.3.1 (Bowen, 2010).** Two Bernoulli shifts $\Gamma \curvearrowright (X_0)^\Gamma$ and $\Gamma \curvearrowright (Y_0)^\Gamma$ of a free group $\Gamma = F_r$ are isomorphic iff $H(X_0) = H(Y_0)$. The sofic entropy distinguishes them.

**Remark 34.3.2.** This resolved a long-open problem: Bernoulli shifts of free groups are classified by their entropy, generalizing Ornstein's theorem to the non-amenable setting.

The proof is more direct than Ornstein's: sofic entropy gives an explicit isomorphism invariant, and the calculation that sofic entropy of a Bernoulli shift equals the base entropy $H(\mu_0)$ provides the complete classification.

There is a subtlety here. The sofic entropy of a Bernoulli shift is the same for every sofic approximation. But for non-Bernoulli actions, the situation is more delicate. The first examples where sofic entropy depends on the sofic approximation were found only recently — this is genuinely live research.

**Theorem 34.3.3 (Ornstein and Weiss, Bowen).** For any sofic group $\Gamma$: if two free ergodic actions are orbit equivalent, their sofic entropies satisfy:
$$h_\Sigma(\Gamma \curvearrowright X) \leq h_\Sigma(\Lambda \curvearrowright Y) \quad \text{if } \Gamma \leq \Lambda \text{ as orbit-equivalence relations.}$$

Sofic entropy is monotone under "orbit sub-relations." This means orbit equivalence constrains entropy even for non-amenable groups — and combined with Bowen's theorem, gives the classification of Bernoulli shifts.

Nobody knows whether sofic entropy depends on the sofic approximation for every action of every non-amenable group. The first examples where it does depend were found only recently. This is live research.
