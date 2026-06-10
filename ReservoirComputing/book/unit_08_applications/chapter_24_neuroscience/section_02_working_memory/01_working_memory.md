# Working Memory as Reservoir Activity

## Working Memory: Definition and Neural Correlate

Working memory is the cognitive system responsible for temporarily holding and manipulating information for use in ongoing mental tasks — verbal rehearsal, spatial reasoning, arithmetic [Baddeley 1986]. It is distinguished from long-term memory by its limited capacity, rapid decay, and dependence on active maintenance.

The canonical neural correlate of working memory is sustained firing in prefrontal cortex (PFC) during the delay period of delayed-response tasks. In a prototypical experiment (Goldman-Rakic variant of the Wisconsin card task), a monkey sees a visual cue, then must remember its location during a 3–15 second delay while no cue is visible, then uses the memory to make a response. Single-unit recordings show that specific PFC neurons fire persistently throughout the delay period at rates 2–10× their baseline — a form of activity that vanishes when the animal is distracted or when the memory is no longer needed [Fuster & Alexander 1971].

## Reservoir Model: Sustained Activity as Attractor Dynamics

The reservoir computing interpretation of working memory treats the sustained PFC activity as an attractor of the cortical network's dynamics. The memory state $\mathbf{m} \in \mathbb{R}^N$ is a fixed point (or slow manifold) of the reservoir dynamics:

$$\mathbf{m} = \tanh(\mathbf{W}^{\text{rec}}\mathbf{m} + \mathbf{b}).$$

Encoding a new memory corresponds to a brief input perturbation that shifts the reservoir state from baseline toward the memory attractor. Maintaining the memory requires the attractor to be stable — perturbations from noise should decay back to $\mathbf{m}$. Recall corresponds to reading out which attractor is currently occupied.

The Compte et al. [2000] attractor model formalizes this: a recurrent cortical network with strong excitatory-excitatory connections and broad inhibitory connections can sustain multiple stable activity bumps, each corresponding to a different remembered location (spatial working memory). The bump position (encoded in which neurons are most active) is a continuous variable representing the remembered cue location.

## Capacity: IPC Bound

How many items can be held in reservoir working memory simultaneously? The information processing capacity (IPC) of a reservoir [Dambre et al. 2012] provides an upper bound. For $N$ reservoir neurons, $\text{IPC} \leq N$, with equality for orthogonal input modes. Each remembered item occupies some effective dimension of the reservoir state space — the number of linearly independent fixed points is bounded by the rank of the autoassociative weight matrix, which cannot exceed $N$.

For a Hopfield-like reservoir, the maximum number of stable fixed points is $\sim 0.14 N$ for random patterns [Hopfield 1982]. For optimally designed reservoirs (using the pseudo-inverse rule for weight storage), the capacity approaches $N$. The attractor model of working memory thus predicts a capacity of $O(N)$ — consistent with Miller's famous observation of $7 \pm 2$ items, if we interpret $N$ as the effective dimensionality of the PFC memory network ($\sim 7$) [Baddeley 1986].

## Noise Robustness and Working Memory Failure

Working memory fails when noise displaces the reservoir state outside the attractor basin. For a fixed-point attractor $\mathbf{m}$, the basin of attraction is the region $\mathcal{B}(\mathbf{m}) = \{\mathbf{x} : \lim_{t\to\infty}\mathbf{x}(t) = \mathbf{m}\}$. The size of this basin determines noise robustness: larger basins tolerate more noise.

For a reservoir with Gaussian noise $\boldsymbol{\eta}_t \sim \mathcal{N}(\mathbf{0}, \sigma^2 \mathbf{I})$ added at each step, the probability of remaining in the memory state after $T$ steps decays approximately as:

$$P(\text{memory intact at } t = T) \approx \exp\!\left(-\frac{T \sigma^2}{2 \|\nabla V(\mathbf{m})\|^2}\right),$$

where $V(\mathbf{m})$ is the depth of the energy well at the attractor (analogous to barrier height in an Arrhenius rate model). Deeper wells (stronger recurrence) and shorter delays ($T$ smaller) both improve memory reliability — consistent with empirical observations of PFC working memory [Compte et al. 2000].

## Manipulation vs. Storage

Baddeley's [1986] model distinguishes the phonological loop (verbal storage), the visuospatial sketchpad (spatial storage), and the central executive (manipulation). The reservoir model addresses storage naturally (fixed-point attractors), but manipulation — transforming the stored representation — requires the readout to apply nontrivial operations to the reservoir state.

In the reservoir framework, manipulation can be implemented by training a readout that computes a transformed version of the stored pattern: $y_t = \mathbf{W}^{\text{out}}\mathbf{x}_t$ where $y_t$ is the transformed value. The transformation is learned by the readout, not by modifying the reservoir — consistent with the reservoir computing philosophy. This provides a computational model of how PFC can both store and transform information, but requires that the transformation be learnable from a linear readout of the current state, which is not guaranteed for all manipulations.

**Epistemic status:** Attractor theory of working memory is the dominant computational model and has substantial supporting evidence from single-unit recordings and lesion studies. The specific cellular and synaptic mechanisms maintaining sustained activity (whether intrinsic bistability, synaptic reverberation, or a combination) remain actively debated.

---

## References

- Baddeley, A. D. (1986). *Working Memory*. Oxford University Press.
- Fuster, J. M., & Alexander, G. E. (1971). Neuron activity related to short-term memory. *Science*, 173(3997), 652–654.
- Compte, A., Brunel, N., Goldman-Rakic, P. S., & Wang, X. J. (2000). Synaptic mechanisms and network dynamics underlying spatial working memory in a cortical network model. *Cerebral Cortex*, 10(9), 910–923.
