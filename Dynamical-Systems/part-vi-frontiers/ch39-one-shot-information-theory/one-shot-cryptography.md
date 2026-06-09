# 39.5 One-Shot Cryptography

## 39.5.1 Randomness Extraction

In cryptography, you often have a source of "weak randomness" — a random variable $X$ that isn't uniformly distributed but has some min-entropy $H_{\min}(X) \geq k$. The goal of a *randomness extractor* is to produce $\ell$ bits that are close to uniform, using only a short random seed.

The Leftover Hash Lemma gives the fundamental limit for this.

**Theorem 39.5.1 (Leftover Hash Lemma — Bennett-Brassard-Crépeau-Maurer, 1995).** If $X$ is a classical random variable with $H_{\min}(X) \geq k$ and $f: \mathcal{X} \to \{0,1\}^\ell$ is a $2$-universal hash function with $\ell \leq k - 2\log(1/\varepsilon)$, then $f(X)$ is $\varepsilon$-close to uniform:
$$\|P_{f(X)} - U_\ell\|_1 \leq \varepsilon.$$

A 2-universal hash function is a family $\{f_s\}$ parametrized by a short seed $s$ such that for any two distinct inputs $x \neq x'$, the probability that $f_s(x) = f_s(x')$ is at most $1/|\{0,1\}^\ell|$ when $s$ is chosen uniformly. You can extract $k - 2\log(1/\varepsilon)$ bits of randomness from $k$ bits of min-entropy, with error $\varepsilon$.

The quantum version handles the case where an adversary has quantum side information — a quantum memory correlated with $X$:

**Theorem 39.5.2 (Quantum Leftover Hash Lemma — Renner, 2005).** For a quantum side information state $\rho_{XE}$ (where $E$ is the adversary's quantum system), a $2$-universal hash function $f$ extracts $\ell \leq H_{\min}^\varepsilon(X|E)_\rho$ bits that are uniform and independent of $E$.

## 39.5.2 Privacy Amplification

**Definition 39.5.3.** In key agreement: Alice and Bob share a weakly random key $X$ (with partial adversary knowledge $E$). *Privacy amplification* extracts a shorter, fully secret key from $X$.

**Theorem 39.5.4 (Privacy Amplification in One Shot).** The maximum length of a secret key extractable from $\rho_{XE}$ using a public random function is:
$$\ell^* = H_{\min}^\varepsilon(X|E)_\rho - 2\log(1/\delta),$$
with security parameter $\delta$ (probability of failure).

The one-shot smooth min-entropy is the right operational quantity here: it's the information in $X$ that's inaccessible to the adversary $E$, even if $E$ has a quantum memory. You can extract exactly $H_{\min}^\varepsilon(X|E)$ bits of secret key, and not more.

This is foundational for quantum key distribution (QKD). In QKD protocols like BB84, the raw key shared by Alice and Bob after quantum transmission has some adversary information leakage. Privacy amplification using hashing removes that leakage, producing a fully secret key. The Quantum Leftover Hash Lemma guarantees the security of this step, with the smooth min-entropy giving the key rate.
