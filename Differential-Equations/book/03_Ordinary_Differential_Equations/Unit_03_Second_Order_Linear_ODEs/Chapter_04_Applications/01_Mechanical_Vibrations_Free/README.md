# Mechanical Vibrations: Free Undamped Motion

The simplest vibration model considers a mass $m > 0$ attached to a spring with spring constant $k > 0$, sliding on a frictionless surface. The only force is the restoring force of the spring, which by Hooke's law is $-kx$ (opposing displacement $x$ from equilibrium). Newton's second law gives

$$m\ddot{x} + kx = 0,$$

or dividing by $m$: $\ddot{x} + \omega_0^2 x = 0$ where $\omega_0 = \sqrt{k/m}$ is the **natural angular frequency** (radians per second).

## Solution

The characteristic equation is $r^2 + \omega_0^2 = 0$, with roots $r = \pm\omega_0 i$ (pure imaginary). The general solution is

$$x(t) = c_1\cos(\omega_0 t) + c_2\sin(\omega_0 t).$$

In amplitude-phase form: $x(t) = A\cos(\omega_0 t - \phi)$ where $A = \sqrt{c_1^2 + c_2^2}$ is the **amplitude** and $\phi$ is the **phase angle**. The motion is purely sinusoidal with period $T = 2\pi/\omega_0$.

## Initial Conditions and Amplitude

For the IVP $x(0) = x_0$ (initial displacement) and $\dot{x}(0) = v_0$ (initial velocity):

$$c_1 = x_0, \quad c_2 = v_0/\omega_0, \quad A = \sqrt{x_0^2 + v_0^2/\omega_0^2}.$$

The amplitude $A$ is the maximum displacement from equilibrium. It depends on both the initial displacement and the initial velocity: all the initial energy is converted between kinetic and potential as the mass oscillates, and $A$ determines the total mechanical energy $E = \frac{1}{2}kA^2$.

**Example.** A 0.5 kg mass on a spring with $k = 8$ N/m is displaced 0.1 m from equilibrium and released from rest. Find the period, frequency, and amplitude.

$\omega_0 = \sqrt{8/0.5} = 4$ rad/s. Period $T = 2\pi/4 = \pi/2 \approx 1.57$ s. Frequency $f = \omega_0/(2\pi) = 2/\pi \approx 0.64$ Hz. Amplitude $A = x_0 = 0.1$ m (since $v_0 = 0$). Solution: $x(t) = 0.1\cos(4t)$.

## Energy Conservation

The potential energy stored in the spring is $V = \frac{1}{2}kx^2 = \frac{1}{2}kA^2\cos^2(\omega_0 t - \phi)$, and the kinetic energy is $K = \frac{1}{2}m\dot{x}^2 = \frac{1}{2}mA^2\omega_0^2\sin^2(\omega_0 t - \phi) = \frac{1}{2}kA^2\sin^2(\omega_0 t - \phi)$ (using $m\omega_0^2 = k$). The total energy $E = V + K = \frac{1}{2}kA^2$ is constant, confirming conservation of energy. The motion is the continual exchange between potential and kinetic energy.

## Isochronism

The period $T = 2\pi/\omega_0 = 2\pi\sqrt{m/k}$ is independent of the amplitude $A$. This is **isochronism**: no matter how large or small the initial displacement, the mass completes each oscillation in the same time. Isochronism is a consequence of linearity (the restoring force is exactly $-kx$, linear in $x$) and breaks down for the nonlinear pendulum, where the period depends on amplitude.

## Comparison with the Pendulum

The linearized pendulum $\ddot{\theta} + (g/\ell)\theta = 0$ has the same form with $\omega_0 = \sqrt{g/\ell}$. For small oscillations, the period is $T = 2\pi\sqrt{\ell/g}$, independent of amplitude (isochronism). For large oscillations, the nonlinear term $\sin\theta - \theta$ becomes significant, breaking isochronism and increasing the period.
