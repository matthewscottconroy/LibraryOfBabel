# 16.3.1 WDM Photonic Spiking Networks (Broadcast-and-Weight)

## A shared bus and a private wavelength

In a broadcast-and-weight network, every neuron is assigned its own optical carrier wavelength. Neuron $i$ emits its spike train — a sequence of picosecond optical pulses from an excitable laser (§16.1) — modulated onto carrier $\lambda_i$. The outputs of all $N$ neurons are wavelength-multiplexed onto a single *broadcast waveguide*, so that the bus carries the superposition
$$P_\text{bus}(t)=\sum_{i=1}^{N}P_i(t)\ \text{at}\ \lambda_i,$$
a comb of $N$ independently modulated channels on one physical line. Because the channels occupy different wavelengths, they propagate without interfering, and the passive splitting of light lets the bus be tapped by every neuron in the network at once. This is the optical solution to the fan-out problem that cripples electronic spiking hardware, where each additional postsynaptic connection needs its own metal wire and driver, with RC-limited bandwidth on every branch.

## The microring weight bank

Each *receiving* neuron taps the shared bus through a **microring weight bank**: a row of add–drop microring resonators side-coupled between the bus (through) waveguide and a drop waveguide (Tait et al. 2016). Ring $j$ in the bank is tuned so that its resonance sits on channel $\lambda_j$. On resonance the ring routes a fraction $d_j$ of the power at $\lambda_j$ to the drop port and passes the remaining $(1-d_j)$ to the through port; off-resonance wavelengths pass by untouched. The drop fraction $d_j\in[0,1]$ is the *synaptic weight*, and it is set continuously either by thermo-optic tuning of the ring (detuning the resonance relative to the channel) or, for a non-volatile weight, by the state of a phase-change patch on the ring (§16.2). One ring per channel gives the receiving neuron an independently programmable connection to every neuron in the network.

## Balanced detection makes weights signed

A drop fraction is non-negative, but neural weights must inhibit as well as excite. The broadcast-and-weight architecture recovers a signed weight by **balanced photodetection**: the drop port and the through port of a channel are sent to the two photodiodes of a balanced pair, which subtract their photocurrents. For channel $j$ the net contribution is
$$I_j=\mathcal{R}\big[\underbrace{d_j P_j}_{\text{drop}}-\underbrace{(1-d_j)P_j}_{\text{through}}\big]=\mathcal{R}\,(2d_j-1)\,P_j,$$
so the effective weight is
$$\boxed{\,w_j=2d_j-1\in[-1,+1]\,}$$
with $\mathcal{R}$ the detector responsivity. A fully dropped channel ($d_j=1$) is maximally excitatory ($w_j=+1$); a fully passed channel ($d_j=0$) is maximally inhibitory ($w_j=-1$); a half-dropped channel ($d_j=0.5$) is disconnected ($w_j=0$). Summed over channels, the single balanced photocurrent
$$I_\text{sum}=\mathcal{R}\sum_{j=1}^{N}(2d_j-1)\,P_j$$
is the weighted input to the neuron. This photocurrent drives the neuron's nonlinear excitable element — an injection-locked or saturable-absorber laser, or a modulator-based neuron — which integrates it and fires when threshold is crossed. WDM has performed the entire fan-in and weighting in the optical domain, in one waveguide, in the time of flight.

As a concrete illustration, take three presynaptic channels each delivering $P_j=20~\mu$W, drop fractions $d=(0.9,\,0.5,\,0.2)$, and $\mathcal{R}=0.8$ A/W. The weights are $w=(0.8,\,0.0,\,-0.6)$, and the net photocurrent is $I_\text{sum}=0.8~\text{A/W}\times(0.8+0.0-0.6)\times20~\mu\text{W}=0.8\times0.2\times20~\mu\text{W}=3.2~\mu$A — a small net excitation, the first channel outweighing the third.

## How many neurons fit on one bus?

The channel count $N$ is limited by how many resolvable microring resonances fit within one free spectral range (FSR). Two constraints bound it:

- **Free spectral range.** A ring of circumference $L=2\pi R$ and group index $n_g$ has $\text{FSR}=c/(n_g L)$. All channels must lie within one FSR, or a ring tuned to $\lambda_j$ would also drop a channel one FSR away (resonance aliasing).
- **Channel spacing and crosstalk.** Each ring has a finite linewidth $\delta\nu=\nu_0/Q_L$. Neighboring channels must be spaced by several linewidths, or a ring intended for $\lambda_j$ will partially drop its neighbors — spectral crosstalk that corrupts the weighting.

Then $N_\text{max}\approx \text{FSR}/\Delta\nu_\text{ch}$. Thermal tuning adds a practical limit: each ring must be trimmed onto its channel and held there, and reconfiguring a weight across a full FSR requires heating over tens of °C, so thermo-optic power and thermal crosstalk between densely packed rings cap the usable count below the spectral maximum.

### Worked Example: channel count of a silicon weight bank

Consider add–drop rings of radius $R=8~\mu$m in a silicon strip waveguide with group index $n_g=4.0$, at $\lambda_0=1550$ nm ($\nu_0=c/\lambda_0=1.935\times10^{14}$ Hz).

*Free spectral range.* Circumference $L=2\pi R=2\pi(8~\mu\text{m})=50.3~\mu$m, so
$$\text{FSR}=\frac{c}{n_g L}=\frac{3.0\times10^{8}}{4.0\times50.3\times10^{-6}}=1.49\times10^{12}~\text{Hz}=1.49~\text{THz}.$$

*Linewidth.* For a loaded quality factor $Q_L=1.0\times10^{4}$,
$$\delta\nu=\frac{\nu_0}{Q_L}=\frac{1.935\times10^{14}}{1.0\times10^{4}}=19.4~\text{GHz}.$$

*Channel spacing.* Requiring $\Delta\nu_\text{ch}\ge5\,\delta\nu$ to suppress drop-port crosstalk gives $\Delta\nu_\text{ch}\approx97~\text{GHz}$, conveniently near the 100 GHz DWDM grid.

*Maximum channels.*
$$N_\text{max}=\frac{\text{FSR}}{\Delta\nu_\text{ch}}=\frac{1.49\times10^{12}}{1.0\times10^{11}}\approx14.9\ \longrightarrow\ \boxed{N\approx14\ \text{WDM neurons per bus}}.$$

Fourteen neurons is modest; scaling up means increasing the FSR (smaller rings), raising $Q_L$ to pack channels tighter — at the cost of lower spiking bandwidth, since $\delta\nu$ also sets the per-channel modulation bandwidth, here $\sim19$ GHz, or a $\sim50$ ps minimum pulse — or federating several buses. Using this architecture at network scale, Tait et al. (2017) demonstrated a 49-node silicon photonic weight-bank network — the first integrated broadcast-and-weight system of that size, and a direct realization of the spike-processing architecture proposed by Tait et al. (2014).

---
## References

Tait, A.N., Nahmias, M.A., Shastri, B.J. & Prucnal, P.R. (2014). "Broadcast and weight: an integrated network for scalable photonic spike processing." *Journal of Lightwave Technology*, 32(21), 4029–4041.

Tait, A.N., de Lima, T.F., Zhou, E., Wu, A.X., Nahmias, M.A., Shastri, B.J. & Prucnal, P.R. (2017). "Neuromorphic photonic networks using silicon photonics weight banks." *Scientific Reports*, 7, 7430.

Tait, A.N., Wu, A.X., de Lima, T.F., Zhou, E., Shastri, B.J., Nahmias, M.A. & Prucnal, P.R. (2016). "Microring weight banks." *IEEE Journal of Selected Topics in Quantum Electronics*, 22(6), 312–325.
