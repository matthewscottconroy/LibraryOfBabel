# Subsection 13.5.1: Self-Attention as Matrix Operations

## Orientation

Attention is, arithmetically, a stack of matrix multiplications — which is why a photonics reader should care, and also why the fit is imperfect. This subsection formalizes scaled dot-product attention, sorts its operations into the two workload classes that decide hardware suitability (static-weight versus dynamic-operand), counts the MACs, and locates the quadratic-in-sequence-length bottleneck. The conclusion sets up Subsection 13.5.2: part of attention maps onto Chapter 12's weight-stationary processors unchanged, and part of it — the defining part — does not.

---

## 13.5.1.1 Scaled Dot-Product Attention

Let the input be a sequence of $L$ tokens, each a $d$-dimensional embedding, stacked as $X \in \mathbb{R}^{L \times d}$. A single attention head first computes three linear **projections**

$$Q = X W_Q, \qquad K = X W_K, \qquad V = X W_V,$$

with learned matrices $W_Q, W_K, W_V \in \mathbb{R}^{d \times d_k}$. The rows of $Q$, $K$, $V$ are the queries, keys, and values. The head then forms the **attention scores** and their row-normalized weights,

$$S = \frac{Q K^{\top}}{\sqrt{d_k}} \in \mathbb{R}^{L \times L}, \qquad A = \operatorname{softmax}(S),$$

where softmax is applied along each row so that $\sum_j A_{ij} = 1$, and produces the **output**

$$Y = A V \in \mathbb{R}^{L \times d_k}.$$

The scaling by $1/\sqrt{d_k}$ keeps the dot products $O(1)$ in variance so the softmax does not saturate. Interpretively, $A_{ij}$ is a *data-dependent* mixing weight: token $i$ pulls in a convex combination of all value vectors, with weights set by query–key similarity. Unlike a convolution or a fully connected layer, the mixing matrix $A$ is not stored — it is computed afresh from the input on every forward pass.

**Multi-head attention.** Real transformers run $h$ heads in parallel, each of dimension $d_k = d/h$, concatenate the $h$ outputs back to width $d$, and apply an output projection $O = \operatorname{Concat}(Y_1,\dots,Y_h)\,W_O$ with $W_O \in \mathbb{R}^{d \times d}$. Splitting into heads does not change the total operation count; it partitions the same $d$ dimensions across $h$ independent score matrices.

---

## 13.5.1.2 Two Workload Classes

The operations above fall into two categories that behave completely differently on photonic hardware.

**(a) Static-weight projections.** $W_Q, W_K, W_V, W_O$ are trained parameters, fixed at inference time. They are exactly the weight matrices of Chapter 12: program them once into an MZI mesh (via SVD, Subsection 12.3.1) or a microring weight bank (Subsection 12.4.2) and stream token embeddings through. The weight-stationary premise holds perfectly — one loading amortized over an entire sequence, indeed over many sequences — and the mapping of Shen et al. (2017) applies without modification.

**(b) Dynamic activation-by-activation products.** $QK^{\top}$ and $AV$ multiply two quantities that are *both* computed at runtime: $Q$ and $K$ are activations, $A$ and $V$ are activations. There is no static operand to hold stationary. A weight-stationary photonic core forced to compute $QK^{\top}$ must load $K$ (or $Q$) as if it were a weight matrix — and then reload it for the next token or sequence, invoking the microsecond-scale weight-update wall (Subsections 12.5 and 13.1.2) on data that arrives at the modulator rate of tens of gigahertz. This mismatch, not the softmax, is the structural reason attention is awkward for photonics.

**The softmax is electronic.** Between the two products sits the row-wise softmax — exponentials plus a normalization — a genuine nonlinearity of the kind Section 13.2 showed is cheapest to evaluate after detection, in the electronic domain. Like the layer activation of Subsection 13.1.1, it is an $O(L^2)$ pointwise cost dwarfed by the $O(L^2 d)$ matrix work around it, so paying for it electronically is affordable; the design burden is the optical-electronic boundary crossing, not the arithmetic.

---

## 13.5.1.3 Operation Count and the Quadratic Bottleneck

Per token, the projection $XW_Q$ costs $d \cdot d_k$ MACs; summed over the $h$ heads, $Q$, $K$, and $V$ together cost $3 d^2$ MACs per token, and $W_O$ adds another $d^2$, for $4d^2$ per token of static-weight work — $O(L d^2)$ over the sequence. The attention products $QK^{\top}$ and $AV$ each cost $L^2 d$ MACs (an $L \times L$ score matrix, each entry a length-$d_k$ dot product, summed over heads), i.e. $2 L^2 d$ of dynamic work — $O(L^2 d)$, **quadratic in sequence length**. The projection-to-attention ratio is therefore $4d^2 / 2L^2 d = 2d/L$: attention overtakes the projections once $L > 2d$, and thereafter dominates and keeps growing.

**Example: one attention block at GPT-2-large scale.** Take $L = 1024$, $d = 768$, $h = 12$ (so $d_k = 64$).

- Projections $Q,K,V$: $3 L d^2 = 3 \cdot 1024 \cdot 768^2 \approx 1.81 \times 10^9$ MAC.
- Output projection $W_O$: $L d^2 = 1024 \cdot 768^2 \approx 6.04 \times 10^8$ MAC. Static total $\approx 2.42 \times 10^9$.
- Scores $QK^{\top}$: $L^2 d = 1024^2 \cdot 768 \approx 8.05 \times 10^8$ MAC.
- Values $AV$: $L^2 d \approx 8.05 \times 10^8$ MAC. Dynamic total $\approx 1.61 \times 10^9$.

The block is $\approx 4.0 \times 10^9$ MAC/token-batch, of which the *dynamic, activation-by-activation* fraction is $1.61/4.0 \approx 40\%$. Because that fraction scales as $L^2$ while the static part scales as $L$, doubling the context to $L = 2048$ (still below $2d$? no: $2d = 1536$, so we are already past the crossover) makes the dynamic products the majority: at $L = 2048$ they reach $\approx 3.2 \times 10^9$ versus $2.42 \times 10^9$ static. A photonic accelerator that speeds up only the weight-stationary projections leaves an increasing, sequence-length-quadratic remainder untouched — precisely the workload Subsection 13.5.2 must address.

---

## References

[1] Vaswani, A., Shazeer, N., Parmar, N., Uszkoreit, J., Jones, L., Gomez, A.N., Kaiser, Ł., & Polosukhin, I. (2017). "Attention is all you need." *Advances in Neural Information Processing Systems (NeurIPS)* 30. [The paper that defined scaled dot-product and multi-head attention; the operation this subsection maps to hardware.]

[2] Anderson, M.G., Ma, S.-Y., Wang, T., Wright, L.G., & McMahon, P.L. (2024). "Optical transformers." *Transactions on Machine Learning Research (TMLR)*. arXiv:2302.10360. [Analyzes transformer inference as a photonic workload and quantifies the static/dynamic split explored here.]

[3] Shen, Y., et al. (2017). "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11, 441–446. [The weight-stationary mesh mapping that the static projections inherit directly.]
