# Autoconceptors: Self-Organizing Pattern Learning

## The Offline Limitation

The conceptor framework as described in Sections 10.4.1–10.4.4 operates in a batch mode: present each pattern, collect states, compute the correlation matrix, invert. This procedure is inherently offline. All patterns must be presented before recall begins, and if a new pattern arrives, the full storage procedure must be repeated. For applications requiring continuous, incremental learning from a stream of inputs — such as a robot that learns new movements over its lifetime — an online variant is needed. Autoconceptors provide this capability [Jaeger 2014].

## The Online Update Rule

An autoconceptor updates the conceptor matrix $\mathbf{C}_t$ incrementally as each new reservoir state $\mathbf{x}_t$ arrives. The update rule is:

$$\mathbf{C}_t = \mathbf{C}_{t-1} + \eta \left( \mathbf{x}_t \mathbf{x}_t^\top - \mathbf{C}_{t-1} \mathbf{x}_t \mathbf{x}_t^\top \mathbf{C}_{t-1} - \alpha^{-2} \mathbf{C}_{t-1} \right),$$

where $\eta > 0$ is a learning rate and $\alpha$ is the aperture parameter. This is a matrix-valued gradient descent step on the objective function:

$$\mathcal{L}(\mathbf{C}) = \left\| \mathbf{x} - \mathbf{C} \mathbf{x} \right\|^2 + \alpha^{-2} \left\| \mathbf{C} \right\|_F^2.$$

The first term penalizes the reconstruction error of projecting $\mathbf{x}$ through $\mathbf{C}$; the second term penalizes the Frobenius norm of $\mathbf{C}$, encouraging parsimony [Jaeger 2014].

## Gradient Derivation

Taking the gradient of $\mathcal{L}$ with respect to $\mathbf{C}$ and simplifying using the identity $\partial_{\mathbf{C}} \|\mathbf{x} - \mathbf{C}\mathbf{x}\|^2 = -2(\mathbf{x} - \mathbf{C}\mathbf{x})\mathbf{x}^\top$:

$$\nabla_{\mathbf{C}} \mathcal{L} = -2(\mathbf{x} - \mathbf{C}\mathbf{x})\mathbf{x}^\top + 2\alpha^{-2} \mathbf{C}.$$

Setting the symmetrized gradient to zero and rearranging gives the stationary condition

$$\mathbf{C}(\mathbf{R} + \alpha^{-2} \mathbf{I}) = \mathbf{R},$$

where $\mathbf{R} = \mathbb{E}[\mathbf{x}\mathbf{x}^\top]$. This yields exactly the offline conceptor $\mathbf{C} = \mathbf{R}(\mathbf{R} + \alpha^{-2}\mathbf{I})^{-1}$. The autoconceptor update is therefore gradient descent on a convex objective whose unique minimum is the offline conceptor. The update $\mathbf{C}_t = \mathbf{C}_{t-1} + \eta \nabla_{\mathbf{C}} \mathcal{L}$ (with appropriate sign) converges to the offline conceptor under mild stationarity conditions [Jaeger 2014].

## Convergence Properties

Let $\mathbf{C}^* = \mathbf{R}(\mathbf{R} + \alpha^{-2}\mathbf{I})^{-1}$ be the offline conceptor. Under the assumption that the reservoir state sequence $\{\mathbf{x}_t\}$ is ergodic with correlation $\mathbf{R}$, the autoconceptor satisfies:

$$\mathbb{E}[\|\mathbf{C}_t - \mathbf{C}^*\|_F^2] \to 0 \quad \text{as} \quad t \to \infty,$$

provided $\eta$ satisfies the Robbins–Monro conditions $\sum_t \eta_t = \infty$, $\sum_t \eta_t^2 < \infty$. For fixed $\eta$, the autoconceptor converges to a neighborhood of $\mathbf{C}^*$ with radius $O(\eta \text{tr}(\mathbf{R}))$.

The convergence is to the offline solution — meaning online and offline conceptors are asymptotically equivalent given sufficient data. In practice, the autoconceptor adapts within a few hundred steps, making it suitable for continual learning scenarios [Jaeger 2014].

## Applications: Continual Learning Without Catastrophic Forgetting

Standard neural network training suffers from catastrophic forgetting: learning a new task overwrites the weights needed for previous tasks. Autoconceptors offer a partial solution. Each pattern $p_k$ develops its own conceptor $\mathbf{C}_k$ by online adaptation while pattern $k$ is being presented. The conceptors for different patterns are stored separately and do not interfere with each other, because each conceptor is a matrix property of the reservoir's response to that pattern — not a modification of the reservoir itself.

When a new pattern $p_{K+1}$ is presented, the existing conceptors $\{\mathbf{C}_1, \ldots, \mathbf{C}_K\}$ remain unchanged. The autoconceptor for $p_{K+1}$ is learned from scratch using the online update rule. The reservoir weights $\mathbf{W}^{\text{rec}}$ are never modified, which is the key reason for the forgetting-free property. This is a direct consequence of the reservoir computing paradigm: only the readout (and here, the conceptors) change during learning [Jaeger 2014].

## Relation to Hopfield Networks and Modern Associative Memory

Hopfield networks [Hopfield 1982] store patterns as fixed points of a recurrent network. Retrieval is pattern completion: start from a noisy version, converge to the stored attractor. The storage capacity of a Hopfield network is $\sim 0.14 N$ patterns for $N$ neurons.

Conceptors store patterns as subspaces rather than fixed points, giving substantially larger capacity: $K$ patterns can be stored as long as $\sum_k \text{rank}(\mathbf{C}_k) \leq N$, which for low-dimensional patterns allows $K \gg 0.14 N$.

Modern Hopfield networks [Ramsauer et al. 2021] use an energy function with polynomial or exponential terms to achieve exponential storage capacity $\sim 2^{N/2}$. These can be interpreted as attention mechanisms in transformers. The connection to conceptors is structural: both frameworks use state correlation matrices and projection operations to manage pattern storage. However, modern Hopfield networks are designed for static pattern retrieval, while conceptors manage dynamical trajectories — a fundamentally different and harder problem [Ramsauer et al. 2021].

---

## References

- Jaeger, H. (2014). Controlling recurrent neural networks by conceptors. *arXiv preprint*, arXiv:1403.3369.
- Ramsauer, H., Schäfl, B., Lehner, J., Seidl, P., Widrich, M., Adler, T., ... & Hochreiter, S. (2021). Hopfield networks is all you need. *International Conference on Learning Representations*.
