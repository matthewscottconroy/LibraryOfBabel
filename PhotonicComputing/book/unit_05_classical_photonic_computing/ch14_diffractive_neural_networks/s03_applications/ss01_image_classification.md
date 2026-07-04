# Subsection 14.3.1: Image Classification

## Orientation

Image classification is to diffractive networks what MNIST is to deep learning generally: the task everyone runs first, the number everyone quotes, and the arena where the architecture's ideas are tested. This subsection traces the benchmark results — accuracy versus depth, and the three refinements that push it upward — and confronts the honest comparison with a digital network of the same parameter count.

---

## 14.3.1.1 The Benchmarks and the Role of Depth

MNIST handwritten digits and the harder Fashion-MNIST are the field's standard tasks. The image is encoded into the input field (amplitude or phase), propagated through the trained diffractive stack, and classified by which of ten output detector regions collects the most intensity. The founding five-layer, phase-only network reached 91.75% on MNIST (Lin et al. 2018). Accuracy improves with depth — additional layers add trainable phase planes and further composed diffraction — but with diminishing returns, saturating after a handful of layers (Mengu et al. 2019); the gain from adding a fifth layer is far smaller than from adding a second.

## 14.3.1.2 What a Diffractive Layer Actually Computes

This point governs the comparisons that follow. Between two planes, free-space diffraction couples every input pixel to every output pixel — a dense, all-to-all linear operator (the Rayleigh–Sommerfeld kernel of Section 14.1). The trainable element, however, is only the per-pixel phase mask, a diagonal operator. A layer is thus (fixed dense diffraction) $\circ$ (trained diagonal phase), and a D2NN is a cascade of these. The connectivity resembles a fully-connected layer — not a convolution — but the free parameters number one phase per pixel per layer (an $N$-pixel layer has $N$ parameters, not $N^2$), which is what makes the optics parameter-efficient and also what limits it.

## 14.3.1.3 Three Refinements

Three refinements define the state of the art. **Class-specific differential detection** (Li et al. 2019) assigns each class two detectors, a positive and a negative, and takes their difference as the class score. Because a passive network can deliver only non-negative intensity, a single detector cannot represent a signed output; the detector pair restores an effective sign and measurably improves accuracy — a direct foreshadowing of the non-negativity problem and its differential-detection fix in Section 14.4. **Ensemble learning** (Rahman et al. 2021) trains many diffractive networks with differing designs and sums their outputs; the ensemble pushes MNIST into the $\sim 98\%$ range, at the cost of running several networks in parallel. **Hybrid diffractive-electronic** designs (Mengu et al. 2019) place a small trained electronic network after the optical front end, jointly optimized, so the optics performs the massively parallel linear projection and a lightweight digital stage cleans up the decision.

## 14.3.1.4 Worked Example: Parameter Budget and Detector Contrast

The five-layer network has $5\times200\times200 = 2\times10^5$ trainable phases. That budget invites a sobering comparison. A single-hidden-layer digital perceptron with 256 units, $784\!\to\!256\!\to\!10$, has $784\times256 + 256\times10 \approx 2.03\times10^5$ weights — essentially the same count — and reaches roughly 98% on MNIST. The D2NN, at matched parameters, reaches about 92%. The deficit is the price of the physical constraints: diagonal (phase-only) modulation instead of full dense weights, and a non-negative intensity readout instead of signed activations. The optics buys passivity, parallelism, and light-speed inference; it does not buy accuracy-per-parameter over digital.

The readout itself is an SNR question. The output plane is tiled into ten detector regions and the class is the arg-max of collected energy. If the transmitted power were spread uniformly, each region would gather $10\%$; a trained network instead concentrates a substantially larger fraction — say $25$–$30\%$ — into the correct region while suppressing the runner-up. The decision margin is the contrast between the winning region and its strongest competitor, and reliable classification requires that margin to stand clear of detector noise and the pooled background from the other nine regions. Deeper networks, differential detection, and ensembles all act, in the end, to widen this energy gap.

---

## References

[1] Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008. [The benchmark result: five-layer phase-only D2NN at 91.75% on MNIST, and the depth study this subsection builds on.]

[2] Li, J., Mengu, D., Luo, Y., Rivenson, Y., & Ozcan, A. (2019). "Class-specific differential detection in diffractive optical neural networks improves inference accuracy." *Advanced Photonics*, 1(4), 046001. [Differential detector pairs restore signed outputs against the non-negative readout, raising accuracy.]

[3] Rahman, M.S.S., Li, J., Mengu, D., Rivenson, Y., & Ozcan, A. (2021). "Ensemble learning of diffractive optical networks." *Light: Science & Applications*, 10, 14. [Ensembles of diffractive networks push MNIST toward the ~98% range.]

[4] Mengu, D., Luo, Y., Rivenson, Y., & Ozcan, A. (2019). "Analysis of diffractive optical neural networks and their integration with electronic neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 3700114. [The depth/diminishing-returns analysis and the hybrid diffractive-electronic architecture.]
