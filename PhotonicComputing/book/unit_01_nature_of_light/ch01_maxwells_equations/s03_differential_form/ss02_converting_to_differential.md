# 1.3.2 Converting Maxwell's Equations to Differential Form

## The Strategy

Each integral form of Maxwell's equations can be converted to differential form by applying the divergence theorem (for the surface-integral equations) or Stokes' theorem (for the loop-integral equations). The result is a relationship that holds at every individual point in space.

## Gauss's Law for $\mathbf{E}$: Divergence Theorem

Start with:
$$\oint_S \mathbf{E} \cdot d\mathbf{A} = \frac{Q_{\text{enc}}}{\varepsilon_0}$$

Express the enclosed charge as a volume integral of the charge density $\rho$:
$$Q_{\text{enc}} = \int_V \rho \, dV$$

Apply the divergence theorem to the left side:
$$\oint_S \mathbf{E} \cdot d\mathbf{A} = \int_V (\nabla \cdot \mathbf{E}) \, dV$$

The equation becomes:
$$\int_V (\nabla \cdot \mathbf{E}) \, dV = \int_V \frac{\rho}{\varepsilon_0} \, dV$$

Since this holds for *any* volume $V$ — large or small, any shape — the integrands must be equal point by point:

$$\boxed{\nabla \cdot \mathbf{E} = \frac{\rho}{\varepsilon_0}}$$

This is the differential form of Gauss's law: the divergence of $\mathbf{E}$ at a point equals the charge density at that point divided by $\varepsilon_0$. Where there is a charge, there is a source of $\mathbf{E}$ field lines.

## Gauss's Law for $\mathbf{B}$

Identically, applying the divergence theorem to $\oint_S \mathbf{B} \cdot d\mathbf{A} = 0$ and using the same argument:

$$\boxed{\nabla \cdot \mathbf{B} = 0}$$

The divergence of $\mathbf{B}$ is zero everywhere. No magnetic monopoles.

## Faraday's Law: Stokes' Theorem

Start with:
$$\oint_C \mathbf{E} \cdot d\boldsymbol{\ell} = -\frac{d}{dt}\int_S \mathbf{B} \cdot d\mathbf{A}$$

Apply Stokes' theorem to the left side:
$$\oint_C \mathbf{E} \cdot d\boldsymbol{\ell} = \int_S (\nabla \times \mathbf{E}) \cdot d\mathbf{A}$$

Move the time derivative inside the integral on the right (valid when the surface $S$ is fixed in space):
$$-\frac{d}{dt}\int_S \mathbf{B} \cdot d\mathbf{A} = -\int_S \frac{\partial \mathbf{B}}{\partial t} \cdot d\mathbf{A}$$

The equation becomes:
$$\int_S (\nabla \times \mathbf{E}) \cdot d\mathbf{A} = -\int_S \frac{\partial \mathbf{B}}{\partial t} \cdot d\mathbf{A}$$

Since this holds for any surface:
$$\boxed{\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t}}$$

The curl of $\mathbf{E}$ at a point equals the negative rate of change of $\mathbf{B}$ at that point. Where $\mathbf{B}$ is changing, the electric field "swirls" around that point.

## Ampère-Maxwell Law

Applying the same procedure to the Ampère-Maxwell integral equation:

$$\boxed{\nabla \times \mathbf{B} = \mu_0 \mathbf{J} + \mu_0\varepsilon_0 \frac{\partial \mathbf{E}}{\partial t}}$$

The curl of $\mathbf{B}$ at a point equals $\mu_0$ times the current density $\mathbf{J}$ plus $\mu_0\varepsilon_0$ times the rate of change of $\mathbf{E}$ at that point.

## The Differential Equations: A Summary

$$\nabla \cdot \mathbf{E} = \frac{\rho}{\varepsilon_0} \tag{1}$$

$$\nabla \cdot \mathbf{B} = 0 \tag{2}$$

$$\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t} \tag{3}$$

$$\nabla \times \mathbf{B} = \mu_0 \mathbf{J} + \mu_0\varepsilon_0 \frac{\partial \mathbf{E}}{\partial t} \tag{4}$$

These four partial differential equations completely specify the electromagnetic field given the sources ($\rho$ and $\mathbf{J}$) and appropriate initial and boundary conditions.

## The Continuity Equation (Charge Conservation)

As a consistency check, take the divergence of equation (4):

$$\nabla \cdot (\nabla \times \mathbf{B}) = \mu_0 \nabla \cdot \mathbf{J} + \mu_0\varepsilon_0 \frac{\partial}{\partial t}(\nabla \cdot \mathbf{E})$$

The left side is always zero: $\nabla \cdot (\nabla \times \mathbf{A}) = 0$ for any vector field $\mathbf{A}$ (the divergence of a curl is identically zero, as can be verified by direct computation in Cartesian coordinates).

Substituting equation (1) for $\nabla \cdot \mathbf{E}$:

$$0 = \mu_0 \nabla \cdot \mathbf{J} + \mu_0\varepsilon_0 \frac{\partial}{\partial t}\left(\frac{\rho}{\varepsilon_0}\right) = \mu_0\left(\nabla \cdot \mathbf{J} + \frac{\partial \rho}{\partial t}\right)$$

Therefore:
$$\frac{\partial \rho}{\partial t} + \nabla \cdot \mathbf{J} = 0$$

This is the **continuity equation** — the mathematical expression of charge conservation. It is automatically satisfied by Maxwell's equations, not imposed separately. This is not surprising: Maxwell added the displacement current specifically to restore this consistency.
