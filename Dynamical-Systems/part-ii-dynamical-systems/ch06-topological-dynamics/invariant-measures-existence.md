# 6.8 Invariant Measures — Existence

We've been studying dynamics purely topologically — no measures, no probabilities. But a central question in applications is: does the system have a natural probability distribution that it preserves? If you pick a "typical" initial condition and let it evolve, what distribution does it settle into?

The answer begins with the question of existence: does any invariant measure exist?

**Definition 6.8.1.** A Borel probability measure $\mu$ on $X$ is *$f$-invariant* if $\mu(f^{-1}(A)) = \mu(A)$ for all Borel $A$, equivalently $\int \varphi \circ f\,d\mu = \int \varphi\,d\mu$ for all $\varphi \in C(X)$.

Invariance says: if you observe the system at time $n+1$, the distribution of states is the same as at time $n$. The measure is a statistical equilibrium — not a fixed point of the dynamics, but a fixed point of the dynamics-on-distributions.

---

## The Krylov-Bogoliubov Theorem

The key theorem is a 1937 result by Nikolai Krylov and Nikolai Bogoliubov. Its proof is a masterpiece of functional analysis, using only compactness and a simple averaging argument.

**Theorem 6.8.2 (Krylov-Bogoliubov).** Every continuous map $f: X \to X$ on a compact metrizable space $X$ has at least one invariant Borel probability measure.

*(proof)* Fix any $\mu_0$ (e.g., a Dirac mass $\delta_{x_0}$). Consider the Cesàro averages $\mu_N = \frac{1}{N}\sum_{n=0}^{N-1} f^n_* \mu_0$. By the Prokhorov / Arzelà-Ascoli argument, the sequence $(\mu_N)$ is tight (since $X$ is compact), so a subsequence converges weakly to some $\mu$. One checks: for $\varphi \in C(X)$:
$$\int \varphi \circ f\,d\mu_N - \int \varphi\,d\mu_N = \frac{1}{N}\left(\int \varphi \circ f^N\,d\mu_0 - \int \varphi\,d\mu_0\right) \to 0.$$
Taking the limit, $\int \varphi \circ f\,d\mu = \int \varphi\,d\mu$, so $\mu$ is $f$-invariant.

The proof is a time average argument in disguise. You pick any starting measure, average it along the orbit of the dynamics, and pass to a subsequential limit. The key observation is that the Cesàro average is "almost invariant" — the error is controlled by $1/N$ times a bounded quantity, which goes to zero. Compactness ensures the subsequential limit exists. The result is an invariant measure.

**Remark 6.8.3.** The Krylov-Bogoliubov theorem guarantees existence but not uniqueness. Multiple invariant measures can coexist. A system with a *unique* invariant measure is called *uniquely ergodic*.

Existence without uniqueness is common. The full shift on $\{0,1\}^{\mathbb N}$ has uncountably many invariant measures — one for each ergodic probability on the system, and there are many. The question of which invariant measure is "physically relevant" is addressed in Chapter 9 with the theory of SRB measures.

---

## Unique Ergodicity

When the invariant measure is unique, something wonderful happens: time averages converge *uniformly*, not just almost surely.

**Definition 6.8.4.** $f: X \to X$ is *uniquely ergodic* if it has a unique invariant probability measure.

**Theorem 6.8.5 (Weyl, Oxtoby).** $f$ is uniquely ergodic if and only if for every $\varphi \in C(X)$, the averages $\frac{1}{N}\sum_{n=0}^{N-1} \varphi(f^n(x))$ converge uniformly in $x \in X$ to the constant $\int \varphi\,d\mu$.

This is a strong theorem. Uniform convergence means the time average converges to the space average at every starting point, with the same rate. There's no exceptional set, no "bad" initial conditions. The system is perfectly statistically homogeneous.

**Example 6.8.6.** Every irrational rotation $R_\alpha$ is uniquely ergodic (with Lebesgue measure as the unique invariant measure). The convergence $\frac{1}{N}\sum_{n=0}^{N-1} e^{2\pi i k (x + n\alpha)} \to 0$ for $k \neq 0$ (Weyl's theorem) establishes equidistribution.

Weyl proved his equidistribution theorem in 1916. It says: the orbit of any point under an irrational rotation equidistributes modulo 1 — the fraction of time spent in any interval equals the length of that interval. This was a foundational result in analytic number theory and is the prototype for all ergodic theorems.

The Krylov-Bogoliubov theorem has done its job: it guarantees an invariant measure exists. Chapter 7 will add the ergodic hypothesis and extract the full power of these measures.
