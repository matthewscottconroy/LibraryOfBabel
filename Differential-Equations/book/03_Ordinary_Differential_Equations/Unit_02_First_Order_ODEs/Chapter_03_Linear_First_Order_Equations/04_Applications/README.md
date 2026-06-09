# Applications of Linear First-Order Equations

The linear first-order equation $y' + p(x)y = q(x)$ models a remarkable variety of physical and biological phenomena. In each application, the term $p(x)y$ represents a loss or damping proportional to $y$, and $q(x)$ represents an external input or forcing. Understanding this structure clarifies both the mathematics and the physics.

## Mixing Problems

A tank contains $V$ liters of brine (salt solution) with initial salt content $Q_0$ grams. Brine with concentration $c_{\text{in}}$ g/L flows in at rate $r_{\text{in}}$ L/min, and well-mixed solution flows out at rate $r_{\text{out}}$ L/min. The volume $V(t) = V_0 + (r_{\text{in}} - r_{\text{out}})t$ changes if the rates differ.

The salt content $Q(t)$ satisfies:

$$\frac{dQ}{dt} = r_{\text{in}}\,c_{\text{in}} - r_{\text{out}}\frac{Q}{V(t)}.$$

This is linear: $Q' + \frac{r_{\text{out}}}{V(t)}Q = r_{\text{in}}\,c_{\text{in}}$, with $p(t) = r_{\text{out}}/V(t)$ and $q(t) = r_{\text{in}}\,c_{\text{in}}$.

**Example.** A 500 L tank initially contains pure water. Brine at 2 g/L flows in at 5 L/min, and the well-mixed solution flows out at 5 L/min (so volume stays constant). Find $Q(t)$.

Here $V = 500$, $r_{\text{in}} = r_{\text{out}} = 5$, $c_{\text{in}} = 2$. The equation is $Q' + Q/100 = 10$. Integrating factor: $\mu = e^{t/100}$. Then $(e^{t/100}Q)' = 10e^{t/100}$, so $e^{t/100}Q = 1000e^{t/100} + C$, giving $Q(t) = 1000 + Ce^{-t/100}$. With $Q(0) = 0$: $C = -1000$. Thus

$$Q(t) = 1000\left(1 - e^{-t/100}\right).$$

As $t \to \infty$, $Q \to 1000$ g = $V \cdot c_{\text{in}} = 500 \cdot 2$, the steady-state salt content.

## Electrical Circuits: RC Circuit

A series circuit consists of a resistor $R$ (ohms), a capacitor $C$ (farads), and a voltage source $E(t)$ (volts). By Kirchhoff's voltage law, the charge $q(t)$ on the capacitor satisfies

$$R\frac{dq}{dt} + \frac{q}{C} = E(t),$$

or in terms of current $I = dq/dt$:

$$R\frac{dI}{dt} + \frac{I}{C} = E'(t).$$

The charge equation $q' + q/(RC) = E(t)/R$ is linear with $p = 1/(RC)$ (the reciprocal of the time constant $\tau = RC$). With $E(t) = E_0$ (constant):

$$q(t) = CE_0 + (q_0 - CE_0)e^{-t/(RC)}.$$

The steady-state charge is $CE_0$, and the transient decays with time constant $RC$. For an AC source $E(t) = E_0\sin(\omega t)$:

$$q(t) = \frac{E_0/R}{\sqrt{(1/(RC))^2 + \omega^2}}\sin(\omega t - \phi) + Ce^{-t/(RC)},$$

where $\tan\phi = \omega RC$. The transient $Ce^{-t/(RC)}$ decays, leaving the sinusoidal steady state.

## Population with Immigration

A population $P(t)$ grows logistically but also receives immigrants at a constant rate $I$:

$$\frac{dP}{dt} = rP\left(1 - \frac{P}{K}\right) + I.$$

This is nonlinear. But if immigration is proportional to the population deficit ($K - P$), the model $dP/dt = r(K - P) + I$ becomes linear: $P' + rP = rK + I$, giving

$$P(t) = K + \frac{I}{r} + \left(P_0 - K - \frac{I}{r}\right)e^{-rt}.$$

All solutions converge to $K + I/r$: immigration raises the effective equilibrium above $K$.

## RL Circuit

A series RL circuit (resistor $R$, inductor $L$, voltage source $E(t)$) satisfies

$$L\frac{dI}{dt} + RI = E(t), \qquad I' + \frac{R}{L}I = \frac{E(t)}{L}.$$

The integrating factor is $\mu = e^{Rt/L}$. For $E(t) = E_0$ (DC):

$$I(t) = \frac{E_0}{R} + \left(I_0 - \frac{E_0}{R}\right)e^{-Rt/L}.$$

The steady-state current is $E_0/R$ (Ohm's law for DC); the transient $e^{-Rt/L}$ decays with time constant $L/R$. A large inductance $L$ slows the approach to steady state; a large resistance $R$ speeds it.

## The Principle Behind All These Applications

In each case, the differential equation expresses a balance: $y' = \text{input} - \text{output}$, where the output is proportional to $y$ (proportional loss or removal). The particular solution tracks the forcing; the homogeneous solution describes how perturbations decay. The time constant $\tau = 1/p$ (when $p$ is constant) measures the relaxation scale.

This balance structure, formalized as the linear equation $y' + py = q$, is one of the most important modeling templates in applied mathematics and engineering.
