# Normal Modes of the Wave Equation

The general solution to the wave equation on $[0,L]$ is an infinite superposition of normal modes (standing waves). The theory of normal modes is not only the classical solution method for the wave equation on bounded domains but is also the basis for Fourier analysis, quantum mechanics (energy eigenstates), and the spectral theory of vibrating systems.

## The General Solution

By superposition of the normal mode solutions $u_n = \sin(n\pi x/L)[A_n\cos(\omega_n t) + B_n\sin(\omega_n t)]$:

$$u(x,t) = \sum_{n=1}^\infty\sin\!\left(\frac{n\pi x}{L}\right)\left[A_n\cos(\omega_n t) + B_n\sin(\omega_n t)\right], \qquad \omega_n = \frac{cn\pi}{L}. \tag{1}$$

Applying initial conditions $u(x,0) = \phi(x)$ and $u_t(x,0) = \psi(x)$:

$$\phi(x) = \sum_{n=1}^\infty A_n\sin\!\left(\frac{n\pi x}{L}\right) \implies A_n = \frac{2}{L}\int_0^L\phi(x)\sin\!\left(\frac{n\pi x}{L}\right)dx.$$

$$\psi(x) = \sum_{n=1}^\infty \omega_n B_n\sin\!\left(\frac{n\pi x}{L}\right) \implies B_n = \frac{2}{\omega_n L}\int_0^L\psi(x)\sin\!\left(\frac{n\pi x}{L}\right)dx = \frac{2}{cn\pi}\int_0^L\psi(x)\sin\!\left(\frac{n\pi x}{L}\right)dx.$$

**Consistency with d'Alembert:** The two representations (separation of variables and d'Alembert's formula) are equivalent. Rewriting the standing waves as traveling waves using $\cos(\omega_n t)\sin(n\pi x/L) = \frac{1}{2}[\sin(n\pi x/L + \omega_n t) + \sin(n\pi x/L - \omega_n t)]$ and summing recovers d'Alembert's formula (as a Fourier series rather than a closed form).

## Musical Interpretation

For a string instrument (guitar, violin, piano):
- The **fundamental mode** ($n=1$) has frequency $f_1 = c/(2L)$ and determines the pitch of the note.
- The **overtones** ($n = 2, 3, 4, \ldots$) are at frequencies $f_n = nf_1$ — integer multiples of the fundamental, the harmonic series.
- The **timbre** (quality of sound) is determined by the amplitudes $A_n$ of the overtones: a plucked string (large low-$n$ amplitudes, small high-$n$) sounds different from a bowed string or a struck string even at the same fundamental frequency.

A guitar string of length $L = 0.65$ m with wave speed $c = 400$ m/s has fundamental frequency $f_1 = 400/(2 \times 0.65) \approx 308$ Hz (roughly $D_4$). Pressing a fret shortens $L$, raising $f_1$.

## Convergence of the Series

**Theorem.** If $\phi \in C^2([0,L])$ with $\phi(0) = \phi(L) = 0$, $\phi''(0) = \phi''(L) = 0$, and $\psi \in C^1([0,L])$ with $\psi(0) = \psi(L) = 0$, then the series (1) converges absolutely and uniformly, and defines a $C^2$ solution.

The conditions on $\phi$ ensure the Fourier coefficients $A_n$ decay as $O(n^{-3})$, making the series $\sum n^2 |A_n|$ convergent (necessary for differentiating twice).

For less regular initial data, the series converges in weaker senses (in $L^2$, or in the sense of distributions), and the solution is understood as a weak solution.

## Parseval's Theorem and Energy

The total energy of the string at time $t$ is:

$$E = \frac{\rho}{2}\int_0^L\left(u_t^2 + c^2 u_x^2\right)dx = \frac{\rho L}{4}\sum_{n=1}^\infty\omega_n^2(A_n^2 + B_n^2).$$

The second equality uses Parseval's theorem and the fact that the kinetic and potential energies of each mode are in balance. The total energy is conserved (independent of $t$), as we showed from the wave equation directly. The energy in the $n$-th mode $E_n = \frac{\rho L}{4}\omega_n^2(A_n^2 + B_n^2)$ is separately conserved.

## Normal Modes in Higher Dimensions

On a rectangle $(0,a)\times(0,b)$ with Dirichlet conditions, the normal modes are:

$$u_{mn}(x,y,t) = \sin\!\left(\frac{m\pi x}{a}\right)\sin\!\left(\frac{n\pi y}{b}\right)\left[A_{mn}\cos(\omega_{mn}t) + B_{mn}\sin(\omega_{mn}t)\right],$$

with $\omega_{mn} = c\pi\sqrt{m^2/a^2 + n^2/b^2}$.

**Degeneracy:** If $a/b$ is rational, some frequencies $\omega_{mn}$ may coincide (degenerate modes). For the square ($a=b$): $\omega_{12} = \omega_{21} = c\pi\sqrt{5}/a$, and the two modes $\sin(\pi x/a)\sin(2\pi y/a)$ and $\sin(2\pi x/a)\sin(\pi y/a)$ have the same frequency. Any linear combination of degenerate modes is also a normal mode, leading to richer spatial patterns.

## Quantum Mechanics Connection

In quantum mechanics, the Schrödinger equation $i\hbar\psi_t = H\psi$ for an infinite square well potential (particle in a box of length $L$) is formally a wave equation after a rotation to imaginary time. The energy eigenstates are exactly the normal modes $\psi_n(x) = \sqrt{2/L}\sin(n\pi x/L)$, with energies $E_n = \hbar^2\pi^2n^2/(2mL^2)$. The general state evolves as $\psi(x,t) = \sum_n c_n e^{-iE_n t/\hbar}\psi_n(x)$, the exact analogue of the wave equation's normal mode expansion.
