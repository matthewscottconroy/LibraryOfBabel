# 24.1.3 Eigenmode Expansion and Beam Propagation

FDTD and FEM make no assumption about the shape of a device, and pay for that generality with a cost that scales with the enclosed volume. But a great many photonic components are *long* and *nearly invariant along the propagation axis*: adiabatic tapers, multimode-interference couplers, arrayed-waveguide gratings, spot-size converters, grating couplers. Simulating a 200 μm taper with a volumetric solver is wasteful, because almost nothing changes from one micrometer to the next. Two methods exploit that structure — eigenmode expansion and beam propagation — and between them cover the long, slowly varying devices that would otherwise dominate a simulation budget.

## Eigenmode Expansion (EME)

EME rests on a single idea: at any cross-section, the field can be written exactly as a superposition of the *local eigenmodes* of that cross-section — forward- and backward-propagating — computed by the mode solver of the previous subsection. The recipe is:

1. **Slice** the structure into sections that are uniform (or nearly so) along $z$.
2. **Solve** for the first $M$ local modes in each section's cross-section.
3. **Propagate** through a uniform section analytically: mode $m$ simply acquires phase $e^{\pm i\beta_m z}$ over its length — the propagation is *free*, independent of how long the section is.
4. **Match** at each interface between sections by projecting the modes of one onto those of the next through overlap integrals, producing an interface scattering matrix.
5. **Cascade** the interface and propagation matrices into a single overall scattering matrix.

Two properties make EME powerful. First, it is **bidirectional**: because both forward and backward modes are kept and the interfaces are matched with full scattering matrices, EME captures reflections and back-scattering exactly — it will correctly predict the return loss of a taper or the ripple of a grating, which the beam propagation method cannot. Second, its cost is set by the number of modes $M$ and the number of *distinct* sections, **not by the length in wavelengths**. A uniform region contributes only a diagonal phase matrix regardless of its physical length, and a periodic structure — a grating of $N$ identical periods — is handled by computing one period's scattering matrix and raising it to the $N$th power. This is EME's superpower: structures that are hundreds of wavelengths long but piecewise uniform or periodic collapse to a tiny computation.

The price is convergence in $M$. The local-mode basis must be rich enough to represent the true field, including its radiating parts; high-index-contrast silicon waveguides and abrupt transitions can demand many modes (tens to over a hundred), and an under-resolved basis silently loses power. The standard tools are Ansys Lumerical MODE (its EME solver) and Photon Design's FIMMWAVE/FIMMPROP.

## Worked Example: EME versus FDTD for a Long Taper

Consider an adiabatic taper widening a $180$ nm inverse-taper tip to a $450$ nm routing waveguide over a length $L = 200\ \mu\text{m}$ — a typical edge-coupler spot-size converter.

**EME.** Slice the taper into 200 sections of $1\ \mu\text{m}$, each treated as locally uniform, and keep $M \approx 30$ modes. Each interface requires a $30\times30$ overlap-matrix computation; the 200 interface matrices and 200 diagonal propagation matrices cascade into the device scattering matrix in **seconds** on a laptop. Crucially, if the taper were made $500\ \mu\text{m}$ long, the cost would be *nearly unchanged* — the sections just get longer, and length is free in EME. Sweeping the taper length to find where the transition becomes adiabatic (say, insertion loss below 0.1 dB) is therefore almost instantaneous.

**FDTD, for contrast.** The same taper in a $200\times2\times2\ \mu\text{m}$ FDTD domain at $\Delta x = 20$ nm needs $10^{4}\times100\times100 = 10^{8}$ cells, and the Courant-limited step count grows with the domain, so a single length is an hours-long GPU job — and every new length is a fresh run. For this class of device, EME is not a little faster; it is the difference between a design sweep over coffee and a week on a cluster.

## The Beam Propagation Method (BPM)

BPM makes a stronger approximation and buys even more speed. It factors the fast phase oscillation out of the field, writing $\mathbf{E}(x,y,z) = \mathbf{u}(x,y,z)\,e^{i\beta_0 z}$ with $\mathbf{u}$ a *slowly varying envelope*, and drops the second $z$-derivative of $\mathbf{u}$ (the **paraxial** or slowly-varying-envelope approximation). Maxwell's equations then reduce to a first-order marching problem in $z$: given the field at one plane, a small linear solve advances it by a step $\Delta z$. Memory is that of a single cross-section, and the march is fast. Implementations split into finite-difference (FD-BPM) and split-step Fourier (FFT-BPM) variants, and **wide-angle (Padé)** corrections partially restore accuracy for fields propagating at larger angles to the axis.

BPM's approximations are also its limits. Being paraxial and **one-way**, it neglects the backward wave entirely: it cannot model a reflection, a resonator, or the return loss of a discontinuity. It degrades for high index contrast and for sharp bends, where the field acquires large transverse wavevector components. Its natural home is therefore weakly guiding, low-reflection, long structures: fiber-to-chip transitions, arrayed-waveguide gratings, titanium-diffused lithium-niobate modulators, and long tapers where reflections are negligible. The classic engines are Synopsys RSoft BeamPROP and Optiwave OptiBPM. As a rough budget, a $200\ \mu\text{m}$ propagation at $\Delta z = 0.5\ \mu\text{m}$ is 400 marching steps, each a modest cross-sectional solve — subsecond, with a memory footprint orders of magnitude below FDTD's.

## Choosing Among the Structured Solvers

The decision reduces to two questions. *Do reflections matter?* If yes — a grating, a resonant coupler, anything where return loss is a spec — use EME, never BPM. *Is the structure piecewise uniform or periodic?* If yes, EME's length-free propagation and period-cascading make it ideal; the arrayed-waveguide grating is the textbook case. If the structure is long, low-contrast, adiabatic, and reflection-free, BPM is the fastest tool that will give the right answer. And when the geometry is compact, high-contrast, and reflection-rich — a ring, a photonic-crystal cavity, an inverse-designed splitter — neither approximation holds and the problem belongs back with FDTD or FEM. Fluency in this triage, more than mastery of any one solver, is what keeps a design cycle moving.
