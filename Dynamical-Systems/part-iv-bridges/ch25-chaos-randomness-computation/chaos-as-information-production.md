# 25.1 Chaos as Information Production

The intuition is simple: a chaotic orbit is sensitive to initial conditions. Two orbits that start $\varepsilon$ apart will be $\varepsilon e^{\lambda t}$ apart at time $t$ (for the maximum Lyapunov exponent $\lambda$). So the information you need to specify an orbit at time $t$, to the same accuracy as you specified it at time 0, grows like $\lambda t$ bits.

This is the information-production rate of chaos. And by Pesin's formula, it equals the KS entropy exactly.

**The Central Identity:** For an ergodic system with KS entropy $h = h_\mu(f)$ and Pesin's formula:
$$h = \sum_{\lambda_i > 0} \lambda_i \quad \text{(sum of positive Lyapunov exponents)}.$$

Each positive Lyapunov exponent $\lambda_i$ contributes information production at rate $\lambda_i$ bits per unit time (using natural log; divide by $\log 2$ for bits).

To see why this is the right count: the positive Lyapunov exponents $\lambda_1 \geq \lambda_2 \geq \cdots \geq \lambda_k > 0 > \lambda_{k+1} \geq \cdots$ correspond to the expanding directions of the map. An initial uncertainty $\varepsilon$ in these directions grows exponentially, so to maintain accuracy $\varepsilon$ at time $t$, you need initial accuracy $\varepsilon e^{-\lambda_i t}$ in direction $i$. The total information needed is:

$$\sum_{i: \lambda_i > 0} \lambda_i t \cdot \frac{1}{\log 2} \text{ bits}.$$

This is $h \cdot t / \log 2$ bits — exactly the KS entropy rate, converted to bits.

The negative Lyapunov exponents don't contribute: they correspond to contracting directions, where initial errors get *smaller* over time. Contracting directions produce information about the future (the orbit converges to the attractor), but this information comes from forgetting initial conditions, not from generating new ones.

**Operational Meaning:** A binary description of an orbit of length $T$ with precision $\varepsilon$ requires $\approx hT/\log 2$ bits. Specifying the initial condition to accuracy $\varepsilon e^{-\lambda T}$ costs $hT/\log 2$ bits and predicts the orbit to time $T$ at accuracy $\varepsilon$.

The practical implication: for a system with $\lambda = 1$ per second (fairly strong chaos), predicting 10 seconds ahead requires 10 bits more precision than predicting 0 seconds ahead. To predict to time $T = 100$ seconds with 1% accuracy ($\varepsilon = 10^{-2}$), you need initial condition accuracy $\delta = 10^{-2} e^{-100} \approx 10^{-46}$ — a precision that is physically impossible to achieve.

This is why weather prediction fails beyond about two weeks. The atmosphere has positive Lyapunov exponents; the information needed for accurate prediction grows exponentially. No supercomputer can save you — the fundamental limit is information-theoretic, not computational.

But there's a subtlety. Pesin's formula holds for smooth invariant measures. For measures that are not absolutely continuous on unstable manifolds (singular measures, atomic measures), the Ruelle inequality gives $h_\mu \leq \sum_{\lambda_i > 0} \lambda_i$ with strict inequality possible. A system can have positive Lyapunov exponents but small or zero entropy if the invariant measure is concentrated on a very regular set (like a low-dimensional fractal) that doesn't fill up the unstable manifolds.

So: positive Lyapunov exponents imply positive topological entropy (the variational principle says $h_{\text{top}} \geq h_\mu$ for any ergodic measure), but an individual invariant measure can have zero entropy even in a chaotic system. The natural physical measure — the SRB measure — satisfies Pesin's formula, so for SRB measures, positive Lyapunov exponents and positive entropy are equivalent.
