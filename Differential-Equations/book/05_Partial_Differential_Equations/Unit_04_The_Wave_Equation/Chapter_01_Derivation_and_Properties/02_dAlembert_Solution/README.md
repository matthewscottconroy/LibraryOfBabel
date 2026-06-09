# D'Alembert's Solution to the Wave Equation

D'Alembert's solution, published in 1747, was the first explicit formula for the solution of a PDE. It gives the complete solution to the Cauchy problem for the one-dimensional wave equation in a form that is both computationally explicit and geometrically transparent: the solution is a superposition of a right-traveling wave and a left-traveling wave, each propagating at speed $c$ without changing shape.

## The Cauchy Problem

We seek $u(x,t)$ satisfying:

$$u_{tt} = c^2 u_{xx}, \qquad x \in \mathbb{R},\; t > 0, \tag{1}$$
$$u(x,0) = \phi(x), \qquad u_t(x,0) = \psi(x). \tag{2}$$

## Derivation via Characteristic Coordinates

Introduce the characteristic coordinates:

$$\xi = x + ct, \qquad \eta = x - ct.$$

The inverse transformation is $x = (\xi+\eta)/2$, $t = (\xi-\eta)/(2c)$. Under this change of variables:

$$u_{tt} - c^2 u_{xx} = -4c^2 u_{\xi\eta}.$$

So the wave equation $u_{tt} = c^2 u_{xx}$ becomes $u_{\xi\eta} = 0$.

The general solution of $u_{\xi\eta} = 0$ is $u = F(\xi) + G(\eta)$ for arbitrary $C^2$ functions $F$ and $G$:

$$u(x,t) = F(x+ct) + G(x-ct). \tag{3}$$

**Applying the initial conditions:**

At $t=0$: $u(x,0) = F(x) + G(x) = \phi(x)$.

$u_t(x,0) = cF'(x) - cG'(x) = \psi(x)$, so $F'(x) - G'(x) = \psi(x)/c$.

Integrating: $F(x) - G(x) = \frac{1}{c}\int_0^x\psi(s)\,ds + K$ for some constant $K$.

Solving for $F$ and $G$:

$$F(x) = \frac{\phi(x)}{2} + \frac{1}{2c}\int_0^x\psi(s)\,ds + \frac{K}{2},$$
$$G(x) = \frac{\phi(x)}{2} - \frac{1}{2c}\int_0^x\psi(s)\,ds - \frac{K}{2}.$$

The constant $K$ cancels in $u = F(x+ct) + G(x-ct)$:

$$\boxed{u(x,t) = \frac{\phi(x+ct) + \phi(x-ct)}{2} + \frac{1}{2c}\int_{x-ct}^{x+ct}\psi(s)\,ds.} \tag{4}$$

This is **d'Alembert's formula**.

## Physical Interpretation

The first term $[\phi(x+ct) + \phi(x-ct)]/2$ is the average of the initial displacement profile evaluated at the two characteristic endpoints $x\pm ct$. The second term $\frac{1}{2c}\int_{x-ct}^{x+ct}\psi(s)\,ds$ is the time-integral contribution of the initial velocity.

**Wave splitting:** The initial displacement $\phi(x)$ splits into two equal halves: one half ($\phi(x-ct)/2$) travels right at speed $c$, the other half ($\phi(x+ct)/2$) travels left. If $\phi$ is a localized bump at $x=0$, at time $t > 0$ there are two bumps of half the original amplitude, one at $x = -ct$ and one at $x = +ct$, moving away from each other.

## Examples

**Example 1: Plucked string.** Initial displacement $\phi(x)$ is a triangular tent function, initial velocity $\psi = 0$.

The solution $u(x,t) = [\phi(x+ct)+\phi(x-ct)]/2$ is the average of two copies of the tent, one moved $ct$ to the right and one $ct$ to the left. The tent splits into two half-height tents that travel in opposite directions.

**Example 2: Struck string (hammer).** Initial displacement $\phi = 0$, initial velocity $\psi(x) = \mathbf{1}_{[a,b]}(x)$ (uniform on $[a,b]$, zero elsewhere).

The solution $u(x,t) = \frac{1}{2c}\int_{x-ct}^{x+ct}\mathbf{1}_{[a,b]}(s)\,ds = \frac{1}{2c}|[x-ct,x+ct]\cap[a,b]|$ is the length of the intersection of $[x-ct,x+ct]$ with $[a,b]$, divided by $2c$. The string rises linearly in the "excited" region and is zero outside. The region of nonzero displacement expands at speed $c$ in both directions.

**Example 3: Sinusoidal wave.** Take $\phi(x) = A\sin(kx)$ and $\psi(x) = cAk\cos(kx)$ (corresponding to a single right-traveling sinusoidal wave).

$$u(x,t) = \frac{A[\sin(k(x+ct))+\sin(k(x-ct))]}{2} + \frac{Ak}{2}\int_{x-ct}^{x+ct}\cos(ks)\,ds.$$

The second term: $\frac{Ak}{2}\cdot\frac{[\sin(k(x+ct))-\sin(k(x-ct))]}{k} = \frac{A}{2}[\sin(k(x+ct))-\sin(k(x-ct))]$.

Total: $u = \frac{A}{2}[2\sin(k(x+ct))] = A\sin(k(x+ct))$ — a purely right-traveling wave, as expected.

## Regularity

D'Alembert's formula shows that the solution has exactly the same smoothness as the initial data: if $\phi \in C^k$ and $\psi \in C^{k-1}$, then $u \in C^k$ (for $k \geq 2$). The wave equation does not smooth its initial data (unlike the heat equation) — initial discontinuities propagate along characteristics and persist for all time.

## Uniqueness

The formula (4) gives an explicit solution; to show it is the only $C^2$ solution, suppose $v$ is another solution. Then $w = u - v$ satisfies the wave equation with zero initial data. Energy conservation gives $E(t) = E(0) = 0$, so $w_t = w_x = 0$ everywhere, hence $w$ is constant, and $w(x,0) = 0$ gives $w \equiv 0$.

## Well-Posedness

D'Alembert's formula shows well-posedness of the Cauchy problem for the wave equation:

1. **Existence:** formula (4) gives an explicit solution.
2. **Uniqueness:** proved by energy conservation.
3. **Continuous dependence:** If $|\phi_1 - \phi_2| < \varepsilon$ and $|\psi_1 - \psi_2| < \varepsilon$ pointwise, then
   $$|u_1(x,t) - u_2(x,t)| \leq \varepsilon + \frac{1}{2c}|\int_{x-ct}^{x+ct}\varepsilon\,ds| = \varepsilon + \varepsilon t.$$
   The error grows at most linearly with time — the solution depends continuously on the data.
