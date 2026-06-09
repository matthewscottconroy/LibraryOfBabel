# Chapter 4: The Korteweg-de Vries Equation

The Korteweg-de Vries (KdV) equation:

$$u_t - 6uu_x + u_{xxx} = 0 \tag{KdV}$$

(or equivalently $u_t + 6uu_x + u_{xxx} = 0$ with the opposite sign convention) was derived by Diederik Korteweg and Gustav de Vries in 1895 to describe the propagation of long water waves in shallow channels. It is now recognized as the archetypical **integrable dispersive nonlinear PDE** — an equation with infinitely many conserved quantities, exact soliton solutions, and a complete solution method (the inverse scattering transform) that is the nonlinear analog of the Fourier transform.

## Physical Origin

In a shallow channel of depth $h$ with gravitational acceleration $g$, small-amplitude long waves satisfy the linear wave equation. When the wavelength $\lambda \gg h$ but the amplitude $a$ is not negligible compared to $h$, two correction effects appear:
1. **Nonlinearity** ($uu_x$ term): finite amplitude effects cause steeper waves to travel faster (the same mechanism as in Burgers' equation, leading to shock formation).
2. **Dispersion** ($u_{xxx}$ term): short waves travel slower than long waves (the dispersion relation is $\omega = k - k^3$ in appropriate units), spreading wave packets.

Korteweg and de Vries showed that for the distinguished scaling where these two effects are of comparable size, the wave profile satisfies their equation. The balance between nonlinear steepening and dispersive spreading produces **solitons** — waves that propagate without distortion.

## The Two Competing Effects

**Without dispersion** (pure nonlinear: $u_t - 6uu_x = 0$): this is the inviscid Burgers-type equation (with $f(u) = -6u$), and characteristics converge to form shocks in finite time.

**Without nonlinearity** (pure dispersion: $u_t + u_{xxx} = 0$): the dispersion relation is $\omega = k^3$ (from $e^{i(kx-\omega t)}$ solutions), so different Fourier modes travel at different speeds $\omega/k = k^2$. The solution $u(x,t) = \int\hat u_0(k)e^{i(kx-k^3t)}\,dk$ spreads out and decays.

**Balance:** the KdV equation supports **soliton solutions** where the two effects exactly cancel, giving a localized traveling wave that persists indefinitely without deformation.

## Structure of This Chapter

**Section 1: Solitons** derives the one-soliton (solitary wave) solution explicitly, analyzes its properties (shape, speed, amplitude), and describes the remarkable soliton interaction property: two solitons collide and emerge unchanged (in shape and speed), as if passing through each other, with only a phase shift as evidence of the collision.

**Section 2: Inverse Scattering Transform — Introduction** outlines the inverse scattering transform (IST), the method that solves the KdV initial value problem exactly. The IST works by a three-step process:
1. **Forward problem:** Map $u_0(x)$ to the scattering data of the associated Schrödinger operator $-\partial_{xx} + u_0$.
2. **Time evolution:** The scattering data evolves linearly in time under simple exponential formulas.
3. **Inverse problem:** Recover $u(x,t)$ from the evolved scattering data via the Gel'fand-Levitan-Marchenko integral equation.

This three-step procedure is the exact analog of the Fourier transform method: forward Fourier transform, multiply by phase factor $e^{-ik^2t}$, inverse Fourier transform. The solitons correspond to the discrete (bound state) part of the spectrum of $-\partial_{xx} + u_0$, and the dispersive radiation corresponds to the continuous spectrum.

## The Conserved Quantities

One hallmark of KdV integrability is the existence of **infinitely many conserved quantities** (first integrals):

$$I_0 = \int u\,dx, \qquad I_1 = \int u^2\,dx, \qquad I_2 = \int\left(\frac{1}{2}u_x^2 + u^3\right)dx, \quad \ldots$$

$I_0$ is conservation of mass, $I_1$ is conservation of momentum ($L^2$ norm), $I_2$ is the Hamiltonian (energy). Each $I_n$ is a polynomial functional of $u$ and its $x$-derivatives. The existence of infinitely many conserved quantities is the formal definition of complete integrability — it is the reason solitons can collide without scattering (energy cannot be transferred between modes because all mode-amplitudes are separately conserved).

## Historical Significance

The discovery of the inverse scattering transform by Gardner, Greene, Kruskal, and Miura (1967) initiated the modern theory of integrable systems. Their work solved the initial value problem for KdV and explained numerically observed soliton collisions (Zabusky and Kruskal, 1965). The method was then generalized to dozens of other nonlinear equations (nonlinear Schrödinger, sine-Gordon, modified KdV, etc.), revealing a deep structure connecting PDE theory, algebraic geometry (solutions in terms of theta functions of algebraic curves), classical mechanics (Hamiltonian integrable systems), and quantum field theory (S-matrix theory for solitons).

The KdV equation thus occupies a unique position: it is a nonlinear PDE with a complete and explicit solution theory, yet it is fundamentally nonlinear (its soliton solutions have no linear analog). Understanding it illuminates both the exceptional structure of integrable equations and — by contrast — the difficulties that arise for non-integrable nonlinear PDEs.
