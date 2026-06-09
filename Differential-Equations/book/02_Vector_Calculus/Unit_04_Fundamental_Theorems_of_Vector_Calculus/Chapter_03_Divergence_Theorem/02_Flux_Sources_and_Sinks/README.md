# Flux, Sources, and Sinks

The Divergence Theorem is not merely a computational device — it embodies a physical principle: the net outward flux of a vector field through a closed surface equals the total "source strength" of the field inside. Where the divergence is positive, the field is being generated; where it is negative, the field is being consumed. The theorem says these interior events are perfectly accounted for by what happens at the boundary.

## Sources and Sinks

A **source** of a vector field at a point $\mathbf{p}$ is a region where $\nabla\cdot\mathbf{F}(\mathbf{p}) > 0$: field lines are being created and spread outward. In fluid mechanics, a source is an inlet where fluid is injected; in electrostatics, a positive charge is a source of $\mathbf{E}$.

A **sink** is where $\nabla\cdot\mathbf{F}(\mathbf{p}) < 0$: field lines converge and are absorbed. A drain in fluid mechanics, or a negative charge in electrostatics.

A **divergence-free** (solenoidal) field has no sources or sinks: $\nabla\cdot\mathbf{F} = 0$ everywhere. Field lines neither begin nor end; they form closed loops or extend to infinity. The magnetic field $\mathbf{B}$ is solenoidal.

## The Balance Law

The Divergence Theorem $\oiint_{\partial V}\mathbf{F}\cdot d\mathbf{S} = \iiint_V\nabla\cdot\mathbf{F}\,dV$ is a **balance law**: the net outflow through the boundary equals the net production inside.

**Corollary.** If $\nabla\cdot\mathbf{F} = 0$ throughout $V$, then $\oiint_{\partial V}\mathbf{F}\cdot d\mathbf{S} = 0$: no net flux leaves or enters. The field is "conserved" — as much flows in as flows out.

**Corollary.** If $\mathbf{F}$ is the velocity field of an incompressible fluid ($\nabla\cdot\mathbf{v} = 0$), then the net volume flow rate through any closed surface is zero. Fluid is neither created nor destroyed.

## Using Divergence-Free Fields

When $\nabla\cdot\mathbf{F} = 0$ on the region between two closed surfaces $S_1$ (outer) and $S_2$ (inner), the Divergence Theorem on the region $V$ between them gives:

$$\oiint_{S_1}\mathbf{F}\cdot d\mathbf{S} = \oiint_{S_2}\mathbf{F}\cdot d\mathbf{S}$$

(both with outward-from-$V$ orientation, which means $S_2$ has the inward orientation relative to its own interior). This is the "flux is preserved" principle for solenoidal fields: the flux is the same through any closed surface surrounding the same set of sources.

**Example: Electric field of a point charge.** $\mathbf{E} = q\mathbf{r}/(4\pi\varepsilon_0 r^3)$ has $\nabla\cdot\mathbf{E} = 0$ for $r > 0$. The flux through any sphere of radius $r$ equals $q/\varepsilon_0$, regardless of $r$ — the flux "carries" outward without accumulation.

## Divergence as Source Density

The Divergence Theorem motivates viewing $\nabla\cdot\mathbf{F}(\mathbf{p})$ as the **source density** at $\mathbf{p}$: the flux produced per unit volume. Precisely:

$$\nabla\cdot\mathbf{F}(\mathbf{p}) = \lim_{V\to\{\mathbf{p}\}}\frac{1}{|V|}\oiint_{\partial V}\mathbf{F}\cdot d\mathbf{S}.$$

This limit definition (divergence as flux per unit volume) is coordinate-independent and gives physical intuition directly from the Divergence Theorem.

## Worked Example

Determine the net outward flux of $\mathbf{F} = (x^2, y^2, z^2)$ through the surface of the ellipsoid $x^2/a^2 + y^2/b^2 + z^2/c^2 \leq 1$.

$\nabla\cdot\mathbf{F} = 2x + 2y + 2z$.

By Divergence Theorem: $\oiint_{\partial V}\mathbf{F}\cdot d\mathbf{S} = \iiint_V 2(x+y+z)\,dV$.

By symmetry: $\iiint_V x\,dV = \iiint_V y\,dV = \iiint_V z\,dV = 0$ (the ellipsoid is symmetric about each coordinate plane, so the positive and negative contributions cancel).

Therefore $\oiint_{\partial V}\mathbf{F}\cdot d\mathbf{S} = 0$.

## Summary

The Divergence Theorem makes precise the physical intuition that sources and sinks inside a region drive the flux through its boundary. Solenoidal fields ($\nabla\cdot\mathbf{F}=0$) carry their flux without creation or destruction. The divergence at a point is the flux per unit volume — a coordinate-free characterization of local source strength. These concepts are the mathematical foundation of conservation laws in fluid mechanics, electromagnetism, and heat conduction.
