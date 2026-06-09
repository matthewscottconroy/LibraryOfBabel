# Applications of the Divergence Theorem in Physics

The Divergence Theorem is the key that converts between the integral and differential forms of the fundamental laws of physics. Every conservation law in classical physics — mass, charge, energy, momentum — has an integral form (what happens across a boundary) and a differential form (what happens locally). The Divergence Theorem connects them. This section works through the most important applications.

## Gauss's Law (Integral to Differential Form)

The integral form of Gauss's law states that the total electric flux through any closed surface $S$ equals the enclosed charge divided by $\varepsilon_0$:

$$\oiint_S\mathbf{E}\cdot d\mathbf{S} = \frac{Q_{\text{enc}}}{\varepsilon_0} = \frac{1}{\varepsilon_0}\iiint_V\rho\,dV,$$

where $\rho$ is the charge density. By the Divergence Theorem, the left side equals $\iiint_V\nabla\cdot\mathbf{E}\,dV$. Since $V$ is arbitrary:

$$\nabla\cdot\mathbf{E} = \frac{\rho}{\varepsilon_0}.$$

This is the **differential form of Gauss's law** (one of Maxwell's equations). It says: the electric field diverges from points of positive charge density, converges toward negative charge density, and is divergence-free in charge-free regions.

Similarly, for gravity: $\nabla\cdot\mathbf{g} = -4\pi G\rho_m$, where $\mathbf{g}$ is the gravitational field and $\rho_m$ is mass density. Mass is a source of gravitational flux (with the opposite sign convention — gravity is attractive).

## Conservation of Mass (Continuity Equation)

Let $\rho$ be fluid density and $\mathbf{v}$ the velocity field. The mass of fluid in a region $V$ at time $t$ is $M(t) = \iiint_V\rho\,dV$. The rate of change is:

$$\frac{dM}{dt} = \iiint_V\frac{\partial\rho}{\partial t}\,dV.$$

Conservation of mass requires: the rate of mass increase inside $V$ equals the rate of mass flowing in through $\partial V$:

$$\frac{dM}{dt} = -\oiint_{\partial V}\rho\mathbf{v}\cdot d\mathbf{S} = -\iiint_V\nabla\cdot(\rho\mathbf{v})\,dV.$$

(The minus sign: outward flux removes mass from $V$.) Since $V$ is arbitrary:

$$\frac{\partial\rho}{\partial t} + \nabla\cdot(\rho\mathbf{v}) = 0.$$

This is the **continuity equation** — the differential form of mass conservation. For incompressible fluid ($\rho = $ const): $\nabla\cdot\mathbf{v} = 0$.

## Heat Flow and Laplace's Equation

Fourier's law: heat flux is $\mathbf{q} = -k\nabla T$ (heat flows from hot to cold). The heat energy per unit volume is $\rho c_p T$. Conservation of energy:

$$\rho c_p\frac{\partial T}{\partial t} = -\nabla\cdot\mathbf{q} + f = k\nabla^2 T + f,$$

where $f$ is internal heat generation per unit volume. This is the **heat equation**.

In steady state ($\partial T/\partial t = 0$) with no sources ($f = 0$): $\nabla^2 T = 0$ — Laplace's equation. The steady-state temperature is harmonic.

**Application of the Divergence Theorem:** The total heat flowing out of $V$ per unit time is $\oiint_{\partial V}\mathbf{q}\cdot d\mathbf{S} = -k\oiint_{\partial V}\nabla T\cdot d\mathbf{S} = -k\iiint_V\nabla^2 T\,dV$. In steady state, this is zero — no net heat leaves a source-free region (as much flows in as out).

## Green's Identities

Applying the Divergence Theorem to $\mathbf{F} = f\nabla g$ gives:

$$\oiint_{\partial V}f\nabla g\cdot d\mathbf{S} = \iiint_V\nabla\cdot(f\nabla g)\,dV = \iiint_V(f\nabla^2 g + \nabla f\cdot\nabla g)\,dV.$$

Rearranging: **Green's First Identity:**

$$\iiint_V f\nabla^2 g\,dV = \oiint_{\partial V} f\frac{\partial g}{\partial n}\,dS - \iiint_V\nabla f\cdot\nabla g\,dV,$$

where $\partial g/\partial n = \nabla g\cdot\hat{\mathbf{n}}$ is the outward normal derivative.

Subtracting the same with $f$ and $g$ interchanged: **Green's Second Identity:**

$$\iiint_V(f\nabla^2 g - g\nabla^2 f)\,dV = \oiint_{\partial V}\left(f\frac{\partial g}{\partial n} - g\frac{\partial f}{\partial n}\right)dS.$$

**Application to harmonic functions.** If $\nabla^2 g = 0$ in $V$ and $g = 0$ on $\partial V$, then setting $f = g$ in Green's first identity:

$$0 = -\iiint_V|\nabla g|^2\,dV \implies \nabla g = \mathbf{0} \implies g = \text{const} = 0.$$

This is the **uniqueness theorem** for Laplace's equation with Dirichlet boundary conditions: the only harmonic function that vanishes on the boundary vanishes throughout the domain.

## The Archimedes Principle

The buoyant force on a submerged object $V$ with boundary $\partial V$ is:

$$\mathbf{F}_{\text{buoy}} = -\oiint_{\partial V} p\,\hat{\mathbf{n}}\,dS,$$

where $p = \rho_{\text{fluid}} g z$ is the hydrostatic pressure (taking $z$ increasing upward). By the Divergence Theorem:

$$F_z = -\oiint_{\partial V} p\,\hat{\mathbf{n}}\cdot\mathbf{k}\,dS = -\iiint_V\frac{\partial p}{\partial z}\,dV = -\iiint_V\rho_{\text{fluid}} g\,dV = -\rho_{\text{fluid}} g\,\text{Vol}(V).$$

The upward buoyant force equals the weight of displaced fluid — Archimedes' principle, derived from the Divergence Theorem.

## Summary

The Divergence Theorem converts every integral conservation law in physics into its differential (pointwise) form. Gauss's law gives $\nabla\cdot\mathbf{E} = \rho/\varepsilon_0$; mass conservation gives $\partial_t\rho + \nabla\cdot(\rho\mathbf{v}) = 0$; heat conservation gives the heat equation; boundary conditions for harmonic functions give uniqueness theorems. Archimedes' principle is a direct consequence. The theorem is not an abstract mathematical result — it is the language in which the physical world conserves its quantities.
