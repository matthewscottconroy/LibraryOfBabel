# PAC Learning and the VC Dimension for Reservoir Readouts

## 28.1 The PAC Learning Framework

Statistical learning theory asks a precise question: given a learner that has observed a finite dataset, when can we guarantee that the learned hypothesis generalizes? The **Probably Approximately Correct (PAC)** framework [Valiant 1984] provides the first rigorous answer.

**Definition 28.1 (PAC Learnability).** A hypothesis class $\mathcal{H}$ is PAC learnable if there exists an algorithm $\mathcal{A}$ and a function $m: (0,1)^2 \to \mathbb{N}$ such that, for all $\varepsilon, \delta \in (0,1)$ and all distributions $\mathcal{D}$ over $\mathcal{X} \times \{0,1\}$, if $\mathcal{A}$ receives $m \geq m(\varepsilon, \delta)$ i.i.d. examples from $\mathcal{D}$, then with probability at least $1 - \delta$, $\mathcal{A}$ outputs a hypothesis $h \in \mathcal{H}$ satisfying

$$
\mathcal{L}_{\mathcal{D}}(h) \leq \varepsilon.
$$

The function $m(\varepsilon, \delta)$ is called the **sample complexity** of $\mathcal{H}$. In words: with probability $\geq 1 - \delta$ (the "probably" part), the learned hypothesis has error at most $\varepsilon$ (the "approximately correct" part).

For a finite hypothesis class $|\mathcal{H}| < \infty$, a union bound argument yields an immediate sample complexity bound. The probability that any single hypothesis $h$ is consistent with $m$ training examples yet has $\mathcal{L}_{\mathcal{D}}(h) > \varepsilon$ is at most $(1-\varepsilon)^m \leq e^{-\varepsilon m}$. Applying the union bound over all $|\mathcal{H}|$ hypotheses,

$$
\Pr\!\left[\exists h \in \mathcal{H} : \hat{\mathcal{L}}(h) = 0 \text{ and } \mathcal{L}_{\mathcal{D}}(h) > \varepsilon\right] \leq |\mathcal{H}| e^{-\varepsilon m}.
$$

Setting this bound equal to $\delta$ and solving for $m$:

$$
m(\varepsilon, \delta) = \frac{1}{\varepsilon}\left(\ln |\mathcal{H}| + \ln \frac{1}{\delta}\right) = \Omega\!\left(\frac{1}{\varepsilon}\ln \frac{|\mathcal{H}|}{\delta}\right).
$$

This is the fundamental PAC sample complexity for finite classes [Blumer et al. 1989]. The challenge for reservoir computing is that the relevant hypothesis class — linear functions of the reservoir state — is infinite. We need a measure of effective complexity for infinite classes.

## 28.2 The VC Dimension

The Vapnik-Chervonenkis (VC) dimension [Vapnik & Chervonenkis 1971] extends the PAC framework to infinite hypothesis classes by measuring the *combinatorial complexity* of $\mathcal{H}$.

**Definition 28.2 (Shattering).** A set $S = \{x_1, \ldots, x_m\} \subseteq \mathcal{X}$ is **shattered** by $\mathcal{H}$ if for every binary labeling $\mathbf{y} \in \{0,1\}^m$, there exists $h \in \mathcal{H}$ such that $h(x_i) = y_i$ for all $i$.

**Definition 28.3 (VC Dimension).** The VC dimension of $\mathcal{H}$ is

$$
d_{\mathrm{VC}}(\mathcal{H}) = \sup\{m \in \mathbb{N} : \exists S \subseteq \mathcal{X},\, |S| = m,\, S \text{ is shattered by } \mathcal{H}\}.
$$

The **Sauer-Shelah lemma** [Sauer 1972, Shelah 1972] bounds the growth function $\Pi_\mathcal{H}(m)$ — the maximum number of distinct labelings $\mathcal{H}$ can produce on any $m$ points:

$$
\Pi_\mathcal{H}(m) \leq \sum_{i=0}^{d_{\mathrm{VC}}} \binom{m}{i} \leq \left(\frac{em}{d_{\mathrm{VC}}}\right)^{d_{\mathrm{VC}}}.
$$

This polynomial growth (once $m > d_{\mathrm{VC}}$) is the key. Using the Sauer-Shelah bound in place of $|\mathcal{H}|$, the PAC sample complexity for any class with finite VC dimension $d$ becomes [Blumer et al. 1989]:

$$
m(\varepsilon, \delta) = O\!\left(\frac{d}{\varepsilon} \ln \frac{d}{\varepsilon} + \frac{1}{\varepsilon} \ln \frac{1}{\delta}\right).
$$

This is the **fundamental theorem of statistical learning**: a hypothesis class is PAC learnable if and only if its VC dimension is finite [Blumer et al. 1989].

## 28.3 VC Dimension of the Linear Readout

In the reservoir computing context, the hypothesis class of interest is the set of linear threshold functions applied to reservoir states. Fix a reservoir with $N$ neurons producing state $\mathbf{x}(u) \in \mathbb{R}^N$ for input sequence $u$. The readout (for binary classification) is

$$
\mathcal{H}_N = \left\{u \mapsto \mathrm{sign}\!\left(\mathbf{w}^T \mathbf{x}(u)\right) \;\middle|\; \mathbf{w} \in \mathbb{R}^N \right\}.
$$

**Theorem 28.1.** The VC dimension of $\mathcal{H}_N$ satisfies $d_{\mathrm{VC}}(\mathcal{H}_N) = N$.

*Proof sketch.* The class $\mathcal{H}_N$ is equivalent to the class of linear halfspaces in $\mathbb{R}^N$ (after mapping inputs to reservoir states). It is a classical result that halfspaces in $\mathbb{R}^N$ have VC dimension $N$ [Vapnik & Chervonenkis 1971]. The lower bound is witnessed by $N$ points in general position; the upper bound follows from Radon's theorem. $\square$

## 28.4 PAC Bound for the Reservoir Readout

Combining the VC dimension result with the PAC sample complexity formula:

**Corollary 28.2 (PAC Sample Complexity for Reservoir Readout).** For the linear readout class $\mathcal{H}_N$ with reservoir size $N$, PAC learnability holds with sample complexity

$$
m(\varepsilon, \delta) = O\!\left(\frac{N}{\varepsilon} \ln \frac{N}{\varepsilon} + \frac{1}{\varepsilon} \ln \frac{1}{\delta}\right).
$$

*Interpretation.* To guarantee generalization error $\leq \varepsilon$ with confidence $\geq 1 - \delta$, it suffices to use approximately $T \sim N/\varepsilon$ training examples (ignoring logarithmic factors). The number of required examples grows linearly with the reservoir size. Doubling the reservoir requires doubling the training set to maintain the same generalization guarantee.

**Practical implication.** Many reservoir computing applications use $N = 100$–$1000$ neurons. At $\varepsilon = 0.05$, $\delta = 0.05$, the PAC bound requires $T \approx 20N$–$200N$ examples. For $N = 500$, this is $T \approx 10{,}000$–$100{,}000$ — a range consistent with standard practice in, e.g., chaotic time series prediction benchmarks.

## 28.5 Limitations of the VC Bound

The VC dimension bound in Corollary 28.2 is **worst-case over all data distributions**. For specific distributions that arise in practice, the bound can be substantially pessimistic. Three sources of tightness loss are:

1. **Reservoir state geometry.** If the reservoir states $\{\mathbf{x}(u_t)\}$ are confined to a low-dimensional subspace of $\mathbb{R}^N$, the effective VC dimension is the dimension of that subspace, not $N$.

2. **Bounded norms.** If $\|\mathbf{w}\|_2 \leq B$ and $\|\mathbf{x}\|_2 \leq R$ for all states, the relevant complexity is controlled by $BR / \sqrt{T}$, not $N/T$. This leads to the Rademacher bounds of Section 28.2.

3. **Spectral decay.** If the singular values of the state matrix $\mathbf{X} \in \mathbb{R}^{T \times N}$ decay rapidly, the effective degrees of freedom are much less than $N$. Covering numbers (Section 28.3) capture this.

**Remark 28.1 (VC vs. Rademacher).** The VC dimension is distribution-free and provides a necessary-and-sufficient condition for PAC learnability. Rademacher complexity is distribution-dependent and provides tighter bounds for specific data regimes. In practice, Rademacher bounds are preferred when the reservoir state distribution is known or can be estimated.

## 28.6 Beyond Binary Classification

For regression problems (which are more common in reservoir computing), the squared loss $\ell(f(u), y) = (f(u) - y)^2$ is not bounded by $\{0,1\}$, and the VC framework requires extension. The **fat-shattering dimension** [Kearns & Schapire 1994] and **$\varepsilon$-dimension** generalize VC theory to real-valued function classes. For the bounded linear readout class with $|f(u)| \leq M$, the fat-shattering dimension at margin $\gamma$ satisfies $\mathrm{fat}_\gamma(\mathcal{H}_N) \leq (BM/\gamma)^2$, independent of $N$. This foreshadows the margin-based bounds developed via Rademacher complexity in Section 28.2.

## References

- Blumer, A., Ehrenfeucht, A., Haussler, D., and Warmuth, M. K. (1989). Learnability and the Vapnik-Chervonenkis dimension. *Journal of the ACM*, 36(4), 929–965.
- Kearns, M. and Schapire, R. (1994). Efficient distribution-free learning of probabilistic concepts. *Journal of Computer and System Sciences*, 48(3), 464–497.
- Sauer, N. (1972). On the density of families of sets. *Journal of Combinatorial Theory, Series A*, 13(1), 145–147.
- Shelah, S. (1972). A combinatorial problem; stability and order for models and theories in infinitary languages. *Pacific Journal of Mathematics*, 41(1), 247–261.
- Valiant, L. G. (1984). A theory of the learnable. *Communications of the ACM*, 27(11), 1134–1142.
- Vapnik, V. N. and Chervonenkis, A. Y. (1971). On the uniform convergence of relative frequencies of events to their probabilities. *Theory of Probability and its Applications*, 16(2), 264–280.
