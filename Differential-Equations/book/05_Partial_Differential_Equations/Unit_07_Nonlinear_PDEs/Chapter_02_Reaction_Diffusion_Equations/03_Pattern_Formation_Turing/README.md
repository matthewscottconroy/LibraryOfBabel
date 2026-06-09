# Pattern Formation and the Turing Instability

In 1952, Alan Turing published "The Chemical Basis of Morphogenesis," proposing that the spatial patterns observed in biological development — stripes, spots, spirals, the arrangement of organs — could arise from a purely chemical mechanism. The key insight was counterintuitive: **diffusion, which normally smooths out concentration differences, can under certain conditions destabilize a spatially uniform state and cause the spontaneous formation of spatial patterns**. This mechanism — now called the **Turing instability** or diffusion-driven instability — requires two interacting chemical species with very different diffusion rates.

## Two-Component Reaction-Diffusion System

Consider a two-component system:

$$\begin{cases}u_t = D_u\Delta u + f(u,v) \\ v_t = D_v\Delta v + g(u,v)\end{cases} \tag{RDS}$$

where $u$ and $v$ are concentrations of two chemical species, $D_u$ and $D_v$ are their diffusion coefficients, and $f$, $g$ describe the reaction kinetics. The standard Turing scenario requires:
- $u$ is an **activator**: $f_u > 0$ (it promotes its own production).
- $v$ is an **inhibitor**: $g_v < 0$ (it inhibits its own production) and it inhibits $u$ ($f_v < 0$).
- $D_v \gg D_u$: the inhibitor diffuses much faster than the activator.

## Linear Stability Analysis

**Step 1: Find a uniform steady state.** A constant solution $(u^*,v^*)$ satisfies $f(u^*,v^*) = 0$ and $g(u^*,v^*) = 0$.

**Step 2: Stability without diffusion.** Linearize (RDS) around $(u^*,v^*)$ with no spatial variation. Let $\mathbf{w} = (u-u^*, v-v^*)^T$ be a small perturbation:

$$\mathbf{w}_t = J\mathbf{w}, \qquad J = \begin{pmatrix}f_u & f_v \\ g_u & g_v\end{pmatrix}\bigg|_{(u^*,v^*)}.$$

For the uniform state to be stable: $\text{tr}J < 0$ and $\det J > 0$, i.e., $f_u + g_v < 0$ and $f_ug_v - f_vg_u > 0$.

**Step 3: Stability with diffusion.** Now allow spatial variation. Expand perturbations in spatial Fourier modes: $\mathbf{w} = \hat{\mathbf{w}}e^{\sigma t + ik\cdot x}$ (on $\mathbb{R}^n$; on a bounded domain, replace $k$ by the Laplacian eigenvalues). Substituting into the linearized (RDS):

$$\sigma\hat{\mathbf{w}} = (J - Dk^2)\hat{\mathbf{w}}, \qquad D = \begin{pmatrix}D_u & 0 \\ 0 & D_v\end{pmatrix}, \quad k = |\mathbf{k}|.$$

The growth rate $\sigma$ is an eigenvalue of the matrix:

$$J(k^2) = J - Dk^2 = \begin{pmatrix}f_u - D_u k^2 & f_v \\ g_u & g_v - D_v k^2\end{pmatrix}.$$

The characteristic equation is $\sigma^2 - \text{tr}(J(k^2))\sigma + \det(J(k^2)) = 0$, where:

$$\text{tr}(J(k^2)) = f_u + g_v - (D_u + D_v)k^2,$$

$$\det(J(k^2)) = (f_u - D_u k^2)(g_v - D_v k^2) - f_v g_u = D_u D_v k^4 - (D_v f_u + D_u g_v)k^2 + \det J.$$

**Turing instability** occurs when there exists $k > 0$ such that $\text{Re}(\sigma) > 0$ — i.e., the mode $e^{ik\cdot x}$ grows in time. Since $\text{tr}(J(k^2)) < 0$ for all $k$ (because $\text{tr}J < 0$), the instability arises from $\det(J(k^2)) < 0$ (the product of the two eigenvalues is negative, so one is positive and one is negative).

## Turing Instability Conditions

**$\det(J(k^2)) < 0$ for some $k > 0$** requires the parabola $h(k^2) = D_u D_v k^4 - (D_v f_u + D_u g_v)k^2 + \det J$ to have a minimum below zero.

The minimum of $h$ over $k^2 > 0$ occurs at:

$$k_{\min}^2 = \frac{D_v f_u + D_u g_v}{2D_u D_v}.$$

For this to be positive (a real instability wavenumber), we need $D_v f_u + D_u g_v > 0$. Since $f_u + g_v < 0$ (stability without diffusion), and $f_u > 0$ (activator promotes itself), the condition $D_v f_u + D_u g_v > 0$ requires $D_v/D_u > -g_v/f_u > 0$ — the inhibitor must diffuse faster than the activator by a factor of at least $-g_v/f_u$.

The minimum value of $h$ is:

$$h_{\min} = \det J - \frac{(D_v f_u + D_u g_v)^2}{4D_u D_v}.$$

**Turing instability condition:** $h_{\min} < 0$, i.e.:

$$(D_v f_u + D_u g_v)^2 > 4D_u D_v\det J. \tag{Turing}$$

This requires both $D_v f_u + D_u g_v > 0$ and the inequality above. The critical ratio $d_c = D_v/D_u$ at which Turing instability first appears is found by setting $h_{\min} = 0$.

## Critical Wavenumber and Pattern Wavelength

At the onset of Turing instability ($h_{\min} = 0$), the unstable mode has wavenumber:

$$k^{*2} = \sqrt{\frac{\det J}{D_u D_v}}.$$

This selects a preferred spatial wavelength:

$$\lambda^* = \frac{2\pi}{k^*} = 2\pi\!\left(\frac{D_u D_v}{\det J}\right)^{1/4}.$$

As $D_v/D_u \to \infty$ (inhibitor diffuses much faster), $D_v$ drops out of $\det J$ and $\lambda^* \sim (D_u/\det J)^{1/4}$ — the pattern scale is set by the activator's diffusion length.

**Physical interpretation.** The wavelength $\lambda^*$ is the scale at which the inhibitor's fast diffusion can exactly balance the activator's local positive feedback. Smaller scales: the inhibitor diffuses away before the activator can amplify — stability. Larger scales: the activator can grow locally before the inhibitor catches up — instability. The Turing mechanism thus selects a finite wavelength, neither zero nor infinity.

## The Gierer-Meinhardt Model

A canonical activator-inhibitor model proposed by Gierer and Meinhardt (1972):

$$u_t = D_u\Delta u + \frac{u^2}{v} - \mu u + \sigma, \qquad v_t = D_v\Delta v + u^2 - \nu v. \tag{GM}$$

Here $u$ is the activator (auto-catalytic: $u^2/v$), $v$ is the inhibitor (produced by $u^2$, degraded by $\nu v$), $\mu$ and $\nu$ are degradation rates, $\sigma$ is a basal production rate for $u$.

**Uniform steady state:** $u^* = (\sigma + u^{*2}/v^*)/\mu$ and $v^* = u^{*2}/\nu$. Solving: $u^* = \sigma\nu/(\mu\nu - 1)$ ... The exact steady state depends on parameters, but the qualitative structure is: $f_u = u/v^* - \mu > 0$ (activator promotes itself), $g_v = -\nu < 0$ (inhibitor degrades).

**Applications.** The Gierer-Meinhardt model predicts:
- **Spots** when the activator is isotropic and inhibitor is fast: pigmentation spots on animal coats (cheetah spots, giraffe patches).
- **Stripes** in anisotropic variants: zebra stripes, zebrafish patterns.
- **Spatial gradients** in developmental biology: the French Flag model of positional information.
- **Spirals** in excitable media (cardiac tissue, Belousov-Zhabotinsky reaction).

## Nonlinear Saturation and Pattern Amplitude

The linear stability analysis predicts which wavenumbers grow, but it cannot determine the pattern amplitude (the nonlinear term saturates the exponential growth). Near the bifurcation point (where the Turing condition is just satisfied), weakly nonlinear theory applies. Setting $d = D_v/D_u = d_c + \varepsilon^2$ (small supercriticality), the amplitude $A$ of the pattern evolves on a slow time scale $T = \varepsilon^2 t$ according to the **amplitude equation** (Stuart-Landau equation):

$$\frac{dA}{dT} = \mu A - \beta |A|^2 A,$$

where $\mu > 0$ is the growth rate and $\beta$ is determined by the nonlinear terms in $f$ and $g$. The saturated amplitude is $|A|^2 = \mu/\beta$ (for $\beta > 0$, supercritical bifurcation — stable pattern).

In two spatial dimensions, the amplitude equations become a system of coupled nonlinear PDEs (Newell-Whitehead-Segel equations) that determine whether the pattern is stripes, hexagons, or spots. The selection between stripes and hexagons depends on the cubic vs. quadratic terms in the amplitude equations, which in turn depend on the specific reaction kinetics.

## Comparison with Linear Diffusion

The contrast with pure linear diffusion (heat equation) is stark:

| | Heat Equation | Turing RD System |
|---|---|---|
| Uniform state | Stable | Can be unstable |
| Role of diffusion | Stabilizing | Can be destabilizing |
| Long-time behavior | Uniform equilibrium | Spatial pattern |
| Preferred wavelength | None (all modes decay) | $\lambda^* = 2\pi/k^*$ |

This table encapsulates Turing's paradox: diffusion, which normally reduces spatial variation, becomes the mechanism for creating it when two species interact with different diffusion rates.
