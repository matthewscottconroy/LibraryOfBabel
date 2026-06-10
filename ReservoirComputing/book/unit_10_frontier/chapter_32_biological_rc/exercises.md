# Chapter 32 Exercises

**Exercise 32.1.** *(Reservoir interpretation of DishBrain)* Model the DishBrain experiment as a reservoir computer:
(a) Specify what the "reservoir," "input weights," "readout," and "target" correspond to in the physical experiment.
(b) Identify two alternative explanations (other than "learning") for the above-chance performance, and describe what experimental controls would distinguish them.
(c) Design an experiment that would provide stronger evidence for task-specific learning vs. non-specific adaptation.

**Exercise 32.2.** *(Heterogeneous reservoir)* Implement a heterogeneous reservoir where each neuron $i$ has its own time constant $\tau_i$, drawn from a log-uniform distribution on $[0.01, 1.0]$ (seconds). The update rule is:
$$x_i(t+1) = (1 - 1/\tau_i) x_i(t) + \tanh\!\left(\sum_j W_{ij} x_j(t) + w_{\text{in},i} u(t)\right)/\tau_i.$$
Compare the performance of this heterogeneous reservoir to a homogeneous reservoir (all $\tau_i = \tau$) on: (a) NARMA-10, (b) a multi-scale temporal task with features at both fast (lag 1-5) and slow (lag 50-100) timescales. What do the results suggest about when heterogeneity helps?

**Exercise 32.3.** *(Short-term plasticity)* Add short-term synaptic depression to a reservoir: each synaptic weight $W_{ij}$ is multiplied by a dynamic factor $d_{ij}(t) \in [0,1]$ that decreases when the presynaptic neuron fires and recovers exponentially:
$$d_{ij}(t+1) = 1 - (1 - d_{ij}(t)(1 - U_d))e^{-\Delta t/\tau_d},$$
where $U_d = 0.2$ (fraction used per spike) and $\tau_d = 0.2$s.
(a) Implement this and test on a task requiring detection of *changes* in input frequency (a task that benefits from high-pass temporal filtering).
(b) Show analytically that short-term depression implements approximate differentiation: the effective output of a depressing synapse is $\approx W_{ij} \cdot du/dt$ at steady state.

**Exercise 32.4.** *(Stochastic reservoir)* Add Gaussian noise to reservoir states: $x(t+1) = \tanh(Wx(t) + W_{\text{in}}u(t)) + \sigma\xi(t)$ where $\xi(t) \sim \mathcal{N}(0, I)$. Test performance as a function of $\sigma$ on the NARMA-10 task. Do you observe stochastic resonance (performance improving at intermediate $\sigma$)? Characterize the optimal noise level as a function of the spectral radius.

**Exercise 32.5.** *(Small-world reservoir)* Construct a small-world reservoir using the Watts-Strogatz rewiring model [Watts1998]: start with a ring lattice (each neuron connected to $k$ nearest neighbors), then rewire each edge with probability $p$ to a random neuron. For $N = 200$ neurons, $k = 10$, vary $p$ from 0 (ring lattice) to 1 (random graph). Compute the clustering coefficient $C(p)$ and average path length $L(p)$. Identify the small-world regime ($C \gg C_{\text{random}}$ but $L \approx L_{\text{random}}$). Test reservoir performance at each $p$. Where is performance maximized?

**Exercise 32.6.** *(Philosophical analysis)* Consider the following thought experiment: a reservoir computer is implemented using cultured cortical neurons from a human iPSC line. The reservoir successfully predicts chaotic time series. The manufacturer describes the device as having "learned" the task.
(a) From the perspective of biological naturalism, functionalism, and IIT, evaluate whether this device might have morally relevant sentience.
(b) What additional experiments would each theory require to determine the device's moral status?
(c) If there is 10% probability that the device has morally relevant sentience, what obligations does this place on the researcher? On the manufacturer?

**Exercise 32.7.** *(Ethics of organoid research)* Read the DishBrain paper [Kagan2022] and two of the following: [Trujillo2019], a neuroscience ethics paper on organoid sentience, and [Munsie2022]. Write a 1000-word analysis addressing:
(a) What did each study actually demonstrate?
(b) What ethical framework does each study implicitly or explicitly adopt?
(c) What additional ethical guidelines would you recommend for biological RC research?

**Exercise 32.8.** *(Neuromorphic implementation)* Research the Intel Loihi 2 neuromorphic chip. Identify three specific design features of Loihi 2 that are inspired by biological neural systems. For each feature, explain: (a) what the biological counterpart is, (b) how it is implemented in silicon, and (c) what computational advantage it provides compared to a conventional GPU implementation of the same reservoir.

**Exercise 32.9.** *(Research problem — optimal biological topology)* The "small-world" and "scale-free" topologies are often cited as biologically plausible and computationally advantageous, but the evidence for their advantage in reservoir computing is mixed. Design a rigorous computational study to resolve this question: propose a set of topology classes to compare, a set of tasks that would differentiate between topologies, and statistical methods for establishing significance. What confounds must you control for (e.g., total number of connections, spectral radius)?
