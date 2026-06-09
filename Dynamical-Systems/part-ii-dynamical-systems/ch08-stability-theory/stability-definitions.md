# 8.1 Stability Definitions

Before we can prove anything about stability, we need precise definitions. The word "stable" has at least four distinct meanings in dynamics, and confusing them leads to real errors — in proofs, in engineering designs, in numerical experiments. Let's be careful.

Throughout this section, we consider the equilibrium $x^* = 0$ of either a continuous-time system $\dot{x} = f(x)$ or a discrete map $x \mapsto f(x)$. The origin is an equilibrium, meaning $f(0) = 0$ (in continuous time) or $f(0) = 0$ (in discrete time). We want to characterize the behavior of solutions starting near 0.

**Definition 8.1.1.** The equilibrium $x^* = 0$ of $\dot{x} = f(x)$ (or map $x \mapsto f(x)$) is:
- *Lyapunov stable*: $\forall \varepsilon > 0\ \exists \delta > 0$: $\|x(0)\| < \delta \Rightarrow \|x(t)\| < \varepsilon$ for all $t \geq 0$
- *Asymptotically stable*: Lyapunov stable and $x(t) \to 0$ as $t \to \infty$
- *Exponentially stable*: $\exists C, \lambda > 0$: $\|x(t)\| \leq C e^{-\lambda t} \|x(0)\|$ for all $t \geq 0$
- *Unstable*: not Lyapunov stable
- *Globally asymptotically stable (GAS)*: asymptotically stable with basin of attraction = whole space

Let's read these carefully. Lyapunov stability is the basic "you don't escape" condition: start close enough and you stay within any prescribed radius. Asymptotic stability adds that you actually *converge* to the equilibrium. Exponential stability gives a quantitative rate for that convergence — a uniform exponential bound that works for all initial conditions in a neighborhood, with the same constants $C$ and $\lambda$.

The distinctions are real. Consider a center (a Hamiltonian equilibrium with purely imaginary eigenvalues): the solutions orbit around the equilibrium forever without approaching it. This is Lyapunov stable but not asymptotically stable. Or consider the system $\dot{x} = -x^3$: the origin is GAS, but solutions decay like $t^{-1/2}$, not like $e^{-\lambda t}$. So it's asymptotically but not exponentially stable.

Global asymptotic stability (GAS) is the strongest condition: the equilibrium is the unique attractor for the whole space. In control theory, GAS is often the design target. In applied dynamics, GAS is what you hope for when you want a system to "forget" its initial condition.

The definitions in hand, the question is: how do you determine which case you're in? That's what Lyapunov's direct method answers, without solving the equations.
