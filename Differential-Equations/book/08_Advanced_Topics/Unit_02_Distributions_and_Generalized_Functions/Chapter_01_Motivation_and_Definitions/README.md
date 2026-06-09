# Chapter 1: Motivation and Definitions

The theory of distributions arises from a concrete problem: many physically natural operations—impulse forces, point charges, surface charges, derivatives of discontinuous functions—cannot be modeled by ordinary functions but are nonetheless mathematically meaningful. This chapter develops the motivation through the Dirac delta function and then builds the precise mathematical framework: the space of test functions and the dual space of distributions.

## Why Ordinary Functions Are Not Enough

Consider the ODE for a mass-spring system subjected to an impulsive force at time $t = 0$:

$$m\ddot{x} + kx = F\delta(t).$$

The impulse $F\delta(t)$ represents a force that delivers a finite impulse $F$ instantaneously. No ordinary function can have zero integral on every interval not containing 0 while simultaneously having a positive total integral. Yet the equation is physically meaningful and has a well-defined solution (the Green's function of the oscillator).

Similarly, in electrostatics, a point charge at the origin satisfies Poisson's equation $-\Delta \phi = q\delta(\mathbf{x})$, with the Coulomb potential $\phi(\mathbf{x}) = q/(4\pi|\mathbf{x}|)$ as the solution. The right-hand side is not a function; the distributional framework makes the equation rigorous.

## The Space of Test Functions

The test functions form the "recipients" of distributional pairing. We need them to be smooth (so that all integration-by-parts manipulations are valid) and compactly supported (so that boundary terms vanish).

**Definition.** The **space of test functions** is $\mathcal{D}(\mathbb{R}^n) = C_c^\infty(\mathbb{R}^n)$: the space of smooth ($C^\infty$) functions $\phi: \mathbb{R}^n \to \mathbb{R}$ with compact support.

**Example.** The "bump function" $\phi(x) = \begin{cases} \exp(-1/(1-|x|^2)) & |x| < 1 \\ 0 & |x| \geq 1 \end{cases}$ is in $\mathcal{D}(\mathbb{R}^n)$. By scaling and translating, one gets bump functions supported in any compact set.

The space $\mathcal{D}$ is equipped with a notion of convergence: $\phi_j \to \phi$ in $\mathcal{D}$ if all $\phi_j$ are supported in a common compact set $K$ and $\sup_x |D^\alpha(\phi_j - \phi)(x)| \to 0$ for every multi-index $\alpha$. This is not a norm topology but a strict inductive limit topology.

## Distributions

**Definition.** A **distribution** on $\mathbb{R}^n$ is a continuous linear functional $T: \mathcal{D}(\mathbb{R}^n) \to \mathbb{R}$. The space of distributions is denoted $\mathcal{D}'(\mathbb{R}^n)$.

Continuity means: if $\phi_j \to 0$ in $\mathcal{D}$, then $T(\phi_j) \to 0$. We write $\langle T, \phi \rangle$ or $T(\phi)$ for the action of $T$ on $\phi$.

## Examples of Distributions

**Regular distributions.** For any locally integrable function $f \in L^1_{\text{loc}}(\mathbb{R}^n)$, define $T_f(\phi) = \int_{\mathbb{R}^n} f(x)\phi(x) \, dx$. This is well-defined (since $f$ is locally integrable and $\phi$ has compact support), linear, and continuous. We identify $f$ with $T_f$ and write $\langle f, \phi \rangle = \int f\phi$.

**The Dirac delta.** $\langle \delta, \phi \rangle = \phi(0)$. More generally, $\langle \delta_a, \phi \rangle = \phi(a)$ for a point mass at $a$. The Dirac delta is not a regular distribution: no locally integrable function $f$ satisfies $\int f\phi = \phi(0)$ for all $\phi \in \mathcal{D}$.

**The principal value distribution.** $\langle \text{p.v.}(1/x), \phi \rangle = \lim_{\varepsilon \to 0} \int_{|x| > \varepsilon} \phi(x)/x \, dx$. This is well-defined for $\phi \in \mathcal{D}(\mathbb{R})$, since the singularity at $x = 0$ is integrable in the principal value sense.

**Distributions supported at a point.** More exotic examples include $\langle T, \phi \rangle = \phi'(0)$, $\langle T, \phi \rangle = \phi''(0)$, etc.—these are derivatives of the delta function in the sense to be defined in Chapter 2.

## Chapter Structure

Section 1 (Motivation from Delta Function) develops the impetus for distribution theory through examples from physics and ODE. Section 2 (Space of Test Functions) treats the topology of $\mathcal{D}$ carefully, including the existence of bump functions and the convergence structure. Section 3 (Distributions and Examples) gives the formal definition and a catalog of examples, including regular distributions, the Dirac delta, the Heaviside function, and distributions derived from measures.
