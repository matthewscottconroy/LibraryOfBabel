# Section 1.3: Maxwell's Equations in Differential Form

## Orientation

The integral form of Maxwell's equations relates field quantities integrated over finite regions (surfaces, loops) to source quantities (total enclosed charge, total enclosed current). This form is closest to experiment. But for theoretical analysis — for deriving the wave equation, analyzing field distributions in waveguides, or understanding boundary conditions at interfaces — we need the differential form, which expresses the field relationships at a single point in space.

The bridge between the integral and differential forms is provided by two theorems of vector calculus: the **divergence theorem** (also called Gauss's theorem) and **Stokes' theorem**. These theorems are not merely mathematical tools; they reflect deep geometric facts about how vector fields behave in three-dimensional space.

This section develops the necessary vector calculus, derives the differential form of Maxwell's equations from the integral form, and explores the meaning of boundary conditions at interfaces between different media.

## The Differential Form at a Glance

$$\nabla \cdot \mathbf{E} = \frac{\rho}{\varepsilon_0}$$

$$\nabla \cdot \mathbf{B} = 0$$

$$\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t}$$

$$\nabla \times \mathbf{B} = \mu_0 \mathbf{J} + \mu_0\varepsilon_0 \frac{\partial \mathbf{E}}{\partial t}$$

Here $\rho$ is the charge density (charge per unit volume, C/m³) and $\mathbf{J}$ is the current density (current per unit area, A/m²). These are the local versions of the same physics as the integral equations.
