# 19.6 Information-Theoretic Security

Most cryptography is *computational*: security relies on the assumed hardness of mathematical problems like factoring or discrete logarithm. An adversary with enough computational power could break such systems. Information-theoretic security is fundamentally different: it provides security against adversaries with *unlimited* computational power. The information simply is not there to be extracted, no matter how clever the attacker.

Shannon himself laid the foundations in 1949, the year after his communication paper.

## 19.6.1 Perfect Secrecy

**Definition 19.6.1 (Shannon's Perfect Secrecy).** An encryption scheme $(M, K, C)$ (plaintext, key, ciphertext) achieves *perfect secrecy* if $I(M; C) = 0$: the ciphertext reveals no information about the message.

Mutual information zero means that, no matter what ciphertext you observe, you learn absolutely nothing about the plaintext. The ciphertext is statistically independent of the message. Perfect secrecy is the strongest possible security guarantee.

The cost:

**Theorem 19.6.2 (Shannon 1949).** Perfect secrecy requires $H(K) \geq H(M)$ — the key must be at least as random as the message. The one-time pad achieves perfect secrecy with equality.

The one-time pad: to encrypt a message $M \in \{0,1\}^n$, draw a key $K \in \{0,1\}^n$ uniformly at random and set $C = M \oplus K$. The ciphertext $C$ is uniform on $\{0,1\}^n$ regardless of $M$, so $I(M; C) = 0$. Perfect secrecy, at the cost of a key as long as the message.

Shannon's theorem says this cost is unavoidable: you cannot do better. No encryption scheme with a shorter key can achieve perfect secrecy. This is why one-time pads are not practical for most uses — generating and securely distributing fresh random key material for every message is prohibitively expensive.

## 19.6.2 The Wiretap Channel

In 1975, Aaron Wyner (the same Wyner as in Wyner-Ziv) introduced the wiretap channel model: the sender communicates with a legitimate receiver over a "good" channel while an eavesdropper observes the transmission over a "bad" (noisier) channel. The question: can the sender achieve reliable communication with the legitimate receiver while keeping the eavesdropper ignorant?

**Setup (Wyner 1975):** Sender $X$ communicates with legitimate receiver $Y$ while an eavesdropper observes $Z$ (a degraded version of $Y$). The channel is $X \to Y \to Z$ (Markov chain — eavesdropper gets a noisier version).

**Definition 19.6.3 (Secrecy Rate).** The *secrecy rate* $R_s$ is achievable if there exist codes with: (1) reliable decoding by the legitimate receiver at rate $R_s$; (2) the eavesdropper's information about the message goes to zero.

**Theorem 19.6.4 (Wyner Wiretap Capacity).** For the degraded wiretap channel ($X \to Y \to Z$):
$$C_s = \max_{p(x)} [I(X; Y) - I(X; Z)].$$

The secrecy capacity is the difference between the legitimate channel's mutual information and the eavesdropper's mutual information. Positive secrecy rate is achievable *iff* the legitimate channel is less noisy than the eavesdropper's channel — $I(X;Y) > I(X;Z)$.

**Theorem 19.6.5 (Gaussian Wiretap).** For $Y = X + N_Y$ and $Z = X + N_Z$ with $\text{Var}(N_Y) < \text{Var}(N_Z)$ (eavesdropper noisier):
$$C_s = \frac{1}{2}\log\left(1 + \frac{P}{N_Y}\right) - \frac{1}{2}\log\left(1 + \frac{P}{N_Z}\right) > 0.$$

Wyner's result was surprising because it showed that physical layer security is possible — noise in the eavesdropper's channel can be exploited to achieve provably secure communication without any shared secret key. The sender uses a stochastic encoding scheme that injects additional randomness to "confuse" the eavesdropper without reducing the legitimate receiver's ability to decode.

This is the theoretical foundation of *physical layer security* in wireless communications — an active research area where the natural noise in wireless channels is treated as a resource rather than a nuisance.
