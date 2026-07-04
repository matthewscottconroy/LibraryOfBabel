# Subsection 14.4.2: Noise and Fabrication Sensitivity

## Orientation

A diffractive network is trained as software and deployed as hardware. The optimizer converges on a set of phase profiles $\{\phi^l\}$ inside a simulator; a fabricator then prints, etches, or writes those profiles into matter, and the matter is thereafter frozen. Every way in which the physical stack differs from the simulated one — a layer a few microns out of place, a feature a little too thick, an index a little off, a detector counting discrete photons — perturbs the trained map and costs accuracy. This subsection catalogues those error channels, shows that their severity scales with wavelength, and explains why the field's easiest demonstrations live at terahertz frequencies while its most capacious ones (visible, metasurface) are the hardest to build.

## 14.4.2.1 The Sim-to-Real Gap

The sim-to-real gap — borrowed from robotics, and central to Chapter 13's treatment of training — is the accuracy lost between the simulator and the bench. For a passive D2NN it is especially unforgiving: unlike a reconfigurable mesh, a printed diffractive layer cannot be retuned after the fact to absorb its own errors. The dominant contributors are (i) lateral and axial *misalignment* of the layers relative to one another and to the input and output planes; (ii) *fabrication error* in feature height — hence phase — and in refractive index; (iii) *illumination* imperfections, a non-ideal, non-uniform, or partially coherent input beam; and (iv) *detection* noise and finite dynamic range at the output regions. Because the network is a chain of interference operations (Subsection 14.4.1), these errors do not stay local: like the entangled phase errors of an MZI mesh (Chapter 12), a perturbation at one layer propagates and mixes into every downstream neuron.

## 14.4.2.2 Misalignment Resilience and Vaccination

Of these channels, inter-layer misalignment is usually the most punishing and the most instructive, because its tolerance scales directly with the wavelength. A lateral or axial displacement matters in units of $\lambda$ and of the pixel pitch: shift a layer by an appreciable fraction of a pixel and the carefully tuned interference at the next plane decoheres.

Mengu et al. (2020) turned this vulnerability into a design variable. Rather than demand tighter alignment, they trained the network with random lateral and axial layer displacements injected into the forward model, so the optimizer sought phase profiles whose performance is *flat* against displacement — a strategy the authors liken to vaccination: expose the network to the perturbation during training and it acquires immunity. Displacement-aware training measurably widens the alignment budget, converting an assembly problem into a software one, at the cost of a modest ceiling on best-case accuracy (a flat optimum is rarely the global one). This is the diffractive analogue of the noise-aware training of Section 13.3.

**Example (alignment budget: terahertz vs. visible).** Adopt a representative tolerance of $\lambda/10$ for lateral registration between layers. At 0.4 THz, $\lambda = 0.75$ mm, so

$$\frac{\lambda}{10} = 75\ \mu\text{m},$$

about one-quarter of the $\approx 0.3$ mm pixel — a shift that ordinary optomechanical mounts and 3D-printing registration (tens of microns) meet without effort. This is the quantitative reason Lin's terahertz network "just worked" on a bench. Now demand the same *relative* tolerance at $\lambda = 532$ nm:

$$\frac{\lambda}{10} = 53\ \text{nm}.$$

Fifty-three-nanometer registration across a centimeter-scale aperture requires interferometric, actively stabilized alignment — the tolerance drops by the wavelength ratio, $0.75\ \text{mm}/532\ \text{nm}\approx 1400\times$. The very wavelength reduction that multiplied the neuron budget a millionfold (Subsection 14.4.1) tightens every mechanical tolerance in proportion. Misalignment-resilient training relaxes the constant in front but not the wavelength scaling itself.

## 14.4.2.3 Fabrication Tolerance, Phase Quantization, and Detector Noise

The same $\lambda$-scaling governs feature fabrication. A layer imprints phase through height, $\phi = \frac{2\pi}{\lambda}(n-1)h$, so a full $2\pi$ modulation needs $h_{2\pi} = \lambda/(n-1)$. For terahertz plastic ($n\approx 1.7$), $h_{2\pi} = 0.75\ \text{mm}/0.7 \approx 1.07$ mm — a millimeter-scale relief that a 3D printer resolves to a small fraction of a wavelength. An $n$-bit phase mask needs $2^n$ height steps; at 8 bits (256 levels) the terahertz step is $1.07\ \text{mm}/256 \approx 4.2\ \mu$m, comfortably printable, so quantization costs almost nothing. Repeat at a visible wavelength with a polymer ($n-1\approx 0.5$): $h_{2\pi}\approx 1.06\ \mu$m and the 8-bit step is $\approx 4$ nm, demanding nanometer height control — the province of electron-beam lithography and nanofabricated metasurfaces (Section 14.2), not desktop printing. Coarse quantization (a handful of levels) costs accuracy at any wavelength; the point is that reaching the near-continuous 8-bit regime is trivial at terahertz and hard in the visible.

Finally, the output itself is noisy. The detectors that read the intensity regions are subject to shot noise and finite dynamic range (Chapter 12): the class scores are photon-count estimates, and when two classes deposit similar energy in adjacent regions, shot noise on those counts can flip the argmax. A network trained noiselessly and read out on a small photon budget loses accuracy exactly as a low-ENOB analog processor does; the design response is to allocate enough optical power per output region to separate the competing scores above the noise floor and — as in Section 13.3 — to fold detector noise into the training loop.

## References

[1] Mengu, D., Zhao, Y., Yardimci, N.T., Rivenson, Y., Jarrahi, M., & Ozcan, A. (2020). "Misalignment resilient diffractive optical networks." *Nanophotonics*, 9(13), 4207–4219. [The "vaccination" method: training with modeled lateral and axial displacements to build alignment tolerance; the primary source for §14.4.2.2.]

[2] Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008. [The terahertz experiment whose millimeter features and $\lambda/10 \approx 75\ \mu$m tolerances make the sim-to-real gap manageable on a bench.]

[3] Mengu, D., Luo, Y., Rivenson, Y., & Ozcan, A. (2019). "Analysis of diffractive optical neural networks and their integration with electronic neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 3700114. [Systematic study of how phase quantization, layer count, and other non-idealities affect diffractive-network accuracy.]
