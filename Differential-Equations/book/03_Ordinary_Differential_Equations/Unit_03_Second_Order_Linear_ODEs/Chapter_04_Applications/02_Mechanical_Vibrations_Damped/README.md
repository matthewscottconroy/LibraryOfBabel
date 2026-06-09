# Mechanical Vibrations: Damped Free Motion

Adding a damping force $-\gamma\dot{x}$ (proportional to velocity, opposing motion) gives the **damped spring-mass equation**

$$m\ddot{x} + \gamma\dot{x} + kx = 0, \qquad m, \gamma, k > 0.$$

The characteristic equation $mr^2 + \gamma r + k = 0$ has roots

$$r = \frac{-\gamma \pm \sqrt{\gamma^2 - 4mk}}{2m}.$$

The discriminant $\Delta = \gamma^2 - 4mk$ determines the qualitative character of the motion.

## Overdamped Case ($\gamma^2 > 4mk$)

Both roots $r_1, r_2$ are real and negative (since $\sqrt{\gamma^2 - 4mk} < \gamma$). The general solution is

$$x(t) = c_1 e^{r_1 t} + c_2 e^{r_2 t}.$$

The mass returns to equilibrium exponentially without oscillating. With both $r_1 < r_2 < 0$, the slower exponential $e^{r_2 t}$ (with smaller magnitude of exponent) dominates for large $t$. The mass can overshoot equilibrium at most once (depending on initial conditions) but thereafter returns monotonically.

## Critically Damped Case ($\gamma^2 = 4mk$)

The repeated root is $r = -\gamma/(2m)$, and the general solution is

$$x(t) = (c_1 + c_2 t)e^{-\gamma t/(2m)}.$$

The system returns to equilibrium fastest among all non-oscillatory cases. If the initial conditions allow a zero crossing ($c_1/c_2 > 0$, so the zero $t^* = -c_1/c_2 > 0$), the mass passes through equilibrium once and returns. Critical damping is $\gamma_c = 2\sqrt{mk}$ and the corresponding time constant is $\tau = 2m/\gamma_c = \sqrt{m/k} = 1/\omega_0$.

## Underdamped Case ($\gamma^2 < 4mk$)

The roots are complex conjugates $r = -\delta \pm \omega_d i$ where

$$\delta = \frac{\gamma}{2m} > 0 \quad \text{(decay rate)}, \qquad \omega_d = \frac{\sqrt{4mk - \gamma^2}}{2m} = \sqrt{\omega_0^2 - \delta^2} > 0 \quad \text{(damped frequency)}.$$

The general solution is

$$x(t) = e^{-\delta t}(c_1\cos\omega_d t + c_2\sin\omega_d t) = Ae^{-\delta t}\cos(\omega_d t - \phi).$$

The motion is an oscillation at the damped frequency $\omega_d < \omega_0$ (lower than the natural frequency), with amplitude envelope $\pm Ae^{-\delta t}$ decaying exponentially. The ratio $Q = \omega_0/(2\delta) = \omega_0 m/\gamma$ is the **quality factor**: a large $Q$ means light damping and many oscillations before decay; a small $Q$ means heavy damping.

## Logarithmic Decrement

In the underdamped case, successive peaks of $x(t)$ occur at times $t_n = (\phi + n\pi)/\omega_d$... (with period $T_d = 2\pi/\omega_d$). The ratio of successive peak amplitudes is $e^{-\delta T_d}$, and the **logarithmic decrement** is

$$\Lambda = \ln\frac{|x(t_n)|}{|x(t_{n+1})|} = \delta T_d = \frac{\pi\gamma}{m\omega_d} = \frac{2\pi\delta}{\omega_d}.$$

Measuring $\Lambda$ from experimental data gives $\delta$ and hence $\gamma$, allowing the damping coefficient to be determined without measuring velocity.

## Energy Dissipation

For the underdamped case, the energy $E(t) = \frac{1}{2}m\dot{x}^2 + \frac{1}{2}kx^2$ satisfies $\dot{E} = -\gamma\dot{x}^2 \leq 0$: energy decreases at a rate proportional to the square of velocity, dissipated as heat by the dashpot. On average over one period, $\langle\dot{x}^2\rangle = \frac{1}{2}\omega_d^2 A^2 e^{-2\delta t}$, so the average energy decays as $E(t) \approx \frac{1}{2}kA^2 e^{-2\delta t}$, consistent with the amplitude envelope $Ae^{-\delta t}$.
