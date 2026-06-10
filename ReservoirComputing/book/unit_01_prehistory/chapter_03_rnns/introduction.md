# Chapter 3: Recurrent Neural Networks

## The Promise and the Wall

There is a particular kind of clarity that comes from getting the architecture right. When you look at a recurrent neural network for the first time — really look at it, past the diagram and into the mathematics — you feel something like recognition. Of course. Of course the way to process sequences is to maintain state. Of course the way to incorporate history is to feed the present moment through a function that also receives the accumulated past. Of course the natural model of a temporal processor is a dynamical system.

The recurrent neural network is the most natural answer to the question posed in Chapter 1: how do we build a machine that knows its own history? The feedforward network's limitation was architectural — it had no mechanism for state, no internal record of what had come before. The RNN resolves this not by engineering a workaround, but by taking the problem seriously. It says: computation over time should be a dynamical system, and dynamical systems have state.

This is a beautiful insight. And it works. RNNs are universal approximators of dynamical systems [Siegelmann1995]. In principle, a recurrent network with enough units and the right weights can simulate any computable function over sequences. They can learn grammar, track musical structure, model protein folding dynamics, predict financial time series. The theoretical case is overwhelming.

And yet for most of the 1990s, training them was nearly impossible.

The wall is not in the architecture. The wall is in the gradient. To train an RNN, you must propagate error signals backward through time — and time is deep. A sequence of length 100 is, from the gradient's perspective, a network with 100 layers. The rules of calculus, applied to this depth, produce a product of 100 Jacobian matrices. Products of matrices can do strange things. They can collapse to zero exponentially fast, making error signals whisper instead of shout. They can explode to infinity, making error signals into noise. This is the vanishing gradient problem, and it is not a minor inconvenience. It is the reason that RNNs, for all their theoretical power, largely failed to learn long-range temporal dependencies in practice.

Understanding this wall — really understanding why it exists, what it implies mathematically, and why it is so hard to avoid — is the central task of this chapter. We will derive the gradient equations completely, examine the spectral conditions that determine whether gradients vanish or explode, and look honestly at what gradients actually look like at depth 100.

Along the way, we will also examine the heroic efforts to climb over the wall: the Long Short-Term Memory network [Hochreiter1997], which uses gating mechanisms to protect gradient flow; Real-Time Recurrent Learning [Williams1989], which avoids unrolling through time altogether; and various other approaches. These are genuine contributions, and they matter.

But the chapter ends somewhere else. It ends with a question that, at the time it was first asked, must have seemed almost too simple: what if we simply did not train the recurrent weights at all?

This question — naive on its surface, profound in its implications — is the pivot on which reservoir computing turns. To appreciate why the answer is not "obviously that won't work," you need to understand the full depth of the wall that preceded it. That is what this chapter builds.

## The Chapter's Arc

Section 3.1 establishes the RNN as a dynamical system — formally, carefully, connecting it to the framework of Chapter 2. We examine the state update equation in full dimensional detail, discuss the roles of the recurrent weight matrix and the input weight matrix, and compare the driven RNN to the driven dynamical systems we studied before.

Section 3.2 addresses what RNNs can in principle represent. This is the good news: Turing completeness, universal approximation over sequences, the expressive case for recurrent computation.

Section 3.3 is where we meet backpropagation through time. We unroll the RNN, write out the full computational graph, and derive the gradient $\partial L / \partial W^{rec}$ step by step. Nothing is skipped. Every matrix, every chain rule application, every subscript is accounted for.

Section 3.4 confronts the gradient problem directly. We prove why products of Jacobians cause gradients to vanish or explode, derive the spectral radius condition, and look at what the gradient landscape actually looks like.

Sections 3.5 and 3.6 survey solutions: LSTM, GRU, RTRL, and their properties. These are partial solutions — they help, but they do not fully resolve the underlying tension.

Section 3.7 is the pivot. Having built the full case for why training RNNs is hard, we ask the question that breaks the problem open: what if we don't?

## A Note on Notation

Throughout this chapter, we use bold lowercase for vectors ($\mathbf{x}$, $\mathbf{u}$, $\mathbf{b}$), bold uppercase for matrices ($W^{rec}$, $W^{in}$), and standard italic for scalars and indices. Time indices are superscripts in parentheses ($x^{(t)}$) when the vector's components also need subscripts, and plain subscripts ($x_t$) when the context is unambiguous. The nonlinearity $f$ is taken to be $\tanh$ unless otherwise noted — the specific choice matters less than its properties, but $\tanh$ has the best-behaved derivative for our analysis.

---

## References

- [Siegelmann1995] Siegelmann, H. T., & Sontag, E. D. (1995). On the computational power of neural nets. *Journal of Computer and System Sciences*, 50(1), 132–150.
- [Hochreiter1997] Hochreiter, S., & Schmidhuber, J. (1997). Long short-term memory. *Neural Computation*, 9(8), 1735–1780.
- [Williams1989] Williams, R. J., & Zipser, D. (1989). A learning algorithm for continually running fully recurrent neural networks. *Neural Computation*, 1(2), 270–280.
