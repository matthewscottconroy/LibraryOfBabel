# 1.2.2 Gauss's Law for Magnetic Fields

## The Equation

$$\oint_S \mathbf{B} \cdot d\mathbf{A} = 0$$

## Physical Meaning: No Magnetic Monopoles

The magnetic flux through *any* closed surface is zero. This equation says something profound: **magnetic field lines never begin or end**. They form closed loops — always.

Compare with Gauss's law for electric fields: there, the flux through a closed surface can be nonzero because field lines can begin and end on electric charges. For magnetic fields, there are no such sources or sinks. A magnetic north pole is not a source from which field lines emanate and stop; every field line that comes out of a north face of a magnet eventually returns through the south face and continues in a closed loop through the magnet's interior.

This empirical fact — that no magnetic monopole has ever been observed — is the physical content of this equation [1].

## What This Means for Electromagnetic Wave Propagation

In an electromagnetic wave, the magnetic field forms closed loops perpendicular to both the direction of propagation and the electric field. The fact that $\nabla \cdot \mathbf{B} = 0$ (the differential form of this equation) constrains the wave solutions and ensures that the wave is *transverse* — the fields oscillate perpendicular to the direction of propagation, not along it. This is why light can be polarized, and why the polarization state of an optical beam carries information that can be manipulated by waveplates and polarizing beamsplitters (Chapter 2).

## If Monopoles Existed

It is worth noting what would change if magnetic monopoles were discovered. Gauss's law for $\mathbf{B}$ would become $\oint_S \mathbf{B} \cdot d\mathbf{A} = \mu_0 g_{\text{enc}}$, where $g$ is magnetic charge. Faraday's law would acquire a source term from magnetic currents. Maxwell's equations would be symmetric between electric and magnetic fields in the same way that Coulomb's law and Ampère's law are symmetric. The theory would be more elegant in a mathematical sense — but the physics of photonic computing would be essentially unchanged, since monopoles (if they exist) are enormously massive objects irrelevant to any practical device.

---

## References

[1] Particle Data Group (2022). *Review of Particle Physics*. *Progress of Theoretical and Experimental Physics*, 2022, 083C01. [The authoritative review of experimental limits on magnetic monopole existence; current limits place the flux below $\sim 10^{-15}$ cm$^{-2}$ s$^{-1}$ sr$^{-1}$.]
