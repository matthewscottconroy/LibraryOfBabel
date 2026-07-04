# 16.2.1 Phase-Change-Material Optical Synapses

A biological synapse stores a weight — a scalar efficacy that scales every spike passing from the presynaptic to the postsynaptic neuron. To build an optical synapse we need a device that (i) sits in the path of a guided optical signal, (ii) multiplies that signal by a stored number, and (iii) holds that number indefinitely without power. Chalcogenide **phase-change materials (PCMs)** meet all three requirements, and they have become the dominant technology for non-volatile photonic weights.

## Two structural phases, two optical constants

The canonical PCM is GST, $\mathrm{Ge_2Sb_2Te_5}$, an alloy from the Ge–Sb–Te family long used in rewritable optical discs and electronic memory. GST is remarkable because it has two solid phases that are both stable at room temperature yet optically very different:

- **Amorphous:** a disordered atomic network with a comparatively low refractive index and, crucially, a *small* extinction coefficient $k$ — the material is relatively transparent at telecom wavelengths.
- **Crystalline:** an ordered, rocksalt-like lattice with a higher refractive index and a *large* extinction coefficient — the material is strongly absorbing at 1550 nm.

The complex refractive index $\tilde n = n + i k$ therefore changes dramatically between phases, and the change in the imaginary part $k$ is what an absorption-based synapse exploits. Because the two phases are separated by an energy barrier, each is non-volatile: once written, a GST cell retains its phase — and hence its optical constants — for years at room temperature, dissipating **zero static power**. This is the property that makes PCM so attractive for holding thousands of network weights.

## The device: PCM on a waveguide

A photonic PCM synapse is built by patterning a small patch of GST — typically a few micrometres long and a few tens of nanometres thick — directly on top of a single-mode $\mathrm{Si}$ or $\mathrm{Si_3N_4}$ waveguide. The guided mode is mostly confined to the high-index core, but its **evanescent tail** extends a short distance into the cladding, where it overlaps the GST patch. Light propagating past the cell is thus sampled by the PCM: in the amorphous state the mode barely notices the nearly transparent film and passes with low loss; in the crystalline state the absorbing film attenuates the mode strongly.

The waveguide **transmission** $T$ therefore encodes the synaptic weight. Writing $c \in [0,1]$ for the *crystalline fraction* of the patch — the volume fraction converted to the crystalline phase — the modal power-attenuation coefficient rises monotonically with $c$. A convenient first-order model interpolates the modal absorption linearly between the fully amorphous ($\alpha_a$) and fully crystalline ($\alpha_c$) limits:

$$\alpha(c) \approx \alpha_a + c\,(\alpha_c - \alpha_a), \qquad T(c) = e^{-\alpha(c)\,L} \tag{1}$$

where $L$ is the cell length. Because the crystalline fraction is a *continuous* physical variable, $T$ is an analog weight — a point developed fully in §16.2.4.

## Writing the weight: SET and RESET

The phase is switched by heating the film, and — this is the elegant part — the heat can be delivered *optically*, through the very same waveguide, using pulses too energetic to be mistaken for signal light. There are two operations:

- **SET (amorphous → crystalline, toward high absorption):** a *longer, moderate-power* pulse heats the film above its crystallization temperature (~150–200 °C) but keeps it below melting. Held in this window, atoms gain enough mobility to nucleate and grow crystallites, so the crystalline fraction increases. Partial pulses give partial crystallization — the basis of multi-level programming.
- **RESET (crystalline → amorphous, toward low absorption):** a *short, intense* pulse briefly melts the film (above ~600 °C) and is then removed so quickly that the melt **quenches** into the disordered amorphous state before it can recrystallize. RESET pulses are higher in peak power but shorter — often sub-nanosecond — than SET pulses.

Reading is non-destructive: a low-power probe pulse, well below the crystallization threshold, simply measures $T$. The same port does write and read — the synapse is fully **all-optical**, with no electrical contacts to the cell.

## Demonstrations

Ríos et al. (2015) reported the first integrated all-photonic non-volatile memory of this kind, storing multiple distinct transmission levels in a GST cell on a waveguide, with retention projected over years and zero static power. Cheng et al. (2017) built an explicit **on-chip photonic synapse**, combining a PCM cell with integrated photonics to emulate synaptic weighting. Feldmann et al. (2019) then wired PCM synapses together with microring resonators into an all-optical spiking neurosynaptic network that demonstrated on-chip, self-learning plasticity — the subject of §16.4. Together these established PCM as the workhorse non-volatile weight for neuromorphic photonics.

## Worked Example: crystalline fraction to weight

*Consider a GST cell whose fully amorphous transmission is $T_a = 0.970$ (insertion loss $0.13$ dB) and whose fully crystalline transmission is $T_c = 0.500$ (loss $3.01$ dB). What weight results from crystallizing half the patch, $c = 0.5$?*

From $T = e^{-\alpha L}$, the two endpoints correspond to
$$\alpha_a L = -\ln T_a = -\ln 0.970 = 0.0305, \qquad \alpha_c L = -\ln T_c = -\ln 0.500 = 0.6931.$$
Applying the linear model (1) at $c = 0.5$:
$$\alpha(0.5)\,L = 0.0305 + 0.5\,(0.6931 - 0.0305) = 0.0305 + 0.3313 = 0.3618,$$
$$T(0.5) = e^{-0.3618} = 0.696.$$

Half-crystallization therefore yields a transmission weight $T \approx 0.70$, a change of $\Delta T = 0.970 - 0.696 = 0.274$ from the amorphous state. Note the weight is **not** halfway between $T_a$ and $T_c$: the normalized weight is
$$w = \frac{T - T_c}{T_a - T_c} = \frac{0.696 - 0.500}{0.970 - 0.500} = 0.42,$$
not $0.50$, because transmission depends *exponentially* on crystalline fraction. The map from $c$ to $T$ is nonlinear — one practical reason weights are set by closed-loop program-and-verify rather than open-loop pulse counting (§16.2.4).

*Write energy (order of magnitude).* A SET operation delivering a peak optical power of order $1$ mW for a duration of order $200$ ns deposits
$$E_\text{write} \sim P\,\Delta t \approx (1\times10^{-3}\ \text{W})(200\times10^{-9}\ \text{s}) = 2\times10^{-10}\ \text{J} = 200\ \text{pJ}.$$
Reported PCM write energies fall in roughly the ~0.1–1 nJ range and depend strongly on cell volume and geometry, so treat this as an order-of-magnitude figure. The key economic point is that this energy is paid **once**, at write time; thereafter the weight is held for years at zero power, so in a fixed-weight inference engine the amortized cost per operation is negligible.

---

## References

- Ríos, C., Stegmaier, M., Hosseini, P., Wang, D., Scherer, T., Wright, C.D., Bhaskaran, H. & Pernice, W.H.P. (2015). "Integrated all-photonic non-volatile multi-level memory." *Nature Photonics*, 9(11), 725–732.
- Cheng, Z., Ríos, C., Pernice, W.H.P., Wright, C.D. & Bhaskaran, H. (2017). "On-chip photonic synapse." *Science Advances*, 3(9), e1700160.
- Feldmann, J., Youngblood, N., Wright, C.D., Bhaskaran, H. & Pernice, W.H.P. (2019). "All-optical spiking neurosynaptic networks with self-learning capabilities." *Nature*, 569(7755), 208–214.
