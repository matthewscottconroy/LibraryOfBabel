# Chapter 02: Linear Maps

A linear map (also called a linear transformation) is a function between vector spaces that respects their algebraic structure: it preserves addition and scalar multiplication. The study of linear maps is, in a deep sense, the central subject of linear algebra, because a finite-dimensional vector space is determined (up to isomorphism) by its dimension, and linear maps between such spaces are completely captured by matrices.

## What This Chapter Covers

**Section 1: Definition and Examples** introduces linear maps and establishes the basic examples. The definition requires $T(\alpha u + \beta v) = \alpha T(u) + \beta T(v)$ for all vectors $u, v$ and scalars $\alpha, \beta$. Key examples include the differentiation operator $D: C^1(I) \to C(I)$ defined by $D(f) = f'$, the integration operator $I: C(I) \to C(I)$, rotation and reflection matrices on $\mathbb{R}^2$, and general matrix-vector multiplication $T(x) = Ax$.

**Section 2: Kernel and Image** defines the two fundamental subspaces associated with a linear map $T: V \to W$: the kernel $\ker T = \{v \in V : T(v) = \mathbf{0}\}$ (the set of inputs mapped to zero) and the image $\text{im}\, T = \{T(v) : v \in V\}$ (the set of outputs). Both are subspaces — of $V$ and $W$ respectively. The kernel measures how much information is "lost" by $T$, while the image measures what part of $W$ is "reachable."

**Section 3: Rank-Nullity Theorem** proves the fundamental dimension relation: $\dim \ker T + \dim \text{im}\, T = \dim V$. This theorem has an enormous number of consequences: it determines when a system of equations has a solution, when a solution is unique, and what the dimension of the solution space of a homogeneous ODE is.

## Connection to Differential Equations

Every linear differential operator $L[y] = a_n y^{(n)} + \cdots + a_0 y$ is a linear map from $C^n(I)$ to $C(I)$. The solution space of the homogeneous equation $L[y] = 0$ is exactly $\ker L$. The Rank-Nullity Theorem tells us that $\dim \ker L = n$ (under appropriate conditions), which is why an $n$-th order linear ODE has an $n$-dimensional solution space. The non-homogeneous problem $L[y] = f$ has a solution iff $f \in \text{im}\, L$ (a solvability condition), and the general solution is any particular solution plus an element of $\ker L$.
