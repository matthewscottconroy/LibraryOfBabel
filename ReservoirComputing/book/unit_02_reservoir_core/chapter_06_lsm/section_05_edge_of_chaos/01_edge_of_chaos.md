# Section 5: The Edge of Chaos

## 5.1 The Phase Transition in Neural Networks

Random recurrent networks of neurons — whether spiking (LSMs) or rate-coded (ESNs) — exhibit a phase transition as the strength of their recurrent connectivity is increased. Below a critical connectivity, the network is in an **ordered phase**: perturbations decay, dynamics are stable, and the network has little intrinsic activity. Above the critical connectivity, the network is in a **chaotic phase**: perturbations grow exponentially, dynamics are sensitive to initial conditions, and the network generates complex autonomous activity.

At the boundary between these phases — the **critical point** or **edge of chaos** — the network exhibits scale-free dynamics, long-range temporal correlations, and (crucially for computation) the highest information processing capacity [Bertschinger2004].

This section develops the theory of this phase transition for networks of binary neurons (the simplest case where everything can be computed analytically), and then generalizes to continuous-activation and spiking networks.

---

## 5.2 Binary Networks and the Order Parameter

Consider $N$ binary neurons $\sigma_i(t) \in \{0, 1\}$ ($i = 1, \ldots, N$). At each time step, each neuron independently fires ($\sigma_i = 1$) with probability $f_i(t)$ determined by its input:

$$f_i(t) = g\!\left(\sum_j w_{ij} \sigma_j(t-1) + b_i + \xi_i\right) \tag{5.1}$$

where $g : \mathbb{R} \to [0,1]$ is a sigmoidal gain function, $\xi_i \sim \mathcal{N}(0, \sigma_{noise}^2)$ is noise, and $w_{ij} \sim \mathcal{N}(0, J^2/(N \cdot K))$ for $K$ connections per neuron.

The key parameter is the **effective coupling strength** $J$. For small $J$, the network is in the ordered phase. For large $J$, it is in the chaotic phase.

**Order parameter.** Define the **activity correlation** between two replicas of the network driven by the same input but starting from different initial states:

$$Q(t) = \frac{1}{N} \sum_i \langle \sigma_i^{(1)}(t) \sigma_i^{(2)}(t) \rangle \tag{5.2}$$

where the superscripts (1), (2) denote the two replicas. In the ordered phase, $Q \to 1$ (both replicas converge to the same trajectory — ESP holds). In the chaotic phase, $Q < 1$ (replicas diverge — ESP fails).

The transition between the two phases is the **critical point**, where $Q$ just barely equals 1 for the unperturbed network.

---

## 5.3 The Lyapunov Exponent as Order Parameter

For continuous dynamical systems, the natural order parameter is the **maximal Lyapunov exponent** $\lambda_{max}$. For a discrete-time system $x_{t+1} = F(x_t; u_t)$:

$$\lambda_{max} = \lim_{T \to \infty} \frac{1}{T} \ln \|DF^T(x_0)\| \tag{5.3}$$

where $DF^T$ is the Jacobian of the $T$-step map. 

- $\lambda_{max} < 0$: **ordered phase**. Perturbations decay exponentially. ESP holds. Memory: good. Separation: poor.
- $\lambda_{max} = 0$: **critical point** (edge of chaos). Perturbations neither grow nor decay on average.
- $\lambda_{max} > 0$: **chaotic phase**. Perturbations grow exponentially. ESP fails. Memory: poor. Separation: good (locally) but unreliable.

The critical point $\lambda_{max} = 0$ is where the computational capacity is maximized.

**For the ESN** with random $W^{rec}$ and zero input, in the large-$N$ limit, the maximal Lyapunov exponent is:

$$\lambda_{max} = \ln \rho(W^{rec}) + \langle \ln |\tanh'(\cdot)| \rangle \tag{5.4}$$

where the average is over the distribution of activations. Near $\rho = 1$ with small inputs, $|\tanh'| \approx 1$ and $\lambda_{max} \approx \ln \rho(W^{rec})$. So $\lambda_{max} = 0$ corresponds to $\rho = 1$ — the spectral radius condition! This is a beautiful consistency: the "edge of chaos" for the ESN is exactly where $\rho = 1$, confirming the rule of thumb from Chapter 5.

---

## 5.4 The Bertschinger-Natschläger Result

The landmark paper of Bertschinger and Natschläger [Bertschinger2004] ("Real-time computation at the edge of chaos in recurrent neural networks") provided the first rigorous, quantitative demonstration that information processing capacity peaks at the critical point.

**Setup.** They considered a network of $N$ binary neurons with random i.i.d. weights $w_{ij} \sim \mathcal{N}(0, J^2/N)$ and binary inputs $u_t \in \{0, 1\}$ applied to a random subset of neurons. They varied the coupling strength $J$.

**Measurement.** They defined the **information processing capacity** $C$ as a measure of how much mutual information the current network state $x_t$ shares with functions of the input history:

$$C = \sum_{k=0}^{\infty} I(x_t; u_{t-k}) \tag{5.5}$$

This is a sum of mutual informations between the current state and the input at lag $k$. High $C$ means the current state contains information about many past inputs.

**Result.** As $J$ is varied from 0 (no coupling) to large values:
- For $J < J_c$ (ordered phase): $C$ increases with $J$ as the network becomes more responsive to inputs.
- At $J = J_c$ (critical point): $C$ reaches its maximum.
- For $J > J_c$ (chaotic phase): $C$ decreases rapidly as the chaotic dynamics "erase" the input information.

The peak of $C$ at the critical point is the main result. It was observed numerically and supported by theoretical arguments based on the mean-field theory of random networks.

**Why does $C$ peak at the critical point?**
- In the ordered phase ($J < J_c$): the network is "too quiet." Its response to inputs is weak (low gain), and correlations between the state and past inputs decay rapidly. Low memory, low capacity.
- In the chaotic phase ($J > J_c$): the network is "too loud." Its autonomous chaotic dynamics produce strong correlations between $x_t$ and the initial state $x_0$, but weak correlations with the input history. High input sensitivity to recent inputs but chaotic forgetting of older ones.
- At the critical point: the network has long-range correlations (because fluctuations decay as power laws rather than exponentially), maximum sensitivity to inputs (maximum linear amplification), and the best balance between memory and forgetting.

---

## 5.5 Critical Connectivity: Deriving $J_c$

For networks of $N$ neurons with weights $w_{ij} \sim \mathcal{N}(0, J^2/N)$ and sigmoidal gain function $g$ with maximum derivative $g'(0) = \beta$ (the gain), the critical coupling is:

$$J_c = \frac{1}{\beta \sqrt{K}} \tag{5.6}$$

where $K$ is the number of connections per neuron (in-degree).

**Derivation (mean-field).** Consider the linearized dynamics near the zero-activity state. The Jacobian of the map $F$ at $x = 0$ is $J^{Jac}_{ij} = w_{ij} g'(0) = w_{ij} \beta$. The network is stable iff all eigenvalues of $J^{Jac}$ have magnitude $< 1$. By the Wigner semicircle law (since $w_{ij}$ are i.i.d. Gaussian with variance $J^2/N$), the spectral radius of $J^{Jac}$ is:

$$\rho(J^{Jac}) = \beta \cdot \rho(W) = \beta \cdot J \cdot \frac{2}{\sqrt{N \cdot K/N}} = \beta J \sqrt{K} \cdot \frac{2}{\sqrt{K}} $$

Wait — let us be careful. For a sparse random matrix with $K$ nonzero connections per row drawn from $\mathcal{N}(0, J^2/K)$ (so that the total input variance per neuron is $J^2$, independent of $K$), the spectral radius is approximately $J$ for large $N$ (by the sparse random matrix theory). The critical condition $\rho(J^{Jac}) = 1$ gives:

$$\beta J_c = 1 \implies J_c = \frac{1}{\beta} \tag{5.7}$$

More precisely, for dense random matrices ($K = N$, $w_{ij} \sim \mathcal{N}(0, J^2/N)$):

$$J_c = \frac{1}{\beta} \tag{5.8}$$

For sparse random matrices with average in-degree $K$ and $w_{ij} \sim \mathcal{N}(0, J^2/K)$ when connected:

$$J_c = \frac{1}{\beta \sqrt{K}} \quad \text{(approximately, for } K \ll N\text{)} \tag{5.9}$$

Note: some references define $J^2$ as the variance of the sum (rather than individual weights), which changes the normalization by $\sqrt{K}$. Equation (5.9) reflects the convention where $J$ is the "total" coupling strength.

For the sigmoidal activation function $g(x) = \tanh(x)$, the gain at the origin is $\beta = g'(0) = 1$, so:

$$J_c = 1 \tag{5.10}$$

This is the same condition as $\rho(W^{rec}) = 1$ for the ESN — the two derivations converge to the same critical condition.

---

## 5.6 Neural Avalanches and Critical Branching

An independent line of evidence for criticality comes from the observation of **neural avalanches** in cortical slices and in vivo. A neural avalanche is a burst of spontaneous activity where each active neuron activates on average one other neuron — precisely the branching ratio $\sigma = 1$ that characterizes the critical point of a branching process.

**Branching process model.** Model the network as a branching process: each active neuron at time $t$ independently activates $k$ other neurons at time $t+1$, where $k \sim \text{Poisson}(\sigma)$. The parameter $\sigma$ is the **branching ratio**.

- $\sigma < 1$: sub-critical. Activity cascades die out rapidly. Avalanche size distribution: exponential.
- $\sigma = 1$: critical. Activity cascades span all scales. Avalanche size distribution: power law ($P(s) \sim s^{-3/2}$, the Bessel function tree distribution).
- $\sigma > 1$: super-critical. Activity cascades explode exponentially. Network is unstable.

**Experimental observation [Beggs2003].** Beggs and Plenz (2003) measured spontaneous activity in rat cortical slices using multi-electrode arrays. They found that the distribution of avalanche sizes follows a power law $P(s) \sim s^{-3/2}$ — consistent with the critical branching process ($\sigma = 1$). This was the first experimental evidence that cortical networks operate near criticality.

The branching ratio $\sigma$ is directly related to the spectral radius $\rho(W^{rec})$ for rate-coded networks: $\sigma \approx \rho$ in the linear regime. So the experimental observation $\sigma \approx 1$ in cortical tissue corresponds to $\rho \approx 1$ in the ESN formalism — consistent with the "edge of stability" design rule.

---

## 5.7 The Full Picture: Information Transmission Peaks at Criticality

Putting the theoretical and empirical evidence together, the picture is:

| Phase | $J$ / $\rho$ | Lyapunov $\lambda_{max}$ | Memory | Sensitivity | Capacity $C$ |
|-------|-------------|------------------------|--------|------------|------------|
| Ordered | $< J_c$ | $< 0$ | Long | Low | Low |
| Critical | $= J_c$ | $= 0$ | Scale-free | Maximal | **Maximal** |
| Chaotic | $> J_c$ | $> 0$ | Short | High/unstable | Low |

The critical point simultaneously maximizes:
1. **Memory capacity** (the long-range correlations at criticality extend the effective memory time constant to infinity — in principle).
2. **Dynamic range** (the sensitivity to input signals is maximized at the critical gain).
3. **Information transmission** (measured as mutual information between input and output) [Shew2009].

This provides a principled, theoretically grounded explanation for why $\rho \approx 1$ is the optimal operating point for ESNs and why biological neural networks appear to operate near criticality.

---

## 5.8 Designing Reservoirs at the Edge of Chaos

In practice, "operating at the edge of chaos" means different things for ESNs and LSMs.

**For ESNs:** Set $\rho(W^{rec}) \approx 0.95$-$0.99$. The slight margin below 1 ensures the ESP holds while keeping the dynamics near-critical. As we showed in Section 3.5 of Chapter 5, the ESP can hold for $\rho > 1$ if inputs are strong and the nonlinearity saturates, but this is not reliable in practice.

**For LSMs (spiking networks):** Control the critical point via:
- The average number of connections per neuron $K$ (connectivity).
- The average synaptic weight magnitude $J$.
- The balance between excitatory and inhibitory neurons.

A useful heuristic is the **E/I balance** condition (Section 6 of this chapter): maintaining a roughly 4:1 ratio of excitatory to inhibitory neurons, with appropriate weight scaling, keeps the network near the critical point. In the Maass et al. 2002 model, this was ensured by using the experimentally measured parameters for cortical connection statistics.

**Adaptive criticality.** One of the remarkable features of biological neural networks is that they appear to *self-organize* toward the critical point. Synaptic plasticity rules (Hebbian learning, spike-timing-dependent plasticity) can drive a network toward $\sigma = 1$ from any initial state. This is the theory of **self-organized criticality** (SOC) in neural systems [Bak1987], which suggests that the brain's operating point near the edge of chaos is not accidentally set by evolution but is dynamically maintained by ongoing learning.

For reservoir computing, this observation raises the tantalizing possibility of reservoirs that automatically tune themselves to the optimal operating point — though implementing reliable self-organized criticality in artificial reservoir systems remains an active research challenge.
