# 17.6 Tsallis Entropy

Shannon entropy is *extensive*: for independent systems $X$ and $Y$, $H(X, Y) = H(X) + H(Y)$. This is what you would hope for — the uncertainty of two independent systems is the sum of their uncertainties.

But not all systems of interest are independent in this additive sense. Long-range interacting systems — systems where distant parts are strongly correlated — do not scale additively. Their total "entropy" grows differently from the sum of the parts. The *Tsallis entropy*, introduced by Constantino Tsallis in 1988, is a generalization designed for such systems.

**Definition 17.6.1 (Tsallis Entropy).** The *Tsallis entropy of order $q$* is:
$$S_q(X) = \frac{1}{q-1}\left(1 - \sum_i p_i^q\right).$$

In the quantum setting, this reads $S_q = (1 - \text{Tr}[\rho^q])/(q-1)$.

As $q \to 1$: $S_q \to H$ (Shannon entropy). At $q = 2$: $S_2 = 1 - \sum p_i^2$ (linear entropy, the most tractable special case).

The defining property of Tsallis entropy is its *non-extensivity*:
$$S_q(X, Y) = S_q(X) + S_q(Y) + (1-q) S_q(X) S_q(Y) \quad \text{for independent } X, Y.$$

At $q = 1$, the third term vanishes and you recover additivity. For $q \neq 1$, the cross-term $(1-q) S_q(X) S_q(Y)$ measures the "non-extensive" interaction between the two systems' entropies.

The maximum entropy distribution for Tsallis entropy with an energy constraint is a *$q$-exponential* — a power law — rather than the Boltzmann exponential:
$$p^*(x) \propto [1 - (1-q)\lambda H(x)]^{1/(1-q)}.$$
This is a generalization of the exponential distribution that interpolates between Gaussian-like behavior (for $q < 1$) and power-law behavior (for $q > 1$). Power-law distributions arise throughout natural phenomena — earthquake magnitudes, word frequencies, wealth distributions — and Tsallis entropy provides a framework for understanding them as maximum entropy distributions under nonstandard constraints.

**Application:** Tsallis entropy appears in non-equilibrium statistical mechanics (systems that have not relaxed to Boltzmann equilibrium), long-range interacting systems (gravitation, self-gravitating gases), and multi-fractal distributions. It is particularly useful when the system has a scale-free structure, since power-law distributions emerge naturally from Tsallis MaxEnt.

One should be honest about the limitations: the physical interpretation of Tsallis entropy is debated, and it is not always clear when non-extensivity is a genuine physical property of the system versus an artifact of the modeling. The framework is productive, but applying it requires care.

What Tsallis entropy makes clear is that Shannon entropy's additivity is a choice, not a necessity. By varying the non-extensivity parameter $q$, you can access a family of thermodynamic descriptions suited to different classes of systems. The Rényi and Tsallis families are closely related: $H_\alpha = \frac{1}{1-\alpha}\log(1-(1-\alpha)S_{1-\alpha})$, so both families parametrize the same set of distributions, just in different coordinate systems.
