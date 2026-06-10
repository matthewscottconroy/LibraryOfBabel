# 4.1.1 The Three-Component Architecture

## Overview

A reservoir computer consists of three components: an **input layer**, a **reservoir**, and a **readout**. Only the readout is trained. The input layer and reservoir are fixed — set randomly at construction time and left unchanged for the entire lifetime of the model.

This is a radical simplification compared to a trained RNN, and it is worth dwelling on exactly what is fixed and what is not, what each component contributes, and what happens if any component is removed.

## Full Model Equations

The complete reservoir computing model is described by three equations:

**State update (reservoir dynamics):**
$$\mathbf{x}_{t+1} = f\!\left(W^{rec}\mathbf{x}_t + W^{in}\mathbf{u}_t + \mathbf{b}\right)$$

**Readout:**
$$\mathbf{y}_t = W^{out}\mathbf{x}_t + \mathbf{b}^{out}$$

**Loss (for training):**
$$L = \|X W^{out,T} + \mathbf{1}\mathbf{b}^{out,T} - \hat{Y}\|_F^2 + \alpha \|W^{out}\|_F^2$$

where $X \in \mathbb{R}^{T_{train} \times N}$ is the matrix of collected reservoir states (one row per time step), $\hat{Y} \in \mathbb{R}^{T_{train} \times M}$ is the matrix of target outputs, and $\alpha \geq 0$ is a regularization parameter.

Let us now account for every term and every dimension.

## Component 1: The Input Layer

The input layer is defined by a single matrix:

$$W^{in} \in \mathbb{R}^{N \times K}$$

where $N$ is the number of reservoir units and $K$ is the dimension of the input signal $\mathbf{u}_t \in \mathbb{R}^K$.

**Role:** $W^{in}$ maps the $K$-dimensional input signal into the $N$-dimensional reservoir space. Each column $W^{in}_{\cdot, k}$ is the "projection direction" associated with input channel $k$: it determines how strongly and in what configuration the $k$-th input channel drives the reservoir.

**Typical construction:** Entries of $W^{in}$ are drawn independently from a distribution — commonly $\text{Uniform}(-\sigma_{in}, \sigma_{in})$ or $\text{Normal}(0, \sigma_{in}^2)$ — and then fixed. The input scaling $\sigma_{in}$ is a hyperparameter (Chapter 8). Sparse constructions (with only one nonzero entry per row) are also common, ensuring that each neuron receives input from only one input channel.

**What it does dynamically:** $W^{in}\mathbf{u}_t$ injects the current input as an additive perturbation to the reservoir's state trajectory. Without $W^{in}$, the reservoir would evolve autonomously, unresponsive to the task's inputs. With $W^{in}$, every new input nudges the reservoir state in a direction that depends on the value of the input, creating input-sensitive trajectories.

**What happens if you remove it:** Without $W^{in}$, the reservoir receives no input and simply runs its autonomous dynamics from the initial condition. The state trajectory is independent of the input signal, and the readout has no access to input-dependent information. The model collapses to a constant predictor (trivially useless).

## Component 2: The Reservoir

The reservoir is defined by:
- A weight matrix $W^{rec} \in \mathbb{R}^{N \times N}$, fixed and typically random.
- A bias vector $\mathbf{b} \in \mathbb{R}^N$, typically zero.
- A nonlinearity $f: \mathbb{R} \to \mathbb{R}$, applied element-wise. Standard choice: $\tanh$.
- An initial state $\mathbf{x}_0 \in \mathbb{R}^N$, typically $\mathbf{0}$.

The state update is:

$$\mathbf{x}_{t+1} = f\!\left(W^{rec}\mathbf{x}_t + W^{in}\mathbf{u}_t + \mathbf{b}\right)$$

**Role:** The reservoir produces, at every time step, an $N$-dimensional state vector $\mathbf{x}_t$ that encodes the history of the input signal up to time $t$. The quality of this encoding depends on the reservoir's dynamics.

**Typical construction:** $W^{rec}$ is constructed in three steps:
1. Draw a random matrix $\tilde{W} \in \mathbb{R}^{N \times N}$ with entries drawn independently from $\text{Normal}(0, 1)$.
2. Optionally sparsify: set a fraction $1 - p$ of entries to zero (keeping only a fraction $p$ connected).
3. Scale to the desired spectral radius: $W^{rec} = \rho \cdot \tilde{W} / \rho(\tilde{W})$.

The spectral radius $\rho(W^{rec}) = \rho$ is the primary hyperparameter of the reservoir (Chapter 5, 8).

**What it does dynamically:** $W^{rec}$ defines an autonomous dynamical system $\mathbf{x}_{t+1} = f(W^{rec}\mathbf{x}_t + \mathbf{b})$. The input $W^{in}\mathbf{u}_t$ drives this system away from its autonomous trajectory. The reservoir acts as a **nonlinear fading memory**: it retains information about past inputs in its state, but with decreasing influence as time passes (fading memory property, Chapter 5).

The key properties the reservoir must have:
- **Echo state property (ESP):** For any two initial conditions and any input sequence, the difference in the reservoir states eventually vanishes. Equivalently: the state is determined by the input history, not the initial condition.
- **Separation property:** Different input histories must produce different states (so the readout can distinguish them).
- **Approximation:** The state trajectory must span a sufficiently rich function space that the readout can approximate the target function.

**What happens if you remove it:** Without the reservoir (i.e., with $\mathbf{x}_t = \mathbf{u}_t$), the readout receives only the raw input at each time step, with no memory of past inputs. The system reduces to a linear filter applied to the current input — no temporal memory, no nonlinear mixing, no compression into a fixed-dimensional state.

**What happens if you make it too small:** If $N$ is too small (say, $N = 5$), the reservoir cannot represent the diversity of input histories it encounters. The state space is too low-dimensional to separate different histories, and the readout's approximation error will be large.

**What happens if you train it:** This is the key question that distinguishes reservoir computing from standard RNN training. If $W^{rec}$ is trained, we recover standard BPTT — with all the gradient problems of Chapter 3. If $W^{rec}$ is fixed randomly, we replace an intractable nonlinear optimization with a tractable linear one.

## Component 3: The Readout

The readout is defined by:

$$\mathbf{y}_t = W^{out}\mathbf{x}_t + \mathbf{b}^{out}$$

where $W^{out} \in \mathbb{R}^{M \times N}$ and $\mathbf{b}^{out} \in \mathbb{R}^M$.

**Role:** $W^{out}$ takes the $N$-dimensional reservoir state and produces the $M$-dimensional output. It is the only trained component. Its training problem is a linear regression: find $W^{out}$ (and $\mathbf{b}^{out}$, or absorb it into $W^{out}$ by augmenting the state) that minimizes the loss $L$ above.

**Training:** Given the collected state matrix $X \in \mathbb{R}^{T_{train} \times N}$ and target matrix $\hat{Y} \in \mathbb{R}^{T_{train} \times M}$, the solution is:

$$W^{out,T} = (X^T X + \alpha I)^{-1} X^T \hat{Y}$$

This is **ridge regression** (Tikhonov regularization) with regularization parameter $\alpha > 0$. The solution is unique, globally optimal, and computable in $O(N^2 T_{train} + N^3)$ operations.

For large $T_{train} \gg N$, this is dominated by the $O(N^2 T_{train})$ cost of forming $X^T X$. For large $N$, the matrix inversion dominates at $O(N^3)$.

**What happens if the readout is nonlinear:** If $W^{out}$ is replaced by a multilayer network (a trained MLP on top of the reservoir), the training problem becomes nonconvex again. This is sometimes done in practice (e.g., reservoir computing with a neural network readout), but it sacrifices the convexity and closed-form solution that make reservoir computing tractable.

**What happens if you remove it:** Without the readout, there is no output. The reservoir is doing computation but nothing is extracting its result.

## Dimensional Summary

For reference, a complete dimensional accounting of all components:

| Symbol | Dimensions | Description |
|---|---|---|
| $\mathbf{u}_t$ | $K \times 1$ | Input vector at time $t$ |
| $W^{in}$ | $N \times K$ | Input weight matrix (fixed) |
| $\mathbf{x}_t$ | $N \times 1$ | Reservoir state at time $t$ |
| $W^{rec}$ | $N \times N$ | Recurrent weight matrix (fixed) |
| $\mathbf{b}$ | $N \times 1$ | Reservoir bias (fixed, often $\mathbf{0}$) |
| $W^{out}$ | $M \times N$ | Readout weight matrix (trained) |
| $\mathbf{b}^{out}$ | $M \times 1$ | Readout bias (trained) |
| $\mathbf{y}_t$ | $M \times 1$ | Output vector at time $t$ |
| $X$ | $T_{train} \times N$ | Collected state matrix |
| $\hat{Y}$ | $T_{train} \times M$ | Target output matrix |

## The Washout Period

There is one additional element of the training procedure worth mentioning here: the **washout period**.

When the reservoir starts from $\mathbf{x}_0 = \mathbf{0}$, its initial state is arbitrary (or rather, is determined by the initial condition, not the input). It takes some time — typically 50–200 time steps for a well-tuned reservoir — for the initial condition to "wash out" and for the state to become a reliable function of the input history. During this washout period, the state trajectory is contaminated by the initial condition and should not be used for training.

In practice: run the reservoir on the training sequence for $T_w$ washout steps, then collect states for the remaining $T_{train}$ steps. The washout period is a hyperparameter; its minimum value is related to the effective memory time of the reservoir, which scales as $\tau_{\text{eff}} \approx -1/\ln \rho(W^{rec})$ (see Chapter 5).

## Why Each Component Is Necessary

A brief taxonomy:

- **Without $W^{in}$:** No input coupling. Reservoir ignores the task. Fail.
- **Without $W^{rec}$ (i.e., $W^{rec} = 0$):** No memory. Readout receives only current input. Same as a linear classifier with no temporal context. Limited.
- **Without the nonlinearity $f$ (i.e., $f = $ identity):** The reservoir is a linear dynamical system. Linear reservoirs can compute Volterra series expansions of the input, which is useful but restricts the class of computable functions. The nonlinearity is what enables approximation of nonlinear temporal functions.
- **Without the washout:** The state trajectory is contaminated by the initial condition, which has nothing to do with the input signal. Training on contaminated states reduces readout accuracy, especially for the early part of the training sequence.
- **Without regularization ($\alpha = 0$):** The solution to the linear regression problem may overfit, especially when $N$ is large relative to $T_{train}$. Regularization is essential for generalization (Chapter 8).

---

## References

- [Jaeger2001] Jaeger, H. (2001). The "echo state" approach to analysing and training recurrent neural networks. *GMD Report 148*.
- [Lukoševičius2012] Lukoševičius, M. (2012). A practical guide to applying echo state networks. In *Neural Networks: Tricks of the Trade* (pp. 659–686). Springer.
- [Verstraeten2007] Verstraeten, D., Schrauwen, B., d'Haene, M., & Stroobandt, D. (2007). An experimental unification of reservoir computing methods. *Neural Networks*, 20(3), 391–403.
