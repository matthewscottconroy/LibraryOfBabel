# 4.1.2 Three- and Four-Level Systems

## The Solution: Decouple Pump and Signal

The two-level laser is impossible because the pump and signal share the same transition. The solution is to introduce additional energy levels so that the pump transition and the laser transition are distinct. This allows the pump to create excited-state population without simultaneously destroying it.

The two canonical schemes are the three-level and four-level laser.

## The Three-Level System

In a three-level laser, the energy levels are ordered $E_1 < E_2 < E_3$. The pump excites atoms from the ground level $|1\rangle$ to level $|3\rangle$ (the pump band). Level $|3\rangle$ decays rapidly and non-radiatively to level $|2\rangle$ (the upper laser level). The laser transition is $|2\rangle \to |1\rangle$, which is the ground state — hence the name three-level.

**Rate equations** (neglecting pump depletion, assuming fast $|3\rangle \to |2\rangle$ decay):

$$\frac{dN_2}{dt} = W_p N_1 - \frac{N_2}{\tau_2} - \sigma \phi (N_2 - N_1) c$$

$$N_1 + N_2 = N_T \quad \text{(total atom density)}$$

where $W_p$ is the pump rate (s$^{-1}$), $\tau_2$ is the upper laser level lifetime, $\sigma$ is the stimulated emission cross-section, $\phi$ is the photon density, and $c$ is the speed of light.

**Threshold condition**: Population inversion ($N_2 > N_1$) requires:

$$W_p > \frac{1}{\tau_2}$$

i.e., the pump rate must exceed the spontaneous decay rate of the upper level. Moreover, since $N_1 + N_2 = N_T$, inversion requires $N_2 > N_T/2$: more than half the atoms must be in the excited state. This makes three-level lasers relatively inefficient and high-threshold.

**Example — Ruby laser and EDFA**: The original Maiman ruby laser is a three-level system: Cr³⁺ ions are pumped by green light (550 nm) into a broad pump band, which relaxes to the upper laser level (lifetime ~3 ms), and the laser transition occurs at 694.3 nm back to the ground state. Erbium-doped fiber amplifiers are also three-level systems at 1550 nm: the $^4$I$_{13/2}$ upper level decays to the $^4$I$_{15/2}$ ground state manifold. This is why EDFAs require high pump powers (~100 mW at 980 nm or 1480 nm) to fully invert the erbium ions.

## The Four-Level System

In a four-level laser, the energy levels are $E_1 < E_2 < E_3 < E_4$. The pump excites from the ground $|1\rangle$ to $|4\rangle$ (pump band); fast non-radiative decay populates $|3\rangle$ (upper laser level); the laser transition is $|3\rangle \to |2\rangle$ (lower laser level, not the ground state); and $|2\rangle$ rapidly empties by non-radiative decay to the ground state $|1\rangle$.

The crucial difference: **the lower laser level is not the ground state**. It empties rapidly, maintaining a near-zero population $N_2 \approx 0$. This means:

- Inversion ($N_3 > N_2$) is achieved for any $N_3 > 0$ — no threshold on pump power is needed to achieve inversion in principle
- The laser threshold is set entirely by cavity losses, not by the need to deplete the ground state

**Rate equations** (four-level, ignoring lower-level population):

$$\frac{dN_3}{dt} = W_p (N_T - N_3) - \frac{N_3}{\tau_3} - \sigma \phi c N_3$$

$$\frac{d\phi}{dt} = \sigma \phi c N_3 - \frac{\phi}{\tau_c} + \beta \frac{N_3}{\tau_3}$$

where $\tau_c = 2L/(c \ln(1/R_1 R_2) + \delta)$ is the cavity photon lifetime and $\beta$ is the fraction of spontaneous emission coupled into the lasing mode (small, typically $10^{-5}$ to $10^{-3}$).

**Threshold condition**: Above threshold, gain equals loss:

$$\sigma c N_{3,\text{th}} = \frac{1}{\tau_c}$$

$$N_{3,\text{th}} = \frac{1}{\sigma c \tau_c}$$

This is always achievable at finite pump power, unlike the three-level case.

**Example — Nd:YAG**: The most important four-level solid-state laser: Nd³⁺ ions in a YAG (yttrium aluminum garnet) host are pumped at 808 nm and lase at 1064 nm. The lower laser level ($^4$I$_{11/2}$ manifold) lies 2000 cm$^{-1}$ above the ground state — well above $k_BT$ at room temperature (208 cm$^{-1}$), so thermal population is negligible. Threshold inversion densities are readily achieved with low pump powers. The 1064 nm output can be frequency-doubled to 532 nm (green) using SHG.

## Semiconductor Lasers: A Special Case

Semiconductor lasers are neither purely three- nor four-level systems. In a direct-bandgap semiconductor (GaAs, InP, GaN), electrons and holes are the "atomic" populations:

- **Upper laser level**: electrons in the conduction band
- **Lower laser level**: holes in the valence band
- **Population inversion condition**: the quasi-Fermi levels for electrons ($F_c$) and holes ($F_v$) must satisfy:

$$F_c - F_v > E_g = \hbar\omega_{\text{lasing}}$$

This is the Bernard-Duraffourg condition (1961), the semiconductor analogue of population inversion [1].

Achieving this condition requires injecting sufficient carrier density $N$ (electrons/cm³). The gain coefficient of a semiconductor is approximately linear in carrier density above transparency:

$$g(N) \approx a(N - N_0)$$

where $a \approx 10^{-16}$ cm² is the differential gain coefficient, $N_0 \approx 10^{18}$ cm$^{-3}$ is the transparency carrier density (where gain = loss = 0), and $N$ is in cm$^{-3}$.

The specific dependence of gain on carrier density and wavelength determines the threshold current, the slope efficiency, and the modulation bandwidth of the semiconductor laser — all critical parameters for photonic computing applications.

## Summary of Level Schemes

| Property | Two-level | Three-level | Four-level | Semiconductor |
|---|---|---|---|---|
| Inversion possible (cw)? | No | Yes (>50% pumped) | Yes (any pump) | Yes (above $N_0$) |
| Threshold pump power | Infinite | High | Low | Low (mA range) |
| Examples | None | Ruby, EDFA (1550 nm) | Nd:YAG, most diodes | GaAs, InP, GaN |
| Lower laser level = ground state? | Yes | Yes | No | No (approximately) |

## References

[1] Bernard, M.G.A., & Duraffourg, G. (1961). "Laser conditions in semiconductors." *Physica Status Solidi*, 1(7), 699–703.
