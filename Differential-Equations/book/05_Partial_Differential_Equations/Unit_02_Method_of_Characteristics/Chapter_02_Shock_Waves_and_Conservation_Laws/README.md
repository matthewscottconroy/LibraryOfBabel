# Chapter 2: Shock Waves and Conservation Laws

When characteristics of a quasilinear PDE converge and cross, the classical smooth solution ceases to exist. This is not a pathology to be avoided — it is the mathematical signature of a shock wave, one of the most important phenomena in fluid dynamics, gas dynamics, and traffic flow. The theory of shock waves requires extending the notion of solution beyond smooth functions to admit discontinuities, and it raises a fundamental question: among all possible discontinuous "solutions," which one is physically correct?

This chapter answers that question through three closely related topics.

## Conservation Laws

A scalar conservation law in one space dimension has the form

$$u_t + f(u)_x = 0,$$

where $u(x,t)$ is the conserved quantity (density, concentration, velocity in a simplified model) and $f(u)$ is the flux function. The equation expresses the conservation of the total amount $\int u\,dx$ in the absence of sources: any decrease in a region must be accounted for by flux out of the region.

Written as a quasilinear equation, this is $u_t + f'(u) u_x = 0$, and the characteristic speed is $f'(u)$. Since $f'(u)$ depends on $u$, different values of $u$ propagate at different speeds — the mechanism for shock formation.

## Shocks and the Rankine-Hugoniot Condition

When smooth solutions break down, one introduces **weak solutions**: functions $u$ that satisfy the conservation law in integral form,

$$\frac{d}{dt}\int_a^b u\,dx = f(u(a,t)) - f(u(b,t)),$$

for all $a < b$. A function with a jump discontinuity along a curve $x = s(t)$ is a weak solution if and only if the shock speed $\dot{s}$ satisfies the **Rankine-Hugoniot condition**:

$$\dot{s} = \frac{f(u^+) - f(u^-)}{u^+ - u^-} = \frac{[f]}{[u]},$$

where $u^\pm = \lim_{x\to s^\pm} u$ are the right and left limits of $u$ at the shock, and $[\cdot]$ denotes the jump. This condition is necessary and sufficient for a step function with a single jump to be a weak solution. Physically, it expresses conservation of flux across the shock.

## Entropy Conditions

The Rankine-Hugoniot condition is necessary but not sufficient to identify the physical solution. For a given Riemann problem (piecewise constant initial data with a single jump), multiple weak solutions may exist — some physically spurious. The **entropy condition** (or admissibility condition) selects the physically relevant one.

For a convex flux $f''(u) > 0$, the Lax entropy condition states that characteristics on the left must enter the shock faster than the shock moves, and characteristics on the right must enter it slower:

$$f'(u^-) > \dot{s} > f'(u^+).$$

Geometrically, characteristics must "run into" the shock from both sides — the shock compresses incoming information rather than creating it. A shock satisfying this condition is **admissible** (entropy-satisfying); a shock violating it is **rarefaction shock** that is physically inadmissible and can be replaced by a smooth rarefaction wave.

## Why This Chapter Matters

The theory of conservation laws and shock waves is fundamental to mathematical physics. Traffic flow, gas dynamics (the Euler equations of compressible flow), flood waves in rivers, and the propagation of combustion fronts are all modeled by systems of conservation laws. The concepts of weak solutions, Rankine-Hugoniot conditions, and entropy conditions that are introduced in this scalar setting carry over — with considerable additional complexity — to systems of conservation laws governing real fluid flows.

Understanding shocks is also prerequisite for understanding the stability and accuracy of numerical schemes for hyperbolic equations: a scheme that does not satisfy a discrete entropy condition may converge to the wrong weak solution.
