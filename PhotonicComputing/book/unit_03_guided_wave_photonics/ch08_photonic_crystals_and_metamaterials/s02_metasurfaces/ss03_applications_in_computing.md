# 8.2.3 Metasurfaces for Optical Computing: Diffractive Neural Networks

## The D²NN Architecture

In 2018, Lin et al. at UCLA published a paper in *Science* that attracted enormous attention in both the photonics and machine learning communities: "All-optical machine learning using diffractive deep neural networks" [1]. The core idea is simple and elegant.

A diffractive deep neural network (D²NN) consists of multiple layers of diffractive metasurfaces separated by free-space propagation distances. Each layer is a passive optical element that applies a complex-valued transmission function $T_\ell(x, y)$ to the incoming wavefront. Between layers, light propagates freely, and diffraction mixes the spatial components. The final layer is followed by a detector array.

The computational analogy is as follows:
- Each point on a metasurface layer is a "neuron"
- The diffraction between layers is the "synaptic connection"
- The complex transmission $T_\ell(x,y)$ encodes the "weights"

The network is trained by optimizing the transmission functions $T_\ell$ to perform a desired classification or transformation task, using backpropagation through a differentiable model of the optical propagation (essentially, repeated applications of the angular spectrum method from Chapter 2).

For a 5-layer D²NN with $N = 200 \times 200 = 40{,}000$ pixels per layer:
- Total trainable parameters: $5 \times 40{,}000 = 200{,}000$
- Total multiply-accumulate operations per inference: approximately $N^2 = 1.6 \times 10^9$ (the Fourier-domain structure means it's actually $O(N \log N)$ but the effective parameter count is $O(N^2)$)
- Inference speed: limited by the speed of light propagation (~1 ps between layers at 300-μm spacing)

The claimed advantage: a D²NN performs its inference in the time light takes to traverse the layers — picoseconds — and the computation is done by passive diffraction without any electronic signal processing.

## The Physics of a Single D²NN Layer

The transmission through a single metasurface layer $\ell$ followed by free-space propagation can be modeled as a linear transformation on the input field $E_{\text{in}}(x, y)$:

$$E_{\text{out}}(x, y) = \int\int h_\ell(x-x', y-y') \cdot T_\ell(x', y') E_{\text{in}}(x', y') \, dx' \, dy'$$

where $h_\ell(x, y)$ is the free-space diffraction kernel (Rayleigh-Sommerfeld propagator for propagation distance $z_\ell$):

$$h_\ell(x, y) = \frac{1}{i\lambda}\frac{z_\ell}{r^2}\left(\frac{1}{r} + \frac{ik}{r}\right)e^{ikr}, \quad r = \sqrt{x^2 + y^2 + z_\ell^2}$$

In the paraxial approximation, this simplifies to the Fresnel propagator, and in the far-field limit ($z_\ell \gg N^2\lambda/4$), to the Fourier transform (Fraunhofer diffraction).

The key physics: the combination of local phase modulation (at the metasurface) and free-space diffraction (between layers) implements a *non-local* linear transformation. The diffraction kernel $h_\ell$ spreads information from each point to many output points — this is the optical analog of the fully connected layer in a neural network.

## Experimental Demonstrations

**Lin et al. 2018** [1]: Numerical simulation + terahertz-wave experimental demonstration (easier fabrication at mm-wave scale). Classified handwritten digits (MNIST dataset) with 91.75% accuracy using 5 layers; physical 3D-printed layers at terahertz frequencies (λ ≈ 750 μm).

**Miscuglio et al. 2020** [2]: Silicon photonic D²NN on-chip at 1550 nm. Used etched silicon patterns as diffractive elements. Demonstrated 2-class classification with ~88% accuracy.

**Weng et al. 2020** [3]: Free-space optical D²NN at 632 nm (visible) for digit and letter classification. Five polymer layers fabricated by 3D printing. Demonstrated 93.4% accuracy on MNIST.

**Luo et al. 2019** [4]: D²NN for CIFAR-10 image classification, 10 classes, achieving ~81% accuracy with a 5-layer design.

## Critical Assessment: What D²NNs Can and Cannot Do

The D²NN concept has generated justified excitement and also some overclaiming. A clear-eyed assessment:

**What D²NNs genuinely offer:**
- **Energy efficiency for inference**: Once fabricated, a passive D²NN consumes zero electrical power for inference (only the illumination source and detector). For high-throughput, fixed-task classification (e.g., sorting microparticles by shape), this is a compelling advantage.
- **Extreme throughput**: At the speed of light, a D²NN can process ~10¹² images/second (limited by detector bandwidth, not the optics). No electronic neural network approaches this.
- **Parallelism across wavelengths**: A broadband D²NN can process many wavelength channels simultaneously, multiplying throughput by the number of wavelength channels.

**What D²NNs cannot do without significant additions:**
- **Non-linearity**: Passive diffraction is linear. A network of linear operations is equivalent to a single linear operation — it has no more representational power than a single linear layer. To match the expressive power of a deep neural network, D²NNs need optical nonlinearity or detection-and-reillumination (which reintroduces electronics and destroys the energy advantage).
- **Reconfigurability**: A fabricated D²NN is fixed. Reprogramming the weights requires fabricating a new device. Active metasurfaces (using PCMs or electro-optic effects to tune $T_\ell(x,y)$) are being developed but are not yet at practical scale or precision.
- **Precision**: The complex transmission function $T_\ell(x,y)$ must be fabricated with precise amplitude and phase. Lithographic errors translate to incorrect weights. State-of-art fabrication achieves ~5–7 bits of phase precision per pixel [5].

**The fundamental limitation**: A D²NN with $L$ layers of $N$ pixels each implements an $N \times N$ complex matrix (the composed transmission, after tracing through all layers). The training process finds the weights in this $N^2$-dimensional parameter space that minimize classification error. But a single freely-trained complex $N \times N$ matrix already has $N^2$ complex parameters — the same as the D²NN. The multi-layer structure doesn't add representational power beyond what a single optimized diffractive layer would have, unless nonlinearity is added between layers [6].

This does not negate the D²NN concept — it clarifies what it is. It is an efficient hardware implementation of a linear (or near-linear) optical transformation, not a deep nonlinear neural network in the usual sense.

## Reconfigurable Metasurfaces: The Next Step

The logical evolution of D²NNs is a **reconfigurable optical computing layer**: a metasurface where each meta-atom can be individually programmed to any desired transmission. This requires integrating a tunable element (PCM, liquid crystal, electro-optic material) at every pixel position.

Promising approaches:
- **PCM-based metasurfaces**: GST or GSST deposited above dielectric pillars; switching between amorphous and crystalline states changes the resonance of each pillar, altering its phase response. Demonstrated for small arrays (16 × 16 pixels) [7].
- **Liquid crystal metasurfaces**: Liquid crystal reorients under applied voltage, changing the local birefringence. High-efficiency spatial light modulators (SLMs) already operate on this principle at pixel pitches of 3–10 μm. Metasurface-enhanced SLMs with sub-micron pixel pitch are an active research area.
- **Electro-optic metasurfaces**: ITO (indium tin oxide) has a large electro-optic coefficient near its epsilon-near-zero (ENZ) frequency (~1300 nm). ITO gate electrodes can shift the resonance of nearby plasmonic or dielectric antennas by ~10–20% — enough for a few bits of phase control per pixel.

A reconfigurable metasurface with $N = 1000 \times 1000$ pixels and 6-bit phase control would have $10^6$ tunable pixels × 64 phase states = the equivalent of a 64-Mpixel complex weight matrix. For inference tasks, this computes at the speed of light; for training, the weight update rate is limited by the reconfiguration time of the tunable elements.

---

## References

[1] Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008. [The foundational D²NN paper; terahertz wave experiment, digit classification, backpropagation training.]

[2] Miscuglio, M., Mehrabian, A., Hu, Z., Azzam, S.I., George, J., Kildishev, A.V., Pelton, M., & Sorger, V.J. (2020). "All-optical nonlinear activation function for photonic neural networks." *Optical Materials Express*, 8(12), 3851–3863. [Silicon photonic D²NN on chip at 1550 nm.]

[3] Weng, J., Ding, Y., Hu, C., Zhu, X., Liang, B., Yang, J., & Cheng, J. (2020). "Meta-neural-network for real-time and passive deep-learning-based object recognition." *Nature Communications*, 11(1), 6309. [Free-space visible-wavelength D²NN for handwritten digit classification; 93.4% accuracy.]

[4] Luo, Y., Mengu, D., Yardimci, N.T., Rivenson, Y., Veli, M., Jarrahi, M., & Ozcan, A. (2019). "Design of task-specific optical systems using broadband diffractive neural networks." *Light: Science & Applications*, 8(1), 112. [D²NN for CIFAR-10 image classification; 10-class, ~81% accuracy.]

[5] Colburn, S., Chu, Y., Shilzerman, E., & Majumdar, A. (2019). "Optical frontend for a convolutional neural network." *Applied Optics*, 58(12), 3179–3186. [Fabrication precision analysis for metasurface D²NNs; phase error impact on accuracy.]

[6] Hughes, T., Williamson, I.A.D., Minkov, M., & Fan, S. (2019). "Wave physics as an analog recurrent neural network." *Science Advances*, 5(12), eaay6946. [Analysis of the representational power of optical linear networks; shows that without nonlinearity, multi-layer optical networks are no more powerful than single-layer.]

[7] de Galarreta, C.R., Alexeev, A.M., Au, Y.-Y., Lopez-Garcia, M., Klemm, M., Bhaskaran, H., Bertolotti, J., & Wright, C.D. (2020). "Nonvolatile reconfigurable phase-change metadevices for beam steering and self-amplitude modulation." *Optica*, 7(5), 476–484. [PCM-based reconfigurable metasurface.]
