# 11.6 Multifractal Analysis

So far we have assigned a single number — the Hausdorff dimension — to each attractor. But for a chaotic attractor with an invariant measure, the dimension only tells us about the support of the measure. It tells us nothing about how the measure is *distributed* across that support.

Here is the subtlety: the invariant measure on a chaotic attractor is almost never uniform. Some parts of the attractor are visited more often than others. The measure is concentrated in complicated patterns that themselves have fractal structure. Multifractal analysis is the tool that quantifies this structure — not with a single number, but with a whole spectrum.

## Local Dimensions and the $f(\alpha)$ Spectrum

The right way to measure how the invariant measure is distributed is through *local dimensions*: for each point $x$ in the attractor, how does $\mu(B(x,\varepsilon))$ scale as $\varepsilon \to 0$?

**Definition 11.6.1.** For a measure $\mu$ on an attractor, the *local dimension* at $x$ is:
$$\alpha(x) = \lim_{\varepsilon \to 0} \frac{\log \mu(B(x,\varepsilon))}{\log \varepsilon}.$$

If $\alpha(x) = \alpha$ for $\mu$-a.e. $x$, we say $\mu$ is *exact-dimensional* with dimension $\alpha$. This is the case for most ergodic invariant measures.

But for SRB measures on genuinely chaotic attractors, the local dimension $\alpha(x)$ typically varies as $x$ varies. The set of points with local dimension exactly $\alpha$ is itself a fractal set. The *multifractal spectrum* $f(\alpha)$ tells us the Hausdorff dimension of that set:

$$f(\alpha) = \dim_H\{x : \alpha(x) = \alpha\}.$$

This function $f(\alpha)$ is the complete picture: it tells you, for each possible value of the local scaling exponent, how "big" the set of points with that exponent is. A single Hausdorff dimension is just the maximum value of $f(\alpha)$.

**Theorem 11.6.2 (Ruelle, Pesin, Eckmann-Procaccia).** For hyperbolic attractors, $f(\alpha)$ is a concave function of $\alpha$, with maximum $f(\alpha_{\text{typ}}) = \dim_H(\text{attractor})$, achieved at the typical dimension $\alpha_{\text{typ}}$.

What this is saying is: the multifractal spectrum is a concave arch shape. The peak is the Hausdorff dimension of the attractor itself, achieved at the "typical" value of the local dimension — the value that $\mu$-a.e. point has. Points with atypical local dimensions (too high or too low) form smaller fractal sets.

For the Bernoulli measure $\mu_p = (p, 1-p)$ on $\{0,1\}^{\mathbb{N}}$ (which arises from the doubling map), the multifractal spectrum is computable in closed form — see Exercise 11.5.

## The Legendre Transform and Rényi Dimensions

The multifractal spectrum is most cleanly stated through its relationship to the *Rényi dimension spectrum* $D_q$. The Rényi dimensions form a one-parameter family $(q \in \mathbb{R})$ that interpolate between box-counting dimension ($q = 0$), information dimension ($q = 1$), and correlation dimension ($q = 2$).

**Legendre Transform:** $f(\alpha)$ is related to the Rényi dimension spectrum $D_q$ via the Legendre transform:
$$D_q = \frac{1}{q-1} \inf_\alpha \left[q\alpha - f(\alpha) + 1\right].$$
Conversely:
$$f(\alpha) = \inf_q \left[q\alpha - (q-1)D_q + 1\right].$$

This is the thermodynamic formalism at work. The Legendre transform relationship means that $f(\alpha)$ and $D_q$ carry exactly the same information — they are two ways of encoding the same function, one in "position space" (values of the local dimension) and one in "momentum space" (the order $q$). This is precisely analogous to the Legendre transform relating free energy and entropy in statistical mechanics.

This connects multifractal analysis directly to Rényi entropies. We will develop this connection in Chapter 17, where the Rényi entropy spectrum $H_q$ for a stationary process plays the same role for information theory that the Rényi dimension spectrum plays here.

The message of multifractal analysis is: a chaotic attractor is not a monolith. It is a stratified object, with different "layers" of scaling behavior, and the full description requires the whole spectrum $f(\alpha)$ — a function, not a number. In the next section, we will see how this spectrum connects to the most fundamental information-theoretic quantity: entropy production.
