# FORCE Learning as a Model of Motor Cortex Learning

## Original Motivation: Motor Cortex Trajectory Generation

Sussillo & Abbott [2009] introduced FORCE learning not primarily as a machine learning algorithm but as a computational theory of motor cortex function. The empirical puzzle they addressed was: how does motor cortex (M1) generate the complex, precisely timed spatiotemporal patterns of activity needed to drive coordinated movement? Rate-based coding models, which treat M1 neurons as encoding instantaneous movement parameters, cannot account for the millisecond-precise temporal structure observed in multi-electrode recordings during reaching tasks. A dynamical systems perspective is required.

The FORCE proposal is that M1 is a random recurrent network whose initial connectivity generates chaotic activity, and that motor learning consists in finding a readout (or set of output synaptic weights) that taps into this chaos to produce the desired movement trajectory. The mechanism is the readout feedback loop described in earlier sections: output signals are fed back into the network, stabilizing the chaos into a structured attractor that matches the target movement.

## Rotational Dynamics in M1: Churchland et al. 2012

The strongest empirical support for the dynamical-systems view of M1 comes from Churchland et al. [2012], who recorded from large populations of M1 neurons in monkeys performing center-out reaching tasks. Applying principal component analysis (PCA) to the population activity matrix, they found that the leading principal components exhibited rotational structure: the trajectory in PC space traced approximate ellipses aligned with the plane spanned by PC1 and PC2. The angular velocity of this rotation varied systematically with the reach target direction.

This rotational structure was reproduced by FORCE-trained networks in simulations by Sussillo & Abbott [2009]. The agreement is not trivial: rotational dynamics emerge naturally from the FORCE training process because the feedback loop couples orthogonal dimensions of the reservoir state, generating stable oscillatory structure. The fact that biological M1 and FORCE-trained models share this geometric signature is a nontrivial prediction of the computational framework.

## The Biological Learning Rule Problem

Despite this success, there is a fundamental reservation about FORCE as a biological model: it requires a global error signal. The RLS update is

$$\mathbf{w}^{\text{out}}(t) \leftarrow \mathbf{w}^{\text{out}}(t-1) + e(t) \mathbf{P}(t) \mathbf{x}(t),$$

where $e(t) = z^*(t) - z(t)$ is the signed error between target and actual output, and $\mathbf{P}(t)$ is the running inverse correlation matrix. Computing $e(t)$ requires knowledge of the target $z^*(t)$ at each moment — a supervised signal. In a biological system, this would require a teaching neuron or population that has access to the correct motor command at every millisecond of movement. No such structure has been identified in M1 [Sussillo & Abbott 2009].

This is the standard objection to any backpropagation-like learning rule in biological circuits: the required error signal is not plausibly available in the brain's local synaptic environments.

## More Biologically Plausible Variants

Several variants of FORCE have been proposed to address the biological plausibility concern.

**Reward-modulated FORCE:** Replace the moment-by-moment error $e(t)$ with a trial-averaged reward signal $R$. Synaptic updates are gated by $R - \bar{R}$ (reward minus running mean), combined with local Hebbian traces. This is biologically more plausible because reward signals (dopamine, serotonin) are available on a trial timescale. However, the convergence properties are weaker than standard FORCE [Sussillo & Abbott 2009].

**Local approximations:** Approximating $\mathbf{P}(t)$ by a scalar or diagonal matrix reduces the update to $\delta \mathbf{w}^{\text{out}} \propto e(t) \mathbf{x}(t)$, a simple error-weighted Hebbian rule. This loses the second-order convergence properties of RLS but may be sufficient for simple tasks and is locally computable.

## Comparison with Cerebellum as Error-Driven System

The cerebellum is the canonical biological site for error-driven supervised learning. The Marr–Albus–Ito model proposes that climbing fibers from the inferior olive carry a teaching signal that drives long-term depression (LTD) at parallel fiber–Purkinje cell synapses [Ito 1984]. This maps onto a reservoir computing architecture with remarkable precision: granule cells as reservoir, Purkinje cells as readout, climbing fiber error as the supervision signal.

Importantly, cerebellar learning is anatomically localized: the error signal arrives directly at the synapse being modified. This is more compatible with biological implementation than FORCE's global error requirement. Some researchers have proposed that M1 motor learning is more cerebellar-like than FORCE-like — that the relevant synaptic modifications occur in projection pathways from motor cortex to spinal cord, where cerebellar-type error signals are plausible [Shadmehr & Mussa-Ivaldi 1994].

## What FORCE Explains and What It Cannot

FORCE is a productive computational model in the following senses: (1) it demonstrates that random recurrent networks can, in principle, generate complex motor trajectories; (2) it correctly predicts rotational dynamics in M1; (3) it provides a mechanistically explicit account of how motor learning could change network dynamics. These are genuine scientific contributions.

What FORCE cannot explain includes: (1) the distributed, spatially structured anatomy of M1, which is not a homogeneous random network; (2) the multi-stage synaptic plasticity mechanisms that real motor learning involves; (3) how the required error signal is computed and delivered; (4) why motor learning requires thousands of practice trials when FORCE converges in $O(T_{\text{train}})$ steps with the correct error signal.

**Epistemic status:** FORCE learning is a productive computational model of motor cortex dynamics. It correctly accounts for several empirical observations and generates testable predictions. It is not a confirmed biological learning mechanism, and the gap between the algorithm's requirements (global error signal, RLS updates) and known neurobiology is substantial. Researchers should treat FORCE as a framework for thinking about motor cortex computation, not as a literal description of synaptic learning rules.

---

## References

- Sussillo, D., & Abbott, L. F. (2009). Generating coherent patterns of activity from chaotic neural networks. *Neuron*, 63(4), 544–557.
- Churchland, M. M., Cunningham, J. P., Kaufman, M. T., Foster, J. D., Nuyujukian, P., Ryu, S. I., & Shenoy, K. V. (2012). Neural population dynamics during reaching. *Nature*, 487(7405), 51–56.
- Shadmehr, R., & Mussa-Ivaldi, F. A. (1994). Adaptive representation of dynamics during learning of a motor task. *Journal of Neuroscience*, 14(5), 3208–3224.
- Ito, M. (1984). *The Cerebellum and Neural Control*. Raven Press.
