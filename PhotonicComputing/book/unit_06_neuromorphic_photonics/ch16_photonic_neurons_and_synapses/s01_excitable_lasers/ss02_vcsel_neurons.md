# 16.1.2 VCSEL-Based Optical Neurons

## Why VCSELs are attractive neurons

A vertical-cavity surface-emitting laser (VCSEL) emits its beam perpendicular to the wafer surface rather than from a cleaved edge. This geometry gives VCSELs a set of properties that make them unusually well suited to serve as photonic neurons at scale. They are **compact** (cavity lengths of order a wavelength, active volumes of a few cubic micrometers) and therefore low-threshold and energy-efficient. Because they emit from the surface, they can be fabricated and tested as dense **two-dimensional arrays**, which is exactly the topology a layer of neurons wants — thousands of identical emitters on a single chip. They are **directly modulated** at multi-GHz rates through their injection current, and their circular, low-divergence output couples efficiently into optical fiber, so a VCSEL neuron is naturally **fiber-friendly** and telecom-compatible. Mass-produced by the datacom and sensing industries, they are also cheap and mature.

## The spiking mechanism: polarization-mode switching

Unlike the saturable-absorber laser of §16.1.1, a VCSEL does not need an embedded absorber to spike. A VCSEL supports two orthogonal, nearly degenerate **linear polarization modes** (say, $x$- and $y$-polarized) that share the same gain medium and therefore compete for carriers. Under suitable bias and optical injection, the device rests in one polarization state; a sufficiently strong perturbation flips it transiently into the orthogonal state and back, emitting a fast, stereotyped pulse of light in the process. The all-or-nothing character comes from the winner-take-all mode competition, and the recovery of the carrier reservoir supplies the refractory period. Because the switching is driven by the fast intracavity dynamics, the resulting spikes are **sub-nanosecond**.

Hurtado & Javaloyes (2015) demonstrated controllable excitable spiking in a **long-wavelength (1300 nm) VCSEL**, showing that polarized optical injection could evoke well-defined, threshold-dependent, sub-ns spikes and controllable spiking patterns at telecom wavelengths. Building on this, Robertson, Hejda, Bueno & Hurtado (2020) demonstrated that a spiking VCSEL neuron could perform genuine information processing: ultrafast temporal **integration** of successive inputs, **inhibition** (where an appropriately timed input *suppresses* firing, the analog of an inhibitory synapse), and **temporal pattern classification**, in which the neuron discriminates between different input spike patterns. This progression — from demonstrating excitability, to demonstrating the neuronal primitives of integration and inhibition, to demonstrating a small learning/classification task — mirrors the way one would validate any candidate neuron model.

## Excitatory and inhibitory operation

The ability to realize both excitation and inhibition in the same device is important architecturally. A network of purely excitatory neurons cannot implement the balanced, subtractive computations that make spiking networks expressive. In the VCSEL neuron, the sign of the effect depends on the polarization and detuning of the injected light relative to the resting mode: some inputs push the system toward switching (excitatory), while others stabilize the resting mode and raise the effective threshold (inhibitory). Combined with the wavelength-multiplexed weighting schemes of §16.3, this gives a route to signed synaptic connectivity built from off-the-shelf emitters.

## Worked Example: energy per spike and array throughput

Estimate the optical energy carried by a single VCSEL spike, and the aggregate spike throughput of a modest 2-D array.

**Energy per spike.** Model the spike as a rectangular optical pulse of peak power $P$ and duration $\tau$. Take representative values for a small long-wavelength VCSEL neuron: a peak output power $P \approx 1\ \text{mW}$ and a sub-nanosecond spike duration $\tau \approx 150\ \text{ps}$. Then

$$E_{\text{spike}} = P\,\tau \approx (1\times10^{-3}\ \text{W})(1.5\times10^{-10}\ \text{s}) = 1.5\times10^{-13}\ \text{J} = 150\ \text{fJ}.$$

So the emitted optical energy per spike is on the order of $10^{-13}\ \text{J}$, i.e. a few hundred femtojoules to about a picojoule depending on the exact power and width. (The *wall-plug* energy is larger because of the laser's finite efficiency, but the emitted-photon budget already sets the scale for a downstream fan-out network.) For orientation, at $1300\ \text{nm}$ a single photon carries $E_{\text{ph}} = hc/\lambda \approx 1.5\times10^{-19}\ \text{J}$, so a $150\ \text{fJ}$ spike contains roughly $E_{\text{spike}}/E_{\text{ph}} \approx 1\times10^{6}$ photons — a comfortably classical, easily detected pulse.

**Array throughput.** Suppose each neuron can fire at up to $f_{\max}\approx 2\ \text{GHz}$ (set by its sub-ns spike plus refractory recovery), and we tile a $10\times10 = 100$-neuron VCSEL array. If all neurons are active, the aggregate spike rate is

$$R_{\text{array}} = N\,f_{\max} = 100 \times (2\times10^{9}\ \text{s}^{-1}) = 2\times10^{11}\ \text{spikes/s}.$$

At $\sim\!150\ \text{fJ}$ per spike, the corresponding emitted optical power is $R_{\text{array}}\times E_{\text{spike}} \approx (2\times10^{11})(1.5\times10^{-13}\ \text{J/s}) \approx 30\ \text{mW}$ spread across 100 emitters — a manageable thermal load. This is the payoff of arrayability: because VCSELs are surface-emitting and can be fabricated in dense 2-D grids, throughput scales with the number of emitters on the wafer, whereas an edge-emitting laser neuron must be laid out and wired one device at a time. The same property makes VCSEL arrays a natural substrate for the spiking convolutional and 2-D neuron-layer architectures discussed later in the chapter.

## References

- Hurtado, A. & Javaloyes, J. (2015). "Controllable spiking patterns in long-wavelength vertical cavity surface emitting lasers for neuromorphic photonics systems." *Applied Physics Letters*, 107(24), 241103.
- Robertson, J., Hejda, M., Bueno, J. & Hurtado, A. (2020). "Ultrafast optical integration and pattern classification for neuromorphic photonics based on spiking VCSEL neurons." *Scientific Reports*, 10, 6098.
- Prucnal, P.R., Shastri, B.J., Ferreira de Lima, T., Nahmias, M.A. & Tait, A.N. (2016). "Recent progress in semiconductor excitable lasers for photonic spike processing." *Advances in Optics and Photonics*, 8(2), 228–299.
