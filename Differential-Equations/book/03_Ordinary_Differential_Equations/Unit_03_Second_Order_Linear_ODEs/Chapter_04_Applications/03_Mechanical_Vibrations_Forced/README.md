# Mechanical Vibrations: Forced Motion

Adding a periodic external force $F(t) = F_0\cos(\omega t)$ to the damped spring-mass system gives the **forced damped oscillator**:

$$m\ddot{x} + \gamma\dot{x} + kx = F_0\cos(\omega t).$$

The general solution $x = x_h + x_p$ consists of the transient $x_h$ (which decays when $\gamma > 0$) and the steady-state particular solution $x_p$.

## The Steady-State Particular Solution

Try $x_p = A\cos\omega t + B\sin\omega t$. Substituting:

$$(-m\omega^2 A + \gamma\omega B + kA)\cos\omega t + (-m\omega^2 B - \gamma\omega A + kB)\sin\omega t = F_0\cos\omega t.$$

Matching coefficients:

$$(k - m\omega^2)A + \gamma\omega B = F_0, \qquad -\gamma\omega A + (k - m\omega^2)B = 0.$$

Let $Z = \sqrt{(k - m\omega^2)^2 + \gamma^2\omega^2}$ (the **mechanical impedance**). Solving:

$$A = \frac{(k - m\omega^2)F_0}{Z^2}, \qquad B = \frac{\gamma\omega F_0}{Z^2}.$$

The amplitude of the steady-state response is

$$C = \sqrt{A^2 + B^2} = \frac{F_0}{Z} = \frac{F_0}{\sqrt{(k - m\omega^2)^2 + \gamma^2\omega^2}}.$$

Writing $\omega_0^2 = k/m$:

$$C = \frac{F_0/k}{\sqrt{(1 - (\omega/\omega_0)^2)^2 + (2\delta\omega/\omega_0^2)^2}} = \frac{F_0/k}{\sqrt{(1 - r^2)^2 + (2\zeta r)^2}},$$

where $r = \omega/\omega_0$ is the frequency ratio and $\zeta = \gamma/(2m\omega_0)$ is the damping ratio.

## The Frequency Response Function

The amplitude $C$ as a function of driving frequency $\omega$ is the **frequency response**. For fixed damping $\gamma > 0$, $C$ attains a maximum at the **resonant frequency**

$$\omega_{\max} = \sqrt{\omega_0^2 - 2\delta^2} = \omega_0\sqrt{1 - 2\zeta^2},$$

provided $\zeta < 1/\sqrt{2}$. At this frequency, the response amplitude is

$$C_{\max} = \frac{F_0/k}{2\zeta\sqrt{1 - \zeta^2}}.$$

For small damping ($\zeta \to 0$), $C_{\max} \to \infty$ as $\omega_{\max} \to \omega_0$: this is the resonance phenomenon.

## Phase Angle

The steady-state response lags the forcing by a phase angle $\phi$:

$$x_p = C\cos(\omega t - \phi), \qquad \tan\phi = \frac{\gamma\omega}{k - m\omega^2} = \frac{2\zeta r}{1 - r^2}.$$

Below resonance ($\omega < \omega_0$), $\phi < \pi/2$: the response is nearly in phase with the forcing. Above resonance, $\phi > \pi/2$: the response is nearly out of phase. At resonance ($\omega = \omega_0$), $\phi = \pi/2$ exactly: the response is 90 degrees out of phase with the forcing. This phase shift is measurable and provides a way to detect resonance experimentally.

## Long-Term Behavior

For $\gamma > 0$, the transient $x_h$ decays exponentially and the solution approaches the steady-state $x_p$. After a time of order $1/\delta = 2m/\gamma$, the transient is negligible and the system oscillates at the driving frequency $\omega$ with amplitude $C$ and phase shift $\phi$. The initial conditions affect only the transient, not the long-term steady state.
