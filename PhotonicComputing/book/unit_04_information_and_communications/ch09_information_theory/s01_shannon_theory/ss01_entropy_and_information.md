# 9.1.1 Entropy and Mutual Information

## Information as Surprise

Before Shannon, information was understood intuitively but not mathematically. Everyone knew that learning "it will rain tomorrow" contains more information than learning "the sun rose this morning" — but no one had quantified *how much* more.

Shannon's insight was to define information in terms of probability: a message is informative to the extent that it was unexpected. If an event has probability $p$, receiving confirmation that it occurred conveys $-\log_2 p$ bits of information. This makes intuitive sense:
- A fair coin flip ($p = 1/2$): $-\log_2(1/2) = 1$ bit per flip.
- Rolling a "6" on a fair die ($p = 1/6$): $-\log_2(1/6) = 2.58$ bits.
- Learning that the sun rose ($p \approx 1$): $-\log_2(1) = 0$ bits — no information.

The base-2 logarithm gives information in bits; natural logarithm gives nats; base-10 gives hartleys. Bits are standard in communications; nats are sometimes used in quantum information.

## Entropy

For a discrete random variable $X$ with outcomes $\{x_1, \ldots, x_n\}$ and probabilities $\{p_1, \ldots, p_n\}$, the **Shannon entropy** is the expected information:

$$H(X) = -\sum_{i=1}^n p_i \log_2 p_i \quad \text{(bits per symbol)}$$

with the convention $0\log_2 0 = 0$.

Key properties:
- **Non-negative**: $H(X) \geq 0$, with equality iff one $p_i = 1$ (certain outcome).
- **Maximum**: $H(X) \leq \log_2 n$, achieved when all outcomes are equally likely (uniform distribution).
- **Concavity**: $H$ is a concave function of the probability distribution.

For a binary source ($n = 2$) with $P(\text{zero}) = p$:

$$H_b(p) = -p\log_2 p - (1-p)\log_2(1-p)$$

This **binary entropy function** equals 1 bit at $p = 1/2$ (maximum, perfectly unpredictable) and 0 at $p = 0$ or $p = 1$ (completely predictable).

## Joint Entropy and Conditional Entropy

For two variables $X$ and $Y$:

$$H(X, Y) = -\sum_{x,y} p(x,y)\log_2 p(x,y)$$

$$H(Y|X) = -\sum_{x,y} p(x,y)\log_2 p(y|x) = H(X,Y) - H(X)$$

$H(Y|X)$ measures how much uncertainty remains about $Y$ after knowing $X$. If $X$ and $Y$ are independent, $H(Y|X) = H(Y)$ — knowing $X$ tells you nothing about $Y$.

## Mutual Information

The **mutual information** between $X$ and $Y$ is the reduction in uncertainty about $Y$ caused by knowing $X$:

$$I(X; Y) = H(Y) - H(Y|X) = H(X) + H(Y) - H(X,Y)$$

Alternatively, $I(X;Y) = \sum_{x,y} p(x,y)\log_2\frac{p(x,y)}{p(x)p(y)}$ — the KL divergence between the joint distribution and the product of marginals.

Mutual information is symmetric: $I(X;Y) = I(Y;X)$. It is zero iff $X$ and $Y$ are independent.

For a communication channel where $X$ is the transmitted symbol and $Y$ is the received symbol:
- $H(X)$ is the entropy of the input (how much information is being sent)
- $H(Y|X)$ is the noise entropy (how much additional randomness the channel adds)
- $I(X;Y) = H(Y) - H(Y|X)$ is how much of the transmitted information survives the channel noise

The channel capacity $C$ is the maximum mutual information over all input distributions:

$$C = \max_{p(x)} I(X;Y) \quad \text{bits per channel use}$$

Shannon's channel coding theorem states that reliable communication at rate $R < C$ is achievable, and impossible for $R > C$.

## Differential Entropy for Continuous Distributions

For a continuous random variable $X$ with probability density $f(x)$, the **differential entropy** is:

$$h(X) = -\int f(x)\log_2 f(x)\, dx$$

Unlike discrete entropy, differential entropy can be negative and depends on the units. Key result: of all continuous distributions with a given variance $\sigma^2$, the Gaussian distribution maximizes differential entropy:

$$h(X_{\text{Gaussian}}) = \frac{1}{2}\log_2(2\pi e\sigma^2) \quad \text{bits}$$

This result is essential for the Shannon-Hartley theorem.

---

## References

[1] Shannon, C.E. (1948). "A mathematical theory of communication." *Bell System Technical Journal*, 27(3), 379–423. [The original source; Sections 2–7 develop entropy and its properties.]

[2] Cover, T.M. & Thomas, J.A. (2006). *Elements of Information Theory*, 2nd ed. Wiley. [The standard graduate textbook; Chapter 2 covers entropy and mutual information rigorously.]
