# 2.7 The Law of Large Numbers and Its Ergodic Generalization

The law of large numbers is one of the oldest theorems in probability, and it has a beautiful dynamical interpretation. It says that if you repeat an independent experiment many times and average the results, the average converges to the expected value. This is the mathematical foundation of statistics.

But here's the key question for ergodic theory: what if the experiments are *not* independent? What if successive observations are correlated — as they are for any deterministic dynamical system? The Birkhoff Ergodic Theorem (Chapter 7) answers this question, and the law of large numbers is its prototype.

## 2.7.1 Laws of Large Numbers

**Theorem 2.7.1 (Weak Law of Large Numbers).** Let $X_1, X_2, \ldots$ be i.i.d. with $E[X_1] = \mu < \infty$. Then $(X_1 + \cdots + X_n)/n \to \mu$ in probability.

**Theorem 2.7.2 (Strong Law of Large Numbers).** Under the same hypotheses, $(X_1 + \cdots + X_n)/n \to \mu$ almost surely.

*(proof sketch — Etemadi)* Assume $X_n \geq 0$. By Borel-Cantelli applied to $P(X_n > n) < \infty$ (since $E[X_1] < \infty$) and a blocking argument, one shows that the truncated sums converge.

What these theorems are really saying: time averages converge to the "true" expected value, if the observations are independent. The weak law says convergence in probability — most of the time, the average is close to $\mu$. The strong law says convergence almost surely — with probability 1, the average eventually settles at $\mu$.

The distinction between weak and strong is subtle. Almost sure convergence implies convergence in probability, but not conversely. In practice, the strong law is the one that matters for most applications.

**The Ergodic Perspective.** The SLLN applies to *independent* random variables — each $X_i$ has no relationship to the others. In a dynamical system, the observations $X_n = f \circ T^n$ for some function $f$ and map $T$ are highly correlated: knowing $X_n$ gives information about $X_{n+1}$ because $T^{n+1}(x) = T(T^n(x))$.

The Birkhoff Ergodic Theorem (Chapter 7) generalizes the SLLN to *stationary* sequences, which include all orbits of measure-preserving maps. The conclusion is the same — time averages converge to space averages — but the hypothesis is *ergodicity* (a property of the dynamical system) rather than independence (a property of the random variables).

This is one of the central insights of ergodic theory: independence can be replaced by dynamical mixing.

## 2.7.2 The Central Limit Theorem

**Theorem 2.7.3 (CLT).** Let $X_1, X_2, \ldots$ be i.i.d. with mean $\mu$ and variance $\sigma^2 \in (0, \infty)$. Then
$$\frac{(X_1 + \cdots + X_n) - n\mu}{\sigma\sqrt{n}} \xrightarrow{d} N(0, 1).$$

The CLT says that the fluctuations of the sample mean around the true mean are approximately Gaussian, at scale $1/\sqrt{n}$. This is a universal result — it doesn't depend on the distribution of the $X_i$, only on their mean and variance.

The CLT has dynamical analogues. For *mixing* dynamical systems, time-averages of observables satisfy a CLT with a variance that depends on the correlation structure of the system — specifically, on how fast correlations decay. If the system is strongly mixing (correlations decay exponentially), the variance is finite and the CLT holds. This is a deep theorem in ergodic theory, and it has practical implications for statistical inference from dynamical data.

These probabilistic tools — LLN, CLT, conditional expectation — are the bridge between the measure-theoretic foundations of this chapter and the ergodic theory of Part II.
