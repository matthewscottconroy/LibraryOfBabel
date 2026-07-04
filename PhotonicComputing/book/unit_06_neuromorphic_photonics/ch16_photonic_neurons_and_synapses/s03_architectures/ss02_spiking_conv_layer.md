# 16.3.2 Photonic Spiking Convolutional Layers

## Convolution as a bank of shared-weight dot products

A convolutional layer applies a small kernel of weights repeatedly across a feature map. For an input of height $H$, width $W$, and $C_\text{in}$ channels, convolved with $C_\text{out}$ kernels each of spatial size $K\times K$, every output activation is the dot product of one kernel with a local $K\times K\times C_\text{in}$ patch of the input:
$$y_{m,n}^{(o)}=\sum_{c=1}^{C_\text{in}}\sum_{u=1}^{K}\sum_{v=1}^{K}W^{(o)}_{u,v,c}\,x_{m+u,\,n+v,\,c}.$$
Two features of this expression make it a natural fit for broadcast-and-weight. First, it is nothing but a large collection of weighted sums — exactly the operation the microring weight bank of §16.3.1 performs. Second, the same kernel weights $W^{(o)}$ are reused at every spatial position $(m,n)$: convolution is *weight sharing*. In a photonic implementation the kernel is loaded once into the ring or phase-change weights and held there — with non-volatile PCM synapses, at zero holding power (§16.2) — while the input patches are streamed past it. Only the data moves; the weights stay put.

## WDM parallelism over the reduction

The dot product inside the sum is a reduction over the $K\!\times\!K\!\times\!C_\text{in}$ kernel taps. This is where wavelength multiplexing pays off: assign each tap of the flattened patch to its own carrier wavelength, broadcast all of them onto the weighted bus, and let the balanced detector integrate them. One detector reading then returns a complete dot product — a full patch's worth of multiply–accumulates — in a single shot, in the time of flight plus one integration window. The excitable neuron downstream converts that weighted sum into a spike, so the array outputs a *spiking* feature map rather than an analog one.

Feldmann et al. (2021) demonstrated exactly this style of wavelength-parallel convolution in an integrated photonic tensor core: a phase-change weight matrix addressed by WDM inputs drawn from a Kerr-comb source, read out by on-chip germanium photodetectors, computing convolutions in parallel at throughputs in the trillions of MAC/s. Shastri et al. (2021) place the scheme in the broader neuromorphic-photonic architecture landscape, where the same broadcast-and-weight substrate carries both the linear reduction and the spiking nonlinearity.

## The 2-D array and its scaling limits

A spiking convolutional layer is thus a 2-D array of photonic neurons that share a common set of kernel weights, with WDM supplying the reduction and passive fan-out supplying the spatial replication. Two hardware resources bound how much of the layer runs truly in parallel:

- **Wavelengths versus kernel size.** The reduction length $K^2 C_\text{in}$ cannot exceed the number of WDM channels the bus supports — on the order of ten to a few tens per FSR (§16.3.1). A kernel longer than the available channel count must be split into $\lceil K^2 C_\text{in}/N_\lambda\rceil$ partial sums accumulated over successive shots.
- **Detectors versus output channels.** Each output channel $o$ needs its own weighted tap and balanced detector to run concurrently; with fewer detectors than $C_\text{out}$, the output channels are time-multiplexed.

Everything not covered by wavelength or detector parallelism is paid for in time-steps.

### Worked Example: mapping a conv layer onto a WDM spiking array

Take a layer with a $32\times32$ input, $C_\text{in}=3$ channels, $K=3$ kernels, $C_\text{out}=16$ output channels, and "same" padding so the output is $32\times32\times16$.

*MAC count.*
$$\text{MACs}=H_\text{out}\,W_\text{out}\,C_\text{out}\,(K^2C_\text{in})=32\cdot32\cdot16\cdot(3\cdot3\cdot3)=1024\cdot16\cdot27=4.42\times10^{5}.$$

*Wavelengths.* Each output activation is a dot product over $K^2C_\text{in}=27$ taps. With $N_\lambda=27$ wavelengths, one balanced-detector reading computes a whole 27-MAC dot product in a single shot.

*Time-steps.* The number of output activations is $H_\text{out}W_\text{out}C_\text{out}=1024\cdot16=16{,}384$. Reusing a single weight bank across all positions and channels would take 16,384 shots. Deploying $C_\text{out}=16$ parallel weight banks that share the same 27-wavelength broadcast — one balanced detector per output channel — collapses the output-channel loop, leaving one shot per spatial position:
$$N_\text{shots}=H_\text{out}W_\text{out}=1024.$$
At a shot rate of $B=10$ GS/s (100 ps per integration window),
$$T_\text{layer}=1024\times100~\text{ps}=102~\text{ns},$$
for an effective throughput of
$$\frac{4.42\times10^{5}~\text{MAC}}{102~\text{ns}}\approx4.3\times10^{12}~\text{MAC/s}=4.3~\text{TMAC/s},$$
squarely in the trillions-of-MAC/s regime reported by Feldmann et al. (2021). Had the bus supported only $N_\lambda=14$ channels (the ring bank of §16.3.1), the 27-tap kernel would need $\lceil27/14\rceil=2$ shots per dot product, doubling the time to $\approx204$ ns.

The example makes the design pressure explicit: convolution's arithmetic is cheap and endlessly shareable, so the binding constraints are not the multiply–accumulates themselves but the number of wavelengths the comb and weight bank can resolve (setting how much of each dot product runs in one shot) and the number of detectors available to read output channels in parallel (setting how many feature maps run at once). Weight sharing is what makes the photonic mapping efficient: the kernel is programmed once into non-volatile synapses and reused across the entire spatial scan, so the recurring cost is only the streaming of input spikes.

---
## References

Feldmann, J. et al. (2021). "Parallel convolutional processing using an integrated photonic tensor core." *Nature*, 589(7840), 52–58.

Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D. & Prucnal, P.R. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15(2), 102–114.
