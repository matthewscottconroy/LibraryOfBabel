# Exercises — Chapter 16: Photonic Neurons and Synapses

## Mathematical Exercises

**16.1** (Linear stability of the laser neuron) A single-mode excitable laser neuron can be written, in normalized photon-number $S$ and carrier-number $N$ variables, as

$$\dot{S} = \big[g(N - N_{tr}) - \tfrac{1}{\tau_p}\big]S, \qquad \dot{N} = \frac{I}{e} - \frac{N}{\tau_s} - g(N-N_{tr})\,S,$$

with photon lifetime $\tau_p \approx 2$ ps, carrier lifetime $\tau_s \approx 1$ ns, and gain slope $g$. The neuron is biased just below threshold so that the quiescent state carries a small sub-threshold photon number $S_0$.

(a) Find the below-threshold fixed point $(S_0, N_0)$ for a bias current $I$ just under the threshold current $I_{th}$.

(b) Linearize the two equations about $(S_0, N_0)$ and write the $2\times 2$ Jacobian $J$.

(c) Show that near threshold the eigenvalues form a complex-conjugate pair $\lambda_\pm = -\tfrac{\gamma}{2} \pm i\,\omega_R$, and identify the relaxation-oscillation frequency $\omega_R$ and the damping rate $\gamma$ in terms of $\tau_p$, $\tau_s$, and $S_0$. Interpret the fixed point as a stable spiral (damped focus).

(d) The boundary between an *excitable* neuron (all perturbations decay, so isolated spikes are possible) and a *self-oscillatory* laser (a limit cycle, continuous pulsation) is the Hopf condition $\gamma \to 0$. Using your expression for $\gamma$, state the parameter condition that separates the two regimes, and explain why excitable operation requires $\gamma > 0$ (damping) but only weakly so.

---

**16.2** (Steady-state activity of a random photonic SNN) Consider an Erdős–Rényi random spiking network of $N$ laser neurons in which each ordered pair is connected with independent probability $p$. A fraction $f_E$ of neurons are excitatory and $f_I = 1 - f_E$ inhibitory. Each neuron has mean fan-in $K = p(N-1)$.

(a) Show that the expected number of synapses scales as $pN^2$ and evaluate it for $N = 100$, $p = 0.1$.

(b) In a mean-field ("balanced network") approximation the population firing rate $\nu$ obeys a self-consistency relation $\nu = \Phi\big(K[\,f_E - g\,f_I\,]\,\nu\,w\big)$, where $w$ is a typical synaptic weight, $g$ the relative inhibitory strength, and $\Phi$ the single-neuron $f$–$I$ transfer function. Explain why $f_E - g f_I \approx 0$ (approximate excitation–inhibition balance) is required for a stable, low, non-saturated firing rate, and what happens to $\nu$ if $f_E - g f_I > 0$.

(c) Estimate the aggregate spike rate of the network (spikes/s) if each neuron fires at $\nu = 1$ GHz — a rate accessible to laser neurons but not to biology — for $N = 1000$. Compare with the same network of biological neurons firing at $\nu = 50$ Hz.

(d) The total optical power the shared WDM bus must carry scales with the aggregate spike rate times the energy per spike. Using an energy per spike of $1$ pJ, estimate the average bus power in each case and comment on which quantity — power or latency — is the binding constraint for the photonic network.

---

**16.3** (Multi-level PCM synapse: precision and SNR) A phase-change synapse is programmed to $L = 8$ equally spaced transmission levels between $T_{\min} = 0.1$ (fully crystalline) and $T_{\max} = 0.9$ (fully amorphous). The read-out transmission is corrupted by Gaussian noise with standard deviation $\sigma = 0.02$.

(a) What weight precision, in bits, do 8 levels represent?

(b) Compute the level spacing $\Delta T = (T_{\max}-T_{\min})/(L-1)$ and the per-level signal-to-noise ratio $\Delta T/\sigma$ in dB.

(c) A read error occurs when noise exceeds half the level spacing. Using $P_e = 2\,Q(\Delta T/2\sigma)$ with $Q$ the Gaussian tail function, estimate the per-level misclassification probability.

(d) Treating the full transmission swing as the signal and $\sigma$ as the noise, compute the overall SNR in dB and the effective number of bits $\mathrm{ENOB} = (\mathrm{SNR}_{\text{dB}} - 1.76)/6.02$. How many levels could in principle be resolved, and how much margin does the 8-level programming leave? Comment on how transmission *drift* over time would erode this margin.

---

**16.4** (STDP and winner-take-all competition) A single LIF-type output neuron receives spike trains from two input synapses with weights $w_1, w_2$, updated by the STDP rule

$$\Delta w = \begin{cases} A_+ e^{-\Delta t/\tau_+}, & \Delta t = t_{\text{post}}-t_{\text{pre}} > 0 \\ -A_- e^{\,\Delta t/\tau_-}, & \Delta t < 0, \end{cases}$$

with $A_+ = 0.010$, $A_- = 0.012$, $\tau_+ = 17$ ms, $\tau_- = 34$ ms, and weights clipped to $[0, w_{\max}]$.

(a) Input 1 reliably fires $5$ ms *before* each postsynaptic spike; input 2 fires $10$ ms *after*. Compute $\Delta w_1$ and $\Delta w_2$ per pairing.

(b) Show that repeated pairing drives $w_1 \to w_{\max}$ and $w_2 \to 0$: the synapse that is causal (predictive of the output) is potentiated and the anti-causal one is depressed.

(c) Show, using the integrated window areas $A_+\tau_+$ and $A_-\tau_-$, that this rule is net-depressive and therefore self-stabilizing (it does not run away to saturation for uncorrelated inputs).

(d) Argue that in a layer of output neurons sharing the same inputs, this dynamics implements winner-take-all competitive learning: each output neuron specializes to a distinct input pattern. What role does lateral inhibition play?

---

## Conceptual Exercises

**16.5** (Photonic-SNN vs. GPU inference energy) Estimate the energy to perform one forward pass of a 10-layer, fully connected SNN with 1000 neurons per layer.

(a) Count the synaptic operations per forward pass (assume full connectivity between adjacent layers). 

(b) *GPU baseline.* Assume an effective energy of $20$ pJ per multiply–accumulate (including memory movement, which dominates in practice). Estimate the GPU energy per inference.

(c) *Photonic SNN.* Assume the weights are stored in passive, non-volatile PCM (the weighting itself is "free"), each neuron fires on average once per inference, and each spike costs $1$ pJ to generate and detect (O–E–O included). Estimate the photonic energy per inference.

(d) State the ratio, then list at least three assumptions that make the photonic figure optimistic — including the always-on laser wall-plug power and per-ring thermal-stabilization power that this event-driven accounting ignores. How would including a $1$ mW/ring hold power for $10^6$ rings change the picture?

---

**16.6** (Why a synapse must be non-volatile) A deployed inference accelerator holds its weights fixed for millions of inferences.

(a) Explain why storing weights in a *volatile* element (e.g., a thermally tuned microring that must be actively held on resonance) imposes a static power floor, and estimate that floor for $10^4$ rings at $\sim 1$ mW each.

(b) Explain how a non-volatile PCM synapse removes this floor entirely — zero power to *hold* a weight — and why the large one-time *write* energy (on the order of nanojoules) is nonetheless acceptable for a fixed-weight deployment.

(c) Identify one application regime where the volatile, reconfigurable approach is preferable despite its power cost.

---

## Programming Projects

**Project 16.1: Photonic LIF neuron ODE simulation**

Simulate an excitable laser neuron and verify its LIF-like behavior.

(a) Integrate the two-variable rate equations of Exercise 16.1 (use $\tau_p = 2$ ps, $\tau_s = 1$ ns) with an adaptive-step ODE solver. Bias just below threshold and inject a Gaussian optical pulse of tunable energy.

(b) Sweep the input pulse energy and show the all-or-nothing threshold: below a critical energy the perturbation decays; above it, one full optical spike is emitted. Plot output spike energy vs. input energy.

(c) Deliver two pulses separated by a variable interval and map out the refractory period — the minimum interval at which a second spike can still be triggered, and the reduced-amplitude "relative refractory" regime at intermediate intervals.

(d) Add Langevin noise to the field equation and measure the jitter in spike timing vs. input amplitude near threshold.

**Skills practiced**: stiff ODE integration, excitable-system dynamics, threshold and refractory characterization, stochastic simulation.

---

**Project 16.2: Multi-level PCM programming simulator**

Model the analog programming of a phase-change synapse.

(a) Model the waveguide transmission as $T = \exp(-\alpha_c\, m\, L)$ where $m \in [0,1]$ is the crystalline fraction and $\alpha_c$ the crystalline absorption. Implement a program-and-verify loop that applies pulses to drive $T$ toward a target level.

(b) Add programming stochasticity (nucleation is random): each pulse changes $m$ by a noisy increment. Show how closed-loop verify-after-write reaches a target within tolerance in a few iterations, whereas open-loop programming does not.

(c) Add a transmission *drift* model (slow relaxation of $m$ toward equilibrium) and quantify how many distinct levels remain reliably separable after $1$ hour, $1$ day, and $1$ year.

(d) Simulate finite endurance by adding cumulative variability that grows with cycle count; estimate the usable cycle count before adjacent levels overlap.

**Skills practiced**: closed-loop control, stochastic device modeling, drift/endurance analysis, level-resolution trade-offs.

---

**Project 16.3: STDP-trained photonic SNN on MNIST**

Build a small spiking classifier trained with an optical-STDP-like rule.

(a) Encode MNIST digits as spike trains (rate or latency coding) and build a single spiking layer with PCM-weighted synapses (weights clipped to a physical transmission range).

(b) Implement the STDP rule of Exercise 16.4 with lateral inhibition between output neurons, and train unsupervised on a subset of digits.

(c) Assign labels to the learned output neurons by their preferred class and report test accuracy; compare rate vs. latency input encoding.

(d) Impose the hardware constraints of a real PCM synapse — 5-bit weights, drift, and write stochasticity from Project 16.2 — and measure the accuracy degradation.

**Skills practiced**: spike encoding, unsupervised competitive learning, hardware-aware quantization, benchmarking.

---

**Project 16.4: Photonic reservoir SNN**

Explore a spiking reservoir as a lighter-weight alternative to trained deep SNNs.

(a) Construct a fixed, randomly connected recurrent pool of LIF/laser neurons (the reservoir) and drive it with an input time series; train only a linear readout on the reservoir states. The reservoir-computing machinery itself — single-node time-delay reservoirs and integrated microring reservoirs — is developed in Unit V (§13.4); reuse it here with spiking nodes rather than continuous nonlinearities.

(b) Tune the reservoir's spectral radius / gain to the "edge of chaos" and show the effect on memory capacity and task performance (e.g., on a spoken-digit or NARMA benchmark).

(c) Compare the training cost (readout only) against the full BPTT/surrogate-gradient training of Project 16.3.

(d) Discuss which photonic substrate — an excitable-laser reservoir vs. a trained broadcast-and-weight SNN — you would choose for a low-power edge classifier, and why.

**Skills practiced**: reservoir computing, recurrent spiking dynamics, linear readout training, architecture trade-off analysis.

---

*These exercises span the four modes of photonic-computing work: analytic derivation (16.1–16.4), system-level reasoning (16.5–16.6), and hardware-aware simulation (Projects 16.1–16.4).*
