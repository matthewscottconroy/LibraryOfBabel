# Chapter 2: Separation of Variables for the Wave Equation

The method of separation of variables applied to the wave equation on a bounded interval gives the classical theory of standing waves and normal modes. This theory underlies the mathematical description of musical instruments, electromagnetic resonators, and quantum mechanical bound states. The eigenfunctions of the spatial operator determine the harmonic frequencies of the system, and the general solution is an infinite superposition of these harmonics.

## Standing Waves and Normal Modes

A **normal mode** (or standing wave) is a solution of the wave equation that has a definite spatial shape that oscillates harmonically in time:

$$u_n(x,t) = X_n(x)\left(A_n\cos(\omega_n t) + B_n\sin(\omega_n t)\right),$$

where $\omega_n = c\lambda_n$ is the angular frequency and $\lambda_n$ is the $n$-th eigenvalue of $-d^2/dx^2$ on $[0,L]$ (with Dirichlet conditions: $\lambda_n = n\pi/L$, so $\omega_n = cn\pi/L$).

Normal modes are not traveling waves — they do not propagate. Instead, different parts of the string oscillate in phase (all parts reach maximum displacement at the same time) but with amplitudes determined by the eigenfunction $X_n(x)$.

## Structure of This Chapter

**Section 1: Standing Waves** derives the normal mode solutions by separation of variables on $[0,L]$. The spatial equation is $X'' + \lambda^2 X = 0$, $X(0)=X(L)=0$, with eigenfunctions $\sin(n\pi x/L)$ and eigenvalues $\lambda_n = n\pi/L$. The temporal equation is $T'' + \omega_n^2 T = 0$ with $\omega_n = cn\pi/L$, giving oscillatory solutions $A_n\cos(\omega_n t) + B_n\sin(\omega_n t)$.

**Section 2: Normal Modes** develops the physical theory of modes. The fundamental frequency is $f_1 = c/(2L)$; the harmonics are $f_n = n\cdot c/(2L)$. The general solution is an infinite superposition of modes, with coefficients determined by the initial conditions:

$$u(x,t) = \sum_{n=1}^\infty\sin\!\left(\frac{n\pi x}{L}\right)\left(A_n\cos\!\left(\frac{cn\pi t}{L}\right) + B_n\sin\!\left(\frac{cn\pi t}{L}\right)\right).$$

The coefficients $A_n = \frac{2}{L}\int_0^L\phi(x)\sin(n\pi x/L)\,dx$ and $B_n = \frac{2}{cn\pi}\int_0^L\psi(x)\sin(n\pi x/L)\,dx$ are determined by initial displacement $\phi$ and initial velocity $\psi$.

**Section 3: Nonhomogeneous Problems** treats forced oscillations — the wave equation with a source term $F(x,t)$ — via the eigenfunction expansion method. The crucial phenomenon of **resonance** occurs when the forcing frequency matches a natural frequency $\omega_n$. At resonance, the amplitude grows linearly with time (for undamped oscillations), which leads to physically unrealistic predictions and must be controlled by damping in real systems.

## Key Physics: The Harmonic Series

For a string of length $L$ with wave speed $c$, the natural frequencies are

$$f_n = \frac{nc}{2L}, \qquad n = 1, 2, 3, \ldots$$

The fundamental $f_1 = c/(2L)$ is the lowest frequency; the overtones $f_2 = 2f_1$, $f_3 = 3f_1$, etc., are integer multiples of the fundamental. This integer relationship (the harmonic series) is the mathematical explanation of why musical instruments produce pleasing sounds — the overtones are in simple harmonic ratios with the fundamental, producing consonant intervals (octave = $2:1$, perfect fifth $= 3:2$, etc.).
