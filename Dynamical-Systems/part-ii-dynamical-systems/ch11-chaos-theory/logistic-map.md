# 11.5 The Logistic Map: A Case Study

If you want to understand chaos and you are only allowed to study one example in complete detail, study the logistic map. It is simple enough to be fully tractable, rich enough to exhibit every phenomenon we care about, and historically central to the development of chaos theory. The bifurcation diagram for the logistic family is one of the most famous pictures in all of mathematics.

The family is $f_\mu: [0,1] \to [0,1]$, $f_\mu(x) = \mu x(1-x)$, for $\mu \in [0, 4]$. The parameter $\mu$ plays the role of population growth rate in an ecological model: $x$ represents population as a fraction of carrying capacity, and the $(1-x)$ factor implements overcrowding. The model is probably not great ecology, but as mathematics it is extraordinary.

## 11.5.1 Complete Picture at $\mu = 4$

At $\mu = 4$, the logistic map is as chaotic as it can be. This is the one parameter value where a complete, exact analysis is available in closed form.

**Theorem 11.5.1.** $f_4$ is topologically conjugate to the tent map $T(x) = 1 - |2x-1|$ via the conjugacy $h(x) = \sin^2(\pi x/2)$ (or equivalently $h(x) = (2/\pi)\arcsin(\sqrt{x})$).

*Proof:* We need to verify $f_4 \circ h = h \circ T$. Compute:
$$f_4(h(\theta)) = 4\sin^2(\pi\theta/2)\!\left(1 - \sin^2(\pi\theta/2)\right) = 4\sin^2(\pi\theta/2)\cos^2(\pi\theta/2) = \sin^2(\pi\theta) = h(2\theta \bmod 1) = h(T(\theta)).$$

What this is saying is: the logistic map at $\mu = 4$ is, up to a smooth change of coordinates, nothing but the tent map — which is itself conjugate to the doubling map $\theta \mapsto 2\theta \bmod 1$. This means all the dynamical properties of $f_4$ can be read off from the well-understood doubling map.

**Consequences of Conjugacy:**

Since $f_4$ is conjugate to the tent map, and the tent map is conjugate to the doubling map, we immediately inherit:
- Topological entropy: $h_{\text{top}}(f_4) = \log 2$
- Lyapunov exponent for Lebesgue-a.e. orbit: $\lambda = \log 2$
- Invariant measure: the arcsine distribution $d\mu_{\text{arc}} = \frac{dx}{\pi\sqrt{x(1-x)}}$ (the pushforward of Lebesgue measure through $h$)
- Dense periodic orbits, topological transitivity, ergodicity with respect to the arcsine measure

The arcsine measure is a beautiful object: it is concentrated near 0 and 1 (where the logistic map spends most of its time near the boundary) and vanishes near $x = 1/2$. If you simulate $f_4$ for a long time and plot a histogram of the orbit, you get a U-shaped curve — that's the arcsine distribution.

## 11.5.2 The Parameter Space

The full parameter range $\mu \in [0, 4]$ is a microscopic universe of dynamical behavior. Here is the complete portrait:

- **$\mu \in (0, 1)$:** all orbits converge to $0$. The fixed point at the origin is stable; no interesting dynamics.
- **$\mu \in (1, 3)$:** unique stable fixed point $x^* = 1 - 1/\mu$. Orbits converge to this fixed point; the dynamics is boring.
- **$\mu = 3$:** the fixed point loses stability via a pitchfork bifurcation. A period-2 orbit is born.
- **$\mu \in (3, 3.449\ldots)$:** the period-2 orbit is stable. The fixed point is unstable.
- **$\mu_\infty \approx 3.5699\ldots$ (the Feigenbaum point):** the accumulation of period-doubling bifurcations. By this point, a period-2 orbit has bifurcated to period 4, then 8, 16, ... This infinite sequence accumulates at $\mu_\infty$.
- **$\mu \in (\mu_\infty, 4)$:** chaotic behavior, but interspersed with periodic windows. At each window, a stable periodic orbit appears and then undergoes its own period-doubling cascade before disappearing.
- **$\mu = 4$:** fully developed chaos (the case analyzed above).

The period-doubling route to chaos, and especially the specific ratio at which the bifurcations accumulate (the Feigenbaum constant $\delta \approx 4.669...$), is universal: it appears in the same form in essentially any one-dimensional family of unimodal maps, and in experiments ranging from fluid convection to electronic circuits. Feigenbaum discovered this universality in 1978, and it was one of the most surprising results in the entire history of dynamical systems — the same number appears everywhere, regardless of the specific details of the system.

This universality is a consequence of a renormalization group fixed-point theory, analogous to (but deeper than) the universality in statistical physics near critical points. We touch on renormalization in complex dynamics in Chapter 13; the full story is in the notes.

The logistic map is our best example of a route to chaos — bifurcation cascades — and in Section 11.6, it will give us a concrete playground for multifractal analysis.
