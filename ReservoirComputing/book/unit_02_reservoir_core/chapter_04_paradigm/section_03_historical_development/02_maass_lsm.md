# 4.3.2 Wolfgang Maass and the Liquid State Machine

## A Different Beginning

Where Jaeger arrived at the reservoir computing idea from the problem of training recurrent networks, Wolfgang Maass arrived at it from a completely different direction: the problem of how biological neural circuits compute.

Maass is a theoretical computer scientist and computational neuroscientist whose primary research interest is the computational theory of biological neural networks. His question in the late 1990s and early 2000s was not "how do we train a recurrent network?" but "how does the brain compute at all?" Specifically: how can a cortical microcircuit, with its chaotic, noisy, highly irregular dynamics and no apparent stable states, perform reliable computations on continuously arriving, time-varying inputs?

This is a deep and still not fully resolved question in computational neuroscience. The dominant computational models of the time — feedforward networks, Hopfield networks, recurrent networks with learned attractors — all seemed to require either the absence of temporal dynamics or the presence of carefully tuned, stable attractor states. Neither seemed biologically realistic. Real cortical circuits are constantly active, never settling to a fixed point, yet they somehow produce reliable behavioral outputs.

Maass's insight was that the cortical microcircuit might be exploiting its own dynamics *as a computational resource* — using the trajectory through its high-dimensional state space as a continuously updated representation of the recent input history, from which multiple readout circuits could extract whatever they needed.

## The Liquid Metaphor

Maass, with Thomas Natschläger and Henry Markram (at EPFL), published the Liquid State Machine framework in *Neural Computation* in 2002 [Maass2002]. The paper's most evocative contribution is not the mathematics but the **metaphor**.

Imagine dropping a stone into a pond. The stone creates a pattern of ripples that spreads across the surface. If you drop two different stones at different locations, they create different but overlapping patterns. The surface of the pond at any given moment encodes, in the interference pattern of all the ripples, the history of all the stones that have been dropped — their locations, their weights, the times they were dropped. A readout mechanism that could "read" the surface state at any moment could in principle extract information about any of these past events.

Now imagine a much more complex liquid — one that responds not just with linear ripples but with turbulent, nonlinear eddies and vortices. Such a liquid would encode the history of all past perturbations in a rich, high-dimensional, nonlinearly mixed representation. Different perturbation histories would produce different surface states. And crucially, the "liquid" itself does not need to be designed or trained — it is simply a physical system with the right kind of dynamics.

The cortical microcircuit, Maass proposed, is such a liquid. The external inputs (sensory signals, top-down feedback) are the stones. The circuit's activity at any moment is the surface state. The readout neurons — projecting to motor cortex, prefrontal cortex, or wherever the output is needed — are the readers of the surface.

The name **Liquid State Machine** captures this metaphor precisely. It is a machine that uses the liquid-like dynamics of its substrate as a computational resource.

## The Mathematical Framework

The mathematical framework of the LSM is more abstract than Jaeger's ESN, reflecting Maass's theoretical computer science background. Rather than specifying a particular network architecture, the LSM framework defines a class of computational systems by three components:

**The Liquid (Filter $L^M$):** Any dynamical system that satisfies two properties:
1. **Separation property (SP):** For any two distinct input functions $u(t) \neq v(t)$, the liquid state $\mathbf{x}_u(t) \neq \mathbf{x}_v(t)$ for almost all $t$. Different inputs produce different states.
2. **Approximation property (AP):** The liquid states can be used to approximate any target function.

**The Readout (Memoryless function $f^M$):** A function that maps the current liquid state to the output. In the LSM framework, this function is required to be *memoryless* — it depends only on the current state $\mathbf{x}(t)$, not on the history of states. This is what makes the liquid's memory implicit: all the temporal context must be in the current state, because the readout has none.

**The Fading Memory Property:** The liquid has fading memory if its response to old inputs decays over time. Formally: for any $\epsilon > 0$, there exists $\delta > 0$ and $T > 0$ such that if $|u(s) - v(s)| \leq \delta$ for all $s \in [-T, 0]$, then $|\mathbf{x}_u(0) - \mathbf{x}_v(0)| \leq \epsilon$.

**The key theorem [Maass2002]:** A liquid with the separation property and the fading memory property can approximate any causal, time-invariant functional with fading memory, to arbitrary accuracy, with a suitable memoryless readout.

This theorem is the LSM's central claim: the computational power of the liquid-readout combination is equivalent to the full class of "nice" (causal, time-invariant, fading memory) temporal functionals. In the language of computer science, the LSM is a universal analog computer for this function class.

## The Biological Motivation

Maass's motivation for the LSM was explicitly biological, and the 2002 paper contains a section demonstrating that simulated cortical microcircuits — with biologically realistic parameters, including excitatory and inhibitory neurons, conductance-based synapses, and Poisson spike trains — could serve as the liquid in an LSM.

The cortical microcircuit simulations were parameterized by Markram's group at EPFL using data from actual cortical tissue. The circuits were not designed to compute; they were parameterized to match biology. And yet, Maass and colleagues showed, they satisfied the separation property and the fading memory property for typical inputs. A linear readout trained on the spike train activity of these simulated circuits could successfully perform various temporal classification tasks.

This was a qualitatively new kind of theoretical result: not just "this architecture can compute this function" but "this biological system, with these realistic parameters, satisfies these computational properties" — linking the theoretical framework to actual neuroscience data.

Henry Markram's involvement was crucial here. Markram, who would later lead the Human Brain Project, was (and is) one of the world's leading experts on cortical microcircuit connectivity and synaptic dynamics. His group's data on the structure of layer 2/3 cortical circuits provided the biological substrate for Maass's computational demonstration.

## The Initial Reception and Subsequent Convergence

The LSM paper was received enthusiastically in the computational neuroscience community, where its biological motivation resonated, and with interest (and some skepticism) in the machine learning community. The machine learning concern was practical: the LSM framework, as initially formulated, was more of a theoretical existence proof than an engineering recipe. It showed that a liquid with the right properties could compute anything, but it did not specify how to build such a liquid or how to tune its parameters.

The connection between Jaeger's ESN and Maass's LSM was recognized quickly — within a year of the LSM paper's publication, researchers had begun using the terms interchangeably and pointing out that the two architectures were, at the mathematical level, nearly identical. The key differences were:

- **Language:** ESN used the language of control theory and signal processing; LSM used the language of theoretical computer science and computational neuroscience.
- **Focus:** ESN focused on practical engineering (how to build and train a working system); LSM focused on theoretical characterization (what conditions guarantee computational universality).
- **Biological grounding:** LSM explicitly grounded itself in cortical neuroscience; ESN was agnostic about biological relevance.

The unification under the "reservoir computing" label [Verstraeten2007] acknowledged this convergence and allowed the field to move forward with a shared vocabulary. In practice, the "reservoir" in most modern work combines elements of both: the practical simplicity of the ESN construction, the theoretical framework of the LSM, and an increasingly sophisticated understanding of how both relate to dynamical systems theory, random matrix theory, and information theory.

## What Maass's Contribution Added

Maass's specific intellectual contribution, beyond the co-discovery of the basic architecture, was the rigorous characterization of the *sufficient conditions* for universal computation. The separation property and the approximation property are not vague intuitions; they are formal mathematical conditions that can be verified for specific systems. This gave the reservoir computing field a theoretical foundation that was more precise than "random weights seem to work."

More concretely: Maass's framework makes clear that the reservoir computing paradigm is not specifically about neural networks, or even about digital computation. Any physical system with a sufficiently rich, high-dimensional, input-sensitive state that satisfies the separation and fading memory properties can serve as a liquid. This insight — that the "liquid" can be literally any physical system — is the conceptual foundation for the physical reservoir computing paradigm that we will explore in Unit VII.

---

## References

- [Maass2002] Maass, W., Natschläger, T., & Markram, H. (2002). Real-time computing without stable states: A new framework for neural computation based on perturbations. *Neural Computation*, 14(11), 2531–2560.
- [Maass2004] Maass, W., Natschläger, T., & Markram, H. (2004). Fading memory and kernel properties of generic cortical microcircuit models. *Journal of Physiology Paris*, 98(4–6), 315–330.
- [Verstraeten2007] Verstraeten, D., Schrauwen, B., d'Haene, M., & Stroobandt, D. (2007). An experimental unification of reservoir computing methods. *Neural Networks*, 20(3), 391–403.
- [Maass2011] Maass, W. (2011). Liquid state machines: Motivation, theory, and applications. In *Computability in Context* (pp. 275–296). World Scientific.
