# 27.3.1 Free-Space Matrix-Vector Multiplication

## The Operation

Every architecture in this section reduces, at bottom, to one primitive: the matrix–vector product $y_i = \sum_j W_{ij} x_j$. This is the same primitive the MZI mesh computes (Chapter 11), but the free-space realization exploits a different physical resource — the transverse degrees of freedom of a propagating wavefront — and so scales differently and fails differently. There are two canonical ways to do it in free space, and it is worth keeping them distinct because they place the matrix in different physical locations and pay different costs.

## Route One: Fourier Optics and the 4f Correlator

The first route is the classical Fourier-optics processor, whose foundation is a single fact established earlier in this book (Chapter 2): a lens performs, exactly in the paraxial limit, a two-dimensional Fourier transform between its front and back focal planes [Goodman, *Introduction to Fourier Optics*, 2005]. Place an input transparency or SLM in the front focal plane of a first lens; its Fourier transform appears one focal length beyond. Insert a mask with transmission $H(f_x, f_y)$ there and a second lens to transform back — the *4f* system — and the output field is

$$E_\text{out}(x,y) \;\propto\; \mathcal{F}^{-1}\{\,H(f_x,f_y)\,\hat{E}_\text{in}(f_x,f_y)\,\} \;=\; E_\text{in}(x,y) * h(x,y),$$

a convolution of the input with the impulse response $h = \mathcal{F}\{H\}$. Multiplication in the Fourier plane is convolution in the image plane, executed for the entire field in one propagation. For convolutional and correlation-based workloads this is extraordinarily efficient: the transform that a digital processor computes with $O(N^2 \log N)$ operations is performed by the optics as a physical side effect of propagation, and the filter kernel is simply the mask. The catch, developed below, is that a fixed 4f system computes one convolution with one kernel; making the kernel programmable means putting an SLM in the Fourier plane and paying its reload time, and turning the operation into an arbitrary dense matrix — rather than a shift-invariant convolution — forgoes the compactness that made Fourier optics attractive in the first place.

## Route Two: Broadcast, Weight, and Integrate

The second route implements a general dense $W$ directly, without a Fourier plane. Encode the input vector $x$ as the intensities or field amplitudes of a row of SLM pixels (or a modulated laser array). Fan that light out — with a lens, a diffuser, or cylindrical optics — so that each input illuminates a column of a second, matrix-bearing plane whose transmission is $W_{ij}$. Then integrate: a detector or camera pixel that collects all the light routed to output index $i$ registers

$$y_i \;=\; \sum_j W_{ij}\,x_j,$$

the desired inner product, formed in a single shot by nothing more than transmission and summation on a photodiode. The optical latency is the transit time across the apparatus — nanoseconds — and the multiply-accumulate over all $j$ happens in parallel, in the analog domain, with no partial sums to store. This is the free-space analogue of the electronic crossbar, and it underlies most free-space optical matrix engines: the weights live on a mask or a second SLM, the summation is done by the detector's collection geometry, and the "computation" is a property of how photons are routed rather than of any active nonlinearity.

## The Space-Bandwidth Ceiling

The seductive number is the pixel count. An $N \times N$ SLM presents up to $N^2 \sim 10^6$ independently programmable elements in a single aperture, so that a broadcast-and-integrate processor can in principle host a weight matrix of order $10^6$ entries — three to four orders of magnitude beyond a state-of-the-art integrated mesh. But the *usable* channel count is not the pixel count; it is the space-bandwidth product (SBP) of the optical system, the number of resolvable, independent modes it can propagate without crosstalk. Diffraction sets this ceiling: a wavefront of aperture $A$ at wavelength $\lambda$ carries roughly $A/\lambda^2$ independent spatial modes, and the mapping from SLM pixels to detector pixels is a diffraction-limited point-spread function whose overlap between neighboring channels *is* off-diagonal error in $W$. Oversampling to suppress that crosstalk, guard bands, and the finite fill factor and phase stroke of real modulators all erode the nominal $N^2$ toward a smaller effective dimension. The honest figure of merit is not "megapixel matrix" but "how many mutually orthogonal channels survive alignment and diffraction," and it is invariably smaller.

## Encoding Signed and Complex Weights

A photodiode measures intensity — a nonnegative real number — but useful linear algebra needs signed and often complex weights. Free-space MVP therefore inherits the same encoding problem as every intensity-based analog optical processor (Chapter 25), and solves it with the same family of tricks. One adds a fixed *bias* so that a nonnegative measured quantity represents a signed value. One uses *differential detection*, splitting $W_{ij}$ into positive and negative masks whose two detector outputs are subtracted electronically to recover a bipolar result. Or one goes fully *coherent*, encoding amplitude and phase together and recovering the complex product with a reference beam and homodyne detection. Each fix buys signed arithmetic at a cost — doubled hardware, an interferometrically stable reference, or a phase-calibrated SLM — and each cost is paid at the interface, not in the propagation.

## Where the Energy and Time Actually Go

This is the section's recurring lesson, and free-space MVP states it cleanly. The optical multiply-accumulate is nearly free: light crosses the processor in nanoseconds and dissipates nothing in the act of being summed on a detector. What is not free is getting data in and out. Loading a new input or a new weight page means driving a DAC and waiting for the SLM to settle — of order milliseconds for a phase liquid-crystal-on-silicon panel running near 60 Hz, down to the kilohertz regime for the fastest digital micromirror devices (DMDs), which are faster but binary, so that grayscale must be built up by time or space multiplexing. Reading the result means exposing and clocking out a camera and running its pixels through ADCs. In a system whose optics finish in nanoseconds, a throughput ceiling set by kilohertz frame rates and an energy budget dominated by modulator drive and photodetection are the true specification. The propagation is free; the interfaces are the machine. Any claim of a free-space "optical TOPS" figure that quotes the propagation and omits the frame-rate-limited reload and readout should be read against the accounting discipline of Chapter 25.

## Boundary Results

Two demonstrations mark the edges of what the primitive can do. At the high-dimensional end, optical *random-feature* processors exploit the fact that light passing through a strongly scattering medium — a layer of white paint, a diffuser — undergoes a fixed, dense, effectively random linear transformation of enormous dimension; measuring the transmitted speckle realizes a random projection with an implicit matrix of size up to $\sim 10^6$, approximating kernels "at the speed of light" without ever fabricating the matrix [Saade et al., *IEEE ICASSP*, 2016]. The medium supplies the parallelism for free, at the price of a matrix one does not get to choose. At the low-energy end, a coherent free-space matrix multiplier has been operated in the regime of *less than one photon per multiplication*, using massive spatial and temporal averaging to pull a signed result out of shot noise and demonstrating neural-network inference at an optical energy per multiply-accumulate far below any electronic multiplier [Wang et al., *Nature Communications*, 2022]. That result is the cleanest existing bound on the optical energy floor of the operation — and, read carefully, it is also a demonstration of how much averaging, hence how much interface time and detector energy, sub-photon operation actually costs. Both results are best read through the lens of the field's canonical review, which frames free-space "deep optics" as a co-designed optical-plus-electronic inference system rather than an optical processor alone [Wetzstein et al., *Nature*, 2020].

---

*References*

[1] Goodman, J.W. (2005). *Introduction to Fourier Optics*, 3rd ed. Roberts & Company.

[2] Saade, A., et al. (2016). "Random projections through multiple optical scattering: approximating kernels at the speed of light." *IEEE ICASSP*.

[3] Wetzstein, G., et al. (2020). "Inference in artificial intelligence with deep optics and photonics." *Nature* 588, 39.

[4] Wang, T., et al. (2022). "An optical neural network using less than 1 photon per multiplication." *Nature Communications* 13, 123.
