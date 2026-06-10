# Chapter 13 — Key Concepts

---

## Deep ESN

An Echo State Network with $L > 1$ stacked reservoir layers, where the state of each layer serves as the input to the layer above. The governing equation for layer $\ell$ is:

$$\mathbf{x}_t^{(\ell)} = (1-\alpha_\ell)\mathbf{x}_{t-1}^{(\ell)} + \alpha_\ell \tanh\!\left(W_\ell^{rec}\mathbf{x}_{t-1}^{(\ell)} + W_\ell^{in}\mathbf{x}_t^{(\ell-1)}\right)$$

The readout is trained on the concatenation of all layer states. Deep ESNs are particularly effective for tasks with multi-scale temporal structure, where different layers capture different timescales simultaneously.

---

## Layerwise Echo State Property

The requirement that the echo state property holds at every layer of a deep ESN. Because each layer is driven by the layer below (not by the external input directly), the ESP must be established inductively: if layer $\ell-1$ satisfies the ESP, then layer $\ell$ is driven by a bounded, uniquely-determined signal, and its ESP follows from the standard single-layer contraction argument. The sufficient condition requires the spectral radius of $W_\ell^{rec}$ to be less than 1 at each layer, and the tanh nonlinearity ensures bounded states.

---

## Effective Memory Time Constant

For a single leaky integrator with leaking rate $\alpha$, the time constant is $\tau \approx 1/\alpha$ — the time for an impulse response to decay to $e^{-1}$ of its peak. In a deep ESN with equal leaking rates, the effective time constant at layer $\ell$ is approximately $\tau^{(\ell)} \approx \ell/\alpha$, growing linearly with depth. This is because stacking layers is equivalent to convolving exponential filters, producing a Gamma-distributed impulse response with mean proportional to the depth.

---

## Timescale Hierarchy

The architectural property of deep ESNs whereby lower layers operate on short timescales (fast dynamics, high-frequency response) and upper layers operate on long timescales (slow dynamics, low-frequency response). This hierarchy is analytically derivable as a consequence of the cascade of low-pass filters implicit in the deep architecture. It can be amplified by choosing decreasing leaking rates: $\alpha_1 > \alpha_2 > \cdots > \alpha_L$.

---

## Inter-Layer Weight Matrix

The matrix $W_\ell^{in} \in \mathbb{R}^{N_\ell \times N_{\ell-1}}$ that maps the state of layer $\ell-1$ into the input space of layer $\ell$. Like the recurrent matrices, this is randomly initialized and fixed (not trained). It determines how information is projected between layers and, together with the spectral radii, controls the timescale separation.

---

## Concatenated Readout

The practice of training the output weights on the concatenation $[\mathbf{x}_t^{(1)\top}, \ldots, \mathbf{x}_t^{(L)\top}]^\top$ of all layer states. This allows the readout to use information at all timescales simultaneously. Empirically, Gallicchio and Micheli [Gallicchio2017b] show that concatenated readouts consistently outperform readouts trained on any single layer alone.

---

## Cascade of Low-Pass Filters

The frequency-domain interpretation of a deep ESN stack. Each leaky integration layer acts as a low-pass filter with cutoff frequency approximately $\alpha_\ell$. The cascade of $L$ layers filters the input through $L$ progressively narrower bandpass filters. The result: layer $\ell$ state contains predominantly the low-frequency content of the input that has persisted for at least $\ell/\alpha$ time steps.

---

## Graph ESN

An ESN architecture where the input is a graph $G = (V, E, \mathbf{X})$ rather than a vector. Each node $i$ maintains a hidden state that is updated by aggregating information from its own previous state and its neighbors' previous states:

$$\mathbf{h}_t^{(i)} = (1-\alpha)\mathbf{h}_{t-1}^{(i)} + \alpha\tanh\!\left(W^{rec}\mathbf{h}_{t-1}^{(i)} + W^{in}\mathbf{x}_t^{(i)} + W^{nb}\sum_{j \in \mathcal{N}(i)}\mathbf{h}_{t-1}^{(j)}\right)$$

The ESP condition for Graph ESNs involves both the spectral radius of $W^{rec}$ and the spectral norm of the adjacency matrix.

---

## Neighbor Aggregation Matrix

The matrix $W^{nb} \in \mathbb{R}^{N \times N}$ in a Graph ESN that weights information arriving from neighboring nodes. Together with $W^{rec}$, it determines the graph ESP condition: $\rho(W^{rec}) + \rho(A) \cdot \rho(W^{nb}) < 1$. This matrix is analogous to the inter-layer matrix $W_\ell^{in}$ in the standard deep ESN: it couples units that are spatially (rather than temporally) distinct.

---

## Intrinsic Plasticity and State Diversity

A metric for evaluating whether layers in a deep ESN are providing genuinely complementary representations. Layers with high state entropy (diverse, distributed activations) contribute more useful features to the readout than layers where neurons are predominantly saturated or quiescent. Gallicchio et al. [Gallicchio2017b] use state entropy as a proxy for the information contributed by each layer and show it decreases with layer depth — consistent with the timescale hierarchy picture.

---

## Skip Connections in Deep ESNs

Optional direct connections from the input $\mathbf{u}_t$ to every layer, not just layer 1. Skip connections prevent the vanishing of the input signal in deep stacks and add a residual path that bypasses the slow upper-layer dynamics. They are analogous to residual connections in deep feedforward networks and can be important for tasks that require simultaneous processing at all timescales.
