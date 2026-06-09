# The Lorenz System

In 1963, meteorologist Edward Lorenz was attempting to model atmospheric convection using a computer to simulate a system of twelve nonlinear ODEs. When he restarted a simulation from the middle, entering a printout value of $0.506$ instead of the stored value $0.506127$, the resulting trajectory diverged dramatically from the original within a simulated two months. This observation—that tiny changes in initial conditions produce radically different long-term behavior—led Lorenz to formulate a dramatically simplified three-dimensional model that now bears his name and stands as the paradigm of deterministic chaos.

## The Equations

The Lorenz system is:

$$\dot{x} = \sigma(y - x),$$
$$\dot{y} = rx - y - xz,$$
$$\dot{z} = xy - bz,$$

where $\sigma > 0$ (the Prandtl number), $r > 0$ (the Rayleigh number ratio), and $b > 0$ are parameters. In Lorenz's original derivation from the Boussinesq equations, $x$ represents the amplitude of convective motion, $y$ the temperature difference between ascending and descending fluid, and $z$ the distortion of the temperature profile from linearity.

The classical parameters are $\sigma = 10$, $b = 8/3$, $r = 28$. At these values, the system exhibits chaotic behavior, and almost all initial conditions converge to the Lorenz attractor.

## Dissipative Structure

The divergence of the vector field is:

$$\text{div}\, F = \frac{\partial}{\partial x}[\sigma(y-x)] + \frac{\partial}{\partial y}[rx - y - xz] + \frac{\partial}{\partial z}[xy - bz] = -\sigma - 1 - b.$$

For $\sigma = 10$, $b = 8/3$: $\text{div}\, F = -10 - 1 - 8/3 = -41/3 < 0$. Volumes in phase space contract at the constant rate $e^{-(\sigma+1+b)t}$. Any invariant set must therefore have zero volume, consistent with the fractal attractor.

## Equilibria

The equilibria of the Lorenz system satisfy $F(x^*) = 0$:

- **Origin $O = (0,0,0)$:** Always an equilibrium. The Jacobian at $O$ has eigenvalues $-b$ and $(-(\sigma+1) \pm \sqrt{(\sigma+1)^2 + 4\sigma(r-1)})/2$. For $r < 1$, all eigenvalues are negative and $O$ is stable. At $r = 1$, a pitchfork bifurcation creates two new equilibria.

- **Symmetric pair $C^\pm = (\pm\sqrt{b(r-1)}, \pm\sqrt{b(r-1)}, r-1)$:** Exist for $r > 1$. These represent steady convection rolls. They are stable for $1 < r < r_H$ where $r_H = \sigma(\sigma + b + 3)/(\sigma - b - 1)$. For $\sigma = 10$, $b = 8/3$: $r_H \approx 24.74$.

At $r = 28 > r_H$, all three equilibria are unstable, and no stable periodic orbits exist (at the classical parameters)—all orbits converge to the strange attractor.

## The Lorenz Attractor

For $r = 28$, $\sigma = 10$, $b = 8/3$, numerical simulation reveals that orbits wind around first one equilibrium $C^+$ and then the other $C^-$, switching irregularly between the two "wings" of the butterfly. The sequence of switches is unpredictable in the long term, even though the system is deterministic.

**Trapping Region.** The ellipsoid

$$\mathcal{E} = \left\{(x,y,z) : rx^2 + \sigma y^2 + \sigma(z - 2r)^2 \leq C\right\}$$

is positively invariant for sufficiently large $C$. Indeed, $\frac{d}{dt}(rx^2 + \sigma y^2 + \sigma(z-2r)^2)$ can be shown to be negative outside a compact set, so all orbits eventually enter $\mathcal{E}$.

The **Lorenz attractor** $A$ is defined as $A = \bigcap_{t \geq 0} \phi_t(\mathcal{E})$. It is compact, invariant, and attracts all orbits with positive measure initial conditions.

## The Geometric Lorenz Model

The actual Lorenz attractor is difficult to analyze rigorously because the fixed points $C^\pm$ are so close to the attractor. Guckenheimer and Williams (1979) introduced a simplified **geometric Lorenz model** that captures the essential features:

1. The Poincaré section is taken as a plane $\Sigma = \{z = r - 1\}$ near the fixed points.
2. The return map $P: \Sigma \to \Sigma$ is approximately one-dimensional: $P(x, y) \approx (f(x), g(x,y))$ where $f$ is approximately a tent map.
3. The one-dimensional map $f$ is expanding ($|f'| > 1$ everywhere), ensuring sensitive dependence on initial conditions.

The geometric model is analytically tractable and establishes rigorously that the Lorenz attractor is a genuine strange attractor. Tucker (2002) proved, using a computer-assisted proof with interval arithmetic, that the actual Lorenz system (not just the geometric model) has a robust strange attractor at the classical parameters. This resolved an open problem that had stood for nearly 40 years.

## Sensitive Dependence and the "Butterfly Effect"

The positive Lyapunov exponent $\lambda_1 \approx 0.906$ quantifies how fast nearby orbits diverge. Two initial conditions differing by $\delta_0$ will be separated by approximately $\delta_0 e^{0.906 t}$ for moderate times. For $\delta_0 = 10^{-6}$ (representing measurement precision), the separation reaches order 1 (full unpredictability) in time $t \approx 6/0.906 \approx 6.6$ dimensionless time units. In the meteorological application, this corresponds to a predictability horizon of a few days.

The term "butterfly effect" derives from Lorenz's metaphor: could the flap of a butterfly's wings in Brazil set off a tornado in Texas? The mathematical content is that exponential sensitivity makes long-term weather prediction fundamentally limited by the precision of initial measurements, regardless of the quality of the model.

## Symmetry of the Lorenz System

The Lorenz system has a discrete symmetry: it is equivariant under $(x, y, z) \mapsto (-x, -y, z)$. The transformation sends $C^+$ to $C^-$ and vice versa, and it maps orbits to orbits. This symmetry is responsible for the bilateral symmetry of the butterfly attractor. It also constrains bifurcation sequences: bifurcations that break this symmetry (symmetry-breaking bifurcations) are governed by a pitchfork normal form, while symmetry-preserving bifurcations are governed by a transcritical normal form.

## Generalizations and Connections

The Lorenz system belongs to a broader class of ODEs that model convection and related phenomena. Variants include the Chen system, the Rössler system (with a spiral attractor rather than a butterfly), and the Sprott systems (low-dimensional systems with the fewest parameters needed for chaos). Each provides a different geometric realization of the stretching-and-folding mechanism underlying chaos.

Lorenz-type attractors have been rigorously proved to exist in several families of three-dimensional ODEs, using the geometric model as a template. The study of these systems continues to drive advances in the theory of hyperbolic and non-uniformly hyperbolic dynamical systems.
