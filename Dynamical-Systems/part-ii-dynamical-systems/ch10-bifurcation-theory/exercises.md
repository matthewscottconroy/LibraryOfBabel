# Exercises — Chapter 10

These exercises develop the computational and conceptual skills of bifurcation theory. Several require drawing bifurcation diagrams — do not skip the pictures.

---

**Exercise 10.1.** Classify the equilibria of $\dot{x} = \mu + x^2 - x^3$ for all $\mu$. Find all bifurcation values and classify each bifurcation type.

**Exercise 10.2.** (Hopf) For the system $\dot{x}_1 = \mu x_1 - x_2 - x_1(x_1^2 + x_2^2)$, $\dot{x}_2 = x_1 + \mu x_2 - x_2(x_1^2 + x_2^2)$: show this undergoes a Hopf bifurcation at $\mu = 0$. Find the amplitude and period of the bifurcating limit cycle.

**Exercise 10.3.** Compute the normal form of $\dot{x} = y$, $\dot{y} = -x + x^3 + \mu y$ near the origin at $\mu = 0$. Classify the bifurcation.

**Exercise 10.4.** (Period-Doubling) For the logistic map $f_\mu(x) = \mu x(1-x)$: find the fixed points and determine their stability for all $\mu > 0$. Find the period-doubling bifurcation value $\mu_1$ where the fixed point loses stability (solve $|f'_\mu(x^*)| = 1$).

**Exercise 10.5.** (Feigenbaum) Given $\mu_n$ (the $n$-th period-doubling bifurcation value of the logistic map), verify numerically that $(\mu_n - \mu_{n-1})/(\mu_{n+1} - \mu_n) \approx 4.669$.

**Exercise 10.6.** (Shilnikov) Describe the qualitative dynamics near a homoclinic orbit in 3D when: (a) $\rho > \lambda$ (Shilnikov condition fails); (b) $\rho < \lambda$ (Shilnikov condition holds). What is the entropy in each case?

**Exercise 10.7.** (Cusp Catastrophe) The cusp catastrophe is given by $V(x, \mu_1, \mu_2) = x^4/4 + \mu_2 x^2/2 + \mu_1 x$. (a) Find the equilibrium surface $\{V_x = 0\}$ in $(x, \mu_1, \mu_2)$-space. (b) Find the bifurcation set (the "cusp" curve in $(\mu_1, \mu_2)$-space). (c) Describe the hysteresis loop as $\mu_1$ varies for fixed $\mu_2 < 0$.
