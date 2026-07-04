# Section 16.4: Learning in Photonic Spiking Neural Networks

Having assembled photonic neurons and synapses into networks (§16.3), we now ask how such a network acquires its weights. Two learning paradigms dominate neuromorphic photonics, and they sit at opposite ends of almost every design axis.

The first is **spike-timing-dependent plasticity (STDP)**: a *local, unsupervised, on-chip* rule. Each synapse updates itself using only the relative timing of the pre- and postsynaptic spikes that pass through it, with no global error signal and no external optimizer. In a photonic synapse this is not a metaphor — the physics of a phase-change cell can be made to implement the update directly, so the network learns as it runs. Feldmann et al. (2019) demonstrated an all-optical plasticity rule of exactly this kind. §16.4.1 develops it.

The second is **surrogate-gradient training**: a *global, supervised, offline* method. A differentiable software model of the spiking network is trained by backpropagation-through-time against labelled data, and the resulting weights are then programmed into the photonic hardware. The obstacle is that a spike is a discontinuous, non-differentiable event; the surrogate-gradient method (Neftci et al. 2019) resolves it by substituting a smooth function for the spike's derivative in the backward pass only. §16.4.2 develops it, including how a well-built training model folds in the hardware's noise and non-idealities so that the deployed weights survive the transfer to silicon.

The two are complementary rather than competing. STDP is attractive for adaptive, low-power, always-on learning at the edge but is hard to steer toward a specified task; surrogate-gradient training delivers task-optimized accuracy but requires an offline model and a careful deployment step. The subsections that follow treat each in turn.

---
## References

Feldmann, J., Youngblood, N., Wright, C.D., Bhaskaran, H. & Pernice, W.H.P. (2019). "All-optical spiking neurosynaptic networks with self-learning capabilities." *Nature*, 569(7755), 208–214.

Neftci, E.O., Mostafa, H. & Zenke, F. (2019). "Surrogate gradient learning in spiking neural networks." *IEEE Signal Processing Magazine*, 36(6), 51–63.
