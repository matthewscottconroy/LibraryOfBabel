# 24.3.1 The Adjoint Method

Inverse design is optimization: define an objective $F$ that measures how well a device meets its target, and maximize it over the design degrees of freedom. Every gradient-based optimizer needs $\nabla F$ — the sensitivity of the objective to each design variable. The obstacle is that computing that gradient by the obvious method, finite differences, costs one full electromagnetic simulation *per variable*: perturb parameter one, re-simulate, measure the change; perturb parameter two, re-simulate; and so on. For a device with a hundred thousand pixels this is a hundred thousand simulations per optimization step, and the whole enterprise is dead on arrival. The adjoint method revives it by computing the entire gradient — every partial derivative — from just **two** simulations, at a cost independent of the number of design variables.

## The Setup

Let the electromagnetic problem, in the frequency domain, be written as a linear system for the field $\mathbf{E}$,

$$A(\varepsilon)\,\mathbf{E} = \mathbf{b},$$

where $A$ is the discretized Maxwell (curl-curl minus $\omega^2\varepsilon$) operator, $\mathbf{b}$ the source, and $\varepsilon(\mathbf{r})$ the design — the permittivity at every point we are allowed to change. The objective $F(\mathbf{E})$ is some function of the field: the power coupled into a target output mode, the transmission into a chosen port, the field intensity at a focus. We want $\partial F/\partial\varepsilon_i$ for every design point $i$.

## The Two-Simulation Result

Differentiating $F$ through the field's dependence on $\varepsilon$, and using the chain rule on the constraint $A\mathbf{E}=\mathbf{b}$, gives

$$\frac{\partial F}{\partial\varepsilon_i} = -\,\text{Re}\!\left\{\boldsymbol{\lambda}^{\!\top}\frac{\partial A}{\partial\varepsilon_i}\,\mathbf{E}\right\},$$

where the **adjoint field** $\boldsymbol{\lambda}$ solves

$$A^{\!\top}\boldsymbol{\lambda} = \left(\frac{\partial F}{\partial\mathbf{E}}\right)^{\!\top}.$$

Two facts collapse this into something cheap. First, the Maxwell operator is *symmetric* by reciprocity, $A^{\!\top}=A$, so the adjoint system is solved by the **same solver** as the forward problem — merely with a different source. That adjoint source, $(\partial F/\partial\mathbf{E})^{\top}$, is placed exactly where the objective is measured (the output monitor) and physically corresponds to launching a field *backward* from the output. Second, because the permittivity enters $A$ only locally — $\partial A/\partial\varepsilon_i$ touches only point $i$, contributing a factor $\sim\omega^2$ — the gradient at every design point reduces to a pointwise product of the two fields already computed:

$$\frac{\partial F}{\partial\varepsilon_i} \;\propto\; \text{Re}\{\,\mathbf{E}_{\text{adj}}(\mathbf{r}_i)\cdot\mathbf{E}_{\text{fwd}}(\mathbf{r}_i)\,\}.$$

That is the whole method. Run the **forward** simulation to get $\mathbf{E}_{\text{fwd}}$ everywhere; run the **adjoint** simulation (same solver, source at the output monitor) to get $\mathbf{E}_{\text{adj}}$ everywhere; multiply them point by point, and read off the sensitivity of the objective to a permittivity change at *every* location simultaneously. Two simulations, complete gradient.

## The Physical Picture

The interpretation is illuminating. The forward field is the light the device actually produces from its input. The adjoint field is the light that *would* arrive at each point if the desired output were run backward through the structure. Where the two overlap strongly and in phase, adding material there steers the device toward the target; where they overlap out of phase, removing material helps. The gradient map is literally an image of where the device "wants" more or less dielectric. In FDTD the adjoint run is a second time-domain simulation with a time-reversed source at the monitor; in frequency-domain solvers it is a transposed linear solve — either way, exactly one extra simulation.

## Worked Example: The Simulation-Count Argument

Consider optimizing a compact mode converter whose design region is discretized into $N = 10^5$ permittivity pixels.

- **Finite differences:** one baseline simulation plus one per pixel to estimate each partial derivative — $N+1 = 100{,}001$ simulations *per optimization iteration*. At a few minutes per FDTD run, a single gradient takes months; a hundred-iteration optimization is inconceivable.
- **Adjoint:** exactly 2 simulations per iteration — forward and adjoint — regardless of whether $N$ is $10^3$, $10^5$, or $10^7$. A hundred-iteration optimization is $\sim$200 simulations, a matter of GPU-hours.

The adjoint method does not make each simulation faster; it makes the *number* of simulations independent of the design's complexity. That decoupling is what let the field jump from optimizing a handful of shape parameters to optimizing every pixel of a device — the topology optimization of the next subsection.

## Shape versus Topology Gradients, and the Tools

The same adjoint framework serves two flavors of design. In **shape optimization**, the design variables are the positions of boundaries — the width and length of a taper, the vertices of a coupler — and the gradient tells how to move each boundary; the topology (what connects to what) is fixed. In **topology optimization**, every pixel's permittivity is free, and the gradient can create or annihilate features anywhere. The adjoint machinery is identical; only the parameterization differs.

The method was brought into nanophotonics through work by Owen Miller, Lalau-Keraly, and Yablonovitch (*Optics Express*, 2013) and developed extensively in Jelena Vučković's group at Stanford. It is now packaged in accessible tools: **ceviche** (Hughes and Minkov), an open-source autograd-based FDFD/FDTD engine that differentiates Maxwell's equations directly; the **meep.adjoint** module of MIT's Meep; Stanford's **SPINS**; and the adjoint solvers built into Ansys Lumerical (lumopt) and Flexcompute's Tidy3D. The convergence of automatic differentiation with electromagnetic solvers — differentiable simulation — has made the adjoint gradient as routine to obtain as a forward transmission, and it is the computational engine beneath everything that follows.
