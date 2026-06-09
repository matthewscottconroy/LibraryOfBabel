# Introduction to the Inverse Scattering Transform

The **inverse scattering transform** (IST) is a method for solving the initial value problem for completely integrable nonlinear PDEs — equations like KdV, the nonlinear Schrödinger equation, the Toda lattice, and others. For the KdV equation $u_t - 6uu_x + u_{xxx} = 0$ with initial data $u(x,0) = u_0(x)$, the IST provides an exact, explicit solution valid for all $t > 0$. The method has three steps: (1) associate a linear scattering problem (Schrödinger equation) to $u_0$, (2) evolve the scattering data in time using simple ODEs, (3) recover $u(x,t)$ from the evolved scattering data via the Gel'fand-Levitan-Marchenko (GLM) equation. This three-step structure is the exact nonlinear analog of the Fourier transform method for solving linear PDEs.

## The Lax Pair

The fundamental insight of Gardner, Greene, Kruskal, and Miura (1967) is that the KdV equation is equivalent to the **Lax pair**:

$$L_t = [B,L] \tag{Lax equation}$$

where $L$ and $B$ are differential operators:

$$L = -\partial_x^2 + u(x,t), \qquad B = -4\partial_x^3 + 6u\partial_x + 3u_x.$$

Here $[B,L] = BL - LB$ is the commutator. The Lax equation $L_t = [B,L]$ is equivalent (by direct computation) to the KdV equation $u_t - 6uu_x + u_{xxx} = 0$.

**Isospectral deformation.** The Lax equation implies that the spectrum of $L$ is time-independent (isospectral deformation): if $L(t)\psi = \lambda\psi$ at time $t$, then $L(t')\psi' = \lambda\psi'$ with the same $\lambda$ for all $t' > 0$. This is the key: as $u$ evolves by KdV, the operator $L = -\partial_x^2 + u$ changes, but its spectrum (eigenvalues) does not.

**Proof of isospectrality.** Suppose $L\psi = \lambda\psi$ and $\psi$ evolves by $\psi_t = B\psi$. Differentiate $L\psi = \lambda\psi$ with respect to $t$:

$$L_t\psi + L\psi_t = \lambda_t\psi + \lambda\psi_t.$$

Substituting $\psi_t = B\psi$ and $L_t = [B,L]$:

$$[B,L]\psi + LB\psi = \lambda_t\psi + \lambda B\psi \implies BL\psi + \lambda_tpsi + \lambda B\psi = \lambda_t\psi + \lambda B\psi.$$

Wait: $[B,L]\psi = BL\psi - LB\psi = B(\lambda\psi) - LB\psi = \lambda B\psi - LB\psi$. So:

$$L_t\psi + LB\psi = \lambda B\psi - LB\psi + LB\psi = \lambda B\psi.$$

Thus $L_t\psi = \lambda B\psi - LB\psi + LB\psi - LB\psi = (BL-LB)\psi + LB\psi - LB\psi$... More carefully: $L\psi_t = LB\psi$. And $\lambda\psi_t = \lambda B\psi$. So the equation $L_t\psi + L\psi_t = \lambda_t\psi + \lambda\psi_t$ becomes $[B,L]\psi + LB\psi = \lambda_t\psi + \lambda B\psi$. Using $[B,L]\psi = BL\psi - LB\psi = \lambda B\psi - LB\psi$: $\lambda B\psi - LB\psi + LB\psi = \lambda_t\psi + \lambda B\psi$, giving $\lambda B\psi = \lambda_t\psi + \lambda B\psi$, so $\lambda_t = 0$. $\square$

## Forward Scattering Problem

The scattering problem associated to KdV is the 1D time-independent Schrödinger equation:

$$-\psi_{xx} + u_0(x)\psi = \lambda\psi, \quad x\in\mathbb{R}. \tag{Schrödinger}$$

For $u_0 \in L^1(\mathbb{R})$ with $\int(1+|x|)|u_0(x)|\,dx < \infty$ (rapidly decaying), the Schrödinger operator $L_0 = -\partial_x^2 + u_0$ has:

**Discrete spectrum.** A finite number of negative eigenvalues $\lambda_n = -\kappa_n^2$ ($\kappa_n > 0$, $n=1,\ldots,N$), corresponding to bound state solutions $\psi_n\in L^2(\mathbb{R})$ satisfying:

$$\psi_n(x) \sim c_n e^{-\kappa_n|x|} \quad \text{as } |x|\to\infty.$$

Normalize: $\int|\psi_n|^2\,dx = 1$, and define the norming constants $b_n(0) = c_n^{-1}$.

**Continuous spectrum.** For $\lambda = k^2 > 0$, the Jost solutions satisfy:

$$f(x,k) \sim e^{ikx} \quad (x\to+\infty), \qquad f(x,k) \sim a(k)e^{ikx} + b(k)e^{-ikx} \quad (x\to-\infty).$$

The **reflection coefficient** is $r(k) = b(k)/a(k)$ and the **transmission coefficient** is $1/a(k)$.

The **scattering data** at $t=0$ is: $\mathcal{S}(0) = \{r(k,0); \kappa_n, b_n(0): n=1,\ldots,N\}$.

## Time Evolution of Scattering Data

By the isospectrality, $\kappa_n(t) = \kappa_n(0)$ for all $t$ — the bound state energies do not change. The eigenfunctions evolve by $\psi_t = B\psi$, and tracking this evolution:

**Discrete spectrum:** $b_n(t) = b_n(0)e^{4\kappa_n^3 t}$.

**Continuous spectrum:** $r(k,t) = r(k,0)e^{8ik^3t}$.

These are remarkably simple linear ODEs! The nonlinear complexity of the KdV equation has been "linearized" in the scattering variables.

## Inverse Scattering Problem: Gel'fand-Levitan-Marchenko Equation

Given the time-evolved scattering data $\mathcal{S}(t)$, recover $u(x,t)$.

**Step 1:** Define the kernel:

$$F(x,t) = \sum_{n=1}^N b_n(t)^2 e^{-\kappa_n x} + \frac{1}{2\pi}\int_{-\infty}^\infty r(k,t)e^{ikx}\,dk.$$

**Step 2:** Solve the **Gel'fand-Levitan-Marchenko (GLM) integral equation** for $K(x,y,t)$:

$$K(x,y,t) + F(x+y,t) + \int_x^\infty K(x,z,t)F(z+y,t)\,dz = 0, \quad y > x. \tag{GLM}$$

**Step 3:** Recover the potential (solution of KdV):

$$u(x,t) = -2\frac{d}{dx}K(x,x,t). \tag{Recovery}$$

The GLM equation is a Fredholm linear integral equation of the second kind, which can be solved analytically in special cases (pure soliton initial data: $r=0$) or numerically in general.

## Soliton Solutions from IST

For **reflectionless potentials** ($r(k,0) = 0$ for all $k$), the GLM equation reduces to a finite-dimensional linear system.

**One soliton ($N=1$, $r=0$).** $F(x,t) = b_1^2 e^{8\kappa_1^3 t}e^{-\kappa_1 x}$. The GLM equation gives:

$$K(x,y,t) = -\frac{b_1^2 e^{8\kappa_1^3 t-\kappa_1(x+y)}}{1+\frac{b_1^2}{2\kappa_1}e^{8\kappa_1^3 t - 2\kappa_1 x}}.$$

Then $K(x,x,t) = -\frac{b_1^2 e^{8\kappa_1^3t-2\kappa_1 x}}{1+\frac{b_1^2}{2\kappa_1}e^{8\kappa_1^3t-2\kappa_1 x}}$, and:

$$u(x,t) = -2\frac{\partial}{\partial x}K(x,x,t) = -2\kappa_1^2\text{sech}^2(\kappa_1 x - 4\kappa_1^3 t - x_0)$$

where $e^{-2x_0} = b_1^2/(2\kappa_1)$. This is the one-soliton solution with speed $c = 4\kappa_1^2$ and $u_{\max} = -2\kappa_1^2$ (the convention here gives negative solitons). Translating to the positive-soliton convention via the sign flip: the soliton moves right at speed $4\kappa_1^2$ with amplitude $2\kappa_1^2$.

**Two solitons ($N=2$, $r=0$).** The GLM equation reduces to a $2\times 2$ linear system, solved explicitly:

$$u(x,t) = -2\frac{\partial^2}{\partial x^2}\log\det M,$$

where $M_{ij} = \delta_{ij} + \frac{b_i b_j}{\kappa_i + \kappa_j}e^{4(\kappa_i^3+\kappa_j^3)t - (\kappa_i+\kappa_j)x}$. The long-time asymptotics of $\log\det M$ decompose into two separated one-soliton solutions with phase shifts, confirming the elastic collision.

## The General Solution

For generic initial data $u_0 \in L^1(\mathbb{R})$ with $N$ bound states and reflection coefficient $r(k)$, the solution $u(x,t)$ decomposes as $t\to\infty$ into:
- **$N$ solitons**: the discrete spectrum contributes $N$ well-separated solitons with amplitudes $2\kappa_n^2$ and speeds $4\kappa_n^2$, ordered from fastest (largest $\kappa_n$) on the right to slowest on the left.
- **Dispersive radiation**: the continuous spectrum contributes a decaying dispersive wave packet $u_{\text{rad}}(x,t) = O(t^{-1/3})$ in the region $x = O(t^{1/3})$ (dispersive decay of the continuous part).

This is the **soliton resolution conjecture** (proved for KdV): every solution with generic initial data eventually resolves into a finite number of solitons plus dispersive radiation. The solitons carry the "permanent" part of the initial energy; the radiation disperses to zero.

## Comparison with Fourier Transform

| | Fourier Transform (linear heat eq.) | Inverse Scattering (KdV) |
|---|---|---|
| Forward transform | $\hat u_0(k) = \int u_0 e^{-ikx}dx$ | Scattering data $\{r(k,0); \kappa_n, b_n(0)\}$ |
| Time evolution | $\hat u(k,t) = \hat u_0(k)e^{-k^2t}$ | $r(k,t) = r(k,0)e^{8ik^3t}$; $b_n(t) = b_n(0)e^{4\kappa_n^3t}$ |
| Inverse transform | $u(x,t) = \int\hat u(k,t)e^{ikx}dk$ | GLM equation $\to u(x,t) = -2\partial_x K(x,x,t)$ |
| Linear/nonlinear | Linear | Nonlinear |
| Decay | All modes decay | Solitons persist; radiation decays |

The IST is the exact nonlinear analog of the Fourier transform — it linearizes the nonlinear KdV dynamics in the "scattering coordinates," where the evolution is simple, then inverts via the GLM equation.

## Other Integrable Equations

The IST framework applies to many other nonlinear PDEs, each with its own Lax pair:

- **Nonlinear Schrödinger (NLS):** $iu_t + u_{xx} \pm |u|^2 u = 0$. Lax pair involves a $2\times 2$ Zakharov-Shabat scattering problem. Solitons are envelope solitons in optics (bright solitons for focusing NLS, dark solitons for defocusing NLS).
- **Sine-Gordon equation:** $u_{tt} - u_{xx} + \sin u = 0$. Solitons are kink solutions (topological solitons): $u = 4\arctan e^{\gamma(x-vt)}$.
- **Toda lattice:** $\ddot q_n = e^{q_{n-1}-q_n} - e^{q_n-q_{n+1}}$ (discrete system). The continuum limit gives KdV.

The common thread: each integrable equation can be written as the compatibility condition of two linear equations (a Lax pair), and the solution is obtained via the corresponding inverse scattering problem.
