# 15.1.3 Spike-Timing-Dependent Plasticity (STDP)

## From Hebb to a timing rule

Learning in the brain is, to first approximation, a change in synaptic weights. Hebb's 1949 postulate — *neurons that fire together wire together* — states that a synapse is strengthened when the presynaptic cell reliably participates in firing the postsynaptic cell. **Spike-timing-dependent plasticity (STDP)** is the experimentally measured, temporally precise form of this idea: the sign and magnitude of the weight change depend on the *relative timing* of pre- and postsynaptic spikes. Bi and Poo (1998) mapped the rule quantitatively in cultured hippocampal neurons, and Markram et al. (1997) demonstrated the same causal ordering in neocortical pyramidal cells.

Define the timing difference

$$\Delta t = t_{post} - t_{pre}.$$

If the presynaptic spike arrives *before* the postsynaptic one ($\Delta t > 0$), the presynaptic input plausibly *helped cause* the output, and the synapse is **potentiated** (long-term potentiation, LTP). If it arrives *after* ($\Delta t < 0$), the input could not have contributed, and the synapse is **depressed** (long-term depression, LTD). The effect is strongest for near-coincident spikes and decays as the interval grows — an exponential **STDP window**:

$$\Delta w = \begin{cases} +A_+\, e^{-\Delta t/\tau_+}, & \Delta t > 0 \quad(\text{LTP}) \\[4pt] -A_-\, e^{\,\Delta t/\tau_-}, & \Delta t < 0 \quad(\text{LTD}) \end{cases} \tag{1}$$

The amplitudes $A_\pm$ are a few percent of the maximum weight, and the biological time constants are $\tau_+ \approx 15$–$20$ ms and $\tau_- \approx 20$–$35$ ms. The window is characteristically **asymmetric**: potentiation and depression have different amplitudes and widths, and in most cortical measurements the integrated depression slightly exceeds the integrated potentiation.

## Worked Example: weight change for two spike pairs

Take representative constants $A_+ = 0.010$, $A_- = 0.012$ (in units of the maximum weight $w_{max}$), $\tau_+ = 17$ ms, and $\tau_- = 34$ ms.

**Causal pairing (pre leads post).** Let $t_{pre} = 0$ and $t_{post} = +10$ ms, so $\Delta t = +10$ ms, which is greater than zero (LTP):

$$\Delta w = A_+\, e^{-\Delta t/\tau_+} = 0.010\times e^{-10/17} = 0.010\times 0.555 = +0.0056,$$

a potentiation of about $+0.56\%$ of $w_{max}$.

**Anti-causal pairing (post leads pre).** Now let $t_{post} = 0$ and $t_{pre} = +10$ ms, so $\Delta t = -10$ ms, less than zero (LTD):

$$\Delta w = -A_-\, e^{\Delta t/\tau_-} = -0.012\times e^{-10/34} = -0.012\times 0.745 = -0.0089,$$

a depression of about $-0.89\%$ of $w_{max}$. The same 10 ms interval produces opposite-sign changes depending only on *order*, and here the anti-causal change is the larger — a consequence of the window asymmetry.

**Net bias.** Integrating (1) over all intervals gives total potentiation $\propto A_+\tau_+ = 0.010\times 17 = 0.17$ and total depression $\propto A_-\tau_- = 0.012\times 34 = 0.41$. The excess depression (ratio $\approx 2.4$) means that uncorrelated pre/post activity, which samples the window at random $\Delta t$, produces net weakening — a built-in stabilizing pressure against runaway potentiation.

**Why timing sets the sign.** The asymmetry has a mechanistic basis. The postsynaptic NMDA receptor behaves as a molecular coincidence detector, passing current only when presynaptic glutamate release coincides with postsynaptic depolarization delivered by a back-propagating action potential. A large, fast calcium transient — pre just before post — triggers the potentiation pathway, whereas the smaller, delayed calcium influx of the reverse ordering recruits depression instead. The exponential windows of equation (1) are thus a compact phenomenological summary of underlying calcium-dependent signaling. That the biology reduces to a function of a single timing variable is precisely why one physical quantity — the temporal overlap of two pulses at a phase-change cell — can emulate it.

## Competitive, unsupervised feature learning

STDP is **local** (each synapse uses only its own pre- and postsynaptic spike times) and **unsupervised** (no external teaching signal), yet repeated pairings organize a neuron's inputs into a meaningful structure. Consider a neuron with many afferents driven by a recurring input pattern. Synapses whose spikes *consistently precede* the neuron's firing are repeatedly potentiated by the $\Delta t > 0$ arm and climb toward the upper bound $w_{max}$; synapses that fire after, or that fire at random times uncorrelated with the output, are net-depressed by the LTD arm and the stabilizing bias and sink toward zero. With hard bounds at $0$ and $w_{max}$, the weight distribution is driven to be **bimodal** — most synapses pinned near $0$ or $w_{max}$, few in between.

This is **competitive learning**: the afferents that best predict the neuron's activity win a larger share of its response, so the neuron becomes selective for the specific input feature carried by its potentiated synapses, while different neurons — with different initial conditions and lateral inhibition — specialize to different features. No labels are needed. This same coincidence-detecting, timing-driven mechanism is what photonic synapses realize physically in Chapter 16: the temporal overlap of a pre-spike pulse and a post-spike pulse at a phase-change cell sets whether the deposited optical energy crystallizes (potentiate) or amorphizes (depress) the material, implementing equation (1) all-optically.

## References

Bi, G.-Q. & Poo, M.-M. (1998). "Synaptic modifications in cultured hippocampal neurons: dependence on spike timing, synaptic strength, and postsynaptic cell type." *Journal of Neuroscience*, 18(24), 10464–10472.

Markram, H., Lübke, J., Frotscher, M. & Sakmann, B. (1997). "Regulation of synaptic efficacy by coincidence of postsynaptic APs and EPSPs." *Science*, 275(5297), 213–215.

Gerstner, W. & Kistler, W.M. (2002). *Spiking Neuron Models: Single Neurons, Populations, Plasticity*. Cambridge University Press.
