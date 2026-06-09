# 16.4 The Noisy Channel

## 16.4.1 Channel Models

Compression handles the source side: how efficiently can we represent information? The noisy channel problem handles the transmission side: given a channel that corrupts data, how reliably can we communicate?

The brilliant insight — perhaps the most surprising in the whole theory — is that reliable communication is *always* possible, at any rate below capacity. Noise does not impose a reliability limit; it imposes a *rate* limit. You can communicate reliably at any rate below the channel capacity, and you cannot do so above it. This dichotomy is exact.

We begin with the right mathematical model of a noisy channel.

**Definition 16.4.1 (Discrete Memoryless Channel).** A *discrete memoryless channel (DMC)* is specified by:
- Input alphabet $\mathcal{X}$, output alphabet $\mathcal{Y}$
- Transition probabilities $p(y|x) = P(Y = y | X = x)$

"Memoryless" means successive channel uses are independent: $p(y_1,\ldots,y_n | x_1,\ldots,x_n) = \prod_i p(y_i|x_i)$.

The memorylessness assumption is an idealization, but a useful one: each use of the channel is an independent probabilistic experiment, summarized entirely by $p(y|x)$.

**Examples 16.4.2.**
- *Binary Symmetric Channel (BSC)*: $\mathcal{X} = \mathcal{Y} = \{0,1\}$, with $p(0|0) = p(1|1) = 1-\epsilon$ and $p(1|0) = p(0|1) = \epsilon$. Each bit is flipped independently with probability $\epsilon$.
- *Binary Erasure Channel (BEC)*: outputs $X$ with probability $1-\epsilon$ and the symbol $\text{?}$ (erasure) with probability $\epsilon$. The output is either the exact input or a known "I didn't receive anything" signal.
- *Gaussian Channel*: $Y = X + Z$ where $Z \sim N(0, N)$. This models telephone lines, wireless communications, and analog channels everywhere.

## 16.4.2 Channel Capacity

With a channel model in hand, we can define the most important quantity in communications engineering: capacity.

**Definition 16.4.3 (Channel Capacity).** The *capacity* of a DMC is:
$$C = \max_{p(x)} I(X; Y).$$

This is the maximum mutual information between input and output, maximized over all possible input distributions $p(x)$. Capacity is measured in bits (or nats) per channel use.

The formula is worth pausing over. Mutual information $I(X;Y)$ measures how much the output $Y$ tells us about the input $X$. Maximizing over input distributions finds the best way to use the channel — the distribution that maximizes the information transfer. Capacity is what you get when you use the channel optimally.

**Examples:**
- BSC with bit-flip probability $\epsilon$: $C = 1 - H(\epsilon, 1-\epsilon) = 1 - h_b(\epsilon)$ bits/use, where $h_b(\epsilon) = -\epsilon\log\epsilon - (1-\epsilon)\log(1-\epsilon)$ is the binary entropy function. At $\epsilon = 0$ (no noise), $C = 1$. At $\epsilon = 1/2$ (complete noise), $C = 0$.
- BEC with erasure probability $\epsilon$: $C = 1 - \epsilon$ bits/use. Remarkably clean: you just lose the fraction $\epsilon$ of channel uses to erasure.
- AWGN channel with power $P$ and noise variance $N$: $C = \frac{1}{2}\log(1 + P/N)$ (Shannon's formula). This is the famous signal-to-noise ratio formula, and it tells engineers exactly what happens as you increase transmit power or decrease noise.

To make the capacity theorem precise, we need to formalize what "reliable communication" means.

**Definition 16.4.4 ($(n, M, \varepsilon)$-Code).** An $(n, M, \varepsilon)$-code for a channel consists of:
- An *encoder*: a mapping from $\{1, \ldots, M\}$ (messages) to $\mathcal{X}^n$ (channel inputs)
- A *decoder*: a mapping from $\mathcal{Y}^n$ to $\{1, \ldots, M\}$

with maximum probability of error $\max_m P(\text{error} \mid m\text{ sent}) \leq \varepsilon$.

A rate $R$ is *achievable* if for every $\varepsilon > 0$ and large enough $n$, there is an $(n, 2^{nR}, \varepsilon)$-code.

In plain terms: we use the channel $n$ times to send one of $M = 2^{nR}$ messages. The rate $R$ is the number of message bits per channel use. An achievable rate is one at which we can transmit reliably — with vanishing error probability — as we take $n \to \infty$.

## 16.4.3 Shannon's Noisy Channel Coding Theorem

This is the result that makes information theory remarkable.

**Theorem 16.4.5 (Shannon's Noisy Channel Coding Theorem — 1948).** The capacity $C$ of a DMC is the supremum of achievable rates:
- *(Achievability)* For every $R < C$ and $\varepsilon > 0$, there exists an $(n, 2^{nR}, \varepsilon)$-code for large enough $n$.
- *(Converse)* For every $R > C$ and every $(n, 2^{nR}, \varepsilon)$-code, $\varepsilon \geq 1 - R/C - o(1)$.

Let's appreciate what this says. For any rate $R$ strictly below capacity, you can communicate reliably — with error probability going to zero — using long codes. The error does not just become small; it vanishes. For rates above capacity, no code works: error is bounded away from zero. The capacity $C$ is the exact threshold.

The proof has two parts, each illuminating in its own way.

*(Achievability proof — Random Coding).* Generate $M = 2^{nR}$ codewords $x^n(1), \ldots, x^n(M)$ i.i.d. from $p^*(x)$, the capacity-achieving input distribution. This is the *random codebook*. To decode $y^n$: find the unique $m$ such that $(x^n(m), y^n)$ are jointly typical. For $R < C$, by the joint typicality lemma, the probability of error goes to zero as $n \to \infty$.

The random coding argument is beautiful precisely because it is non-constructive. Shannon proved that good codes *exist* without saying how to find them efficiently. This separation of existence from construction drove coding theorists to search for explicit good codes for the next 50 years — and eventually produce turbo codes, LDPC codes, and polar codes, which achieve capacity with efficient algorithms.

*(Converse proof — Fano's Inequality).* The converse relies on a fundamental inequality:

**Theorem 16.4.6 (Fano's Inequality).** Let $M$ be a random variable uniform on $\{1, \ldots, 2^{nR}\}$, transmitted over the channel, and $\hat{M}$ the decoded estimate. If $P_e = P(M \neq \hat{M})$:
$$H(M|\hat{M}) \leq H(P_e) + P_e \cdot nR.$$

Fano's inequality says: if you make errors with probability $P_e$, then the remaining uncertainty in $M$ given your guess $\hat{M}$ is at most $H(P_e) + P_e \cdot nR$. For $P_e$ small, this is small. But the channel also bounds $H(M|\hat{M}) \geq H(M) - nC = n(R - C)$. These two bounds together force $R \leq C + o(1)$ for reliable communication.

Fano's inequality is one of the most versatile tools in information theory — it appears not just here but in statistical estimation, learning theory, and communication complexity. Keep it in mind.
