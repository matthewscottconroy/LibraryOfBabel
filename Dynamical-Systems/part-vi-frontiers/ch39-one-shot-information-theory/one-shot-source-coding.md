# 39.3 One-Shot Source Coding

Shannon's source coding theorem says: you need at least $H(X)$ bits per symbol to represent a source $X$, asymptotically. In one shot, the right quantity is the smooth min-entropy.

**Theorem 39.3.1 (One-Shot Source Coding).** Given a classical source $X$ with distribution $P$, a code of length $\ell$ bits achieves error $\varepsilon$ iff:
$$\ell \geq H_{\min}^\varepsilon(X).$$

The optimal code length (for given error tolerance $\varepsilon$) is exactly the smooth min-entropy.

This is a tight, operational characterization. You can compress $X$ to $H_{\min}^\varepsilon(X)$ bits with error at most $\varepsilon$, and you cannot do better. Compare with Shannon's theorem: to compress $n$ i.i.d. copies of $X$ with error $\varepsilon$, you need $\approx nH(X)$ bits. The one-shot version gives $H_{\min}^\varepsilon(X^n)$, which for large $n$ equals $nH(X) + O(\sqrt{n})$.

For quantum sources:

**Theorem 39.3.2 (One-Shot Quantum Source Coding).** For a quantum source $\rho$, the optimal number of qubits $q$ for compression with fidelity $\geq 1 - \varepsilon$ satisfies:
$$q \approx H_{\min}^\varepsilon(\rho).$$

The quantum one-shot source coding theorem is the analogue of Schumacher compression (Chapter 21) for finite resources. Schumacher compression achieves $S(\rho)$ qubits per copy in the limit; one-shot compression achieves $H_{\min}^\varepsilon(\rho)$ qubits for a single copy with error $\varepsilon$.

**Second-Order Asymptotics:** For i.i.d. $\rho^{\otimes n}$ with error $\varepsilon$:
$$q^* = nS(\rho) + \sqrt{n \cdot V(\rho)} \cdot \Phi^{-1}(\varepsilon) + O(\log n),$$
where $V(\rho) = \text{Var}(-\log\rho)$ is the *entropy variance* and $\Phi^{-1}$ is the inverse Gaussian CDF. The $\sqrt{n}$ correction is the "dispersion" term.

The dispersion $V(\rho)$ measures how concentrated the eigenvalues of $-\log\rho$ are around their mean $S(\rho)$. If $V(\rho) = 0$, the source is "flat" (all eigenvalues equal) and no backoff from $nS(\rho)$ is needed. If $V(\rho)$ is large, you need a significant backoff for finite $n$.

This second-order asymptotic formula is the most precise characterization of finite-blocklength quantum compression. The $\sqrt{n}$ term is the penalty you pay for not taking $n \to \infty$.
