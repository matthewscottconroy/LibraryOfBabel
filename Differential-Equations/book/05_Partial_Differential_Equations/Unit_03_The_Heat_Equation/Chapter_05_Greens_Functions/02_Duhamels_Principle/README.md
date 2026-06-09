# Duhamel's Principle

Duhamel's principle is the parabolic analogue of the variation of parameters formula for ODEs. It reduces the nonhomogeneous heat equation (with a source term $F(\mathbf{x},t)$) to a family of homogeneous problems, one for each instant $s$ at which the source acts. The total solution is obtained by superposing the responses to all these instantaneous sources.

## The Principle

Consider the nonhomogeneous heat equation:

$$u_t = \kappa\Delta u + F(\mathbf{x},t), \qquad \mathbf{x}\in\Omega,\; t > 0,$$
$$u = 0 \text{ on } \partial\Omega, \qquad u(\mathbf{x},0) = 0$$

(homogeneous initial condition; the contribution from nonhomogeneous initial data is handled separately by the homogeneous problem).

**Duhamel's principle** states that the solution is

$$u(\mathbf{x},t) = \int_0^t w(\mathbf{x},t-s;s)\,ds, \tag{1}$$

where $w(\mathbf{x},\tau;s)$ solves the homogeneous heat equation with initial data $F(\mathbf{x},s)$:

$$w_\tau = \kappa\Delta w, \qquad \mathbf{x}\in\Omega,\; \tau > 0,$$
$$w = 0 \text{ on }\partial\Omega, \qquad w(\mathbf{x},0;s) = F(\mathbf{x},s).$$

In words: at each time $s$, the source $F(\mathbf{x},s)$ deposits a "pulse" of heat into the system. This pulse subsequently evolves under the homogeneous heat equation for an elapsed time $\tau = t - s$. The total effect at time $t$ is the integral over all such pulses from $s=0$ to $s=t$.

## Verification

Write $w(\mathbf{x},\tau;s) = \int_\Omega G(\mathbf{x},\tau;\mathbf{y},0)F(\mathbf{y},s)\,d\mathbf{y}$, where $G$ is the homogeneous heat Green's function. Then:

$$u(\mathbf{x},t) = \int_0^t\!\!\int_\Omega G(\mathbf{x},t-s;\mathbf{y},0)F(\mathbf{y},s)\,d\mathbf{y}\,ds = \int_0^t\!\!\int_\Omega G(\mathbf{x},t;\mathbf{y},s)F(\mathbf{y},s)\,d\mathbf{y}\,ds,$$

which is the second term in the representation formula (2) of the previous section. The full solution (with nonhomogeneous initial data) is:

$$u(\mathbf{x},t) = \underbrace{\int_\Omega G(\mathbf{x},t;\mathbf{y},0)f(\mathbf{y})\,d\mathbf{y}}_{\text{initial data response}} + \underbrace{\int_0^t\!\!\int_\Omega G(\mathbf{x},t;\mathbf{y},s)F(\mathbf{y},s)\,d\mathbf{y}\,ds}_{\text{source response (Duhamel)}}.$$

**Differentiating (1) with respect to $t$:**

$$u_t = w(\mathbf{x},0;t) + \int_0^t w_\tau(\mathbf{x},t-s;s)\,ds = F(\mathbf{x},t) + \int_0^t \kappa\Delta w(\mathbf{x},t-s;s)\,ds = F + \kappa\Delta u,$$

confirming the PDE.

## Application: Sinusoidal Source

Consider $u_t = u_{xx} + \sin(x)\cos(t)$ on $[0,\pi]$ with $u(0,t) = u(\pi,t) = 0$ and $u(x,0) = 0$.

**Step 1:** Find $w(\mathbf{x},\tau;s) = F(\mathbf{x},s)\,S(\tau)$... More precisely, the source at time $s$ is $F(x,s) = \sin(x)\cos(s)$.

The homogeneous heat equation $w_\tau = w_{xx}$, $w(0,\tau) = w(\pi,\tau) = 0$, $w(x,0) = \sin(x)\cos(s)$ has solution:

$$w(x,\tau;s) = e^{-\tau}\sin(x)\cos(s)$$

(since $\sin(x)$ is the $n=1$ eigenfunction with eigenvalue $\lambda_1 = 1$).

**Step 2:** Apply Duhamel:

$$u(x,t) = \int_0^t e^{-(t-s)}\sin(x)\cos(s)\,ds = \sin(x)\int_0^t e^{-(t-s)}\cos(s)\,ds.$$

The integral: $\int_0^t e^{-(t-s)}\cos(s)\,ds = e^{-t}\int_0^t e^s\cos(s)\,ds$. Integrating by parts twice:

$$\int e^s\cos(s)\,ds = \frac{e^s(\cos s + \sin s)}{2} + C.$$

So $\int_0^t e^{-(t-s)}\cos(s)\,ds = \frac{\cos t + \sin t - e^{-t}}{2}$.

The solution: $u(x,t) = \frac{\sin(x)}{2}(\cos t + \sin t - e^{-t})$.

**Check:** $u_t = \frac{\sin(x)}{2}(-\sin t + \cos t + e^{-t})$, $u_{xx} = -\sin(x)\cdot\frac{\cos t + \sin t - e^{-t}}{2}$.

$u_{xx} + \sin(x)\cos(t) = -u + \sin(x)\cos(t) = \frac{\sin(x)}{2}(-\cos t - \sin t + e^{-t}) + \sin(x)\cos(t) = \frac{\sin(x)}{2}(\cos t - \sin t + e^{-t}) \neq u_t$...

Let me restate: $u_t - u_{xx} = \frac{\sin(x)}{2}[(-\sin t + \cos t + e^{-t}) - (-\cos t - \sin t + e^{-t})] = \frac{\sin(x)}{2}\cdot 2\cos(t) = \sin(x)\cos(t)$. Correct.

## Duhamel's Principle for the Wave Equation

The same idea applies to the wave equation. For $u_{tt} = c^2\Delta u + F(\mathbf{x},t)$ with zero initial data:

$$u(\mathbf{x},t) = \int_0^t w(\mathbf{x},t-s;s)\,ds,$$

where $w$ solves the homogeneous wave equation $w_{\tau\tau} = c^2\Delta w$ with $w(\mathbf{x},0;s) = 0$ and $w_\tau(\mathbf{x},0;s) = F(\mathbf{x},s)$. This is a direct analogue of Duhamel's principle and is derived identically.

## Generalization: Abstract ODE Form

Duhamel's principle has a clean abstract formulation. The heat equation can be written as the ODE $u' = Au + F$ in a Banach space $X$ (with $A = \kappa\Delta$ the generator), whose solution with $u(0) = 0$ is the variation of parameters formula:

$$u(t) = \int_0^t e^{A(t-s)}F(s)\,ds.$$

This is Duhamel's principle in operator form. It applies to any linear evolution equation $u' = Au + F$ where $A$ generates a $C_0$-semigroup, unifying the heat equation, wave equation, Schrödinger equation, and more within a single abstract framework.
