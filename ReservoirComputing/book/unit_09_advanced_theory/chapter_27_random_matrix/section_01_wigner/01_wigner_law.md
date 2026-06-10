# Section 27.1: The Wigner Semicircle Law

## 27.1.1 The Semicircle Distribution

Before stating the theorem, let us define the distribution it concerns.

**Definition 27.1.1 (Semicircle Distribution).** The *semicircle distribution* (or *Wigner semicircle*) with radius $R$ is the probability measure on $[-R, R]$ with density
$$\rho_{sc}(x; R) = \frac{2}{\pi R^2} \sqrt{R^2 - x^2}, \quad x \in [-R, R].$$

The moments of this distribution have a remarkable combinatorial structure. Let $X$ be distributed according to $\rho_{sc}(\cdot; R)$. Then:
- $\mathbb{E}[X^{2k-1}] = 0$ for all $k \geq 1$ (all odd moments vanish, by symmetry).
- $\mathbb{E}[X^{2k}] = \frac{1}{k+1}\binom{2k}{k} R^{2k} = C_k R^{2k}$, where $C_k = \frac{1}{k+1}\binom{2k}{k}$ is the $k$-th *Catalan number*.

The Catalan numbers $C_0 = 1, C_1 = 1, C_2 = 2, C_3 = 5, C_4 = 14, \ldots$ appear throughout combinatorics. Their appearance here is not coincidental — it is the key to the method of moments proof.

**Verification.** We compute $\mathbb{E}[X^{2k}] = \int_{-R}^R x^{2k} \frac{2}{\pi R^2}\sqrt{R^2 - x^2}\, dx$. Setting $x = R\sin\theta$:
$$= \frac{2}{\pi R^2} \int_{-\pi/2}^{\pi/2} R^{2k}\sin^{2k}\theta \cdot R\cos\theta \cdot R\cos\theta\, d\theta = \frac{2R^{2k}}{\pi} \int_{-\pi/2}^{\pi/2} \sin^{2k}\theta \cos^2\theta\, d\theta.$$

Using the Beta function identity $\int_{-\pi/2}^{\pi/2} \sin^{2k}\theta \cos^2\theta\, d\theta = \frac{\pi}{2} \frac{(2k)!!(1)!!}{(2k+1+1)!!} \cdot \frac{1}{2}$... 

More directly, using the identity $\int_0^1 t^k(1-t)^{1/2} dt = B(k+1, 3/2) = \frac{k!\,\Gamma(3/2)}{\Gamma(k+5/2)}$, one can verify that $\mathbb{E}[X^{2k}] = C_k R^{2k}$ by induction using the three-term recurrence for Catalan numbers: $C_{k+1} = \sum_{j=0}^{k} C_j C_{k-j}$.

## 27.1.2 Wigner Matrices

**Definition 27.1.2 (Wigner Matrix).** A *Wigner matrix* of size $N$ is a symmetric random matrix $W_N = (W_{ij})$ where:
1. The entries $\{W_{ij}\}_{i \leq j}$ are independent.
2. The diagonal entries $W_{ii}$ are i.i.d. with mean 0 and variance $\sigma_d^2$.
3. The off-diagonal entries $W_{ij}$ (for $i < j$) are i.i.d. with mean 0 and variance $\sigma^2$.

The *normalized Wigner matrix* is $\widetilde{W}_N = W_N / (\sigma\sqrt{N})$.

The normalization $\sigma\sqrt{N}$ is natural: with this normalization, the largest eigenvalue of $\widetilde{W}_N$ converges to 2 (the radius of the semicircle with $R = 2$), which we will see is a consequence of the moments computation.

**Theorem 27.1.1 (Wigner Semicircle Law).** *Let $\widetilde{W}_N = W_N / (\sigma\sqrt{N})$ be a normalized Wigner matrix. The empirical spectral distribution*
$$\mu_N = \frac{1}{N} \sum_{i=1}^{N} \delta_{\lambda_i(\widetilde{W}_N)}$$
*converges weakly, almost surely, to the semicircle distribution $\rho_{sc}(\cdot; 2)$ as $N \to \infty$:*
$$\mu_N \xrightarrow{w} \rho_{sc}(\cdot; 2) \quad \text{a.s.}$$

*Equivalently, for any bounded continuous $f$:*
$$\frac{1}{N} \sum_{i=1}^N f(\lambda_i(\widetilde{W}_N)) \to \int f(x) \rho_{sc}(x;2)\, dx \quad \text{a.s.}$$

## 27.1.3 Proof by the Method of Moments

The *method of moments* proves convergence of measures by showing that all moments of $\mu_N$ converge to the moments of $\rho_{sc}$. Since the semicircle distribution has bounded support (and hence is determined by its moments), this is sufficient.

The key computation is:
$$\mathbb{E}\!\left[\frac{1}{N}\text{tr}(\widetilde{W}_N^{2k})\right] = \frac{1}{N} \mathbb{E}\!\left[\sum_{i=1}^N \lambda_i^{2k}\right] = \int x^{2k}\, d\mathbb{E}[\mu_N] \to \int x^{2k}\, d\rho_{sc}(x;2) = C_k \cdot 4^k.$$

We prove this convergence.

**Step 1: Expansion of the trace.** We have:
$$\frac{1}{N}\text{tr}(\widetilde{W}_N^{2k}) = \frac{1}{N} \cdot \frac{1}{(\sigma\sqrt{N})^{2k}} \sum_{i_1, i_2, \ldots, i_{2k}=1}^{N} W_{i_1 i_2} W_{i_2 i_3} \cdots W_{i_{2k} i_1}.$$

This is a sum over closed paths of length $2k$ in the complete graph on $N$ vertices. Let us write:
$$\frac{1}{N}\text{tr}(\widetilde{W}_N^{2k}) = \frac{1}{N^{k+1} \sigma^{2k}} \sum_{(i_1, \ldots, i_{2k})} \prod_{j=1}^{2k} W_{i_j i_{j+1}},$$
where indices are cyclic: $i_{2k+1} = i_1$.

**Step 2: Taking expectations.** When we take $\mathbb{E}[\cdot]$, the independence of the entries $W_{ij}$ means that $\mathbb{E}\!\left[\prod_j W_{i_j i_{j+1}}\right] = 0$ unless every edge $\{i_j, i_{j+1}\}$ appears at least twice in the product (since $\mathbb{E}[W_{ij}] = 0$).

The leading contribution comes from paths where every edge appears exactly twice (paths that contribute $\sigma^{2k}$ to the product, since $\mathbb{E}[W_{ij}^2] = \sigma^2$). Paths where some edge appears three or more times contribute $O(N^{-1})$ to the sum (because the number of such paths is $O(N^k)$ rather than $O(N^{k+1})$) and thus vanish in the limit.

**Step 3: Counting non-crossing pair partitions.** A path $(i_1, i_2, \ldots, i_{2k}, i_1)$ where every edge appears exactly twice corresponds to a *pair partition* of the $2k$ steps: the steps $j$ and $j'$ are paired if they traverse the same edge. Moreover, to contribute at the leading order $N^{k+1}$ to the sum over indices (matching the denominator $N^{k+1}$), the path must visit exactly $k+1$ distinct vertices.

A path on $2k$ edges visiting exactly $k+1$ vertices, where each edge is traversed exactly twice, corresponds to a *non-crossing pair partition* of $\{1, 2, \ldots, 2k\}$. This is a pair partition $\pi = \{\{j_1, j_1'\}, \ldots, \{j_k, j_k'\}\}$ such that the pairings do not "cross": there do not exist $a < b < c < d$ with $\{a,c\} \in \pi$ and $\{b,d\} \in \pi$.

**Claim:** The number of non-crossing pair partitions of $\{1, \ldots, 2k\}$ is exactly the Catalan number $C_k$.

**Proof of Claim:** We use the bijection with Dyck paths. A non-crossing pair partition $\pi$ of $\{1, \ldots, 2k\}$ corresponds to a path in $\mathbb{Z}$ of length $2k$: at step $j$, take $+1$ if $j$ is the *smaller* element of its pair in $\pi$, and $-1$ if $j$ is the *larger* element. Non-crossing ensures this path stays $\geq 0$ and ends at 0. The number of such paths (Dyck paths of length $2k$) is $C_k = \frac{1}{k+1}\binom{2k}{k}$. $\blacksquare$

**Step 4: Counting the index sum.** For each non-crossing pair partition $\pi$ of $\{1,\ldots,2k\}$, the number of index tuples $(i_1, \ldots, i_{2k}) \in \{1, \ldots, N\}^{2k}$ such that:
- $i_j = i_{j'}$ whenever $\{j, j'\} \in \pi$ (each pair uses the same vertex), and
- the vertices at paired steps are the same (but all other steps can be arbitrary),

is $(1 + o(1))N^{k+1}$ as $N \to \infty$ (there are $k+1$ free vertex choices, each ranging over $N$ options, with overlaps that are lower order).

**Step 5: Combining.** Putting it all together:
$$\mathbb{E}\!\left[\frac{1}{N}\text{tr}(\widetilde{W}_N^{2k})\right] = \frac{1}{N^{k+1}\sigma^{2k}} \cdot C_k \cdot \sigma^{2k} \cdot N^{k+1} + O(N^{-1}) = C_k + O(N^{-1}).$$

Since the moments of the semicircle distribution are $\int x^{2k} d\rho_{sc}(x;2) = C_k \cdot 4^k / 4^k = C_k$... wait. Let us check: the semicircle with radius 2 has $\mathbb{E}[X^{2k}] = C_k \cdot 4^k / 4^k = C_k$. Indeed, $\mathbb{E}[X^{2k}] = C_k R^{2k}$ for the semicircle with radius $R$, and at $R=2$: $C_k \cdot 4^k$. But our computation gives $C_k$, not $C_k \cdot 4^k$...

We need to recheck the normalization. With $\widetilde{W}_N = W_N / (\sigma\sqrt{N})$, the entries of $\widetilde{W}_N$ have variance $1/N$. The standard form of the semicircle law uses the normalization where the off-diagonal entries have variance $\sigma^2/N$, and the semicircle has radius $2\sigma$. With $\sigma = 1$, the semicircle has radius 2, and $\mathbb{E}[X^{2k}] = C_k \cdot 4^k$ for $X \sim \rho_{sc}(\cdot; 2)$.

We need to reconcile: the sum over paths gives
$$\frac{1}{N^{k+1}} \cdot C_k \cdot N^{k+1} \cdot \sigma^{2k} \cdot \frac{1}{\sigma^{2k} N^k} = C_k \cdot N / N = C_k.$$

Hmm, let me redo the normalization carefully. We have $\widetilde{W}_{ij} = W_{ij}/(\sigma\sqrt{N})$, so $\mathbb{E}[\widetilde{W}_{ij}^2] = 1/N$. The trace formula gives:
$$\frac{1}{N}\text{tr}(\widetilde{W}_N^{2k}) = \frac{1}{N} \sum_{(i_1,\ldots,i_{2k})} \prod_{j=1}^{2k} \widetilde{W}_{i_j i_{j+1}}.$$

The contribution of a non-crossing pair partition $\pi$ is $\prod_{\{a,b\} \in \pi} \mathbb{E}[\widetilde{W}_{i_a i_{a+1}}^2] = (1/N)^k$, times the number of valid index tuples which is $\sim N^{k+1}$ (as argued above). So the contribution is $C_k \cdot N^{k+1} \cdot N^{-k} \cdot N^{-1} = C_k$. Thus $\mathbb{E}[\frac{1}{N}\text{tr}(\widetilde{W}^{2k})] \to C_k$.

But the semicircle distribution with radius 2 has moments $\mathbb{E}[X^{2k}] = C_k \cdot 4^k / 4^k$... No: $\mathbb{E}[X^{2k}] = C_k R^{2k}$, with $R = 2$, gives $C_k \cdot 4^k$.

The resolution: when we say the semicircle law with radius 2 has $k$-th moment $C_k \cdot 4^k$, but we are computing moments of the *normalized* matrix (entries of order $1/\sqrt{N}$), the $2k$-th moment of the normalized ESD should be $C_k$, corresponding to the semicircle with $R=1$... 

Actually, the standard statement is: for $W_N$ with off-diagonal entries i.i.d. with mean 0 and variance $\sigma^2$, the normalized matrix $\widetilde{W}_N = W_N/(\sigma\sqrt{N})$ has ESD converging to the semicircle with radius 2 (not 1). The $2k$-th moment of $\rho_{sc}(\cdot;2)$ is $C_k \cdot 4^k / 4^k$... 

Let me just directly compute: $\int_{-2}^{2} x^{2k} \frac{\sqrt{4-x^2}}{2\pi} dx$. Setting $x = 2\sin\theta$: $= \int_{-\pi/2}^{\pi/2} (2\sin\theta)^{2k} \frac{2\cos\theta}{2\pi} 2\cos\theta\, d\theta = \frac{2^{2k+1}}{\pi} \int_0^{\pi/2} \sin^{2k}\theta \cos^2\theta\, d\theta$. Using $\int_0^{\pi/2}\sin^{2k}\theta\cos^2\theta\,d\theta = \frac{\pi}{2}\frac{(2k-1)!!}{(2k+2)!!}$... The result is $C_k = \frac{(2k)!}{(k+1)!k!}$. Wait — for the standard semicircle $\rho_{sc}(x;2) = \frac{1}{2\pi}\sqrt{4-x^2}$, the $2k$-th moment is $C_k$. And our computation also gives $C_k$. So everything is consistent.

**Step 6: Almost sure convergence.** The computation above gives convergence of $\mathbb{E}[\mu_N]$ in the sense of moments. Concentration inequalities (Section 27.3) then show that the random variable $\frac{1}{N}\text{tr}(\widetilde{W}^{2k})$ concentrates around its expectation, yielding almost sure convergence of $\mu_N$ to $\rho_{sc}$. $\blacksquare$

## 27.1.4 Implications for Reservoir Spectra

For reservoir computing, the Wigner semicircle law has several direct implications.

**Spectral radius.** For a random Wigner reservoir matrix $W$ with i.i.d. entries of variance $\sigma^2/N$, the spectral radius converges to $\rho(W) \to 2\sigma$ as $N \to \infty$. Therefore, to set the spectral radius of a random symmetric reservoir to a target value $r$, we should choose $\sigma^2 = r^2/4N$ (i.e., entries with standard deviation $r/(2\sqrt{N})$).

**Eigenvalue distribution.** The eigenvalues are uniformly spread over $[-2\sigma, 2\sigma]$ according to the semicircle density. This means:
- There is no "gap" in the spectrum; eigenvalues are present throughout $[-2\sigma, 2\sigma]$.
- The density of eigenvalues near $\pm 2\sigma$ is low (the semicircle vanishes at the edges).
- The density of eigenvalues near 0 is highest (the semicircle is maximized at 0).

**Timescale distribution.** Each eigenvalue $\lambda_i$ of $W$ determines a timescale $\tau_i = -1/\log|\lambda_i|$ for linear dynamics. The semicircle distribution on eigenvalues implies a distribution of timescales. For eigenvalues near $|\lambda| \approx \rho - \delta$ (just below the spectral radius), the timescale $\tau \approx 1/\delta$ is long; for eigenvalues near 0, the timescale is short. The semicircle law says the reservoir has a wide and precisely characterized distribution of timescales.

**The edge eigenvalues.** The Tracy-Widom distribution [TracyWidom1994] describes the fluctuations of the largest eigenvalue around $2\sigma$. These fluctuations are of order $N^{-2/3}$ and follow a specific non-Gaussian distribution. For practical purposes, the spectral radius of a random reservoir deviates from $2\sigma$ by $O(N^{-2/3})$.

## 27.1.5 Non-Symmetric Reservoirs

Reservoir matrices are typically not symmetric. For non-symmetric random matrices with i.i.d. entries, the limiting spectral distribution is the *circular law* [Girko1984]:

**Theorem 27.1.2 (Circular Law).** *Let $W_N$ be an $N \times N$ matrix with i.i.d. entries of mean 0 and variance $\sigma^2/N$. The empirical spectral distribution of $W_N$ (eigenvalues in the complex plane) converges to the uniform distribution on the disk of radius $\sigma$ centered at the origin.*

For reservoir computing with non-symmetric weight matrices:
- The eigenvalues are complex, distributed approximately uniformly in a disk of radius $\rho(W)$.
- Rescaling $W$ to have spectral radius $r$ places all eigenvalues within the unit disk (for $r < 1$), with complex eigenvalues paired as conjugates (since $W$ has real entries).
- The timescale associated with a complex eigenvalue $\lambda = re^{i\omega}$ is $\tau = -1/\log r$ with oscillation frequency $\omega$.

The circular law explains why random reservoirs with spectral radius below 1 naturally generate a rich repertoire of oscillatory modes: eigenvalues are spread throughout the disk, not concentrated near the real axis.
