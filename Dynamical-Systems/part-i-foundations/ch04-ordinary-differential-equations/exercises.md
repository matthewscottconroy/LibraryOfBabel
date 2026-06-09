# Chapter 4 — Exercises

These exercises develop the ODE theory through computation and proof. Several of them are standard results that illuminate the theory; Exercise 4.8 connects to the fundamental distinction between rational and irrational dynamics.

---

**Exercise 4.1.** Use Picard iteration to find the solution of $\dot{x} = x^2$, $x(0) = 1$. Show the solution blows up in finite time. What does this illustrate about maximal solutions?

**Exercise 4.2.** Classify all equilibria of the harmonic oscillator $\ddot{x} + x = 0$ (written as a planar system). Compute the matrix exponential $e^{tA}$ and draw the phase portrait.

**Exercise 4.3.** For $\dot{x} = -x + x^3$: (a) find all equilibria; (b) classify them (using the Jacobian); (c) draw the phase portrait on $\mathbb{R}$; (d) find the stable and unstable manifolds explicitly.

**Exercise 4.4.** The van der Pol oscillator: $\dot{x} = y$, $\dot{y} = \mu(1-x^2)y - x$ for $\mu > 0$.
(a) Show that the origin is an unstable equilibrium.
(b) Argue (without computing explicitly) that there must be a limit cycle. (*Hint:* Use Poincaré-Bendixson and construct an annular trapping region.)

**Exercise 4.5.** (Variational Equations) For $\dot{x} = f(x)$ with flow $\Phi_t$, prove that $J(t) = D_{x_0}\Phi_t$ satisfies $\dot{J} = Df(\Phi_t(x_0)) J$ with $J(0) = I$. Compute $\det(J(t))$ using the formula $\frac{d}{dt}\det(J) = \text{tr}(Df) \cdot \det(J)$.

**Exercise 4.6.** Show that a 2D Hamiltonian system $\dot{q} = \partial H/\partial p$, $\dot{p} = -\partial H/\partial q$ cannot have asymptotically stable equilibria. (*Hint:* Use Liouville's theorem: the flow preserves area, so volumes cannot contract.)

**Exercise 4.7.** (Center Manifold) Consider $\dot{x} = xy$, $\dot{y} = -y + x^2$ near the origin. The linearization at $(0,0)$ has eigenvalues $0$ and $-1$. The center manifold has the form $y = h(x)$ for small $x$. Find $h(x)$ to second order by substituting into the invariance equation.

**Exercise 4.8.** Let $f: \mathbb{T}^2 \to \mathbb{T}^2$ be the flow of $\dot{\theta}_1 = 1$, $\dot{\theta}_2 = \alpha$. Show every orbit is dense iff $\alpha \in \mathbb{R} \setminus \mathbb{Q}$. If $\alpha \in \mathbb{Q}$, show every orbit is periodic.
