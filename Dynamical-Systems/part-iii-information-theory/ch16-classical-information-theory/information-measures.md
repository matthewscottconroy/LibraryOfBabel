# 16.1 Information Measures

## 16.1.1 Shannon Entropy

Shannon asked a deceptively simple question: how do you measure the information content of a message? Not its meaning — Shannon explicitly set meaning aside — but its pure *unpredictability*. How surprised should you be to receive this particular message?

The answer turned out to be essentially unique. If you want a function that measures "average surprise" and satisfies a handful of natural axioms — continuity, additivity across independent sources, maximum at the uniform distribution — there is only one choice (up to a constant). Shannon called it *entropy*, borrowing the term from thermodynamics, and the resonance was not accidental: the two concepts are deeply related, as we will see in Chapter 17.

**Definition 16.1.1 (Shannon Entropy).** The *Shannon entropy* of a discrete random variable $X$ with probability mass function $p(x) = P(X = x)$ is:
$$H(X) = -\sum_{x \in \mathcal{X}} p(x) \log p(x) = \mathbb{E}[-\log p(X)].$$

(Convention: $0 \log 0 = 0$. The logarithm base determines units: base 2 gives *bits*, base $e$ gives *nats*, base 10 gives *bans*.)

Here is the right way to read this formula: $-\log p(x)$ is the *surprise* of seeing outcome $x$ — low-probability events are more surprising, high-probability events are less so. Entropy is the *expected surprise*: the average amount of information you gain from learning the value of $X$.

A loaded coin ($P(H) = 0.99$) has low entropy — you're rarely surprised. A fair coin has maximum entropy — you're maximally uncertain before each flip. A deterministic variable ($P(X = x_0) = 1$) has entropy zero — no uncertainty, no information.

Entropy satisfies a clean set of basic properties:

**Theorem 16.1.2 (Basic Properties of Entropy).**
1. *Nonnegativity*: $H(X) \geq 0$, with equality iff $X$ is deterministic ($p(x_0) = 1$ for some $x_0$).
2. *Maximum*: $H(X) \leq \log |\mathcal{X}|$, with equality iff $X$ is uniform.
3. *Continuity*: $H$ is continuous in $p$.
4. *Expansibility*: $H(X, 0) = H(X)$ (adding an impossible event doesn't change entropy).
5. *Chain rule*: $H(X_1, \ldots, X_n) = \sum_{k=1}^n H(X_k | X_1, \ldots, X_{k-1})$.

*Proof of (2):* By Jensen's inequality applied to the convex function $t \mapsto t\log t$: since the uniform distribution maximizes $\sum_x p(x) \log p(x)$, the Gibbs inequality (or directly by the log-sum inequality) gives $H(X) \leq \log |\mathcal{X}|$.

The uniqueness of entropy — the fact that these axioms force the formula — is the content of Shannon and Khinchin's characterization theorem:

**Theorem 16.1.3 (Axiomatic Characterization — Shannon/Khinchin).** The only function satisfying continuity, the chain rule, expansibility, and maximum at the uniform distribution is $H = -c\sum p_i \log p_i$ for some $c > 0$.

This is reassuring: we did not pull a formula out of thin air and then check that it has nice properties. The properties themselves determine the formula.

## 16.1.2 Joint and Conditional Entropy

Once we have entropy for a single variable, we can extend it to pairs — and the extensions are exactly what you would hope for.

**Definition 16.1.4.** For a joint random variable $(X, Y)$:
$$H(X, Y) = -\sum_{x,y} p(x,y)\log p(x,y)$$
$$H(X|Y) = \sum_y p(y) H(X|Y=y) = -\sum_{x,y} p(x,y)\log p(x|y).$$

The joint entropy $H(X,Y)$ measures the total uncertainty in the pair. The conditional entropy $H(X|Y)$ measures the remaining uncertainty in $X$ after we learn $Y$ — it averages the entropy of $X$ over all possible values of $Y$.

These quantities satisfy the *chain rule*, which is perhaps the most important formula in classical information theory:

**Chain Rule:** $H(X, Y) = H(X) + H(Y|X) = H(Y) + H(X|Y)$.

Read this as: the total uncertainty in $(X, Y)$ equals the uncertainty in $X$ plus the remaining uncertainty in $Y$ once $X$ is known. It is a decomposition of uncertainty, and it extends naturally to any number of variables.

One immediate consequence: learning something cannot increase your uncertainty.

**Theorem 16.1.5 (Conditioning Reduces Entropy).** $H(X|Y) \leq H(X)$, with equality iff $X$ and $Y$ are independent.

## 16.1.3 Mutual Information

Now for one of the most useful quantities in the whole theory. Mutual information measures how much $X$ and $Y$ "share" — how much knowing one reduces uncertainty about the other.

**Definition 16.1.6 (Mutual Information).** The *mutual information* between $X$ and $Y$ is:
$$I(X; Y) = H(X) + H(Y) - H(X, Y) = H(X) - H(X|Y) = H(Y) - H(Y|X).$$

Equivalently:
$$I(X;Y) = \sum_{x,y} p(x,y) \log \frac{p(x,y)}{p(x)p(y)} = D_{\text{KL}}(p_{XY} \| p_X \otimes p_Y).$$

The second formula is illuminating: $I(X;Y)$ is the KL divergence between the joint distribution and the product of the marginals. It measures how far $(X,Y)$ is from being independent.

Mutual information is the right answer to the question: "how much does $Y$ tell me about $X$?" It satisfies:

**Properties:**
1. $I(X;Y) \geq 0$, with equality iff $X \perp Y$.
2. $I(X;Y) = I(Y;X)$ (symmetric — $X$ tells you as much about $Y$ as $Y$ tells you about $X$).
3. $I(X;Y) = H(X) - H(X|Y)$ (the reduction in uncertainty about $X$ after observing $Y$).
4. Chain rule: $I(X_1,\ldots,X_n; Y) = \sum_k I(X_k; Y | X_1,\ldots,X_{k-1})$.

In Chapter 16.4, mutual information will appear again as the key quantity determining the capacity of a noisy channel: the channel capacity is precisely the maximum of $I(X;Y)$ over all input distributions. This is not a coincidence — it is the operational meaning of mutual information.

## 16.1.4 KL Divergence

The last member of the family of basic information measures is also, in some ways, the most fundamental. KL divergence — also called *relative entropy* — measures how different two probability distributions are.

**Definition 16.1.7 (Kullback-Leibler Divergence).** The *Kullback-Leibler divergence* (relative entropy) between distributions $P$ and $Q$ is:
$$D_{\text{KL}}(P \| Q) = \sum_x p(x) \log \frac{p(x)}{q(x)} = E_P\left[\log\frac{p(X)}{q(X)}\right].$$

Read $D_{\text{KL}}(P\|Q)$ as: "the extra bits per symbol you pay for assuming $Q$ when the truth is $P$." If you use a code optimized for $Q$ but the source is actually $P$, you'll need roughly $D_{\text{KL}}(P\|Q)$ extra bits per symbol.

The most important property of KL divergence is its nonnegativity:

**Theorem 16.1.8 (Gibbs' Inequality / Nonnegativity).** $D_{\text{KL}}(P \| Q) \geq 0$, with equality iff $P = Q$.

*Proof:* By Jensen's inequality applied to the convex function $-\log$:
$$D_{\text{KL}}(P\|Q) = E_P\left[-\log\frac{q(X)}{p(X)}\right] \geq -\log E_P\left[\frac{q(X)}{p(X)}\right] = -\log\sum_x q(x) = 0.$$

Despite looking like a distance, KL divergence is emphatically not a metric. It fails two of the three metric axioms:

**Warning:** $D_{\text{KL}}$ is not a metric: it is asymmetric ($D_{\text{KL}}(P\|Q) \neq D_{\text{KL}}(Q\|P)$ in general) and does not satisfy the triangle inequality.

The asymmetry has a statistical interpretation: $D_{\text{KL}}(P\|Q)$ is the extra cost of assuming $Q$ when $P$ is true. This is different from assuming $P$ when $Q$ is true. The two directions correspond to different error types in hypothesis testing — a theme we will return to in Chapter 20.

All of entropy, conditional entropy, mutual information, and joint entropy can be expressed in terms of KL divergence. They are not separate concepts but different views of the same underlying geometry. We turn next to what happens when we apply these measures to long sequences — which is where the real magic happens.
