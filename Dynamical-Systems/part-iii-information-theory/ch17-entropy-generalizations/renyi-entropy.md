# 17.1 Rényi Entropy

Alfred Rényi, working in Budapest in the early 1960s, asked a natural question: Shannon entropy satisfies a particular set of axioms, and these axioms uniquely determine the formula. What if you relax one of the axioms — specifically, the chain rule? What family of entropies do you get?

The answer is the *Rényi entropies*, a one-parameter family indexed by $\alpha > 0$ that includes Shannon entropy as a special case and reveals several other important limits at the endpoints.

**Definition 17.1.1 (Rényi Entropy).** The *Rényi entropy of order $\alpha$* ($\alpha > 0$, $\alpha \neq 1$) of a discrete random variable $X$ with pmf $p = (p_1, \ldots, p_n)$ is:
$$H_\alpha(X) = \frac{1}{1-\alpha} \log \sum_i p_i^\alpha.$$

When $\alpha = 1$, we define $H_1 = H$ (Shannon entropy) by continuity. The formula for general $\alpha$ involves $\sum p_i^\alpha$, which is the $\alpha$-th power mean of the probabilities — a summary of the distribution that places different weight on common versus rare events depending on the value of $\alpha$.

The key to understanding the family is to look at its limits:

**Limits:**
- As $\alpha \to 1$: $H_\alpha(X) \to H_1(X) = -\sum p_i \log p_i$ (Shannon entropy — the generic case).
- As $\alpha \to 0$: $H_0(X) = \log |\text{supp}(X)|$ (*Hartley entropy* — log of the support size, ignoring probabilities entirely).
- As $\alpha \to \infty$: $H_\infty(X) = -\log \max_i p_i$ (*min-entropy* — determined entirely by the most likely outcome).
- At $\alpha = 2$: $H_2(X) = -\log \sum_i p_i^2$ (*collision entropy* — relevant for birthday paradox-type problems and randomness testing).

Each limit has a distinct operational character. Hartley entropy counts outcomes without caring how likely they are. Min-entropy measures the probability of the most dangerous event (for a cryptographic adversary, the most likely plaintext). Collision entropy captures the probability that two independent draws from $p$ give the same value.

These are not just mathematical curiosities. Each limit arises naturally in a different application:
- *Privacy amplification* uses min-entropy.
- *Guessing problems* (how many guesses to find $X$?) are governed by Rényi entropy of order $1/2$.
- *Large deviation exponents* are controlled by Rényi divergence (Section 17.3).
- *Fractal dimensions* of measures (see Chapter 14) are computed via Rényi entropies.

The Rényi family is monotone in $\alpha$:

**Theorem 17.1.2 (Monotonicity).** $H_\alpha(X)$ is non-increasing in $\alpha$: if $\alpha < \beta$ then $H_\alpha \geq H_\beta$.

This makes sense intuitively: larger $\alpha$ places more weight on the most probable outcomes, which are the "least surprising" ones — so larger $\alpha$ gives lower entropy. Min-entropy is the smallest of the family.

Rényi also gave a characterization theorem analogous to Shannon's:

**Theorem 17.1.3 (Rényi's Characterization).** The family $\{H_\alpha\}$ is the unique family (up to constants) satisfying: (1) symmetry in $(p_i)$, (2) a chain rule using a weighted average $\langle H_\alpha(Y|X = x)\rangle_\alpha$ (weighted by $p_i^\alpha$ rather than $p_i$), and (3) normalization $H_\alpha(\frac{1}{2},\frac{1}{2}) = 1$.

The key difference from Shannon's characterization is the weighted chain rule: instead of weighting the conditional entropies by the probabilities $p(x)$ (as Shannon entropy does), Rényi entropy weights by $p(x)^\alpha$. This gives more weight to likely events when $\alpha > 1$ and more weight to unlikely events when $\alpha < 1$. At $\alpha = 1$, the two coincide and you recover Shannon's chain rule.
