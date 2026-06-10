# Section 29.1: Ergodic Theory Basics

## 29.1.1 Measure-Preserving Transformations

**Definition 29.1.1 (Measure-Preserving Transformation).** Let $(\Omega, \mathcal{F}, \mu)$ be a probability space. A measurable map $T: \Omega \to \Omega$ is *measure-preserving* (or $\mu$-*preserving*) if
$$\mu(T^{-1}(A)) = \mu(A) \quad \text{for all } A \in \mathcal{F}.$$

Equivalently, $T_* \mu = \mu$ (the pushforward measure equals $\mu$). This says the transformation $T$ reshuffles the probability space without changing the size of measurable sets.

**Example 29.1.1 (Circle Rotation).** Let $\Omega = [0,1)$, $\mathcal{F}$ the Borel $\sigma$-algebra, $\mu$ Lebesgue measure, and $T(\omega) = \omega + \alpha \pmod{1}$ for a fixed $\alpha \in [0,1)$. Then $T$ is measure-preserving. The orbit $\omega, T(\omega), T^2(\omega), \ldots$ visits all parts of the circle uniformly if $\alpha$ is irrational (equidistribution) and is periodic if $\alpha$ is rational.

**Example 29.1.2 (Shift Map).** Let $\Omega = \{0,1\}^{\mathbb{Z}}$ (bi-infinite binary sequences), $\mu$ the Bernoulli($1/2$) product measure, and $T$ the left shift: $(T\omega)_n = \omega_{n+1}$. Then $T$ is measure-preserving. This is the canonical model for a stationary stochastic process.

**Example 29.1.3 (Input Process as a Measure-Preserving System).** In reservoir computing, the input $u: \mathbb{Z} \to U$ is often modeled as a stationary stochastic process. Stationarity means the distribution of $(u(t_1), \ldots, u(t_k))$ is the same as $(u(t_1+s), \ldots, u(t_k+s))$ for all $s$. The shift map $T$ on the probability space $(\Omega, \mathcal{F}, \mathbb{P})$ of input sequences is measure-preserving. The input at time $t$ is the function $u_t(\omega) = u(t, \omega)$, and stationarity means $u_t \circ T = u_{t+1}$.

## 29.1.2 Ergodicity and Mixing

**Definition 29.1.2 (Ergodicity).** A measure-preserving transformation $T: (\Omega, \mathcal{F}, \mu) \to (\Omega, \mathcal{F}, \mu)$ is *ergodic* if every $T$-invariant set has measure 0 or 1:
$$T^{-1}(A) = A \implies \mu(A) \in \{0, 1\}.$$

Ergodicity means the system cannot be decomposed into two invariant subsets of positive measure. In physical terms: the orbit of almost every initial condition eventually visits every part of the space (in the time-average sense).

**Definition 29.1.3 (Mixing).** $T$ is *strongly mixing* if
$$\lim_{n \to \infty} \mu(T^{-n}(A) \cap B) = \mu(A)\mu(B) \quad \text{for all } A, B \in \mathcal{F}.$$

Mixing says that events far apart in time become independent. Strong mixing implies ergodicity. For reservoir computing, mixing of the input process ensures that long-range dependence decays, which is related to the fading memory property of the reservoir.

**Hierarchy of properties:** Independent $\Rightarrow$ Strong mixing $\Rightarrow$ Weak mixing $\Rightarrow$ Ergodic.

**Example 29.1.4.** The Bernoulli shift (Example 29.1.2) is strongly mixing (independent increments). Circle rotation by irrational $\alpha$ is ergodic but not mixing (no decay of correlations; $\mathbb{E}[f(T^n\omega)g(\omega)] = \hat{f}_n \bar{g}_0$ where $\hat{f}_n$ does not go to 0 for all $f, g$).

## 29.1.3 Birkhoff's Ergodic Theorem

The cornerstone result of ergodic theory is Birkhoff's theorem, which establishes the relationship between time averages (what we observe from a single orbit) and space averages (expectations under $\mu$).

**Theorem 29.1.1 (Birkhoff's Pointwise Ergodic Theorem, 1931).** *Let $T: (\Omega, \mathcal{F}, \mu) \to (\Omega, \mathcal{F}, \mu)$ be measure-preserving and $f \in L^1(\mu)$. Then the time averages*
$$A_n(f)(\omega) = \frac{1}{n}\sum_{k=0}^{n-1} f(T^k \omega)$$
*converge almost surely to a $T$-invariant function $\bar{f} \in L^1(\mu)$:*
$$\lim_{n \to \infty} A_n(f)(\omega) = \bar{f}(\omega) \quad \text{for } \mu\text{-a.e. } \omega.$$
*Moreover, $\int \bar{f}\, d\mu = \int f\, d\mu$ and $\bar{f} \circ T = \bar{f}$ a.s.*

*If $T$ is additionally ergodic, then $\bar{f}$ is constant a.s.:*
$$\bar{f}(\omega) = \int f\, d\mu \quad \text{for } \mu\text{-a.e. } \omega.$$

This is the mathematical statement that time averages equal ensemble averages for ergodic systems.

**Proof of Birkhoff's Theorem.** We give the proof due to Katznelson and Weiss (1982), which is more transparent than Birkhoff's original.

**Step 1: Maximal inequality.** Define the *maximal ergodic average*:
$$f^*(n) = \max_{1 \leq k \leq n} A_k(f).$$
We claim: $\int_{\{f^*(\infty) > 0\}} f\, d\mu \geq 0$, where $f^*(\infty) = \sup_k A_k(f)$.

This is the *maximal ergodic theorem* (Hopf-Dunford-Schwartz): define $F_n = \max(f, f+Tf, \ldots, f + Tf + \cdots + T^{n-1}f)$. Then $F_n \geq 0$ on $\{f^*(\infty) > 0\}$, $F_n \circ T \leq F_n + f - T^n f / n$... 

We use a cleaner formulation. Let $S_n f = \sum_{k=0}^{n-1} f(T^k\omega)$ and $M_n f = \max(0, S_1 f, \ldots, S_n f)$. Then:
$$M_n f = M_n f \circ T + f - \min(S_n f, M_n f \circ T + f),$$
but the key identity is:
$$M_n f \geq M_n f \circ T + f - M_n f \circ T = f \quad \text{when } M_n f > 0.$$

More precisely: on $\{M_n f > 0\}$, we have $f \leq M_n f - M_{n-1}f \circ T \leq M_n f - M_n f \circ T + f(T^n\cdot)/n$...

Let me use Garsia's streamlined proof. Define $S_n^+ = \max_{1 \leq k \leq n} S_k f$ (the running maximum). The key is:

$$\int_{S_n^+ > 0} f\, d\mu \geq 0.$$

**Proof of maximal ergodic lemma:** On the set $E_n = \{S_n^+ > 0\}$, write $S_n^+ = S_1 f \cdot \mathbf{1}_{\{S_1 = S_n^+\}} + \ldots$. Since $S_n^+ \geq S_k f$ for all $k \leq n$, and $S_n^+(T\omega) = \max_{1 \leq k \leq n} (S_{k+1}f(\omega) - f(\omega))$:
$$S_n^+ \geq f + (S_n^+ \circ T) \text{ on } E_n.$$
Integrating: $\int_{E_n} f\, d\mu \geq \int_{E_n} (S_n^+ \circ T - S_n^+)\, d\mu + \int_{E_n}(S_n^+ - S_n^+ \circ T)\, d\mu = 0$ (by $\mu$-preserving and $S_n^+ \geq 0$). $\square$

**Step 2: Convergence a.s.** For $a < b$, let $U(a,b) = \{\omega : \liminf_n A_n f(\omega) < a < b < \limsup_n A_n f(\omega)\}$ (the set where $A_n f$ has infinitely many "upcrossings"). We show $\mu(U(a,b)) = 0$.

Consider $g = f - b$. Then $A_n g = A_n f - b$, and $\limsup_n A_n g > 0$ on $U(a,b)$. Apply the maximal ergodic lemma to $g$: $\int_{\{f^*_g > 0\}} g\, d\mu \geq 0$, so $\int_{\{f^*_g > 0\}} f\, d\mu \geq b \mu(\{f^*_g > 0\})$.

Similarly for $h = a - f$: $\int_{\{f^*_h > 0\}} h\, d\mu \geq 0$, so $\int_{\{f^*_h > 0\}} f\, d\mu \leq a \mu(\{f^*_h > 0\})$.

Since $U(a,b) \subseteq \{f^*_g > 0\} \cap \{f^*_h > 0\}$, we get $b\mu(U(a,b)) \leq \int_{U(a,b)} f\, d\mu \leq a\mu(U(a,b))$. Since $a < b$, this forces $\mu(U(a,b)) = 0$.

This holds for all rational $a < b$, and the set of $\omega$ where $A_n f(\omega)$ does not converge is $\bigcup_{a < b, a,b \in \mathbb{Q}} U(a,b)$, which has measure zero.

**Step 3: Identifying the limit.** The limit $\bar{f} = \lim_n A_n f$ satisfies $\bar{f} \circ T = \bar{f}$ (by the argument $A_n(f)(T\omega) - A_n(f)(\omega) = (f(T^n\omega) - f(\omega))/n \to 0$ a.s.) and $\int \bar{f}\, d\mu = \int f\, d\mu$ (by dominated convergence, using $L^1$ convergence which follows from the a.s. convergence and uniform integrability). $\blacksquare$

## 29.1.4 Applications to Reservoir Computing

**Time averages in reservoir output.** Consider a reservoir with ESP driven by an ergodic stationary input $u$. The output $y(t) = W_{\text{out}} x(t)$ is a stationary process (since $x(t) = H(u_{\leq t})$ and $u$ is stationary). By Birkhoff's theorem, the time average of the output converges:
$$\frac{1}{T}\sum_{t=1}^T y(t) \to \mathbb{E}[y(t)] \quad \text{a.s.}$$

This is why training a reservoir on a long time series gives a good approximation to the ensemble expectation. The stationarity of the input-output pair, combined with ergodicity, ensures that time averages (what we compute from data) equal ensemble averages (what we care about theoretically).

**Training as time averaging.** The ridge regression readout
$$W_{\text{out}} = \left(\frac{1}{T}\sum_t y^{\text{target}}(t) x(t)^\top\right)\left(\frac{1}{T}\sum_t x(t)x(t)^\top + \lambda I\right)^{-1}$$
converges (as $T \to \infty$) to
$$W_{\text{out}}^* = \mathbb{E}[y^{\text{target}}(t)x(t)^\top]\left(\mathbb{E}[x(t)x(t)^\top] + \lambda I\right)^{-1}$$
by the ergodic theorem applied to each matrix entry. This is why long training sequences improve performance: the ergodic theorem guarantees convergence of the empirical covariances to their population counterparts.

**Mixing and fading memory.** The connection between ergodic theory and fading memory is through mixing. Recall that a functional has fading memory if its dependence on the input decays with the age of the input. For a strongly mixing input process, the correlation between $u(t)$ and $u(t-k)$ decays as $k \to \infty$. A reservoir with fading memory, driven by a mixing input, produces a mixing output process — the mixing of the input "transfers" through the reservoir.

More precisely: if the input $u$ is $\phi$-mixing with rate $\phi(k)$, and the reservoir satisfies an exponential echo state property ($\|x_u(t) - x_v(t)\| \leq C e^{-\gamma t}\|x_u(0) - x_v(0)\|$ for any two states), then the reservoir output is also $\phi$-mixing (with a different rate). This fact underlies the validity of applying the ergodic theorem to reservoir outputs in practice.
