# Entropy Conditions

The Rankine-Hugoniot condition tells us how fast a shock must move, but it does not tell us which weak solutions are physically correct. For a given Riemann problem, there may be multiple weak solutions satisfying the RH condition — some physical, some not. The entropy condition is a selection criterion that picks out the unique admissible weak solution.

The name "entropy condition" comes from gas dynamics: the physically correct shock must satisfy the second law of thermodynamics, which requires entropy to increase across the shock (in the direction of flow). For scalar conservation laws, analogous conditions can be stated purely in terms of the characteristics, without reference to thermodynamics.

## Lax's Entropy Condition

For the scalar conservation law $u_t + f(u)_x = 0$ with strictly convex flux ($f'' > 0$), the **Lax entropy condition** for a shock at $x = s(t)$ with left state $u^-$ and right state $u^+$ requires:

$$f'(u^-) > \dot{s} > f'(u^+). \tag{1}$$

In words: the characteristic speed on the left exceeds the shock speed, and the shock speed exceeds the characteristic speed on the right. Geometrically, characteristics run into the shock from both sides — the shock is "compressive." Information flows into the shock and is absorbed.

An alternative form: since $f$ is convex and $f'$ is increasing, condition (1) is equivalent to $u^- > u^+$. Shocks for convex conservation laws have the faster-moving (larger $u$) state on the left.

**Why this is the right condition:** A shock violating (1) — where characteristics run away from the shock on both sides — would be "expansive." It would generate information rather than absorb it. Such a shock can always be replaced by a smoother rarefaction wave, which is more physical (it arises from taking the limit of viscous approximations).

## The Oleinik Entropy Condition

For a general (not necessarily convex) flux $f$, the Lax condition must be modified. The **Oleinik entropy condition** (or Oleinik's condition E) states that a shock from $u^-$ to $u^+$ is admissible if and only if

$$\frac{f(u) - f(u^-)}{u - u^-} \geq \dot{s} \geq \frac{f(u) - f(u^+)}{u - u^+}$$

for all $u$ between $u^-$ and $u^+$. Equivalently, the chord connecting $(u^-, f(u^-))$ and $(u^+, f(u^+))$ on the graph of $f$ lies above (or below, depending on the direction of the shock) the graph of $f$ between those two points.

Geometrically, for a shock from $u_L$ to $u_R$ with $u_L > u_R$ (a genuine compressive shock for convex $f$), the condition requires the chord from $(u_R, f(u_R))$ to $(u_L, f(u_L))$ to lie below the graph of $f$ — which is exactly convexity of $f$, reducing to the Lax condition when $f$ is convex.

## The Viscous Limit and Vanishing Viscosity

The most physically motivated selection principle is the **vanishing viscosity criterion**: the physically correct weak solution is the limit as $\varepsilon \to 0^+$ of the smooth solutions $u^\varepsilon$ of the viscous equation

$$u_t + f(u)_x = \varepsilon u_{xx}.$$

The viscous equation always has smooth solutions (the diffusion term prevents shock formation), and as $\varepsilon \to 0$ these solutions converge to a well-defined limit — the entropy solution.

**Why viscosity selects the correct shock:** For Burgers' equation, the viscous shock profile (a smooth transition from $u^-$ to $u^+$) exists only if $u^- > u^+$. This is exactly the Lax condition. When $u^- < u^+$, no monotone viscous shock profile exists, and the vanishing viscosity limit gives a rarefaction wave instead.

**Theorem (Kruzkov, 1970).** For a scalar conservation law with locally Lipschitz $f$ and $L^\infty$ initial data, there exists a unique weak solution satisfying the entropy condition (in Kruzkov's formulation):

$$|u - k|_t + [\text{sgn}(u-k)(f(u)-f(k))]_x \leq 0 \quad \text{in the sense of distributions, for all constants }k.$$

This is the Kruzkov entropy condition. It is equivalent to the Oleinik condition for smooth $f$ and encompasses all the others as special cases.

## Entropy-Entropy Flux Pairs

A more algebraic formulation uses the concept of an entropy-entropy flux pair $(\eta, q)$: a pair of functions where $\eta$ is convex and $q'(u) = \eta'(u)f'(u)$ (so that $q$ is the natural "entropy flux" associated with $\eta$). A weak solution is an **entropy solution** if it satisfies

$$\eta(u)_t + q(u)_x \leq 0 \quad \text{in the sense of distributions,}$$

for every convex entropy $\eta$.

In gas dynamics, $\eta = -\rho s$ (negative of the physical entropy density) and $q = -\rho sv$ are the entropy-entropy flux pair, and the condition $\eta(u)_t + q(u)_x \leq 0$ is exactly the second law of thermodynamics (entropy can only increase following a fluid particle). This connects the mathematical entropy condition to the physical one.

## The Riemann Problem: Complete Solution

For Burgers' equation $u_t + (u^2/2)_x = 0$ with Riemann data $u_L$ on the left and $u_R$ on the right, the entropy solution is:

**Case 1: $u_L > u_R$ (shock).** The Lax condition (1) is satisfied with $\dot{s} = (u_L+u_R)/2$. The solution is a traveling shock:

$$u(x,t) = \begin{cases} u_L & x < \frac{u_L+u_R}{2}t \\ u_R & x > \frac{u_L+u_R}{2}t \end{cases}.$$

**Case 2: $u_L < u_R$ (rarefaction).** A shock would violate the Lax condition (the shock would be expansive). The vanishing viscosity limit gives the rarefaction fan:

$$u(x,t) = \begin{cases} u_L & x < u_L t \\ x/t & u_L t \leq x \leq u_R t \\ u_R & x > u_R t \end{cases}.$$

This is the complete, entropy-admissible solution for all Riemann problems for Burgers' equation.

## Long-Time Behavior and N-Waves

For integrable initial data $\phi \in L^1(\mathbb{R})$ with $\int \phi\,dx = 0$, the entropy solution of Burgers' equation converges as $t\to\infty$ to an N-wave:

$$u(x,t) \approx \frac{x/t}{\sqrt{1 + t/t^*}} \cdot \mathbf{1}_{x_-(t) < x < x_+(t)},$$

where $x_\pm(t)$ are the positions of the trailing and leading shocks. This self-similar profile is the attractor for a broad class of initial data and illustrates how entropy solutions — even starting from smooth data — develop shock structures that persist and dominate at large times.
