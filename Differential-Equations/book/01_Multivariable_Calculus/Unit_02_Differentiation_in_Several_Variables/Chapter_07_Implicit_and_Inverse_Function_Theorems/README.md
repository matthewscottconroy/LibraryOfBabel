# Chapter 07 Implicit and Inverse Function Theorems

Every map $f: \mathbb{R}^n \to \mathbb{R}^m$ raises two fundamental questions: can an equation $f(\mathbf{x}) = \mathbf{c}$ be solved for some of the variables in terms of the others, and can the map $f$ itself be locally inverted? The **implicit function theorem** answers the first question; the **inverse function theorem** answers the second. Both are consequences of the same underlying principle — the contraction mapping theorem — and both are local results: they guarantee smooth solutions or inverses in a neighborhood of a point, under the hypothesis that a certain Jacobian is nonsingular at that point.

## What This Chapter Covers

**Section 1 (Implicit Function Theorem)** considers a system $F(x_1,\ldots,x_k, y_1,\ldots,y_m) = \mathbf{0}$ of $m$ equations in $k+m$ unknowns. If the $m\times m$ Jacobian of $F$ with respect to the $y$-variables is nonsingular at a solution $(x_0, y_0)$, then the equation implicitly defines $y$ as a smooth function of $x$ near $(x_0, y_0)$: there is a smooth map $\mathbf{y} = \boldsymbol{\phi}(\mathbf{x})$ with $\boldsymbol{\phi}(\mathbf{x}_0) = \mathbf{y}_0$ and $F(\mathbf{x}, \boldsymbol{\phi}(\mathbf{x})) = \mathbf{0}$. The theorem also provides a formula for the derivative of $\boldsymbol{\phi}$ in terms of the Jacobians of $F$.

**Section 2 (Inverse Function Theorem)** considers a map $f: \mathbb{R}^n \to \mathbb{R}^n$ (same domain and codomain dimension). If the Jacobian $J_f(\mathbf{a})$ is nonsingular at a point $\mathbf{a}$, then $f$ is locally invertible near $\mathbf{a}$: there are open neighborhoods $U$ of $\mathbf{a}$ and $V$ of $f(\mathbf{a})$ such that $f|_U: U\to V$ is a bijection with a smooth inverse $f^{-1}: V\to U$, and $J_{f^{-1}}(f(\mathbf{a})) = [J_f(\mathbf{a})]^{-1}$.

**Section 3 (Introduction to Manifolds)** uses the implicit function theorem to define **smooth manifolds**: subsets of $\mathbb{R}^n$ that locally look like $\mathbb{R}^k$ for some $k$. The level set $\{F = \mathbf{0}\}$ of a smooth map $F:\mathbb{R}^n\to\mathbb{R}^m$ is a smooth $(n-m)$-dimensional manifold near any regular point of $F$. This section introduces the language of manifolds that becomes essential in advanced differential geometry and the global theory of differential equations.

## How the Sections Build on Each Other

The inverse function theorem is a special case of the implicit function theorem (apply the implicit function theorem to $F(\mathbf{x},\mathbf{y}) = f(\mathbf{x}) - \mathbf{y}$ with $m=n$). The manifold definition in Section 3 is an application of the implicit function theorem to describe what constraint sets look like locally. The three sections thus have a logical progression: from implicit solutions to local inverses to the global structure of level sets.

## How This Chapter Fits into the Unit

The implicit and inverse function theorems are among the deepest and most powerful results in multivariable calculus. They appear repeatedly in higher mathematics: in differential geometry (defining smooth manifolds and tangent spaces), in differential topology (degree theory, Sard's theorem), and in the theory of differential equations (existence and uniqueness of solutions to systems of ODEs, which follows from the contraction mapping theorem that also proves these two theorems). The Lagrange multiplier theorem of Chapter 6 is rigorously justified by the implicit function theorem. The change of variables formula for integration (Unit 3) requires the inverse function theorem to guarantee the validity of the substitution.
