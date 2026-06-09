# Unit 7: Nonlinear Partial Differential Equations

The equations studied in Units 3, 4, and 5 — heat, wave, and Laplace — are all linear. Their solutions obey superposition: if $u$ and $v$ satisfy the equation, so does $au+bv$. This linearity enabled the method of separation of variables, Fourier expansion, and explicit solutions in terms of known special functions. Nonlinear PDEs abandon superposition entirely. New phenomena emerge that have no linear analogue: solutions can blow up in finite time, concentrate into thin layers, form shock discontinuities, or organize into stable patterns. This unit introduces the main classes of nonlinear PDEs and three of the most important examples: reaction-diffusion equations (Fisher's equation), Burgers' equation, and the Korteweg-de Vries (KdV) equation.

## What Makes Nonlinearity Hard

For a linear PDE $Lu = f$ with bounded coefficients, the solution operator $L^{-1}$ exists (under appropriate conditions) and is continuous: small changes in $f$ produce small changes in $u$. For nonlinear PDEs, existence is a theorem to be proved separately for each equation, uniqueness may fail, and solutions may not depend continuously on initial data (well-posedness can break down in finite time). Global existence for even the simplest nonlinear wave equation in 3D remains one of the central problems of geometric analysis (global existence for Yang-Mills fields, critical exponent blow-up for semilinear wave equations, etc.).

Three fundamental difficulties distinguish nonlinear PDEs from linear ones:

1. **Finite-time blow-up.** For the ODE $u' = u^2$ with $u(0) = 1$: $u(t) = 1/(1-t)$ blows up at $t=1$. For the PDE $u_t = u^2$ (no spatial operator), the same blow-up occurs. When nonlinear reaction terms are added to the heat equation ($u_t = \Delta u + u^p$), blow-up can occur for $p > 1$ and large initial data.

2. **Shock formation.** For the inviscid Burgers equation $u_t + uu_x = 0$, characteristics are straight lines with slope $1/u(x,0)$. If $u_0'(x) < 0$ somewhere, characteristics cross in finite time, and the smooth solution ceases to exist. The solution continues as a weak solution with a propagating discontinuity (shock).

3. **Pattern formation and instability.** For reaction-diffusion systems, diffusion is normally stabilizing (spreading concentrations), but when the diffusion rates of two species are very different, the interplay between diffusion and reaction can cause spatially uniform states to become unstable, generating spatial patterns (Turing instability). This mechanism underlies the formation of animal coat patterns, chemical waves, and developmental biology.

## Structure of This Unit

**Chapter 1: Introduction to Nonlinear PDEs** classifies nonlinear PDEs (semilinear, quasilinear, fully nonlinear) and surveys the challenges: loss of superposition, need for new solution concepts (weak solutions, viscosity solutions, entropy solutions), and the interplay between analysis and geometry.

**Chapter 2: Reaction-Diffusion Equations** studies $u_t = D\Delta u + f(u)$ with nonlinear reaction term $f$. The prototypical example is **Fisher's equation** $u_t = Du_{xx} + ru(1-u)$ (logistic growth with diffusion), which models the spatial spread of advantageous genes. The key phenomenon is **traveling wave solutions** $u(x,t) = U(x-ct)$ — spatially translating profiles with characteristic speed $c$. The Turing instability (Chapter 2.3) shows how two-component reaction-diffusion systems generate spatial patterns.

**Chapter 3: Burgers' Equation** $u_t + uu_x = \varepsilon u_{xx}$ provides the model problem for nonlinear wave phenomena with and without viscosity. The inviscid Burgers equation ($\varepsilon=0$) captures shock formation; the viscous equation ($\varepsilon>0$) is exactly linearizable via the **Hopf-Cole transformation** $u = -2\varepsilon(\log\phi)_x$, reducing it to the heat equation. The exact solution allows a complete description of shock structure, shock thickness ($\sim\varepsilon$), and the inviscid limit $\varepsilon\to 0$.

**Chapter 4: KdV Equation** $u_t + 6uu_x + u_{xxx} = 0$ introduces dispersive nonlinear PDEs, whose solutions exhibit a radically different behavior from dissipative (heat-type) equations. The KdV equation has exact **soliton solutions** — localized traveling waves that emerge unchanged after collisions with other solitons, an astonishing nonlinear superposition principle. The inverse scattering transform (IST) provides a complete solution method, conceptually the analog of the Fourier transform for this nonlinear equation.

## Prerequisites and Level

This unit assumes mastery of the method of characteristics (Unit 2), including the Rankine-Hugoniot condition and entropy conditions for scalar conservation laws. The heat kernel and Fourier transform (Unit 3) are used in the Hopf-Cole reduction. The reader should also be comfortable with phase plane analysis for ODEs (second-order autonomous systems), which governs the existence of traveling wave solutions.

The material in this unit is more analytic and less computational than earlier units: the emphasis is on understanding qualitative behavior, existence of special solutions, and the interplay between nonlinearity and the structure (dispersion vs. dissipation) of the PDE.
