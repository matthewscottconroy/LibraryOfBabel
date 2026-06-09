# Chapter 3: The Multidimensional Wave Equation

The wave equation in more than one spatial dimension governs sound in rooms, electromagnetic waves in free space, seismic waves, and the vibration of membranes and solid bodies. The extension from one to higher dimensions introduces phenomena with no one-dimensional counterpart, most notably the strong Huygens principle — the fact that in odd dimensions greater than one, sharp wavefronts propagate exactly at speed $c$ with no trailing signal, while in even dimensions (and in one dimension), waves leave a "wake."

## The Equation in Higher Dimensions

The wave equation in $n$ spatial dimensions is:

$$u_{tt} = c^2\Delta u, \qquad \mathbf{x} \in \mathbb{R}^n,\; t > 0,$$

with $\Delta = \sum_{i=1}^n \partial^2/\partial x_i^2$.

## Structure of This Chapter

**Section 1: 2D and 3D Wave Equation** derives the explicit solution formulas. In 3D, the solution is given by the **Kirchhoff formula**: the value of $u$ at $(\mathbf{x},t)$ is the average of $\psi$ over the sphere of radius $ct$ centered at $\mathbf{x}$, plus the time derivative of the average of $\phi$ over the same sphere. Notably, the solution depends only on data on the sphere (not inside it) — this is the strong form of Huygens' principle in 3D.

In 2D (Poisson's formula), the solution at $(\mathbf{x},t)$ depends on initial data in the entire disk of radius $ct$, including the interior — there are trailing waves even for compactly supported initial data.

**Section 2: Circular Membranes and the Drumhead** solves the 2D wave equation on a disk — the mathematical model of a drum. Separation of variables in polar coordinates gives Bessel functions in the radial direction. The natural frequencies are $\omega_{mn} = cj_{mn}/R$ where $j_{mn}$ is the $n$-th zero of the Bessel function $J_m$. The nodal lines (curves where $u=0$ for all $t$) form the characteristic patterns visible on vibrating membranes (Chladni figures).

**Section 3: Spherical Waves** studies radially symmetric solutions of the 3D wave equation. The substitution $v = ru$ reduces the equation to the 1D wave equation for $v$, giving the explicit solution $u = [f(r+ct) + g(r-ct)]/r$ — outgoing and incoming spherical waves with amplitude decaying as $1/r$.

## Key Dimension-Dependence

The most striking feature of this chapter is the profound dependence of wave behavior on spatial dimension:
- **Odd dimensions $\geq 3$:** Sharp Huygens principle holds — waves have sharp fronts, no trailing signal.
- **Even dimensions:** Waves have trailing signals — a sharp impulse produces a sound that lingers.
- **One dimension:** Also has trailing signal (d'Alembert's formula shows dependence on entire interval, not just endpoints).

This dimension-dependence arises from the theory of Riesz potentials and the explicit structure of the fundamental solutions in each dimension.
