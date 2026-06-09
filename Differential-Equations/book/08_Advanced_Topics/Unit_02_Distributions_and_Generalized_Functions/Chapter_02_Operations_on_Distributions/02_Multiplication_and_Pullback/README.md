# Multiplication and Pullback of Distributions

Not all operations extend from functions to distributions without restriction. Multiplication of two arbitrary distributions cannot be defined consistently, but multiplication of a distribution by a smooth function is always valid. Pullback (change of variables) works under appropriate conditions. Understanding both what is possible and what is not—and why—is essential for applying distribution theory to nonlinear problems.

## Multiplication by a Smooth Function

If $T \in \mathcal{D}'(\mathbb{R}^n)$ and $f \in C^\infty(\mathbb{R}^n)$, define the product $fT$ by:

$$\langle fT, \phi \rangle = \langle T, f\phi \rangle, \quad \phi \in \mathcal{D}.$$

**Well-definedness.** Since $f \in C^\infty$ and $\phi \in \mathcal{D}$, the product $f\phi \in \mathcal{D}$ (smooth and compactly supported). The map $\phi \mapsto f\phi$ is continuous in the $\mathcal{D}$ topology (all derivatives of $f\phi$ can be bounded in terms of derivatives of $\phi$), so $fT$ is indeed a distribution.

**Consistency.** If $T = T_g$ for $g \in L^1_{\text{loc}}$, then $\langle fT_g, \phi \rangle = \langle T_g, f\phi \rangle = \int g \cdot f\phi \, dx = \langle T_{fg}, \phi \rangle$. So $f \cdot T_g = T_{fg}$.

**The Leibniz rule.** For $f \in C^\infty$ and $T \in \mathcal{D}'$:

$$\partial_i(fT) = (\partial_i f)T + f(\partial_i T).$$

More generally, for multi-indices: $D^\alpha(fT) = \sum_{\beta \leq \alpha} \binom{\alpha}{\beta} D^\beta f \cdot D^{\alpha-\beta} T$ (where $\binom{\alpha}{\beta} = \prod_i \binom{\alpha_i}{\beta_i}$).

**Examples.**
- $x \cdot \delta = 0$. Proof: $\langle x\delta, \phi \rangle = \langle \delta, x\phi \rangle = (x\phi)|_{x=0} = 0$.
- $x \cdot \delta' = -\delta$. Proof: $\langle x\delta', \phi \rangle = \langle \delta', x\phi \rangle = -(x\phi)'|_{x=0} = -(\phi(0) + 0 \cdot \phi'(0)) = -\phi(0)$.
- $x^2 \cdot \text{p.v.}(1/x) = x$ (as a distribution, since $x^2/x = x$ away from $0$ and $x$ is locally integrable).

## The Impossibility of Multiplying Two Distributions

One cannot define a bilinear product on $\mathcal{D}'$ that simultaneously extends the classical product of smooth functions and satisfies the Leibniz rule. This is the **Schwartz impossibility theorem**:

**Theorem (Schwartz, 1954).** There is no associative bilinear map $\mathcal{D}'(\mathbb{R}) \times \mathcal{D}'(\mathbb{R}) \to \mathcal{D}'(\mathbb{R})$ that:
1. Extends the pointwise product of smooth functions.
2. Satisfies $\partial_x(ST) = (\partial_x S)T + S(\partial_x T)$ for all $S, T \in \mathcal{D}'$.
3. Satisfies $1 \cdot T = T$ for all $T$.

The proof exhibits a contradiction using $H^2 = H$ (since $H$ takes values in $\{0,1\}$ pointwise) and differentiating twice. If such a product existed: $2H \cdot H' = (H^2)' = 2H \cdot H'$, consistent; but $H \cdot \delta = H \cdot H' = \frac{1}{2}\delta$ by one route (using $H \cdot H = H$ and differentiating), yet another route using $\delta = H'$ gives $H \cdot H' = (H')^2 \stackrel{?}{=} \delta^2$, which is undefined.

This impossibility is not a deficiency of the theory but a mathematical fact: the product of two distributions is genuinely ambiguous in general. For specific pairs of distributions (e.g., those with "compatible singular supports"), products can be defined, and this is the basis of the theory of microlocal analysis.

## Pullback Under Smooth Maps

For a smooth map $F: U \to V$ between open sets and a distribution $T \in \mathcal{D}'(V)$, the **pullback** $F^*T = T \circ F$ is a distribution on $U$. The definition is motivated by the change of variables formula for integrals:

$$\int_V f(y)\phi(y) \, dy = \int_U f(F(x))\phi(F(x))|\det DF(x)| \, dx.$$

For a smooth submersion $F$ (surjective $DF$ everywhere), define:

$$\langle F^*T, \phi \rangle = \langle T, F_*\phi \rangle,$$

where $F_*\phi$ is the pushforward of $\phi$ defined by integration along fibers of $F$. For diffeomorphisms, $F_*\phi(y) = \sum_{x: F(x)=y} \phi(x)/|\det DF(x)|$ (a sum with one term), giving:

$$\langle F^*T, \phi \rangle = \langle T, (\phi \circ F^{-1}) \cdot |\det D(F^{-1})| \rangle.$$

**Examples.**
- Scaling: $(S_a T)(x) = T(ax)$ for $a \neq 0$: $\langle S_a T, \phi \rangle = \langle T, \frac{1}{|a|}\phi(x/a) \rangle$. In particular, $\delta(ax) = \delta(x)/|a|$.
- Translation: $\tau_a T(x) = T(x-a)$: $\langle \tau_a T, \phi \rangle = \langle T, \phi(x+a) \rangle$.
- Composition with smooth functions: $\delta(f(x)) = \sum_{f(x_i)=0} \delta(x-x_i)/|f'(x_i)|$ when $f$ has only simple zeros $x_i$.

## Supports of Distributions

The **support** of a distribution $T$ is the complement of the largest open set on which $T = 0$:

$$\text{supp}(T) = \mathbb{R}^n \setminus \bigcup\{U \text{ open} : T|_U = 0\}.$$

Properties:
- $\text{supp}(\delta_a) = \{a\}$.
- $\text{supp}(D^\alpha\delta_a) = \{a\}$.
- $\text{supp}(fT) \subset \text{supp}(f) \cap \text{supp}(T)$.
- $\text{supp}(D^\alpha T) \subset \text{supp}(T)$.

Distributions with compact support form a subspace $\mathcal{E}'(\mathbb{R}^n)$ (the dual of $C^\infty(\mathbb{R}^n)$, smooth functions without compact support restriction). The inclusion is $\mathcal{E}' \subset \mathcal{D}'$.

## Convolution

For $T \in \mathcal{D}'$ and $\phi \in \mathcal{D}$, the **convolution** $T * \phi$ is a smooth function:

$$(T * \phi)(x) = \langle T, \phi(x - \cdot) \rangle = \langle T_y, \phi(x - y) \rangle.$$

This is smooth in $x$ and satisfies $D^\alpha(T*\phi) = T * (D^\alpha\phi) = (D^\alpha T)*\phi$.

For two distributions $S, T$ with at least one of them compactly supported:

$$\langle S * T, \phi \rangle = \langle S_x, \langle T_y, \phi(x+y) \rangle \rangle,$$

and convolution is commutative, associative, and compatible with differentiation: $D^\alpha(S*T) = (D^\alpha S)*T = S*(D^\alpha T)$.
