# Exercises — Chapter 16

The following exercises develop the main ideas of the chapter. Work through them in order if you can: each one builds intuition that the next one uses.

**Exercise 16.1.** Compute $H(X)$ for $X$ with $P(X = 0) = 1/2$, $P(X = 1) = 1/4$, $P(X = 2) = 1/4$. Build the Huffman code and verify $L(C^*) = H(X)$ in this case.

**Exercise 16.2.** Prove that for the BSC with flip probability $\epsilon$: $C = 1 - H(\epsilon)$. (*Hint:* $I(X;Y) = H(Y) - H(Y|X) = H(Y) - H(\epsilon)$; maximize $H(Y)$ over input distributions.)

**Exercise 16.3.** Prove Fano's inequality. (*Hint:* Let $E = \mathbf{1}[M \neq \hat{M}]$. Use the chain rule for entropy: $H(M|\hat{M}) = H(M,E|\hat{M}) = H(E|\hat{M}) + H(M|E,\hat{M})$. Bound each term.)

**Exercise 16.4.** (AEP) Let $X_1, X_2, \ldots$ be i.i.d. $\text{Bernoulli}(1/3)$. Describe the typical set $A_\varepsilon^{(n)}$: what sequences are typical? How many are there (approximately)?

**Exercise 16.5.** (Joint Typicality) State the Joint Typicality Lemma: if $(X^n, Y^n)$ are generated from $p(x,y)$, they are jointly typical; if $\tilde{X}^n$ is independent of $Y^n$ with marginal $p(x)$, the probability that $(\tilde{X}^n, Y^n)$ are jointly typical is $\approx 2^{-nI(X;Y)}$.

**Exercise 16.6.** Prove the rate-distortion lower bound $R(D) \geq \max_{p(\hat{x})} [H(\hat{X}) - h_b(D)] - \log|\hat{\mathcal{X}}|$ for the binary source with Hamming distortion.

**Exercise 16.7.** (Connections to Dynamics) For the doubling map $T: x \mapsto 2x \pmod 1$: interpret the $n$-bit binary expansion of $x$ as the result of a source coding scheme. Show the entropy rate of the source is $\log 2$ bits per symbol. How does this connect to $h_\mu(T) = \log 2$?
