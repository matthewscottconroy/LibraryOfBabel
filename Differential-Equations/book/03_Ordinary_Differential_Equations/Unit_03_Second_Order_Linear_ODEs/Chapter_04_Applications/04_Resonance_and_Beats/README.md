# Resonance and Beats

When the driving frequency of a forced oscillator approaches the natural frequency, dramatically different phenomena arise depending on whether damping is present. With damping, the amplitude grows to a large but finite maximum (practical resonance). Without damping, the amplitude grows without bound (pure resonance). When two nearly equal frequencies combine, a slow amplitude modulation called **beats** emerges.

## Pure Resonance (Undamped, $\omega = \omega_0$)

For $m\ddot{x} + kx = F_0\cos(\omega_0 t)$ (no damping, driving frequency equals natural frequency), the forcing term $F_0\cos(\omega_0 t)$ is a homogeneous solution. The modification rule requires multiplying the trial by $t$: try $x_p = t(A\cos\omega_0 t + B\sin\omega_0 t)$.

Substituting:

$$x_p'' + \omega_0^2 x_p = -2A\omega_0\sin\omega_0 t + 2B\omega_0\cos\omega_0 t = F_0\cos\omega_0 t.$$

So $A = 0$ and $B = F_0/(2m\omega_0)$. The particular solution is

$$x_p = \frac{F_0}{2m\omega_0}t\sin(\omega_0 t).$$

The factor $t$ causes the amplitude to grow linearly in time: the oscillations build without bound. This is **pure resonance**, a physically catastrophic situation (the Tacoma Narrows Bridge is the canonical example).

The general solution $x = c_1\cos\omega_0 t + c_2\sin\omega_0 t + \frac{F_0}{2m\omega_0}t\sin\omega_0 t$ has amplitude growing as $(F_0/(2m\omega_0))t$ for large $t$.

## Beats (Undamped, $\omega \approx \omega_0$, $\omega \neq \omega_0$)

For $\ddot{x} + \omega_0^2 x = (F_0/m)\cos\omega t$ with $\omega \neq \omega_0$ and initial rest ($x(0) = 0$, $\dot{x}(0) = 0$), the solution is

$$x(t) = \frac{F_0/m}{\omega_0^2 - \omega^2}(\cos\omega t - \cos\omega_0 t).$$

Using the identity $\cos A - \cos B = 2\sin\left(\frac{A+B}{2}\right)\sin\left(\frac{B-A}{2}\right)$:

$$x(t) = \frac{2F_0/m}{\omega_0^2 - \omega^2}\sin\!\left(\frac{\omega_0 + \omega}{2}t\right)\sin\!\left(\frac{\omega_0 - \omega}{2}t\right).$$

This is a product of two sinusoids. The high-frequency factor $\sin\!\left(\frac{\omega_0+\omega}{2}t\right)$ oscillates at the average frequency $\bar{\omega} = (\omega_0+\omega)/2$. The low-frequency factor $\sin\!\left(\frac{\omega_0-\omega}{2}t\right)$ modulates the amplitude slowly, with period $T_{\text{beat}} = 2\pi/|\omega_0 - \omega|$. The result is a **beat**: oscillations at the average frequency, with amplitude rising and falling periodically.

As $\omega \to \omega_0$, $T_{\text{beat}} \to \infty$ and the maximum amplitude $2F_0/(m(\omega_0^2 - \omega^2)) \to \infty$: beats evolve continuously into pure resonance.

Beats are heard acoustically when two musical tones of nearly equal frequency are sounded simultaneously: the perceived pitch is the average frequency, and the perceived volume pulses at the beat frequency $|\omega_0 - \omega|/(2\pi)$ Hz.

## Practical Resonance with Damping

For the damped system, the steady-state amplitude $C = F_0/Z$ (from the previous section) attains its maximum at $\omega_{\max} = \omega_0\sqrt{1 - 2\zeta^2}$. As $\zeta \to 0$, the peak amplitude $C_{\max} \to \infty$ and $\omega_{\max} \to \omega_0$. For any $\zeta > 0$, however, $C_{\max}$ is finite.

The bandwidth of the resonance peak (the width at half-power amplitude $C_{\max}/\sqrt{2}$) is approximately $\Delta\omega = 2\zeta\omega_0 = \gamma/m$. A sharp resonance (large $Q = \omega_0/(2\delta)$, small $\zeta$) has narrow bandwidth; a broad resonance (small $Q$) responds over a wider frequency range. This is the engineering language of frequency selectivity.

## Physical Examples

Resonance in mechanical structures (buildings, aircraft wings, turbine blades) must be avoided or managed by damping. Resonance in electrical circuits (RLC circuits) is deliberately exploited for frequency selection in radio tuners. Magnetic resonance imaging (MRI) uses nuclear magnetic resonance: nuclei in a magnetic field precess at the Larmor frequency, and resonant radio waves are absorbed when the driving frequency matches this natural frequency. The mathematics is identical in all cases.
