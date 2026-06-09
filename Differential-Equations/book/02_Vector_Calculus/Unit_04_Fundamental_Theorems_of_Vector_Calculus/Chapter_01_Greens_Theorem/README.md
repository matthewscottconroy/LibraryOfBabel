# Chapter 1: Green's Theorem

Green's Theorem is the two-dimensional bridge between a double integral over a planar region and a line integral around its boundary. It is named after the British mathematician George Green (1793–1841), who stated and used it in his 1828 essay on the mathematical theory of electricity and magnetism. While not the most general of the fundamental theorems, Green's Theorem is the most computationally immediate and serves as the logical gateway to Stokes' Theorem and the Divergence Theorem.

## Statement

**Green's Theorem.** Let $D$ be a bounded, simply connected open region in $\mathbb{R}^2$ whose boundary $\partial D$ is a simple closed curve that is piecewise smooth. Let $P$ and $Q$ be functions that are $C^1$ on an open set containing the closure $\overline{D}$. Then

$$\oint_{\partial D} P\,dx + Q\,dy = \iint_D \left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right)dA,$$

where $\partial D$ is traversed counterclockwise (the positive orientation: the interior is on the left as you walk along the boundary).

## Chapter Overview

**Section 1: Statement and Proof** gives a careful proof for rectangular regions and then for general regions that can be decomposed into simple pieces. The key idea is that the boundary contributions of interior interfaces cancel, leaving only the exterior boundary. The proof is a clean application of the Fundamental Theorem of Calculus applied twice.

**Section 2: Area via Line Integrals** is one of the most striking applications: the area of a region $D$ can be computed as a line integral around its boundary. From Green's Theorem with $Q = x$ and $P = 0$: $\iint_D 1\,dA = \oint_{\partial D} x\,dy$. Similarly with $P = -y$, $Q = 0$: $\iint_D 1\,dA = -\oint_{\partial D} y\,dx$. Combining: $A(D) = \frac{1}{2}\oint_{\partial D} (x\,dy - y\,dx)$. This formula underlies the shoelace formula for polygonal area.

**Section 3: Normal Form and Divergence** presents the "flux form" of Green's Theorem, which equates the outward normal flux of a field around a closed curve to the divergence of the field over the enclosed region. This is the two-dimensional Divergence Theorem: $\oint_{\partial D}\mathbf{F}\cdot\hat{\mathbf{n}}\,ds = \iint_D\nabla\cdot\mathbf{F}\,dA$. It makes the connection to the three-dimensional Divergence Theorem explicit.

## Why Green's Theorem Works

The proof reduces to the observation that $\int_c^d\frac{\partial Q}{\partial x}\,dx = Q(d, y) - Q(c, y)$, which is the one-variable FTC applied to each horizontal slice. Summing over all slices and exchanging the order of integration gives the boundary integral. The same argument applies to the $P$ term with vertical slices. The two pieces combine to give the full theorem.

## Connection to Stokes' Theorem

Green's Theorem is a special case of Stokes' Theorem with the surface $S$ taken to be the flat region $D$ in the $xy$-plane. The curl of $\mathbf{F} = P\,\mathbf{i} + Q\,\mathbf{j}$ in the $z$-direction is $\partial Q/\partial x - \partial P/\partial y$, and $d\mathbf{S} = \mathbf{k}\,dA$ (the upward normal times area element). Stokes' Theorem then gives: $\oint_{\partial D}\mathbf{F}\cdot d\mathbf{r} = \iint_D(\nabla\times\mathbf{F})\cdot\mathbf{k}\,dA = \iint_D(\partial Q/\partial x - \partial P/\partial y)\,dA$, which is exactly Green's Theorem.
