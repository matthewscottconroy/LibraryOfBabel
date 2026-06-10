# Symbol Binding and Variable Binding in Reservoir Computing

## The Binding Problem

The binding problem asks how a cognitive system associates values with variables — how it represents not just that "red" and "circle" are both present, but that the circle is red (not that the square is red) [Smolensky 1990]. This is a fundamental problem for connectionist systems, which represent information in distributed activation patterns that do not naturally distinguish "which feature belongs to which object."

Classical symbolic artificial intelligence solves binding trivially through data structures: a record with fields `shape = circle` and `color = red` explicitly associates values with variables. Connectionist systems, including reservoirs, must represent bindings implicitly in activation patterns — a fundamentally harder problem.

For reservoir computing specifically, the binding problem arises in tasks requiring the system to associate temporal events: "the tone at $t = 1$ was followed by the light at $t = 5$" requires binding the tone (variable: first event) to its value (tone) and the light (variable: second event) to its value (light), while distinguishing this from the reverse order.

## Reservoir Approach: Temporal Co-occurrence Binding

The simplest reservoir approach to binding exploits temporal co-occurrence: variable $v$ and filler $f$ co-occur within the reservoir's memory window. If the reservoir state at time $t + \Delta$ retains information about both $v(t)$ and $f(t + \Delta)$, the readout can extract the association $v \leftrightarrow f$ from the state at $t + \Delta$.

This works when $\Delta$ is small relative to the reservoir's effective memory $\tau_{\text{mem}} = 1/(\alpha(1-\rho))$. For binding events separated by longer intervals, the reservoir's fading memory attenuates the earlier event's contribution, and the binding fails.

This temporal binding is implicit and not robust to interference: if another pair $(v', f')$ co-occurs within the same memory window, the reservoir state contains a mixture of both bindings, and the readout may confuse them.

## Tensor Product Representations

Smolensky [1990] proposed tensor product representations (TPR) as a distributed representation of variable-filler bindings. The binding of variable $v \in \mathbb{R}^{d_v}$ to filler $f \in \mathbb{R}^{d_f}$ is represented by the outer product:

$$\mathbf{B}(v, f) = \mathbf{v} \otimes \mathbf{f} \in \mathbb{R}^{d_v \times d_f},$$

vectorized to $\mathbf{b} = \text{vec}(\mathbf{v} \otimes \mathbf{f}) \in \mathbb{R}^{d_v d_f}$.

Multiple bindings are superposed: the representation of $\{v_1 \leftrightarrow f_1, v_2 \leftrightarrow f_2\}$ is:

$$\mathbf{B}_{\text{total}} = \mathbf{v}_1 \otimes \mathbf{f}_1 + \mathbf{v}_2 \otimes \mathbf{f}_2.$$

Retrieval: given $\mathbf{v}_1$, retrieve $f_1$ by computing $\mathbf{v}_1^{\dagger} \mathbf{B}_{\text{total}} \approx \mathbf{f}_1$ (where $\mathbf{v}_1^\dagger$ is the pseudo-inverse of $\mathbf{v}_1$), provided $\mathbf{v}_1 \perp \mathbf{v}_2$ [Smolensky 1990].

## Holographic Reduced Representations

Plate [1995] proposed holographic reduced representations (HRR), which replace the outer product with circular convolution (a.k.a. vector binding):

$$\mathbf{v} \circledast \mathbf{f} = \mathcal{F}^{-1}(\mathcal{F}(\mathbf{v}) \odot \mathcal{F}(\mathbf{f})),$$

where $\mathcal{F}$ is the DFT and $\odot$ is elementwise multiplication. This produces a $d$-dimensional binding vector (same dimension as $\mathbf{v}$ and $\mathbf{f}$), unlike the outer product which has dimension $d_v d_f$. Multiple bindings are superposed:

$$\mathbf{S} = \mathbf{v}_1 \circledast \mathbf{f}_1 + \mathbf{v}_2 \circledast \mathbf{f}_2 + \cdots$$

Retrieval: $\mathbf{v}_1^{-1} \circledast \mathbf{S} \approx \mathbf{f}_1$ (exact when $\mathbf{v}_i$ are random unit vectors, with noise $O(K/d)$ for $K$ superposed bindings) [Plate 1995].

## ESN with HRR Inputs

ESNs can process HRR-encoded inputs directly, since HRRs are $d$-dimensional real vectors. The reservoir processes the sequence of HRR vectors $\{\mathbf{s}_t\}$, and the readout can extract bound pairs from the reservoir state.

The decoding is performed by the linear readout: train $\mathbf{w}_{\text{unbind}}^{\text{out}}$ to decode $f_1$ from the reservoir state given input $\mathbf{s}_t = \mathbf{v}_1 \circledast \mathbf{f}_1 + \mathbf{v}_2 \circledast \mathbf{f}_2$. The reservoir's nonlinear dynamics can in principle amplify the signal from the relevant binding by attending to the temporal context (if $\mathbf{v}_1$ was presented at $t-k$ for known $k$).

**Capacity limitation:** The HRR representation supports approximately $K \leq d/10$ reliable bindings before interference becomes prohibitive. For $d = 100$, approximately 10 simultaneous bindings are reliable [Plate 1995]. This is a fundamental capacity limitation of vector symbolic architectures.

## Limitations vs. Symbolic Binding

The capacity, noise robustness, and systematicity of reservoir-based and vector-symbolic binding are fundamentally limited compared to classical symbolic binding. Symbolic data structures can hold arbitrary numbers of bindings with zero interference; reservoir-based and HRR approaches have capacity $O(d)$ and noise that grows with capacity. For tasks requiring hundreds or thousands of simultaneous bindings (database operations, program execution), symbolic approaches remain indispensable [Marcus 2019].

---

## References

- Smolensky, P. (1990). Tensor product variable binding and the representation of symbolic structures in connectionist systems. *Artificial Intelligence*, 46(1–2), 159–216.
- Plate, T. A. (1995). Holographic reduced representations. *IEEE Transactions on Neural Networks*, 6(3), 623–641.
- Gayler, R. W. (2003). Vector symbolic architectures answer Jackendoff's challenges for cognitive neuroscience. In *ICCS/ASCS International Conference on Cognitive Science*, 133–138.
- Marcus, G. (2019). The next decade in AI: Four steps towards robust artificial intelligence. *arXiv preprint*, arXiv:2002.06177.
