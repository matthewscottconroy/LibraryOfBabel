# Chapter 1: Maxwell's Equations and Electromagnetic Waves

> *"The special theory of relativity owes its origin to Maxwell's equations of the electromagnetic field."*
>
> — Albert Einstein, "Maxwell's Influence on the Development of the Conception of Physical Reality" (1931)

---

## Why This Chapter Exists

There is a temptation, when writing a book about photonic *computing*, to begin with computation — with neurons, matrices, and waveguides. To treat Maxwell's equations as background knowledge, mentioned briefly and moved past. This book resists that temptation, and for a reason that goes to the heart of what makes photonic computing physically possible.

Photonic computing is not merely a faster version of electronic computing using a different carrier. It is a fundamentally different physical regime of information processing, and understanding *why* requires going all the way back to the structure of the electromagnetic field. The key properties of light that make it attractive for computing — its speed, its lack of charge, its ability to propagate without dissipation, its extraordinary bandwidth — are not engineering facts that someone decided on. They are consequences of Maxwell's equations. They flow from the mathematical structure of how the electromagnetic field behaves.

This chapter derives Maxwell's equations, shows that they demand the existence of electromagnetic waves, and establishes the physical interpretation of those waves. Everything in the chapters that follow — waveguides, resonators, modulators, lasers, photonic neural networks, quantum optical computers — is built on this foundation.

---

## The Central Question: What Is Light?

For most of human history, light was understood through its behavior: it traveled in straight lines (usually), it reflected off mirrors, it refracted when passing between media, it could be focused by lenses. Newton proposed in 1704 that light consisted of particles — "corpuscles" — which obeyed the laws of mechanics [1]. Huygens had proposed in 1678 that light was a wave in some medium, analogous to sound [2].

The wave picture won, decisively, in the early 19th century: Thomas Young's double-slit experiment (1801) showed interference patterns that no particle theory could explain [3], and Augustin-Jean Fresnel's wave theory of diffraction (1818) quantitatively matched observation [4]. But what kind of wave? What was waving?

Maxwell's answer, completed in 1865, was that light is a wave in the electromagnetic field — a self-sustaining oscillation of electric and magnetic fields that propagates through space without any medium [5]. The medium the 19th century had called the "luminiferous ether" was unnecessary. Light needed nothing to propagate through. The field itself was the thing.

This is a profound conceptual shift. The electromagnetic field — not matter, not medium — is a physical entity that carries energy and momentum through empty space. Photonic computing is, at its deepest level, the art of using that entity to process information.

---

## What You Will Learn in This Chapter

By the end of this chapter, you will be able to:

1. State and physically interpret each of Maxwell's four equations in both integral and differential form.
2. Derive the electromagnetic wave equation from Maxwell's equations.
3. Write down and interpret the plane wave solutions to the wave equation.
4. Calculate the speed of light from the fundamental constants $\varepsilon_0$ and $\mu_0$.
5. Compute the energy flux (Poynting vector), radiation pressure, and angular momentum carried by an electromagnetic wave.
6. Write Maxwell's equations in matter and explain the physical meaning of the dielectric constant and magnetic permeability.
7. Derive and apply boundary conditions at dielectric interfaces.

---

## References

[1] Newton, I. (1704). *Opticks, or a Treatise of the Reflexions, Refractions, Inflexions and Colours of Light*. London: Smith and Walford.

[2] Huygens, C. (1690). *Traité de la Lumière*. Leiden: Pieter van der Aa. [English translation: *Treatise on Light*, University of Chicago Press, 1945.]

[3] Young, T. (1804). "The Bakerian Lecture: Experiments and calculations relative to physical optics." *Philosophical Transactions of the Royal Society of London*, 94, 1–16.

[4] Fresnel, A.J. (1816). "Mémoire sur la diffraction de la lumière." *Annales de Chimie et de Physique*, 1, 239–281.

[5] Maxwell, J.C. (1865). "A dynamical theory of the electromagnetic field." *Philosophical Transactions of the Royal Society of London*, 155, 459–512.
