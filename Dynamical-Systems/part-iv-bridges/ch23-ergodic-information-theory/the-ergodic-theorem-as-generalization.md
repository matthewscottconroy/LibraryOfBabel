# 23.4 The Ergodic Theorem as a Generalization

It's worth stepping back and seeing exactly where the SMB theorem sits in the hierarchy of limit theorems. This hierarchy is one of the cleanest organizational structures in probability and ergodic theory — each level generalizes the previous one, and the SMB theorem is the natural ergodic-theoretic culmination.

The hierarchy:

- **LLN:** $(1/n)\sum X_i \to E[X]$ for i.i.d. (convergence in probability)
- **SLLN:** a.s. convergence
- **AEP:** $(1/n)\log p(X_1,\ldots,X_n) \to -h$ for i.i.d. processes (LLN for log-probabilities)
- **SMB:** a.s. convergence for stationary ergodic processes (Birkhoff for log-probabilities)

The SMB theorem is: *Birkhoff's ergodic theorem applied to the information function*.

Let's make this precise. The information function is $\phi(x) = -\log \mu(\xi(x))$, where $\xi(x)$ is the partition atom containing $x$. For an i.i.d. process, $-\frac{1}{n}\log p(X_1, \ldots, X_n) = \frac{1}{n}\sum_{k=0}^{n-1} (-\log p(X_k)) = \frac{1}{n}\sum_{k=0}^{n-1} \phi(f^k(x))$. This is a Birkhoff sum, and by Birkhoff's theorem it converges a.s. to $\int \phi \, d\mu = E[-\log p(X_1)] = h$.

For a general stationary ergodic process, the information function becomes a conditional information function — the information in the current symbol given the past — but the same Birkhoff argument applies, once you handle the growing conditioning carefully (this requires the martingale convergence theorem, as in Breiman's proof).

This hierarchy has a beautiful structure: each generalization removes an assumption.

LLN → SLLN: removes the requirement of convergence in probability, strengthens to a.s.

AEP → SMB: removes the independence assumption, allowing arbitrary stationary ergodic dependencies.

What can't be removed? Ergodicity is necessary: for a non-ergodic process, the time averages converge to random variables (integrals with respect to the ergodic components), not to constants. For source coding, ergodicity is the assumption that the source has a well-defined entropy rate — without it, the compression rate isn't a single number.

The hierarchy also shows where information theory and ergodic theory are doing the same work. The AEP is the law of large numbers for information; the SMB theorem is Birkhoff's ergodic theorem for information. The law of large numbers and the ergodic theorem are the same theorem in different settings — and so are the AEP and the SMB theorem.

Shannon and Kolmogorov never collaborated, but their contributions illuminate each other across this hierarchy. Shannon's AEP (1948) is the information-theoretic LLN; Kolmogorov and Sinai's entropy (1958–59) provides the ergodic-theoretic quantity that the SMB theorem says is the limit. The SMB theorem — proved by Breiman in 1957, between these two milestones — is the bridge.
