# Chapter 19 — Network Information Theory

> *Shannon's theory covers one sender and one receiver. The real world has networks. Network information theory asks: what are the capacity limits of multi-user communication systems?*

**Prerequisites:** Chapter 16 (classical information theory, channel capacity).

**What this chapter builds:** Multiple access channels; broadcast channels; the Slepian-Wolf theorem for distributed source coding; the Wyner-Ziv theorem; relay channels; and information-theoretic security.

---

## 19.1 Multiple Access Channels

**Definition 19.1.1.** A *multiple access channel (MAC)* has two senders (with inputs $X_1 \in \mathcal{X}_1$, $X_2 \in \mathcal{X}_2$) and one receiver (output $Y \in \mathcal{Y}$), with channel $p(y|x_1, x_2)$.

**Definition 19.1.2.** The *capacity region* $\mathcal{C}_{\text{MAC}}$ is the closure of achievable rate pairs $(R_1, R_2)$: rates at which senders 1 and 2 can communicate reliably to the receiver simultaneously.

**Theorem 19.1.3 (MAC Capacity Region — Ahlswede, Liao 1971).** The capacity region of the MAC with channel $p(y|x_1,x_2)$ is the convex hull of the union over all product input distributions $p(x_1)p(x_2)$ of the region:
$$\{(R_1, R_2) : R_1 \leq I(X_1;Y|X_2), R_2 \leq I(X_2;Y|X_1), R_1+R_2 \leq I(X_1,X_2;Y)\}$$

**Intuition:** 
- $R_1 \leq I(X_1; Y|X_2)$: sender 1 can communicate at rate equal to the channel capacity *given that the receiver knows sender 2's message* (cooperative bound).
- $R_1 + R_2 \leq I(X_1,X_2; Y)$: total information from both senders is bounded by the channel's mutual information with both inputs.

**Example 19.1.4 (Gaussian MAC).** $Y = X_1 + X_2 + Z$, $Z \sim N(0,1)$, power constraints $E[X_i^2] \leq P_i$. Capacity region:
$$R_1 \leq \frac{1}{2}\log(1+P_1),\ R_2 \leq \frac{1}{2}\log(1+P_2),\ R_1+R_2 \leq \frac{1}{2}\log(1+P_1+P_2).$$

The sum-rate capacity equals the single-user capacity when only one user transmits at the max rate — showing multi-user cooperation is "free" in additive channels.

---

## 19.2 Broadcast Channels

**Definition 19.2.1.** A *broadcast channel (BC)* has one sender (input $X \in \mathcal{X}$) and two receivers (outputs $Y_1 \in \mathcal{Y}_1$, $Y_2 \in \mathcal{Y}_2$), with channel $p(y_1, y_2|x)$.

**Definition 19.2.2.** The *capacity region* consists of pairs $(R_1, R_2)$ where rate $R_i$ is achievable for receiver $i$ simultaneously.

**Theorem 19.2.3 (Degraded BC — Cover, Bergmans 1972).** If $X \to Y_1 \to Y_2$ form a Markov chain ("receiver 1 is less noisy than receiver 2"), the capacity region is:
$$R_1 \leq I(X; Y_1|U), \quad R_2 \leq I(U; Y_2),$$
for all auxiliary random variables $U$ such that $U \to X \to (Y_1, Y_2)$ form a Markov chain.

*The auxiliary variable $U$ represents the "common message" for receiver 2; the remaining information $X|U$ is the "private message" for receiver 1.*

**General BCs:** The capacity region of general (non-degraded) broadcast channels is unknown. Marton's inner bound and the UV outer bound are the best known in general.

---

## 19.3 Distributed Source Coding — Slepian-Wolf

**Problem:** Two correlated sources $X$ and $Y$ encode their data *separately* (no communication between encoders) but send to a *common decoder*. What rates are needed?

**Theorem 19.3.1 (Slepian-Wolf, 1973).** The achievable rate region for distributed lossless coding of correlated sources $(X, Y)$ is:
$$R_X \geq H(X|Y), \quad R_Y \geq H(Y|X), \quad R_X + R_Y \geq H(X,Y).$$

**Remarkable feature:** Encoder $X$ needs only $H(X|Y)$ bits per symbol — the entropy of $X$ given $Y$ — even though the encoder does not know $Y$! The decoder uses both coded streams to reconstruct $(X, Y)$ jointly.

**Proof idea (achievability):** Encoder $X$ partitions sequences $x^n$ into $2^{n H(X|Y)}$ bins (using a random code). Encoder $Y$ partitions $y^n$ into $2^{n H(Y|X)}$ bins. The decoder receives the bin indices and finds the unique jointly typical pair $(x^n, y^n)$ in the intersection of the given bins.

**Example 19.3.2.** If $X = Y$ (perfectly correlated): $H(X|Y) = 0$, so encoder $X$ needs 0 bits. If $X \perp Y$: $H(X|Y) = H(X)$, so each encoder needs the full entropy — no savings.

---

## 19.4 Wyner-Ziv — Lossy Coding with Side Information

**Problem:** Encoder compresses $X$ to rate $R$ bits/sample; decoder has access to correlated side information $Y$ (not available at encoder). What rate $R$ achieves distortion $D$?

**Theorem 19.4.1 (Wyner-Ziv, 1976).** The rate-distortion function with decoder side information is:
$$R_{\text{WZ}}(D) = \min_{p(u|x): E[d(X,g(U,Y))] \leq D} [I(X;U|Y) - I(X;U)].$$

Wait — more cleanly: $R_{\text{WZ}}(D) = \min_{p(\hat{x}|x,u), p(u|x)} I(X;U|Y)$, minimized over choices of $U$ and reconstructor $g: U \times Y \to \hat{X}$ achieving distortion $\leq D$.

**Key result:** $R_{\text{WZ}}(D) = R(D|Y)$, the rate-distortion function with side information at *both* encoder and decoder. Side information at the decoder is "as good as" side information at both ends — another surprising result like Slepian-Wolf.

**Applications:** Video compression (side information = adjacent frames), sensor networks, genomics.

---

## 19.5 Relay Channels

**Setup:** Source $X$ sends to destination $Y$ via intermediate relay $Y_r$ (which can also transmit $X_r$ to $Y$). The relay can hear the source and retransmit to help the destination.

**Theorem 19.5.1 (Cover-El Gamal, 1979).** The *decode-and-forward* inner bound: if the relay fully decodes the source message and re-encodes:
$$R \leq \max_{p(x,x_r)} \min\{I(X,X_r; Y), I(X; Y, Y_r | X_r)\}.$$

The *compress-and-forward* inner bound: relay compresses its observation and sends the compressed version:
$$R \leq \max_{p(x)p(x_r)p(\hat{y}_r|y_r,x_r)} I(X;Y,\hat{Y}_r|X_r) \text{ subject to } I(X_r;Y) \geq I(Y_r; \hat{Y}_r|X_r, Y).$$

The capacity of the relay channel is not known in general — one of the major open problems in network information theory.

---

## 19.6 Information-Theoretic Security

### 19.6.1 Perfect Secrecy

**Definition 19.6.1 (Shannon's Perfect Secrecy).** An encryption scheme $(M, K, C)$ (plaintext, key, ciphertext) achieves *perfect secrecy* if $I(M; C) = 0$: the ciphertext reveals no information about the message.

**Theorem 19.6.2 (Shannon 1949).** Perfect secrecy requires $H(K) \geq H(M)$ — the key must be at least as random as the message. The one-time pad achieves perfect secrecy with equality.

### 19.6.2 The Wiretap Channel

**Setup (Wyner 1975):** Sender $X$ communicates with legitimate receiver $Y$ while an eavesdropper observes $Z$ (a degraded version of $Y$). What rate can be achieved with *information-theoretic security*?

**Definition 19.6.3.** The *secrecy rate* $R_s$ is achievable if there exist codes with: (1) reliable decoding by legitimate receiver at rate $R_s$; (2) eavesdropper's information about the message $\to 0$.

**Theorem 19.6.4 (Wyner Wiretap Capacity).** For the degraded wiretap channel ($X \to Y \to Z$):
$$C_s = \max_{p(x)} [I(X;Y) - I(X;Z)].$$

Positive secrecy rate is achievable iff the legitimate channel is less noisy than the eavesdropper's channel.

**Theorem 19.6.5 (Gaussian Wiretap).** For $Y = X + N_Y$ and $Z = X + N_Z$ with $\text{Var}(N_Y) < \text{Var}(N_Z)$ (eavesdropper noisier):
$$C_s = \frac{1}{2}\log\left(1 + \frac{P}{N_Y}\right) - \frac{1}{2}\log\left(1 + \frac{P}{N_Z}\right) > 0.$$

---

## 19.7 Secret Key Agreement

**Problem:** Two parties observe $(X^n, Y^n)$ (a correlated source) with an eavesdropper observing $Z^n$, correlated with both. They communicate over a public channel (visible to the eavesdropper). How many secret key bits can they generate?

**Theorem 19.7.1 (Maurer 1993, Ahlswede-Csiszár 1993).** The *secret key capacity* is:
$$C_K = I(X;Y) - I(X;Z) \quad \text{(one-way communication)}$$
or more generally (with two-way communication):
$$C_K = \sup [\text{agreement}] - [\text{eavesdropper's information}].$$

The full characterization with two-way communication is still open in general.

---

## Exercises

**Exercise 19.1.** Compute the capacity region of the Gaussian MAC with $P_1 = P_2 = 1$ and $N = 1$ (unit noise). Draw the region. What happens as $P_i \to \infty$?

**Exercise 19.2.** State and prove the Slepian-Wolf converse: show that $R_X < H(X|Y)$ or $R_Y < H(Y|X)$ or $R_X + R_Y < H(X,Y)$ leads to positive probability of error.

**Exercise 19.3.** For binary symmetric correlated sources: $X \sim \text{Bernoulli}(1/2)$ and $Y = X \oplus E$ where $E \sim \text{Bernoulli}(\epsilon)$ independently. Compute $H(X|Y)$, $H(Y|X)$, and $H(X,Y)$. Draw the Slepian-Wolf rate region.

**Exercise 19.4.** (Wyner-Ziv) For Gaussian sources $X \sim N(0, \sigma^2)$ and side information $Y = X + Z$, $Z \sim N(0, N)$ (independent of $X$), compute the Wyner-Ziv rate-distortion function for squared-error distortion. Verify that $R_{\text{WZ}}(D) = R(D|Y)$.

**Exercise 19.5.** (Wiretap) For the binary wiretap channel where $Y = X \oplus N_Y$, $Z = X \oplus N_Y \oplus N_Z$ (Bernoulli noise): compute the secrecy capacity $C_s$ when $P(N_Y = 1) = \epsilon_Y < \epsilon_Z = P(N_Z = 1)$.

---

## Chapter Notes

Network information theory is covered comprehensively in El Gamal and Kim's *Network Information Theory* (Cambridge, 2011) — the definitive modern reference. The subject is large and many problems remain open.

Key historical papers: Slepian-Wolf (1973) in *Bell System Technical Journal*; Wyner's wiretap paper (1975) in the same journal; Cover-El Gamal relay channel (1979) in *IEEE Transactions on Information Theory*.

For the open problems: the capacity of the relay channel, the capacity region of the interference channel (only the weak and strong interference regimes are known), and the capacity of multi-hop networks are all unsolved.
