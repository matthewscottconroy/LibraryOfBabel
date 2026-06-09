# Chapter 3: The Fundamental Theorem for Line Integrals

The Fundamental Theorem of Calculus says that $\int_a^b f'(x)\,dx = f(b) - f(a)$: the integral of a derivative over an interval is determined entirely by the values of the original function at the boundary of that interval. The Fundamental Theorem for Line Integrals is the multivariable analogue: the line integral of a gradient over a curve is determined entirely by the values of the potential function at the endpoints of that curve. This theorem transforms the computation of line integrals for conservative fields from a parametric calculation into a simple evaluation, and it gives the precise mathematical meaning to "path independence."

## Chapter Overview

**Section 1: Path Independence** develops the concept formally. A vector field $\mathbf{F}$ has path-independent line integrals if the value of $\int_C\mathbf{F}\cdot d\mathbf{r}$ between any two points $A$ and $B$ is the same for every curve $C$ from $A$ to $B$ (within the domain). Path independence, conservativity (existence of a potential), and vanishing circulation are shown to be equivalent.

**Section 2: Conservative Fields and Potential Functions** revisits the material from Unit 1 in the light of what we now know about line integrals. The gradient theorem provides an explicit formula for the potential function via the line integral: $f(\mathbf{r}) = \int_{\mathbf{r}_0}^{\mathbf{r}} \mathbf{F}\cdot d\mathbf{r}$ (where the integral is path-independent, so any path can be used). This gives an alternative method for finding potential functions — integration along a convenient path rather than the step-by-step integration of components.

**Section 3: Simply Connected Regions** addresses the topological subtlety that determines when the curl test is sufficient for conservativity. On simply connected domains (those without "holes"), the conditions $\nabla\times\mathbf{F} = \mathbf{0}$, path independence, and conservativity are all equivalent. On domains with holes, a curl-free field need not be conservative, as the vortex field example showed.

## The Theorem

**Fundamental Theorem for Line Integrals.** Let $\mathbf{F} = \nabla f$ be a conservative vector field on an open connected domain $D \subseteq \mathbb{R}^n$, and let $C$ be a piecewise smooth curve in $D$ from point $A$ to point $B$. Then

$$\int_C \mathbf{F}\cdot d\mathbf{r} = f(B) - f(A).$$

The proof is a direct application of the one-variable chain rule: parametrize $C$ by $\mathbf{r}(t)$, write $\mathbf{F}(\mathbf{r}(t))\cdot\mathbf{r}'(t) = \nabla f(\mathbf{r}(t))\cdot\mathbf{r}'(t) = d/dt\,[f(\mathbf{r}(t))]$, and integrate both sides from $a$ to $b$.

## Significance

This theorem is one of the first and most important results in vector calculus. It shows that the line integral of a gradient is path-independent — indeed, it reduces to a boundary evaluation. This is the exact multivariable counterpart of the one-dimensional FTC, and it is the template for the other fundamental theorems: Green's Theorem, Stokes' Theorem, and the Divergence Theorem all say "integral of a 'derivative' over a domain = values on the boundary," with the Generalized Stokes' Theorem in Unit 4 unifying all of them.

## Topological Significance

The failure of the curl test to be sufficient on non-simply-connected domains is not a nuisance but a deep fact connecting analysis and topology. The vortex field $\mathbf{F} = (-y, x)/(x^2+y^2)$ has zero curl everywhere on $\mathbb{R}^2\setminus\{0\}$ but nonzero circulation around the origin. This is the starting point of de Rham cohomology: the obstruction to a closed form being exact is measured by the topology of the domain. These ideas, glimpsed here for the first time, become a major theme in differential geometry and topology.
