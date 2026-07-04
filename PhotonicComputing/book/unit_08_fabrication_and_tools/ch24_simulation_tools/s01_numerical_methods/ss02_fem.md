# 24.1.2 The Finite Element Method (FEM)

Where FDTD tiles space with a rigid rectangular grid, the finite element method meshes it with triangles (in 2D) or tetrahedra (in 3D) that conform to the geometry. A curved ring boundary or a sloped etched sidewall is represented by mesh edges that lie *on* the boundary, not by a staircase of cells that approximate it. This conformity, together with the ability to refine the mesh locally where fields vary rapidly and coarsen it where they do not, makes FEM the method of choice for two photonic tasks: computing the guided modes of a waveguide, and solving the multiphysics couplings — thermal, mechanical, electrical — that a purely optical solver cannot touch.

## From the Curl-Curl Equation to a Weak Form

FEM works in the frequency domain. Starting from the time-harmonic vector wave equation for the electric field,

$$\nabla\times\left(\mu_r^{-1}\nabla\times\mathbf{E}\right) - k_0^2\,\varepsilon_r\,\mathbf{E} = 0,$$

one does not attempt to satisfy it pointwise. Instead the equation is multiplied by a test function and integrated over the domain, moving one derivative onto the test function via integration by parts. The result is the **weak form**: an integral statement that the residual is orthogonal to a chosen space of functions. The field is then expanded in local basis functions — low-order polynomials supported on individual elements — and the weak form collapses to a large, *sparse* matrix equation. Sparsity is the point: each basis function overlaps only its neighbors, so the system matrix has a bandwidth set by the mesh connectivity and is solved efficiently by sparse direct or iterative solvers.

One subtlety is essential to get right. Naïve nodal (scalar-per-component) interpolation of a vector field produces **spurious modes** — nonphysical solutions that pollute the spectrum — because it fails to enforce $\nabla\cdot(\varepsilon\mathbf{E})=0$ and mishandles the tangential-continuous, normal-discontinuous behavior of **E** at dielectric interfaces. The cure, now universal, is **edge (Nédélec) elements**, which assign degrees of freedom to element edges and enforce tangential continuity exactly, banishing the spurious modes. Any credible photonic FEM code uses them.

## The Mode Solver: FEM's Bread and Butter

The single most common use of FEM in photonics is not a full device simulation but a **mode solve** on a 2D waveguide cross-section. Assuming propagation as $e^{-i\beta z}$, the vector wave equation becomes a generalized eigenvalue problem for the propagation constant $\beta$ (equivalently the effective index $n_\text{eff}=\beta/k_0$) and the transverse mode profile. Solving it returns the fundamental and higher-order modes, their effective and group indices, their polarization (TE/TM) fraction, and — with a perturbation or a complex permittivity — their loss. Every compact model in Section 24.2 begins here: the $n_\text{eff}$ and $n_g$ that set a ring's resonance and a delay line's latency come from a mode solve. (The closely related **finite-difference eigenmode (FDE)** solver in Ansys Lumerical MODE does the same job on a rectangular grid; COMSOL, Ansys HFSS, and the open-source FEniCS use true FEM.)

## Worked Example: Mode Solve of a Silicon Strip Waveguide

Take the canonical $500\times220$ nm silicon strip in oxide at $\lambda_0 = 1550$ nm and solve for the fundamental TE mode.

**Discretization.** The cross-section computational window need only be a few micrometers on a side — say $4\times3\ \mu\text{m}$ — because the mode decays evanescently into the cladding; PML or a scattering boundary closes the window. A conforming mesh with $\sim$20–30 nm elements through the high-field core (finer at the corners, where the field is singular) and a coarsening mesh outside yields on the order of $10^4$ nodes, or a few $\times 10^4$ vector degrees of freedom.

**Cost.** This is a *small, sparse generalized eigenproblem*, and only a handful of eigenvalues near a target index are wanted, so a shift-and-invert Arnoldi solver returns the modes in a fraction of a second on a laptop. Contrast this with the ring FDTD of the previous subsection — $7\times10^7$ cells marched for $10^6$ steps. For anything that reduces to *find the mode and its index*, a 2D mode solve is three to four orders of magnitude cheaper than a 3D time-domain run, which is precisely why the hierarchy pushes as much work as possible down to it.

**Result and convergence.** The solver returns $n_\text{eff}\approx 2.44$ for the fundamental TE mode and $n_g\approx 4.2$, with the mode tightly confined in the core but with appreciable evanescent tails — the tails that make directional couplers and bend losses what they are. The effective index converges as $\mathcal{O}(h^2)$ in the element size $h$; refining the core mesh below $\sim$25 nm stabilizes $n_\text{eff}$ to better than $10^{-4}$. Differentiating the converged solution numerically reproduces the fabrication sensitivities used throughout Chapter 23 — $\partial n_\text{eff}/\partial w \approx 1.5\times10^{-3}\ \text{nm}^{-1}$ — so the mode solver is not only a design tool but the source of the variability budgets.

## Multiphysics: Where FEM Has No Rival

FEM's second decisive advantage is that the same mesh and the same weak-form machinery solve *any* field problem, so different physics can be coupled self-consistently. This is COMSOL Multiphysics' domain, and it matters enormously for photonic computing because the dominant tuning mechanism is thermal. A realistic thermo-optic simulation solves the steady heat equation for the temperature field $T(x,y)$ produced by a TiN or doped-silicon heater dissipating a known power, maps it to an index perturbation through the thermo-optic coefficient $\Delta n = (dn/dT)\,\Delta T$ (for silicon, $dn/dT \approx 1.86\times10^{-4}\ \text{K}^{-1}$), and feeds that perturbed permittivity back into the mode solve to predict the phase shift — and thus the tuning efficiency in nm/mW and the crosstalk to neighboring waveguides. The same framework computes the mechanical stress that shifts birefringence, the carrier distribution in a doped modulator, and the RF electrode fields that set a modulator's bandwidth. No time-domain electromagnetic solver reaches these couplings; FEM absorbs them naturally.

## When to Reach for FEM

Choose FEM (or FDE) for anything where curved geometry, resonant precision, or an eigenproblem dominates, and above all for mode solving and multiphysics. Its frequency-domain nature is a weakness for broadband problems — each wavelength is a separate solve, the opposite of FDTD's one-shot spectrum — and full 3D FEM of an electrically large device produces a matrix too large to factor, so volumetric device simulation usually stays with FDTD. The division of labor is clean: FEM finds the modes and the temperatures; FDTD finds the broadband scattering; and the circuit layer of Section 24.2 stitches their outputs together.
