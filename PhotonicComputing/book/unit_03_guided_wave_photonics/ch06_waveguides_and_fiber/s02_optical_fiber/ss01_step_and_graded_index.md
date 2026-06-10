# 6.2.1 Step-Index and Graded-Index Fiber; LP Modes

## Cylindrical Waveguide Modes

In a cylindrical (step-index) fiber with core radius $a$, core index $n_1$, cladding index $n_2$, Maxwell's equations in cylindrical coordinates give six field components. The exact solution requires Bessel functions $J_m(\kappa r)$ in the core and modified Bessel functions $K_m(\gamma r)$ in the cladding:

Core ($r < a$): $E_z = A J_m(\kappa r) e^{im\phi} e^{i\beta z}$

Cladding ($r > a$): $E_z = B K_m(\gamma r) e^{im\phi} e^{i\beta z}$

where $\kappa^2 = n_1^2 k_0^2 - \beta^2$ and $\gamma^2 = \beta^2 - n_2^2 k_0^2$, as before.

The exact modes of a step-index fiber are **HE$_{mn}$** and **EH$_{mn}$** modes (hybrid modes), plus the **TE$_{0n}$** and **TM$_{0n}$** modes (which have no azimuthal variation). The subscript $m$ is the azimuthal mode number; $n$ counts radial modes.

## Weakly Guiding Approximation and LP Modes

For fibers with small index contrast ($\Delta = (n_1^2 - n_2^2)/(2n_1^2) \ll 1$ — valid for standard silica fiber with $\Delta \approx 0.003$), the exact modes degenerate into pairs that can be combined into linearly polarized (LP) modes:

$$\text{LP}_{lm} = \begin{cases} \text{HE}_{l+1,m} & l \geq 1 \\ \text{TE}_{0m} + \text{TM}_{0m} + \text{HE}_{2m} & l = 0 \end{cases}$$

The LP$_{lm}$ mode has the field profile of a Bessel function $J_l(\kappa r)$ in the core. The mode number $l$ gives the azimuthal variation, $m$ the radial node count.

**LP mode eigenvalue equation** (characteristic equation):

$$\frac{J_{l-1}(u)}{J_l(u)} = -\frac{\gamma a}{u a} \cdot \frac{K_{l-1}(w)}{K_l(w)} \cdot \frac{u/a}{w/a}$$

where $u = \kappa a$ and $w = \gamma a$, subject to $u^2 + w^2 = V^2 = (k_0 a \text{NA})^2$.

## Single-Mode Condition for Fiber

The LP$_{01}$ mode (fundamental, analogous to TEM$_{00}$ Gaussian) has no cutoff. The LP$_{11}$ mode (first higher-order) has cutoff at $V_c = 2.405$ (the first zero of $J_0$). Single-mode operation requires:

$$V = \frac{2\pi a}{\lambda}\text{NA} < 2.405$$

For SMF-28 at 1550 nm: $a = 4.5$ μm, NA = 0.14:

$$V = \frac{2\pi \times 4.5 \times 10^{-6}}{1.55 \times 10^{-6}} \times 0.14 = 2.56 \times 0.14 \times 2\pi = 2.56 \text{ (approximately)}$$

Actually: $V = 2\pi \times 4.5/1.55 \times 0.14 = 2.55 < 2.405$? Let me correct: SMF-28 cutoff wavelength $\lambda_c = 1260$ nm; at 1550 nm it is single-mode. The cutoff wavelength is:

$$\lambda_c = \frac{2\pi a \text{NA}}{2.405}$$

For single-mode at 1310 nm: $a < \lambda/(2.405/(2\pi\text{NA})) = 1310 \text{ nm}/(2.405/(2\pi \times 0.14)) = 4.88$ μm. SMF-28 uses $a = 4.1$ μm core radius with NA = 0.14, giving $V(1310\text{nm}) = 2.17$ — safely single-mode.

## Graded-Index Fiber

Graded-index (GRIN) fiber has a continuously varying core index:

$$n(r) = n_1\left(1 - 2\Delta (r/a)^\alpha\right)^{1/2} \approx n_1\left(1 - \Delta(r/a)^\alpha\right)$$

For $\alpha = 2$ (parabolic profile): the fiber acts as a distributed lens with focusing equivalent to a GRIN lens. All modes travel the same total optical path length (faster modes travel a longer geometric path in the lower-index outer region), giving minimal *intermodal dispersion*. The optimal $\alpha$ for minimum intermodal dispersion is:

$$\alpha_{opt} = 2 + \frac{12\Delta}{5} + \ldots \approx 2.01 \text{ for silica}$$

GRIN multimode fiber (OM3/OM4 type, 50 μm core, $\alpha = 2$) is the standard for short-reach data center interconnects at 850 nm with 25–100 Gbps VCSELs. Its large core (50 μm vs. 9 μm for SMF) makes alignment tolerant and coupling efficient but supports many modes ($V \sim 30$) — it cannot be used for coherent transmission or dispersion-sensitive applications.
