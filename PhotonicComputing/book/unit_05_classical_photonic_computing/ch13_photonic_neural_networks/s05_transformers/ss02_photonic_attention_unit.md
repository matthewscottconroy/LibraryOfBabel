# Subsection 13.5.2: Photonic Dot-Product Accelerator for Attention

## Orientation

Subsection 13.5.1 split attention into a weight-stationary part (the projections) and a dynamic, activation-by-activation part (the products $QK^{\top}$ and $AV$). The design problem follows immediately: the projections drop onto Chapter 12 hardware unchanged, but the dynamic products need a multiplier that ingests *two* runtime operand streams and holds no static weights. This subsection lays out the architectural options, presents the energy-scaling argument that motivates the whole enterprise — why the optical advantage grows with model size — and quantifies the per-token energy at which photonics could plausibly win.

---

## 13.5.2.1 Mapping the Static Projections

Nothing new is required for $W_Q, W_K, W_V, W_O$. Each is a $d \times d$ matrix programmed once into an MZI mesh (SVD form, Subsection 12.3.1) or a WDM weight bank (Subsection 12.4.2); token embeddings stream through at the modulator rate; the product emerges in one optical transit ($\sim$10–100 ps) independent of $d$. Because the same weights serve every token in the sequence and every sequence in the batch, the microsecond weight-update cost is amortized to negligibility — the batch-1, weight-stationary regime where photonics is most advantaged (Subsection 13.1.4). This is the "easy" 60% of the block from the previous example.

---

## 13.5.2.2 The Dynamic-Operand Problem and Two Solutions

The hard part is $QK^{\top}$ and $AV$, where both operands are activations. Two families of solution exist.

**Option A — one operand as reconfigured weights.** Treat $K$ as a weight matrix, load it into a mesh or weight bank, and stream $Q$ through. This works arithmetically but fails economically: a *new* $K$ must be loaded for every token position (and every sequence), so the multiplier is reprogrammed at the token rate. With thermo-optic weights at 10–100 μs per update against nanosecond-scale optical transits, the accelerator spends essentially all of its time settling phase shifters. Only weight technologies far faster than thermo-optics — and none is yet mature at scale — could make the reconfigured-weight route competitive for the dynamic products.

**Option B — stream both operands (coherent multiplication).** The photoelectric-multiplication architecture of Hamerly et al. (2019) multiplies two *streams* rather than a stream against a stored matrix. Each operand modulates a field; the two fields interfere at a beam splitter; balanced homodyne detection yields the product $\propto \operatorname{Re}[E_1^{*} E_2]$, and the detector sums a length-$d_k$ dot product automatically. No static weight is held, so nothing must be reprogrammed between tokens — a natural fit for activation-by-activation work. The $L$ independent query–key dot products are parallelized across wavelength (WDM) or time, so the $O(L^2 d)$ attention core is spread over the optical parallelism rather than serialized through a weight-update bottleneck. This is the architecturally correct answer to the dynamic-operand problem, at the cost of two coherent operand paths and phase-stable detection.

---

## 13.5.2.3 The Energy-Scaling Argument

Why pursue any of this? Because of a scaling law made explicit in the "Optical transformers" analysis of Anderson et al. (2024). The optical energy of a matrix–vector multiply is dominated by the photons needed at the detector: shot-noise-limited precision requires a fixed mean photon number $\bar{n}$ *per detected output symbol* (Subsection 12.3.2), but each detection integrates a dot product over $N$ input terms, so the optical energy **per MAC** is $\bar{n}\,h\nu / N$ — it *falls* as the matrices grow. Digital hardware, by contrast, pays a roughly fixed energy per MAC regardless of matrix size. Hence the optical advantage widens with model dimension $d$: the larger the transformer, the larger the matrices, the lower the optical energy per MAC, and the greater the fraction of total inference energy that photonics can capture. Anderson et al. model real transformer inference under this accounting and conclude that at large scale the optical matrix-multiply energy can dominate the total, *provided* the system reaches a sufficiently low optical energy per MAC (approaching femtojoules with noise-tolerant, error-corrected operation).

**Caveats kept in view.** Softmax and layer normalization remain electronic (Subsection 13.2.2); every optical stage that feeds a nonlinearity pays an O/E/O crossing; and the whole scheme lives at 6–8 bit precision (Subsection 13.1.3), tolerable for inference but not beyond. The advantage is real but conditional — it is an advantage in the linear algebra, purchased against fixed electronic and conversion overheads that the scaling law does not shrink.

---

## 13.5.2.4 Worked Example: Energy per Token

Take an attention-plus-feedforward block of $\sim 2 \times 10^9$ MAC/token (the order of magnitude of Subsection 13.5.1's example, or of one GPT-3-class layer with $d \approx 12{,}288$, where $\sim 12 d^2 \approx 1.8 \times 10^9$).

*Optical.* At a system figure of $\sim 1$ fJ/MAC (large-$N$, shot-noise-limited, including conversion),

$$E_{\text{opt}} \approx 2 \times 10^{9}\ \text{MAC} \times 10^{-15}\ \text{J/MAC} = 2 \times 10^{-6}\ \text{J} = 2\ \mu\text{J/token}.$$

*Digital.* An A100-class GPU delivers $\sim$312 TFLOPS BF16 at $\sim$400 W, i.e. $400 / (312 \times 10^{12}) \approx 1.3$ pJ/FLOP, or $\approx 2.6$ pJ per MAC at peak utilization; a generous "delivered" figure including data movement at batch 1 is $\sim 1$ pJ/MAC. Then

$$E_{\text{GPU}} \approx 2 \times 10^{9}\ \text{MAC} \times 10^{-12}\ \text{J/MAC} = 2 \times 10^{-3}\ \text{J} = 2\ \text{mJ/token},$$

a $\sim 10^{3}\times$ gap. The gap is credible only if the 1 fJ/MAC optical figure is: the shot-noise floor is far below it. For 7 ENOB, $\bar{n} = 10^{(6.02\cdot 7 + 1.76)/10} \approx 2.5 \times 10^{4}$ photons per detected symbol, and at $h\nu(1550\,\text{nm}) = 1.28 \times 10^{-19}$ J that is $\sim 3.2$ fJ per *detection* — amortized over a length-$d$ dot product ($d = 768$) it is $\sim 4$ aJ per MAC of *optical* energy, consistent with the sub-attojoule regime Hamerly et al. reach at $N \sim 10^{6}$. The fJ/MAC *system* figure is therefore set not by the shot-noise floor but by O/E/O conversion, DAC/ADC, and laser wall-plug efficiency — exactly the overheads the scaling law leaves fixed. The crossover thus sits wherever those fixed overheads, divided across a token's MACs, fall below the digital per-MAC energy: it favors photonics precisely at large $d$ and large batch, and evaporates for small models where conversion dominates.

---

## References

[1] Anderson, M.G., Ma, S.-Y., Wang, T., Wright, L.G., & McMahon, P.L. (2024). "Optical transformers." *Transactions on Machine Learning Research (TMLR)*. arXiv:2302.10360. [The energy-scaling analysis of optical transformer inference central to this subsection.]

[2] Hamerly, R., Bernstein, L., Sludds, A., Soljačić, M., & Englund, D. (2019). "Large-scale optical neural networks based on photoelectric multiplication." *Physical Review X*, 9, 021032. [Coherent stream-versus-stream multiplication — the natural accelerator for activation-by-activation products — and the sub-attojoule optical-energy floor at large $N$.]

[3] Nahmias, M.A., Ferreira de Lima, T., Tait, A.N., Peng, H.-T., Shastri, B.J., & Prucnal, P.R. (2020). "Photonic multiply-accumulate operations for neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 7701518. [The careful MAC-energy accounting, including conversion overheads, behind the fJ/MAC system figures.]

[4] Vaswani, A., et al. (2017). "Attention is all you need." *Advances in Neural Information Processing Systems (NeurIPS)* 30. [The workload whose energy this subsection estimates.]
