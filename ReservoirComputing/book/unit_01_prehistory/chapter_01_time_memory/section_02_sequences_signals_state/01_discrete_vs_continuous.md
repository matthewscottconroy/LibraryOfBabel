# 1.2.1 Discrete Versus Continuous Time Signals

## Two Worlds of Time

Temporal signals come in two fundamental flavors, and the choice between them is not merely mathematical pedantry — it shapes every architectural decision that follows.

A **continuous-time signal** is a function $u: \mathbb{R} \to \mathbb{R}^d$, assigning a value to every instant in a continuous time domain. The world is, at the physical level, continuous. Electrical voltages evolve continuously. Neurons fire in continuous time (even if the firing events themselves are discrete). The Lorenz equations describe continuous trajectories.

A **discrete-time signal** is a sequence $u: \mathbb{Z} \to \mathbb{R}^d$, assigning values only to integer time steps. This is what you get when you sample a continuous signal at regular intervals — and it is what nearly every practical machine learning system works with, because computers operate in discrete steps.

The relationship between the two is given by the **Nyquist-Shannon sampling theorem**: a bandlimited continuous signal with maximum frequency $f_{max}$ can be perfectly reconstructed from samples taken at rate $f_s \geq 2 f_{max}$. Below this rate, aliasing occurs — different continuous signals produce identical discrete samples.

For reservoir computing, we will primarily work in discrete time, because:
1. Most practical data arrives in discrete samples.
2. The mathematical theory of discrete-time dynamical systems is cleanly self-contained.
3. The reservoir update equation $\mathbf{x}_{t+1} = f(\mathbf{x}_t, u_t)$ is naturally discrete.

However, continuous-time reservoirs exist and are important (particularly in physical implementations and in connection with neural ODEs — see Chapter 30). We will develop the continuous-time theory alongside the discrete when it illuminates the discrete.

## Causality

A system is **causal** if its output at time $t$ depends only on inputs at times $\leq t$. All physically realizable systems are causal: you cannot respond to an input before you receive it.

This seems obvious, but it has a non-obvious mathematical consequence: the space of causal functionals is much smaller than the space of all functionals. A causal functional $H$ maps the *half-infinite past* $(u_t, u_{t-1}, u_{t-2}, \ldots)$ to the current output $y_t$. This is a map from an infinite-dimensional space to $\mathbb{R}$.

**Why infinite-dimensional?** Because the past of a signal contains infinitely many values, one for each past time step. Even if each individual value $u_{t-k}$ is a scalar, the collection of all past values is an infinite-dimensional object. This is the source of the fundamental difficulty of temporal computation: we are trying to map from an infinite-dimensional input space using a finite-dimensional computing machine.

The fading memory property (Section 1.4) resolves this by showing that, for well-behaved systems, the effective input dimensionality is finite — infinitely many past inputs, but only finitely many of them matter at any given precision.

## Time-Invariance

A system is **time-invariant** if its input-output behavior does not change with time. Formally: if shifting the input by $k$ steps shifts the output by $k$ steps with no other change:

$$H[\mathbf{u}]_t = H[\text{shift}^k \mathbf{u}]_{t-k}$$

Most of the systems we will study are time-invariant: the reservoir update rule is the same at every time step, and the readout weights do not change. This is a simplifying assumption — one that excludes many interesting phenomena (adaptation, learning, non-stationarity) — but it is the right place to start.

Time-invariant, causal systems are the natural objects of study in classical signal processing, control theory, and the theory of fading-memory approximation. The Boyd-Chua theorem applies specifically to this class.

## The State Variable

Given a causal, time-invariant system, the **state** at time $t$ is any finite-dimensional vector $\mathbf{x}_t \in \mathbb{R}^N$ such that the future behavior of the system is determined by $(\mathbf{x}_t, u_{t+1}, u_{t+2}, \ldots)$ — that is, future inputs plus the current state are sufficient to compute all future outputs.

For a causal system without state, this would require knowing the entire past. The state compresses that past into a fixed-dimensional vector. Different systems have different state representations:

| System | State $\mathbf{x}_t$ | Dimension |
|--------|---------------------|-----------|
| FIR filter, order $K$ | $(u_t, u_{t-1}, \ldots, u_{t-K+1})$ | $K \cdot d$ |
| IIR filter (linear) | $(y_t, y_{t-1}, \ldots)$ | finite (if poles inside unit circle) |
| Linear RNN | $(h_t)$ | $N$ hidden units |
| ESN (Echo State Network) | $(x_t)$ | $N$ reservoir units |
| Physical reservoir | Physical measurement | depends on substrate |

The key insight of reservoir computing is that the state $\mathbf{x}_t$ of a reservoir is a high-dimensional, nonlinear function of the past — much richer than a delay line or a linear filter — and this richness allows a simple linear readout to approximate complex temporal functionals.

## Stationarity and the Ergodic Assumption

A key practical assumption in temporal machine learning is **stationarity**: the statistical properties of the signal do not change over time. For a stationary signal:
- The mean $\mathbb{E}[u_t]$ does not depend on $t$.
- The autocorrelation $\mathbb{E}[u_t u_{t-k}]$ depends only on the lag $k$, not on $t$.

Stationarity is what justifies learning from finite data: if the signal has the same statistics everywhere, then a sample from one period is representative of any other period.

Most reservoir computing theory assumes stationarity, and most benchmarks use stationary or ergodic processes (Mackey-Glass, Lorenz, NARMA). Real-world signals are often non-stationary, and handling non-stationarity is an active area of research (Section 22.4).

---

## References

- [Oppenheim1999] Oppenheim, A.V. & Schafer, R.W. (1999). *Discrete-Time Signal Processing*, 2nd ed. Prentice Hall.
- [Haykin2002] Haykin, S. (2002). *Adaptive Filter Theory*, 4th ed. Prentice Hall.
- [Papoulis2002] Papoulis, A. & Pillai, S.U. (2002). *Probability, Random Variables, and Stochastic Processes*, 4th ed. McGraw-Hill.
