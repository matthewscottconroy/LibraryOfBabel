# 31.1 Equidistribution and Weyl's Theorem

The most basic question of equidistribution: if you take the fractional parts of $\alpha, 2\alpha, 3\alpha, \ldots$ for an irrational number $\alpha$, do they fill up the interval $[0,1)$ uniformly? Intuitively yes — the fractional parts should be "spread out" because $\alpha$ is not rational. The precise version of this intuition is Weyl's theorem, proved in 1916, and the ergodic proof (via Birkhoff's theorem applied to irrational rotations) is one of the most satisfying proofs in the subject.

**Definition 31.1.1.** A sequence $(x_n)$ in $[0,1)$ is *equidistributed* (or *uniformly distributed mod 1*) if for every subinterval $[a,b) \subseteq [0,1)$:
$$\lim_{N\to\infty} \frac{1}{N}\#\{n \leq N : x_n \in [a,b)\} = b - a.$$

Equidistribution says the sequence visits each part of $[0,1)$ proportionally to its length — exactly as if the sequence were drawn uniformly at random. But these sequences are deterministic. Equidistribution is a deterministic shadow of random-looking behavior.

**Theorem 31.1.2 (Weyl's Equidistribution Theorem, 1916).** The sequence $(n\alpha \pmod 1)$ is equidistributed for any irrational $\alpha$.

*(proof)* By the Weyl criterion: $(x_n)$ is equidistributed iff for all $k \neq 0$: $\frac{1}{N}\sum_{n=1}^N e^{2\pi ikx_n} \to 0$. For $x_n = n\alpha$:
$$\frac{1}{N}\sum_{n=1}^N e^{2\pi ikn\alpha} = \frac{1}{N}\cdot\frac{e^{2\pi ik(N+1)\alpha}-e^{2\pi ik\alpha}}{e^{2\pi ik\alpha}-1} \to 0$$
since $e^{2\pi ik\alpha} \neq 1$ for irrational $\alpha$.

The Weyl criterion is the Fourier-analytic characterization of equidistribution: a sequence is equidistributed if and only if all nontrivial Fourier modes of its empirical distribution vanish in the limit. For the rotation sequence $n\alpha$, each Fourier mode is a geometric series that cancels precisely because $e^{2\pi ik\alpha} \neq 1$.

**Ergodic Proof:** Equidistribution of $(n\alpha)$ is exactly Birkhoff's theorem for the rotation $R_\alpha: x \mapsto x + \alpha \pmod 1$ — the time average of the indicator $\mathbf{1}_{[a,b)}$ converges to the space average $b-a$ for Lebesgue-a.e. starting point. For $R_\alpha$ with irrational $\alpha$, the system is uniquely ergodic (Lebesgue is the unique invariant measure), so the convergence holds for *all* starting points.

The ergodic proof is more powerful than the Fourier proof: unique ergodicity gives equidistribution starting from every point, not just a.e. This is a strictly stronger statement. And the unique ergodicity of irrational rotations — every orbit is equidistributed — follows from minimality (every orbit is dense) plus the fact that the only invariant measure for a minimal equicontinuous system is Haar measure.

Weyl also proved equidistribution for polynomial sequences:

**Theorem 31.1.3 (Weyl Polynomial Equidistribution).** For any polynomial $p(n) = \alpha_d n^d + \cdots + \alpha_0$ with at least one irrational non-constant coefficient, the sequence $(p(n) \pmod 1)$ is equidistributed.

The polynomial case requires more work — the Weyl criterion still applies, but bounding the exponential sums for polynomial sequences requires van der Corput differencing, a technique that repeatedly reduces the degree of the polynomial by differencing, until reaching the linear case where Weyl's original argument applies.

Polynomial equidistribution connects to the ergodic theory of nilpotent groups, which is the setting for Furstenberg's multiple recurrence theorem and the Green-Tao theorem. We'll pick up that thread in Section 31.2.
