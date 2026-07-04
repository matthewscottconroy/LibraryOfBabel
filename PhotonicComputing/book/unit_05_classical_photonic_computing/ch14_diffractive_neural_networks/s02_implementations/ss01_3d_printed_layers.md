# Subsection 14.2.1: 3D-Printed Diffractive Layers

## Orientation

The 3D-printed phase plate is the diffractive network's original body: the platform on which the first physical D2NN was built (Lin et al. 2018), and still the cheapest route to a working multilayer optical classifier. Its logic is direct — encode the trained phase mask as the physical thickness of a transparent slab, print the slab, and stack several with air gaps between them. This subsection explains why that logic works beautifully at terahertz and millimetre wavelengths and breaks down completely in the visible.

---

## 14.2.1.1 Phase Encoded as Surface Height

A wave crossing a dielectric slab of refractive index $n$ and local thickness $h(x,y)$ accumulates optical path relative to the surrounding air, acquiring the phase

$$\phi(x,y) = \frac{2\pi}{\lambda}\,(n-1)\,h(x,y).$$

Because the phase-only transmission $t^l(x,y) = e^{i\phi^l(x,y)}$ of Section 14.1 is periodic, the mask never needs more than a full $2\pi$ of modulation, which corresponds to a height swing of

$$h_{2\pi} = \frac{\lambda}{\,n-1\,}.$$

The trainable parameter of the framework — the per-pixel phase $\phi^l(x,y)$ — is therefore literally a relief map, and fabricating a layer means printing that map to a height precision fine enough to render the phase faithfully. Amplitude is left essentially untouched (a well-chosen low-loss polymer is nearly transparent), so the printed slab realizes the pure phase modulator the model assumes.

## 14.2.1.2 Why Terahertz, and With What Materials

The height tolerance a printer must hit scales with $h_{2\pi} \propto \lambda$, and the lateral pixel must be comparable to $\lambda$ to diffract usefully. Both requirements are generous when $\lambda$ is large. At terahertz and millimetre-wave frequencies the wavelength runs from roughly 3 mm (0.1 THz) down to 0.3 mm (1 THz), so a printer whose voxel is 50–100 μm resolves the surface to a small fraction of a wavelength — a few percent — and the diffractive pixel, at a few hundred micrometres, is a well-formed feature rather than a single dot.

The materials are commodity photopolymers and thermoplastics that happen to be transparent in this band, with refractive indices around $n \approx 1.6$–$1.7$ and modest absorption. Fabrication is ordinary additive manufacturing: stereolithography (SLA) cures a photopolymer layer by layer, and PolyJet jets and UV-cures droplets, both reaching tens-of-micrometre layer thicknesses. The finished plate is passive and permanent — its weights are frozen in cured plastic the instant it is printed.

## 14.2.1.3 Worked Example: A Layer at 0.4 THz

Take the operating point of the founding demonstration: $f = 0.4$ THz, so

$$\lambda = \frac{c}{f} = \frac{3\times10^8\ \text{m/s}}{0.4\times10^{12}\ \text{Hz}} = 0.75\ \text{mm},$$

in a polymer with $n = 1.7$. The full-$2\pi$ height is

$$h_{2\pi} = \frac{0.75\ \text{mm}}{1.7 - 1} = 1.07\ \text{mm}.$$

Encoding the phase to 8 bits (256 levels) would demand height steps of $1.07\ \text{mm}/256 \approx 4.2\ \mu$m. This is finer than a printer's lateral voxel, so in practice the achievable z-resolution (tens of micrometres) — not the 8-bit ideal — sets the quantization, delivering an effective 5–6 bits of phase. That is ample: D2NN accuracy saturates well below 8-bit phase resolution (Mengu et al. 2019), so the printer is not the accuracy bottleneck. A pixel pitch of 0.3–0.5 mm ($0.4$–$0.7\,\lambda$) then gives well-formed, wide-angle diffractive neurons, and a $200\times200$ array occupies an 8 cm aperture — the scale of the five stacked plates that classified MNIST at 91.75% (Lin et al. 2018; Luo et al. 2019).

## 14.2.1.4 Passive, Fixed — and Blocked from the Visible

Two properties follow from printing phase into plastic. First, the network is passive and non-reconfigurable: there is no gain and no adjustable knob, and to change a weight you print a new plate. This is a virtue for a deployed, power-free front end and a liability for training, which must be done entirely in simulation and transferred to hardware (the sim-to-real problem of Section 14.4).

Second, the platform cannot climb to the visible. Scaling the worked example to $\lambda = 532$ nm gives $h_{2\pi} = 532\ \text{nm}/0.7 \approx 760$ nm — a height swing a fine printer could just about manage — but the pixel pitch must fall to $\sim\lambda/2 \approx 266$ nm to preserve wide-angle diffraction, whereas a printer voxel of 50 μm is about $94\lambda$ across. A single voxel would span some ninety wavelengths, obliterating any sub-wavelength phase structure. The wavelength has shrunk by a factor of $\sim1400$ relative to the terahertz case while the printer's resolution has not, so visible-light diffractive networks must abandon additive manufacturing for the lithographic and liquid-crystal platforms of the next two subsections.

---

## References

[1] Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008. [The founding demonstration: five 3D-printed phase plates at 0.4 THz classifying MNIST — the platform this subsection dissects.]

[2] Mengu, D., Luo, Y., Rivenson, Y., & Ozcan, A. (2019). "Analysis of diffractive optical neural networks and their integration with electronic neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 3700114. [Quantifies how phase quantization and layer count affect accuracy, justifying the modest bit-depth that printers deliver.]

[3] Luo, Y., Mengu, D., Yardimci, N.T., Rivenson, Y., Veli, M., Jarrahi, M., & Ozcan, A. (2019). "Design of task-specific optical systems using broadband diffractive neural networks." *Light: Science & Applications*, 8, 112. [Printed terahertz diffractive systems, and the material and fabrication regime of millimetre-wave D2NNs.]
