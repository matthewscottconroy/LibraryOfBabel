# 16.4.1 STDP with Optical Pulses

## The rule to be implemented

Spike-timing-dependent plasticity makes a synapse's weight change depend on the *relative timing* of pre- and postsynaptic spikes. Writing $\Delta t=t_\text{post}-t_\text{pre}$, the canonical window (§15.1.3; Bi & Poo 1998) is
$$\Delta w=\begin{cases}+A_+\,e^{-\Delta t/\tau_+}, & \Delta t>0\ (\text{pre before post: potentiation, LTP})\\[4pt]-A_-\,e^{\,\Delta t/\tau_-}, & \Delta t<0\ (\text{post before pre: depression, LTD}).\end{cases}$$
The synapse is strengthened when it helped cause the postsynaptic spike (causal ordering) and weakened when it fired too late to have contributed (anti-causal) — a Hebbian, entirely local rule. In biology $\tau_\pm$ are tens of milliseconds. The task of a photonic implementation is to reproduce the *shape* of this window while compressing its timescale by six to eight orders of magnitude.

## Physical implementation at a phase-change synapse

Feldmann et al. (2019) realized such a rule all-optically on a Si$_3$N$_4$ chip using phase-change-material (PCM) synapses and ring resonators. The mechanism exploits the same PCM physics that stores the synaptic weight (§16.2). A presynaptic spike arrives as an optical pulse on carrier $\lambda_\text{pre}$ and a postsynaptic spike as a pulse on $\lambda_\text{post}$; both are routed to the PCM cell. What matters is the *coincidence* of the two pulses at the cell:

- When the pulses overlap strongly (small $|\Delta t|$), their energies add, and the combined deposited energy is enough to drive the PCM across a phase-change threshold — crystallizing the film a little further (raising transmission → **potentiation**) or, with a short, intense overlap, melt-quenching it (lowering transmission → **depression**), according to the pulse shaping.
- When the pulses are far apart in time (large $|\Delta t|$), neither alone reaches threshold, the deposited energy dissipates, and the weight is essentially unchanged.

Because the amount of overlapping energy falls off with the temporal separation $\Delta t$ between the pulses, the induced weight change $\Delta w(\Delta t)$ naturally traces out an STDP-like window. Which side of the window potentiates and which depresses — whether coincidence crystallizes or amorphizes — is fixed by engineering the pulse amplitudes and durations relative to the SET and RESET thresholds of the PCM (§16.2).

## Setting $\Delta t$ with propagation delay

In a photonic network the timing difference $\Delta t$ is not a free abstract parameter; it is *physical propagation delay*. A spike travels from neuron to synapse along a waveguide of group index $n_g$ at group velocity $v_g=c/n_g$, so a path of length $L$ contributes a delay
$$\tau_\text{delay}=\frac{L}{v_g}=\frac{n_g L}{c}.$$
By choosing the relative path lengths of the pre- and post-synaptic routes — or by inserting an explicit optical **delay line** — the designer *sets* $\Delta t$ and thereby positions the synapse on the STDP curve. The width of the learning window itself, $\tau_\pm$, is likewise engineered, through the pulse durations and the delay-line increments, rather than being fixed by biology.

### Worked Example: from delay line to weight change

Suppose the photonic STDP window has a potentiation time constant $\tau_+=100$ ps and amplitude $A_+=0.05$ (a 5% maximum step), on a Si$_3$N$_4$ platform with group index $n_g=2.0$ (so $v_g=c/n_g=1.5\times10^8$ m/s).

*What length sets a given $\Delta t$?* To place the pre-pulse $\Delta t=40$ ps ahead of the post-pulse, the pre-synaptic path must be longer (or the post path shorter) by
$$\Delta L=v_g\,\Delta t=(1.5\times10^{8}~\text{m/s})(40\times10^{-12}~\text{s})=6.0\times10^{-3}~\text{m}=6.0~\text{mm}.$$
A 6 mm spiral delay waveguide — readily integrated — realizes a 40 ps causal offset.

*Resulting potentiation.* With $\Delta t=+40$ ps in the LTP branch,
$$\Delta w=A_+\,e^{-\Delta t/\tau_+}=0.05\,e^{-40/100}=0.05\times e^{-0.4}=0.05\times0.670=+0.034,$$
a $+3.4\%$ strengthening. Doubling the delay to $\Delta t=+80$ ps ($\Delta L=12$ mm) gives $\Delta w=0.05\,e^{-0.8}=+0.022$, a $+2.2\%$ step — the exponential window at work. Had the post-pulse instead led by 40 ps ($\Delta t=-40$ ps), with $A_-=0.05$ and $\tau_-=120$ ps, the synapse would depress by $\Delta w=-0.05\,e^{-40/120}=-0.036$, a $-3.6\%$ step.

*Energy check.* For any update to occur, the coincident pulses must deposit enough energy to cross the crystallization threshold. Heating a PCM patch of volume $V\sim0.5~\mu\text{m}\times1~\mu\text{m}\times10$ nm by $\Delta T\sim150$ °C takes, order-of-magnitude,
$$E\sim\rho\,c_p\,V\,\Delta T\approx(6.2\times10^{3}~\text{kg/m}^3)(210~\text{J·kg}^{-1}\text{K}^{-1})(5\times10^{-21}~\text{m}^3)(150~\text{K})\approx1~\text{pJ},$$
though real device write energies, once heat lost to the substrate is included, are larger and are typically quoted in the pJ–nJ range. The point is that only *near-coincident* pre/post pulses pool enough energy to reach this threshold, which is precisely what confines plasticity to a narrow timing window.

## Relation to biology

The photonic window has the same causal shape as the hippocampal STDP curve of §15.1.3, but its time constants are picoseconds rather than the biological $\tau_+\approx15$–$20$ ms and $\tau_-\approx20$–$35$ ms — a compression of roughly eight orders of magnitude, matching the picosecond-versus-millisecond speed gap between photonic and biological spikes. Learning that would take a cortical synapse tens of milliseconds is completed here in the time light needs to cross a few millimeters of waveguide. This is the appeal of on-chip optical STDP: an adaptive, local, unsupervised rule that runs at the native speed of the photonic network, with the weight physically stored in the same PCM cell that computes with it (Prucnal & Shastri 2017).

---
## References

Feldmann, J., Youngblood, N., Wright, C.D., Bhaskaran, H. & Pernice, W.H.P. (2019). "All-optical spiking neurosynaptic networks with self-learning capabilities." *Nature*, 569(7755), 208–214.

Bi, G.-Q. & Poo, M.-M. (1998). "Synaptic modifications in cultured hippocampal neurons: dependence on spike timing, synaptic strength, and postsynaptic cell type." *Journal of Neuroscience*, 18(24), 10464–10472.

Prucnal, P.R. & Shastri, B.J. (2017). *Neuromorphic Photonics*. CRC Press.
