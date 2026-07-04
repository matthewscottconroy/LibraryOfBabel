# 27.4.1 Photonic Topological Insulators

## The Idea, Borrowed from Electrons

Topological photonics begins by taking a concept proven for electrons and asking what survives the translation to light. In a crystalline solid, the electronic states organize into Bloch bands $E_n(\mathbf{k})$ over the Brillouin zone, and each isolated band carries a global integer — a topological invariant — computed from how the Bloch wavefunction twists as $\mathbf{k}$ sweeps the zone. The paradigm is the integer quantum Hall effect, whose transverse conductance is quantized because each filled band contributes an integer *Chern number*

$$C_n \;=\; \frac{1}{2\pi}\int_{\text{BZ}} \Omega_n(\mathbf{k})\,d^2k \;\in\; \mathbb{Z},$$

the Brillouin-zone integral of the Berry curvature $\Omega_n$. The Chern number cannot change under any smooth deformation of the Hamiltonian that keeps the band gap open; it changes only when the gap closes. The framework, its invariants, and its extension to time-reversal-invariant systems (the $\mathbb{Z}_2$ topological insulators) are laid out in the standard condensed-matter review [Hasan & Kane, *Reviews of Modern Physics*, 2010], the right background reference for everything that follows. The photonic program is to reproduce this structure for electromagnetic Bloch modes in periodic dielectric or magneto-optic media, where the "bands" are photonic bands and the invariant is computed from the photonic Bloch fields.

## Bulk–Boundary Correspondence

The reason invariants matter for devices is the *bulk–boundary correspondence*. If two materials with different values of a topological invariant are placed in contact — say a photonic crystal with $C = 1$ against one with $C = 0$, or simply against vacuum — the invariant cannot jump discontinuously while the gap stays open, so the gap must close *at the interface*. The closing manifests as states localized on the boundary, spectrally inside the bulk band gap, whose number and chirality are fixed by the difference of invariants across the interface. These edge states are not a fragile consequence of a particular boundary shape; they are mandated by a global property of the two bulks. That is the precise sense in which they are "protected": a perturbation that respects the relevant symmetry and does not close the bulk gap can move, bend, or deform the edge state but cannot remove it, because removing it would require changing an integer. The two authoritative reviews of the field develop this correspondence for each photonic platform and are the sources to consult throughout this section [Lu et al., *Nature Photonics*, 2014; Ozawa et al., *Reviews of Modern Physics*, 2019].

## The One-Dimensional Case: Photonic SSH

The simplest realization is one-dimensional. The Su–Schrieffer–Heeger (SSH) model — a chain of sites with alternating strong and weak couplings — has two topologically distinct phases distinguished by a winding number $\nu \in \{0,1\}$, and in the nontrivial phase it supports a state pinned to the middle of the band gap at the chain's end. Photonic SSH chains, built from arrays of evanescently coupled waveguides or resonators with alternating spacings, reproduce this exactly: a protected midgap edge mode appears at the termination, its existence guaranteed by the winding number and its frequency fixed at midgap by the chiral (sublattice) symmetry of the lattice. The photonic SSH chain is the cleanest teaching example of bulk–boundary correspondence in optics — the invariant is a single integer, the protected state is visible as a bright spot at the array's edge — and it establishes the pattern the two-dimensional systems elaborate.

## Two Dimensions: Chiral Edge States and Their Time-Reversal-Invariant Cousins

The landmark result is the photonic analogue of the quantum Hall effect. Wang, Chong, Joannopoulos, and Soljačić proposed and then observed *unidirectional, backscattering-immune* electromagnetic edge states in a two-dimensional magneto-optic photonic crystal, in which a static magnetic field breaks time-reversal symmetry and opens a gap with nonzero Chern number [Wang et al., *Nature*, 2009]. Light in the resulting edge channel propagates in one direction only; because there is no counter-propagating mode at the same frequency on that edge, a defect or a sharp corner has nothing to scatter *into*, and the transport is immune to backscattering. This is the strongest form of topological protection available to photons, and its demonstration at microwave frequencies is the field's founding experiment.

The difficulty is that magneto-optic response is weak at optical frequencies and awkward to integrate, so much subsequent work sought protection *without* breaking time-reversal symmetry. Two routes matter here. First, all-dielectric designs that engineer a photonic analogue of the quantum spin-Hall effect using crystalline symmetry: Wu and Hu showed that a simple honeycomb arrangement of dielectric rods, exploiting a band inversion at the zone center, yields topological edge states in an ordinary lossless dielectric with no magnetic field at all [Wu & Hu, *Physical Review Letters*, 2015]. Second, *Floquet* topological insulators, in which the topology is generated not by a static field but by periodic driving. Rechtsman et al. realized this in an array of coupled waveguides that spiral helically along the propagation direction; because the propagation coordinate plays the role of time, the helical modulation is a periodic drive that induces topologically protected edge states traversing the array — a topological insulator for light built entirely from transparent glass [Rechtsman et al., *Nature*, 2013]. On the integrated-photonics side most relevant to computing, Hafezi and coworkers built a lattice of silicon ring resonators whose engineered coupling phases emulate a magnetic field for photons, and directly *imaged* the resulting topological edge states circulating the lattice boundary on a silicon chip — a CMOS-compatible platform in which the edge modes are visible in the near field [Hafezi et al., *Nature Photonics*, 2013].

## What "Protection" Does and Does Not Mean

The honest reading of these results requires drawing the boundary of the protection precisely, because the word invites overreach. Three qualifications are essential. First, the protection is against a *specific* class of perturbation — most cleanly, backscattering into a counter-propagating mode that a chiral edge simply does not possess — and not against everything. It does nothing about material absorption, nothing about scattering out of the plane or out of the band, and nothing about disorder strong enough to close the bulk gap; a lossy topological waveguide is still lossy. Second, photons carry no charge and obey Bose rather than Fermi statistics, and there is no photonic Fermi level to fill bands up to. Every invariant and every protection theorem therefore has to be *re-derived* for the photonic setting — the symmetry that protects a given edge mode (chiral, time-reversal-like, crystalline) must be identified explicitly, and its protection extends only as far as that symmetry is respected by the real device. Third, and most important for the next subsection, "protected" is a statement about the *existence and connectivity of modes*, not about performance: it guarantees that an edge channel exists and cannot be gapped out by weak symmetric disorder, not that it routes light better than a well-designed conventional waveguide. Keeping that distinction sharp is the precondition for evaluating what topology actually buys, which is the subject of Section 27.4.2.

---

*References*

[1] Hasan, M.Z., Kane, C.L. (2010). "Colloquium: Topological insulators." *Reviews of Modern Physics* 82, 3045.

[2] Wang, Z., Chong, Y., Joannopoulos, J.D., Soljačić, M. (2009). "Observation of unidirectional backscattering-immune topological electromagnetic states." *Nature* 461, 772.

[3] Rechtsman, M.C., et al. (2013). "Photonic Floquet topological insulators." *Nature* 496, 196.

[4] Hafezi, M., Mittal, S., Fan, J., Migdall, A., Taylor, J.M. (2013). "Imaging topological edge states in silicon photonics." *Nature Photonics* 7, 1001.

[5] Wu, L.-H., Hu, X. (2015). "Scheme for achieving a topological photonic crystal by using dielectric material." *Physical Review Letters* 114, 223901.

[6] Lu, L., Joannopoulos, J.D., Soljačić, M. (2014). "Topological photonics." *Nature Photonics* 8, 821.

[7] Ozawa, T., et al. (2019). "Topological photonics." *Reviews of Modern Physics* 91, 015006.
