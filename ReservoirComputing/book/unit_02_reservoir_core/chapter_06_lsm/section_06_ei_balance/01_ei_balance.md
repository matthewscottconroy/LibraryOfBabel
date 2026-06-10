# Section 6.6: Excitatory-Inhibitory Balance

## 6.6.1 Biological Background

Cortical circuits are not made of interchangeable neurons. About 80% of cortical neurons are excitatory pyramidal cells — they release glutamate and increase the firing probability of their postsynaptic targets. The remaining 20% are inhibitory interneurons, releasing GABA and suppressing their targets [DeFelipe1992]. This excitatory-inhibitory (E/I) asymmetry in cell count is remarkably conserved across species and cortical areas.

A related biological fact is *Dale's law* [Dale1935]: a neuron releases the same neurotransmitter at all of its synapses. A neuron that is excitatory (releases glutamate) makes excitatory synapses onto all its postsynaptic targets; an inhibitory neuron inhibits all of its targets. This constrains the sign structure of the biological weight matrix: the $i$-th row of the connectivity matrix is either all non-negative (excitatory neuron $i$) or all non-positive (inhibitory neuron $i$).

These biological facts may seem like engineering nuisances for artificial reservoir design. In fact, they have deep functional consequences: the E/I ratio and the balanced state that emerges from it are essential for the rich, irregular dynamics that make biological circuits — and biologically constrained reservoirs — computationally powerful.

## 6.6.2 The Balanced Network Model

Van Vreeswijk and Sompolinsky [vanVreeswijk1996] provided the theoretical framework for understanding E/I balance. They considered a network of $N_E$ excitatory and $N_I$ inhibitory neurons ($N_E / N_I = 4$, matching the biological ratio) with random, sparse connectivity. Each neuron receives $K$ synaptic inputs on average, with connection strengths of order $J/\sqrt{K}$.

The key observation is that in the large-$K$ limit, the total excitatory drive and total inhibitory drive to each neuron are both of order $J \sqrt{K}$ — much larger than the threshold for spiking. Naive intuition suggests the network should be overwhelmingly active. Instead, van Vreeswijk and Sompolinsky showed that the excitatory and inhibitory inputs *cancel to leading order*, leaving a fluctuating net drive of order $J$ (the standard deviation):

$$I_{net}(t) = I_E(t) + I_I(t) \approx 0 + \mathcal{O}(J),$$

where the cancellation occurs dynamically: if activity increases slightly, inhibitory feedback grows proportionally and restores balance.

This is the *balanced state*: a network operating point where mean excitation and inhibition cancel, and neurons are driven by fluctuations around zero mean. The mean field equations for the balanced state are:

$$J_{EE} \nu_E + J_{EI} \nu_I = \theta_E - \mu_E^{ext},$$
$$J_{IE} \nu_E + J_{II} \nu_I = \theta_I - \mu_I^{ext},$$

where $\nu_{E,I}$ are the mean firing rates of the excitatory and inhibitory populations, $J_{\alpha\beta}$ are the effective connection strengths, $\theta$ are thresholds, and $\mu^{ext}$ is external input. The balanced state exists when these equations have a unique, positive solution — which requires the inhibitory feedback to be strong enough to stabilize runaway excitation.

## 6.6.3 Mathematical Analysis: The Mean-Field Fixed Point

For a reservoir with $N_E$ excitatory and $N_I$ inhibitory neurons, write the dynamics as

$$\tau \dot{\mathbf{r}} = -\mathbf{r} + \phi(W \mathbf{r} + W^{in} \mathbf{u} + \boldsymbol{\xi}),$$

where $\mathbf{r} = (\mathbf{r}_E, \mathbf{r}_I)^\top$ is the full firing rate vector, $\phi$ is a nonlinear gain function (sigmoid or threshold-linear), $W$ is the weight matrix (block structure $W_{EE}, W_{EI}, W_{IE}, W_{II}$), and $\boldsymbol{\xi}$ represents noise.

The mean-field fixed point $(\bar{r}_E, \bar{r}_I)$ satisfies

$$\bar{r}_E = \phi\!\left(\frac{N_E J_{EE}}{K} \bar{r}_E p_{EE} - \frac{N_I J_{EI}}{K} \bar{r}_I p_{EI} + \bar{u}_E\right),$$

$$\bar{r}_I = \phi\!\left(\frac{N_E J_{IE}}{K} \bar{r}_E p_{IE} - \frac{N_I J_{II}}{K} \bar{r}_I p_{II} + \bar{u}_I\right),$$

where $p_{\alpha\beta}$ is the connection probability from population $\beta$ to population $\alpha$ and $K = N_E p_{EE}$ is the mean in-degree. In the balanced state, the arguments of $\phi$ are $O(1)$ despite individual terms being $O(\sqrt{K})$, because the leading terms cancel.

The stability of the balanced state is governed by the eigenvalues of the Jacobian $\partial \mathbf{r} / \partial \mathbf{r}|_{\bar{\mathbf{r}}}$. The balanced state is stable when inhibition is fast enough to track excitation — the *inhibition-stabilized network* regime [Tsodyks1997].

## 6.6.4 E/I Balance in Reservoir Design

For artificial reservoirs, E/I balance manifests in two design decisions:

**Dale's law constraints.** If one enforces Dale's law, the weight matrix $W$ must have non-negative entries in rows corresponding to excitatory neurons and non-positive entries in rows corresponding to inhibitory neurons. The standard random reservoir violates Dale's law (each neuron has both positive and negative outgoing weights).

**Performance under Dale's law.** Reservoirs with Dale's law constraints perform similarly to unconstrained reservoirs on most benchmark tasks [Lukosevicius2012]. The key is maintaining the correct E/I ratio: with 80% excitatory neurons (strong positive weights) and 20% inhibitory (strong negative weights), the total input to each neuron is balanced, recreating the conditions of the van Vreeswijk-Sompolinsky model.

Formally, enforcing Dale's law changes the sign structure of $W$ but preserves the essential property that the effective spectral radius can be tuned by the weight magnitude. The echo state property continues to hold for $\rho(W) < 1$ under Dale's law [Lukosevicius2012], provided the E/I balance ensures the fixed point is stable.

## 6.6.5 The Asynchronous Irregular State

The dynamical correlate of E/I balance in spiking networks is the *asynchronous irregular* (AI) state [Brunel2000]. In the AI state:
- Individual neurons fire irregularly, with inter-spike intervals close to Poisson (coefficient of variation $\approx 1$).
- Population activity is asynchronous: there is no collective oscillation.
- The auto-correlation of population activity decays rapidly (timescale $\sim$ ms), giving the network rapid response to input changes.

The AI state is computationally favorable because it maximizes the *linear response range* of the network: small changes in input produce proportional changes in output, rather than saturating nonlinear responses. Brunel's phase diagram [Brunel2000] maps the AI state onto a region of the ($g = J_I/J_E$, $\nu_{ext}/\nu_{thr}$) plane, providing quantitative design rules for biologically constrained reservoirs.

For rate-model reservoirs, the analogue of the AI state is the regime near the edge of chaos (Section 6.5): dynamics that are active and varied but not periodic or chaotic. E/I balance is one mechanism for achieving this regime robustly.

## 6.6.6 Inhibition-Stabilized Networks and Balanced Amplification

A striking feature of E/I-balanced networks is *balanced amplification* [Murphy2009]: the network can amplify transient inputs by a large factor, while remaining stable. In a purely excitatory network, large amplification requires the network to be near an instability (large spectral radius), which causes slow settling and long-lasting responses. In an E/I-balanced network, fast inhibitory feedback suppresses the runaway component, allowing large amplification of structured inputs (aligned with the readout) while suppressing unstructured noise.

Mathematically, the amplification factor for the fastest-amplifying direction in an E/I-balanced network can be $O(\sqrt{N})$, while the network remains stable [Murphy2009]. This is an inherently non-normal amplification mechanism: the weight matrix $W$ is non-normal (asymmetric), and its pseudospectrum extends far beyond the spectral radius, allowing transient growth even when all eigenvalues are within the unit disk.

For reservoir computing, balanced amplification means that E/I-balanced reservoirs can achieve large effective gain for task-relevant input directions while maintaining stability — precisely the property needed for sensitive but stable temporal processing.

---

## References

- **[Brunel2000]** N. Brunel. "Dynamics of sparsely connected networks of excitatory and inhibitory spiking neurons." *Journal of Computational Neuroscience*, 8(3):183-208, 2000.
- **[Dale1935]** H. Dale. "Pharmacology and nerve-endings." *Proceedings of the Royal Society of Medicine*, 28(3):319-332, 1935.
- **[DeFelipe1992]** J. DeFelipe and I. Fariñas. "The pyramidal neuron of the cerebral cortex: Morphological and chemical characteristics of the synaptic inputs." *Progress in Neurobiology*, 39(6):563-607, 1992.
- **[Lukosevicius2012]** M. Lukosevicius. "A practical guide to applying echo state networks." In *Neural Networks: Tricks of the Trade*, Springer, pp. 659-686, 2012.
- **[Murphy2009]** B. K. Murphy and K. D. Miller. "Balanced amplification: A new mechanism of selective amplification of neural activity patterns." *Neuron*, 61(4):635-648, 2009.
- **[Tsodyks1997]** M. V. Tsodyks, W. E. Skaggs, T. J. Sejnowski, and B. L. McNaughton. "Paradoxical effects of external modulation of inhibitory interneurons." *Journal of Neuroscience*, 17(11):4382-4388, 1997.
- **[vanVreeswijk1996]** C. van Vreeswijk and H. Sompolinsky. "Chaos in neuronal networks with balanced excitatory and inhibitory activity." *Science*, 274(5293):1724-1726, 1996.
