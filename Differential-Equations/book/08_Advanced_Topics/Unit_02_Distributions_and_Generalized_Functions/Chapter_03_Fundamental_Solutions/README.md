# Chapter 3: Fundamental Solutions

A fundamental solution of a linear differential operator $P(D)$ is a distribution $E$ satisfying $P(D)E = \delta$. It is the response of the system to a point source. Once a fundamental solution is known, the solution to $P(D)u = f$ for any (suitably regular) $f$ is obtained by convolution: $u = E * f$. This is the rigorous generalization of the Green's function method to the distributional setting.

## Why Fundamental Solutions

The Dirac delta acts as the identity for convolution: $f * \delta = f$. If $P(D)E = \delta$ and $P(D)u = f$, then formally:

$$P(D)(E * f) = (P(D)E) * f = \delta * f = f.$$

So $u = E * f$ solves $P(D)u = f$. The distributional framework makes this argument rigorous.

The approach is extremely powerful: once $E$ is found (once and for all, for a given operator $P(D)$), the problem of solving $P(D)u = f$ is reduced to a convolution—an operation that can be computed explicitly in many cases, or estimated effectively in others.

## The Malgrange-Ehrenpreis Theorem

A foundational existence theorem ensures that fundamental solutions always exist for constant-coefficient operators:

**Theorem (Malgrange-Ehrenpreis, 1955).** Every nonzero linear partial differential operator with constant coefficients $P(D) = \sum_{|\alpha| \leq m} a_\alpha D^\alpha$ (with $a_\alpha \in \mathbb{R}$) has a fundamental solution $E \in \mathcal{D}'(\mathbb{R}^n)$.

The proof constructs $E$ using the Fourier transform: $\widehat{P(D)E} = P(2\pi i\xi)\hat{E} = \hat\delta = 1$, so $\hat{E} = 1/P(2\pi i\xi)$. The technical difficulty is that $P(2\pi i\xi)$ may vanish, so $1/P$ is not a smooth function and its inverse Fourier transform requires careful distribution theory.

## Chapter Structure

**Section 1: Fundamental Solutions of PDEs** develops the theory for the classical operators: the Laplacian, the heat operator, and the wave operator. Each fundamental solution has a specific form reflecting the geometry and physics of the operator.

**Section 2: Green's Functions via Distributions** shows how the distributional fundamental solution specializes, in the presence of boundary conditions, to the Green's function of boundary value problems. The distributional approach unifies the abstract existence theory (Malgrange-Ehrenpreis) with the concrete Green's function construction from classical ODE/PDE theory.

The fundamental solutions developed in this chapter are the building blocks for elliptic regularity theory, heat kernel estimates, and the wave equation's Huygens principle—all central themes of modern PDE analysis.
