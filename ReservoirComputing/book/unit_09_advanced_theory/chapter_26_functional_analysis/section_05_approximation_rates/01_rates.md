# Section 26.5: Approximation Rates and Sample Complexity

## 26.5.1 From Existence to Rates

The Boyd-Chua theorem guarantees that any fading-memory functional can be approximated, but it says nothing about *how many* reservoir units are needed to achieve a given accuracy $\varepsilon$. This is the question of approximation rates, and it is both harder and more practically relevant than the existence result.

We frame the problem precisely. Suppose the target functional $H$ belongs to some smoothness class $\mathcal{F}$ (to be defined). An $N$-unit reservoir implements a functional $\hat{H}_N$ in some approximation class $\mathcal{G}_N$. The approximation rate is the function $\varepsilon(N) = \inf_{P \in \mathcal{G}_N} \sup_{H \in \mathcal{F}} \|H - P\|$, which tells us how quickly the worst-case error decreases as $N$ grows.

We address three questions:
1. What is the appropriate smoothness class for functionals? (Sobolev-type spaces for sequences.)
2. What are upper bounds on $\varepsilon(N)$, and when are they tight?
3. What does this mean for the number of reservoir units required in practice?

## 26.5.2 Smoothness Classes for Functionals

For scalar functions $f: \mathbb{R}^d \to \mathbb{R}$, the standard smoothness class is the Sobolev space $W^{s,p}$ of functions with $s$ weak derivatives in $L^p$. For functionals on sequence spaces, we need an analogous notion.

**Definition 26.5.1 (Sobolev-type Functional Class).** Fix a weight sequence $w = (w_k)_{k \geq 0}$ with $w_k > 0$ and smoothness parameters $s \geq 1$, $R > 0$. Define the class $\mathcal{F}(w, s, R)$ to be all functionals $H: X_w \to \mathbb{R}$ such that:

1. $H$ is $s$-times Fréchet differentiable on $X_w$.
2. The $s$-th Fréchet derivative $D^s H$ satisfies:
$$\sum_{k_1, \ldots, k_s \geq 0} \left|\frac{\partial^s H}{\partial u_{-k_1} \cdots \partial u_{-k_s}}\right|^2 w_{k_1}^2 \cdots w_{k_s}^2 \leq R^2.$$

This condition says the functional has $s$ "smooth" derivatives in the weighted sense: the higher-order sensitivity of $H$ to inputs far in the past decays at rate prescribed by $w$.

**Example 26.5.1 (Volterra Series).** A functional given by a Volterra series of order $s$:
$$H(\mathbf{u}) = h_0 + \sum_{k=0}^{K} h_1(k) u_{-k} + \sum_{k,\ell=0}^{K} h_2(k,\ell) u_{-k} u_{-\ell} + \cdots + \sum_{k_1,\ldots,k_s} h_s(k_1,\ldots,k_s) u_{-k_1} \cdots u_{-k_s}$$
belongs to $\mathcal{F}(w, s, R)$ provided the Volterra kernels $h_j$ decay appropriately. The condition $\sum |h_s(k_1,\ldots,k_s)|^2 w_{k_1}^2 \cdots w_{k_s}^2 \leq R^2$ is the key constraint.

## 26.5.3 Approximation by Polynomial Functionals: Upper Bounds

**Theorem 26.5.1 (Approximation Rate, Upper Bound).** *Let $H \in \mathcal{F}(w, s, R)$ and suppose $w_k \geq C k^\alpha$ for some $C, \alpha > 0$. Then there exists a polynomial functional $P_N$ of degree $s$ depending on at most $N$ past input values such that*
$$\sup_{\mathbf{u} \in K} |H(\mathbf{u}) - P_N(\mathbf{u})| \leq C(s, \alpha, R) \cdot N^{-\alpha s/d_{\text{eff}}}$$
*where $d_{\text{eff}}$ is an effective dimension related to the interaction structure of $H$.*

The proof uses Taylor expansion: expand $H(\mathbf{u})$ in a Taylor series around zero, truncate at degree $s$ and at lag $N$, and bound the truncation error using the smoothness assumption.

**Proof sketch.** For the linear term:
$$\left|\sum_{k > N} \frac{\partial H}{\partial u_{-k}}\bigg|_{\mathbf{0}} u_{-k}\right| \leq \left(\sum_{k > N} \left|\frac{\partial H}{\partial u_{-k}}\right|^2 w_k^2\right)^{1/2} \left(\sum_{k>N} \frac{|u_{-k}|^2}{w_k^2}\right)^{1/2}.$$
The first factor is bounded by $R$ (smoothness assumption). The second factor is bounded by $\|\mathbf{u}\|_w^2 / w_N^2$. If $w_N \geq CN^\alpha$, then this is $O(N^{-\alpha})$. The same argument applies to each degree in the polynomial expansion, yielding the stated rate. $\blacksquare$

**Remark 26.5.1 (Curse of Dimensionality).** The rate degrades with the degree $s$ and the dimension of the input. For a functional that depends on $d$ inputs simultaneously (i.e., has an $s$-th order term involving all $d$ variables), the approximation rate is $O(N^{-s/d})$ — the classical curse of dimensionality for polynomial approximation in $d$ dimensions. For temporal functionals, $d$ is replaced by the effective number of interacting time steps.

## 26.5.4 Lower Bounds

Upper bounds tell us the best achievable rate; lower bounds tell us we cannot do better.

**Theorem 26.5.2 (Approximation Rate, Lower Bound).** *There exists a class $\mathcal{F}(w, s, R)$ (satisfying the conditions of Theorem 26.5.1) such that for any approximation scheme using polynomial functionals of degree $\leq s$ and at most $N$ past values,*
$$\sup_{H \in \mathcal{F}} \|H - P_N\| \geq c(s, \alpha, R) \cdot N^{-\alpha s}.$$

The lower bound is proved by constructing a "hard" functional in $\mathcal{F}$ that requires many time steps to approximate. The key idea is a *packing argument*: there exist $M$ functionals in $\mathcal{F}$ that are mutually far apart ($\|H_i - H_j\| \geq \delta$ for all $i \neq j$), but any polynomial functional with fewer than $N$ parameters can be close to at most a fraction of them. A counting argument then gives the lower bound.

## 26.5.5 Sample Complexity: How Large Must the Reservoir Be?

In practice, we have training data: $T$ input-output pairs $(\mathbf{u}^{(1)}, y^{(1)}), \ldots, (\mathbf{u}^{(T)}, y^{(T)})$ sampled from some distribution $\mu$ on $K \times \mathbb{R}$. We wish to learn a readout $W$ such that the reservoir output $W x$ approximates $y$ with small expected error.

The sample complexity problem has two components:
1. **Approximation error**: Even with infinite data, the best linear readout of the $N$-dimensional reservoir state incurs approximation error $\varepsilon_{\text{approx}}(N)$.
2. **Estimation error**: With finite $T$ samples, the learned readout incurs estimation error $\varepsilon_{\text{est}}(T, N)$ on top of the approximation error.

**Theorem 26.5.3 (Generalization Bound).** *Let the reservoir have $N$ units, and let the readout be learned by ridge regression with regularization $\lambda > 0$. Assume the reservoir states $x^{(t)}$ are uniformly bounded by $B_x$ and the target outputs by $B_y$. Then with probability at least $1 - \delta$ over the training data,*
$$\mathbb{E}_\mu\left[(y - Wx)^2\right] \leq \varepsilon_{\text{approx}}(N)^2 + C \sqrt{\frac{N \log(N/\delta)}{T}} \cdot B_x B_y + \lambda \|W^*\|^2$$
*where $W^* = \arg\min_W \mathbb{E}[(y - Wx)^2]$ is the optimal readout and $C$ is a universal constant.*

**Proof sketch.** The proof uses the standard bias-variance decomposition:
- **Bias term**: $\varepsilon_{\text{approx}}(N)^2$ comes from the approximation theory of Section 26.5.3.
- **Variance term**: The estimation error is controlled by a Rademacher complexity argument. For a linear readout over an $N$-dimensional feature space with bounded features, the Rademacher complexity of the function class $\{x \mapsto Wx : \|W\|_F \leq R\}$ is $O(R B_x \sqrt{N/T})$. By the Rademacher generalization bound (see Section 27.3 for related matrix Bernstein tools), the deviation between empirical and expected loss is bounded as stated.
- **Regularization term**: Ridge regression introduces a bias proportional to $\lambda\|W^*\|^2$ but reduces the variance term by shrinking $W$ toward zero.

**Setting $\lambda$ optimally**: Choose $\lambda \propto \sqrt{N/T}$ to balance the variance and regularization terms. This gives a total estimation error of $O((N/T)^{1/4} B_x B_y \|W^*\|^{1/2})$.

**Corollary 26.5.1 (Required Reservoir Size).** *Suppose the target functional belongs to $\mathcal{F}(w, s, R)$ with $w_k \geq Ck^\alpha$, and we have $T$ training samples. To achieve total error $\varepsilon$, we need:*
$$N = \Omega\!\left(\varepsilon^{-d_{\text{eff}}/(\alpha s)}\right) \quad \text{units}$$
$$T = \Omega\!\left(N \cdot \varepsilon^{-2} \log(1/\varepsilon)\right) \quad \text{samples.}$$

*Combining:*
$$T = \Omega\!\left(\varepsilon^{-2 - d_{\text{eff}}/(\alpha s)} \log(1/\varepsilon)\right).$$

This is the reservoir computing analogue of classical sample complexity bounds. Several observations:

1. **Smoothness helps**: Higher $s$ (smoother functional) and larger $\alpha$ (faster weight growth, hence stronger fading memory) both reduce the required $N$ and $T$.
2. **Curse of dimensionality**: The $d_{\text{eff}}$ factor in the exponent reflects the effective number of interacting time steps in the functional. Functionals with low-rank interaction structure (e.g., additive functionals $H(\mathbf{u}) = \sum_k h(u_{-k})$) have $d_{\text{eff}} = 1$ and are much easier to approximate.
3. **Comparison to deep networks**: For feedforward networks approximating functions on $\mathbb{R}^d$, the sample complexity is $T = \Omega(\varepsilon^{-2-d/s})$. Our bound has an analogous form, with the spatial dimension $d$ replaced by the effective temporal dimension $d_{\text{eff}}$.

## 26.5.6 The Role of the Reservoir's Nonlinearity

The bounds above implicitly assume that the reservoir state $x$ already contains all polynomial features of the input up to the relevant degree. In practice, a reservoir with $\tanh$ nonlinearities approximates this through the Taylor expansion of $\tanh$.

**Lemma 26.5.1 (Taylor Approximation of Reservoir Features).** *Let $\sigma = \tanh$ and consider a single reservoir unit with weight vector $w \in \mathbb{R}^N$ and input $v \in \mathbb{R}^N$. Then*
$$\sigma(w^\top v) = \sum_{j \text{ odd}} \frac{\sigma^{(j)}(0)}{j!} (w^\top v)^j.$$
*Expanding $(w^\top v)^j = \sum_{|\alpha|=j} \frac{j!}{\alpha!} w^\alpha v^\alpha$ (multi-index notation), each monomial $v^\alpha$ of degree $j$ appears with coefficient $\frac{\sigma^{(j)}(0)}{\alpha!} w^\alpha$.*

This shows that a single $\tanh$ unit with a random weight vector $w$ generates a weighted random combination of all odd-degree monomials in $v$. For degree-$d$ polynomial approximation, one needs enough units to "cover" all relevant monomials — which is why the required reservoir size grows with the degree $d$.

**Proposition 26.5.1 (Reservoir as Random Feature Map).** *An $N$-unit reservoir with random weights $w^{(j)} \sim \mathcal{N}(0, \sigma_w^2 I)$ implements a random feature map $\phi: \mathbb{R}^L \to \mathbb{R}^N$ (where $L$ is the number of past inputs used). Under suitable conditions on $\sigma_w$, the random features $\phi(v) = (\sigma(w^{(1)\top} v), \ldots, \sigma(w^{(N)\top} v))$ approximate a kernel feature map corresponding to an infinite-width reservoir.*

This connects to the random features literature [Rahimi2007], where it is shown that $N$ random features suffice to approximate a kernel regression with error $O(1/\sqrt{N})$ in the regression function. For reservoir computing, this suggests that $N = O(\varepsilon^{-2})$ units suffice for approximation to accuracy $\varepsilon$ — a much faster rate than the polynomial approximation bound, but only when the target functional lies in the RKHS of the limiting kernel.

## 26.5.7 Open Questions in Approximation Rates

The theory developed here has several important gaps, which we flag honestly:

1. **Tight bounds for random reservoirs**: The bounds in Theorems 26.5.1 and 26.5.2 apply to polynomial functionals. For random reservoirs specifically, tight upper and lower bounds on approximation rates are not known for general smoothness classes. The random feature perspective (Proposition 26.5.1) gives suggestive results but does not close the gap.

2. **The role of reservoir dynamics**: Our bounds treat the reservoir state as a fixed feature map. In reality, the reservoir dynamics create temporal correlations between states, which may help or hurt approximation. A theory of approximation rates for *dynamical* reservoirs (as opposed to static feature maps) remains largely open.

3. **Optimal weight sequences**: For a given target class $\mathcal{F}$, what weight sequence $w$ minimizes the required reservoir size? This is an optimal design problem that has not been fully analyzed.

4. **Non-compact input sets**: What happens when inputs are not confined to a compact set? Generalization bounds involving sub-Gaussian tail assumptions on inputs have been studied [Gonon2020], but a complete theory is lacking.

These open questions are revisited in Chapter 34, where we discuss the most important unsolved problems in reservoir computing theory.
