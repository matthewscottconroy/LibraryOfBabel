# 6.1.2 TE and TM Modes

## Setting Up the Eigenvalue Problem

We consider a symmetric slab waveguide: core index $n_1$, cladding index $n_2$, core extends from $x = -d/2$ to $x = +d/2$. The fields propagate in the $z$-direction as $e^{i(\beta z - \omega t)}$ and are uniform in $y$.

Maxwell's equations in the three regions (two claddings and core), with the propagating field ansatz, reduce to a second-order ODE for the transverse field component. For **TE modes** (transverse electric: $E_y$ component, $E_x = E_z = 0$):

$$\frac{d^2 E_y}{dx^2} + (n^2k_0^2 - \beta^2)E_y = 0$$

Define:
- Core: $\kappa^2 = n_1^2 k_0^2 - \beta^2 > 0$ (oscillatory in core)
- Cladding: $\gamma^2 = \beta^2 - n_2^2 k_0^2 > 0$ (evanescent in cladding; required for guidance)

## TE Mode Solutions

The solutions in each region must be:
- **Core** ($|x| < d/2$): $E_y = A\cos(\kappa x)$ (even mode) or $A\sin(\kappa x)$ (odd mode)
- **Cladding** ($x > d/2$): $E_y = B e^{-\gamma(x-d/2)}$ (decaying)
- **Cladding** ($x < -d/2$): $E_y = \pm B e^{+\gamma(x+d/2)}$ (+/− for even/odd)

Matching $E_y$ and $H_z = (i/\mu_0\omega)dE_y/dx$ at the interfaces gives:

**Even modes**: $\kappa\tan(\kappa d/2) = \gamma$

**Odd modes**: $-\kappa\cot(\kappa d/2) = \gamma$

## TM Mode Solutions

For **TM modes** (transverse magnetic: $H_y$ component), the same procedure gives:

**Even**: $\kappa\tan(\kappa d/2) = (n_1/n_2)^2 \gamma$

**Odd**: $-\kappa\cot(\kappa d/2) = (n_1/n_2)^2 \gamma$

The TM modes differ from TE modes by the factor $(n_1/n_2)^2$ in the matching condition. This factor makes TM modes less tightly confined than TE modes for the same core thickness (TM modes extend further into the cladding). In high-index-contrast waveguides (silicon photonics), this difference is very pronounced — TE and TM modes have significantly different effective indices, dispersions, and propagation properties.

## Mode Classification and Physical Meaning

- $m = 0$ (fundamental mode, even): single intensity lobe at center of core
- $m = 1$ (first higher order mode, odd): two lobes, one node at center
- $m = 2$ (second order, even): three lobes, two nodes

For silicon photonics: the strong confinement ($\Delta n \approx 2$) in a 450 nm waveguide means that at 1550 nm, typically only the fundamental TE₀₀ and TM₀₀ modes are guided. Higher-order modes are beyond cutoff. This is the single-mode condition discussed in Section 6.1.3.

**Why TE is preferred in silicon photonics**: Ring resonators, directional couplers, and MZIs designed for photonic computing are almost universally TE-polarized because:
1. The grating coupler (standard fiber coupling interface) preferentially excites TE
2. TE modes in a 450 × 220 nm waveguide have lower loss than TM
3. TE-polarized ring resonators have sharper resonances (higher Q) due to better confinement
4. Device behavior is more reproducible for TE (less sensitive to height variations)
