# 12.6 The Zeta Function

One of the most beautiful structures in dynamical systems is the counting of periodic orbits. For a dynamical system, the periodic points of period $n$ — the fixed points of $f^n$ — are natural invariants. How many are there? How do their counts grow? Can we encode all these counts in a single function?

For subshifts, the answer is yes, and the encoding is the *dynamical zeta function* — a formal power series whose coefficients are (scaled) periodic orbit counts.

**Definition 12.6.1.** The *dynamical zeta function* of $(X, \sigma)$ is:
$$\zeta_X(t) = \exp\left(\sum_{n=1}^\infty \frac{|\text{Fix}(\sigma^n)|}{n} t^n\right),$$
where $|\text{Fix}(\sigma^n)| = |\{x \in X : \sigma^n(x) = x\}|$ counts the periodic points of period $n$ (or dividing $n$).

The exponential form might look strange, but it is the right one: it has the property that the poles and zeros of $\zeta_X(t)$ correspond to the eigenvalues of the transition matrix. For SFTs, this gives a remarkable rationality result:

**Theorem 12.6.2 (Rationality for SFTs).** For an irreducible SFT with transition matrix $A$:
$$\zeta_{X_A}(t) = \frac{1}{\det(I - tA)}.$$

*Proof:* The number of periodic points of period $n$ (i.e., sequences $x$ with $\sigma^n(x) = x$) equals the number of closed paths of length $n$ in the graph, which is $\text{tr}(A^n) = \sum_i \lambda_i^n$. Now sum the series:
$$\sum_{n \geq 1} \frac{\text{tr}(A^n)}{n} t^n = \sum_i \sum_{n \geq 1} \frac{\lambda_i^n t^n}{n} = -\sum_i \log(1 - \lambda_i t) = -\log\prod_i (1 - \lambda_i t) = -\log\det(I - tA).$$
Exponentiating gives $\zeta(t) = 1/\det(I - tA)$.

What this is saying is: the zeta function of an SFT is a rational function. Its poles are at $t = 1/\lambda_i$ for the eigenvalues $\lambda_i$ of $A$. The dominant pole — the one closest to 0 — is at $t = 1/\lambda_{\text{PF}}$, confirming that the periodic orbit growth rate is $\lambda_{\text{PF}}$ — the same as the topological entropy. Periodic orbit growth and topological entropy are not independent; they are the same underlying phenomenon.

**Example 12.6.3.** For the golden mean shift with $A = \begin{pmatrix}1&1\\1&0\end{pmatrix}$:
$$\det(I - tA) = \det\begin{pmatrix}1-t & -t \\ -t & 1\end{pmatrix} = (1-t)(1) - (-t)(-t) = 1 - t - t^2.$$
So $\zeta_{\text{GM}}(t) = \frac{1}{1-t-t^2}$.

Expanding as a power series: $\zeta_{\text{GM}}(t) = 1 + t + 2t^2 + 3t^3 + 5t^4 + \cdots$, where the coefficients are again the Fibonacci numbers. Indeed, the number of closed paths of length $n$ in the golden mean graph satisfies the Fibonacci recurrence.

The rationality of the zeta function is one of the features that distinguishes SFTs (and sofic shifts) from more general dynamical systems. For smooth diffeomorphisms, the zeta function can be transcendental. For systems with a Markov partition — hyperbolic systems, which have SFT models — rationality follows from the SFT structure.
