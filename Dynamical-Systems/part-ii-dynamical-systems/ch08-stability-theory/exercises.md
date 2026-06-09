# Exercises — Chapter 8

These exercises develop both the analytical and computational aspects of stability theory. Several require finding explicit Lyapunov functions — a skill that is as much art as science.

---

**Exercise 8.1.** Find a Lyapunov function for the system $\dot{x}_1 = -x_1 + x_2^2$, $\dot{x}_2 = -x_2 - x_1 x_2$. Show the origin is GAS.

**Exercise 8.2.** (LaSalle) Consider $\dot{x}_1 = x_2$, $\dot{x}_2 = -\sin(x_1) - x_2$ (damped pendulum). Take $V = 1 - \cos(x_1) + x_2^2/2$. Apply LaSalle's theorem to show all bounded solutions converge to an equilibrium.

**Exercise 8.3.** For the system $\dot{x} = -x^3 + x^5$: (a) Find the equilibria. (b) Is the origin stable? GAS? Compute the Lyapunov function $V = x^2/2$ and $\dot{V}$. For which initial conditions does the solution diverge?

**Exercise 8.4.** (Floquet) The Mathieu equation $\ddot{x} + (a + b\cos t) x = 0$ is a periodic linear ODE with period $\pi$. For $a = 1$, $b = 0$: find the Floquet multipliers explicitly. For small $b$, describe what happens to stability.

**Exercise 8.5.** (Lyapunov Exponents) For the tent map $T(x) = 1 - |2x-1|$: compute $|DT(x)| = 2$ almost everywhere. Use Birkhoff's theorem to show the Lyapunov exponent of Lebesgue-a.e. orbit is $\log 2$.

**Exercise 8.6.** Prove Pesin's formula for linear toral automorphisms: $h_\mu(f_A) = \sum_{\lambda > 1} \log \lambda$ where $\lambda$ ranges over eigenvalues of $A$ with $|\lambda| > 1$. (*Hint:* Use the Bernoulli generator given by a Markov partition.)

**Exercise 8.7.** (Research) For the Collatz map on ${\mathbb Z}_2$: the map $T$ is piecewise linear. Compute its local expansion rates. What would it mean for the "Lyapunov exponent" of the Collatz map to be positive?
