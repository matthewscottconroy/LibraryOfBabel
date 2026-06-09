# Nonhomogeneous Wave Equation and Resonance

The nonhomogeneous wave equation $u_{tt} = c^2 u_{xx} + F(x,t)$ models a vibrating string subject to an external force. The method of eigenfunction expansion converts this PDE into infinitely many decoupled ODE problems (one for each mode), each of which is a driven harmonic oscillator. The most important physical phenomenon that emerges is **resonance**: when the driving frequency matches a natural frequency of the string, the amplitude grows without bound (in the absence of damping).

## Eigenfunction Expansion Method

Consider the problem on $[0,L]$ with Dirichlet conditions and zero initial data (for simplicity):

$$u_{tt} = c^2 u_{xx} + F(x,t), \quad u(0,t)=u(L,t)=0, \quad u(x,0)=0, \quad u_t(x,0)=0.$$

Expand both $u$ and $F$ in the eigenbasis $\{\sin(n\pi x/L)\}$:

$$u(x,t) = \sum_{n=1}^\infty T_n(t)\sin\!\left(\frac{n\pi x}{L}\right), \qquad F(x,t) = \sum_{n=1}^\infty f_n(t)\sin\!\left(\frac{n\pi x}{L}\right),$$

where $f_n(t) = \frac{2}{L}\int_0^L F(x,t)\sin(n\pi x/L)\,dx$.

Substituting into the PDE and using orthogonality, each mode satisfies:

$$T_n'' + \omega_n^2 T_n = f_n(t), \qquad T_n(0) = 0, \quad T_n'(0) = 0,$$

where $\omega_n = cn\pi/L$. This is a driven harmonic oscillator ODE.

## Solution by Variation of Parameters

The ODE $T_n'' + \omega_n^2 T_n = f_n(t)$ with zero initial conditions has solution:

$$T_n(t) = \frac{1}{\omega_n}\int_0^t f_n(s)\sin(\omega_n(t-s))\,ds.$$

This is Duhamel's principle for the harmonic oscillator. The total solution is:

$$u(x,t) = \sum_{n=1}^\infty\frac{1}{\omega_n}\left[\int_0^t f_n(s)\sin(\omega_n(t-s))\,ds\right]\sin\!\left(\frac{n\pi x}{L}\right).$$

## Resonance

Consider a periodic forcing $F(x,t) = g(x)\sin(\Omega t)$ with frequency $\Omega$. Then $f_n(t) = g_n\sin(\Omega t)$ where $g_n = \frac{2}{L}\int_0^L g(x)\sin(n\pi x/L)\,dx$.

The mode equation is $T_n'' + \omega_n^2 T_n = g_n\sin(\Omega t)$.

**Off-resonance ($\Omega \neq \omega_n$):** The particular solution is $g_n\sin(\Omega t)/(\omega_n^2 - \Omega^2)$ — a bounded oscillation with amplitude proportional to $1/|\omega_n^2 - \Omega^2|$.

**At resonance ($\Omega = \omega_N$ for some $N$):** The standard result for the driven harmonic oscillator at resonance gives:

$$T_N(t) = -\frac{g_N}{2\omega_N}t\cos(\omega_N t).$$

The amplitude grows linearly with time — **resonance**. The physical system absorbs energy from the forcing at exactly the rate the system's natural mode can accept, and the oscillation amplitude grows without bound.

In practice, any physical system has some damping (internal friction, air resistance), which limits the resonant amplitude to $g_N/(2\omega_N\beta)$ where $\beta$ is the damping coefficient. But near resonance (small damping), the amplitude can be enormous.

## Physical Examples of Resonance

**Tacoma Narrows Bridge (1940):** The bridge oscillated in a torsional mode driven by vortex shedding from the wind at a frequency near the bridge's natural torsional frequency. The resonant buildup of amplitude led to the bridge's catastrophic collapse.

**Mechanical resonance in engines:** Rotating machinery can excite structural resonances at frequencies equal to multiples of the rotation frequency. Engineers design to avoid these resonances or add damping.

**Acoustic resonance:** A loudspeaker driving a room at the room's resonant frequency (standing wave frequency) produces large-amplitude pressure oscillations — the room "rings."

**MRI (Magnetic Resonance Imaging):** Radio-frequency pulses at the Larmor frequency of protons (determined by the magnetic field) resonantly excite the nuclear spin, which then emits detectable signals.

## Duhamel's Principle for the Wave Equation

The general nonhomogeneous problem with nonzero initial data $u(x,0)=\phi(x)$, $u_t(x,0)=\psi(x)$ separates into:

1. The homogeneous wave equation with initial data $(\phi,\psi)$ — solved by normal mode expansion or d'Alembert's formula.
2. The nonhomogeneous equation with zero initial data — solved by the Duhamel integral above.

The total solution is the sum. This decomposition is the wave equation version of Duhamel's principle:

$$u(\mathbf{x},t) = \underbrace{u_{\text{hom}}(\mathbf{x},t)}_{\text{initial data}} + \underbrace{\int_0^t w(\mathbf{x},t-s;s)\,ds}_{\text{forcing}},$$

where $w(\mathbf{x},\tau;s)$ solves the homogeneous wave equation with $w(\mathbf{x},0;s)=0$, $w_t(\mathbf{x},0;s)=F(\mathbf{x},s)$.
