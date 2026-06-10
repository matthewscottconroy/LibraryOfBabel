# Section 4.4: Why Random Fixed Weights Work

## 4.4.1 The Puzzle

At first glance, using random, fixed weights for the recurrent reservoir seems almost perverse. Neural networks spend their training budget adjusting every weight. Reservoir computing throws away most of the degrees of freedom — fixing $W^{rec}$ and $W^{in}$ at initialization and only training the linear readout — yet achieves comparable or better performance on many tasks. Why?

The short answer is that random projections into high-dimensional spaces are, with high probability, remarkably faithful. The long answer involves three complementary perspectives: the random feature expansion view, the Johnson-Lindenstrauss lemma, and the blessing of dimensionality. Together, they explain why the specific values of reservoir weights matter far less than a handful of global properties.

## 4.4.2 The Random Feature Expansion Perspective

The reservoir can be understood as a nonlinear feature map. Given an input history $\mathbf{u}_{1:t}$, the reservoir state $\mathbf{x}(t) \in \mathbb{R}^N$ is a high-dimensional representation. The readout then finds a linear function of this representation that approximates the target output. This is the kernel machine paradigm [Scholkopf2002]: replace a hard nonlinear regression problem in the original input space with a linear regression problem in a lifted feature space.

The key insight of Rahimi and Recht [Rahimi2007] is that random feature maps approximate shift-invariant kernels. Specifically, Bochner's theorem states that any continuous, positive definite, shift-invariant kernel $k(\mathbf{x}, \mathbf{y}) = k(\mathbf{x} - \mathbf{y})$ can be written as the Fourier transform of a non-negative measure $p(\boldsymbol{\omega})$:

$$k(\mathbf{x} - \mathbf{y}) = \int_{\mathbb{R}^d} p(\boldsymbol{\omega}) e^{i\boldsymbol{\omega}^\top(\mathbf{x} - \mathbf{y})} d\boldsymbol{\omega} = \mathbb{E}_{\boldsymbol{\omega} \sim p}[e^{i\boldsymbol{\omega}^\top \mathbf{x}} \overline{e^{i\boldsymbol{\omega}^\top \mathbf{y}}}].$$

Drawing $N$ random frequencies $\boldsymbol{\omega}_1, \ldots, \boldsymbol{\omega}_N \sim p(\boldsymbol{\omega})$ and defining the random feature map

$$\boldsymbol{\phi}(\mathbf{x}) = \frac{1}{\sqrt{N}} \bigl(\cos(\boldsymbol{\omega}_1^\top \mathbf{x} + b_1), \ldots, \cos(\boldsymbol{\omega}_N^\top \mathbf{x} + b_N)\bigr)^\top,$$

with random phases $b_j \sim \text{Uniform}[0, 2\pi]$, gives the Monte Carlo approximation

$$k(\mathbf{x}, \mathbf{y}) \approx \boldsymbol{\phi}(\mathbf{x})^\top \boldsymbol{\phi}(\mathbf{y}).$$

Rahimi and Recht showed that this approximation is uniform: $\sup_{\mathbf{x},\mathbf{y}} |k(\mathbf{x},\mathbf{y}) - \boldsymbol{\phi}(\mathbf{x})^\top \boldsymbol{\phi}(\mathbf{y})| \leq \varepsilon$ with probability at least $1 - \delta$ whenever $N = O(d \log(1/\delta) / \varepsilon^2)$ [Rahimi2007].

The reservoir is precisely a structured random feature map for input histories rather than static vectors: the recurrent weights $W^{rec}$ mix input features across time, creating temporal cross-products that approximate the RKHS of some kernel on sequence space. The specific random draw matters less than whether the feature map covers the relevant portion of function space — and with high probability, it does.

The *Extreme Learning Machine* (ELM) [Huang2006] makes this connection explicit for feedforward networks: with random hidden weights and biases drawn from any continuous distribution, the hidden layer outputs form a universal basis for approximating continuous functions on compact sets, and only the output weights need training. The same reasoning applies to the reservoir's random projection of the input.

## 4.4.3 The Johnson-Lindenstrauss Lemma

A second perspective comes from geometry. The Johnson-Lindenstrauss lemma [JohnsonLindenstrauss1984] states that a set of $n$ points in $\mathbb{R}^d$ can be embedded into $\mathbb{R}^k$ with $k = O(\log n / \varepsilon^2)$ such that all pairwise distances are preserved to within a factor of $(1 \pm \varepsilon)$:

**Lemma 4.4.1 (Johnson-Lindenstrauss).** *For any $0 < \varepsilon < 1$ and any set of $n$ points $\mathbf{x}_1, \ldots, \mathbf{x}_n \in \mathbb{R}^d$, there exists a map $f: \mathbb{R}^d \to \mathbb{R}^k$ with $k = O(\log n / \varepsilon^2)$ such that for all $i, j$:*
$$(1 - \varepsilon) \|\mathbf{x}_i - \mathbf{x}_j\|^2 \leq \|f(\mathbf{x}_i) - f(\mathbf{x}_j)\|^2 \leq (1 + \varepsilon) \|\mathbf{x}_i - \mathbf{x}_j\|^2.$$

*Moreover, a random linear map $f(\mathbf{x}) = \frac{1}{\sqrt{k}} A \mathbf{x}$, where $A \in \mathbb{R}^{k \times d}$ has i.i.d. $\mathcal{N}(0,1)$ entries, achieves this with high probability.*

The proof follows from the concentration of the chi-squared distribution: for a unit vector $\mathbf{v}$, the random variable $\|A\mathbf{v}\|^2/k$ has mean 1 and variance $2/k$, so by Bernstein's inequality it concentrates within $\varepsilon$ of 1 with probability at least $1 - 2\exp(-k\varepsilon^2/4)$. Taking a union bound over all $\binom{n}{2}$ pairs and setting $k \geq 4\log(2n^2/\delta)/\varepsilon^2$ gives the result.

For reservoir computing, the JL lemma reassures us that random projections preserve the structure of the input space that matters for classification and regression: the relative distances between distinct input patterns. If the input histories $\mathbf{u}_{1:t}$ form a set of $n$ patterns (or can be covered by $n$ representative patterns), then a random projection of dimension $N = O(\log n / \varepsilon^2)$ suffices to separate them. This is the sense in which **the reservoir does not need to be designed** — it merely needs to be large enough.

## 4.4.4 Global Properties Trump Local Connectivity

The JL and random feature perspectives both suggest that what matters is not the precise pattern of individual connections but the global statistical properties of the weight matrix. This intuition is formalized by several observations:

**Spectral radius $\rho(W)$:** The spectral radius controls the fading memory timescale. For $|\rho| < 1$, perturbations decay exponentially with rate $\log \rho(W)$. The exact distribution of weights within the $\rho(W)$-ball matters far less than $\rho(W)$ itself, because the long-term dynamics are governed by the dominant eigenvalue.

**Connection density $p$:** Sparse random matrices with connectivity fraction $p$ have spectral radius concentrating near $2\sigma\sqrt{p}$ (a correction to the dense Wigner law; see Section 27.1). What determines reservoir performance is not which neurons are connected but whether the density is sufficient to sustain communication across the network.

**Weight distribution:** Gaussian versus uniform distributions for $W^{rec}$ produce nearly identical reservoirs for large $N$, by the universality results for random matrices [Tao2012]. Signed weights (zero-mean) are essential for the echo state property to hold generically; all-positive weights can lead to a single saturating mode.

Empirically, extensive studies have confirmed that the detailed connectivity pattern has small effect on performance, while $\rho$, $p$, and the input scaling $\sigma_{in}$ have large effects [Lukosevicius2012]. This is the operational meaning of "random works": within the broad class of random matrices with appropriate global properties, performance is largely invariant to the specific draw.

## 4.4.5 The Blessing of Dimensionality

A common intuition in machine learning holds that high dimensions are dangerous — data become sparse, distances concentrate, and classifiers fail. This is the *curse of dimensionality*. But reservoir computing exploits a complementary phenomenon, sometimes called the *blessing of dimensionality*: in high-dimensional spaces, random projections of structured data sets tend to become linearly separable.

**Cover's theorem** [Cover1965] states that a set of $n$ points in $\mathbb{R}^d$ is linearly separable with high probability whenever $n < 2d$. More precisely, the fraction of dichotomies of $n$ points in general position that are linearly separable is

$$P(\text{sep}) = 2^{-(n-1)} \sum_{k=0}^{d-1} \binom{n-1}{k},$$

which approaches 1 rapidly once $n \ll d$.

Applying this to reservoir computing: the $T$ reservoir state vectors $\mathbf{x}(t) \in \mathbb{R}^N$ form a point cloud in the state space. If $N \gg T$ (many more neurons than time steps), nearly every dichotomy of the time steps is linearly separable by a readout weight vector. In practice, useful targets are far from random, so the actual requirement is much weaker — but the dimensionality of the reservoir still provides a substantial margin.

The blessing of dimensionality also explains why random feature maps work: in the lifted feature space, the geometry becomes more favorable for linear discrimination, regardless of whether the features were designed or drawn at random.

## 4.4.6 Why Anything Beyond the Echo State Property Is Unnecessary

Pulling these strands together: reservoir computing works because (i) fading memory ensures that the state $\mathbf{x}(t)$ is a well-defined functional of the recent input history (Boyd-Chua, Section 26.3); (ii) the random projection of this history into $\mathbb{R}^N$ approximately preserves distances and approximates the relevant kernel (JL + random features); and (iii) the high-dimensional state space makes linear separation of the resulting representations generically easy.

No single weight needs to be tuned, because the three guarantees above hold with high probability for any draw from a broad class of random distributions, as long as the global parameters $\rho$, $p$, and $\sigma_{in}$ are in the right range. This is the deep reason why reservoir computing works with random fixed weights — not magic, but geometry.

---

## References

- **[Cover1965]** T. M. Cover. "Geometrical and statistical properties of systems of linear inequalities with applications in pattern recognition." *IEEE Transactions on Electronic Computers*, 14(3):326-334, 1965.
- **[Huang2006]** G.-B. Huang, Q.-Y. Zhu, and C.-K. Siew. "Extreme learning machine: Theory and applications." *Neurocomputing*, 70(1-3):489-501, 2006.
- **[JohnsonLindenstrauss1984]** W. B. Johnson and J. Lindenstrauss. "Extensions of Lipschitz mappings into a Hilbert space." *Contemporary Mathematics*, 26:189-206, 1984.
- **[Lukosevicius2012]** M. Lukosevicius. "A practical guide to applying echo state networks." In *Neural Networks: Tricks of the Trade*, Springer, pp. 659-686, 2012.
- **[Rahimi2007]** A. Rahimi and B. Recht. "Random features for large-scale kernel machines." *Advances in Neural Information Processing Systems*, 20, 2007.
- **[Tao2012]** T. Tao. *Topics in Random Matrix Theory*. American Mathematical Society, 2012.
