# 24.2.1 The Transfer Matrix Method

The abstraction that makes system-scale photonic simulation possible is the **scattering matrix**. A component with $N$ optical ports is described not by its internal fields but by a matrix $S$ relating the complex amplitudes of the waves leaving the ports, $\mathbf{b}$, to those entering them, $\mathbf{a}$:

$$\mathbf{b} = S\,\mathbf{a}, \qquad S_{ij} = \frac{\text{amplitude out of port } i}{\text{amplitude into port } j}.$$

Each $S_{ij}$ is complex and frequency-dependent: its magnitude encodes transmission or coupling, its phase encodes optical delay. For a passive, lossless, reciprocal device the matrix obeys two constraints that are worth internalizing because they catch modeling errors instantly: **reciprocity** makes $S$ symmetric ($S_{ij}=S_{ji}$), and **losslessness** makes it unitary ($S^\dagger S = I$, i.e. power in equals power out). A directional coupler, an MMI, a Y-junction, a grating coupler — each reduces to a small $S$ matrix, extracted once from FDTD or from an analytic model, and thereafter reused at negligible cost.

## Scattering versus Transfer Matrices

Scattering matrices describe components perfectly but cascade awkwardly: connecting two $S$-blocks in series requires the **Redheffer star product**, which solves for the fields on the shared internal ports. For the special but common case of a *chain* — components connected one after another along a single waveguide, as in a Bragg grating, a coupled-resonator optical waveguide (CROW), or a thin-film stack — there is a tidier device. Re-express each component as a **transfer (ABCD) matrix** $T$ that maps the forward and backward amplitudes on its *left* face to those on its *right* face. Because the output plane of one element is the input plane of the next, cascading becomes ordinary matrix multiplication:

$$T_\text{total} = T_N\,T_{N-1}\cdots T_2\,T_1.$$

This is the "transfer matrix method." Its convenience — multiply to cascade — is why it is the standard analytic tool for layered and periodic structures. Its hazard is numerical: transfer matrices contain growing exponentials for evanescent or lossy sections, so long chains can lose precision, and general (non-chain) circuit topologies are better handled by the scattering-matrix nodal solvers of the next subsection. For chains, though, the transfer matrix is unbeatable, and it yields closed-form answers that build intuition no black-box simulator provides.

## Worked Example: The All-Pass Ring Resonator

The microring is the canonical analytic circuit, and it exposes every quantity a resonator designer cares about. An all-pass ring couples a single bus waveguide to a loop of circumference $L = 2\pi R$ through a point coupler with **self-coupling** $r$ and cross-coupling $\kappa$, obeying $r^2+\kappa^2=1$ for a lossless coupler. Let $a=e^{-\alpha L/2}$ be the round-trip amplitude transmission (so $a^2$ is the round-trip power survival, including propagation loss) and $\theta=\beta L = (2\pi n_\text{eff}/\lambda)L$ the round-trip phase. Summing the infinite series of circulating paths gives the through-port power transmission

$$T = \frac{a^2 - 2ra\cos\theta + r^2}{1 - 2ra\cos\theta + r^2a^2}.$$

**Resonance and free spectral range.** Resonances occur when $\theta = m\,2\pi$ (an integer number of wavelengths fit the loop), where $\cos\theta=1$ and $T$ dips. Adjacent resonances are separated by the free spectral range

$$\text{FSR} = \frac{\lambda^2}{n_g L}.$$

Take the Chapter 23 ring: $R = 5\ \mu\text{m}$ so $L = 31.4\ \mu\text{m}$, with $n_g = 4.2$ at $\lambda = 1550$ nm. Then $\text{FSR} = (1550\ \text{nm})^2/(4.2 \times 31.4\ \mu\text{m}) \approx 18.2$ nm — reproducing, from the circuit model alone, the 18 nm FSR quoted for that ring in the fabrication chapter.

**Linewidth, quality factor, and finesse.** Near a resonance the dip has a Lorentzian shape whose full width at half maximum sets the loaded quality factor $Q=\lambda/\text{FWHM}$ and the finesse $\mathcal{F}=\text{FSR}/\text{FWHM}$, with

$$\mathcal{F} = \frac{\pi\sqrt{ra}}{1-ra}.$$

To hit $Q = 15{,}000$ — the value used for the weight-bank sensitivity analysis of Chapter 23 — the linewidth must be $\text{FWHM}=\lambda/Q = 1550/15{,}000 \approx 0.10$ nm, giving a finesse $\mathcal{F} = 18.2/0.10 \approx 177$. Inverting the finesse formula requires $ra\approx 0.982$: the product of coupling transmission and round-trip survival must sit just below unity, which is why high-$Q$ rings demand both low loss and weak coupling.

**Critical coupling.** When $a=r$ — round-trip loss exactly matched by the coupling — the numerator vanishes at resonance and $T\to 0$: *all* input power is dissipated in the ring. Under-coupling ($r>a$) and over-coupling ($r<a$) both raise the transmission minimum, and they can be distinguished experimentally by the sign of the phase response. This single condition governs the design of every ring modulator, filter, and weight element.

**Closing the loop with fabrication.** The resonance sits at $\theta=2\pi m$, i.e. $m\lambda_\text{res}=n_\text{eff}L$, so a width-induced index error $\delta n_\text{eff}$ shifts it by $\delta\lambda = \lambda\,\delta n_\text{eff}/n_g$ — the exact relation used in Chapter 23 to convert a 1 nm linewidth error into a $\sim$0.55 nm resonance shift, tens of linewidths for this $Q$. The circuit model and the variability model are the same equation viewed from two directions.

## Extracting Compact Models from Field Simulations

Most real components have no closed form. The workflow is then: run one broadband FDTD simulation (Section 24.1.1), record the transmission and reflection at every port over the band, and store the result as a table of complex $S_{ij}(\lambda)$ — the component's **compact model**. Passivity ($S^\dagger S \preceq I$) and reciprocity are checked as sanity constraints, and the tabulated model is dropped into the circuit simulator as a reusable block. This is the join between the two halves of the chapter: the rigorous solver pays the cost once per component, and the transfer- and scattering-matrix machinery composes those models into systems of arbitrary size for essentially free. The next subsection describes the simulators that perform that composition at the scale of a real photonic processor.
