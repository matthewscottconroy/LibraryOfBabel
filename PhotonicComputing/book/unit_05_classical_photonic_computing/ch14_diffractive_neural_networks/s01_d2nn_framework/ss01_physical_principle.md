# Subsection 14.1.1: Physical Principle

## Orientation

A diffractive neuron is not a circuit element but a patch of transparent material; its "connections" are not wires but the diffraction of light across the gap to the next layer. This subsection establishes that picture — the neuron as a phase/amplitude pixel, the Huygens–Fresnel fan-out that wires the neurons all-to-all, and the passivity that makes the whole stack one linear optical element — and estimates the geometry (pixel pitch, aperture, inter-layer spacing) at which it works.

---

## 14.1.1.1 The Neuron as a Pixel on a Thin Layer

A diffractive layer is a thin transmissive (or reflective) plate divided into a grid of pixels. Each pixel — one neuron — multiplies the incident complex field by a local complex transmission coefficient

$$t^l(x,y) = a^l(x,y)\,e^{i\phi^l(x,y)}, \qquad 0 \le a^l(x,y) \le 1.$$

The plate has no gain: $a^l \le 1$ always, because a passive medium can only attenuate or delay light, never amplify it. In the most common design the pixel is loss-free and modulates *phase only*, $|t^l| = 1$, so $t^l = e^{i\phi^l}$. That phase comes from the physical thickness of the plate: a pixel of height $\Delta(x,y)$ in a dielectric of refractive index $n$ retards the wavefront by

$$\phi(x,y) = \frac{2\pi}{\lambda}(n-1)\,\Delta(x,y).$$

Training the network *is* choosing the height map $\Delta(x,y)$ of every layer. A full $2\pi$ of phase control requires a height range $\Delta_{2\pi} = \lambda/(n-1)$ — a genuinely manufacturable relief, as the worked example shows.

---

## 14.1.1.2 Diffraction as All-to-All Wiring

By the Huygens–Fresnel principle, every illuminated pixel acts as a secondary source of a spherical wavelet. The field it launches spreads as it crosses the air gap $d$ to the next layer, so a single neuron does not address one downstream neuron — it illuminates *all* of them, each with an amplitude and phase set by the propagation geometry. The field arriving at a neuron of layer $l{+}1$ is therefore the coherent superposition of wavelets from every neuron of layer $l$. This is the defining structural fact of a D2NN: the layer-to-layer connectivity is *all-to-all*, and it is realized by nothing but free-space propagation.

Contrast the electronic case. A fully-connected layer with $N$ inputs and $N$ outputs needs $N^2$ physical connections; in silicon this fan-out dominates area and energy, and it is why dense layers are expensive. In a D2NN the fan-out is free — diffraction connects everything to everything automatically — and the cost reappears elsewhere: in the space–bandwidth product of the optics, and in the fact that the connection weights are not independently programmable but are dictated by a single propagation kernel (Subsection 14.1.2).

Because the light is *coherent*, these superpositions interfere. A diffractive layer computes with complex amplitudes, not intensities, all the way to the final detector; this is what lets a stack of passive plates implement a nontrivial complex-valued linear transform rather than a mere blurring.

---

## 14.1.1.3 Why "Deep": Stacking Layers Across Air Gaps

Depth is built by stacking: layer, air gap $d$, layer, air gap, and so on. Each plate applies its diagonal modulation $t^l$; each gap applies a propagation operator. A five-layer network is five modulations interleaved with the propagations between them (plus the input-to-first and last-to-detector gaps). Crucially, in the original passive design there is *no* nonlinearity between layers — every stage is linear — so the entire optical stack is one large linear operator (Subsection 14.1.2 makes this explicit, and the collapse theorem of Subsection 13.1.1 explains why the *only* thing keeping the network from reducing to a single equivalent plate is the intensity measurement at the output). "Deep," here, buys richer structured linear maps and more trainable parameters, not the layered nonlinear feature hierarchy of an electronic deep network.

---

## 14.1.1.4 Worked Example: Terahertz D2NN Geometry

Consider a representative terahertz D2NN of the kind realized by Lin et al.

**Wavelength.** At $f = 0.4$ THz, $\lambda = c/f = (3\times10^8)/(0.4\times10^{12}) = 0.75$ mm.

**Pixel and aperture.** Take a pixel pitch $w \approx 0.4\lambda = 0.3$ mm. A layer of $200\times200$ neurons then spans an aperture $a = 200 \times 0.3\,\text{mm} = 60\,\text{mm} = 6$ cm on a side — a plate the size of a coaster, holding 40,000 neurons.

**Phase depth.** In a THz-printable dielectric with $n \approx 1.7$, full $2\pi$ phase control needs a height range $\Delta_{2\pi} = \lambda/(n-1) = 0.75/0.7 \approx 1.07$ mm. So a relief just over a millimetre deep, printed at 0.3 mm lateral resolution, encodes the entire weight set of a layer.

**Connectivity and spacing.** A neuron of width $w = 0.3$ mm is *subwavelength* ($w < \lambda$): the single-aperture diffraction condition $\sin\theta = \lambda/w$ gives $\lambda/w = 2.5$, which has no real solution — the pixel has no diffraction null and radiates across essentially the full forward hemisphere. Its light therefore floods the entire next layer even across a short gap. To make this quantitative, require the diffraction cone (paraxial half-angle $\theta \sim \lambda/w$) to span the full aperture over the gap $d$:

$$d \gtrsim \frac{a\,w}{\lambda} = \frac{(60\,\text{mm})(0.3\,\text{mm})}{0.75\,\text{mm}} = 24\,\text{mm}.$$

An inter-layer spacing of a few centimetres — Lin et al. used of order $d \sim 3$ cm $\approx 40\lambda$ — thus comfortably guarantees genuine all-to-all connectivity, and the true wide-angle diffraction of a subwavelength pixel achieves it at even shorter range (the paraxial estimate above is conservative here). Each neuron is wired to all 40,000 neurons of the next layer, and the wiring costs nothing.

---

## References

[1] Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008. [Introduces the diffractive-neuron picture and the Huygens-source model used throughout this subsection.]

[2] Goodman, J.W. (2017). *Introduction to Fourier Optics* (4th ed.). W.H. Freeman. [The Huygens–Fresnel principle, secondary-source superposition, and the diffraction-angle estimates invoked here; Chapters 3–4.]

[3] Kulce, O., Mengu, D., Rivenson, Y., & Ozcan, A. (2021). "All-optical information-processing capacity of diffractive surfaces." *Light: Science & Applications*, 10, 25. [Formalizes how a diffractive surface's neuron count and connectivity bound the transforms it can realize.]
