# Section 26.1: The Stone-Weierstrass Theorem

## 26.1.1 The Classical Weierstrass Theorem

We begin with the classical result, proved by Karl Weierstrass in 1885, before generalizing it substantially.

**Theorem 26.1.1 (Weierstrass Approximation Theorem).** *Let $f: [a,b] \to \mathbb{R}$ be continuous. For every $\varepsilon > 0$, there exists a polynomial $p$ such that*
$$\sup_{x \in [a,b]} |f(x) - p(x)| < \varepsilon.$$

This theorem is striking: continuous functions, which can be arbitrarily "wild" (oscillating, non-differentiable), can always be uniformly approximated by smooth polynomials. The proof we give now uses *Bernstein polynomials*, a constructive approach that will also illuminate the probabilistic nature of the approximation.

**Definition 26.1.1 (Bernstein Polynomial).** For $f: [0,1] \to \mathbb{R}$ and $n \geq 1$, the $n$-th Bernstein polynomial of $f$ is
$$B_n(f)(x) = \sum_{k=0}^{n} f\!\left(\frac{k}{n}\right) \binom{n}{k} x^k (1-x)^{n-k}.$$

The combinatorial weights $\binom{n}{k} x^k (1-x)^{n-k}$ are the binomial probabilities: if $X \sim \text{Binomial}(n, x)$, then $B_n(f)(x) = \mathbb{E}[f(X/n)]$.

**Proof of Theorem 26.1.1.** Without loss of generality take $[a,b] = [0,1]$. We claim $B_n(f) \to f$ uniformly as $n \to \infty$.

Since $f$ is continuous on a compact set, it is uniformly continuous. Let $\varepsilon > 0$ and choose $\delta > 0$ such that $|x - y| < \delta$ implies $|f(x) - f(y)| < \varepsilon/2$. Also let $M = \sup |f|$.

For fixed $x \in [0,1]$, let $X_1, \ldots, X_n$ be i.i.d. Bernoulli($x$) random variables and $S_n = X_1 + \cdots + X_n$, so $S_n/n \sim \text{Binomial}(n,x)/n$. Then:
$$B_n(f)(x) - f(x) = \mathbb{E}\!\left[f(S_n/n) - f(x)\right].$$

Split the expectation based on whether $|S_n/n - x| < \delta$:
$$|B_n(f)(x) - f(x)| \leq \mathbb{E}\!\left[|f(S_n/n) - f(x)|\right]$$
$$= \mathbb{E}\!\left[|f(S_n/n) - f(x)|\, \mathbf{1}_{|S_n/n - x| < \delta}\right] + \mathbb{E}\!\left[|f(S_n/n) - f(x)|\, \mathbf{1}_{|S_n/n - x| \geq \delta}\right].$$

The first term is at most $\varepsilon/2$. The second term is at most $2M \cdot \mathbb{P}(|S_n/n - x| \geq \delta)$. By Chebyshev's inequality and the fact that $\text{Var}(S_n/n) = x(1-x)/n \leq 1/(4n)$:
$$\mathbb{P}(|S_n/n - x| \geq \delta) \leq \frac{1}{4n\delta^2}.$$

Therefore:
$$|B_n(f)(x) - f(x)| \leq \frac{\varepsilon}{2} + \frac{2M}{4n\delta^2} = \frac{\varepsilon}{2} + \frac{M}{2n\delta^2}.$$

For $n > M/(\varepsilon \delta^2)$, this bound is less than $\varepsilon$, uniformly in $x$. $\blacksquare$

## 26.1.2 Algebras of Continuous Functions

The Weierstrass theorem generalizes enormously. The key abstraction is the concept of a *function algebra*.

**Definition 26.1.2 (Algebra of Functions).** Let $K$ be a compact Hausdorff space and $C(K)$ the space of continuous real-valued functions on $K$, equipped with the uniform norm $\|f\|_\infty = \sup_{x \in K} |f(x)|$. A subset $\mathcal{A} \subseteq C(K)$ is a *subalgebra* if it is closed under:
1. Addition: $f, g \in \mathcal{A} \Rightarrow f + g \in \mathcal{A}$,
2. Scalar multiplication: $f \in \mathcal{A},\ c \in \mathbb{R} \Rightarrow cf \in \mathcal{A}$,
3. Pointwise multiplication: $f, g \in \mathcal{A} \Rightarrow fg \in \mathcal{A}$.

$\mathcal{A}$ *separates points* if for every $x \neq y$ in $K$, there exists $f \in \mathcal{A}$ with $f(x) \neq f(y)$.

$\mathcal{A}$ *contains the constants* if the constant function $\mathbf{1} \in \mathcal{A}$.

The polynomial algebra on $[a,b]$ clearly separates points (the function $f(x) = x$ distinguishes any two points) and contains constants. Stone's theorem says these two properties — along with being a subalgebra — are sufficient to guarantee density in $C(K)$.

## 26.1.3 The Stone-Weierstrass Theorem

**Theorem 26.1.2 (Stone-Weierstrass Theorem).** *Let $K$ be a compact Hausdorff space and $\mathcal{A} \subseteq C(K)$ a subalgebra that separates points and contains the constants. Then $\mathcal{A}$ is dense in $C(K)$ in the uniform norm: for every $f \in C(K)$ and $\varepsilon > 0$, there exists $g \in \mathcal{A}$ with $\|f - g\|_\infty < \varepsilon$.*

We prove this theorem in full. The proof has three stages: (i) the closure $\overline{\mathcal{A}}$ is a *lattice*, (ii) $\overline{\mathcal{A}}$ can approximate any piecewise-linear function, and (iii) density follows.

**Lemma 26.1.1.** *Let $\mathcal{A}$ be as above and $\overline{\mathcal{A}}$ its closure in $C(K)$. If $f \in \overline{\mathcal{A}}$, then $|f| \in \overline{\mathcal{A}}$.*

**Proof of Lemma.** It suffices to show that if $f \in \overline{\mathcal{A}}$, then $|f|$ can be uniformly approximated by elements of $\overline{\mathcal{A}}$ (since $\overline{\mathcal{A}}$ is itself closed). Note that $|t| = \sqrt{t^2}$. We use the fact that $\sqrt{t^2} = |t|$ can be uniformly approximated by polynomials in $t^2$ on any bounded interval.

More precisely: let $\|f\|_\infty \leq M$. Then $|f(x)| = M\sqrt{(f(x)/M)^2}$. We need to approximate $s \mapsto \sqrt{s}$ uniformly on $[0,1]$. By the classical Weierstrass theorem (applied to $[0,1]$), for any $\varepsilon > 0$ there exists a polynomial $p$ such that $|p(s) - \sqrt{s}| < \varepsilon/M$ for all $s \in [0,1]$. Then the function $x \mapsto M \cdot p((f(x)/M)^2)$ is in $\overline{\mathcal{A}}$ (since $\overline{\mathcal{A}}$ is an algebra closed under limits) and approximates $|f|$ uniformly within $\varepsilon$. $\blacksquare$

**Corollary 26.1.1.** *$\overline{\mathcal{A}}$ is a *lattice*: if $f, g \in \overline{\mathcal{A}}$, then $\max(f,g) \in \overline{\mathcal{A}}$ and $\min(f,g) \in \overline{\mathcal{A}}$.*

**Proof.** Use the identities $\max(f,g) = \frac{1}{2}(f + g + |f-g|)$ and $\min(f,g) = \frac{1}{2}(f + g - |f-g|)$. Since $f - g \in \overline{\mathcal{A}}$, Lemma 26.1.1 gives $|f-g| \in \overline{\mathcal{A}}$, and the result follows from the linear structure. $\blacksquare$

**Lemma 26.1.2 (Pointwise Interpolation).** *For any $x, y \in K$ with $x \neq y$ and any $\alpha, \beta \in \mathbb{R}$, there exists $h \in \mathcal{A}$ with $h(x) = \alpha$ and $h(y) = \beta$.*

**Proof.** Since $\mathcal{A}$ separates points, there exists $\phi \in \mathcal{A}$ with $\phi(x) \neq \phi(y)$. Set
$$h = \alpha + (\beta - \alpha) \cdot \frac{\phi - \phi(x)}{\phi(y) - \phi(x)}.$$
This is a linear combination and shift of $\phi$, hence in $\mathcal{A}$ (since $\mathcal{A}$ contains constants and is a vector space). It satisfies $h(x) = \alpha$ and $h(y) = \beta$. $\blacksquare$

**Proof of Stone-Weierstrass Theorem.** Let $f \in C(K)$ and $\varepsilon > 0$.

**Step 1: Local approximation.** Fix $y \in K$. For each $x \in K$, by Lemma 26.1.2 choose $h_{x,y} \in \mathcal{A}$ such that $h_{x,y}(x) = f(x)$ and $h_{x,y}(y) = f(y)$. By continuity of $h_{x,y}$ and $f$, the set
$$U_{x,y} = \{z \in K : h_{x,y}(z) > f(z) - \varepsilon\}$$
is open and contains $x$. The family $\{U_{x,y}\}_{x \in K}$ is an open cover of $K$. By compactness, extract a finite subcover: $K \subseteq U_{x_1,y} \cup \cdots \cup U_{x_m,y}$ for some $x_1, \ldots, x_m$.

**Step 2: Combining for fixed $y$.** Define $g_y = \max(h_{x_1,y}, \ldots, h_{x_m,y})$. By Corollary 26.1.1, $g_y \in \overline{\mathcal{A}}$. By construction, for every $z \in K$:
$$g_y(z) > f(z) - \varepsilon.$$
Also, for each $x_i$, $h_{x_i,y}(y) = f(y)$, so $g_y(y) = f(y)$ (since all $h_{x_i,y}(y) = f(y)$).

By continuity of $g_y - f$, the set
$$V_y = \{z \in K : g_y(z) < f(z) + \varepsilon\}$$
is open and contains $y$.

**Step 3: Global combination.** The family $\{V_y\}_{y \in K}$ covers $K$. Extract a finite subcover: $K \subseteq V_{y_1} \cup \cdots \cup V_{y_\ell}$. Define
$$g = \min(g_{y_1}, \ldots, g_{y_\ell}).$$
Again by Corollary 26.1.1, $g \in \overline{\mathcal{A}}$.

For any $z \in K$, $z \in V_{y_j}$ for some $j$, so $g(z) \leq g_{y_j}(z) < f(z) + \varepsilon$. For any $z \in K$ and any $i$, $g(z) \geq g_{y_i}(z) > f(z) - \varepsilon$ does not quite follow — wait. We have $g_{y_i}(z) > f(z) - \varepsilon$ for all $z$ (from Step 2). Therefore $g(z) = \min_i g_{y_i}(z) > f(z) - \varepsilon$.

Hence $|g(z) - f(z)| < \varepsilon$ for all $z \in K$. Since $\varepsilon$ was arbitrary, $f \in \overline{\mathcal{A}}$, so $\overline{\mathcal{A}} = C(K)$. $\blacksquare$

**Remark 26.1.1.** The compactness of $K$ is essential and is used twice: once to extract a finite subcover in Step 1, and once in Step 3. Without compactness, Stone-Weierstrass fails. For example, polynomials are not dense in $C_b(\mathbb{R})$ (bounded continuous functions on all of $\mathbb{R}$) because $\sin(x)$ cannot be uniformly approximated by polynomials on all of $\mathbb{R}$.

**Remark 26.1.2 (Complex Version).** For complex-valued functions, one must add the assumption that $\mathcal{A}$ is closed under complex conjugation. Without this, the result can fail: the analytic functions on the disk do not form a dense algebra in $C(\mathbb{D})$ for continuous functions on the closed disk.

## 26.1.4 Application: Polynomial Approximation

The Stone-Weierstrass theorem immediately recovers classical results and yields powerful extensions.

**Corollary 26.1.2 (Multivariate Weierstrass).** *Polynomials in $d$ variables are dense in $C(K)$ for any compact $K \subseteq \mathbb{R}^d$.*

**Proof.** The algebra of polynomials in $d$ variables contains constants (the constant function 1), separates points (the functions $x \mapsto x_i$ separate points in $\mathbb{R}^d$), and is closed under products. Apply Stone-Weierstrass. $\blacksquare$

**Corollary 26.1.3 (Trigonometric Approximation).** *Trigonometric polynomials $\sum_{k=-n}^{n} c_k e^{ikx}$ are dense in $C(\mathbb{T})$, where $\mathbb{T} = \mathbb{R}/2\pi\mathbb{Z}$ is the circle.*

**Proof.** The trigonometric polynomials form an algebra (products of exponentials are exponentials), contain constants ($e^{0} = 1$), and separate points on $\mathbb{T}$ (the function $e^{ix}$ distinguishes any two points). Apply Stone-Weierstrass. $\blacksquare$

## 26.1.5 Application: Neural Network Universal Approximation

The Stone-Weierstrass framework provides a clean route to understanding neural network universal approximation. We present the Cybenko-type result [Cybenko1989] and its sharper descendants.

Let $\sigma: \mathbb{R} \to \mathbb{R}$ be a continuous, non-constant, bounded function (a *sigmoidal* or *activation* function). Define the class of single-hidden-layer networks:
$$\Sigma_n = \left\{ x \mapsto \sum_{j=1}^{n} c_j \sigma(w_j \cdot x + b_j) : c_j, b_j \in \mathbb{R},\ w_j \in \mathbb{R}^d \right\}.$$

**Theorem 26.1.3 (Cybenko 1989; Hornik 1991).** *The union $\bigcup_{n \geq 1} \Sigma_n$ is dense in $C(K)$ for any compact $K \subseteq \mathbb{R}^d$, provided $\sigma$ is continuous and non-polynomial.*

The Stone-Weierstrass approach to this theorem [Leshno1993] works as follows:

**Proof sketch.** Suppose toward contradiction that $\overline{\bigcup_n \Sigma_n} \neq C(K)$. By the Hahn-Banach theorem, there exists a nonzero bounded linear functional $\mu$ on $C(K)$ (realized as a signed measure by Riesz representation) such that $\int \sigma(w \cdot x + b)\, d\mu(x) = 0$ for all $w, b$.

The function $w \mapsto \int \sigma(w \cdot x + b)\, d\mu(x) = 0$ for all $w$ means $\hat{\mu}$ (the Fourier transform of $\mu$ in some sense) vanishes on a dense set. One shows by a Fourier-analytic argument that this forces $\mu = 0$, contradicting our assumption. $\blacksquare$

The Stone-Weierstrass approach gives a more direct proof when $\sigma$ is chosen so that the network algebra separates points. If $\sigma$ is strictly monotone, then for any $x \neq y$, the function $t \mapsto \sigma(t\cdot(x-y) + b)$ separates $x$ and $y$ for appropriate $t, b$. The span of $\{\sigma(w \cdot x + b)\}$ forms an algebra under the right conditions, and Stone-Weierstrass delivers density.

**Remark 26.1.3 (Depth vs Width).** Stone-Weierstrass is an existence result; it does not quantify the number of neurons $n$ required. Approximation rate theory (Section 26.5) addresses this. The theorem also says nothing about the difficulty of finding the optimal coefficients $c_j, w_j, b_j$ — that is a question about optimization, not approximation.

## 26.1.6 The Uniform Algebra Perspective

A *uniform algebra* is a subalgebra of $C(K)$ that is closed in the uniform norm. Stone-Weierstrass characterizes which uniform algebras equal all of $C(K)$: they are precisely the ones that separate points and contain constants. Any proper closed subalgebra fails one of these conditions.

This perspective is powerful for reservoir computing. The question "can reservoirs approximate any functional?" becomes: "does the collection of reservoir functionals form a subalgebra that separates points (in some appropriate function space) and contains constants?" The Boyd-Chua theorem (Section 26.3) answers this in the affirmative, under the fading memory assumption.

The connection between Stone-Weierstrass and reservoir computing is not metaphorical. The proof of the Boyd-Chua theorem literally applies Stone-Weierstrass — after establishing that the fading memory condition endows the relevant input space with a compact topology.
