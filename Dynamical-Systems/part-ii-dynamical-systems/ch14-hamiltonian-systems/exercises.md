# Exercises — Chapter 14

The problems below develop the symplectic geometry, integrability theory, and KAM theory from the chapter. Several require both explicit computation (action-angle coordinates, standard map analysis) and conceptual arguments (why Liouville's theorem prevents attractors, what quantum ergodicity says and does not say).

---

**Exercise 14.1.** Verify that the standard symplectic form $\omega_0 = \sum_i dq_i \wedge dp_i$ is closed ($d\omega_0 = 0$) and nondegenerate. Write out Hamilton's equations for $H = |p|^2/2 + V(q)$.

**Exercise 14.2.** (Liouville-Arnold) For the 2D harmonic oscillator $H = (p_1^2 + q_1^2)/2 + (p_2^2 + q_2^2)/2$, find the action-angle coordinates $(I_1, I_2, \theta_1, \theta_2)$. Show $H = I_1 + I_2$.

**Exercise 14.3.** Compute the standard map $f(q, p) = (q + p + K\sin q,\ p + K\sin q) \pmod{2\pi}$ for $K = 0$ (integrable). Show that for small $K$, the circles $\{p = \text{const}\}$ are perturbed to invariant curves. Estimate the critical $K$ where the last KAM torus breaks down (it is approximately $K_c \approx 0.9716\ldots$).

**Exercise 14.4.** (Poincaré-Birkhoff) For a twist map with rotation number $p/q$ (rational), the Poincaré-Birkhoff theorem guarantees at least two periodic orbits of period $q$. Verify this for the standard map at $K = 0$ for the orbits with $p/q = 1/2$.

**Exercise 14.5.** For the pendulum $H = p^2/2 - \cos q$: (a) sketch the phase portrait; (b) find all equilibria; (c) identify the separatrix (the curve connecting the saddle to itself); (d) compute the period of libration orbits as a function of amplitude; (e) show the system is integrable. What happens when we add a small periodic perturbation $\varepsilon\sin(q - t)$?

**Exercise 14.6.** (Quantum Ergodicity) For the quantum harmonic oscillator $\hat{H} = -d^2/dx^2 + x^2$ on $\mathbb{R}$, the eigenfunctions are Hermite functions $\psi_n$. Does $|\psi_n|^2 \to$ Lebesgue measure? (The answer is no — but Shnirelman's theorem applies to compact manifolds with ergodic geodesic flow.)
