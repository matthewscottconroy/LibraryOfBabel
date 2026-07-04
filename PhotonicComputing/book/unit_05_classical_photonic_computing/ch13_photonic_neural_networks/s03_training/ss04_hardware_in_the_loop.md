# Subsection 13.3.4: Hardware-in-the-Loop Training

## Orientation

Hardware-in-the-loop training resolves the epistemology problem of Section 13.3 by refusing to model the forward pass at all: the *physical chip itself* computes each forward evaluation during training, so its true, unmodeled response — every static offset, every parasitic coupling — enters the loss directly and is optimized away without ever being characterized. The gradient still has to come from somewhere, and here it comes from an approximate, differentiable *digital model* of the hardware rather than from the optics. This forward-physical / backward-digital split is the essence of **physics-aware training (PAT)**, and it is the pragmatic frontier of the field: it absorbs all static forward error while demanding neither a faithful simulator (Subsection 13.3.1) nor intrusive optical gradient-readout hardware (Subsection 13.3.2).

---

## 13.3.4.1 The Forward-Physical, Backward-Digital Split

Place the three training strategies of this section side by side and the design space is clear:

| Method | Forward pass | Backward pass | Absorbs static forward error? |
|---|---|---|---|
| Offline (13.3.1) | digital twin | digital twin | No — this is the sim-to-real gap |
| In-situ backprop (13.3.2) | physical | physical (optical adjoint) | Yes |
| Hardware-in-the-loop / PAT (13.3.4) | **physical** | digital model $f_{\text{model}}$ | Yes (forward); gradient approximate |

In PAT the parameters $\boldsymbol{\theta}$ are updated by

$$\boldsymbol{\theta} \leftarrow \boldsymbol{\theta} - \eta\,\left(\frac{\partial f_{\text{model}}}{\partial \boldsymbol{\theta}}\right)^{\!\top}\!\frac{\partial \mathcal{L}}{\partial \mathbf{y}}\Bigg|_{\mathbf{y}\,=\,f_{\text{hw}}(\mathbf{x};\,\boldsymbol{\theta})},$$

where the output $\mathbf{y}$ and hence the loss are read from the **physical** system $f_{\text{hw}}$, while the Jacobian $\partial f_{\text{model}}/\partial\boldsymbol{\theta}$ comes from a differentiable digital twin. The subtlety that makes this work: because the forward values are physically real, the loss and all intermediate activations already contain the hardware's static errors — the network is optimizing its *true* output. The backward Jacobian need only be approximate. Gradient descent tolerates a surprisingly inexact gradient provided it correlates positively with the true one (points, on average, downhill), so a digital twin that is wrong in absolute terms but right in its *derivatives* suffices to train the real device. This is why PAT succeeds where pure offline training fails on the identical hardware and identical imperfect model.

## 13.3.4.2 Physics-Aware Training and Its Reach

Wright et al. (*Nature*, 2022) formalized this as physics-aware training and demonstrated it across strikingly diverse physical substrates — a driven mechanical oscillator, an analog electronic circuit, and a nonlinear optical (second-harmonic-generation) system — each trained to perform vowel and image classification with the forward pass executed on the physical apparatus and the backward pass on a differentiable digital model of it. The generality is the point: PAT does not require the physics to be linear, reciprocal, or even well understood, only that a differentiable approximate model exists. Where no analytic model is available, the twin can itself be *learned* — a neural network trained to imitate the input-output map of the hardware, then differentiated — which extends the method to black-box systems.

The optical special case predates and motivates the general result. Spall, Guo, and Lvovsky (2022) trained an optical network with a **hybrid** scheme — optical forward multiplication, digital backward pass — the direct ancestor of the forward-physical/backward-digital split, and the bridge from the in-situ methods of Subsection 13.3.2 to full physics-aware training. The contrast with Hughes et al. (2018) is instructive: in-situ backpropagation obtains an *exact* gradient by physical backward propagation but demands taps, monitor detectors, and coherent error injection at every phase shifter; PAT obtains an *approximate* gradient from a cheap digital model but needs nothing on the chip beyond the forward inference path the deployed system already has. One buys gradient exactness with hardware complexity; the other buys hardware simplicity with gradient approximation.

## 13.3.4.3 Worked Example: The Wall-Clock Cost of a Real Epoch

The price of putting hardware in the loop is measured in weight-reprogramming time. Consider a mesh with $\sim\!2000$ thermo-optic phase shifters (a $64\times64$-class Clements layer) trained on MNIST (60,000 examples per epoch). Thermo-optic heaters settle in $\sim\!100\,\mu\text{s}$; driven in parallel (one DAC channel per heater), a full weight update takes one settling time, $t_{\text{upd}} \approx 100\,\mu\text{s}$, independent of shifter count.

**Forward measurement.** The optical forward pass runs at GHz symbol rates; realistically each example is I/O-limited to load its input vector, propagate, integrate to the photon budget, and digitize — take $\sim\!1\,\mu\text{s}$/example. Per epoch:

$$t_{\text{fwd}} \approx 60{,}000 \times 1\,\mu\text{s} = 60\ \text{ms}.$$

**Weight updates.** One update per gradient step; steps per epoch depend on batch size $B$.

- *Pure SGD ($B = 1$):* $60{,}000$ updates $\times\,100\,\mu\text{s} = 6\ \text{s}$ per epoch — a $100\times$ overhead over the 60 ms of actual measurement. The wall clock is **entirely weight-reprogramming**; the optics sit idle waiting for heaters to settle.
- *Mini-batch ($B = 100$):* $600$ updates $\times\,100\,\mu\text{s} = 60\ \text{ms}$, now comparable to the measurement time; total $\approx 0.12\ \text{s}$ per epoch.

The lesson is architectural: mini-batching is not merely a variance-reduction convenience here, it is what **amortizes** the slow $\mu$s-scale thermo-optic update across a batch of fast forward passes. At $B = 100$ and $\sim\!40$ epochs to converge, on-hardware training completes in single-digit seconds; run as SGD it would take minutes, all of it heater-settling. The binding resource in hardware-in-the-loop training is therefore never the physics — which is fast — but the electro-optic weight-update bandwidth, exactly as in the in-situ methods of Subsection 13.3.2. Reducing $t_{\text{upd}}$ (faster phase-change or MEMS or carrier-based tuners in place of thermo-optic heaters) buys training speed one-for-one.

## 13.3.4.4 Where It Sits on the Ladder

Hardware-in-the-loop training is the top rung of the escalation ladder of Section 13.3. It gives up the manufacturing economy of a single model transferred to many chips — each chip trains against *itself* — in exchange for immunity to every static imperfection that offline training cannot see and every hardware requirement that in-situ backpropagation imposes. For chips whose forward pass is fast and whose imperfections are dominated by fixed, hard-to-model error, it is the method that most reliably reaches the hardware's true accuracy ceiling, and it is the approach that scales to the large diffractive and free-space systems where no per-element gradient access exists at all.

---

## References

[1] Wright, L.G., Onodera, T., Stein, M.M., Wang, T., Schachter, D.T., Hu, Z., & McMahon, P.L. (2022). "Deep physical neural networks trained with backpropagation." *Nature*, 601, 549–555. [The physics-aware-training paper: physical forward pass, differentiable-digital backward pass, demonstrated on optical and other substrates — the central reference of this subsection.]

[2] Spall, J., Guo, X., & Lvovsky, A.I. (2022). "Hybrid training of optical neural networks." *Optica*, 9(7), 803–811. [Optical forward pass with digital backward pass; the optical prototype of the forward-physical/backward-digital split.]

[3] Pai, S., et al. (2023). "Experimentally realized in situ backpropagation for deep learning in photonic neural networks." *Science*, 380(6643), 398–404. [The exact-gradient alternative against which PAT's approximate-gradient economy is contrasted in Subsection 13.3.4.2.]

[4] Hughes, T.W., Minkov, M., Shi, Y., & Fan, S. (2018). "Training of photonic neural networks through in situ backpropagation and gradient measurement." *Optica*, 5(7), 864–871. [The adjoint in-situ method whose hardware cost motivates the digital-backward compromise of hardware-in-the-loop training.]
