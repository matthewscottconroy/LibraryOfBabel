# Expressiveness of Deep Reservoirs

## Function Classes and Depth Separation

A central question in deep learning theory is whether deeper networks are strictly more expressive than shallower ones, or merely more parameter-efficient. The landmark result of Telgarsky [2016] established depth separation for feedforward networks: there exist functions computable by a depth-$2k$ ReLU network with $O(N)$ neurons per layer that require $\Omega(N^k)$ neurons to compute with a depth-$k$ network. This exponential separation holds for specific "zigzag" functions and is the theoretical foundation for preferring deep over shallow architectures.

Do analogous depth-separation results apply to deep reservoir computing? The answer is partial and requires careful framing, because the expressiveness of a reservoir computing architecture involves two distinct components: the expressiveness of the reservoir state representation and the expressiveness of the linear readout trained on that representation.

## Expressiveness of the Reservoir State

The reservoir state $\mathbf{x}_t^{(\ell)} \in \mathbb{R}^{N_\ell}$ at layer $\ell$ is a nonlinear function of the input history $\{\mathbf{u}_s\}_{s \leq t}$. The function class representable by the $L$-layer DeepESN state is strictly richer than the class representable by the $(L-1)$-layer state, under mild conditions. The intuition is that layer $\ell$ can compute functions of the layer $(\ell-1)$ state — which is itself a nonlinear function of the input history — giving a composition of $\ell$ nonlinear operators. Each additional layer adds a new level of temporal abstraction [Gallicchio & Micheli 2017].

More precisely, the space of functions $u_{(-\infty,t]} \mapsto x_t^{(\ell)}$ forms a function space $\mathcal{F}_\ell$. The cascade structure guarantees $\mathcal{F}_1 \subseteq \mathcal{F}_2 \subseteq \cdots \subseteq \mathcal{F}_L$, with strict containment when the additional layers are nontrivial (spectral radius $\rho_\ell > 0$ and $N_\ell > 0$). The concatenated state $[\mathbf{x}_t^{(1)}; \ldots; \mathbf{x}_t^{(L)}]$ provides access to the union $\bigcup_\ell \mathcal{F}_\ell$, which is the richest representation available.

## Depth Separation for Reservoir States

The depth-separation results of Telgarsky [2016] do not apply directly to reservoir states because: (1) reservoir states are recurrent, not feedforward; (2) the "width" of a reservoir layer is not the same as the width of a feedforward layer; (3) the universal approximation theorem for reservoirs (Boyd–Chua) makes different assumptions than feedforward universality. Nevertheless, the structural argument for depth — that composition of temporal operators provides richer representations than a single operator — is sound and supported empirically.

For the concatenated state representation, the relevant metric is whether the linear readout trained on $[\mathbf{x}_t^{(1)}; \ldots; \mathbf{x}_t^{(L)}]$ can achieve lower error than on any single $\mathbf{x}_t^{(\ell)}$. Because the concatenation is a superset of any single layer's features, the answer is always yes or equal — but the gain depends on whether the additional layers provide truly complementary (linearly independent) information. Gallicchio & Micheli [2017] demonstrated empirically that layers at different depths provide information about the input at different timescales, and these timescale-specific representations are genuinely complementary, yielding strict improvement from concatenation.

## Graph ESNs and Kronecker Product Conditions

The expressiveness of reservoir architectures extends to structured input domains. Graph Echo State Networks (GraphESN) define the reservoir over an input graph topology, where each node $v$ in the input graph has its own reservoir state $\mathbf{x}_v$, and the recurrence couples reservoir nodes according to the graph structure [Gallicchio & Micheli 2020]:

$$\mathbf{x}_v^{(t)} = (1-\alpha)\mathbf{x}_v^{(t-1)} + \alpha \tanh\!\left(\mathbf{W}^{\text{rec}} \mathbf{x}_v^{(t-1)} + \sum_{u \in \mathcal{N}(v)} \mathbf{W}^{\text{nb}} \mathbf{x}_u^{(t-1)} + \mathbf{W}^{\text{in}} \mathbf{u}_v\right),$$

where $\mathcal{N}(v)$ are the graph neighbors of $v$. The ESP for GraphESNs involves the Kronecker product of the graph adjacency matrix and the reservoir weight matrix. Specifically, the sufficient condition for ESP is:

$$\rho(\mathbf{A}_G \otimes \mathbf{W}^{\text{nb}}) < 1,$$

where $\mathbf{A}_G$ is the graph adjacency matrix. This condition ensures that the joint graph-reservoir dynamical system contracts, allowing the reservoir to encode graph-structured input history in a linearly accessible form [Gallicchio & Micheli 2020].

## The Representation Power of Concatenated States

The concatenated state $\mathbf{s}_t = [\mathbf{x}_t^{(1)}; \ldots; \mathbf{x}_t^{(L)}] \in \mathbb{R}^{\sum_\ell N_\ell}$ combines temporal features at all timescales. For a task requiring information at $M$ distinct timescales, a single-layer reservoir of size $N$ can represent at most one timescale per layer. A DeepESN with $L = M$ layers and appropriate $\alpha_\ell$ can represent all $M$ timescales simultaneously, with each layer's contribution being orthogonal in the frequency domain.

The number of linearly independent functions of the input history representable by the concatenated state is bounded by $\sum_\ell N_\ell$ — the total reservoir dimension. This bound is tight when each layer's states are linearly independent of all other layers' states, which holds when the layers operate at sufficiently different timescales [Gallicchio & Micheli 2017].

---

## References

- Telgarsky, M. (2016). Benefits of depth in neural networks. *Proceedings of the 29th Conference on Learning Theory (COLT)*, 1517–1539.
- Gallicchio, C., & Micheli, A. (2017). Echo state property of deep reservoir computing networks. *Cognitive Computation*, 9(3), 337–350.
- Gallicchio, C., & Micheli, A. (2020). Ring reservoir neural networks for graphs. *International Joint Conference on Neural Networks (IJCNN)*, 1–7.
