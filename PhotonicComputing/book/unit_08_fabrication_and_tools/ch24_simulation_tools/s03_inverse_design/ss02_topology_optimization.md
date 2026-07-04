# 24.3.2 Topology Optimization

The adjoint method delivers the gradient of an objective with respect to the permittivity at every point in a design region, at the cost of two simulations. Topology optimization is what you build on top of it: a procedure that starts from a featureless block of dielectric and, guided by that gradient, sculpts it pixel by pixel into a device — free to place material anywhere, create any number of features, and adopt any topology, subject only to the constraint that a foundry must be able to make the result. It is the technique behind the now-familiar inverse-designed components: the wavelength demultiplexer of a few square micrometers, the ultra-broadband splitter, the mode and polarization converters that look like etched static and outperform anything drawn by hand.

## The Density Method

The core difficulty is that manufacturing is *binary* — a pixel is either silicon or oxide — but gradient-based optimization needs *continuous* variables. The resolution is the **density method** borrowed from mechanical structural optimization. Assign each pixel a design variable $\rho\in[0,1]$ and interpolate the permittivity continuously,

$$\varepsilon(\rho) = \varepsilon_{\text{SiO}_2} + \rho\,(\varepsilon_{\text{Si}} - \varepsilon_{\text{SiO}_2}),$$

so $\rho=0$ is cladding, $\rho=1$ is core, and intermediate values are a fictitious gray material that exists only during optimization. Now the design space is continuous, the adjoint gradient applies directly, and a gradient ascent can begin. The catch is equally clear: the final device cannot contain gray. The entire craft of topology optimization is getting from a useful continuous relaxation to a *binary, manufacturable* geometry without destroying performance.

## Filtering and Projection

Two operations, applied to the density field at every iteration, accomplish this and simultaneously enforce a length scale.

**Filtering** convolves the raw density with a smoothing kernel of radius $R$. This removes the pathological checkerboard patterns and single-pixel features that unconstrained pixel optimization loves to produce, and — crucially — it imposes a **minimum length scale**: no feature smaller than roughly $R$ can survive the filter. The filter radius is how a foundry design rule enters the optimization.

**Projection** then pushes the filtered, still-gray density toward 0 or 1 using a smoothed-Heaviside threshold with sharpness $\beta$ and midpoint $\eta$,

$$\tilde\rho = \frac{\tanh(\beta\eta)+\tanh\!\big(\beta(\rho_{\text{filt}}-\eta)\big)}{\tanh(\beta\eta)+\tanh\!\big(\beta(1-\eta)\big)}.$$

At small $\beta$ the projection is gentle and the optimization landscape is smooth; $\beta$ is then **annealed upward** over the run, sharpening toward a true binary geometry as the design settles. By the final iterations $\beta$ is large, gray has been squeezed out, and the density field *is* the mask.

## Fabrication Constraints and Robustness

An inverse-designed device that works in simulation but not on silicon is worthless, and the early literature produced its share. Manufacturability is enforced on two fronts. First, **geometric constraints** — minimum feature width and minimum gap (e.g. 180 nm and 200 nm from the PDK's DRC), plus bounds on curvature and prohibitions on isolated islands or pinholes — are imposed through the filter radius and through explicit morphological penalty terms. Second, and more subtly, the design is made **robust** to the fabrication bias it will actually experience. The optimizer simulates not one geometry but three: an *eroded* version (as if the etch removed a few extra nanometers everywhere), the *nominal* version, and a *dilated* version (as if under-etched), and it optimizes the worst case among them. A device that performs well across a $\pm$few-nanometer uniform bias is one that survives the process-corner spread of Chapter 23. The same idea extends to bandwidth (optimize across many wavelengths) and to the fabrication statistics themselves, tying inverse design directly to the parametric-yield thinking of the fabrication chapter.

## The Optimizer and the Cost

Each iteration runs the forward and adjoint simulations, forms the pixel gradient, and takes a step with a bound-constrained optimizer — most commonly the **method of moving asymptotes (MMA)** or L-BFGS, which handle the many variables and the box constraints $\rho\in[0,1]$ gracefully. A design typically converges in tens to a few hundred iterations. The per-iteration cost is not quite two simulations but two *times* the number of wavelengths in the objective *times* the number of robustness corners — still utterly independent of the pixel count, which is the whole point.

## Worked Example: Design-Variable Budget for a Compact Splitter

Take a $3\times3\ \mu\text{m}$ design region discretized at $20$ nm pixels: $150\times150 = 22{,}500$ design variables.

- **Gradient by finite differences:** $22{,}501$ simulations per iteration — impossible.
- **Gradient by adjoint:** 2 simulations per iteration, independent of the 22,500 variables.

Suppose the objective spans $N_\lambda = 5$ wavelengths (for broadband operation) and $3$ robustness corners (eroded/nominal/dilated at $\pm10$ nm bias). Each iteration then costs $2\times5\times3 = 30$ simulations, and a $100$-iteration optimization costs $\sim$3000 simulations — GPU-hours on a modern FDTD engine, entirely feasible. A **filter radius of $R\approx90$ nm** guarantees a minimum feature near $180$ nm, satisfying the foundry rule, while the $\pm10$ nm robust corners ensure the returned device tolerates the etch bias that Chapter 23 says is unavoidable. The output is a binary, DRC-respecting silicon pattern — often visually unintelligible — that the layout flow of Section 24.2.3 drops straight into a GDSII file.

## What It Delivers, and Its Caveats

Topology optimization's signature results are *compactness* and *bandwidth* beyond conventional design: Piggott and colleagues' wavelength demultiplexer (*Nature Photonics*, 2015) separated 1300 and 1550 nm in a $2.8\times2.8\ \mu\text{m}$ footprint, and inverse-designed splitters, grating couplers, and mode converters routinely beat their hand-drawn counterparts in size or bandwidth. The costs are real too. The devices are hard to interpret, so intuition and debugging suffer; performance can be sensitive to how faithfully the simulation modeled the process; and without the robustness and DRC machinery above, an optimized device may simply not reproduce on silicon. The maturation of the field — codified in tools like Stanford's SPINS (Su et al., 2020) — is largely the story of making topology optimization *fabrication-aware* enough that its spectacular simulated devices become spectacular measured ones. That reliability, not raw performance, is what has moved inverse design from a curiosity into the standard toolbox.
