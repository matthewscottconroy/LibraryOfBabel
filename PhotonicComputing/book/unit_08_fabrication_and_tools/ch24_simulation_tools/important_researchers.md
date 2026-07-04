# Chapter 24: Important Researchers

---

## Kane S. Yee (1934–)

Kane Yee, a mathematician then at Lawrence Livermore, published in 1966 the algorithm that founded computational electromagnetics: a staggered spatial grid on which electric and magnetic field components interleave so that each is surrounded by the curl partners it needs, integrated by a leapfrog time march. The scheme was largely overlooked for two decades until computing caught up with it. Essentially unchanged, the **Yee cell** is the beating heart of every FDTD code in use today — Meep, Lumerical, Tidy3D — and of the ring-resonator and coupler simulations of this chapter.

---

## Allen Taflove (1949–2021)

Allen Taflove of Northwestern University coined the term "finite-difference time-domain," revived and extended Yee's method through the 1980s and 1990s, and — with Susan Hagness — wrote the textbook that made it teachable. His work on stability, dispersive-material models, and absorbing boundaries turned a neglected numerical scheme into the dominant tool of computational electromagnetics. When this chapter treats the Courant limit and PML as bedrock, it is transmitting Taflove's synthesis.

---

## John D. Joannopoulos (1947–)

John Joannopoulos of MIT built one of the world's most influential computational-photonics groups, and his text *Photonic Crystals: Molding the Flow of Light* shaped how a generation thinks about structured light. As much as his own work on photonic bandgaps, his legacy runs through his students and postdocs — Steven Johnson, Shanhui Fan, Marin Soljačić among them — who carried simulation-driven photonics into every corner of the field, including the open-source tools used here.

---

## Steven G. Johnson

Steven Johnson (MIT applied mathematics) is the principal architect of the open-source computational-photonics stack: the **Meep** FDTD package, the **MPB** mode solver, and the `harminv` filter-diagonalization tool that extracts a resonator's $Q$ from a short time trace. His subpixel-smoothing scheme restored high accuracy to FDTD at curved boundaries, and his rigorous attention to convergence and numerical error set a standard for the field. Much of this chapter's exercise flow runs on his software.

---

## Jelena Vučković

Jelena Vučković (Stanford) leads one of the groups that turned photonic inverse design from concept into practice. Her lab produced the fabricated compact wavelength demultiplexer that convinced skeptics (Piggott et al., 2015) and the **SPINS** framework that codified fabrication-aware topology optimization — filtering, projection, minimum-feature and robustness constraints. Her insistence on devices that survive real fabrication, not just simulation, is the throughline of Section 24.3.2.

---

## Alexander Y. Piggott

Alexander Piggott, as a doctoral student in Vučković's group, designed and demonstrated the $2.8\times2.8\ \mu\text{m}$ inverse-designed wavelength demultiplexer (*Nature Photonics*, 2015) — the result most responsible for the inverse-design wave that followed. His work showed that topology optimization could produce compact, broadband, manufacturable silicon devices with performance beyond hand design, and he carried the methods into industry.

---

## Shanhui Fan

Shanhui Fan (Stanford), another of Joannopoulos's academic descendants, has driven the fusion of electromagnetic simulation with modern optimization and machine learning. His group developed differentiable Maxwell solvers and adjoint methods for linear and nonlinear devices, contributed foundational ideas to photonic neural networks and temporal photonics, and produced tools and students that populate the differentiable-simulation landscape of this chapter.

---

## Tyler W. Hughes and Momchil Minkov

Working in Fan's group, Hughes and Minkov built **ceviche** — an open-source, autograd-differentiable FDFD/FDTD engine that made the adjoint gradient as routine to obtain as a forward transmission — and related tools such as Minkov's guided-mode-expansion solver `legume`. Now at Flexcompute (the company behind Tidy3D), they exemplify the convergence of automatic differentiation and electromagnetic simulation that underlies Sections 24.2.2 and 24.3.

---

## Eli Yablonovitch (1946–)

Eli Yablonovitch (UC Berkeley), a co-originator of the photonic bandgap concept, was also an early champion of **adjoint electromagnetic optimization**. Work from his group — with Owen Miller and Christopher Lalau-Keraly — gave the field an accessible, influential statement of the two-simulation gradient (*Optics Express*, 2013), connecting the mathematics of adjoint sensitivity to practical photonic device design.

---

## Wim Bogaerts

Wim Bogaerts (Ghent University–IMEC) is a central figure in **photonic design automation**: compact modeling, circuit simulation, variability analysis, and the layout-to-tape-out methodology that this chapter presents as the discipline of the field. His reviews (notably with Chrostowski, 2018) map the design stack, and his work on programmable photonic circuits connects the simulation tools of Chapter 24 to the reconfigurable architectures of Units V and VI.
