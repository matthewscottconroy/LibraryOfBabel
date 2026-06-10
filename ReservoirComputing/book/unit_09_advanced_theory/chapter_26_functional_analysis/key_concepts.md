# Chapter 26: Key Concepts

**Stone-Weierstrass Theorem.** The fundamental density result for function algebras on compact Hausdorff spaces. Any subalgebra of $C(K)$ that separates points and contains the constants is dense in $C(K)$ under the uniform norm. Generalizes the classical Weierstrass polynomial approximation theorem to arbitrary compact Hausdorff spaces.

**Compact Hausdorff Space.** A topological space that is both compact (every open cover has a finite subcover) and Hausdorff (distinct points have disjoint neighborhoods). These spaces are the natural setting for Stone-Weierstrass and for the input spaces of fading-memory functionals.

**Function Algebra.** A subalgebra of $C(K)$: a set of continuous functions closed under addition, scalar multiplication, and pointwise multiplication. The Stone-Weierstrass theorem characterizes which function algebras are dense in $C(K)$.

**Separates Points.** A family $\mathcal{F}$ of functions separates points of $K$ if for every $x \neq y$ in $K$, there exists $f \in \mathcal{F}$ with $f(x) \neq f(y)$. This condition prevents the algebra from being "blind" to the difference between distinct points.

**Bernstein Polynomial.** $B_n(f)(x) = \sum_{k=0}^n f(k/n)\binom{n}{k}x^k(1-x)^{n-k}$. Provides a constructive proof of the Weierstrass theorem via the probabilistic identity $B_n(f)(x) = \mathbb{E}[f(X/n)]$ where $X \sim \text{Binomial}(n,x)$. Converges uniformly to $f$ on $[0,1]$.

**Fading Memory Property.** A functional $H: X_w \to \mathbb{R}$ has fading memory if it is continuous with respect to a weighted norm $\|\mathbf{u}\|_w = \sup_{k \geq 0} w_k |u_{-k}|$ where $w_k \to \infty$. Formalizes the notion that inputs from the distant past have diminishing influence on the current output.

**Weighted Norm / Fading Memory Space.** $\|\mathbf{u}\|_w = \sup_{k \geq 0} w_k |u_{-k}|$ with $w_k \to \infty$. The space $X_w = \{\mathbf{u} : \|\mathbf{u}\|_w < \infty\}$ consists of sequences whose values in the distant past are small enough to be down-weighted. The key compactness property: bounded sets with equismall tails are compact in $X_w$.

**Polynomial Functional.** A functional $P(\mathbf{u}) = \sum_{\text{finite}} c_{k_1 \cdots k_m} u_{-k_1} \cdots u_{-k_m}$ — a finite polynomial in evaluations of the input at finitely many past times. The polynomial functionals form an algebra that separates points of $X_w$, enabling application of Stone-Weierstrass.

**Boyd-Chua Theorem.** Any continuous (fading-memory) functional on a compact subset of $X_w$ can be uniformly approximated by polynomial functionals, and hence by a finite-dimensional dynamical system (reservoir computer). The master universal approximation theorem for reservoir computing.

**Sobolev-type Smoothness Class.** $\mathcal{F}(w, s, R)$: the class of $s$-times Fréchet-differentiable functionals whose $s$-th derivative is bounded in a weighted Sobolev sense. The smoothness parameter $s$ and the weight decay rate $\alpha$ together determine the approximation rate.

**Approximation Rate.** The function $\varepsilon(N)$ describing the worst-case error over a smoothness class when approximating with at most $N$ parameters. For functionals in $\mathcal{F}(w, s, R)$ with $w_k \geq Ck^\alpha$, the rate is $\varepsilon(N) \asymp N^{-\alpha s / d_{\text{eff}}}$.

**Curse of Dimensionality.** The phenomenon whereby approximation rates degrade exponentially with the dimension of the problem. For temporal functionals, the relevant "dimension" $d_{\text{eff}}$ reflects the number of interacting time steps in the target functional, not just the number of inputs at each time.

**Sample Complexity.** The number of training samples $T$ required to achieve generalization error $\varepsilon$. Combines the approximation error (from using a finite reservoir) and the estimation error (from learning with finite data). For reservoir computing, $T = \Omega(\varepsilon^{-2 - d_{\text{eff}}/(\alpha s)})$ under standard smoothness assumptions.

**Random Feature Map.** An $N$-unit reservoir with random weights implements a random feature map $\phi: v \mapsto (\sigma(w^{(1)\top}v), \ldots, \sigma(w^{(N)\top}v))$ that approximates a kernel feature map. This connects reservoir computing to kernel methods and provides an alternative (and often sharper) route to approximation rate bounds.

**Rademacher Complexity.** A measure of the expressiveness of a function class, used to derive generalization bounds. For a linear readout over an $N$-dimensional feature space with bounded features, the Rademacher complexity scales as $O(\sqrt{N/T})$, leading to estimation error $O(\sqrt{N/T})$ after $T$ samples.

**Hahn-Banach Theorem.** A fundamental result in functional analysis: a bounded linear functional on a subspace can be extended to the entire space without increasing its norm. Used in the proof of neural network universal approximation via the contradiction argument (a separating hyperplane in the function space would correspond to a nonzero measure annihilating all network outputs).

**Fréchet Derivative.** The generalization of the derivative to functionals on infinite-dimensional spaces. A functional $H: X \to \mathbb{R}$ is Fréchet differentiable at $\mathbf{u}$ if there exists a bounded linear map $DH(\mathbf{u}): X \to \mathbb{R}$ such that $|H(\mathbf{u} + \mathbf{h}) - H(\mathbf{u}) - DH(\mathbf{u})[\mathbf{h}]| = o(\|\mathbf{h}\|)$ as $\|\mathbf{h}\| \to 0$. Smoothness classes for functionals are defined using higher Fréchet derivatives.
