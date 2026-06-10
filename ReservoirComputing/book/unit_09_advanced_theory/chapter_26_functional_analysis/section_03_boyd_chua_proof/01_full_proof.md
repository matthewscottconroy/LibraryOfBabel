# Section 26.3: The Boyd-Chua Theorem — Full Proof

## 26.3.1 Setup and Statement

The Boyd-Chua theorem [BoydChua1985] is the cornerstone theoretical result of reservoir computing. It establishes that any causal, time-invariant functional with the fading memory property can be uniformly approximated, on a compact set of inputs, by a polynomial functional that is realized by a finite-dimensional dynamical system — that is, by a reservoir computer.

We set up the precise framework before stating the theorem.

**Input space.** Let $\ell^\infty(\mathbb{Z}_{-})$ denote the space of all bounded bi-infinite sequences indexed by $\mathbb{Z}_{-} = \{\ldots, -2, -1, 0\}$, taking values in a compact set $U \subseteq \mathbb{R}^d$. We write elements as $\mathbf{u} = (\ldots, u_{-2}, u_{-1}, u_0)$. The standard norm on $\ell^\infty(\mathbb{Z}_{-})$ is $\|\mathbf{u}\|_\infty = \sup_{k \leq 0} |u_k|$.

**Weighted norm and fading memory.** For a *weight sequence* $w = (w_0, w_1, w_2, \ldots)$ with $w_k > 0$ and $w_k \to \infty$ as $k \to \infty$, define the weighted norm:
$$\|\mathbf{u}\|_w = \sup_{k \geq 0} w_k |u_{-k}|.$$
The *fading memory space* with weight $w$ is:
$$X_w = \{\mathbf{u} \in \ell^\infty(\mathbb{Z}_{-}) : \|\mathbf{u}\|_w < \infty\}.$$

This space consists of sequences whose values in the distant past are small relative to the weight $w_k$. The intuition is that $w_k$ measures "how much we care about events $k$ steps in the past" — and requiring $\|\mathbf{u}\|_w < \infty$ means that the sequence is small enough in the distant past that the weighted contribution is finite.

**Definition 26.3.1 (Fading Memory Functional).** A causal, time-invariant functional $H: X_w \to \mathbb{R}$ has the *fading memory property* with weight $w$ if $H$ is continuous with respect to the $\|\cdot\|_w$ norm. Explicitly: for every $\mathbf{u} \in X_w$ and $\varepsilon > 0$, there exists $\delta > 0$ such that
$$\|\mathbf{u} - \mathbf{v}\|_w < \delta \implies |H(\mathbf{u}) - H(\mathbf{v})| < \varepsilon.$$

The fading memory property says: two input histories that agree in the recent past (and whose difference in the distant past is down-weighted by $w$) produce similar outputs. It formalizes the intuitive notion that the system "forgets" its distant past.

**Definition 26.3.2 (Polynomial Functional).** A functional $P: X_w \to \mathbb{R}$ is a *polynomial functional* of degree $m$ if it has the form
$$P(\mathbf{u}) = c_0 + \sum_{k=0}^{\infty} a_{k} u_{-k} + \sum_{k,\ell=0}^{\infty} b_{k\ell}\, u_{-k} u_{-\ell} + \cdots + \sum_{k_1, \ldots, k_m \geq 0} c_{k_1 \cdots k_m}\, u_{-k_1} \cdots u_{-k_m}$$
where all sums are finite (only finitely many coefficients are nonzero).

A polynomial functional depends on only finitely many past values of the input, combining them polynomially.

**Theorem 26.3.1 (Boyd-Chua, 1985).** *Let $H: X_w \to \mathbb{R}$ be a continuous (fading memory) functional. Let $K \subseteq X_w$ be a compact set. For every $\varepsilon > 0$, there exists a polynomial functional $P$ such that*
$$\sup_{\mathbf{u} \in K} |H(\mathbf{u}) - P(\mathbf{u})| < \varepsilon.$$

*Moreover, $P$ can be realized as the output of a finite-dimensional polynomial dynamical system (a reservoir computer with polynomial activation).*

## 26.3.2 The Topology on Input Space

The proof requires a careful analysis of the topology of the input space. The key step is to show that the weighted norm $\|\cdot\|_w$ induces a topology under which compact sets are "small" in a precise sense — specifically, that the evaluation functionals $\mathbf{u} \mapsto u_{-k}$ are continuous and the input space has "enough" compact sets for Stone-Weierstrass to apply.

**Proposition 26.3.1.** *For each $k \geq 0$, the evaluation functional $e_k: X_w \to \mathbb{R}$, $e_k(\mathbf{u}) = u_{-k}$, is continuous in the $\|\cdot\|_w$ norm.*

**Proof.** We have $|e_k(\mathbf{u}) - e_k(\mathbf{v})| = |u_{-k} - v_{-k}| \leq \frac{1}{w_k}\|\mathbf{u} - \mathbf{v}\|_w$. Since $w_k > 0$, this is a valid Lipschitz bound. $\blacksquare$

**Proposition 26.3.2 (Compact Sets in $X_w$).** *A set $K \subseteq X_w$ is compact in the $\|\cdot\|_w$ topology if and only if it is closed, bounded in $\|\cdot\|_w$, and equismall at infinity: for every $\varepsilon > 0$, there exists $N$ such that $\sup_{\mathbf{u} \in K} \sup_{k > N} w_k |u_{-k}| < \varepsilon$.*

**Proof.** ($\Rightarrow$) Compactness implies closed and bounded in any metric space. The equismallness condition is the analogue of equicontinuity for function spaces; it follows by a standard diagonal argument (if it fails, extract a sequence with no convergent subsequence).

($\Leftarrow$) Under these conditions, $K$ is a subset of $\prod_{k=0}^{N} [-M/w_k, M/w_k] \times \prod_{k>N} [-\varepsilon/w_k, \varepsilon/w_k]$, where $M$ is the $\|\cdot\|_w$-bound on $K$. This is a product of compact sets, and $K$ being closed in this product implies compactness by Tychonoff's theorem (since each factor is compact). $\blacksquare$

The equismallness condition is precisely what the fading memory property gives us: inputs in $K$ all "fade" at a rate controlled by $w$.

## 26.3.3 The Polynomial Algebra Separates Points

Let $\mathcal{P}$ denote the algebra of polynomial functionals on $X_w$ (with only finitely many nonzero coefficients). We verify the hypotheses of Stone-Weierstrass.

**Lemma 26.3.1.** *$\mathcal{P}$ contains the constants.*

**Proof.** The constant functional $P \equiv c$ is a polynomial functional (with $c_0 = c$ and all other coefficients zero). $\blacksquare$

**Lemma 26.3.2.** *$\mathcal{P}$ is a subalgebra of $C(K)$: it is closed under addition, scalar multiplication, and pointwise multiplication.*

**Proof.** Addition and scalar multiplication are obvious. For multiplication: if $P$ and $Q$ are polynomial functionals, then $(PQ)(\mathbf{u}) = P(\mathbf{u})Q(\mathbf{u})$ is a polynomial in the evaluation functionals $e_k(\mathbf{u}) = u_{-k}$, since the product of two finite polynomials in the $e_k$ is again a finite polynomial. $\blacksquare$

**Lemma 26.3.3.** *$\mathcal{P}$ separates points of $K$.*

**Proof.** Let $\mathbf{u}, \mathbf{v} \in K$ with $\mathbf{u} \neq \mathbf{v}$. Then there exists $k \geq 0$ with $u_{-k} \neq v_{-k}$. The linear polynomial functional $e_k: \mathbf{u} \mapsto u_{-k}$ is in $\mathcal{P}$ and satisfies $e_k(\mathbf{u}) \neq e_k(\mathbf{v})$. $\blacksquare$

**Remark 26.3.1.** The crucial role of the topology is now clear: the evaluation functionals $e_k$ are *continuous* on $K$ in the $\|\cdot\|_w$ topology (Proposition 26.3.1), so they are elements of $C(K)$. Without the continuity, they would not lie in the function algebra, and Stone-Weierstrass would not apply.

## 26.3.4 Main Proof

We now assemble the proof of the Boyd-Chua theorem.

**Proof of Theorem 26.3.1.**

**Step 1: Reformulation as a Stone-Weierstrass problem.** We are given a compact set $K \subseteq X_w$ and a continuous functional $H \in C(K)$ (continuous in the $\|\cdot\|_w$ topology). We wish to show that $H$ can be uniformly approximated by polynomial functionals on $K$.

By Lemmas 26.3.1, 26.3.2, and 26.3.3, the collection $\mathcal{P}|_K$ (polynomial functionals restricted to $K$) is a subalgebra of $C(K)$ that contains the constants and separates points.

**Step 2: $K$ is compact Hausdorff.** The space $X_w$ with the $\|\cdot\|_w$ norm is a metric space (hence Hausdorff), and $K$ is compact by assumption. Therefore $K$ is a compact Hausdorff space.

**Step 3: Apply Stone-Weierstrass.** By Theorem 26.1.2, $\mathcal{P}|_K$ is dense in $C(K)$. Therefore, for any $\varepsilon > 0$, there exists $P \in \mathcal{P}$ such that
$$\sup_{\mathbf{u} \in K} |H(\mathbf{u}) - P(\mathbf{u})| < \varepsilon. \qquad \blacksquare$$

**Remark 26.3.2 (What we have actually proved).** The argument above proves approximation by polynomial functionals. The further claim — that $P$ is realized by a finite-dimensional dynamical system — follows from the observation that a polynomial functional $P(\mathbf{u}) = \sum a_{k_1 \cdots k_m} u_{-k_1} \cdots u_{-k_m}$ (finite sum, depending on at most $N$ time steps) can be computed by a finite-dimensional linear reservoir. Specifically, a delay-line reservoir of length $N$ stores the vector $(u_0, u_{-1}, \ldots, u_{-(N-1)})$ in its state, and the readout computes the polynomial $P$ from this state vector. This is a valid reservoir architecture.

## 26.3.5 Discussion: What the Theorem Does and Does Not Say

**What it says:**
1. *Approximation is possible*: Any fading-memory functional, on any compact set of inputs, can be approximated arbitrarily well by polynomial functionals, and hence by reservoir computers.
2. *The architecture is flexible*: The reservoir does not need to have any particular structure (not a random RNN, not a specific recurrent architecture) — a delay-line reservoir suffices for the universal approximation claim.
3. *Compactness is explicit*: The theorem holds on compact sets $K \subseteq X_w$. The weight $w$ and the compact set $K$ must be specified; the approximation may degrade outside $K$.

**What it does not say:**
1. *It does not give rates*: The theorem is existential. It does not say how large the polynomial degree, or the number of reservoir units, must be to achieve accuracy $\varepsilon$. This is the subject of Section 26.5.
2. *It does not address random reservoirs*: The proof uses a delay-line architecture. Whether a random reservoir achieves universal approximation is a separate (and harder) question. In practice, random reservoirs work extremely well, but the theoretical guarantee comes through different arguments.
3. *It does not handle all inputs*: The compactness assumption is real. For inputs that are not in some compact set, the approximation guarantee does not apply. However, the class of compact sets in $X_w$ is rich (Proposition 26.3.2), so this is not a severe restriction in practice.
4. *Nonlinearity is needed*: Polynomial functionals of degree $\geq 2$ are genuinely nonlinear. A purely linear reservoir (without squashing nonlinearities or polynomial readout) can only approximate *linear* functionals.

## 26.3.6 Extensions

**Continuous-time version.** The theorem extends to continuous-time signals $u: (-\infty, 0] \to \mathbb{R}^d$ in $L^\infty(-\infty, 0]$ with an appropriate weighted norm. The argument is essentially the same, with the evaluation functionals replaced by time-localized measurements $u \mapsto \int u(s)\phi(s)ds$ for test functions $\phi$ [Matthews1993].

**Vector-valued outputs.** The theorem trivially extends to $\mathbb{R}^m$-valued outputs by applying it component-wise.

**Approximate vs exact ESP.** The fading memory property is weaker than the echo state property (ESP). The ESP says the reservoir state is uniquely determined by the input history. Fading memory says the *output* depends continuously on the input history. These are related: a reservoir with the ESP and a continuous readout implements a fading memory functional. But the Boyd-Chua theorem shows that fading memory is sufficient for universal approximation, so we do not need the strict ESP.

**The role of nonlinearity.** An important subtlety: the polynomial functional $P$ involves products of input values at different time steps (e.g., $u_{-k} \cdot u_{-\ell}$). These cross-terms represent *interactions* between different time steps. A linear reservoir cannot produce these terms; nonlinearity is required. In practice, this is achieved by the sigmoid or tanh activation function of the reservoir neurons — the Taylor expansion of $\tanh(w \cdot x)$ contains all polynomial terms in the components of $x$.
