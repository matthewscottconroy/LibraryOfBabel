# Chapter 1 — Exercises, Thought Experiments, and Labs

---

## Conceptual Exercises

These exercises test and deepen your understanding of the ideas in the chapter. Most can be worked with pen and paper.

**Exercise 1.1 — The Phase Problem**
Prove rigorously that no feedforward network with a fixed input window of size $W < \infty$ can distinguish the sequence $u = [\ldots, 1, 0, 1, 0, 1, 0, \ldots]$ (alternating) starting at an even phase from the same sequence starting at an odd phase, for all initial conditions. (Hint: construct two sequences that agree on the window but differ before it.)

**Exercise 1.2 — The Memory Bound**
Let $f: \mathbb{R}^W \to \mathbb{R}$ be any function and define $y_t = f(u_t, u_{t-1}, \ldots, u_{t-W+1})$. Show that this system cannot compute the parity function $y_t = \bigoplus_{k=0}^{t} u_{t-k}$ (XOR of all past binary inputs) for any finite $W$. What does this tell us about the relationship between memory depth and computational tasks?

**Exercise 1.3 — The Volterra Truncation Error**
The first-order Volterra approximation to a system $H$ is:

$$\hat{y}(t) = \int_0^\infty h_1(\tau) u(t-\tau)\, d\tau$$

The second-order approximation adds:

$$\iint_0^\infty h_2(\tau_1, \tau_2) u(t-\tau_1) u(t-\tau_2)\, d\tau_1 d\tau_2$$

(a) Compute the first- and second-order Volterra approximations to the system $y(t) = u(t) \cdot u(t-1)$ (the product of the current and previous input).

(b) Show that the second-order approximation is exact for this system. What is the kernel $h_2(\tau_1, \tau_2)$?

(c) Consider the system $y(t) = u(t)^2 \cdot u(t-1)$. At what Volterra order is the exact representation achieved?

**Exercise 1.4 — Fading Memory: Checking the Definition**
Verify that the following systems have (or do not have) the fading memory property as defined in Section 1.4:

(a) $H[\mathbf{u}]_t = u_{t-5}$ (pure delay of 5 steps). Does this have fading memory? For what weighting sequence $w$?

(b) $H[\mathbf{u}]_t = \int_{-\infty}^t e^{-(t-s)} u(s)\, ds$ (exponential smoothing). What is the appropriate $w$?

(c) $H[\mathbf{u}]_t = \lim_{T \to \infty} \frac{1}{T}\int_0^T u(s)\, ds$ (long-time average). Does this have fading memory? Why or why not?

**Exercise 1.5 — Volterra and Neural Networks**
A Volterra series of order $p$ truncated at lag $K$ can be thought of as a polynomial function of the last $K$ inputs. This is equivalent to a one-layer neural network with polynomial activation functions applied to a windowed input.

(a) Show that a quadratic Volterra approximation with window $K$ requires $O(K^2)$ parameters.

(b) Show that a degree-$p$ Volterra approximation requires $O(K^p)$ parameters.

(c) Argue that the parameter explosion in Volterra series is the functional analog of the curse of dimensionality in standard regression. Why does this motivate the need for a compact state representation?

**Exercise 1.6 — Boyd-Chua and Reservoir Computing**
The Boyd-Chua theorem guarantees the existence of a dynamical system approximating any fading-memory functional. It does not specify which dynamical system to use.

(a) List three properties that a dynamical system should have in order to be a good candidate for approximating a wide class of fading-memory functionals. (Think about diversity, nonlinearity, and stability.)

(b) A system with exactly one stable fixed point (and no other dynamics) is a poor reservoir. Why? What functional class can it not approximate?

(c) A system in a chaotic regime (positive Lyapunov exponent with no input) is also a poor reservoir for some tasks. Why? Under what conditions might chaotic dynamics be useful?

---

## Thought Experiments

These are open-ended questions without unique correct answers. Engage with them seriously — they are designed to build intuition that mathematics alone cannot provide.

**Thought Experiment 1.A — The Amnesiac Agent**
Imagine an intelligent agent that has perfect perceptual acuity (it perceives the current state of its environment with perfect accuracy) but no memory whatsoever — each moment arrives fresh, with no record of what came before.

(a) What tasks can this agent perform? List five.

(b) What tasks cannot it perform? List five.

(c) Now give the agent a memory window of exactly 1 second. What new tasks become possible? What remains impossible?

(d) Suppose the agent could choose between a 100-step perfect memory of recent inputs, and an infinite but exponentially fading memory. For which tasks would the perfect short memory be preferable? For which would fading memory be preferable?

**Thought Experiment 1.B — Reversing the Arrow**
We have been discussing causal systems where the output depends on the past. Consider a system where the output at time $t$ depends on the *future* input as well:

$$y_t = H(u_{t+5}, u_{t+4}, \ldots, u_t, u_{t-1}, \ldots)$$

(a) Is such a system physically realizable in real time? Under what circumstances could it be approximated by a causal system?

(b) In offline batch processing (where the entire input sequence is available before any output is computed), anticipatory dependencies become possible. Describe a task where knowing future inputs helps. How does this change the architecture we need?

(c) This leads to the concept of **bidirectional** recurrent networks. Argue for or against the use of bidirectional reservoirs in batch processing settings.

**Thought Experiment 1.C — Memory as Compression**
The brain contains approximately $10^{11}$ neurons with roughly $10^{15}$ synaptic connections. Yet it encodes memories of a lifetime — billions of moments of experience — in this finite substrate.

(a) What must be true about the nature of memory in the brain for this to be possible? (Consider: is it lossless storage, or lossy compression?)

(b) The fading memory property implies that distant memories are represented with less precision than recent ones. Is this consistent with human memory? Where does it agree, and where does it disagree?

(c) What does this suggest about the kind of computational memory a brain-inspired system should implement?

**Thought Experiment 1.D — The Sufficient Statistic**
In statistics, a **sufficient statistic** for a parameter $\theta$ given data $X$ is any statistic $T(X)$ that captures all the information about $\theta$ contained in $X$. No additional function of $X$ can provide more information about $\theta$.

Reinterpret this concept in the temporal setting: the "parameter" is the target output $y_t$, and the "data" is the sequence of past inputs $\mathbf{u}$.

(a) Define a **sufficient state** for a temporal functional $H$: the minimal information about the past that is necessary to compute $H[\mathbf{u}]_t$ for any $t$.

(b) For the system $y_t = u_t + 0.5 u_{t-1}$, what is the sufficient state? What is its dimension?

(c) For the Lorenz system (to be studied in Chapter 2), what would the sufficient state be? Could it be represented in finite dimensions?

(d) The goal of a reservoir is to compute an approximation of the sufficient state. Why is approximation necessary rather than exact representation?

---

## Lab Exercises

These are computational experiments. Each can be implemented in Python with NumPy in an hour or less. The goal is to develop intuition through direct observation.

**Lab 1.1 — The Window Wall**

*Objective:* Empirically observe the ceiling effect of sliding window models.

1. Generate 10,000 samples of the NARMA-10 time series (defined in Appendix E).
2. Train a feedforward neural network with 2 hidden layers (64 units each) using window sizes $W \in \{2, 5, 10, 15, 20, 30\}$.
3. Plot validation NRMSE as a function of $W$. Where does the error plateau? What happens for $W < 10$ vs. $W > 10$?
4. Now generate NARMA-30 and repeat. How does the plateau shift?
5. **Reflection:** What does this tell you about the relationship between task memory requirements and optimal window size?

```python
# Starter code: NARMA-10 generation
def narma10(T, seed=42):
    rng = np.random.default_rng(seed)
    u = rng.uniform(0, 0.5, T)
    y = np.zeros(T)
    for t in range(10, T):
        y[t] = (0.3 * y[t-1] 
                + 0.05 * y[t-1] * np.sum(y[t-10:t])
                + 1.5 * u[t-10] * u[t-1] + 0.1)
    return u, y
```

**Lab 1.2 — The Volterra Approximation**

*Objective:* Explore the order-complexity tradeoff in Volterra series approximation.

1. Generate 5,000 samples of the system $y_t = \tanh(u_t \cdot u_{t-1} + 0.5 u_{t-2}^2)$.
2. Build Volterra approximators of orders 1, 2, and 3 with window $K = 5$. (Use polynomial features of the windowed input.)
3. For each order, measure the training error and test error. How does order affect the bias-variance tradeoff?
4. Fix order 2 and vary $K \in \{2, 5, 10, 20\}$. Count the number of features for each. Plot error vs. number of features.
5. **Reflection:** Where does the Volterra series break down? At what order does it overfit on small training sets?

**Lab 1.3 — Fading Memory: Seeing It in Action**

*Objective:* Visualize what fading memory looks like in a simple dynamical system.

1. Implement the leaky integrator: $x_{t+1} = (1-\alpha) x_t + \alpha u_t$ for $\alpha \in \{0.1, 0.5, 0.9\}$.
2. Feed an impulse input: $u_0 = 1$, $u_t = 0$ for $t > 0$.
3. Plot $x_t$ as a function of $t$ for each $\alpha$. This is the system's memory of the impulse.
4. For each $\alpha$, compute the effective memory time constant (the time for $x_t$ to decay to $e^{-1} \approx 0.37$ of its peak).
5. Now feed a random input for 100 steps, then set $u_t = 0$. How long does it take for $x_t$ to "forget" the random input? Compare to your theoretical prediction.
6. **Reflection:** The leaky integrator is a linear fading-memory system. Is a linear system ever sufficient for temporal computation? What can it compute, and what can it not?

---

## Programming Projects

These are larger, more open-ended projects suitable for a week or more of work.

**Project 1.A — The Volterra Neural Network**

Build a "Volterra neural network" that uses polynomial features of a sliding window as its input layer, followed by a standard feedforward network. Train it on 5 different time-series prediction tasks (NARMA-10, Mackey-Glass, Lorenz x-coordinate, a financial return series of your choice, and a speech segment). For each task:
- Tune the window size and polynomial order jointly via cross-validation.
- Report the parameter count and training data efficiency.
- Compare to a pure sliding-window feedforward baseline.

Write a 1-page analysis: when does the polynomial expansion help, and when does it hurt?

**Project 1.B — Memory Capacity Estimation**

Implement a memory capacity estimator for any dynamical system (following Jaeger 2002):
- Run the system on a random binary input sequence.
- For each delay $k$, train a linear readout to reconstruct $u_{t-k}$ from the current state.
- Measure $r^2$ (coefficient of determination). Plot $r^2$ vs. $k$.
- Total memory capacity: $\text{MC} = \sum_k r^2_k$.

Apply this to:
1. A leaky integrator with several $\alpha$ values.
2. A delay line of length $N$.
3. A sliding window system.

What is the theoretical maximum memory capacity for each? Does your measurement match?

**Project 1.C — The Causal Functional Zoo**

Choose 10 causal functionals from the list below. For each:
- Determine whether it has the fading memory property (prove or disprove).
- If it does, estimate (empirically) the memory time constant.
- Determine whether a linear dynamical system can implement it exactly.
- Determine whether a nonlinear dynamical system is required.

Functionals to consider:
1. $y_t = u_t + u_{t-3}$
2. $y_t = u_t \cdot u_{t-1}$
3. $y_t = \max(u_t, u_{t-1}, \ldots, u_{t-9})$
4. $y_t = \sum_{k=0}^{\infty} \rho^k u_{t-k}$ (geometric average)
5. $y_t = \mathbf{1}[\sum_{k=0}^{4} u_{t-k} > 2.5]$ (threshold on running sum)
6. $y_t = u_{t-5} \cdot u_{t-10}$ (product of two lagged inputs)
7. $y_t = u_t$ if $u_{t-1} > 0$ else $-u_t$ (sign-dependent relay)
8. $y_t = \exp(-\sum_{k=0}^{t} u_k^2)$ (cumulative energy decay) — does this have fading memory?
9. $y_t = $ the $k$-th Legendre polynomial of the recent input window (for $k = 3$)
10. $y_t = $ the autocorrelation of the input at lag 5: $\mathbb{E}[u_t u_{t-5}]$, estimated from recent history

---

## References

- [Boyd1985] Boyd, S. & Chua, L.O. (1985). Fading memory and the problem of approximating nonlinear operators with Volterra series. *IEEE Transactions on Circuits and Systems*, 32(11), 1150–1161.
- [Jaeger2002] Jaeger, H. (2002). Short term memory in echo state networks. GMD Technical Report 152, German National Research Center for Information Technology.
- [Mackey1977] Mackey, M.C. & Glass, L. (1977). Oscillation and chaos in physiological control systems. *Science*, 197(4300), 287–289.
- [Waibel1989] Waibel, A. et al. (1989). Phoneme recognition using time-delay neural networks. *IEEE Transactions on Acoustics, Speech, and Signal Processing*, 37(3), 328–339.
- [Volterra1930] Volterra, V. (1930). *Theory of Functionals and of Integral and Integro-Differential Equations*. Blackie & Son.
