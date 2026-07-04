# Subsection 13.1.2: Backpropagation

## Orientation

Training a network means finding weights that minimize a loss function, and every practical method does so by gradient descent: nudge each of the millions of weights in the direction that reduces the loss. The miracle that makes this affordable is backpropagation — an organization of the chain rule that computes the gradient with respect to *every* parameter for roughly the cost of three forward passes. This subsection derives the algorithm cleanly, then reads it with photonic eyes: what hardware would have to exist for the gradient computation itself to run through the optics.

---

## 13.1.2.1 The Setup

Given a training example $(\mathbf{x}, \mathbf{y})$, the network produces $\hat{\mathbf{y}} = \mathbf{a}^{(L)}$ and incurs a loss $\mathcal{L}(\hat{\mathbf{y}}, \mathbf{y})$ — mean squared error for regression, cross-entropy $\mathcal{L} = -\sum_i y_i \log p_i$ (with softmax $p$) for classification. Training minimizes the loss averaged over data by stochastic gradient descent (SGD):

$$W^{(l)} \leftarrow W^{(l)} - \eta \, \frac{\partial \mathcal{L}}{\partial W^{(l)}}$$

with learning rate $\eta$, usually dressed with momentum and per-parameter scaling (the Adam optimizer). Everything reduces to computing $\partial\mathcal{L}/\partial W^{(l)}$ efficiently.

## 13.1.2.2 The Backward Recursion

Define the **error vector** at layer $l$ as the gradient of the loss with respect to that layer's pre-activation:

$$\boldsymbol{\delta}^{(l)} \equiv \frac{\partial \mathcal{L}}{\partial \mathbf{z}^{(l)}}$$

Applying the chain rule to the layer recursion $\mathbf{z}^{(l+1)} = W^{(l+1)} f(\mathbf{z}^{(l)}) + \mathbf{b}^{(l+1)}$ gives the central result, the *backward recursion*:

$$\boxed{\;\boldsymbol{\delta}^{(l)} = \left(W^{(l+1)}\right)^{T} \boldsymbol{\delta}^{(l+1)} \odot f'\!\left(\mathbf{z}^{(l)}\right)\;}$$

where $\odot$ is the elementwise product. The recursion starts at the output — for softmax-plus-cross-entropy it takes the memorably simple form $\boldsymbol{\delta}^{(L)} = \mathbf{p} - \mathbf{y}$ — and runs backward to the first layer. The parameter gradients then fall out as outer products of quantities already in hand:

$$\frac{\partial \mathcal{L}}{\partial W^{(l)}} = \boldsymbol{\delta}^{(l)} \left(\mathbf{a}^{(l-1)}\right)^{T}, \qquad \frac{\partial \mathcal{L}}{\partial \mathbf{b}^{(l)}} = \boldsymbol{\delta}^{(l)}$$

**Cost accounting.** The backward recursion is one matrix-vector product per layer (by $W^T$) — the same cost as the forward pass. The weight-gradient outer products cost another $N_l N_{l-1}$ MACs per layer. Total: training arithmetic $\approx 3\times$ inference arithmetic per example, all of it, once again, matrix multiplication. Backpropagation also requires *memory*: every $\mathbf{a}^{(l)}$ and $\mathbf{z}^{(l)}$ from the forward pass must be stored until the backward pass consumes it.

---

## 13.1.2.3 Reading the Algorithm as a Photonic Engineer

Three features of backpropagation determine everything about photonic training hardware.

**1. The backward pass multiplies by the transpose.** A photonic layer that implements $W$ must, for training, also implement $W^T$. Here physics is unexpectedly generous: waveguide optics is *reciprocal* (Chapter 6). Light injected backward through a linear photonic circuit experiences the transposed transfer matrix — send $\boldsymbol{\delta}^{(l+1)}$ into the *output* ports of the mesh that implements $W^{(l+1)}$, and what emerges from the input ports is $(W^{(l+1)})^T \boldsymbol{\delta}^{(l+1)}$, computed at light speed by the same hardware. (For the complex-valued case, propagating backward gives $W^T$, not the conjugate transpose $W^\dagger$; phase-conjugation subtleties are handled in the in-situ training protocols of Section 13.3.2.) No electronic accelerator gets its transpose this cheaply.

**2. The elementwise factor $f'(\mathbf{z})$ requires knowing the derivative of the actual hardware nonlinearity.** If the activation is electronic, $f'$ is known exactly. If it is a physical electro-optic or all-optical device, $f$ is whatever the device does — temperature-dependent, device-varying — and $f'$ must be measured or modeled. Error in $f'$ biases every gradient upstream of it. This is a principal reason hybrid schemes keep the nonlinearity in the well-characterized electronic domain.

**3. The weight update $\boldsymbol{\delta}\,\mathbf{a}^T$ must land *in the weights*.** Computing gradients optically is pointless if applying them takes microseconds per phase shifter. With thermo-optic weights (update time $\sim$10 μs, Section 12.2.4) a mesh of $10^3$ phases updates in tens of milliseconds — $10^6\times$ slower than a GPU's weight update from SRAM. This mismatch between fast analog inference and slow analog weight writing, more than any other single number, is why photonic *training* remains mostly aspirational while photonic *inference* is commercial. Section 13.3 surveys the attempts to break the impasse; Chapter 16's photonic synapses attack the device problem directly.

---

## 13.1.2.4 Worked Example: Gradients Through a Two-Layer Network

Let a 2-layer network classify with $N_0 = 4$ inputs, $N_1 = 3$ hidden ReLU units, $N_2 = 2$ softmax outputs. Forward: $\mathbf{z}^{(1)} = W^{(1)}\mathbf{x} + \mathbf{b}^{(1)}$; $\mathbf{a}^{(1)} = \text{ReLU}(\mathbf{z}^{(1)})$; $\mathbf{z}^{(2)} = W^{(2)}\mathbf{a}^{(1)} + \mathbf{b}^{(2)}$; $\mathbf{p} = \text{softmax}(\mathbf{z}^{(2)})$.

Backward, for true class $\mathbf{y} = (1, 0)^T$:

1. $\boldsymbol{\delta}^{(2)} = \mathbf{p} - \mathbf{y}$ — 2 subtractions.
2. $\boldsymbol{\delta}^{(1)} = (W^{(2)})^T \boldsymbol{\delta}^{(2)} \odot \mathbb{1}[\mathbf{z}^{(1)} > 0]$ — a $3\times2$ transposed matrix-vector product (6 MACs) and a mask.
3. Gradients: $\partial\mathcal{L}/\partial W^{(2)} = \boldsymbol{\delta}^{(2)}(\mathbf{a}^{(1)})^T$ (6 MACs); $\partial\mathcal{L}/\partial W^{(1)} = \boldsymbol{\delta}^{(1)}\mathbf{x}^T$ (12 MACs).

Forward pass: $4\times3 + 3\times2 = 18$ MACs. Backward + gradients: $6 + 6 + 12 = 24$ MACs. The 3:1 training-to-inference arithmetic ratio, and the dominance of matrix products, are both visible even at toy scale. In a photonic implementation, steps using $W$ and $W^T$ are optical transits (forward and backward through the same mesh); the masks, outer products, and updates are electronic — and the updates are the slow part.

---

## References

[1] Rumelhart, D.E., Hinton, G.E., & Williams, R.J. (1986). "Learning representations by back-propagating errors." *Nature*, 323, 533–536. [The paper that established backpropagation as the training algorithm for multilayer networks.]

[2] Goodfellow, I., Bengio, Y., & Courville, A. (2016). *Deep Learning*. MIT Press. [Chapter 6.5 gives the full backpropagation derivation with computational-graph generality.]

[3] Hughes, T.W., Minkov, M., Shi, Y., & Fan, S. (2018). "Training of photonic neural networks through in situ backpropagation and gradient measurement." *Optica*, 5(7), 864–871. [Shows how the backward recursion — including the transpose and the gradient extraction — maps onto physical light propagation in a reciprocal mesh; developed fully in Section 13.3.2.]

[4] Kingma, D.P., & Ba, J. (2015). "Adam: A method for stochastic optimization." *International Conference on Learning Representations (ICLR)*. [The optimizer used, in practice, for every photonic-hardware training result cited in this chapter.]
