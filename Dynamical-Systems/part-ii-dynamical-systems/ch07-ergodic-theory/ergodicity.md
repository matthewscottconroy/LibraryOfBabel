# 7.4 Ergodicity

## Definition and Characterizations

Ergodicity is the irreducibility condition that makes the Birkhoff theorem give its strongest conclusion. Without it, the time averages converge, but to an invariant function that can vary from orbit to orbit — different orbits might have different long-run statistics. With ergodicity, the statistics are the same for almost every orbit: the single number $\int \varphi\,d\mu$.

**Definition 7.4.1.** An MPT $(X, \mathcal{B}, \mu, f)$ is *ergodic* if every $f$-invariant set has measure 0 or 1: $f^{-1}(A) = A$ implies $\mu(A) \in \{0, 1\}$.

What this is really saying: there are no proper invariant "pieces" of positive measure. If the system were not ergodic, you could split $X$ into two invariant pieces, each with positive measure, and the dynamics on each piece would be independent of the dynamics on the other. Ergodicity is the condition that prevents this decomposition — the system is "indecomposable" in the measure-theoretic sense.

**Theorem 7.4.2 (Equivalences for Ergodicity).** The following are equivalent:
1. $(X, \mathcal{B}, \mu, f)$ is ergodic
2. The only $f$-invariant functions in $L^1(\mu)$ are constants a.e.
3. For $\mu$-a.e. $x$, the orbit of $x$ equidistributes: $\frac{1}{N}\sum_{n=0}^{N-1} \varphi(f^n(x)) \to \int \varphi\,d\mu$ for all $\varphi \in L^1$
4. The Koopman operator $U_f$ has $1$ as a simple eigenvalue
5. For all $A, B \in \mathcal{B}$ with positive measure: $\frac{1}{N}\sum_{n=0}^{N-1} \mu(f^{-n}(A) \cap B) \to \mu(A)\mu(B)$ (Cesàro mixing)

These equivalences are worth understanding one by one. Condition (2) says that invariant functions are trivial — which is the functional-analytic reformulation of the set-level condition (1). Condition (3) is the orbit equidistribution statement — what most people think of when they hear "ergodic theorem." Condition (4) is the spectral version: $1$ is an eigenvalue because constant functions are invariant, and simplicity means there's no other invariant direction. Condition (5) is a time-averaged mixing property — on average, the system mixes sets together proportionally to their measures.

---

## Examples

**Example 7.4.3.**
- Irrational rotation $R_\alpha$ is ergodic. Proof: if $f \in L^2$ is $R_\alpha$-invariant, its Fourier coefficients $\hat{f}(k) = \int f(x) e^{-2\pi i kx} dx$ satisfy $\hat{f}(k) = e^{2\pi i k\alpha} \hat{f}(k)$, so $\hat{f}(k) = 0$ for all $k \neq 0$ (since $e^{2\pi i k\alpha} \neq 1$ for $k \neq 0$, $\alpha \notin {\mathbb Q}$). Hence $f$ is constant.
- Doubling map $f(x) = 2x \pmod{1}$ is ergodic. Same Fourier argument: $\hat{f}(k) = \hat{f}(2k)$ for all $k$, so $\hat{f}(k) = \hat{f}(2^n k) \to 0$ for $k \neq 0$.

The Fourier argument for the irrational rotation is slick and instructive. An invariant function must have Fourier coefficients satisfying $\hat{f}(k) = e^{2\pi i k\alpha} \hat{f}(k)$. Since $e^{2\pi i k \alpha} \neq 1$ for $k \neq 0$ (this is exactly the irrationality of $\alpha$), we need $\hat{f}(k) = 0$ for all nonzero $k$. So $f$ is constant.

For the doubling map, the same trick works in reverse: the invariance equation gives $\hat{f}(k) = \hat{f}(2k)$, which forces $\hat{f}(k) = \hat{f}(2^n k) \to 0$ by the Riemann-Lebesgue lemma. So $\hat{f}(k) = 0$ for $k \neq 0$, and $f$ is constant.

---

## Ergodic Decomposition

Not every system is ergodic, but every system contains ergodic pieces.

**Theorem 7.4.4 (Ergodic Decomposition).** Every MPT $(X, \mathcal{B}, \mu, f)$ decomposes as an integral over ergodic measures: $\mu = \int \mu_x\,d\mu(x)$, where each $\mu_x$ is an ergodic $f$-invariant measure and $\mu_x = \mu_y$ whenever $y$ is in the orbit of $x$.

The ergodic decomposition says: even if your system isn't ergodic, you can always break it into ergodic components. Each component is an irreducible piece, and the full measure is a convex combination (actually an integral) of these ergodic pieces. This is the analogue of the topological result about minimal subsystems.

With ergodicity in hand, the full power of the Birkhoff theorem is available: time averages equal space averages, for almost every starting point. The next section asks for even more: not just that averages converge, but that correlations decay.
