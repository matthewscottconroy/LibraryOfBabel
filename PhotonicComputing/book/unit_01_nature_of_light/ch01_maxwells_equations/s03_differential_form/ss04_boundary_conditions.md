# 1.3.4 Boundary Conditions at Interfaces

## Why Boundary Conditions Matter for Photonic Computing

Every photonic device involves interfaces between materials with different electromagnetic properties: the silicon-SiO₂ interface in a waveguide, the air-facet interface of a laser chip, the metal-dielectric interface in a plasmonic device. At each interface, the electromagnetic fields must satisfy conditions that follow directly from Maxwell's equations. These boundary conditions determine how much light reflects and transmits, how optical modes are confined, and what field distributions are supported.

Boundary conditions are not additional physical laws — they are consequences of Maxwell's equations. We derive them here.

## Strategy: Apply Integral Maxwell's Equations to Infinitesimal Regions

The idea is to choose surfaces or loops that span the interface but are infinitesimally thin in the direction perpendicular to the interface, so that all the "action" occurs at the boundary plane itself.

Let medium 1 (above the interface) have parameters $\varepsilon_1$, $\mu_1$, and medium 2 (below) have parameters $\varepsilon_2$, $\mu_2$. Let $\hat{\mathbf{n}}$ be the unit normal pointing from medium 2 into medium 1.

## Normal Components: From Gauss's Laws

**Apply Gauss's law for $\mathbf{D}$** to a thin pillbox spanning the interface (area $A$, height $\delta \to 0$):

The flux through the top and bottom faces: $(D_{1n} - D_{2n})A$.
The flux through the side: vanishes as $\delta \to 0$.
The enclosed free charge: $\sigma_f A$, where $\sigma_f$ is the surface free charge density (C/m²).

Gauss's law gives:
$$(D_{1n} - D_{2n}) = \sigma_f$$

In the absence of free surface charges (the common case for interfaces between insulators like glass and silicon, or between undoped semiconductor regions):
$$\boxed{D_{1n} = D_{2n}} \quad \Rightarrow \quad \varepsilon_1 E_{1n} = \varepsilon_2 E_{2n}$$

The normal component of $\mathbf{D}$ is continuous across a charge-free interface. The normal component of $\mathbf{E}$ is *discontinuous* — it jumps by a factor $\varepsilon_2/\varepsilon_1$.

**Apply Gauss's law for $\mathbf{B}$** to the same pillbox. Since $\nabla \cdot \mathbf{B} = 0$ everywhere:
$$\boxed{B_{1n} = B_{2n}}$$

The normal component of $\mathbf{B}$ is always continuous.

## Tangential Components: From Stokes' Theorem

**Apply Faraday's law** to a thin rectangular loop spanning the interface (length $\ell$ along the interface, height $\delta \to 0$):

The line integral of $\mathbf{E}$ along the top edge (in medium 1): $E_{1t}\ell$.
The line integral along the bottom edge (in medium 2): $-E_{2t}\ell$ (opposite direction of traversal).
The contribution of the sides: vanishes as $\delta \to 0$.
The rate of change of magnetic flux: $\sim \partial B/\partial t \cdot \ell \cdot \delta \to 0$ as $\delta \to 0$.

Therefore:
$$\boxed{E_{1t} = E_{2t}}$$

**The tangential component of $\mathbf{E}$ is continuous across any interface.** This is one of the most important results in electromagnetism.

**Apply the Ampère-Maxwell law** to the same loop. If there is no surface current density (true for insulators and most dielectrics):
$$\boxed{H_{1t} = H_{2t}}$$

## The Four Boundary Conditions

| Field | Condition | Reason |
|-------|-----------|--------|
| Normal $\mathbf{D}$ | $D_{1n} - D_{2n} = \sigma_f$ | Gauss's law for $\mathbf{E}$ |
| Normal $\mathbf{B}$ | $B_{1n} = B_{2n}$ | Gauss's law for $\mathbf{B}$ |
| Tangential $\mathbf{E}$ | $E_{1t} = E_{2t}$ | Faraday's law |
| Tangential $\mathbf{H}$ | $H_{1t} - H_{2t} = \mathbf{K}_f \times \hat{\mathbf{n}}$ | Ampère-Maxwell law |

Here $\mathbf{K}_f$ is the surface free current density (if any). For a perfect conductor, $\mathbf{K}_f$ can be nonzero.

## Application: Total Internal Reflection and Waveguiding

The boundary conditions determine what happens when light hits an interface between media with different refractive indices. The full analysis (Chapter 2) yields the Fresnel equations for reflection and transmission coefficients. Here we just note the key consequence for waveguides.

At the interface between silicon ($n_1 = 3.47$) and SiO₂ ($n_2 = 1.44$), the continuity of tangential $\mathbf{E}$ and normal $\mathbf{D}$ requires the field to be continuous at the interface while adjusting its direction. For light hitting the interface at a sufficiently steep angle (greater than the critical angle $\theta_c = \arcsin(n_2/n_1) = \arcsin(1.44/3.47) \approx 24.5°$), the solution to the field equations has no propagating wave in medium 2 — only an evanescent field that decays exponentially away from the interface. All the power is reflected back into medium 1. This is total internal reflection, and it is the mechanism by which silicon photonic waveguides confine light.

The boundary conditions also determine the discrete set of field distributions (modes) supported by a waveguide of given geometry — we analyze this in Chapter 6.

## Application: Fresnel Coefficients

From the boundary conditions, one can derive the Fresnel reflection and transmission coefficients for a plane wave incident on an interface. For a wave polarized with $\mathbf{E}$ perpendicular to the plane of incidence (TE polarization, also called s-polarization):

$$r_s = \frac{n_1\cos\theta_i - n_2\cos\theta_t}{n_1\cos\theta_i + n_2\cos\theta_t}$$

$$t_s = \frac{2n_1\cos\theta_i}{n_1\cos\theta_i + n_2\cos\theta_t}$$

where $\theta_i$ is the angle of incidence and $\theta_t = \arcsin[(n_1/n_2)\sin\theta_i]$ is the transmitted angle (from Snell's law). These are derived by applying the four boundary conditions and solving the resulting system of linear equations. The derivation is left as Exercise 1.5.
