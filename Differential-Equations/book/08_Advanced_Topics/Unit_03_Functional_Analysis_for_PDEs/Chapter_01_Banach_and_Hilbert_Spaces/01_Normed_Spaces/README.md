# Normed Spaces

A normed space is a vector space equipped with a notion of length. This seemingly simple addition has profound consequences: it introduces a topology, enabling one to speak of convergence, continuity, and limits. Normed spaces are the foundational objects of functional analysis, and the function spaces used in PDE theory—$L^p$, Sobolev spaces, spaces of continuous functions—are all normed spaces.

## Definition and First Examples

**Definition.** A **normed space** (or **normed vector space**) is a pair $(X, \|\cdot\|)$ where $X$ is a real (or complex) vector space and $\|\cdot\|: X \to [0,\infty)$ satisfies:

1. **Positive definiteness:** $\|x\| = 0 \iff x = 0$.
2. **Homogeneity:** $\|\lambda x\| = |\lambda|\|x\|$ for all scalars $\lambda$ and $x \in X$.
3. **Triangle inequality:** $\|x + y\| \leq \|x\| + \|y\|$ for all $x, y \in X$.

The norm induces a metric $d(x,y) = \|x-y\|$, making $X$ a metric space.

**Examples:**
- $\mathbb{R}^n$ with $\|x\|_p = (\sum |x_i|^p)^{1/p}$ for $1 \leq p < \infty$, or $\|x\|_\infty = \max_i|x_i|$.
- $C([a,b])$ (continuous functions on $[a,b]$) with $\|f\|_\infty = \max_{[a,b]}|f(x)|$.
- $C^k([a,b])$ with $\|f\|_{C^k} = \sum_{j=0}^k \max|f^{(j)}|$.
- $L^p(\Omega)$ for a measurable set $\Omega \subset \mathbb{R}^n$ with $\|f\|_p = (\int_\Omega |f|^p)^{1/p}$, for $1 \leq p < \infty$.
- $L^\infty(\Omega)$ with $\|f\|_\infty = \text{ess}\sup_\Omega|f|$.

## Hölder and Minkowski Inequalities

Two fundamental inequalities for $L^p$ spaces:

**Hölder's inequality.** For $1/p + 1/q = 1$ (conjugate exponents, $p, q \in [1,\infty]$):
$$\|fg\|_1 \leq \|f\|_p \|g\|_q, \quad \text{i.e., } \int |fg| \leq \left(\int |f|^p\right)^{1/p}\left(\int |g|^q\right)^{1/q}.$$

**Minkowski's inequality.** For $1 \leq p \leq \infty$:
$$\|f + g\|_p \leq \|f\|_p + \|g\|_p.$$

Minkowski's inequality is precisely the triangle inequality for $\|\cdot\|_p$, confirming that $L^p$ is a normed space.

## Equivalent Norms

Two norms $\|\cdot\|_1$ and $\|\cdot\|_2$ on a vector space $X$ are **equivalent** if there exist constants $c, C > 0$ with $c\|x\|_1 \leq \|x\|_2 \leq C\|x\|_1$ for all $x$. Equivalent norms define the same topology (same open sets, convergent sequences, continuous maps). In finite dimensions, all norms are equivalent. In infinite dimensions, different norms on the same space may be inequivalent, giving genuinely different topological structures.

**Example.** On $C([0,1])$, $\|\cdot\|_\infty$ and $\|\cdot\|_{L^2}$ are inequivalent: the sequence $f_n(x) = x^n$ satisfies $\|f_n\|_\infty = 1$ but $\|f_n\|_{L^2} = 1/\sqrt{2n+1} \to 0$.

## The Hahn-Banach Theorem

A **bounded linear functional** on a normed space $X$ is a linear map $\Lambda: X \to \mathbb{R}$ (or $\mathbb{C}$) with $\|\Lambda\| = \sup_{\|x\|=1}|\Lambda(x)| < \infty$. The space $X^* = B(X, \mathbb{R})$ of bounded linear functionals is the **dual space** of $X$. It is always a Banach space (even if $X$ is not complete).

**Theorem (Hahn-Banach).** Let $Y$ be a subspace of a normed space $X$, and $\Lambda: Y \to \mathbb{R}$ a bounded linear functional. Then $\Lambda$ extends to a bounded linear functional $\tilde{\Lambda}: X \to \mathbb{R}$ with $\|\tilde{\Lambda}\| = \|\Lambda\|$.

**Consequences:**
- For every $x_0 \neq 0$ in $X$, there exists $\Lambda \in X^*$ with $\|\Lambda\| = 1$ and $\Lambda(x_0) = \|x_0\|$.
- The dual space $X^*$ separates points of $X$: if $\Lambda(x) = \Lambda(y)$ for all $\Lambda \in X^*$, then $x = y$.
- Closed convex sets in $X$ can be separated by hyperplanes (the geometric Hahn-Banach theorem).

## Convergence in Normed Spaces

Two notions of convergence are important:

**Strong convergence.** $x_n \to x$ in $X$ means $\|x_n - x\| \to 0$.

**Weak convergence.** $x_n \rightharpoonup x$ means $\Lambda(x_n) \to \Lambda(x)$ for all $\Lambda \in X^*$.

Strong convergence implies weak convergence, but not conversely in infinite dimensions. The failure of the converse is the source of many subtleties in PDE analysis.

**Example.** In $L^2([0,1])$, $f_n(x) = \sin(2\pi nx) \rightharpoonup 0$ weakly (by the Riemann-Lebesgue lemma: $\int_0^1 \sin(2\pi nx) g(x) \, dx \to 0$ for all $g \in L^2$), but $\|f_n\|_{L^2} = 1/\sqrt{2}$ for all $n$, so $f_n$ does not converge strongly to 0.

## Compact Sets and Compactness

A subset $K \subset X$ is **compact** if every sequence in $K$ has a convergent subsequence (sequential compactness; in metric spaces, equivalent to compactness). In finite-dimensional normed spaces, closed bounded sets are compact (Heine-Borel). In infinite dimensions, closed bounded sets are NOT compact.

**Ascoli-Arzelà theorem.** A subset $\mathcal{F} \subset C([a,b])$ is relatively compact (closure is compact) if and only if $\mathcal{F}$ is uniformly bounded and equicontinuous.

Compact sets are crucial in PDE theory: they appear in the Rellich-Kondrachov theorem (compact embedding of Sobolev spaces), the theory of compact operators, and in arguments showing that sequences of approximate solutions have convergent subsequences.

## Applications to PDE: Function Spaces

For PDE theory, the most important normed spaces are:

- $H = L^2(\Omega)$ (Hilbert, for $L^2$ energy methods).
- $V = H^1(\Omega) = W^{1,2}(\Omega)$ (Hilbert, for first-order energy).
- $V_0 = H^1_0(\Omega)$ (homogeneous Dirichlet conditions, closed subspace of $H^1$).
- $H^{-1}(\Omega) = (H^1_0(\Omega))^*$ (dual, for distributional right-hand sides).

The Lax-Milgram theorem, proved in Section 4, establishes existence and uniqueness of solutions in $H^1_0$ for elliptic PDEs, given data in $H^{-1}$.
