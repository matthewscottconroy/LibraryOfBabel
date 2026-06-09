# 39.4 One-Shot Channel Coding

For source coding, the smooth min-entropy gives the one-shot bound. For channel coding — how many bits can you transmit with one channel use? — the relevant quantity is the hypothesis testing relative entropy.

**Theorem 39.4.1 (One-Shot Channel Coding — Polyanskiy-Poor-Verdú, 2010).** For a single use of a channel $W: \mathcal{X} \to \mathcal{Y}$, the maximum number of bits $M$ transmittable with error $\varepsilon$ satisfies:
$$\log M \approx H_{\min}^\varepsilon(Y | X),$$
where the approximation is in terms of the hypothesis testing relative entropy:
$$D_H^\varepsilon(P_{XY} \| P_X \otimes P_Y) = -\log \beta_\varepsilon(P_{XY}, P_X \otimes P_Y).$$

Here $\beta_\varepsilon$ is the minimum type-II error probability when type-I error $\leq \varepsilon$.

The hypothesis testing relative entropy $D_H^\varepsilon$ measures how distinguishable the joint distribution $P_{XY}$ is from the product $P_X \otimes P_Y$ (independence). This is exactly the amount of correlation in the channel output — the relevant quantity for transmission.

For $n$ i.i.d. uses of the channel, the second-order expansion gives:

**Theorem 39.4.2 (Second-Order Channel Coding).** For $n$ i.i.d. uses of channel $W$ with capacity $C$ and dispersion $V$:
$$\log M^*(n, \varepsilon) = nC - \sqrt{nV}\Phi^{-1}(\varepsilon) + O(\log n).$$

The $-\sqrt{nV}\Phi^{-1}(\varepsilon)$ term is the *backoff from capacity* due to finite blocklength.

The dispersion $V$ of a channel is $V = \text{Var}_{(X,Y) \sim P_{XY}^*}[\log(P_{XY}(X,Y)/P_X(X)P_Y(Y))]$, the variance of the information density under the capacity-achieving input distribution. A channel with small dispersion can be operated close to capacity even for moderate blocklength; a channel with large dispersion requires significantly longer codes to approach capacity.

This result had enormous practical impact when Polyanskiy, Poor, and Verdú published it in 2010. It explained why practical codes (LDPC codes, turbo codes) with blocklength in the hundreds to thousands perform below capacity: the $\sqrt{n}$ term is not negligible for small $n$. And it gave a tight benchmark for how close to capacity a given code of length $n$ should be.
