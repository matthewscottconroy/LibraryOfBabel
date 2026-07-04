# 24.1.1 Finite-Difference Time-Domain (FDTD)

FDTD is the workhorse of rigorous photonic simulation: general, conceptually simple, and — its defining virtue — able to return a device's entire broadband spectrum from a single run. It solves Maxwell's two curl equations directly, discretized on a grid, by marching the fields forward in time. Nothing about the mode structure, the resonances, or the reflections is assumed; they emerge from the dynamics. That generality is also its cost, and learning to estimate that cost before launching a job is the practical skill this subsection builds.

## The Yee Cell and the Leapfrog Update

The algorithm's foundation is the staggered grid introduced by Kane Yee in 1966. The electric and magnetic field components are not colocated; each **E** component sits on the midpoint of a cell edge and each **H** component on the midpoint of a cell face, so that every field component is surrounded by the curl partners it needs. In time the fields are equally staggered: **E** is updated at integer time steps, **H** at half-integer steps. The two curl equations then become an explicit **leapfrog** scheme,

$$\frac{\partial \mathbf{H}}{\partial t} = -\frac{1}{\mu}\nabla\times\mathbf{E}, \qquad \frac{\partial \mathbf{E}}{\partial t} = \frac{1}{\varepsilon}\nabla\times\mathbf{H},$$

in which each new **H** is computed from the spatial curl of the current **E**, then each new **E** from the curl of the just-updated **H**. Every update is local — each field point talks only to its immediate neighbors — which is why FDTD parallelizes almost perfectly across CPU cores and GPUs. The staggering makes the scheme second-order accurate in space and time and, elegantly, time-reversible in the lossless case.

Two numerical artifacts follow from the grid. **Numerical dispersion** — the grid's own frequency-dependent phase velocity — demands a resolution of roughly 15–20 points per wavelength *in the highest-index material*; too coarse a grid and a wave accumulates a spurious phase error over long propagation. **Staircasing** — the approximation of curved or angled boundaries by the rectangular grid — is the dominant error for rings and tapers; production codes such as MIT's open-source **Meep** (Oskooi et al., 2010) mitigate it with subpixel averaging of the permittivity, recovering near-second-order accuracy even where a boundary cuts through a cell.

## Stability: The Courant Limit

Because the scheme is explicit, the time step is not free. Information on the grid may not outrun light: a field disturbance must not cross a cell in less than one time step. This is the **Courant–Friedrichs–Lewy (CFL)** condition. In three dimensions with cell sizes $\Delta x, \Delta y, \Delta z$,

$$\Delta t \le \frac{1}{c\sqrt{\dfrac{1}{\Delta x^2}+\dfrac{1}{\Delta y^2}+\dfrac{1}{\Delta z^2}}},$$

which for cubic cells $\Delta x = \Delta y = \Delta z$ reduces to the familiar $\Delta t \le \Delta x/(c\sqrt{3})$. Halving the spatial grid to resolve a finer feature therefore *doubles* the number of time steps as well as multiplying the cell count by eight — the origin of FDTD's brutal $\mathcal{O}(N^4)$ scaling of a 3D simulation with linear resolution.

## Sources, Boundaries, and Broadband Monitors

A simulation is excited by a **source**: most usefully a *mode source*, which injects the fundamental waveguide mode (precomputed by a mode solver) across the waveguide cross-section, or a total-field/scattered-field plane wave for scattering problems. The domain is terminated by a **perfectly matched layer (PML)** — an artificial anisotropic absorber, due to Bérenger, that swallows outgoing radiation with reflections below $-60$ dB when properly graded, so a finite grid mimics open space.

The broadband capability comes from the source and monitors together. Excite with a short *Gaussian pulse* whose spectrum spans the wavelength band of interest, and record the fields at flux monitors that accumulate a running **discrete Fourier transform** at many frequencies simultaneously. One time-domain run, normalized against a straight-waveguide reference run, then yields transmission and reflection — the device's scattering parameters (Section 24.2) — across the *entire band at once*. This is the decisive advantage over frequency-domain solvers, which must repeat the whole solve at every wavelength.

## Worked Example: FDTD Budget for a Microring Resonator

Consider simulating an add-drop microring of radius $R = 5\ \mu\text{m}$ (matching the $\sim 18$ nm free-spectral-range ring of Chapter 23) in a silicon-on-insulator stack at $\lambda_0 = 1550$ nm.

**Grid.** The high-index medium is silicon, $n_\text{Si} = 3.47$, so the in-material wavelength is $\lambda_0/n = 447$ nm. Choosing $\Delta x = 20$ nm gives $447/20 \approx 22$ points per wavelength — comfortably above the numerical-dispersion floor. The domain must enclose the ring (diameter 10 μm), the bus waveguide, dielectric padding, and PML: take $14 \times 14 \times 3\ \mu\text{m}$ (the thin vertical extent spans the 220 nm core and its oxide cladding). The cell count is

$$N = \frac{14000}{20}\times\frac{14000}{20}\times\frac{3000}{20} = 700\times700\times150 \approx 7.4\times10^{7}\ \text{cells}.$$

**Memory.** Each Yee cell stores six field components; with material arrays and the auxiliary variables PML requires, a realistic footprint is $\sim$100 bytes/cell. The fields alone are $7.4\times10^{7}\times 6\times 4\ \text{bytes}\approx 1.8$ GB (single precision); the full simulation lands in the **4–8 GB** range — a single high-memory GPU or workstation, not a cluster.

**Time step.** With cubic cells, $\Delta t = \Delta x/(c\sqrt{3}) = 20\times10^{-9}/(3\times10^{8}\times1.732) \approx 3.85\times10^{-17}\ \text{s} = 0.038\ \text{fs}.$

**Run length.** A resonance of quality factor $Q = 15{,}000$ has a photon lifetime $\tau = Q/\omega = Q\lambda/(2\pi c) \approx 12.3$ ps. To let a naive DFT resolve the linewidth, the ring must be allowed to ring down for several lifetimes, say $5\tau \approx 60$ ps, which is

$$N_\text{steps} = \frac{60\times10^{-12}}{3.85\times10^{-17}} \approx 1.6\times10^{6}\ \text{time steps}.$$

**Run time.** The work is $N_\text{steps}\times N_\text{cells}\times\mathcal{O}(10)$ flops $\approx 10^{15}$ floating-point operations. On a memory-bound CPU this is tens of hours; on a GPU (or a cloud FDTD engine such as **Tidy3D**) it is minutes — exactly the "hours on CPU, minutes on GPU" the field quotes. A practical shortcut avoids the full ringdown entirely: **harminv**, the filter-diagonalization method built into Meep, extracts the resonant frequency and $Q$ from a short, still-ringing time trace, cutting $N_\text{steps}$ by an order of magnitude.

## The Tool Landscape

The commercial standard is **Ansys Lumerical FDTD**, prized for its meshing, material models, and integration with the rest of the design flow. **Meep** is the open-source reference — free, scriptable in Python, and the engine used in this book's exercises. **Tidy3D** (Flexcompute) runs FDTD on cloud GPUs and returns large 3D jobs in minutes, which has quietly changed what problem sizes are considered routine. Whichever the engine, the estimates above — cells, Courant-limited step count, memory — transfer unchanged, and knowing them before you press *run* is the difference between a simulation that finishes over lunch and one that never finishes at all.
