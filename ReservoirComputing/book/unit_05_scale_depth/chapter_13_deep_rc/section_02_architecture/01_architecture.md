# Deep ESN Architecture

## Layer Equations

The Deep Echo State Network (DeepESN) stacks $L$ reservoir layers, each with its own recurrent weight matrix and leak rate. Layer $\ell = 1, \ldots, L$ updates according to:

$$\mathbf{x}_t^{(\ell)} = (1 - \alpha_\ell) \mathbf{x}_{t-1}^{(\ell)} + \alpha_\ell \tanh\!\left(\mathbf{W}^{\text{rec},\ell} \mathbf{x}_{t-1}^{(\ell)} + \mathbf{W}^{\text{in},\ell} \mathbf{x}_t^{(\ell-1)}\right),$$

where $\mathbf{x}_t^{(0)} \equiv \mathbf{u}_t$ is the external input [Gallicchio & Micheli 2017]. The layer state dimension is $\mathbf{x}_t^{(\ell)} \in \mathbb{R}^{N_\ell}$, the recurrent matrix $\mathbf{W}^{\text{rec},\ell} \in \mathbb{R}^{N_\ell \times N_\ell}$ is drawn randomly and fixed, and the inter-layer weight matrix $\mathbf{W}^{\text{in},\ell} \in \mathbb{R}^{N_\ell \times N_{\ell-1}}$ (with $N_0 = d_{\text{in}}$, the input dimension) is also fixed.

The first layer receives the external input:

$$\mathbf{x}_t^{(1)} = (1 - \alpha_1)\mathbf{x}_{t-1}^{(1)} + \alpha_1 \tanh\!\left(\mathbf{W}^{\text{rec},1} \mathbf{x}_{t-1}^{(1)} + \mathbf{W}^{\text{in},1} \mathbf{u}_t\right).$$

Each subsequent layer $\ell \geq 2$ receives the state of layer $\ell - 1$:

$$\mathbf{x}_t^{(\ell)} = (1 - \alpha_\ell)\mathbf{x}_{t-1}^{(\ell)} + \alpha_\ell \tanh\!\left(\mathbf{W}^{\text{rec},\ell} \mathbf{x}_{t-1}^{(\ell)} + \mathbf{W}^{\text{in},\ell} \mathbf{x}_t^{(\ell-1)}\right).$$

Note the temporal indexing: $\mathbf{x}_t^{(\ell-1)}$ uses the current-time state of the layer below, not the previous-time state. This means the information flows through the stack in a feedforward manner within each time step before the recurrent update is applied [Gallicchio & Micheli 2017].

## Readout from All Layers

The readout is trained on the concatenation of all layer states:

$$\mathbf{y}_t = \mathbf{W}^{\text{out}} \begin{bmatrix} \mathbf{x}_t^{(1)} \\ \mathbf{x}_t^{(2)} \\ \vdots \\ \mathbf{x}_t^{(L)} \end{bmatrix}, \qquad \mathbf{W}^{\text{out}} \in \mathbb{R}^{d_{\text{out}} \times \sum_\ell N_\ell}.$$

This design is critical: restricting the readout to only the final layer would throw away the fast-timescale representations computed by lower layers. The concatenation lets the readout freely combine features from all timescales, and the linear readout trained by ridge regression on $[\mathbf{x}_t^{(1)}; \ldots; \mathbf{x}_t^{(L)}]$ finds the optimal linear combination.

## Echo State Property of Deep ESNs

The echo state property (ESP) requires that the network state at time $t$ is uniquely determined by the input history $\{\mathbf{u}_s\}_{s \leq t}$, regardless of initial conditions. For deep ESNs, the ESP decomposes layer by layer.

**Theorem** (Gallicchio & Micheli 2017): A DeepESN has the ESP if and only if each layer independently has the ESP when receiving the appropriate input signal ($\mathbf{u}_t$ for layer 1, and $\mathbf{x}_t^{(\ell-1)}$ for layer $\ell$).

**Proof sketch** (by induction on $\ell$): For layer 1, the ESP is equivalent to the single-layer ESP condition: the spectral radius $\rho(\mathbf{W}^{\text{rec},1}) < 1$ is sufficient (Chapter 5). Assuming layer $\ell - 1$ has the ESP, its states $\mathbf{x}_t^{(\ell-1)}$ are uniquely determined by the input history. Layer $\ell$ is then an ESN driven by the input process $\{\mathbf{x}_t^{(\ell-1)}\}$. If $\rho(\mathbf{W}^{\text{rec},\ell}) < 1$, layer $\ell$ also has the ESP. By induction, the full $L$-layer network has the ESP if $\rho(\mathbf{W}^{\text{rec},\ell}) < 1$ for all $\ell$ [Gallicchio & Micheli 2017]. $\square$

A sufficient condition is therefore:

$$\rho(\mathbf{W}^{\text{rec},\ell}) < 1 \quad \forall \ell = 1, \ldots, L.$$

In practice, one initializes each $\mathbf{W}^{\text{rec},\ell}$ to have the desired spectral radius $\rho_\ell < 1$, typically chosen close to 1 for layers with small $\alpha_\ell$ to extend their effective memory.

## Inter-Layer Connectivity Variants

The cascade architecture described above (layer $\ell$ receives only from layer $\ell - 1$) is the simplest design. Several variants offer different tradeoffs.

**Skip connections:** Layer $\ell$ receives input from all previous layers $1, \ldots, \ell - 1$:

$$\mathbf{W}^{\text{in},\ell} \mathbf{x}_t^{(\ell-1)} \to \sum_{j < \ell} \mathbf{W}^{\text{in},\ell j} \mathbf{x}_t^{(j)}.$$

This ensures that rapid features from layer 1 are directly available to all higher layers, at the cost of increased inter-layer weight parameters.

**Full connectivity:** Each layer receives from all other layers and the input, including feedback from higher layers to lower layers. This creates a globally recurrent system that is more expressive but more difficult to analyze (the ESP inductive argument no longer applies directly).

**Hierarchical readout:** Instead of reading from all layers simultaneously, use a per-layer readout $\mathbf{y}_t^{(\ell)} = \mathbf{W}_\ell^{\text{out}} \mathbf{x}_t^{(\ell)}$ and combine them through a second stage of ridge regression. This allows explicit measurement of each layer's contribution to the final output.

## Implementation Considerations

The total state dimension of a DeepESN is $N_{\text{total}} = \sum_\ell N_\ell$. For fixed total parameter budget, one can choose equal layer sizes ($N_\ell = N_{\text{total}} / L$) or pyramid architectures ($N_1 > N_2 > \cdots > N_L$). Equal-size layers are the standard baseline; pyramid architectures reflect the intuition that higher layers need to represent more compressed, abstract information [Gallicchio & Micheli 2017].

---

## References

- Gallicchio, C., & Micheli, A. (2017). Echo state property of deep reservoir computing networks. *Cognitive Computation*, 9(3), 337–350.
