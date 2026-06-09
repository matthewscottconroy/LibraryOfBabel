# Chapter 2: Reaction-Diffusion Equations

Reaction-diffusion equations $u_t = D\Delta u + f(u)$ couple two competing processes: diffusion, which spreads concentration uniformly and tends to smooth spatial gradients, and reaction (or growth), which modifies the local concentration based on its current value. The interplay between these two mechanisms produces a rich variety of phenomena: traveling waves propagating at fixed speed, spatial patterns arising from diffusion-driven instability, pulse solutions in excitable media, and front propagation in combustion and ecology.

## Structure and Physical Origins

A single-species reaction-diffusion equation has the form:

$$u_t = D\Delta u + f(u), \qquad D > 0, \quad f:\mathbb{R}\to\mathbb{R}. \tag{RD}$$

The function $f(u)$ is the reaction (or source) term, representing local production/consumption of the quantity $u$.

**Ecological interpretation.** $u(x,t)$ is the population density of a species at location $x$ and time $t$. Diffusion models random dispersal (each individual moves randomly, net flux down gradients). The reaction term $f(u)$ encodes birth-death dynamics: $f(u) = ru$ (exponential growth), $f(u) = ru(1-u/K)$ (logistic growth with carrying capacity $K$).

**Chemical kinetics.** $u$ is the concentration of a chemical species. Reaction term $f(u)$ gives the net production rate by chemical reactions. Diffusion (Fick's law) spreads the concentration.

**Combustion.** $u$ is temperature; $f(u) = \lambda e^{-E/u}$ (Arrhenius kinetics). The exponential reaction term causes the temperature to jump sharply in flame fronts.

## The Two Sections of This Chapter

**Section 1: Fisher's Equation** is the canonical single-species reaction-diffusion equation:

$$u_t = Du_{xx} + ru(1-u),$$

proposed independently by Fisher (population genetics, 1937) and Kolmogorov-Petrovskii-Piskunov (1937) as a model for the spatial spread of an advantageous gene allele. The reaction term $f(u) = ru(1-u)$ has roots at $u=0$ (extinction equilibrium, unstable) and $u=1$ (carrying capacity, stable). The equation admits traveling wave solutions $u(x,t) = U(x-ct)$ that propagate the stable state $u=1$ into the unstable state $u=0$. The minimum wave speed $c^* = 2\sqrt{rD}$ (Fisher's speed) is determined by the linear stability analysis at $u=0$.

**Section 2: Traveling Waves** treats the general theory of traveling front solutions $U(\xi)$ (with $\xi = x-ct$) for reaction-diffusion equations $u_t = Du_{xx} + f(u)$. The ODE system for $U$ in the traveling frame is:

$$DU'' + cU' + f(U) = 0, \quad U(-\infty) = 1, \quad U(+\infty) = 0.$$

Phase plane analysis in the $(U,U')$ plane determines when heteroclinic orbits (connecting the two equilibria $(0,0)$ and $(1,0)$) exist and at what speeds $c$. For monostable $f$ (one stable equilibrium), there is a minimal speed $c^*$; for bistable $f$ (two stable equilibria), a unique speed $c^*$ exists.

**Section 3: Pattern Formation and Turing Instability** examines two-component reaction-diffusion systems:

$$\begin{cases}u_t = D_u\Delta u + f(u,v) \\ v_t = D_v\Delta v + g(u,v)\end{cases}$$

A uniform steady state $(u^*,v^*)$ that is stable to spatially uniform perturbations can become unstable to spatially varying perturbations if $D_u \ll D_v$ (the inhibitor diffuses much faster than the activator). This **Turing instability** (Turing, 1952, "The Chemical Basis of Morphogenesis") is the mathematical mechanism for spontaneous pattern formation: stripes, spots, hexagons. The instability selects a preferred spatial wavelength $\lambda^* = 2\pi/k^*$, where $k^*$ is the wave number at which the growth rate is maximized.

## Key Results to Be Developed

**Fisher-KPP theorem.** For Fisher's equation with initial data $0 \leq u_0 \leq 1$, $u_0 \not\equiv 0$ with compact support: the solution satisfies $u(x,t) \to 1$ locally uniformly as $t\to\infty$, and the level set $\{x: u(x,t) = 1/2\}$ moves at speed $c^* = 2\sqrt{rD}$ as $t\to\infty$. This is the mathematical statement that the species spreads at the minimal wave speed.

**Turing instability criterion.** The two-component system linearized around $(u^*,v^*)$ has eigenvalues determined by the Jacobian $J = \begin{pmatrix}f_u & f_v \\ g_u & g_v\end{pmatrix}$ evaluated at the steady state. For Turing instability, the conditions are:
1. The steady state is stable without diffusion: $\text{tr}J < 0$ and $\det J > 0$.
2. With diffusion, there exists $k > 0$ such that the eigenvalues of $J - Dk^2$ (where $D = \text{diag}(D_u,D_v)$) have positive real part.

The condition reduces to: $D_v f_u + D_u g_v > 2\sqrt{D_u D_v\det J}$ (with $f_u > 0$ and $g_v < 0$: an activator-inhibitor system), and requires $D_v/D_u$ large enough. The critical wavenumber $k^{*2} = \sqrt{\det J/(D_uD_v)}$.

## Comparison with Linear Equations

Unlike the linear heat equation (which simply smooths initial data without creating new spatial structures), reaction-diffusion equations can:
- **Support traveling fronts** with specific speeds determined by the nonlinear dynamics.
- **Generate spatial patterns** from spatially uniform initial conditions via Turing instability.
- **Exhibit metastability**: near-equilibrium states that are locally stable but can transition over long time scales.

The central theme is that **spatial structure emerges from the interplay of local nonlinear dynamics (reaction) and spatial coupling (diffusion)**, not from any externally imposed spatial heterogeneity.
