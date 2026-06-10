# 3.1.1 RNN Architecture: The Network as a Dynamical System

## The Central Equation

A recurrent neural network is defined by a single equation, repeated at every time step:

$$\mathbf{x}_{t+1} = f\!\left(W^{rec}\mathbf{x}_t + W^{in}\mathbf{u}_t + \mathbf{b}\right)$$

This equation is at once simple and rich. Before going further, let us account for every term.

**The state vector** $\mathbf{x}_t \in \mathbb{R}^N$ is the internal memory of the network at time $t$. It is a vector of $N$ real numbers, one per neuron. The subscript $t$ runs over discrete time steps $t = 0, 1, 2, \ldots, T$. There is an initial condition $\mathbf{x}_0$, usually taken to be the zero vector.

**The input vector** $\mathbf{u}_t \in \mathbb{R}^K$ is the external signal arriving at time $t$. It has $K$ components, one per input channel.

**The recurrent weight matrix** $W^{rec} \in \mathbb{R}^{N \times N}$ governs how the current state transforms into the next state. Its $(i, j)$ entry $W^{rec}_{ij}$ is the synaptic weight from neuron $j$ to neuron $i$. This matrix is the defining structure of the network: it encodes the internal dynamics, the attractor landscape, the way the network's "thoughts" evolve in the absence of input.

**The input weight matrix** $W^{in} \in \mathbb{R}^{N \times K}$ maps the $K$-dimensional input signal into the $N$-dimensional state space. Its $(i, k)$ entry $W^{in}_{ik}$ is the weight from input channel $k$ to neuron $i$. This matrix determines how the external world drives the internal dynamics.

**The bias vector** $\mathbf{b} \in \mathbb{R}^N$ provides each neuron with a constant offset that can shift its operating point — allowing neurons to be active or inactive in the absence of any input or recurrent drive.

**The nonlinearity** $f: \mathbb{R} \to \mathbb{R}$ is applied element-wise. The standard choice is $\tanh$, which maps $\mathbb{R}$ to $(-1, 1)$ and has derivative $f'(x) = 1 - \tanh^2(x)$, bounded between 0 and 1. Other choices include the logistic sigmoid $\sigma(x) = 1/(1 + e^{-x})$ or rectified linear units $\text{ReLU}(x) = \max(0, x)$. For most of this chapter, we use $\tanh$ and denote its derivative $f'$.

## Dimensional Analysis

The dimensional analysis of the state update equation is worth doing explicitly, because it forces clarity about what is being computed.

The term $W^{rec}\mathbf{x}_t$ has the shape:

$$\underbrace{W^{rec}}_{N \times N} \cdot \underbrace{\mathbf{x}_t}_{N \times 1} = \underbrace{(\text{result})}_{N \times 1}$$

Each row of $W^{rec}$ dot-products with $\mathbf{x}_t$ to produce a scalar: the net recurrent input to that neuron. The result is an $N$-dimensional vector representing the total recurrent drive to each neuron.

The term $W^{in}\mathbf{u}_t$ has the shape:

$$\underbrace{W^{in}}_{N \times K} \cdot \underbrace{\mathbf{u}_t}_{K \times 1} = \underbrace{(\text{result})}_{N \times 1}$$

Each row of $W^{in}$ dot-products with $\mathbf{u}_t$ to produce the external drive to that neuron. The result is an $N$-dimensional vector representing the total input drive.

The sum $W^{rec}\mathbf{x}_t + W^{in}\mathbf{u}_t + \mathbf{b}$ is therefore an $N$-dimensional vector representing the total pre-activation drive to all neurons. The nonlinearity $f$ is applied element-wise, yielding the new state $\mathbf{x}_{t+1} \in \mathbb{R}^N$.

## The Output Equation

The RNN also has a readout: a mapping from the state to the network's output at each time step:

$$\mathbf{y}_t = W^{out}\mathbf{x}_t + \mathbf{b}^{out}$$

Here $W^{out} \in \mathbb{R}^{M \times N}$ maps the $N$-dimensional state to the $M$-dimensional output, and $\mathbf{b}^{out} \in \mathbb{R}^M$ is a readout bias. The output is typically linear in the state, though nonlinear readouts are possible. The linearity of the readout is not an accident: it makes the output a simple linear function of the state, which means (as we will see in Chapter 4) that it can be trained by ordinary linear regression.

The full model is therefore a dynamical system with state $\mathbf{x}_t$, driven by inputs $\mathbf{u}_t$, with outputs $\mathbf{y}_t$ read from the state by a linear map.

## The RNN as a Dynamical System

The state update equation is precisely a **driven discrete-time dynamical system** of the form we studied in Chapter 2:

$$\mathbf{x}_{t+1} = F(\mathbf{x}_t, \mathbf{u}_t)$$

where $F: \mathbb{R}^N \times \mathbb{R}^K \to \mathbb{R}^N$ is defined by:

$$F(\mathbf{x}, \mathbf{u}) = f\!\left(W^{rec}\mathbf{x} + W^{in}\mathbf{u} + \mathbf{b}\right)$$

This is not a superficial resemblance. The RNN *is* a driven dynamical system. The fixed points of the autonomous system $\mathbf{x}^* = F(\mathbf{x}^*, \mathbf{0})$ are the rest states of the network when no input is applied. The eigenvalues of the Jacobian of $F$ at a fixed point determine local stability. Limit cycles are possible. Chaotic dynamics are possible. Everything we learned in Chapter 2 applies here.

The Jacobian of $F$ with respect to the state $\mathbf{x}$, evaluated at a particular state $\mathbf{x}_t$, is:

$$J_t = \frac{\partial F}{\partial \mathbf{x}}\bigg|_{\mathbf{x}_t} = \text{diag}\!\left(f'(W^{rec}\mathbf{x}_t + W^{in}\mathbf{u}_t + \mathbf{b})\right) \cdot W^{rec}$$

That is: $J_t = D_t W^{rec}$, where $D_t \in \mathbb{R}^{N \times N}$ is the diagonal matrix of pointwise derivatives $f'(\cdot)$ evaluated at the pre-activation values. This Jacobian will reappear, over and over, in our analysis of gradients and stability. Its eigenvalues determine whether perturbations to the state grow or shrink over time.

## Comparison with Chapter 2

In Chapter 2, we studied input-driven dynamical systems from a purely dynamical perspective: given a fixed flow $F$ and an input signal, what does the state trajectory look like? We asked questions about the geometry of state space, the nature of attractors, and the conditions under which the system's response is unique and stable.

The RNN introduces something new: the parameters $W^{rec}$, $W^{in}$, and $\mathbf{b}$ are not fixed by physics or design. They are to be *learned* from data. The goal is to find parameter values such that the network computes a useful function of its input history. This is the training problem, and it is what the rest of this chapter is about.

The key conceptual link is this: when we train an RNN, we are not simply fitting a function to data. We are designing the dynamics of a dynamical system so that those dynamics perform a computation. The weights $W^{rec}$ determine the attractor landscape, the time constants, the memory, and the computational substrate. Choosing them well requires propagating error signals backward through the very dynamical system they define — which is where the difficulties begin.

## The Autonomously Recurrent Regime

It is worth noting a special and important case: when the network is run in **generative mode** (also called *autonomous* or *closed-loop* mode), the output is fed back as the next input:

$$\mathbf{u}_{t+1} = \mathbf{y}_t = W^{out}\mathbf{x}_t$$

This makes the network a fully autonomous dynamical system, with no external input:

$$\mathbf{x}_{t+1} = f\!\left(W^{rec}\mathbf{x}_t + W^{in}W^{out}\mathbf{x}_t + \mathbf{b}\right) = f\!\left(\left(W^{rec} + W^{in}W^{out}\right)\mathbf{x}_t + \mathbf{b}\right)$$

In this mode, the network generates sequences from internal dynamics alone. The stability and character of these sequences depend entirely on the combined weight matrix $W^{rec} + W^{in}W^{out}$. This generative mode is important for tasks like sequence generation, motor control, and — as we will see — for training reservoir computers in the FORCE learning framework (Chapter 11).

## Summary

The RNN architecture is defined by three weight matrices ($W^{rec}$, $W^{in}$, $W^{out}$), one bias vector ($\mathbf{b}$), and a nonlinearity ($f$). Its state update equation $\mathbf{x}_{t+1} = f(W^{rec}\mathbf{x}_t + W^{in}\mathbf{u}_t + \mathbf{b})$ is a discrete-time driven dynamical system. Every concept from Chapter 2 — fixed points, stability, Jacobians, attractors — applies directly. The Jacobian $J_t = D_t W^{rec}$ is the central object for both stability analysis and gradient computation.

---

## References

- [Elman1990] Elman, J. L. (1990). Finding structure in time. *Cognitive Science*, 14(2), 179–211.
- [Jordan1986] Jordan, M. I. (1986). Serial order: A parallel distributed processing approach. *ICS Report 8604*, University of California, San Diego.
- [Rumelhart1986] Rumelhart, D. E., Hinton, G. E., & Williams, R. J. (1986). Learning representations by back-propagating errors. *Nature*, 323, 533–536.
