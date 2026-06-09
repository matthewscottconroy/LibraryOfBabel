# The Diffusion Interpretation of the Heat Equation

The heat equation $u_t = \kappa\Delta u$ does not merely model heat conduction. The same equation — with different physical interpretations for $u$ and different values of the diffusivity $\kappa$ — governs an astonishing range of phenomena: the spread of a chemical plume in a river, the migration of charge carriers in a semiconductor, the Brownian motion of a colloidal particle, and even the volatility of financial derivatives. Understanding the heat equation as a diffusion equation opens connections to probability theory and statistical physics that are as mathematically rich as the PDE theory itself.

## Fick's Laws of Diffusion

Adolf Fick (1855) proposed two laws governing the diffusion of a solute in a solution, in direct analogy with Fourier's laws.

**Fick's First Law:** The diffusion flux $\mathbf{J}$ (moles per unit area per unit time) of a species with concentration $c$ (moles per unit volume) is

$$\mathbf{J} = -D\nabla c,$$

where $D$ is the **diffusion coefficient** (m²/s). Like Fourier's law, this says that the flux is proportional to and in the direction of the negative concentration gradient — the species flows from regions of high concentration to low concentration.

**Fick's Second Law:** Conservation of mass for the diffusing species (no sources or sinks) gives

$$c_t = -\nabla\cdot\mathbf{J} = D\Delta c.$$

This is identical in form to the heat equation. Every solution method, every qualitative property, and every theorem about the heat equation applies directly to the diffusion equation.

## Brownian Motion and the Heat Kernel

The deepest connection between the heat equation and diffusion comes from probability theory. Consider a particle undergoing Brownian motion in $\mathbb{R}^n$: it moves randomly, with increments that are independent and Gaussian. The position $\mathbf{X}(t)$ of the particle starting at $\mathbf{X}(0) = \mathbf{x}_0$ is a stochastic process, and its probability density $p(\mathbf{x},t)$ — the probability of finding the particle near $\mathbf{x}$ at time $t$ — satisfies the diffusion equation:

$$p_t = D\Delta p, \qquad p(\mathbf{x},0) = \delta(\mathbf{x} - \mathbf{x}_0).$$

The fundamental solution of the heat equation (heat kernel) is:

$$K(\mathbf{x}-\mathbf{x}_0, t) = \frac{1}{(4\pi D t)^{n/2}}\exp\!\left(-\frac{|\mathbf{x}-\mathbf{x}_0|^2}{4Dt}\right).$$

This is the probability density of a Gaussian distribution with zero mean and variance $2Dt$ in each component. The fact that the variance grows as $t$ (not as $t^2$, as it would for ballistic motion) is the hallmark of diffusion — particles spread with the square root of time, not linearly.

The connection between Brownian motion and the heat equation is the content of the Feynman-Kac formula: the solution of $u_t = D\Delta u + V(\mathbf{x})u$ with initial data $u(\mathbf{x},0) = f(\mathbf{x})$ can be written as

$$u(\mathbf{x},t) = \mathbb{E}\!\left[f(\mathbf{X}(t))\exp\!\left(\int_0^t V(\mathbf{X}(s))\,ds\right) \;\Big|\; \mathbf{X}(0)=\mathbf{x}\right],$$

a remarkable formula expressing a PDE solution as a stochastic expectation.

## The Einstein Relation

Albert Einstein (1905), in one of his annus mirabilis papers, derived the diffusion coefficient of a spherical particle of radius $r$ in a fluid of viscosity $\eta$ at temperature $T$:

$$D = \frac{k_B T}{6\pi\eta r},$$

where $k_B$ is Boltzmann's constant. This **Einstein-Smoluchowski relation** connects microscopic thermal fluctuations to macroscopic diffusivity. It was one of the first quantitative confirmations of the existence of atoms and provided a method (used by Perrin experimentally in 1908) to measure Avogadro's number.

The formula also reveals the physical mechanism of diffusion: Brownian motion is driven by thermal fluctuations (proportional to $k_B T$) and resisted by viscous drag (proportional to $\eta r$). At higher temperature or smaller particle size, diffusion is faster.

## The Mean Squared Displacement

For a diffusing particle, the mean squared displacement grows linearly with time:

$$\mathbb{E}\bigl[|\mathbf{X}(t) - \mathbf{x}_0|^2\bigr] = 2nDt,$$

where $n$ is the spatial dimension. This is the defining characteristic of normal diffusion. Anomalous diffusion, where $\mathbb{E}[|\mathbf{X}|^2] \sim t^\alpha$ with $\alpha \neq 1$, is governed by fractional diffusion equations — nonlocal generalizations of the heat equation.

## Scaling and Self-Similarity

The heat equation (and diffusion equation) are invariant under the parabolic scaling $(x,t) \mapsto (\lambda x, \lambda^2 t)$: if $u(x,t)$ solves $u_t = \kappa u_{xx}$, so does $u(\lambda x, \lambda^2 t)$. This $2:1$ scaling (space:time) is why diffusion length scales as $\sqrt{t}$.

The heat kernel $K(x,t) = (4\pi\kappa t)^{-1/2}e^{-x^2/(4\kappa t)}$ is a self-similar solution: it satisfies $K(x,t) = \lambda^{-1}K(x/\lambda, t/\lambda^2)$ for all $\lambda > 0$. Written in the similarity variable $\xi = x/\sqrt{4\kappa t}$, the heat kernel is simply $K = (4\pi\kappa t)^{-1/2}e^{-\xi^2}$ — a Gaussian in the scaled variable.

This self-similar structure governs the large-time behavior of all solutions: for integrable initial data, $u(x,t) \to \left(\int \phi\,dx\right)K(x,t)$ as $t\to\infty$ (the solution converges to the heat kernel scaled by the "total mass" of the initial data). The spatial profile becomes Gaussian regardless of the initial shape — this is a version of the central limit theorem for diffusion.

## The Black-Scholes Equation

The Black-Scholes equation for the price $V(S,t)$ of a financial option with underlying asset price $S$ is

$$V_t + \frac{1}{2}\sigma^2 S^2 V_{SS} + rS V_S - rV = 0,$$

where $\sigma$ is the volatility and $r$ is the risk-free interest rate. Under the change of variables $S = e^x$, $t \mapsto T-t$ (time to expiry), and a further substitution $V = e^{ax+bt}v$, this reduces exactly to the heat equation $v_\tau = \frac{\sigma^2}{2}v_{xx}$. The Black-Scholes formula for a European call option is nothing other than the fundamental solution (heat kernel) of this equation, evaluated with the appropriate initial (terminal) condition. The profound financial insight that option pricing reduces to a diffusion problem is the mathematical expression of the idea that asset prices follow Brownian motion.
