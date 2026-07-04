# Subsection 13.1.1: The Feedforward Network as Matrix Operations

## Orientation

Strip away the biological metaphors and a deep neural network is a composition of very simple functions: affine maps alternating with fixed pointwise nonlinearities. This subsection establishes that picture precisely, counts where the arithmetic lives, and marks the boundary — which will organize the entire chapter — between what a photonic matrix engine computes and what it leaves to electronics.

---

## 13.1.1.1 The Layer Equation

A feedforward network with $L$ layers maps an input vector $\mathbf{a}^{(0)} = \mathbf{x} \in \mathbb{R}^{N_0}$ to an output $\mathbf{a}^{(L)}$ through the recursion

$$\mathbf{z}^{(l)} = W^{(l)} \mathbf{a}^{(l-1)} + \mathbf{b}^{(l)}, \qquad \mathbf{a}^{(l)} = f\!\left(\mathbf{z}^{(l)}\right), \qquad l = 1, \dots, L$$

where $W^{(l)} \in \mathbb{R}^{N_l \times N_{l-1}}$ is the layer's weight matrix, $\mathbf{b}^{(l)}$ its bias vector, $\mathbf{z}^{(l)}$ the *pre-activation*, and $f$ a scalar nonlinearity applied elementwise. The standard choices of $f$ are the rectified linear unit $\text{ReLU}(z) = \max(0, z)$, the sigmoid $\sigma(z) = 1/(1+e^{-z})$, $\tanh$, and their smooth modern variants (GELU, SiLU). For a classifier, the final layer typically feeds a **softmax**, $p_i = e^{z_i}/\sum_j e^{z_j}$, converting scores into probabilities.

The nonlinearity is not decoration; it is the entire source of the model's power. If $f$ were the identity, the composition would collapse: $W^{(L)} \cdots W^{(2)} W^{(1)} = W_{\text{eff}}$, a single matrix, and depth would buy nothing. (Subsection 13.2.1 makes this the starting point for the optical-nonlinearity problem, and Chapter 14 shows a system — the diffractive network — that lives, instructively, right at the edge of this theorem.) With nonlinearities, by contrast, a network of sufficient width is a universal approximator: it can represent any continuous function on a compact set to any desired accuracy.

**The photonic mapping.** In the architectures of Chapter 12, $W^{(l)}$ is programmed into an MZI mesh (via SVD) or a microring weight bank; the vector $\mathbf{a}^{(l-1)}$ arrives as optical amplitudes or powers; and the product $W^{(l)}\mathbf{a}^{(l-1)}$ emerges in one optical transit. The bias addition and the function $f$ are performed in the electronic domain (or, in ambitious designs, by an optical nonlinearity — Section 13.2). One layer = one optical linear stage + one electronic (or electro-optic) pointwise stage.

---

## 13.1.1.2 Counting Operations: Where the Arithmetic Lives

For layer $l$, the matrix-vector product costs $N_l N_{l-1}$ multiply-accumulate (MAC) operations; the bias and activation cost $N_l$ additions and $N_l$ function evaluations. The ratio of linear to pointwise work is $\approx N_{l-1}$ — a factor of hundreds to tens of thousands in real networks.

**Worked example: an MNIST MLP.** Architecture 784→256→128→10:

| Layer | MACs | Activations |
|---|---|---|
| 1 | $784 \times 256 = 200{,}704$ | 256 |
| 2 | $256 \times 128 = 32{,}768$ | 128 |
| 3 | $128 \times 10 = 1{,}280$ | 10 |
| **Total** | **234,752** | **394** |

99.83% of the arithmetic is MACs. This ratio only grows with model scale: a GPT-3-class transformer layer with model dimension $d = 12{,}288$ performs $\sim\!12d^2 \approx 1.8 \times 10^9$ MACs per token against $\sim\!10d \approx 10^5$ pointwise operations. The design conclusion is immediate and central: **an accelerator that speeds up only the linear algebra addresses essentially all of the arithmetic** — the pointwise remainder is too small to matter energetically, *provided* crossing the optical-electronic boundary to compute it does not itself become the dominant cost. That proviso is the subject of Section 13.2.

**Convolutions are matrix multiplications too.** A convolutional layer correlating $C_{\text{in}}$ input channels with $C_{\text{out}}$ kernels of size $k \times k$ can be rewritten (the *im2col* construction) as a matrix product between a $C_{\text{out}} \times (k^2 C_{\text{in}})$ weight matrix and a matrix whose columns are the unrolled input patches. GPUs execute convolutions this way; photonic processors do the same — the Feldmann tensor core and the Xu accelerator of Section 12.4.3 both ran convolutions as (wavelength- or time-parallel) matrix products. Alternatively, free-space optics computes convolutions natively via the 4f Fourier architecture of Chapter 11 — one of the few operations with *two* natural optical embeddings.

---

## 13.1.1.3 Precision: What Inference Actually Requires

Digital training uses 16–32-bit floating point, and analog photonics cannot approach that. The saving fact — established empirically across the industry — is that *inference* does not need it. Post-training quantization to INT8 typically costs well under 1% accuracy on vision and language models, and 4-bit and even lower-precision inference is routine with quantization-aware training. The deep reason is the same graceful degradation encountered in Section 12.3.2: classification and next-token prediction are decisions among alternatives, with margins; small perturbations of pre-activations rarely flip the argmax. Networks are also *trainable into* robustness — if the perturbation statistics are present during training, the optimizer finds weights whose decisions are insensitive to them (Section 13.3.3).

This aligns the requirement (6–8 effective bits for inference) with the capability of photonic hardware (4–9 ENOB, Chapter 12) — closely enough to be workable, tightly enough that every dB of the optical error budget matters. Training, by contrast, requires accumulating millions of small gradient updates, which demands both higher precision and enormous weight-update bandwidth; this is the quantitative root of the field's concentration on inference (Subsections 13.1.3 and 13.3 return to it).

---

## 13.1.1.4 Batching, Throughput, and the Shape of the Workload

A deployed accelerator rarely processes one vector at a time. Batching $B$ inputs turns matrix-vector products into matrix-matrix products $Z = WX$ ($X \in \mathbb{R}^{N \times B}$), amortizing weight-loading over $B$ columns. Digital accelerators *need* large $B$ to reach peak efficiency because fetching $W$ from DRAM is their dominant cost (Subsection 13.1.3). A weight-stationary photonic processor holds $W$ in phase shifters or ring detunings and streams input columns through at the modulator rate — so its efficiency is nearly independent of batch size. This is why **batch-1, latency-critical inference** (interactive agents, control loops, in-line signal processing) is the workload where photonic accelerators are most structurally advantaged, a theme that will recur in every application discussion of this unit.

---

## References

[1] LeCun, Y., Bengio, Y., & Hinton, G. (2015). "Deep learning." *Nature*, 521, 436–444. [The standard compact survey of the feedforward framework, activations, and training.]

[2] Goodfellow, I., Bengio, Y., & Courville, A. (2016). *Deep Learning*. MIT Press. [The reference text for everything in this section; Chapters 6–8 cover feedforward networks and optimization.]

[3] Jouppi, N.P., et al. (2017). "In-datacenter performance analysis of a tensor processing unit." *Proceedings of the 44th International Symposium on Computer Architecture (ISCA)*. [The TPU paper: documents empirically that datacenter inference is dominated by matrix multiplication and tolerates 8-bit precision — the two facts on which photonic acceleration bets.]

[4] Shen, Y., et al. (2017). "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11, 441–446. [Maps the layer equation onto the mesh-plus-nonlinearity split assumed throughout this section.]
