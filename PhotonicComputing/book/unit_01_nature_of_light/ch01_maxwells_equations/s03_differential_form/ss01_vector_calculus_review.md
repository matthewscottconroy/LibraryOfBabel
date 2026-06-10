# 1.3.1 Vector Calculus: Divergence, Curl, and the Key Theorems

## Why Vector Calculus?

The electromagnetic field assigns a vector (with magnitude and direction) to every point in space. To describe how that vector field changes from point to point — whether it is spreading out, rotating, or flowing — we need the operations of vector calculus: the divergence and the curl.

These are not arbitrary mathematical constructs. Each has a precise physical interpretation, and understanding those interpretations is essential for reading and using Maxwell's equations correctly.

## The Gradient

The **gradient** of a scalar function $f(x, y, z)$ is the vector:

$$\nabla f = \frac{\partial f}{\partial x}\hat{\mathbf{x}} + \frac{\partial f}{\partial y}\hat{\mathbf{y}} + \frac{\partial f}{\partial z}\hat{\mathbf{z}}$$

The gradient points in the direction of steepest ascent of $f$ and has magnitude equal to the rate of change in that direction.

*Physical example*: If $V(x,y,z)$ is the electric potential (voltage), then $\mathbf{E} = -\nabla V$ — the electric field points "downhill" in potential, toward lower voltage.

## The Divergence

The **divergence** of a vector field $\mathbf{F}(x,y,z) = F_x\hat{\mathbf{x}} + F_y\hat{\mathbf{y}} + F_z\hat{\mathbf{z}}$ is the scalar:

$$\nabla \cdot \mathbf{F} = \frac{\partial F_x}{\partial x} + \frac{\partial F_y}{\partial y} + \frac{\partial F_z}{\partial z}$$

**Physical interpretation**: The divergence at a point measures the rate at which $\mathbf{F}$ is spreading out from (or converging toward) that point — it is a measure of "source strength."

- If $\nabla \cdot \mathbf{F} > 0$ at a point: $\mathbf{F}$ is spreading out there (source).
- If $\nabla \cdot \mathbf{F} < 0$ at a point: $\mathbf{F}$ is converging there (sink).
- If $\nabla \cdot \mathbf{F} = 0$ everywhere: $\mathbf{F}$ has no sources or sinks; field lines neither begin nor end.

*Physical examples*:
- The electric field of a positive point charge has positive divergence at the location of the charge (it is a source).
- The magnetic field has zero divergence everywhere (no monopoles; field lines are closed loops).

## The Curl

The **curl** of a vector field $\mathbf{F}$ is the vector:

$$\nabla \times \mathbf{F} = \begin{vmatrix} \hat{\mathbf{x}} & \hat{\mathbf{y}} & \hat{\mathbf{z}} \\ \partial/\partial x & \partial/\partial y & \partial/\partial z \\ F_x & F_y & F_z \end{vmatrix}$$

Expanding this determinant:

$$\nabla \times \mathbf{F} = \left(\frac{\partial F_z}{\partial y} - \frac{\partial F_y}{\partial z}\right)\hat{\mathbf{x}} + \left(\frac{\partial F_x}{\partial z} - \frac{\partial F_z}{\partial x}\right)\hat{\mathbf{y}} + \left(\frac{\partial F_y}{\partial x} - \frac{\partial F_x}{\partial y}\right)\hat{\mathbf{z}}$$

**Physical interpretation**: The curl at a point measures the "rotation" or "circulation" of $\mathbf{F}$ around that point. The direction of the curl vector is the axis of rotation; the magnitude is the rate of circulation.

- If $\nabla \times \mathbf{F} = 0$ everywhere: $\mathbf{F}$ is irrotational (conservative); its line integral around any closed loop is zero.
- If $\nabla \times \mathbf{F} \neq 0$: $\mathbf{F}$ "swirls" around that region.

*Physical examples*:
- The magnetic field around a straight current-carrying wire has a nonzero curl at the location of the wire (the field circulates around the wire).
- In the Faraday law, $\nabla \times \mathbf{E} \neq 0$ when the magnetic field is changing — the electric field "swirls" around the region of changing flux.

## The Divergence Theorem (Gauss's Theorem)

This theorem relates the volume integral of the divergence of a vector field to the surface integral of the field over the boundary of that volume:

$$\int_V (\nabla \cdot \mathbf{F}) \, dV = \oint_S \mathbf{F} \cdot d\mathbf{A}$$

where $S$ is the closed surface bounding the volume $V$.

**Meaning**: The total "source strength" inside a volume (measured by integrating the divergence) equals the total flux of the field out through the surface. If no field is created or destroyed inside $V$, what goes in must come out.

**Derivation sketch**: Consider an infinitesimal rectangular box of volume $\Delta V = \Delta x \, \Delta y \, \Delta z$. The flux of $\mathbf{F}$ through the two faces perpendicular to $\hat{\mathbf{x}}$ is $[F_x(x+\Delta x, y, z) - F_x(x, y, z)] \Delta y \Delta z \approx (\partial F_x/\partial x) \Delta V$. Similarly for $y$ and $z$ faces. The total flux through all six faces equals $(\nabla \cdot \mathbf{F}) \Delta V$. Summing over all infinitesimal boxes filling the volume $V$, the interior contributions cancel (each interior face is shared by two adjacent boxes with opposite orientation) and only the surface contributions remain, giving the theorem.

## Stokes' Theorem

This theorem relates the surface integral of the curl to the line integral around the boundary:

$$\int_S (\nabla \times \mathbf{F}) \cdot d\mathbf{A} = \oint_C \mathbf{F} \cdot d\boldsymbol{\ell}$$

where $C$ is the closed curve bounding the surface $S$, and the direction of $d\mathbf{A}$ is related to the direction of traversal of $C$ by the right-hand rule.

**Meaning**: The total "rotation" inside a surface (measured by integrating the curl) equals the circulation of the field around the boundary curve. If $\mathbf{F}$ swirls inside the surface, that swirling manifests as a net circulation around the edge.

**Derivation sketch**: Similar to the divergence theorem but for two dimensions. Consider an infinitesimal rectangular patch in the $xy$-plane of area $\Delta A = \Delta x \, \Delta y$. The line integral around this patch (counterclockwise) picks up $(\partial F_y/\partial x - \partial F_x/\partial y) \Delta A = (\nabla \times \mathbf{F})_z \Delta A$. Summing over all patches filling the surface, interior edges cancel, leaving only the boundary.

## The Laplacian

The **Laplacian** of a scalar function is the divergence of the gradient:

$$\nabla^2 f = \nabla \cdot (\nabla f) = \frac{\partial^2 f}{\partial x^2} + \frac{\partial^2 f}{\partial y^2} + \frac{\partial^2 f}{\partial z^2}$$

The Laplacian measures the "curvature" of a function: it is positive where the function is locally below its surroundings, negative where it is above.

The Laplacian of a vector field is defined component by component:

$$\nabla^2 \mathbf{F} = (\nabla^2 F_x)\hat{\mathbf{x}} + (\nabla^2 F_y)\hat{\mathbf{y}} + (\nabla^2 F_z)\hat{\mathbf{z}}$$

We will use the vector identity:

$$\nabla \times (\nabla \times \mathbf{F}) = \nabla(\nabla \cdot \mathbf{F}) - \nabla^2 \mathbf{F}$$

This identity is the key tool for deriving the wave equation from Maxwell's equations (Section 1.4). Commit it to memory, or at least to your reference sheet.
