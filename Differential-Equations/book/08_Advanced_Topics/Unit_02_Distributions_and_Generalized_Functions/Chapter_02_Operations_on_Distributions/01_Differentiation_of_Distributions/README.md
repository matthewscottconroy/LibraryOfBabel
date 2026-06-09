# Differentiation of Distributions

Every distribution can be differentiated any number of times, producing another distribution. This unlimited differentiability is one of the most powerful features of distribution theory and contrasts sharply with the classical setting, where a function must already be differentiable before its derivative makes sense. In distribution theory, the derivative of a distribution is always defined, is always a distribution, and agrees with the classical derivative whenever the latter exists.

## The Definition

**Definition.** The **distributional derivative** (or **weak derivative**) of a distribution $T \in \mathcal{D}'(\mathbb{R}^n)$ with respect to $x_i$ is the distribution $\partial_i T$ defined by:

$$\langle \partial_i T, \phi \rangle = -\langle T, \partial_i \phi \rangle, \quad \phi \in \mathcal{D}(\mathbb{R}^n).$$

More generally, for a multi-index $\alpha = (\alpha_1, \ldots, \alpha_n)$:

$$\langle D^\alpha T, \phi \rangle = (-1)^{|\alpha|} \langle T, D^\alpha \phi \rangle.$$

**Consistency with classical differentiation.** If $f \in C^1(\mathbb{R}^n)$, then for the regular distribution $T_f$:

$$\langle \partial_i T_f, \phi \rangle = -\langle T_f, \partial_i\phi \rangle = -\int f \partial_i\phi \, dx = \int (\partial_i f) \phi \, dx = \langle T_{\partial_i f}, \phi \rangle.$$

(Integration by parts; boundary terms vanish since $\phi \in \mathcal{D}$.) So the distributional derivative of a $C^1$ function agrees with its classical derivative. $\square$

**Well-definedness.** The map $\phi \mapsto -\langle T, \partial_i\phi \rangle$ is linear. Continuity: if $\phi_j \to 0$ in $\mathcal{D}$, then $\partial_i\phi_j \to 0$ in $\mathcal{D}$ (differentiation is continuous in the test function topology), so $\langle T, \partial_i\phi_j \rangle \to 0$ by the continuity of $T$. Thus $\partial_i T \in \mathcal{D}'$.

## Key Properties

**Linearity.** $D^\alpha(aS + bT) = aD^\alpha S + bD^\alpha T$.

**Commutativity of mixed partials.** $\partial_i\partial_j T = \partial_j\partial_i T$ for all $T \in \mathcal{D}'$. Proof: $\langle \partial_i\partial_j T, \phi \rangle = -\langle \partial_j T, \partial_i\phi \rangle = \langle T, \partial_j\partial_i\phi \rangle = \langle T, \partial_i\partial_j\phi \rangle = \langle \partial_j\partial_i T, \phi \rangle$, using the classical equality of mixed partials for test functions. $\square$

**Continuity.** The map $D^\alpha: \mathcal{D}' \to \mathcal{D}'$ is continuous in the weak-* topology: if $T_j \to T$, then $D^\alpha T_j \to D^\alpha T$.

**Infinite differentiability.** Every distribution has distributional derivatives of all orders. This is in stark contrast to classical analysis, where a function need not be differentiable at all.

## Examples

**Heaviside function.** $H(x) = \mathbf{1}_{[0,\infty)}$. $\langle H', \phi \rangle = -\langle H, \phi' \rangle = -\int_0^\infty \phi'(x) \, dx = \phi(0) = \langle \delta, \phi \rangle$. So $H' = \delta$.

**$H'' = \delta'$.** $\langle \delta', \phi \rangle = -\langle \delta, \phi' \rangle = -\phi'(0)$.

**$|x|$.** $|x|' = \text{sgn}(x) = 2H(x) - 1$. Then $|x|'' = 2\delta$ (since $(\text{sgn}(x))' = 2\delta$).

**$\log|x|$.** $\langle (\log|x|)', \phi \rangle = -\int \log|x| \phi'(x) \, dx = \int \frac{\phi(x)}{x} \, dx$ (integration by parts, using the improper integral carefully) $= \langle \text{p.v.}(1/x), \phi \rangle$. So $(\log|x|)' = \text{p.v.}(1/x)$.

**$1/|\mathbf{x}|$ in $\mathbb{R}^3$.** In dimension 3, $\Delta(1/|\mathbf{x}|) = -4\pi\delta$ distributionally. Proof: for any $\phi \in \mathcal{D}(\mathbb{R}^3)$, $\langle \Delta(1/|\mathbf{x}|), \phi \rangle = \langle 1/|\mathbf{x}|, \Delta\phi \rangle = \int_{\mathbb{R}^3} \frac{\Delta\phi}{|\mathbf{x}|} \, d^3x$. Split the integral into $|\mathbf{x}| < \varepsilon$ and $|\mathbf{x}| > \varepsilon$, integrate by parts on each region using Green's second identity, and take $\varepsilon \to 0$. The result is $-4\pi\phi(0) = -4\pi\langle\delta, \phi\rangle$.

## Distributional Derivatives and Jump Discontinuities

For a function $f$ that is $C^1$ on $\mathbb{R} \setminus \{a\}$ but has a jump discontinuity at $x = a$:

$$f_{\text{dist}}' = (f')_{\text{classical}} + [f]_a \delta_a,$$

where $[f]_a = f(a^+) - f(a^-) = \lim_{x \to a^+} f(x) - \lim_{x \to a^-} f(x)$ is the jump.

**Proof.** For $\phi \in \mathcal{D}$:

$$\langle f_{\text{dist}}', \phi \rangle = -\int_{-\infty}^\infty f\phi' \, dx = -\int_{-\infty}^a f\phi' - \int_a^\infty f\phi' \, dx$$
$$= \int_{-\infty}^a f'\phi \, dx - [f\phi]_{-\infty}^a + \int_a^\infty f'\phi \, dx - [f\phi]_a^\infty$$
$$= \int_{\mathbb{R}} (f')_{\text{classical}} \phi \, dx + f(a^-)\phi(a) - f(a^+)\phi(a) = \langle (f')_{\text{cl}}, \phi\rangle - [f]_a\phi(a).$$

Rearranging: $f_{\text{dist}}' = (f')_{\text{cl}} + [f]_a\delta_a$. (Note the sign: if $f$ jumps up at $a$, i.e., $[f]_a > 0$, there is a positive delta contribution.) $\square$

**Example.** If $f(x) = \sin(x)H(x)$ (sine for $x > 0$, zero for $x < 0$), then $[f]_0 = \sin(0) - 0 = 0$ and $(f')_{\text{cl}} = \cos(x)H(x)$. So $f' = \cos(x)H(x)$ distributionally (no delta term, since $f$ is continuous).

## Connection to Weak Derivatives in Sobolev Spaces

The distributional derivative is closely related to the **weak derivative** used in Sobolev space theory (Unit 3). A function $f \in L^1_{\text{loc}}$ has a weak derivative $g = D^\alpha f$ if $g \in L^1_{\text{loc}}$ and $\langle g, \phi \rangle = (-1)^{|\alpha|}\langle f, D^\alpha\phi \rangle$ for all $\phi \in \mathcal{D}$. This is the distributional derivative being a regular distribution. Sobolev spaces $W^{k,p}$ consist of functions all of whose weak derivatives up to order $k$ are in $L^p$.

The distributional framework is strictly more general: the distributional derivative always exists but may not be a regular distribution (e.g., the distributional derivative of the Heaviside function is $\delta$, which is not in $L^p$). Sobolev spaces single out the case where the weak derivative is representable by an $L^p$ function.
