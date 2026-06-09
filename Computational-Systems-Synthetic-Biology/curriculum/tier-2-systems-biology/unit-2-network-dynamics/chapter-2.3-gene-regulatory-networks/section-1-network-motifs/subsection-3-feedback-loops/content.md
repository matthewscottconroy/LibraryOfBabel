# Feedback Loops

## Types and Fundamental Properties

There is a single question that, if answered, tells you most of what you need to know about a regulatory circuit's behavior: does it have feedback, and if so, what is the sign? Negative feedback creates homeostasis. Positive feedback creates memory. This is not a rule of thumb — it is a theorem that follows directly from the mathematics of dynamical systems.

A **feedback loop** occurs when a regulatory path from gene A influences gene A's own expression through one or more intermediaries. The qualitative behavior of the feedback — generating homeostasis or bistability — depends entirely on the sign of the overall loop:

- **Negative feedback loop**: an odd number of repressing edges in the cycle
- **Positive feedback loop**: an even number of repressing edges (or all activating)

This classification is not merely descriptive — it predicts the dynamical repertoire of the system.

## Negative Feedback Loops

### Core Properties

A negative feedback loop drives a system back toward a set point when perturbed. This generates:

1. **Homeostasis**: the system resists deviations from equilibrium
2. **Robustness**: steady-state level is insensitive to perturbations in production rate
3. **Potential for oscillation**: if the feedback path has sufficient delay or involves nonlinear amplification

That third property is worth dwelling on. Negative feedback with delay can produce oscillations — counterintuitive but mathematically precise. The delay turns the thermostat into a pendulum. Rather than correcting the error smoothly, the system overcorrects, then overcorrects in the other direction, and the resulting overshoot cycles are oscillations. This is not a defect of negative feedback; it is an exploitable feature, as the circadian clock amply demonstrates.

### The Goodwin Oscillator

The simplest biological oscillator is the three-gene negative feedback loop:

$$\frac{dx}{dt} = \frac{\alpha}{1 + z^n} - \delta x$$
$$\frac{dy}{dt} = \beta x - \delta y$$
$$\frac{dz}{dt} = \gamma y - \delta z$$

Here X activates Y, Y activates Z, and Z represses X — closing the negative feedback loop. This is the **Goodwin oscillator** (1965).

The system oscillates if and only if the Hill coefficient satisfies:
$$n > \frac{8 \cos^3(\pi/3)}{1 - 2\cos(\pi/3)} \approx 8$$

(for equal degradation rates). This seemingly high threshold reflects the fact that oscillations require a steep enough nonlinearity to overcome the stabilizing tendency of negative feedback. In biological systems, post-translational modifications (multiple phosphorylation steps) provide effective cooperativity far exceeding what a simple Hill function describes.

### The Repressilator

The **repressilator** (Elowitz & Leibler 2000) is the experimental realization of a three-gene negative feedback ring in *E. coli*:

```
lacI ⊣ tetR ⊣ cI ⊣ lacI
```

Each TF represses the next, and the last represses the first. The three proteins oscillate with a period of approximately 150 minutes — far longer than the protein half-lives (30-60 min), demonstrating that the period is set by the regulatory circuit, not by individual molecular properties.

The repressilator was a landmark result because it showed that rational circuit design could produce predictable oscillatory dynamics in living cells. However, the original version showed substantial cell-to-cell variability in period and amplitude, motivating improved designs with additional feedback to synchronize oscillations (Potvin-Trottier et al. 2016).

### Negative Feedback and Oscillation: General Conditions

For a linear negative feedback loop of length $n$, oscillations require that the open-loop gain exceed unity at the critical frequency $\omega_c$:

$$|G(i\omega_c)| > 1, \quad \angle G(i\omega_c) = -\pi$$

Biological nonlinearity (sigmoidal response curves, ultrasensitivity) makes this condition easier to satisfy. The period of oscillation scales with the total delay in the feedback loop — longer pathways produce longer-period oscillations.

### Integral Feedback and Perfect Adaptation

There is a third property that negative feedback can confer, beyond homeostasis and oscillation. When a negative feedback loop contains an **integrating element** — one whose dynamics accumulate the error signal over time — the system achieves **perfect adaptation**: the output returns exactly to its set point regardless of the magnitude of a step perturbation. Not approximately — exactly. This is a remarkable property, and it is not the result of fine-tuning; it is a structural consequence of having an integrator in the loop.

When a negative feedback loop contains an **integrating element** — one whose dynamics accumulate the error signal over time — the system achieves **perfect adaptation**: the output returns exactly to its set point regardless of the magnitude of a step perturbation.

In *E. coli* chemotaxis, the chemoreceptor methylation system provides integral feedback:
- Output = tumbling frequency (CheA kinase activity)
- Error = deviation from set-point tumbling rate
- Integrator = CheR (methylates receptors at constant rate) + CheB-P (demethylates proportionally to CheA activity)

At any steady-state attractant concentration, CheB-P activity adjusts methylation to return tumbling rate to set point. This is provably perfect adaptation — a consequence of the integral feedback structure, not parameter tuning.

## Positive Feedback Loops

### Core Properties

Where negative feedback restores, positive feedback amplifies. A positive feedback loop amplifies deviations from equilibrium, generating:

1. **Bistability**: two stable steady states (ON and OFF)
2. **Hysteresis**: the input level required to switch ON differs from the level required to switch OFF
3. **Memory**: the state persists after the inducing signal is removed
4. **Switch-like transitions**: small inputs produce large, binary responses

These properties make positive feedback the molecular substrate of decision-making. Whenever a biological system must choose irreversibly between two states — commit to a cell fate, enter mitosis, lock in a phage lifecycle — the circuit almost always contains a positive feedback loop. Hysteresis is the memory mechanism: the system remembers which state it entered, and does not return to the OFF state just because the inducing signal has faded.

### Mathematical Conditions for Bistability

For a single-gene positive autoregulation model:
$$\frac{dp}{dt} = \alpha_0 + \frac{\alpha p^n}{K^n + p^n} - \delta p$$

Bistability requires the production curve $f(p) = \alpha_0 + \alpha p^n/(K^n + p^n)$ to intersect the degradation line $\delta p$ at three points. This requires:

1. Hill coefficient $n \geq 2$ (cooperativity)
2. Sufficiently strong feedback: $\alpha > 4\delta K/(n-1)$ (approximately, for large $n$)
3. Small leaky expression $\alpha_0$ (high leakiness shrinks the bistable region)

### Mutual Repression: A Robust Bistable Switch

The most robust bistable switch motif uses two mutually repressing TFs (A and B):

$$\frac{da}{dt} = \frac{\alpha}{1 + (b/K)^n} - \delta a$$
$$\frac{db}{dt} = \frac{\alpha}{1 + (a/K)^n} - \delta b$$

This **toggle switch** (Gardner et al. 2000) has two stable states: (A high, B low) and (A low, B high). Positive input to A drives the system toward the A-high state; positive input to B drives it toward the B-high state. The switch is bistable for $n \geq 2$.

The toggle switch was one of the first synthetic genetic devices, built by Collins and colleagues using the repressors LacI and TetR in *E. coli*. It demonstrated that bistability and memory could be engineered from first principles.

### Long-Range Positive Feedback: Cell Fate Circuits

In developmental biology, positive feedback loops spanning multiple nodes create stable attractors corresponding to cell fates. The GATA1/PU.1 system in hematopoiesis involves:
- GATA1 activating its own expression (PAR of GATA1)
- PU.1 activating its own expression (PAR of PU.1)
- GATA1 repressing PU.1
- PU.1 repressing GATA1

This double-negative feedback (equivalent to a positive loop) with autoactivation creates two stable attractors: erythroid (GATA1 high, PU.1 low) and myeloid (PU.1 high, GATA1 low). Commitment to a cell fate corresponds to transition between attractors — a one-way event under normal developmental signals.

The two-gene toggle, once grasped, can be read everywhere in developmental biology: NANOG/GATA6 in the embryo, MyoD/Id in muscle, Snail/E-cadherin in epithelial-mesenchymal transition. The topology is the same; only the molecular players differ.

## Why This Matters

Feedback loop analysis provides a powerful shortcut: simply identifying the sign of a regulatory cycle tells you whether to expect homeostasis/oscillations (negative) or bistability/memory (positive). This principle scales from bacterial transcription networks to mammalian developmental circuits to synthetic biology designs. Before building or analyzing any regulatory network, identify its feedback loops and classify their signs — this predicts the fundamental dynamical repertoire of the system.
