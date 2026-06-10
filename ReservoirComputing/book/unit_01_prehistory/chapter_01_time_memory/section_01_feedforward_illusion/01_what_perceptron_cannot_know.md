# 1.1.1 What a Perceptron Can and Cannot Know

## The Architecture of Now

A perceptron — the simplest neural processing unit — computes a weighted sum of its inputs and passes the result through a nonlinearity:

$$y = \sigma\left(\sum_{i=1}^n w_i x_i + b\right)$$

This computation is instantaneous. The inputs $x_1, \ldots, x_n$ arrive, the weights $w_1, \ldots, w_n$ act on them, and the output $y$ emerges. The computation has no duration; it does not accumulate anything over time; it has no relationship with any previous computation performed by the same unit.

A multilayer feedforward network extends this: each layer applies a linear transformation followed by a pointwise nonlinearity, and layers are chained so that the final output is a composition of these maps:

$$\mathbf{y} = f_L \circ f_{L-1} \circ \cdots \circ f_1(\mathbf{x})$$

This is a more complex function, but it is still a function — a static mapping from input $\mathbf{x}$ to output $\mathbf{y}$. The network has no state that persists between calls. You can, of course, run it on a sequence of inputs $\mathbf{x}_1, \mathbf{x}_2, \mathbf{x}_3, \ldots$ and get a sequence of outputs $\mathbf{y}_1, \mathbf{y}_2, \mathbf{y}_3, \ldots$, but each output depends only on the corresponding input:

$$\mathbf{y}_t = F(\mathbf{x}_t)$$

The subscript $t$ is irrelevant. It could just as well be an unordered set of inputs as a temporal sequence. Order does not matter; time does not matter; history does not matter.

## The Expressive Power of Feedforward Networks

Before going further, let's be precise about what feedforward networks *can* do, lest we understate their power. The Universal Approximation Theorem [Cybenko1989, Hornik1989] tells us that a feedforward network with a single hidden layer of sufficient width can approximate any continuous function $f: \mathbb{R}^n \to \mathbb{R}^m$ to arbitrary accuracy on compact sets, given appropriate nonlinear activation functions.

This is a profound result. It means that the class of computable functions is not what limits feedforward networks. Any pattern that exists within a single, fixed-dimensional input — any structure, any nonlinearity, any interaction among features — can in principle be captured.

The limitation is more subtle and more fundamental than expressiveness. It is about *what the input represents*. If the input to the network is the current state of the world at time $t$, then the network's output can only depend on the current state. No configuration of weights, no matter how clever, can make the output depend on what happened at time $t-1$, or $t-5$, or $t-100$, because that information is simply not present in the input.

This is not a failure of architecture. It is a failure of interface. You are asking the network to tell you something the network cannot be told.

## A Precise Statement of the Limitation

Let us state this precisely. Define a **causal functional** as a mapping $H$ from sequences to sequences such that the output at time $t$ depends only on inputs up to time $t$:

$$y_t = H[\mathbf{u}]_t = H(u_t, u_{t-1}, u_{t-2}, \ldots)$$

A feedforward network $F$ with fixed input dimension $n$ can only compute:

$$y_t = F(u_t, u_{t-1}, \ldots, u_{t-n+1})$$

That is, it computes a causal functional, but only one that depends on a *finite, fixed window* of the past. Crucially, the window size $n$ is fixed at design time. The network cannot adapt its effective memory to the task.

**Theorem (Informal):** There exist causal functionals with fading memory that cannot be approximated to arbitrary accuracy by any feedforward network with fixed input dimension, regardless of the network's depth or width.

The proof is simple: consider the functional $H[\mathbf{u}]_t = u_{t-k}$ for any $k > n$. This returns the input from $k$ time steps ago. A feedforward network with $n$-dimensional input cannot even represent the inputs at time $t - k$ for $k > n$, let alone compute a function of them.

## The Fundamental Asymmetry

There is something philosophically interesting happening here that is worth pausing to consider.

A feedforward network's representational capacity and its temporal scope are coupled. If you want more temporal reach, you need more input dimensions. If you want more input dimensions, you need more parameters. If you need more parameters, you need more training data. The complexity of learning grows with the temporal scope you require.

But the world's temporal dependencies do not cooperate with this constraint. A language model needs to track dependencies across hundreds or thousands of words. An ecological system's current state may depend sensitively on events from years ago. A climate system integrates forcing over decades.

The feedforward network, for all its power within its domain, is architecturally committed to the present moment. To engage with the past, it needs a fundamentally different kind of machinery.

---

**Key Insight:** The limitation of feedforward networks in temporal tasks is not about expressiveness or depth. It is about the fixed, finite nature of the input — which means temporal dependence must be explicitly encoded in the input, at design time, rather than learned from data. This design-time commitment is what a recurrent or reservoir architecture eliminates.

---

## References

- [Cybenko1989] Cybenko, G. (1989). Approximation by superpositions of a sigmoidal function. *Mathematics of Control, Signals and Systems*, 2(4), 303–314.
- [Hornik1989] Hornik, K., Stinchcombe, M., & White, H. (1989). Multilayer feedforward networks are universal approximators. *Neural Networks*, 2(5), 359–366.
- [Funahashi1989] Funahashi, K. (1989). On the approximate realization of continuous mappings by neural networks. *Neural Networks*, 2(3), 183–192.
