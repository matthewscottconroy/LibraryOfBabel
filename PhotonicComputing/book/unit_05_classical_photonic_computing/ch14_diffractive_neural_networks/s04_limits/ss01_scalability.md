# Subsection 14.4.1: Scalability, Depth, and Width

## Orientation

Two numbers describe the size of a diffractive layer: its *width* — the number of independently controllable neurons (pixels) it carries — and the network's *depth* — how many such layers are stacked along the optical axis. Digital intuition says both should buy capacity without bound. Optics disagrees on both counts. Width is capped by diffraction through the space-bandwidth product; depth is subtler still, because passive diffractive layers compose linearly and cannot, by themselves, deepen a network in the sense a nonlinear digital stack does. This subsection quantifies both ceilings, then the footprint, connectivity, and energy costs that come with approaching them.

## 14.4.1.1 The Space-Bandwidth Product Ceiling on Width

The number of independent complex degrees of freedom a diffractive surface can imprint on a field is its space-bandwidth product (SBP). Over an aperture of side $A$ sampled at pixel pitch $\delta$, the one-dimensional count is $A/\delta$, so a square layer holds

$$\mathrm{SBP} = \left(\frac{A}{\delta}\right)^2 \ \text{neurons.}$$

Diffraction bounds how fine $\delta$ can be: a feature smaller than roughly $\lambda/(2\,\mathrm{NA})$ radiates only into evanescent waves that never reach the next layer, so the pixel pitch obeys $\delta \gtrsim \lambda/(2\,\mathrm{NA})$, where NA is the numerical aperture the geometry supports. Substituting the finest admissible pitch,

$$\mathrm{SBP} \lesssim \left(\frac{2A\,\mathrm{NA}}{\lambda}\right)^2.$$

Kulce et al. (2021) placed this counting on rigorous footing, computing the all-optical information-processing capacity of one and of several cascaded diffractive surfaces, and showing that $N$ surfaces can approach the SBP-set number of independent input–output connections. A diffractive layer is thus an information channel whose capacity is fixed by aperture, wavelength, and NA — not by how many pixels a fabricator chooses to print.

**Example (neuron budget: terahertz vs. visible).** Take an aperture $A = 6$ cm and a moderate $\mathrm{NA} = 0.4$. At the Lin et al. (2018) operating wavelength $\lambda = 0.75$ mm (0.4 THz),

$$\frac{2A\,\mathrm{NA}}{\lambda} = \frac{2(0.06)(0.4)}{0.75\times10^{-3}} = 64, \qquad \mathrm{SBP} \approx 64^2 \approx 4.1\times10^{3}.$$

Pushing to the hard diffraction limit $\mathrm{NA}\to 1$ raises the one-dimensional count to $2A/\lambda = 160$, i.e. $\mathrm{SBP}\approx 2.6\times10^4$. The $200\times200 = 4\times10^4$ neurons per layer that Lin used therefore sit essentially *at* the diffraction limit: they are affordable only because that experiment stacked its layers close together (large effective NA, pixel pitch $\approx 0.4$ mm $\approx 0.5\lambda$), not because pixels are free. Now keep the same 6 cm aperture and the same $\mathrm{NA}=0.4$ but move to visible light, $\lambda = 0.5$ μm:

$$\frac{2A\,\mathrm{NA}}{\lambda} = \frac{2(0.06)(0.4)}{0.5\times10^{-6}} = 9.6\times10^{4}, \qquad \mathrm{SBP}\approx 9.2\times10^{9}.$$

The neuron budget scales as $\lambda^{-2}$, so shrinking the wavelength by $1500\times$ multiplies capacity by $(1500)^2 \approx 2\times10^{6}$. This single scaling is the strongest argument for pushing D2NNs out of the terahertz and into the visible, or onto the integrated on-chip platforms of Section 14.2: the same footprint holds a million times more neurons.

## 14.4.1.2 Depth and the Linearity Subtlety

Adding layers is supposed to add representational power, and in the studies of Lin and of Mengu deeper diffractive networks do classify better — but with diminishing returns, and for a reason that must be stated carefully. Recall the forward model of Section 14.1: each layer applies a phase-only transmission $\mathrm{diag}(t^l)$, $t^l = e^{i\phi^l}$, followed by free-space (Rayleigh–Sommerfeld) propagation $H_d$. A stack of $L$ layers is therefore the *product*

$$M = H_d\,\mathrm{diag}(t^L)\,H_d\cdots H_d\,\mathrm{diag}(t^1),$$

a single complex-valued linear operator. No nonlinearity acts *between* layers — the field stays complex and never touches a detector until the output plane. Consequently a purely linear diffractive stack of any depth is mathematically equivalent to one (generally non-realizable) linear transform: depth adds no expressivity the way a ReLU-separated digital stack does.

Where, then, does the measured depth advantage come from? Two places. First, each layer is *phase-only* — a severe constraint, since one phase mask plus one propagation cannot realize an arbitrary dense transform. Cascading several phase-only layers with diffraction between them supplies enough free parameters to approximate a far wider class of complex-linear maps, and a larger effective connectivity, than a single layer can; more layers means more trainable degrees of freedom serving the same target operator (Mengu et al. 2019). Second, the network's only genuine nonlinearity is the terminal intensity readout $|\cdot|^2$ (Subsection 14.4.3): the classifier is a complex-linear map followed by a magnitude-squared, which is nonlinear in the input intensities and lends even a "linear" D2NN real discriminative power. Depth helps by realizing better linear maps under the phase-only, passive constraint — not by stacking nonlinearities that are simply not there.

## 14.4.1.3 Footprint, Connectivity, and Energy

Physical footprint is blunt: a network of $L$ layers spaced $d$ apart occupies an optical volume of transverse area $A^2$ and length $L\times d$. The spacing $d$ is not free either — all-to-all connectivity, the property that makes a diffractive layer a *fully connected* layer, requires each pixel's diffraction cone to fan out across the entire next aperture. A pixel of width $\delta$ diffracts into a half-angle $\theta\sim\lambda/\delta$, spreading to width $\sim d\lambda/\delta$ after distance $d$; demanding this reach the full aperture gives $d \gtrsim A\delta/\lambda$. Pack the layers closer and the fan-in shrinks, the connectivity becomes local (convolution-like) rather than global, and neurons at opposite edges no longer communicate.

Energy closes the accounting. A passive D2NN dissipates essentially zero power computing — the layers only redirect incident light — so the "inference energy" is whatever optical power must be launched to raise the output regions above the detector's noise floor. Total optical power scales as input intensity × aperture area, and the per-inference energy is set, as in any analog optical processor (Chapter 12), by the detector SNR the task demands: more neurons and finer output distinctions require more photons per region, hence more launched power. Scaling width therefore costs illumination power as well as area, and the two grow together.

## References

[1] Kulce, O., Mengu, D., Rivenson, Y., & Ozcan, A. (2021). "All-optical information-processing capacity of diffractive surfaces." *Light: Science & Applications*, 10, 25. [Rigorous capacity analysis of how many independent connections a diffractive surface or cascade can realize; the formal basis for the SBP ceiling used here.]

[2] Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008. [The founding D2NN paper; source of the $5\times200\times200$-neuron, 0.4 THz architecture whose neuron budget is worked out in the Example.]

[3] Mengu, D., Luo, Y., Rivenson, Y., & Ozcan, A. (2019). "Analysis of diffractive optical neural networks and their integration with electronic neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 3700114. [Quantifies how classification accuracy scales with the number of layers and neurons, and documents the diminishing returns of depth.]
