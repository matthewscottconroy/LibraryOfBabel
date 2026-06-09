# 41.3 Statistical Model of the Collatz Map

Even if we can't prove the conjecture, we can understand it probabilistically. If the successive bits of a Collatz orbit are "essentially random" — if the parity pattern looks like i.i.d. coin flips — then we can model the orbit as a random walk and predict its behavior.

**The Heuristic:** Consider the orbit $n, C(n), C^2(n), \ldots$. If consecutive bits of $n$ are "random" (i.i.d. Bernoulli(1/2)):
- Each step: halve with probability 1/2 (if even), apply $3n+1$ with probability 1/2 (if odd)
- After one step: $E[C(n)] \approx n/2 \cdot 1/2 + (3n+1)/2 \cdot 1/2 \approx n \cdot 3/4$

Since $3/4 < 1$: each step reduces $n$ by factor $3/4$ on average. So orbits should reach 1 in $O(\log n)$ steps.

The key number is $3/4$. Each step multiplies $n$ by $3/4$ on average — contracting. After $k$ steps, $E[C^k(n)] \approx n \cdot (3/4)^k$. For this to reach 1, you need $k \approx \log n / \log(4/3)$.

**Definition 41.3.1.** The *stochastic process* model replaces $C$ by a random walk on $(0,\infty)$: $X_0 = \log n$, $X_{t+1} = X_t + \varepsilon_t$ where $\varepsilon_t \in \{-\log 2, +\log(3/2)\}$ i.i.d. with equal probability.

Since $E[\varepsilon_t] = \frac{1}{2}(-\log 2 + \log(3/2)) = \frac{1}{2}\log(3/4) < 0$, the random walk has a negative drift: $X_t \to -\infty$ a.s. This supports the Collatz conjecture probabilistically.

The random walk model is the cleanest way to see why the conjecture should be true. The drift is negative — $\frac{1}{2}\log(3/4) = -\frac{1}{2}\log(4/3) \approx -0.144$ per step. A random walk with negative drift goes to $-\infty$ almost surely by the law of large numbers. So orbits of the stochastic model always reach 0 (corresponding to reaching 1 in the original system).

**Theorem 41.3.2 (Wirsching, 1998).** Under the stochastic model, the time to reach 0 (corresponding to reaching 1) is:
$$\tau \approx \frac{\log n}{|\log(3/4)|/2} = \frac{2\log n}{\log(4/3)}.$$

This matches the observed empirical stopping times.

The critical gap: the stochastic model treats the parity sequence as i.i.d. random. In reality, the parity sequence of a Collatz orbit is completely deterministic — it's computable from $n$. Whether real Collatz orbits behave like the random model is exactly what's being asked.
