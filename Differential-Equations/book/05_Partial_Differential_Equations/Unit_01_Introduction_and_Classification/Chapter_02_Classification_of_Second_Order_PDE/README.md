# Chapter 2: Classification of Second-Order PDEs

The classification of second-order linear PDEs is one of the most important structural results in the entire subject. It asserts that any second-order linear PDE in two independent variables can be reduced, by a smooth change of coordinates, to one of three canonical forms — the wave equation, the heat equation, or Laplace's equation — and that this reduction is determined entirely by the discriminant of the principal part of the equation. The classification is not merely a mathematical curiosity: the three types have fundamentally different physical interpretations, require different auxiliary conditions for well-posedness, and demand different analytical and numerical methods.

## The General Second-Order Linear PDE

The general second-order linear PDE in two independent variables $x$ and $y$ is

$$Au_{xx} + Bu_{xy} + Cu_{yy} + Du_x + Eu_y + Fu = G,$$

where $A, B, C, D, E, F, G$ are given functions of $(x,y)$. The **principal part** of the equation consists of the second-order terms $Au_{xx} + Bu_{xy} + Cu_{yy}$, and it is this part that determines the character of the equation.

The classification is based on the discriminant:

$$\Delta = B^2 - 4AC.$$

- If $\Delta > 0$: the equation is **hyperbolic** at that point.
- If $\Delta = 0$: the equation is **parabolic** at that point.
- If $\Delta < 0$: the equation is **elliptic** at that point.

An equation can change type from point to point if $A, B, C$ depend on $(x,y)$. The Tricomi equation $y u_{xx} + u_{yy} = 0$ is elliptic for $y > 0$ and hyperbolic for $y < 0$.

## The Three Chapters

This chapter develops the classification theory in three sections.

**Chapter 2.1: Canonical Forms** explains how to reduce a second-order PDE to canonical form via a change of independent variables $\xi = \xi(x,y)$, $\eta = \eta(x,y)$. The transformation changes the coefficients of the second-order terms according to a specific algebraic rule, and the canonical form is achieved by choosing $\xi$ and $\eta$ as solutions of the characteristic equations. The canonical forms are: the wave equation $u_{\xi\eta} = \ldots$ or $u_{\xi\xi} - u_{\eta\eta} = \ldots$ (hyperbolic), the heat equation $u_{\xi\xi} = \ldots$ (parabolic), and Laplace's equation $u_{\xi\xi} + u_{\eta\eta} = \ldots$ (elliptic).

**Chapter 2.2: Elliptic, Parabolic, and Hyperbolic Equations** explores the physical interpretation and key properties of each type. Elliptic equations describe equilibrium states; their solutions are smooth and satisfy maximum principles. Parabolic equations describe diffusion processes evolving in time; they smooth data and have an arrow of time. Hyperbolic equations describe wave propagation; they preserve singularities and have finite propagation speed. This chapter makes the physical reasoning precise and connects it to the mathematics.

**Chapter 2.3: Characteristics** develops the theory of characteristic curves in detail. Characteristics are the curves along which the Cauchy problem is not uniquely determined — they are the "lines of singularity propagation." For hyperbolic equations, there are two families of real characteristics; for parabolic equations, one family; for elliptic equations, none (the characteristics are complex). The method of characteristics, developed at length in Unit 2, is the direct generalization of this classification theory to the problem of constructing solutions.

## Why This Classification Is Deep

The classification theorem reveals that the three canonical PDEs — wave, heat, Laplace — are not special examples but representatives of fundamentally different mathematical categories. Every second-order linear PDE belongs to one of these categories (at least locally), inherits its qualitative behavior from the canonical representative, and should be treated with the methods appropriate to that category.

This is analogous to the classification of conic sections in analytic geometry (hyperbola, parabola, ellipse), and the analogy is not accidental: the discriminant condition is exactly the same algebraic criterion, applied to the associated quadratic form of the principal symbol.
