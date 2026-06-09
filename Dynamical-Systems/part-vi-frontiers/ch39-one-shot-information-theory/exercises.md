# Exercises — Chapter 39

These exercises develop facility with smooth entropy computations, the Leftover Hash Lemma, and the second-order coding bounds. Exercise 39.3 requires numerical computation.

---

**Exercise 39.1.** Compute $H_{\min}(X)$ and $H_{\max}(X)$ for $X$ with distribution $P(1) = 1/2$, $P(2) = 1/4$, $P(3) = 1/8$, $P(4) = 1/8$. How do they compare to $H(X)$?

**Exercise 39.2.** (Smooth Entropy) For $X$ with $P(1) = 1/2$, $P(2) = 1/2 - \varepsilon$, $P(3) = \varepsilon$: compute $H_{\min}^\varepsilon(X)$ and show it is approximately $1$ (one bit) for small $\varepsilon$.

**Exercise 39.3.** (Second-Order Coding) For the binary symmetric channel with crossover probability $p = 0.1$:
- Compute capacity $C = 1 - h_b(p)$
- Compute dispersion $V = p(1-p)(\log\frac{1-p}{p})^2$
- For blocklength $n = 100$ and error $\varepsilon = 0.01$, find the second-order approximation to the maximum rate

**Exercise 39.4.** Verify the Leftover Hash Lemma for the following scenario: $X$ is uniform on $\{0,1\}^k$ (so $H_{\min}(X) = k$). A $2$-universal hash function maps $X$ to $\{0,1\}^\ell$ with $\ell = k - 2$. Show the output is $1/2$-close to uniform (by direct computation for small $k$).
