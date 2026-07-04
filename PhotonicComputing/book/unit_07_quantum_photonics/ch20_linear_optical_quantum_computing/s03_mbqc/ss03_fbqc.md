# 20.3.3 Fusion-Based Quantum Computing

## The Whole Computation Is Fusions

Cluster-state MBQC (Section 20.3.2) still imagines two phases: build a large entangled lattice, then measure it. Fusion-based quantum computing (FBQC), formalized by Bartolucci et al. (2023) at PsiQuantum, erases the boundary between them. Its ingredients are only two: a supply of small, **constant-size resource states** (a handful of entangled photons each — a common choice is the six-photon ring), and **fusion measurements** (the type-II partial Bell measurements of Section 20.3.1) that join them. The large cluster is never assembled and stored; instead, resource states are generated and immediately consumed by fusions, and the *pattern of fusion outcomes is the computation*. There is no separate "prepare then measure" — generation, entanglement, and measurement happen in one continuous flow, which suits photons, whose natural state is in motion.

This is the architecture that finally made linear-optical quantum computing look manufacturable. Original KLM demanded $10^4$–$10^5$ operations per two-qubit gate (Section 20.2.3); cluster states cut that by orders of magnitude; FBQC cuts it again by using resource states so small that they can be produced in parallel from multiplexed heralded sources (Chapter 19), with no large entangled state ever needing to survive intact. The failures that plagued KLM are absorbed at the level of individual fusions, where they are cheap and — crucially — *located*.

## Fusion Networks and Ballistic Percolation

The geometry of "which resource state fuses with which" is a **fusion network** — a graph specifying the plumbing. Because a linear-optics type-II fusion succeeds with probability only $p_f = 1/2$ (Browne & Rudolph, 2005), and can be *boosted* toward one using ancilla photons (Grice, 2011; Ewert & van Loock, 2014), the central question is whether a network riddled with failed fusions still contains a connected, computationally useful cluster. This is a **percolation** problem. Kieling, Rudolph, and Eisert (2007) showed that once the bond-success probability exceeds the lattice's percolation threshold $p_c$, a spanning cluster appears with near-certainty, and renormalization recovers a perfect lattice from the percolated one — turning nondeterministic fusions into deterministic computation. Gimeno-Segovia et al. (2015) made the picture concrete and *ballistic*: starting from only three-photon GHZ resource states and boosted fusions, a suitable network percolates into a universal cluster **with no feed-forward at all** — every fusion is attempted once, simultaneously, and the network is designed so that the statistically expected pattern of successes already spans. FBQC generalizes this to fault-tolerant networks whose fusion outcomes directly measure the checks of a topological code (Section 20.5.2), so that loss and Pauli errors are corrected within the same fusion pattern that runs the algorithm.

The payoff is a hardware error budget expressed per photon. FBQC networks tolerate photon-loss rates of a few percent up to roughly $10\%$ per photon, depending on the resource state and the amount of fusion boosting — thresholds within reach of foundry silicon photonics and SNSPDs, and the reason PsiQuantum (Section 20.5.3) bet an entire company on this model.

## Why the Overhead Collapsed

Three features compound to make FBQC cheap where original KLM was ruinous. Resource states are *constant-size*, so they can be manufactured in massive parallelism from multiplexed heralded sources (Chapter 19) and stockpiled, decoupling the generation rate from the circuit depth. No large entangled state is ever *stored* — the vulnerable object in cluster-state schemes — so loss and decoherence have only the lifetime of a single fusion in which to act. And failure is *local and typed*: a failed fusion is a known erasure on two named photons, feeding directly into the loss-tolerant decoder rather than corrupting a shared resource. KLM's $10^4$–$10^5$ operations per two-qubit gate become, in this accounting, a modest multiple of the raw fusion count. That is why FBQC is the first linear-optical architecture whose resource estimates a semiconductor foundry can seriously contemplate — the subject of Section 20.5.3.

## Worked Example: Boosting a Fusion Past the Percolation Threshold

**Boosted fusion probability.** An unboosted linear-optics type-II fusion succeeds with $p_f = 1/2$. Supplying an ancillary Bell pair (two extra photons) raises it to $p_f = 3/4$; each additional Bell pair halves the remaining failure probability, giving the general ladder

$$p_f(k) = 1 - 2^{-(k+1)} \quad\Longrightarrow\quad \tfrac{1}{2},\ \tfrac{3}{4},\ \tfrac{7}{8},\ \tfrac{15}{16},\dots$$

for $k = 0, 1, 2, 3$ boosting Bell pairs.

**Percolation margin.** Consider building a cluster on a diamond lattice, whose bond-percolation threshold is $p_c \approx 0.388$. Even an *unboosted* fusion at $p_f = 1/2 > 0.388$ percolates — but with negligible margin once loss is included. Model each photon as surviving with probability $\eta$; a fusion consumes two photons, so the effective bond-success probability is $p_{\text{bond}} = p_f\,\eta^2$. Requiring $p_{\text{bond}} > p_c$ gives a loss budget:

$$\eta > \sqrt{\frac{p_c}{p_f}}.$$

For $p_f = 1/2$: $\eta > \sqrt{0.776} \approx 0.881$, i.e. up to $\sim 12\%$ loss per photon merely to *percolate*. Boost to $p_f = 3/4$ and the tolerance opens to $\eta > \sqrt{0.517} \approx 0.719$ — nearly $28\%$ loss per photon. The extra two ancilla photons per fusion have bought a large loss margin, exactly the trade FBQC exploits.

**The honest caveat.** Bare percolation is *not* fault tolerance. A merely-connected cluster still carries residual Pauli errors and stray erasures that a spanning path alone does not correct; the true fault-tolerant thresholds (Section 20.5.2) sit well below these percolation numbers, at the few-percent-per-photon level quoted above. The calculation nonetheless captures the design logic of FBQC precisely: pick a resource state and a boost level, compute the effective bond probability against the network's threshold, and read off the photon-loss budget that the sources, waveguides, and detectors of Chapter 19 must then meet.
