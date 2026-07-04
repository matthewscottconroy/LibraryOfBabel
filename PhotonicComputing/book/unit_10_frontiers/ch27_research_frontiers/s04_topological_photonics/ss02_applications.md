# 27.4.2 Applications: Robust Transport, Topological Lasers, and Non-Hermitian Photonics

Having established what topological protection is (Section 27.4.1), we can ask the engineer's question directly: what is it good for, and where does the advantage survive comparison with a competent conventional design? Three application areas dominate the literature, and they range from the genuinely promising to the actively contested. Throughout, the two field reviews remain the anchor [Lu et al., *Nature Photonics*, 2014; Ozawa et al., *Reviews of Modern Physics*, 2019], and throughout we hold the physics result apart from the computing claim.

## Robust Transport and Routing

The first and most obvious application is waveguiding. A chiral topological edge channel carries light around sharp corners, past fabrication defects, and through disorder with suppressed backscattering, because — as Section 27.4.1 established — there is no counter-propagating mode for a defect to scatter into. For dense photonic interconnect, where light must be routed through congested layouts full of bends and crossings, an in-principle backscattering-immune channel is attractive: reflections are a genuine problem in tightly packed silicon photonics, and a routing fabric that suppressed them by topology rather than by careful bend design would be valuable.

Whether that in-principle advantage is a *practical* one is genuinely contested, and honesty requires saying so. The comparison is not against a naive waveguide but against a well-engineered conventional one, and modern low-loss silicon and silicon-nitride waveguides already route light around adiabatic bends with very low backscattering. Against that baseline the topological channel must pay real costs — a larger footprint, since the edge mode occupies a supercell of a photonic lattice rather than a single-mode wire, and, in magneto-optic realizations, materials that are hard to integrate — while its protection covers only backscattering and leaves in-plane and out-of-plane loss untouched. The result is that topological routing is a demonstrated physical effect whose net engineering benefit over a competent trivial waveguide is, as of roughly 2025, not clearly established. It is exactly the kind of claim the chapter's first discipline was written for: identify the baseline, and the baseline here is strong.

## Topological Lasers

The most compelling application turns the edge state into a laser mode. If a topological edge channel is given gain — by pumping the edge of a nontrivial lattice — the lasing mode inherits the edge state's robustness: it is pinned to the boundary, spatially extended around the entire perimeter, and resistant to the fabrication disorder and local defects that spoil single-mode operation in conventional cavity arrays. Theory and experiment were reported together: Harari et al. laid out the theory of a topological insulator laser [Harari et al., *Science*, 2018], and Bandres et al. demonstrated it, showing single-mode lasing on a topologically protected edge mode of an active photonic lattice with improved efficiency and mode purity relative to a topologically trivial control [Bandres et al., *Science*, 2018]. The inclusion of a trivial control is what makes this case strong: the comparison is built into the experiment, and the topological device wins it. Here the protection does concrete work — it enforces single-mode operation over a large gain area that would otherwise support many competing modes — and the application is correspondingly the field's most persuasive.

## The Non-Hermitian and PT-Symmetric Frontier

A parallel frontier drops the assumption that the photonic system is Hermitian (lossless) and instead engineers gain and loss as design resources. When gain and loss are balanced according to *parity–time* (PT) symmetry — a structure invariant under combined spatial reflection and time reversal — the non-Hermitian Hamiltonian can nonetheless possess entirely real eigenvalues, until, as the gain/loss contrast is increased past a threshold, the system undergoes a *PT-symmetry-breaking* transition. The transition point is an *exceptional point* (EP): a non-Hermitian degeneracy at which two (or more) eigenvalues *and their eigenvectors* coalesce, unlike the ordinary degeneracies of Hermitian systems where only eigenvalues meet. The physics and its photonic embodiments are surveyed in the field's reviews [Feng et al., *Nature Photonics*, 2017; El-Ganainy et al., *Nature Physics*, 2018]. Photonics is an unusually natural home for these ideas because gain (from optical amplifiers or pumped media) and loss are both readily available and spatially patternable, and PT-symmetric coupled waveguides, microrings, and lasers have all been realized.

Exceptional points bring genuinely unusual behavior — single-mode selection by symmetry breaking, nonreciprocal response, unconventional mode management — and one much-publicized application: *sensing*. Near an $n$-th-order EP, a perturbation of strength $\epsilon$ splits the degenerate frequencies not linearly but as $\epsilon^{1/n}$, so that for small $\epsilon$ the bare response is parametrically larger than the linear splitting of a conventional sensor [Miri & Alù, *Science*, 2019]. Taken at face value this promises dramatically enhanced sensitivity. The essential honest caveat is that the promise is *contested once noise is properly accounted for*: the same non-Hermitian mathematics that steepens the signal response also modifies the noise — linewidth broadening near the EP and excess noise from the non-orthogonality of the coalescing eigenvectors — and a careful analysis of the signal-to-noise ratio, rather than the bare frequency splitting, can erode or eliminate the apparent advantage. As of roughly 2025 the community has not reached consensus that EP sensors deliver a net improvement in fundamental sensitivity for a fair figure of merit. This is precisely the pattern the chapter warns about: a spectacular response function is a physics result; a better sensor is an engineering claim, and the two must be argued separately.

## What This Buys Computing

It is worth stating plainly where all of this sits relative to the subject of the book. Topological and non-Hermitian photonics are, for the most part, *enabling infrastructure* rather than computing primitives. None of the results above is a new way to multiply matrices, store a bit, or apply a nonlinearity — the operations from which a photonic processor is built (Units V–VII). What they offer instead is better *components*: routing fabrics with suppressed backscattering, lasers that stay single-mode over large areas despite disorder, and novel mechanisms for mode selection and control that could stabilize the sources and interconnects on which every architecture in this chapter depends. That is a real contribution, and it should be claimed as exactly what it is. The reader should keep the distinction sharp: the demonstrated physics here is often superb, but its value to computing flows through the reliability and manufacturability of infrastructure, not through a new arithmetic. Whether topology becomes part of the standard photonic-computing toolkit will be decided, as usual, at the interfaces and against strong conventional baselines — not by the elegance of the underlying invariant.

---

*References*

[1] Feng, L., El-Ganainy, R., Ge, L. (2017). "Non-Hermitian photonics based on parity–time symmetry." *Nature Photonics* 11, 752.

[2] Harari, G., et al. (2018). "Topological insulator laser: theory." *Science* 359, eaar4003.

[3] Bandres, M.A., et al. (2018). "Topological insulator laser: experiments." *Science* 359, eaar4005.

[4] El-Ganainy, R., et al. (2018). "Non-Hermitian physics and PT symmetry." *Nature Physics* 14, 11.

[5] Miri, M.-A., Alù, A. (2019). "Exceptional points in optics and photonics." *Science* 363, eaar7709.

[6] Lu, L., Joannopoulos, J.D., Soljačić, M. (2014). "Topological photonics." *Nature Photonics* 8, 821.

[7] Ozawa, T., et al. (2019). "Topological photonics." *Reviews of Modern Physics* 91, 015006.
