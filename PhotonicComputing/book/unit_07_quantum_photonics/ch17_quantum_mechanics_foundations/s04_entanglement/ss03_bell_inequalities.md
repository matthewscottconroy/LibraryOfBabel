# 17.4.3 Bell Inequalities

## From Philosophy to Measurement

EPR's 1935 argument was sharp: if measuring photon A instantly fixes the outcome at distant photon B, then either something travels faster than light (unacceptable), or the outcome at B was determined all along by properties the quantum state fails to mention — **local hidden variables**. For thirty years this seemed a matter of interpretation, since both views predicted the same observed correlations. John Bell's 1964 theorem ended the truce: *the two views make different, testable predictions.* Any local hidden-variable (LHV) theory obeys quantitative constraints on correlations that quantum mechanics violates.

## The CHSH Inequality

The experimentally practical form is due to Clauser, Horne, Shimony, and Holt (1969). Alice measures her photon with one of two settings $a, a'$; Bob with $b, b'$; every measurement yields $\pm 1$ (e.g., transmitted/reflected at a polarizing analyzer). Define the correlation $E(a, b) = \langle A_a B_b \rangle$.

**The LHV bound.** In a local hidden-variable model, each photon pair carries variables $\lambda$ (distributed as $p(\lambda)$) that fix all four *local* response functions $A_a(\lambda), A_{a'}(\lambda), B_b(\lambda), B_{b'}(\lambda) \in \{\pm 1\}$ — Alice's outcome cannot depend on Bob's setting (locality), and all four values coexist (realism). For each $\lambda$, consider

$$S(\lambda) = A_a(B_b - B_{b'}) + A_{a'}(B_b + B_{b'})$$

One of the parentheses is $0$ and the other $\pm 2$ (since $B_b, B_{b'} = \pm 1$), so $S(\lambda) = \pm 2$ always. Averaging over $\lambda$:

$$\boxed{\;|S| = \left|E(a,b) - E(a,b') + E(a',b) + E(a',b')\right| \;\leq\; 2\;}$$

**The quantum prediction.** For qubit observables $\hat{A}_a = \mathbf{a}\cdot\hat{\boldsymbol{\sigma}}$ etc., the Bell state $|\Phi^+\rangle$ gives $E(a, b) = \cos 2(\theta_a - \theta_b)$ for linear polarization analyzers at angles $\theta_a, \theta_b$.

**Worked example (the optimal angles).** Choose $\theta_a = 0°$, $\theta_{a'} = 45°$, $\theta_b = 22.5°$, $\theta_{b'} = 67.5°$. Then every one of the four analyzer-angle differences is $22.5°$ except $(a, b')$, which is $67.5°$:

$$S = \cos 45° - \cos 135° + \cos 45° + \cos 45° = \frac{3}{\sqrt{2}} + \frac{1}{\sqrt{2}} = 2\sqrt{2} \approx 2.83$$

Quantum mechanics exceeds the classical bound by 41%. And $2\sqrt{2}$ is itself a theorem — the **Tsirelson bound**: for any quantum state and any $\pm1$-valued observables, $|S| \leq 2\sqrt{2}$, a consequence of the operator identity $\hat{S}^2 = 4\cdot\mathbb{1} - [\hat{A}_a, \hat{A}_{a'}]\otimes[\hat{B}_b, \hat{B}_{b'}]$ with each commutator bounded by 2. Quantum correlations are stronger than classical but not maximally strong; nature violates locality with restraint.

## The Experiments: Photons Decide

Every landmark Bell test has been photonic (with one notable solid-state exception):

- **Freedman & Clauser (1972)** — first Bell test, polarization-entangled photons from a calcium atomic cascade: violation observed.
- **Aspect, Grangier, Roger & Dalibard (1981–82)** — cascade photons with fast *time-varying* analyzers, switching settings while the photons were in flight, closing (approximately) the locality loophole; violation by dozens of standard deviations.
- **Weihs et al. (1998)** — SPDC pairs over 400 m with truly random, spacelike-separated setting choices.
- **2015, the loophole-free year**: Hensen et al. (Delft) using entangled NV-center spins heralded by photons (closing detection and locality loopholes simultaneously); Giustina et al. (Vienna) and Shalm et al. (NIST) using SPDC photon pairs with $>90\%$-efficient detectors (transition-edge sensors) and spacelike-separated random settings. All three: LHV models excluded with high statistical significance.
- **Nobel Prize in Physics 2022** to Clauser, Aspect, and Zeilinger "for experiments with entangled photons, establishing the violation of Bell inequalities and pioneering quantum information science."

The loopholes were not pedantry. The **detection loophole** (with inefficient detectors, the detected subensemble can fake a violation — closed only when overall efficiency exceeds $\sim 83\%$ for CHSH, which is why SNSPDs and TES detectors of Chapter 19 were prerequisites) and the **locality loophole** (settings must be chosen too late for any light-speed influence to reach the other side) both had to fall in a *single* experiment before the conclusion was airtight.

## What the Violation Means — and What It Buys

The conclusion is not "signals travel faster than light" (they cannot; the no-signaling structure of 17.4.1 survives). It is that **no theory in which measurement outcomes are locally predetermined can describe nature**. The Born rule's randomness (17.1.3) is not ignorance of hidden facts; before measurement, the individual outcomes genuinely do not exist. Quantum correlations are relations without relata carried by any local messenger.

For this book, the violation is also a *certificate* with engineering value:

1. **Entanglement verification.** A measured $S > 2$ witnesses entanglement across any channel, with no assumptions about the source. Deployed entangled-pair links (Chapter 22) routinely quote their CHSH $S$ as a figure of merit.
2. **Device-independent QKD.** If Alice and Bob's correlations violate CHSH, *no eavesdropper can hold a predictive copy of their outcomes* — security guaranteed by physics, not by trusting the hardware vendor. The 2015-style loophole-free configuration is precisely the security model.
3. **Certified randomness.** Outcomes that violate a Bell inequality are provably not pre-recorded; quantum random-number services expand entropy on exactly this basis.

Einstein's "spooky action" thus completed one of science's great arcs: from a thought experiment meant to embarrass quantum mechanics, to a measured property of photon pairs, to a commodity specification on quantum network hardware.
