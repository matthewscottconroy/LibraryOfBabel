# Completeness and Banach Spaces

Completeness—the convergence of every Cauchy sequence—is the property that separates functional analysis from mere analysis of normed spaces. Without completeness, limiting arguments break down: sequences that ought to converge may not, and fixed point theorems fail. Banach spaces (complete normed spaces) are the natural setting for linear operator theory, and the three fundamental theorems of functional analysis (uniform boundedness, open mapping, closed graph) hold precisely for Banach spaces.

## Cauchy Sequences and Completeness

In a normed space $(X, \|\cdot\|)$, a sequence $(x_n)$ is **Cauchy** if $\|x_n - x_m\| \to 0$ as $n, m \to \infty$: for every $\varepsilon > 0$, there exists $N$ such that $\|x_n - x_m\| < \varepsilon$ for all $n, m > N$.

Every convergent sequence is Cauchy (triangle inequality: $\|x_n - x_m\| \leq \|x_n - x\| + \|x - x_m\|$), but the converse need not hold.

**Definition.** A normed space is **complete** if every Cauchy sequence converges. A complete normed space is a **Banach space**.

**Example of incompleteness.** The space $C([0,1])$ with the $L^1$ norm $\|f\|_1 = \int_0^1 |f|$ is not complete. The sequence $f_n(x) = \min(1, n \cdot \max(0, x - 1/2))$ (approximating the step function at $x = 1/2$ from below) is Cauchy in $\|\cdot\|_1$ but its limit $H(x-1/2)$ is not in $C([0,1])$.

## Key Banach Spaces

**$\mathbb{R}^n$ (or $\mathbb{C}^n$).** Complete (by the Bolzano-Weierstrass theorem applied componentwise).

**$C([a,b])$ with $\|\cdot\|_\infty$.** Complete: a uniform Cauchy sequence of continuous functions converges uniformly to a continuous function.

**$L^p(\Omega)$ for $1 \leq p \leq \infty$.** Complete by the **Riesz-Fischer theorem**:

**Theorem (Riesz-Fischer).** $L^p(\Omega)$ is complete for every $1 \leq p \leq \infty$.

The proof for $p < \infty$ uses the criterion: $X$ is complete if and only if every absolutely convergent series $\sum_{n=1}^\infty \|x_n\| < \infty$ converges in $X$. For $L^p$: if $\sum \|f_n\|_p < \infty$, then by Minkowski's inequality, $g = \sum |f_n| \in L^p$, and $\sum f_n$ converges almost everywhere and in $L^p$.

## Series in Banach Spaces

In a Banach space, absolutely convergent series converge. More precisely:

**Theorem.** A normed space $X$ is complete if and only if every absolutely convergent series $\sum_{n=1}^\infty x_n$ (i.e., $\sum \|x_n\| < \infty$) converges in $X$.

This equivalence is frequently used to prove completeness of new spaces.

## The Three Fundamental Theorems

The following three theorems hold for Banach spaces and are the cornerstones of functional analysis. They have no analogue in general incomplete normed spaces.

**Theorem (Uniform Boundedness / Banach-Steinhaus).** Let $X$ be a Banach space and $Y$ any normed space. If $\{T_\alpha\}_{\alpha \in A}$ is a family of bounded linear operators $T_\alpha: X \to Y$ such that $\sup_\alpha \|T_\alpha x\| < \infty$ for every $x \in X$, then $\sup_\alpha \|T_\alpha\| < \infty$.

The conclusion is "pointwise bounded implies uniformly bounded." The proof uses the **Baire category theorem**: a complete metric space is not a countable union of nowhere dense sets.

**Corollary.** If $T_n x \to T x$ for all $x \in X$ (strong limit), then $T$ is a bounded linear operator and $\|T\| \leq \liminf_n \|T_n\|$.

**Theorem (Open Mapping).** If $T: X \to Y$ is a bounded surjective linear operator between Banach spaces, then $T$ is an open map: it maps open sets to open sets.

**Corollary (Bounded Inverse Theorem).** If $T: X \to Y$ is a bounded bijection, then $T^{-1}: Y \to X$ is also bounded.

This corollary is the key that prevents anomalies: a continuous bijective linear operator between Banach spaces automatically has a continuous inverse.

**Theorem (Closed Graph).** Let $T: X \to Y$ be a linear operator between Banach spaces. If the graph $\{(x, Tx) : x \in X\} \subset X \times Y$ is closed, then $T$ is bounded.

The closed graph theorem is often used in PDE theory to show that an operator defined by a natural limiting argument is bounded. If $x_n \to x$ and $Tx_n \to y$, and one can show $Tx = y$ (by some property of $T$), then $T$ is bounded.

## Completion of a Normed Space

Every normed space $X$ can be **completed**: there exists a Banach space $\hat{X}$ and an isometric embedding $\iota: X \to \hat{X}$ with $\iota(X)$ dense in $\hat{X}$. The completion is unique up to isometric isomorphism.

**Examples:**
- The completion of $C_c(\mathbb{R})$ (continuous compactly supported functions) with the $L^p$ norm is $L^p(\mathbb{R})$.
- The completion of $C_c^\infty(\Omega)$ with the $H^1$ norm is $H^1_0(\Omega)$ (Sobolev space with zero boundary conditions).

Completions are the rigorous basis for "extending" operators and PDEs from smooth functions to distributional solutions.

## Application: Existence via Compactness and Completeness

Many existence proofs in PDE theory follow the pattern:
1. Construct a sequence of approximate solutions $(u_n)$ (e.g., by Galerkin approximation).
2. Show $(u_n)$ is bounded in a Banach space $X$.
3. Use compactness (relative compactness of bounded sets in a better space, via Rellich-Kondrachov) to extract a convergent subsequence.
4. Show the limit is an exact solution.

Completeness of $X$ ensures step 3 produces a limit in $X$. Without completeness, the limit might not belong to the space, and the argument fails. This pattern—boundedness, extraction of convergent subsequence, identification of the limit—is the backbone of modern PDE existence theory.
