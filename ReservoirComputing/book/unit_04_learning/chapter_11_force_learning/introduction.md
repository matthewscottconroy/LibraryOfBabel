# Chapter 11: FORCE Learning

## The Paper That Shocked the Reservoir Community

In 2009, David Sussillo and Larry Abbott published "Generating Coherent Patterns of Activity from Chaotic Neural Networks" in *Neuron* [SussilloAbbott2009]. The paper demonstrated something that contradicted a central assumption of reservoir computing: they trained the *feedback* weights of a recurrent network — weights that fed the output back into the reservoir — online, and showed that a chaotic, seemingly uncontrollable recurrent network could be tamed into producing arbitrarily complex rhythmic outputs.

The method was called FORCE (First Order Reduced and Controlled Error) learning. Its reception in the reservoir computing community was complicated. On one hand, it was a technical tour de force: the derivation was elegant, the results were striking, and the connection to biological neural circuits was compelling. On the other hand, it broke the rules. Reservoir computing was supposed to be the fixed-reservoir paradigm — the elegance of the approach came precisely from not touching the recurrent weights. FORCE reached into the reservoir (via the feedback loop) and changed it during training.

Was FORCE still reservoir computing? The question is less important than understanding what FORCE actually does and when it works. In this chapter, we derive the FORCE algorithm from first principles, analyze its convergence, and discuss the Full-FORCE extension that addresses FORCE's main failure mode.

## What FORCE Does

The key insight of FORCE is the use of **recursive least squares (RLS)** to update the output weights online, while simultaneously feeding the current output *back into the network*. The feedback loop is crucial: it is not just a read-off of the reservoir's state, but an injection that actively shapes the reservoir's dynamics during training.

As training progresses, the output (which starts as chaotic, driven by the initial random feedback weights) gradually converges toward the target signal. The RLS algorithm updates the feedback weights to minimize the current error, and because the feedback drives the reservoir, this gradually reshapes the reservoir's trajectory toward the desired pattern. By the end of training, the network has "learned" to spontaneously produce the target without external driving — it can be run autonomously.

## Is This Still Reservoir Computing?

The honest answer is: it depends on what you mean by "reservoir computing." If you mean the strict paradigm of fixed recurrent weights with only the readout trained, then FORCE is not reservoir computing — the recurrent dynamics are indirectly modified through the feedback loop. If you mean the broader philosophy of exploiting a high-dimensional nonlinear dynamical substrate for computation, then FORCE is very much in the spirit of reservoir computing: the random recurrent network is still the substrate, and the learning is confined to a single linear output layer (the FORCE update trains only the feedback weights, which are a linear readout from the network state to the input of the feedback signal).

Sussillo and Abbott were explicit about this: FORCE can be seen as training a linear readout, but the readout's output is fed back into the network, making the system nonlinearly coupled. The recurrent weights $W$ are never changed; only the output/feedback weights are updated.

---

*Prerequisites: Linear algebra (rank-1 updates, matrix inversion lemma), basic control theory (closed-loop stability) at an introductory level. The RLS algorithm is derived from scratch.*
