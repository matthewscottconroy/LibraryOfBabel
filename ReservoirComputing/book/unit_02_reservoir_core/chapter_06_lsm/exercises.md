# Chapter 6: Exercises

## Part A: Conceptual Exercises

**A1. The f-I Curve of a Leaky Integrate-and-Fire Neuron.**

For a LIF neuron with $C_m = 1$, $R_m = 10$ M$\Omega$, $V_{rest} = 0$, $V_{th} = 1$, $V_{reset} = 0$, $\tau_{ref} = 2$ ms, driven by constant current $I$:

(a) Compute the membrane time constant $\tau_m = R_m C_m$ (in units where $C_m$ is in nF and $R_m$ in M$\Omega$, so $\tau_m$ is in ms).

(b) Find the rheobase current $I_{th}$ (the minimum current for sustained firing).

(c) Derive the f-I curve $f(I)$ for $I > I_{th}$ (equation 2.7). Express your answer in Hz (spikes/second) as a function of $I$ in nA.

(d) Plot the f-I curve for $I \in [0, 5 I_{th}]$. What is the maximum firing rate (saturation level)? What drives the saturation?

(e) Compute $df/dI$ at $I = 1.5 I_{th}$ and at $I = 5 I_{th}$. How does the "gain" of the neuron change with firing rate?

(f) Compare the LIF f-I curve to the simpler approximation $f(I) = \beta [I - I_{th}]_+$ (a linear threshold function). For which values of $I$ is this a good approximation?

---

**A2. Tsodyks-Markram Synapse: Facilitation vs. Depression.**

Consider a synapse with parameters:

**Depressing:** $U = 0.5$, $\tau_D = 800$ ms, $\tau_F = 10$ ms, $w = 1$.
**Facilitating:** $U = 0.05$, $\tau_D = 100$ ms, $\tau_F = 1000$ ms, $w = 1$.

(a) For each synapse, compute the steady-state released fraction $u \cdot R$ under periodic firing at rate $r$ Hz. Solve the TM equations at steady state (set time derivatives to zero and evaluate at each spike). Express your answer as a function of $r$.

(b) Plot the steady-state efficacy $u \cdot R$ vs. firing rate $r \in [0, 100]$ Hz for both synapses. Which acts as a high-pass filter? Which acts as a low-pass filter?

(c) For the depressing synapse, compute the ratio of the efficacy at 50 Hz to the efficacy at 10 Hz. Is this ratio $> 1$ or $< 1$? What does this mean for the role of this synapse in the network?

(d) Suppose we want a synapse that is equally strong at all firing rates (a "flat" filter). What combination of $U$, $\tau_D$, $\tau_F$ would achieve this? Is this biologically reasonable?

---

**A3. The Separation Property and Kernel Quality.**

An LSM has $N = 100$ neurons. We run it with $M = 20$ different input sequences and record the liquid state at time $T$ for each: $x^{(1)}, \ldots, x^{(20)} \in \mathbb{R}^{100}$.

(a) Construct the $20 \times 100$ matrix $X$ with rows $x^{(m)\top}$. What is the maximum possible rank of $X$?

(b) Suppose $\text{rank}(X) = 10$. What does this say about the separation property? How many input sequences can be perfectly distinguished?

(c) Two LSMs are compared: one with full rank (rank 20) and one with rank 10. For a linear readout with $M = 20$ classes, which can achieve zero training error? Why?

(d) The **kernel quality** can be measured as $\kappa = \det(X X^\top) / \|X\|^{2M}$ (a normalized version of the Gram matrix determinant). Explain why this quantity is high when the rows of $X$ are "spread out" and low when they are "clustered." (Hint: think about what $\det(XX^\top)$ measures geometrically.)

---

**A4. Separation Implies Approximation for Linear Readouts.**

Let the liquid produce states $x^{(1)}, \ldots, x^{(M)} \in \mathbb{R}^N$ for $M$ input sequences, and suppose the targets are $y^{(1)}, \ldots, y^{(M)} \in \mathbb{R}$.

(a) Show that if $\{x^{(m)}\}$ are linearly independent (i.e., no $x^{(m)}$ is a linear combination of the others), then there exists a vector $w \in \mathbb{R}^N$ such that $w^\top x^{(m)} = y^{(m)}$ for all $m = 1, \ldots, M$, regardless of the values $y^{(m)}$.

(b) What does this say about the approximation property for linear readouts?

(c) Now suppose the liquid states are not linearly independent: $\text{rank}(X) = r < M$. For which target vectors $y = (y^{(1)}, \ldots, y^{(M)})$ does a perfect linear readout still exist? For which targets does no perfect linear readout exist?

(d) In practice, $M \gg N$ (many more time steps than neurons). Show that in this case, the system is overdetermined and a perfect readout generally does not exist. What is the best we can do? (Hint: least squares.)

---

**A5. The Edge of Chaos: Critical Connectivity.**

A random network has $N = 1000$ binary neurons with i.i.d. weights $w_{ij} \sim \mathcal{N}(0, J^2/N)$ and sigmoidal activation with gain $\beta = 1$.

(a) According to equation (5.8), what is the critical coupling $J_c$ for this network?

(b) For $J = 0.8 J_c$: is the network in the ordered or chaotic phase? What is the expected value of the maximal Lyapunov exponent (approximately)?

(c) For $J = 1.2 J_c$: same question.

(d) Suppose we increase $\beta$ (e.g., by shifting the working point of the neurons to a steeper part of the sigmoid). How does $J_c$ change? What does this imply for the design of high-gain reservoirs?

(e) The critical condition $J_c = 1/\beta$ is equivalent to $\rho(W) = 1$ in ESN notation. Verify this: for $\beta = 1$ and $w_{ij} \sim \mathcal{N}(0, J^2/N)$, what is $\rho(W)$ in terms of $J$, in the large-$N$ limit?

---

**A6. Branching Ratio and Neural Avalanches.**

In a branching process model of neural activity, each active neuron activates exactly $k$ other neurons, where $k \sim \text{Poisson}(\sigma)$ with $\sigma$ the branching ratio.

(a) For $\sigma < 1$: compute the expected total size of an avalanche starting from a single active neuron. (Hint: the expected number of neurons active at generation $t$ is $\sigma^t$; sum over generations.)

(b) For $\sigma = 1$: the mean avalanche size diverges. What does this mean physically?

(c) At criticality ($\sigma = 1$), the distribution of avalanche sizes follows a power law $P(s) \propto s^{-3/2}$. What is the exponent $-3/2$ called in statistical physics? What other systems exhibit power-law avalanche distributions with this exponent?

(d) If you recorded from a cortical network and observed an exponential distribution of avalanche sizes (not a power law), would you conclude the network is sub-critical, critical, or super-critical? What intervention might shift it toward criticality?

---

**A7. The Rate-Coding Limit.**

A network of $N = 100$ LIF neurons with TM synapses is operating at a mean firing rate of 20 Hz. The membrane time constant is $\tau_m = 20$ ms and the synaptic time constant is $\tau_s = 5$ ms.

(a) Are the conditions for the rate-coding limit approximately satisfied? Check each of the four conditions listed in Section 7.2.

(b) Suppose we replace each LIF neuron with a rate-coded neuron using $f = \tanh$ activation and $\alpha = \Delta t/\tau_m$. What step size $\Delta t$ would you use (in ms) to match the membrane time constant?

(c) What is the key difference between the ESN that results from the rate-coding approximation and a standard ESN with randomly initialized weights? (Hint: consider the weight distribution and the normalization of $W^{rec}$.)

(d) The TM synaptic dynamics are lost in the simple rate-coding limit. How would you modify the ESN equations to include a crude approximation to TM depression? (Write the modified update equations.)

---

## Part B: Thought Experiments

**B1. What Would a Perfect Liquid Look Like?**

Imagine you could design the "perfect" liquid for a temporal classification task: classify input sequences of length $T = 100$ samples into one of $C = 50$ classes.

- What properties would the perfect liquid have? List at least five, justifying each.
- Is a larger liquid ($N = 1000$) always better than a smaller one ($N = 100$)? When might the smaller liquid be preferable?
- The perfect liquid for this classification task might be terrible for a regression task (predicting a continuous output). Why? What is the tradeoff?
- Is there a "universally perfect" liquid — one that is optimal for all tasks simultaneously? If not, what does this imply for the design of general-purpose reservoirs?

**B2. Is the Brain an LSM?**

Wolfgang Maass's 2002 paper suggests that the cortex might function as a liquid state machine. Evaluate this hypothesis.

- List three pieces of experimental evidence consistent with the LSM hypothesis.
- List three pieces of experimental evidence that are difficult to explain within the LSM framework.
- The LSM framework says the recurrent connections of the cortex are essentially random and untrained. Is this biologically accurate? What evidence do we have about the specificity of cortical connectivity?
- If the cortex is an LSM, where are the "readout" neurons? Name a specific brain area that might serve this role and explain the anatomical evidence.
- What experiment would definitively test whether a cortical area is functioning as an LSM?

**B3. What is the Biological Readout?**

The LSM framework requires a readout that maps liquid states to outputs. In a machine learning context, this is a linear classifier trained by gradient descent or ridge regression. But the brain does not do ridge regression.

- What biological mechanism might implement the readout? (Consider: Hebbian learning, spike-timing-dependent plasticity, reward-modulated plasticity.)
- The RLS algorithm can be implemented locally (each synapse updates based on pre- and post-synaptic activity and a global error signal). Is there a biological analog of the error signal $e_t$?
- The readout in the LSM must be re-trained for each new task. Is there evidence that downstream cortical areas can rapidly reconfigure their input weights for new tasks? What timescale does this operate on?

**B4. Is the Edge of Chaos a Design Principle or an Evolutionary Outcome?**

The edge of chaos maximizes information processing capacity. Does the brain operate there because evolution drove it there, or because it would be at the edge of chaos regardless?

- Argue for the position that the edge of chaos is an evolutionary design principle: natural selection favored brains that operate near criticality because this maximizes computational capacity.
- Argue for the position that the edge of chaos is a consequence of neural dynamics, not a target: any network with balanced excitation and inhibition, short-term plasticity, and homeostatic mechanisms will naturally evolve toward criticality.
- Is the edge of chaos a robust operating point? What happens if the network is slightly perturbed (e.g., by a drug that increases excitatory tone)? Does the network drift away from criticality, or does it self-correct?
- What evidence from clinical neuroscience is relevant? (Consider: epilepsy as super-critical dynamics, depression as sub-critical dynamics [Tagliazucchi2012].)

---

## Part C: Lab Exercises

**L1. Building a Spiking LSM with LIF Neurons.**

Implement a simple LSM using leaky integrate-and-fire neurons and simulate it on an auditory task.

```python
import numpy as np

class LIFNeuron:
    def __init__(self, V_rest=-65.0, V_th=-50.0, V_reset=-65.0,
                 tau_m=20.0, tau_ref=2.0, dt=0.1):
        """
        Leaky integrate-and-fire neuron.
        
        Args:
            V_rest: resting potential (mV)
            V_th: spike threshold (mV)
            V_reset: reset potential (mV)
            tau_m: membrane time constant (ms)
            tau_ref: refractory period (ms)
            dt: simulation time step (ms)
        """
        self.V_rest = V_rest
        self.V_th = V_th
        self.V_reset = V_reset
        self.tau_m = tau_m
        self.tau_ref = tau_ref
        self.dt = dt
        self.V = V_rest
        self.refractory_count = 0
        
    def step(self, I):
        """
        Simulate one time step.
        
        Args:
            I: input current (nA)
        
        Returns:
            spike: 1 if spike occurred, else 0
        """
        # YOUR CODE HERE
        # 1. If in refractory period: hold at reset, decrement counter
        # 2. Otherwise: integrate LIF equation (Euler method)
        # 3. If V >= V_th: emit spike, set V = V_reset, set refractory counter
        pass

class SimpleLSM:
    def __init__(self, N=100, p_conn=0.1, J=1.0, 
                 frac_inh=0.2, seed=42):
        """
        Simple liquid state machine with LIF neurons.
        
        Args:
            N: number of neurons
            p_conn: connection probability
            J: overall synaptic weight scale
            frac_inh: fraction of inhibitory neurons
            seed: random seed
        """
        rng = np.random.default_rng(seed)
        self.N = N
        self.dt = 0.1  # ms
        
        # Create neurons
        self.neurons = [LIFNeuron(dt=self.dt) for _ in range(N)]
        
        # Assign excitatory/inhibitory identity
        self.is_inhibitory = rng.random(N) < frac_inh
        
        # YOUR CODE HERE: create connectivity matrix W
        # W[i,j] = weight from j to i (0 if not connected)
        # Excitatory: W[i,j] > 0; Inhibitory: W[i,j] < 0
        self.W = None
        
        # Synaptic state: exponentially decaying PSCs
        self.tau_syn = 5.0  # ms
        self.psc = np.zeros(N)  # post-synaptic currents
        
    def step(self, I_ext):
        """
        Simulate one time step.
        
        Returns:
            spikes: array of 0/1 spike indicators
            states: filtered spike train for readout
        """
        # YOUR CODE HERE:
        # 1. Compute total current to each neuron = W @ psc + I_ext
        # 2. Update each neuron
        # 3. Update PSC: decay + add spikes
        pass
    
    def run(self, I_ext_sequence):
        """
        Run LSM for a sequence of external currents.
        
        Args:
            I_ext_sequence: (T, N_in) array of input currents
        
        Returns:
            states: (T, N) array of filtered spike trains
        """
        # YOUR CODE HERE
        pass

# Simple test: driven by a sinusoidal input to 20% of neurons
N = 100
T = 1000  # time steps = 100 ms
lsm = SimpleLSM(N=N, p_conn=0.1, J=2.0)

t = np.arange(T) * lsm.dt  # in ms
freq_hz = 40  # input frequency in Hz
I_ext = np.zeros((T, N))
input_neurons = np.random.choice(N, size=20, replace=False)
I_ext[:, input_neurons] = 2.0 * np.sin(2 * np.pi * freq_hz * t * 1e-3)[:, None]

states = lsm.run(I_ext)  # (T, N)
print(f"Mean firing rate: {states.mean() / lsm.dt:.1f} Hz")
print(f"State matrix rank: {np.linalg.matrix_rank(states)}")
```

**Questions:**
(a) What is the mean firing rate of the neurons? Is this biologically reasonable?
(b) What is the rank of the state matrix? How does it change with connectivity $J$?
(c) Train a linear readout to classify whether the input frequency is above or below 20 Hz (use two different frequencies). What classification accuracy do you achieve?

---

**L2. Kernel Quality vs. Connectivity.**

Measure the kernel quality of an LSM as a function of the coupling strength $J$ (from sub-critical to super-critical).

```python
import numpy as np
from scipy.linalg import svd

def kernel_quality(lsm, input_sequences, T_readout=100):
    """
    Measure kernel quality: the rank of the state matrix 
    formed by running the LSM on different input sequences.
    
    Args:
        lsm: SimpleLSM instance
        input_sequences: list of (T, N_in) input arrays
        T_readout: number of time steps to use for readout
    
    Returns:
        kappa: kernel quality (effective rank of state matrix)
        singular_values: singular values of the state matrix
    """
    M = len(input_sequences)
    states = []
    
    for I_ext in input_sequences:
        # Run LSM
        # YOUR CODE HERE: record final T_readout states
        pass
    
    # Stack into matrix and compute kernel quality
    X = np.array(states)  # (M, N * T_readout)
    _, s, _ = svd(X, full_matrices=False)
    
    # Effective rank: number of singular values above threshold
    threshold = s.max() * 1e-3
    kappa = np.sum(s > threshold)
    
    return kappa, s

# Test across coupling strengths
J_values = np.linspace(0.5, 3.0, 15)
kappa_values = []

M = 20  # number of test input sequences
input_sequences = [
    # YOUR CODE HERE: generate M distinct input sequences
    # (different frequencies, different spatial patterns, etc.)
]

for J in J_values:
    lsm = SimpleLSM(N=100, p_conn=0.1, J=J)
    kappa, _ = kernel_quality(lsm, input_sequences)
    kappa_values.append(kappa)
    print(f"J={J:.2f}: kernel_quality={kappa}")

# Plot J vs kernel_quality
```

**Questions:**
(a) How does kernel quality depend on $J$? Is there a maximum?
(b) At what $J$ does the liquid transition from ordered to chaotic? (Hint: look for where kernel quality starts decreasing.)
(c) For the depressing synapse parameters from Exercise A2, does adding TM dynamics change the kernel quality at any $J$ value? By how much?

---

**L3. Comparing ESN and LSM on Spoken Digit Classification.**

Implement both an ESN and an LSM and compare them on the Spoken Digit Recognition task using the TI-46 dataset (or a synthetic substitute).

```python
import numpy as np

# For this exercise we use a synthetic spoken digit task:
# Each "digit" is a sinusoidal chirp with a digit-specific frequency trajectory
# Digit d has base frequency f_d Hz and linear chirp rate r_d Hz/s

def generate_synthetic_digit(digit, duration_ms=200, dt_ms=0.1, seed=None):
    """
    Generate a synthetic 'spoken digit' as a time-varying frequency signal.
    
    Digits 0-9 are encoded as different (f_base, chirp_rate) combinations.
    Returns a (T,) array of input current values.
    """
    params = {
        # (base_freq_Hz, chirp_rate_Hz_per_s)
        0: (100, 0),    1: (120, 50),   2: (140, 100),
        3: (160, -50),  4: (180, 150),  5: (200, -100),
        6: (220, 200),  7: (240, -150), 8: (260, 50),
        9: (280, -200)
    }
    f0, chirp = params[digit]
    rng = np.random.default_rng(seed)
    T = int(duration_ms / dt_ms)
    t_s = np.arange(T) * dt_ms * 1e-3
    freq = f0 + chirp * t_s
    signal = np.sin(2 * np.pi * np.cumsum(freq) * dt_ms * 1e-3)
    noise = rng.standard_normal(T) * 0.1
    return signal + noise

# Generate dataset
n_trials_per_digit = 20
n_digits = 10
dataset = []

for digit in range(n_digits):
    for trial in range(n_trials_per_digit):
        signal = generate_synthetic_digit(
            digit, duration_ms=200, seed=digit * 1000 + trial
        )
        dataset.append((digit, signal))

# YOUR CODE HERE:
# 1. Train/test split (70/30)
# 2. Run ESN on each signal, extract final state as feature
# 3. Run LSM on each signal, extract final state as feature
# 4. Train linear SVM on features, evaluate classification accuracy
# 5. Compare ESN vs LSM accuracy
```

**Questions:**
(a) Which achieves higher classification accuracy: the ESN or the LSM? Is the difference statistically significant?
(b) How does performance vary with reservoir size $N$ for each?
(c) For the LSM, does using TM synapses improve performance compared to static synapses?
(d) The ESN is much faster to simulate. What is the simulation time ratio (ESN vs. LSM) for equivalent reservoir size and task duration?

---

## Part D: Programming Projects

**P1. Biologically Realistic LSM Implementation.**

Implement a full LSM using the Maass et al. 2002 parameter set:
- 135 LIF neurons (108 excitatory, 27 inhibitory)
- Tsodyks-Markram synapses with cell-type-specific parameters
- Distance-dependent connection probability
- Realistic input via 30 randomly selected excitatory neurons

Train a readout to:
(a) Classify whether input is a low-frequency or high-frequency sinusoid
(b) Estimate the integral of the input over the last 100 ms
(c) Classify which of 5 spatial input patterns was presented

Report: kernel quality, classification accuracy, simulation time, and comparison with an equivalent-size ESN.

**P2. Self-Organized Criticality in Reservoirs.**

Implement a synaptic scaling rule that drives an LSM toward the critical branching ratio $\sigma = 1$. The rule should:
- Measure the average branching ratio $\hat{\sigma}$ from recent activity (count the average number of neurons activated per active neuron).
- Adjust the overall synaptic weight scale $J$ toward $J_c$: $J \leftarrow J + \eta(\hat{\sigma} - 1)$.

Test this adaptive rule starting from both sub-critical ($J = 0.5 J_c$) and super-critical ($J = 2 J_c$) initial conditions. Show that:
(a) The system converges to $\sigma \approx 1$.
(b) The converged network has higher kernel quality than networks at fixed $J \neq J_c$.
(c) Performance on a temporal classification task is improved when the network is at criticality.
