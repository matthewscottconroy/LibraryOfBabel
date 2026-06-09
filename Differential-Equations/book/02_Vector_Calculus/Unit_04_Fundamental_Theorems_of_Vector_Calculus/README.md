# Unit 4: Fundamental Theorems of Vector Calculus

The Fundamental Theorem of Calculus states that $\int_a^b f'(x)\,dx = f(b) - f(a)$: the integral of a derivative over an interval equals the boundary values of the original function. This unit develops the three great generalizations of this principle to higher dimensions, plus a fourth theorem that unifies them all.

## The Pattern

Each theorem in this unit has the same structure: the integral of a "derivative" over a region equals the integral of the original quantity over the boundary of that region. The derivatives are the curl and the divergence; the regions are planar domains, surfaces, and volumes; the boundaries are curves and surfaces. Arranging the theorems by dimension:

| Theorem | "Interior" | "Derivative" | "Boundary" |
|---|---|---|---|
| FTC for Line Integrals | Curve from $A$ to $B$ | Gradient | Boundary points $A$, $B$ |
| Green's Theorem | Planar region $D$ | Curl ($z$-component) | Boundary curve $\partial D$ |
| Stokes' Theorem | Surface $S$ | Curl $\nabla\times\mathbf{F}$ | Boundary curve $\partial S$ |
| Divergence Theorem | Volume $V$ | Divergence $\nabla\cdot\mathbf{F}$ | Boundary surface $\partial V$ |

All four are special cases of the **Generalized Stokes' Theorem** $\int_M d\omega = \int_{\partial M}\omega$, stated in the language of differential forms in Chapter 4.

## Unit Structure

**Chapter 1: Green's Theorem** relates a double integral over a planar region $D$ to a line integral around its boundary $\partial D$:

$$\oint_{\partial D} P\,dx + Q\,dy = \iint_D\left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right)dA.$$

Green's Theorem has multiple applications: computing area via line integrals, evaluating circulation integrals, and the "normal form" (involving divergence) connecting it to the Divergence Theorem in 2D.

**Chapter 2: Stokes' Theorem** generalizes Green's Theorem to surfaces in $\mathbb{R}^3$:

$$\oint_{\partial S}\mathbf{F}\cdot d\mathbf{r} = \iint_S(\nabla\times\mathbf{F})\cdot d\mathbf{S}.$$

Circulation around the boundary of a surface equals the flux of the curl through the surface. Stokes' Theorem is the key tool in electrodynamics (deriving the differential form of Faraday's law from its integral form) and in understanding why $\nabla\times(\nabla f) = \mathbf{0}$.

**Chapter 3: The Divergence Theorem** relates the total outward flux through a closed surface $\partial V$ to the integral of divergence over the enclosed volume $V$:

$$\oiint_{\partial V}\mathbf{F}\cdot d\mathbf{S} = \iiint_V\nabla\cdot\mathbf{F}\,dV.$$

This theorem is the key to deriving the differential forms of Gauss's law and the continuity equation from their integral forms. It also gives the precise relationship between the physical concept of "source strength" (divergence) and "net outflow" (flux).

**Chapter 4: Unification via Differential Forms** introduces the language of differential forms and the exterior derivative $d$. In this language, all four theorems collapse to the single statement $\int_M d\omega = \int_{\partial M}\omega$. This unification is not merely aesthetic — it reveals the deep structure connecting the three theorems, extends naturally to arbitrary dimensions, and is the starting point for modern differential geometry, topology, and mathematical physics.

## Why This Unit is the Climax of the Module

The fundamental theorems transform integrals: they convert a difficult integral over a region into a potentially easier integral over the boundary (or vice versa). This conversion is what makes them powerful:

- To compute the work done by a conservative field, use path independence (avoid integrating along the path).
- To compute circulation, use Stokes' Theorem (replace a line integral by a surface integral of the curl).
- To compute flux through a closed surface, use the Divergence Theorem (replace a surface integral by a volume integral of divergence).

Each direction of the conversion is useful in different circumstances, and the art of applying these theorems lies in recognizing which conversion simplifies the problem at hand.
