# Section 1.2: Maxwell's Equations in Integral Form

## Orientation

The integral form of Maxwell's equations is the form closest to experiment. Each equation relates a quantity integrated over a surface or around a loop — quantities we can measure with probes, coils, and voltmeters — to a source term that we can control or characterize. This is why the integral form is the natural starting point, and why it was the form in which the laws were originally discovered.

The differential form (Section 1.3) is more powerful for theoretical analysis because it expresses the field relationships point by point rather than over finite regions. But it requires vector calculus tools — the divergence theorem and Stokes' theorem — to derive from the integral form. We develop those tools in Section 1.3.

For now: integral form first, physical meaning paramount.

## The Four Equations at a Glance

$$\oint_S \mathbf{E} \cdot d\mathbf{A} = \frac{Q_{\text{enc}}}{\varepsilon_0} \quad \text{(Gauss's law for } \mathbf{E}\text{)}$$

$$\oint_S \mathbf{B} \cdot d\mathbf{A} = 0 \quad \text{(Gauss's law for } \mathbf{B}\text{)}$$

$$\oint_C \mathbf{E} \cdot d\boldsymbol{\ell} = -\frac{d}{dt}\int_S \mathbf{B} \cdot d\mathbf{A} \quad \text{(Faraday's law)}$$

$$\oint_C \mathbf{B} \cdot d\boldsymbol{\ell} = \mu_0 I_{\text{enc}} + \mu_0\varepsilon_0 \frac{d}{dt}\int_S \mathbf{E} \cdot d\mathbf{A} \quad \text{(Ampère-Maxwell law)}$$

In the subsections that follow, we examine each equation in detail: what it says mathematically, what it means physically, and how it constrains the fields in situations relevant to photonic computing.
