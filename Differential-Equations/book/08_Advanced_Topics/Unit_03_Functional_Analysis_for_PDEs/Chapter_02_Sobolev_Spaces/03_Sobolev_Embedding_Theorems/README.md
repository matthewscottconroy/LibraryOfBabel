# Sobolev Embedding Theorems

A function in $W^{k,p}(\Omega)$ has $k$ weak derivatives in $L^p$. But what does this say about the function itself? Can it be continuous? Hölder continuous? Can it be bounded? The Sobolev embedding theorems answer these questions by showing that $W^{k,p}$ functions are, in fact, elements of other (better) function spaces—and the space they land in depends on the relationship between $k$, $p$, and the dimension $n$. These theorems are indispensable for converting the abstract existence results of PDE theory (which give solutions in $H^1$ or $H^2$) into regularity statements (the solution is continuous, differentiable, etc.).

## The Critical Exponent

The key quantity governing Sobolev embeddings is the **Sobolev exponent**:

$$p^* = \frac{np}{n - kp} \quad \text{(when } kp < n\text{)}.$$

This is the endpoint of the critical embedding $W^{k,p}(\mathbb{R}^n) \hookrightarrow L^{p^*}(\mathbb{R}^n)$. The formula has a natural scaling derivation: if $u \in W^{k,p}$ and $u_\lambda(x) = u(\lambda x)$, then $\|u_\lambda\|_{W^{k,p}} \sim \lambda^{k - n/p}$ and $\|u_\lambda\|_{L^q} \sim \lambda^{-n/q}$, so a scale-invariant bound $\|u\|_{L^q} \lesssim \|u\|_{W^{k,p}}$ forces $q = p^*$.

Three regimes determine the nature of the embedding:

1. **Subcritical case: $kp < n$.** Embedding into $L^{p^*}$: continuous but not compact.
2. **Critical case: $kp = n$.** Embedding into $L^q$ for all finite $q$, or into BMO (bounded mean oscillation).
3. **Supercritical case: $kp > n$.** Embedding into Hölder spaces $C^{k - n/p}$: pointwise continuity.

## The Sobolev Embedding Theorem

**Theorem (Gagliardo-Nirenberg-Sobolev inequality).** Let $1 \leq p < n$ and $p^* = np/(n-p)$. For every $u \in W^{1,p}(\mathbb{R}^n)$:

$$\|u\|_{L^{p^*}(\mathbb{R}^n)} \leq C_{n,p} \|\nabla u\|_{L^p(\mathbb{R}^n)}.$$

**Proof for $p = 1$.** For $u \in C_c^\infty(\mathbb{R}^n)$, write $u(x) = \int_{-\infty}^{x_i} \partial_i u(x_1, \ldots, t, \ldots, x_n) \, dt$ for each $i$, giving $|u(x)| \leq \int_{-\infty}^\infty |\partial_i u| \, d_{i}$. Therefore:

$$|u(x)|^{n/(n-1)} \leq \prod_{i=1}^n \left(\int_{-\infty}^\infty |\partial_i u| \, dx_i\right)^{1/(n-1)}.$$

Integrate over all variables and apply the generalized Hölder inequality ($n-1$ times), using the AM-GM structure. The result (after density of $C_c^\infty$ in $W^{1,1}$) gives $\|u\|_{L^{n/(n-1)}} \leq \|\nabla u\|_{L^1}$, which is the $p = 1$, $p^* = n/(n-1)$ case. The general case $p > 1$ follows by applying the $p = 1$ case to $|u|^\alpha$ for appropriate $\alpha$ and using Hölder's inequality. $\square$

**General statement.** Let $\Omega \subset \mathbb{R}^n$ be a bounded open set with $C^1$ boundary (or $\Omega = \mathbb{R}^n$). Let $1 \leq p < \infty$ and $k \geq 1$. Then:

$$W^{k,p}(\Omega) \hookrightarrow L^q(\Omega) \text{ for } \frac{1}{q} = \frac{1}{p} - \frac{k}{n} \text{ (when } kp < n\text{)}.$$

$$W^{k,p}(\Omega) \hookrightarrow L^q(\Omega) \text{ for all } 1 \leq q < \infty \text{ (when } kp = n\text{)}.$$

$$W^{k,p}(\Omega) \hookrightarrow C^{m,\alpha}(\bar\Omega) \text{ for } m + \alpha = k - n/p, \text{ (when } kp > n\text{)}.$$

Here $C^{m,\alpha}$ is the Hölder space of $m$-times differentiable functions with $\alpha$-Hölder derivatives.

## The Morrey Embedding (Supercritical Case)

**Theorem (Morrey).** If $kp > n$, then $W^{k,p}(\mathbb{R}^n) \hookrightarrow C^{0,\alpha}(\mathbb{R}^n)$ where $\alpha = k - n/p$ (assuming $\alpha \leq 1$; for integer values, use $W^{k,p} \hookrightarrow C^{\lfloor\alpha\rfloor, \alpha - \lfloor\alpha\rfloor}$).

**Important special case.** For $n = 1$: $W^{1,p}(\mathbb{R}) \hookrightarrow C^0(\mathbb{R})$ for all $p \geq 1$. In one dimension, $H^1$ functions are automatically continuous.

For $n = 2$: $W^{1,p}$ with $p > 2$ embeds into $C^{0,1-2/p}$. Functions in $H^1 = W^{1,2}$ are borderline and NOT necessarily continuous (log singularities are in $H^1$ in 2D).

For $n = 3$: $H^2 = W^{2,2}$ with $2 \cdot 2 = 4 > 3 = n$, so $H^2(\Omega) \hookrightarrow C^0(\bar\Omega)$. In contrast, $H^1$ functions in 3D are not necessarily continuous.

## The Rellich-Kondrachov Theorem (Compact Embeddings)

The Sobolev embeddings above are continuous but generally not compact. Compact embeddings hold for strictly subcritical exponents on bounded domains:

**Theorem (Rellich-Kondrachov).** Let $\Omega \subset \mathbb{R}^n$ be a bounded open set with $C^1$ boundary, and $1 \leq p < \infty$. If $q < p^* = np/(n-p)$ (subcritical), then the embedding

$$W^{1,p}(\Omega) \hookrightarrow L^q(\Omega)$$

is **compact**: every bounded sequence in $W^{1,p}$ has a convergent subsequence in $L^q$.

In particular, the embedding $H^1(\Omega) \hookrightarrow L^2(\Omega)$ is compact for bounded $\Omega$ (since $2 < p^* = 2n/(n-2)$ for $n \geq 3$).

**Proof idea.** Boundedness in $W^{1,p}$ implies equicontinuity (after mollification) and uniform boundedness of mollified approximations. Arzelà-Ascoli gives compactness in $L^q$ for finite $q < p^*$.

The Rellich-Kondrachov theorem is used in:
- Proving the spectral theorem for the Laplacian on bounded domains (the inverse $(-\Delta)^{-1}: L^2 \to L^2$ is compact).
- Existence proofs that extract convergent subsequences from bounded sequences of approximate solutions.
- Mountain pass and variational methods in nonlinear PDE.

## Applications

**Existence and regularity for the Poisson equation.** By Lax-Milgram, for $f \in L^2(\Omega)$ there exists $u \in H^1_0(\Omega)$ with $-\Delta u = f$ weakly. By elliptic regularity, $u \in H^2(\Omega)$. By Morrey (for $n \leq 3$, $2 \cdot 2 > n$), $u \in C^0(\bar\Omega)$.

**Interpolation inequalities.** The Gagliardo-Nirenberg interpolation inequality generalizes: $\|u\|_{W^{j,p}} \leq C\|u\|_{W^{k,q}}^\theta \|u\|_{L^r}^{1-\theta}$ for appropriate $\theta$, $j$, $k$, $p$, $q$, $r$. These interpolation estimates are used in nonlinear PDE theory to control intermediate norms.
