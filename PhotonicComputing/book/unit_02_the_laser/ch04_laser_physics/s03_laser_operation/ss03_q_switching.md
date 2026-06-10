# 4.3.3 Q-Switching

## High-Energy Pulses from Lasers

Mode-locking produces short pulses (femtoseconds to picoseconds) at high repetition rates (MHz–GHz) with modest peak powers. Q-switching produces much longer pulses (nanoseconds) but with very high peak powers, at lower repetition rates (kHz–MHz). The two techniques address different applications.

Q-switching is less directly relevant to photonic computing in the continuous-wave, low-power regimes that characterize matrix processors and fiber links, but it is important for pulsed lidar (light detection and ranging), optical coherence tomography, and pulsed quantum light source pumping. It is also instructive as an example of resonator dynamics.

## The Q-Switch Principle

The cavity quality factor $Q$ (proportional to photon lifetime $\tau_p$) can be controlled by an element — the Q-switch — that introduces switchable loss. The operation cycle:

1. **Q off (low cavity Q)**: The Q-switch introduces high loss, preventing lasing. The gain medium is pumped, building up a large population inversion far above the (now very high) threshold.

2. **Q switched on (high cavity Q)**: At the peak of population inversion, the Q-switch is removed, restoring the high-Q (low-loss) cavity. The gain is now far above threshold for the high-Q cavity.

3. **Pulse buildup**: Stimulated emission grows rapidly from spontaneous emission noise, depleting the inversion and producing an intense, short pulse.

4. **Pulse end**: When the inversion falls below threshold, stimulated emission ceases. The pulse exits the cavity.

## Rate Equations for Q-Switching

The Q-switched dynamics are governed by the same rate equations as cw operation, but with the cavity loss term $1/\tau_p$ switching from a large value (Q off) to a small value (Q on):

$$\frac{dN}{dt} = W_p - \frac{N}{\tau} - v_g \sigma N \phi$$

$$\frac{d\phi}{dt} = v_g \sigma N \phi - \frac{\phi}{\tau_p(t)}$$

where $1/\tau_p(t)$ jumps from a large to a small value at the switching time.

**Peak power**: Approximately $P_{peak} \approx \hbar\omega (N_i - N_f) V_a / \tau_p$, where $N_i$ is the initial (peak) inversion and $N_f$ is the final (after-pulse) inversion.

**Pulse energy**: $E_{pulse} = \int P\,dt \approx \hbar\omega (N_i - N_f) V_a \eta_e$, where $\eta_e$ is the extraction efficiency.

**Pulse duration**: $\Delta t \approx E_{pulse}/P_{peak} \approx \tau_p \cdot (N_i/N_{th} - 1)^{-1}$.

**Example**: An Nd:YAG Q-switched laser with $\tau_p = 10$ ns, $N_i/N_{th} = 10$: $\Delta t \approx 10$ ns / 9 $\approx$ 1 ns. With 10 mJ pulse energy: $P_{peak} = 10$ MW.

## Types of Q-Switches

- **Active**: Acousto-optic deflectors, Pockels cells (electro-optic). Switching time ~1 ns; allows variable repetition rates.
- **Passive**: Saturable absorbers (Cr:YAG for Nd:YAG, SESAM). Self-Q-switching triggered by intensity buildup. Simpler but less controlled.

## Q-Switching for Photonic Computing Applications

Q-switched lasers are used in photonic computing contexts primarily as pump sources: e.g., pulsed pumping of SPDC sources for heralded single-photon generation in quantum photonic processors (Unit VII), where the high peak power enables efficient parametric down-conversion with low duty cycle (reducing background counts between pulses).
