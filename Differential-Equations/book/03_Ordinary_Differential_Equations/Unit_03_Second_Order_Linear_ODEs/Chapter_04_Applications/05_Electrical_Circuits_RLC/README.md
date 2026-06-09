# Electrical Circuits: The RLC Circuit

A series RLC circuit consists of a resistor (resistance $R$ ohms), an inductor (inductance $L$ henrys), and a capacitor (capacitance $C$ farads) connected in series with a voltage source $E(t)$ volts. By Kirchhoff's voltage law, the sum of voltage drops around the loop equals the applied voltage:

$$L\frac{d^2q}{dt^2} + R\frac{dq}{dt} + \frac{q}{C} = E(t),$$

where $q(t)$ is the charge on the capacitor and $I = dq/dt$ is the current. This is mathematically identical to the spring-mass equation $m\ddot{x} + \gamma\dot{x} + kx = F(t)$ under the correspondences $L \leftrightarrow m$, $R \leftrightarrow \gamma$, $1/C \leftrightarrow k$, $E \leftrightarrow F$.

## Free Oscillations ($E = 0$)

With no applied voltage, the equation $L\ddot{q} + R\dot{q} + q/C = 0$ has characteristic equation $Lr^2 + Rr + 1/C = 0$ with roots

$$r = \frac{-R \pm \sqrt{R^2 - 4L/C}}{2L}.$$

The three cases are:
- **Overdamped** ($R^2 > 4L/C$): charge decays without oscillation.
- **Critically damped** ($R^2 = 4L/C$): fastest non-oscillatory decay.
- **Underdamped** ($R^2 < 4L/C$): oscillating charge (LC oscillations with damping).

The underdamped natural frequency is $\omega_d = \sqrt{1/(LC) - R^2/(4L^2)}$, close to the LC resonant frequency $\omega_0 = 1/\sqrt{LC}$ for small $R$.

## Driven RLC Circuit

For $E(t) = E_0\cos(\omega t)$, the steady-state current $I = dq/dt$ oscillates at frequency $\omega$ with amplitude

$$I_0 = \frac{E_0}{|Z|}, \qquad Z = R + i\left(\omega L - \frac{1}{\omega C}\right) \quad \text{(complex impedance)}.$$

$|Z| = \sqrt{R^2 + (\omega L - 1/(\omega C))^2}$ is the **impedance magnitude**, the AC analog of resistance. Resonance occurs at $\omega = \omega_0 = 1/\sqrt{LC}$, where $\omega L = 1/(\omega C)$, the impedance is purely resistive ($|Z| = R$), and the current amplitude is maximized to $I_0 = E_0/R$.

At resonance, the inductive reactance $\omega L$ equals the capacitive reactance $1/(\omega C)$, and the phase shift between current and voltage is zero: current and voltage are in phase, maximizing power transfer.

## Power Dissipation

The power dissipated in the resistor is $P = I^2 R$. At resonance, $P_{\max} = E_0^2/(2R)$ (averaged over one cycle). At off-resonance frequencies, the reactive components (inductor and capacitor) store and return energy without net dissipation, but the real power to the resistor decreases. The **half-power bandwidth** is $\Delta\omega = R/L$, and the quality factor is $Q = \omega_0 L/R = 1/(R\sqrt{L/C})$.

## Applications

Radio tuners use variable capacitors to adjust $C$ and hence $\omega_0 = 1/\sqrt{LC}$, selecting the desired broadcast frequency by matching it with the resonant frequency of the LC circuit. Transformers exploit mutual inductance (not present in the simple model but analyzable by the same techniques). Filter circuits use the frequency-selective properties of RLC networks to pass certain frequencies and block others: low-pass, high-pass, and band-pass filters are all designed on this principle.

The mathematics of the RLC circuit is also the foundation for understanding electromagnetic cavities, quartz crystal oscillators (where the crystal's mechanical resonance is modeled as an RLC circuit), and quantum-mechanical two-level systems coupled to oscillating electromagnetic fields.
