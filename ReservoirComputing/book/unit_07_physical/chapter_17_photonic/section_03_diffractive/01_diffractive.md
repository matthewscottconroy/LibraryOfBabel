# Diffractive Optical Networks as Reservoirs

## Lin et al. 2018: All-Optical Deep Neural Networks

Lin et al. [2018] demonstrated that free-space diffractive optical systems can implement multi-layer neural network computations entirely with light. Their D$^2$NN (Deep Diffractive Neural Network) consists of successive planar layers of diffractive elements — spatial light modulators (SLMs) or fabricated diffractive surfaces — separated by free-space propagation distances. Each layer applies a complex-valued transmission mask to the optical wavefront, followed by Huygens–Fresnel propagation to the next layer.

The key insight is that the optical wave field at each diffractive layer performs a spatial linear transformation (determined by the propagation kernel), and the amplitude or phase modulation at each layer introduces a pointwise nonlinearity (saturation, thresholding, or simply amplitude modulation). Together, these operations implement a multi-layer neural network at the speed of light.

## Huygens–Fresnel Propagation as the Reservoir Transformation

The Huygens–Fresnel propagation integral describes how a scalar optical field $u(\mathbf{r})$ at plane $z = 0$ propagates to field $u'(\mathbf{r}')$ at plane $z = d$:

$$u'(\mathbf{r}') = \frac{-i}{\lambda d} \int u(\mathbf{r}) \exp\!\left(\frac{i\pi}{\lambda d} |\mathbf{r}' - \mathbf{r}|^2\right) d\mathbf{r},$$

where $\lambda$ is the wavelength and $d$ is the propagation distance [Born & Wolf 1999]. This integral is a linear, shift-variant transformation determined entirely by $\lambda$ and $d$. In the Fourier plane ($d$ chosen for Fraunhofer diffraction), it reduces to a 2D Fourier transform.

The reservoir interpretation: the cascade of diffractive layers and propagation distances constitutes a fixed (after fabrication) transformation of the input optical field. This transformation is deterministic, physically realized by light propagation, and computes a rich nonlinear mapping of the input onto the detector plane — directly analogous to the fixed random weight matrix of a simulated reservoir [Bueno et al. 2018].

## The Reservoir Interpretation

In the reservoir computing framing, the diffractive optical network is:

- **Reservoir:** The cascade of diffractive layers, which transforms the input field into a high-dimensional representation at the output plane. This transformation is fixed (the diffractive patterns are fabricated and immutable) and nonlinear (complex amplitude modulation introduces phase nonlinearities).
- **Readout:** A learned amplitude mask at the output plane, which applies weighted sums over spatial detector pixels. This is trained to extract the desired information from the diffractive representation.
- **Input:** Spatial light modulator or object transparency at the input plane.

The readout is the only trained component — all diffractive layers are fixed, exactly as in standard reservoir computing where only the output weights are learned [Lin et al. 2018].

## Speed: Computation at the Speed of Light

The most striking property of diffractive optical networks is their computational speed. Light propagates through a diffractive layer at the speed of light ($c \approx 3 \times 10^8$ m/s). For a layer separation of $d = 1$ cm, the transit time is $d/c \approx 33$ ps. A 5-layer network spans $\sim 5$ cm and computes in $\sim 165$ ps — approximately 6 GHz computation rate. With terahertz light (infrared), this could reach exaflop-scale effective rates.

Importantly, this computation consumes no active power for the diffractive layers themselves — only passive diffraction and interference occur. Power is required only for the input (illumination source) and readout (detector).

## Limitations: No Temporal Dynamics

The critical limitation of feedforward diffractive networks as reservoirs is the absence of temporal dynamics. A feedforward diffractive system processes each input independently: there is no memory of previous inputs, no fading memory property, and no time-varying state. This disqualifies it as a reservoir in the strict sense — it is a spatial transformer, not a temporal filter.

The Boyd–Chua theorem requires fading memory as a necessary condition for universal approximation of temporal functionals. A feedforward diffractive network cannot satisfy this requirement.

## Modified Temporal Designs: Adding Delay Feedback

Several approaches add temporal memory to diffractive optical systems. **Feedback diffractive networks:** the output plane is optically fed back to the input plane with a delay, creating a recurrent system analogous to a delay-feedback reservoir. **Modulated diffractive network:** a spatial light modulator between layers allows dynamic updating of the network weights, enabling temporal adaptation. **Diffractive network with temporal multiplexing:** the input is modulated in time, and the diffractive network processes temporal sequences spatially spread across the aperture [Bueno et al. 2018].

These extensions restore the temporal dimension and bring diffractive optical networks into the reservoir computing framework properly, at the cost of increased system complexity.

---

## References

- Lin, X., Rivenson, Y., Yardimci, N. T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). All-optical machine learning using diffractive deep neural networks. *Science*, 361(6406), 1004–1008.
- Bueno, J., Maktoobi, S., Froehly, L., Fischer, I., Jacquot, M., Larger, L., & Brunner, D. (2018). Reinforcement learning in a large-scale photonic recurrent neural network. *Optica*, 5(6), 756–760.
- Born, M., & Wolf, E. (1999). *Principles of Optics* (7th ed.). Cambridge University Press.
