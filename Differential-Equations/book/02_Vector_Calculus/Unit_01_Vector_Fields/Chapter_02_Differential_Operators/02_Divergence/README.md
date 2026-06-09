# Divergence

Imagine a region of space filled with a flowing fluid. At some points, the fluid spreads outward in all directions — a spring, or a source. At others, the fluid converges from all directions toward a point — a drain, or a sink. At still others, fluid flows through without net accumulation or depletion. The divergence is the mathematical operator that measures exactly this behavior: it quantifies the net rate of outward flow of a vector field from an infinitesimal neighborhood of a point.

## Definition

Let $\mathbf{F}: D \subseteq \mathbb{R}^3 \to \mathbb{R}^3$ be a $C^1$ vector field with components $\mathbf{F} = P\,\mathbf{i} + Q\,\mathbf{j} + R\,\mathbf{k}$. The **divergence** of $\mathbf{F}$ is the scalar field

$$\nabla \cdot \mathbf{F} = \frac{\partial P}{\partial x} + \frac{\partial Q}{\partial y} + \frac{\partial R}{\partial z}.$$

The notation $\nabla \cdot \mathbf{F}$ emphasizes the formal dot product of the del operator $\nabla = (\partial_x, \partial_y, \partial_z)$ with the vector $\mathbf{F} = (P, Q, R)$.

In two dimensions, $\nabla \cdot \mathbf{F} = \partial P/\partial x + \partial Q/\partial y$.

## Geometric Interpretation

The divergence at a point $\mathbf{p}$ measures the infinitesimal net outward flux of $\mathbf{F}$ per unit volume. More precisely, let $B_\varepsilon(\mathbf{p})$ be a ball of radius $\varepsilon$ centered at $\mathbf{p}$, with surface $S_\varepsilon$ and volume $V_\varepsilon = \frac{4}{3}\pi\varepsilon^3$. Then

$$(\nabla \cdot \mathbf{F})(\mathbf{p}) = \lim_{\varepsilon \to 0} \frac{1}{V_\varepsilon}\iint_{S_\varepsilon} \mathbf{F} \cdot d\mathbf{S}.$$

This is not just a mnemonic — it is a theorem (a consequence of the Divergence Theorem, proved in Unit 4) — and it is the right way to think about divergence. The flux integral $\iint_{S_\varepsilon} \mathbf{F} \cdot d\mathbf{S}$ measures the total outward flow through $S_\varepsilon$; dividing by $V_\varepsilon$ and shrinking the ball to a point gives the flux density.

**Positive divergence:** outflow exceeds inflow; the point is a source.
**Negative divergence:** inflow exceeds outflow; the point is a sink.
**Zero divergence:** inflow equals outflow; the field is source-free at that point.

## Solenoidal Fields

A vector field with zero divergence everywhere is called **solenoidal** (or **divergence-free** or **incompressible**). The term comes from electrodynamics: a magnetic field $\mathbf{B}$ satisfies $\nabla \cdot \mathbf{B} = 0$ everywhere, which is one of Maxwell's equations and reflects the empirical fact that magnetic monopoles do not exist.

For fluid mechanics, an incompressible fluid has velocity field satisfying $\nabla \cdot \mathbf{v} = 0$: the fluid neither accumulates nor depletes at any point, so density remains constant following the flow.

**Theorem.** If $\mathbf{F} = \nabla \times \mathbf{G}$ for some $C^2$ vector field $\mathbf{G}$, then $\nabla \cdot \mathbf{F} = 0$. (The divergence of a curl is always zero.)

This theorem — proved in detail in Section 5 — provides a way to construct solenoidal fields and underlies the concept of a vector potential in electrodynamics: since $\nabla \cdot \mathbf{B} = 0$, we can write $\mathbf{B} = \nabla \times \mathbf{A}$ for some vector potential $\mathbf{A}$.

## Worked Examples

**Example 1.** Let $\mathbf{F}(x,y,z) = x^2\,\mathbf{i} + y^2\,\mathbf{j} + z^2\,\mathbf{k}$.

$$\nabla \cdot \mathbf{F} = \frac{\partial(x^2)}{\partial x} + \frac{\partial(y^2)}{\partial y} + \frac{\partial(z^2)}{\partial z} = 2x + 2y + 2z.$$

The divergence is positive when $x+y+z > 0$, meaning the field acts as a source in that half-space, and as a sink in the complementary half-space.

**Example 2: Radial field.** Let $\mathbf{F} = \mathbf{r} = x\,\mathbf{i} + y\,\mathbf{j} + z\,\mathbf{k}$.

$$\nabla \cdot \mathbf{F} = 1 + 1 + 1 = 3.$$

The divergence is constant and positive — the radial field has uniform outward expansion at every point. Geometrically, the outward-pointing radial arrows expand more and more as you move away from the origin.

**Example 3: Inverse-square field.** Let $\mathbf{F} = \mathbf{r}/|\mathbf{r}|^3$ (Coulomb/gravitational field, defined for $\mathbf{r} \neq \mathbf{0}$). Computing:

$$\nabla \cdot \left(\frac{\mathbf{r}}{|\mathbf{r}|^3}\right) = 0 \quad \text{for } \mathbf{r} \neq \mathbf{0}.$$

This is a remarkable fact: away from the origin, the inverse-square field has zero divergence despite its dramatic variation. This reflects the fact that, in a fluid interpretation, as much fluid flows in through the inner part of a shell as flows out through the outer part — the flux law perfectly balances. At the origin itself, the "divergence" is a Dirac delta function (a distribution), reflecting a point source of total strength $4\pi$.

**Verification of Example 3.** Let $P = x/(x^2+y^2+z^2)^{3/2}$.

$$\frac{\partial P}{\partial x} = \frac{(x^2+y^2+z^2)^{3/2} - x \cdot \frac{3}{2}(x^2+y^2+z^2)^{1/2}\cdot 2x}{(x^2+y^2+z^2)^3} = \frac{r^2 - 3x^2}{r^5}.$$

By symmetry, $\partial Q/\partial y = (r^2 - 3y^2)/r^5$ and $\partial R/\partial z = (r^2 - 3z^2)/r^5$. Summing:

$$\nabla \cdot \mathbf{F} = \frac{3r^2 - 3(x^2+y^2+z^2)}{r^5} = \frac{3r^2 - 3r^2}{r^5} = 0.$$

## Divergence in Curvilinear Coordinates

In cylindrical coordinates $(r, \theta, z)$:

$$\nabla \cdot \mathbf{F} = \frac{1}{r}\frac{\partial(rF_r)}{\partial r} + \frac{1}{r}\frac{\partial F_\theta}{\partial \theta} + \frac{\partial F_z}{\partial z}.$$

In spherical coordinates $(\rho, \theta, \phi)$:

$$\nabla \cdot \mathbf{F} = \frac{1}{\rho^2}\frac{\partial(\rho^2 F_\rho)}{\partial \rho} + \frac{1}{\rho\sin\theta}\frac{\partial(\sin\theta\, F_\theta)}{\partial \theta} + \frac{1}{\rho\sin\theta}\frac{\partial F_\phi}{\partial \phi}.$$

These formulas account for the varying volume elements in curvilinear systems. The extra scale factors ensure that the limit formula (flux per unit volume) continues to hold.

## Physical Applications

**Continuity equation.** If $\rho$ is fluid density and $\mathbf{v}$ is velocity, the mass flux is $\mathbf{J} = \rho\mathbf{v}$. The law of mass conservation (continuity equation) states:

$$\frac{\partial \rho}{\partial t} + \nabla \cdot (\rho \mathbf{v}) = 0.$$

For an incompressible fluid with constant density, $\nabla \cdot \mathbf{v} = 0$.

**Gauss's Law (differential form).** In electrostatics, the electric field $\mathbf{E}$ satisfies $\nabla \cdot \mathbf{E} = \rho/\varepsilon_0$, where $\rho$ is the electric charge density. Charge is the source of the electric field, and the divergence of $\mathbf{E}$ measures how much charge is present per unit volume.

## Summary

The divergence of a vector field measures the net outward flux per unit volume at each point. Positive divergence indicates a source; negative indicates a sink; zero divergence (solenoidal) indicates conservation. The precise relationship between divergence and total flux over a closed surface is the content of the Divergence Theorem, which will be proved in Unit 4. Understanding divergence is essential for any application involving conservation laws.
