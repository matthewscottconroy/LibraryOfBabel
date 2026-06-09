# Exercises — Chapter 15

The problems here span the chapter's two main themes: infinite-dimensional PDE dynamics (semigroup theory, global attractors, energy estimates) and stochastic dynamics (Itô's formula, the Ornstein-Uhlenbeck process, random attractors). Several require using Itô's formula to compute exact solutions of SDEs.

---

**Exercise 15.1.** Show that the heat semigroup $T(t)f = e^{t\Delta}f$ satisfies all properties of a $C_0$-semigroup on $L^2(\Omega)$. Compute its generator. What is the domain $D(\mathcal{A})$?

**Exercise 15.2.** (Navier-Stokes Attractor) The 2D Navier-Stokes semiflow satisfies $\frac{d}{dt}\|u\|^2 \leq -\nu\|\nabla u\|^2 + \|f\|^2/\nu$ (energy estimate). Use this to show the semiflow is dissipative (every orbit eventually enters a fixed bounded set).

**Exercise 15.3.** Use Itô's formula to compute $d(e^{-\alpha t} X_t^2)$ for the Ornstein-Uhlenbeck process $dX_t = -\alpha X_t\,dt + \sigma\,dW_t$. Conclude the formula for $E[X_t^2]$ and verify the stationary variance $\sigma^2/(2\alpha)$.

**Exercise 15.4.** (Stochastic Logistic) $dX_t = X_t(\mu - X_t)\,dt + \sigma X_t\,dW_t$. Use the substitution $Y_t = 1/X_t$ and Itô's formula to solve this SDE explicitly.

**Exercise 15.5.** Construct a random attractor for the random system $\dot{x} = -x + \xi(t)$ where $\xi$ is Ornstein-Uhlenbeck noise. The random attractor is a single (time-dependent) point $x^*(\omega)$. Find it.
