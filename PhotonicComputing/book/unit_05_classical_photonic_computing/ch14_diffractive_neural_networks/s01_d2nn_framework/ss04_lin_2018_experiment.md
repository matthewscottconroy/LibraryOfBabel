# Subsection 14.1.4: The Original Lin et al. 2018 Experiment

## Orientation

The framework of the previous three subsections was introduced to the world in a single paper: Lin, Rivenson, Yardimci, Veli, Luo, Jarrahi, and Ozcan's 2018 report in *Science*, which trained a diffractive network in simulation, 3D-printed it, and classified handwritten digits with terahertz light. This subsection narrates that experiment — the setup, the results, and why it set the agenda for the rest of the chapter.

---

## 14.1.4.1 The Physical Setup

The network was a stack of *five* transmissive diffractive layers, each a $200\times200$ grid of phase-only neurons — about $0.2$ million learnable features in all. Illumination was continuous-wave terahertz radiation at $0.4$ THz ($\lambda = 0.75$ mm), a band chosen because its millimetre-scale wavelength makes neuron-scale (sub-millimetre) features 3D-printable with a standard printer. Each layer was a plate of dielectric with refractive index $n \approx 1.7$; the learned phase at each pixel was realized as a local thickness $\Delta(x,y)$ through $\phi = \frac{2\pi}{\lambda}(n-1)\Delta$, so a relief of order a millimetre (full $2\pi$ needing $\Delta_{2\pi} = \lambda/(n-1) \approx 1.07$ mm) encoded the weights. The layers were spaced by tens of wavelengths — a few centimetres — placing the propagation in the near-field regime of Subsection 14.1.2. The finished classifier was a passive object several centimetres on a side and of order 10–15 cm deep.

---

## 14.1.4.2 Training, Fabrication, Measurement

The design followed the recipe of Subsection 14.1.3 exactly: a differentiable Rayleigh–Sommerfeld model, error-backpropagation to optimize the $\sim2\times10^5$ phases, and a softmax over ten output detector regions for the digit classes. The trained height maps were then *printed* — frozen into plastic — and the physical stack was assembled and probed with a terahertz source and a detector scanning the output plane. This is the sim-to-real crossing that defines the field: the same phase profile that had been a tensor in a computer became the thickness of a plate on an optical bench.

---

## 14.1.4.3 Results

On MNIST handwritten digits, the five-layer phase-only network reached a *numerical* blind-test accuracy of **91.75%**, and the 3D-printed network's measured classifications agreed with the simulation to within a few percent — the first demonstration that a trained diffractive stack survives the trip from simulation into matter. A network trained on Fashion-MNIST (clothing images, a harder task) reached about **81.13%**. The authors also built an *imaging* D2NN — a diffractive stack trained to reproduce, rather than classify, an input amplitude at the output plane — showing the framework computes general linear transforms, not merely classifiers. In every case the readout was the same: the output plane carried ten detector regions (or, for imaging, a full grid), and the decision was taken as the brightest, $\hat y = \arg\max_k I_k$.

---

## 14.1.4.4 Significance

The experiment established several things at once. It was the first physical realization of an all-optical *deep* network — depth built from stacked diffraction — and it demonstrated the two properties that make D2NNs attractive: inference at the speed of light, and zero electrical power in the inference path (the only power is the illumination). It also exposed the constraints the rest of the chapter takes up: the network is passive and, between layers, purely linear (the expressivity ceiling of Section 14.4); the detector measures non-negative intensity (the non-negativity problem of Subsection 14.4.3); and a printed plate cannot be edited, so accuracy hinges on fabrication fidelity and alignment (Subsection 14.4.2). The founding result and its limitations are two sides of the same passive stack.

---

## Worked Example: Inference Latency vs Training Time

**Inference latency.** The signal traverses roughly the stack depth — five layers with few-centimetre gaps, an optical path of order $\ell \approx 15$ cm. In free space,

$$\tau = \frac{\ell}{c} = \frac{0.15\,\text{m}}{3\times10^8\,\text{m/s}} = 5\times10^{-10}\,\text{s} = 0.5\,\text{ns}.$$

The classification is complete about half a nanosecond after the light enters — and, being passive, the stack draws no power to do it.

**Training time**, by contrast, is minutes to hours of GPU simulation (Subsection 14.1.3), performed once, offline.

**Throughput** is set not by the 0.5 ns transit but by how fast inputs can be presented and outputs read: with a fixed printed network the input is imaged in, so the limit is the source and detector electronics, not the optics.

The asymmetry is the whole value proposition — a large one-time (digital) training cost, amortized into a physical object that then infers at light speed for essentially free.

---

## References

[1] Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008. [The experiment narrated in this subsection: five 3D-printed terahertz layers, MNIST at 91.75%, Fashion-MNIST at 81.13%, and an imaging D2NN.]

[2] Mengu, D., Luo, Y., Rivenson, Y., & Ozcan, A. (2019). "Analysis of diffractive optical neural networks and their integration with electronic neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 3700114. [Follow-up analysis quantifying accuracy, depth, and the diffractive–electronic hybrids that push past the passive-linear ceiling.]

[3] Wetzstein, G., Ozcan, A., Gigan, S., Fan, S., Englund, D., Soljačić, M., Denz, C., Miller, D.A.B., & Psaltis, D. (2020). "Inference in artificial intelligence with deep optics and photonics." *Nature*, 588, 39–47. [Places the Lin et al. result in the broader landscape of light-speed, low-power optical inference.]
